//! The persistent control loop.
//!
//! One controlling application drives one harness over stdin and stdout. Work is
//! executed one submission at a time in this slice; a standing pool with
//! caller-named scopes is a separate milestone. Cancellation is genuinely
//! concurrent, because a controlling application that cannot interrupt a running
//! task has no way to bound it.

use std::sync::Arc;

use lapis_acp_agent::Catalog;
use lapis_acp_agent::availability::{self, PathProbe};
use serde_json::{Value, json};
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncWrite, AsyncWriteExt};
use tokio::sync::{Mutex, watch};

use crate::invocation::{InvocationConfig, invoke_with_probe};
use crate::protocol::{Request, Response, code_control, parse_request_text};
use crate::record::TerminalRecord;
use crate::task::{TaskRejection, code};

/// One in-flight submission and the handle that cancels it.
struct InFlight {
    request_id: String,
    cancel: watch::Sender<bool>,
}

/// How the control loop ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServeExit {
    /// The controlling application asked for shutdown.
    Shutdown,
    /// The input stream ended.
    Eof,
    /// The input stream failed.
    InputFailed,
}

/// Summarize the enabled runners for a control frame.
///
/// Availability is probed, never installed, and an unavailable runner reports its
/// guidance so a controlling application can show an operator what to do.
pub fn runner_summaries(catalog: &Catalog) -> Vec<Value> {
    availability::probe_all(catalog.list(), &PathProbe)
        .into_iter()
        .map(|report| {
            json!({
                "id": report.runner_id,
                "label": report.label,
                "available": report.available(),
                "command": report.resolved.as_ref().map(|spec| spec.command.clone()),
                "args": report.resolved.as_ref().map(|spec| spec.args.clone()),
                "installHint": report.install_hint,
                "installInstructionsUrl": report.install_instructions_url,
            })
        })
        .collect()
}

/// Write one frame, holding the lock across the flush so lines cannot interleave.
async fn emit<W: AsyncWrite + Unpin>(writer: &Mutex<W>, response: &Response) {
    let mut line = response.to_line();
    line.push('\n');
    let mut guard = writer.lock().await;
    // A write failure means the controlling application is gone; there is nobody
    // left to tell, so it is dropped rather than escalated.
    let _ = guard.write_all(line.as_bytes()).await;
    let _ = guard.flush().await;
}

