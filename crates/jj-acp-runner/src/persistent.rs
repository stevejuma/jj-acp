//! Negotiation and turns on a retained ACP connection.
use crate::TurnOutcome;
use crate::turn::Agent;
use jj_acp_protocol::budget::TurnBudget;
use jj_acp_protocol::handler::SessionHandler;
use jj_acp_protocol::initialize::{ClientCapabilities, ClientInfo, InitializeResult};
use jj_acp_protocol::permission::PermissionDecision;
use jj_acp_protocol::session::{Session, SessionNewRequest};
use jj_acp_protocol::{Error, StopReason};
use std::time::Duration;
use tokio::sync::watch;

/// Negotiate and create or explicitly load a caller-selected session.
pub async fn open<H: SessionHandler>(
    agent: &mut Agent,
    handler: &mut H,
    request: &SessionNewRequest<'_>,
    load: Option<&str>,
    budget: TurnBudget,
) -> Result<(InitializeResult, Session), Error> {
    let initialized = agent
        .connection
        .initialize_within(
            handler,
            &ClientInfo::jj_acp(),
            ClientCapabilities::default(),
            budget,
        )
        .await?;
    let session = if let Some(id) = load {
        if !initialized.advertises(&["loadSession"]) {
            return Err(Error::Protocol("resume_unsupported".into()));
        }
        let result = agent.connection.call_within(handler, "session/load",
            serde_json::json!({"sessionId":id,"cwd":request.cwd,"mcpServers":request.mcp_servers}), budget).await?;
        Session {
            id: id.into(),
            raw: result,
        }
    } else {
        agent
            .connection
            .session_new_within(handler, request, budget)
            .await?
    };
    Ok((initialized, session))
}

/// Prompt an existing session, retaining the connection only after a real stop.
pub async fn prompt<H: SessionHandler>(
    agent: &mut Agent,
    handler: &mut H,
    session_id: &str,
    text: &str,
    budget: TurnBudget,
    grace: Duration,
    cancel: &mut watch::Receiver<bool>,
) -> TurnOutcome {
    let params = jj_acp_protocol::session::prompt_text_params(session_id, &[text]);
    let id = match agent
        .connection
        .send_request("session/prompt", params)
        .await
    {
        Ok(id) => id,
        Err(error) => return TurnOutcome::from_protocol(&error, Some(session_id.into())),
    };
    let result = tokio::select! {
        value = agent.connection.await_response(handler, id, budget) => Some(value),
        () = cancellation(cancel) => None,
    };
    let result = if let Some(result) = result {
        result
    } else {
        if let Some(permission) = handler.pending_permission() {
            let _ = agent
                .connection
                .answer_permission(&permission, &PermissionDecision::Cancel)
                .await;
        }
        let _ = agent.connection.session_cancel(session_id).await;
        match agent
            .connection
            .await_response(handler, id, TurnBudget::fixed(grace))
            .await
        {
            Ok(value) => Ok(value),
            Err(error) => return TurnOutcome::from_protocol(&error, Some(session_id.into())),
        }
    };
    match result.and_then(|value| jj_acp_protocol::session::parse_stop_reason(&value)) {
        Ok(StopReason::Cancelled) => TurnOutcome::cancelled(Some(session_id.into())),
        Ok(reason) => TurnOutcome::completed(reason, session_id.into()),
        Err(error) => TurnOutcome::from_protocol(&error, Some(session_id.into())),
    }
}

/// Wait for a caller cancellation without interpreting sender loss as approval.
pub async fn cancellation(cancel: &mut watch::Receiver<bool>) {
    loop {
        if *cancel.borrow_and_update() {
            return;
        }
        if cancel.changed().await.is_err() {
            std::future::pending::<()>().await;
        }
    }
}
