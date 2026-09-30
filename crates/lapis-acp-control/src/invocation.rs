//! Executing one prepared task and mapping it to a terminal record.
//!
//! This is the composition point: it resolves a runner from the catalog, assembles
//! a launch environment, executes one turn through the runner crate, stops the
//! process within bounds, and reduces everything observed to one sanitized record
//! and one exit code. It owns no policy that belongs to the controlling
//! application, which chose the runner, the working directory and the prompt.

use std::path::PathBuf;
use std::time::Duration;

use lapis_acp_agent::Catalog;
use lapis_acp_agent::availability::{self, CommandProbe, PathProbe};
use lapis_acp_agent::environment::{self, EnvironmentPlan};
use lapis_acp_protocol::Connection;
use lapis_acp_protocol::budget::TurnBudget;
use lapis_acp_protocol::handler::RecordingHandler;
use lapis_acp_protocol::session::SystemPromptTransport;
use lapis_acp_runner::launch::LaunchSpec;
use lapis_acp_runner::outcome::{FailureKind, TurnOutcome, TurnStatus};
use lapis_acp_runner::process::ProcessGuard;
use lapis_acp_runner::turn::{Agent, TurnRequest, run_turn};
use tokio::sync::watch;

use crate::record::{ExitCode, Status, TerminalRecord, code as record_code};
use crate::task::{TaskDocument, TaskRejection, code, requested_cap};

/// Trusted launch configuration for one invocation.
#[derive(Debug, Clone)]
pub struct InvocationConfig {
    /// Runners the caller may select from.
    pub catalog: Catalog,
    /// The runner this process was launched to act as, when the caller pinned one.
    pub configured_runner: Option<String>,
    /// Working directory. A task document cannot change it.
    pub cwd: PathBuf,
    /// Idle window reset by agent output.
    pub idle: Duration,
    /// Absolute cap no task document can raise.
    pub absolute_cap: Duration,
    /// Cooperative cancellation grace before a kill.
    pub cancel_grace: Duration,
    /// Reap bound after a kill.
    pub reap: Duration,
    /// Model to request when the agent offers it.
    pub model: Option<String>,
    /// Standing instructions, when configured.
    pub system_prompt: Option<String>,
    /// Where the adapter expects standing instructions.
    pub prompt_transport: SystemPromptTransport,
    /// Inherited environment, the lowest precedence layer.
    pub inherited_env: Vec<(String, String)>,
}

/// The result of one attempted invocation.
#[derive(Debug, Clone, PartialEq)]
pub struct Invocation {
    /// The single terminal record to write to stdout.
    pub record: TerminalRecord,
    /// The exit code, unless `cancelled` is set.
    pub exit: ExitCode,
    /// Set when a cancellation signal ended the run, so the caller can choose the
    /// signal-specific exit code it actually observed.
    pub cancelled: bool,
    /// Operator diagnostics for stderr. Never written to stdout.
    pub diagnostics: Vec<String>,
}

impl Invocation {
    fn new(record: TerminalRecord, exit: ExitCode, diagnostics: Vec<String>) -> Self {
        Self {
            record,
            exit,
            cancelled: false,
            diagnostics,
        }
    }
}

fn exit_for(record: &TerminalRecord) -> ExitCode {
    match record.status {
        Status::Completed => ExitCode::Ok,
        Status::TimedOut => ExitCode::Deadline,
        Status::Invalid => ExitCode::Invalid,
        Status::Failed | Status::Cancelled => ExitCode::Failure,
    }
}

fn reject(document: Option<&TaskDocument>, rejection: &TaskRejection) -> Invocation {
    let record = match document {
        Some(task) => TerminalRecord::invalid_with_id(task.task_id.clone(), rejection.code),
        None => TerminalRecord::invalid(rejection.code),
    };
    Invocation::new(record, ExitCode::Invalid, vec![rejection.reason.clone()])
}

/// Execute one validated task document to a terminal record.
pub async fn invoke(
    config: &InvocationConfig,
    document: TaskDocument,
    cancel: &mut watch::Receiver<bool>,
) -> Invocation {
    invoke_with_probe(config, document, cancel, &PathProbe).await
}