/// Run the control loop until shutdown or end of input.
///
/// The loop never blocks on a submission: work is spawned so that a later frame
/// can still cancel it. Stdout carries control frames only, and diagnostics that
/// the one-shot path would print to stderr travel as `diagnostic` frames instead,
/// because a controlling application parsing stdout must not have to distinguish
/// prose from frames.
pub async fn serve<R, W>(input: R, output: W, config: InvocationConfig) -> ServeExit
where
    R: AsyncBufRead + Unpin,
    W: AsyncWrite + Unpin + Send + 'static,
{
    let writer = Arc::new(Mutex::new(output));
    let config = Arc::new(config);
    let inflight: Arc<Mutex<Option<InFlight>>> = Arc::new(Mutex::new(None));

    emit(
        &writer,
        &Response::Ready {
            runners: runner_summaries(&config.catalog),
        },
    )
    .await;

    let mut lines = input.lines();
    loop {
        let line = match lines.next_line().await {
            Ok(Some(line)) => line,
            Ok(None) => return ServeExit::Eof,
            Err(_) => return ServeExit::InputFailed,
        };
        if line.trim().is_empty() {
            continue;
        }
        match parse_request_text(&line) {
            Err(rejection) => {
                emit(&writer, &refusal(&line, &rejection)).await;
            }
            Ok(Request::Runners { request_id }) => {
                emit(
                    &writer,
                    &Response::Runners {
                        request_id,
                        runners: runner_summaries(&config.catalog),
                    },
                )
                .await;
            }
            Ok(Request::Shutdown { request_id }) => {
                emit(&writer, &Response::Ack { request_id }).await;
                drain(&inflight, &config, &writer).await;
                return ServeExit::Shutdown;
            }
            Ok(Request::Cancel { request_id, target }) => {
                let matched = {
                    let guard = inflight.lock().await;
                    match guard.as_ref() {
                        Some(current) if current.request_id == target => {
                            let _ = current.cancel.send(true);
                            true
                        }
                        _ => false,
                    }
                };
                // Emitted after the lock is released: the spawned task clears
                // in-flight under the same lock, and holding it across a write
                // would let two frames deadlock each other.
                if matched {
                    emit(&writer, &Response::Ack { request_id }).await;
                } else {
                    emit(
                        &writer,
                        &Response::Error {
                            request_id: Some(request_id),
                            code: code_control::NOT_IN_FLIGHT.to_owned(),
                            reason: format!("no submission {target:?} is in flight"),
                        },
                    )
                    .await;
                }
            }
            Ok(Request::Submit { request_id, task }) => {
                let busy = inflight.lock().await.is_some();
                if busy {
                    emit(
                        &writer,
                        &Response::Error {
                            request_id: Some(request_id),
                            code: code_control::BUSY.to_owned(),
                            reason: "a submission is already in flight".to_owned(),
                        },
                    )
                    .await;
                    continue;
                }
                let (sender, receiver) = watch::channel(false);
                *inflight.lock().await = Some(InFlight {
                    request_id: request_id.clone(),
                    cancel: sender,
                });
                let writer = writer.clone();
                let config = config.clone();
                let inflight = inflight.clone();
                tokio::spawn(async move {
                    let mut receiver = receiver;
                    let invocation =
                        invoke_with_probe(&config, task, &mut receiver, &PathProbe).await;
                    for message in &invocation.diagnostics {
                        emit(
                            &writer,
                            &Response::Diagnostic {
                                message: message.clone(),
                            },
                        )
                        .await;
                    }
                    emit(
                        &writer,
                        &Response::Record {
                            request_id: request_id.clone(),
                            record: invocation.record,
                        },
                    )
                    .await;
                    *inflight.lock().await = None;
                });
            }
        }
    }
}

/// Wait for an in-flight submission to finish before shutting down.
///
/// Bounded by the configuration's own cleanup intervals through the invocation,
/// so a shutdown cannot hang indefinitely on an agent that will not stop.
async fn drain<W: AsyncWrite + Unpin + Send + 'static>(
    inflight: &Arc<Mutex<Option<InFlight>>>,
    config: &Arc<InvocationConfig>,
    writer: &Arc<Mutex<W>>,
) {
    let grace = config.cancel_grace + config.reap;
    let deadline = tokio::time::Instant::now() + grace + config.cancel_grace;
    loop {
        let busy = inflight.lock().await.is_some();
        if !busy || tokio::time::Instant::now() >= deadline {
            return;
        }
        let _ = writer;
        tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
    }
}

/// Build a refusal frame, recovering the correlation id when one is present.
///
/// A controlling application that sent a malformed frame still needs to correlate
/// the refusal, so the identifier is read from the raw text rather than from a
/// parse that already failed.
fn refusal(line: &str, rejection: &TaskRejection) -> Response {
    let request_id = serde_json::from_str::<Value>(line)
        .ok()
        .and_then(|frame| {
            frame
                .get("requestId")
                .and_then(Value::as_str)
                .map(str::to_owned)
        })
        .filter(|value| !value.is_empty());
    let mapped = if rejection.code == code::INVALID_TASK_JSON
        || rejection.code == code::TASK_TOO_LARGE
        || rejection.code == code::UNSUPPORTED_TASK_VERSION
        || rejection.code == code::INVALID_TASK_ID
        || rejection.code == code::INVALID_PROMPT
        || rejection.code == code::INVALID_DURATION
    {
        // A task document problem is reported as a terminal record so a
        // controlling application handles one shape for every task outcome.
        return Response::Record {
            request_id: request_id.unwrap_or_default(),
            record: TerminalRecord::invalid(rejection.code),
        };
    } else {
        rejection.code.to_owned()
    };
    Response::Error {
        request_id,
        code: mapped,
        reason: rejection.reason.clone(),
    }
}
