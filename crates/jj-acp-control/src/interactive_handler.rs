//! Bounded streamed activity and exact permission lifetimes.
use crate::interactive_wire::{MAX_FRAME, frame};
use jj_acp_protocol::handler::SessionHandler;
use jj_acp_protocol::permission::{PermissionDecision, PermissionRequest};
use jj_acp_protocol::update::SessionUpdate;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicU64, Ordering},
};
use std::time::Duration;
use tokio::sync::{mpsc, oneshot, watch};

/// One permission, owned only while the adapter awaits a response.
pub struct PendingPermission {
    pub options: Vec<String>,
    pub answer: oneshot::Sender<Option<String>>,
}
/// Live permission decisions; independent of any durable application state.
pub type Permissions = Arc<Mutex<HashMap<String, PendingPermission>>>;
/// Shared ordered stream for one interactive session.
#[derive(Clone)]
pub struct Events {
    pub output: mpsc::Sender<Value>,
    pub sequence: Arc<AtomicU64>,
    pub cancel: watch::Sender<bool>,
}
impl Events {
    /// Queue a complete frame or cancel on bounded backpressure.
    pub fn emit(&self, kind: &str, id: &str, mut body: Value) {
        body["sequence"] = json!(self.sequence.fetch_add(1, Ordering::Relaxed) + 1);
        let value = frame(kind, Some(id), body);
        if serde_json::to_vec(&value).map_or(true, |bytes| bytes.len() > MAX_FRAME)
            || self.output.try_send(value).is_err()
        {
            let _ = self.cancel.send(true);
        }
    }
}
struct PermissionLifetime {
    permissions: Permissions,
    id: String,
}
impl Drop for PermissionLifetime {
    fn drop(&mut self) {
        if let Ok(mut map) = self.permissions.lock() {
            map.remove(&self.id);
        }
    }
}
/// Caller-observed handler; content is sent to the trusted parent, never logged.
pub struct Handler {
    pub events: Events,
    pub request_id: String,
    pub permissions: Permissions,
    pub timeout: Duration,
    pub pending_rpc: Option<Value>,
}
impl SessionHandler for Handler {
    fn pending_permission(&self) -> Option<Value> {
        self.pending_rpc.clone()
    }
    fn session_update(&mut self, update: SessionUpdate) {
        self.events
            .emit("activity", &self.request_id, json!({"update":update.raw()}));
    }
    async fn permission(&mut self, request: PermissionRequest) -> PermissionDecision {
        self.pending_rpc = Some(request.id.clone());
        let id = format!(
            "permission-{}",
            self.events.sequence.fetch_add(1, Ordering::Relaxed) + 1
        );
        let (sender, receiver) = oneshot::channel();
        let options = request
            .options
            .iter()
            .map(|option| option.option_id.clone())
            .collect();
        if let Ok(mut map) = self.permissions.lock() {
            map.insert(
                id.clone(),
                PendingPermission {
                    options,
                    answer: sender,
                },
            );
        } else {
            return PermissionDecision::Cancel;
        }
        let lifetime = PermissionLifetime {
            permissions: self.permissions.clone(),
            id: id.clone(),
        };
        self.events.emit(
            "permission",
            &self.request_id,
            json!({"permissionId":id,"request":request.raw}),
        );
        let mut cancel = self.events.cancel.subscribe();
        let decision = tokio::select! {
            result=tokio::time::timeout(self.timeout,receiver)=>match result {
                Ok(Ok(Some(option)))=>PermissionDecision::Select(option), _=>PermissionDecision::Cancel,
            },
            ()=jj_acp_runner::persistent::cancellation(&mut cancel)=>PermissionDecision::Cancel,
        };
        self.events.emit(
            "permission_resolved",
            &self.request_id,
            json!({"permissionId":id,
            "optionId":match &decision {PermissionDecision::Select(option)=>Some(option),_=>None}}),
        );
        self.pending_rpc = None;
        drop(lifetime);
        decision
    }
}