/// Execute one validated task document, with an injectable executable probe.
pub async fn invoke_with_probe(
    config: &InvocationConfig,
    document: TaskDocument,
    cancel: &mut watch::Receiver<bool>,
    probe: &dyn CommandProbe,
) -> Invocation {
    let task_id = document.task_id.clone();

    // The mis-execution guard: a task addressed to a different runner is refused
    // before anything is spawned. This prevents accidental execution by the wrong
    // agent; it makes no claim about who authored the document.
    if let Some(configured) = &config.configured_runner
        && *configured != document.agent
    {
        return reject(
            Some(&document),
            &TaskRejection::new(
                code::RUNNER_MISMATCH,
                "task addresses a different runner than this process was launched as",
            ),
        );
    }

    let definition = match config.catalog.resolve(&document.agent) {
        Ok(definition) => definition,
        Err(error) => {
            return reject(
                Some(&document),
                &TaskRejection::new(code::INVALID_RUNNER, error.to_string()),
            );
        }
    };

    let report = availability::probe(definition, probe);
    let Some(command) = report.resolved.clone() else {
        let mut diagnostics = vec![format!(
            "runner {} has no resolvable executable",
            definition.id
        )];
        if let Some(hint) = &report.install_hint {
            diagnostics.push(format!("install hint: {hint}"));
        }
        if let Some(url) = &report.install_instructions_url {
            diagnostics.push(format!("documentation: {url}"));
        }
        return Invocation::new(
            TerminalRecord::failed(task_id, code::AGENT_SPAWN_FAILED),
            ExitCode::Failure,
            diagnostics,
        );
    };

    let plan: EnvironmentPlan = environment::plan(&config.inherited_env, &definition.env, &[]);
    for stripped in &plan.stripped {
        eprintln!("lapis-acp: stripped reserved environment key {stripped}");
    }
    let harness_env = environment::harness_variables(Some(&document.task_id), &definition.id);
    let spec = LaunchSpec {
        command,
        environment: plan,
        harness_env,
        cwd: Some(config.cwd.clone()),
    };

    let mut child = match lapis_acp_runner::spawn(&spec) {
        Ok(child) => child,
        Err(error) => {
            return Invocation::new(
                TerminalRecord::failed(task_id, code::AGENT_SPAWN_FAILED),
                ExitCode::Failure,
                // The executable name is safe to report; its environment is not.
                vec![format!(
                    "cannot spawn {}: {error}",
                    lapis_acp_runner::describe(&spec.command)
                )],
            );
        }
    };

    let stdout = match child.stdout.take() {
        Some(stdout) => stdout,
        None => {
            return Invocation::new(
                TerminalRecord::failed(task_id, code::PROTOCOL_ERROR),
                ExitCode::Failure,
                vec!["agent stdout was unavailable".to_owned()],
            );
        }
    };
    let stdin = child.stdin.take().expect("stdin was piped");
    let mut agent = Agent::new(Connection::new(stdout, stdin), ProcessGuard::new(child));

    // The shorter of the requested cap and the trusted cap always wins, so a task
    // document can lower the bound but never raise it.
    let absolute = requested_cap(&document).min(config.absolute_cap);
    let request = TurnRequest {
        prompt: std::slice::from_ref(&document.prompt),
        cwd: &config.cwd.to_string_lossy(),
        system_prompt: config.system_prompt.as_deref().map(|text| {
            lapis_acp_protocol::session::SystemPrompt {
                transport: config.prompt_transport,
                text,
            }
        }),
        session_title: None,
        mcp_servers: Vec::new(),
        budget: TurnBudget::new(config.idle, absolute),
        cancel_grace: config.cancel_grace,
        model: config.model.as_deref(),
    };

    let mut handler = RecordingHandler::default();
    let outcome = run_turn(&mut agent, &request, &mut handler, cancel).await;

    // Offer the agent an orderly exit before falling back to a bounded kill.
    // Flushing is not enough: closing a pipe write end is what produces the end
    // of input a well-behaved agent waits for, and that happens when the handle
    // is dropped. The connection is therefore released before the grace window
    // starts, and only the process guard is kept.
    agent.close().await;
    let Agent {
        connection,
        mut guard,
    } = agent;
    drop(connection);
    let termination = guard.terminate(config.cancel_grace, config.reap).await;

    let mut diagnostics = Vec::new();
    if !termination.confirmed_stopped() {
        diagnostics
            .push("agent process was not confirmed stopped; cleanup is best-effort".to_owned());
    } else if termination.group_killed {
        diagnostics.push("agent process group was killed after the grace window".to_owned());
    }
    if let Some(failure) = &outcome.failure {
        // Untrusted agent text stays on stderr and never reaches the record.
        diagnostics.push(format!("{:?}: {}", failure.kind, failure.detail));
    }

    let cancelled = *cancel.borrow();
    let record = map_outcome(&task_id, &outcome, cancelled);
    let mut invocation = Invocation::new(record.clone(), exit_for(&record), diagnostics);
    invocation.cancelled = cancelled;
    let _ = handler;
    invocation
}

/// Reduce an observed turn outcome to one sanitized record.
fn map_outcome(task_id: &str, outcome: &TurnOutcome, cancelled: bool) -> TerminalRecord {
    if cancelled {
        return TerminalRecord::cancelled(Some(task_id.to_owned()));
    }
    match outcome.status {
        TurnStatus::Completed => match outcome.stop_reason {
            Some(reason) => TerminalRecord::completed(task_id, reason),
            None => TerminalRecord::failed(task_id, code::PROTOCOL_ERROR),
        },
        TurnStatus::Cancelled => match outcome.stop_reason {
            Some(reason) => TerminalRecord::completed(task_id, reason),
            None => TerminalRecord::cancelled(Some(task_id.to_owned())),
        },
        TurnStatus::TimedOut => TerminalRecord::timed_out(
            Some(task_id.to_owned()),
            match outcome.timeout {
                Some(lapis_acp_runner::TimeoutKind::Idle) => record_code::IDLE_TIMEOUT,
                _ => record_code::DEADLINE_EXCEEDED,
            },
        ),
        TurnStatus::Failed => TerminalRecord::failed(
            task_id,
            match outcome.failure.as_ref().map(|f| f.kind) {
                Some(FailureKind::Spawn) => code::AGENT_SPAWN_FAILED,
                Some(FailureKind::AgentExited) => code::AGENT_EXITED,
                Some(FailureKind::AgentError) => code::AGENT_ERROR,
                Some(FailureKind::WriteTimeout) => code::WRITE_TIMEOUT,
                _ => code::PROTOCOL_ERROR,
            },
        ),
    }
}
