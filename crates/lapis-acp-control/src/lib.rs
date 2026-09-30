//! Controlling application boundary.
//!
//! Owns the versioned task document, the single terminal record emitted per
//! attempted invocation, the published exit codes, bounded document input, and the
//! composition that turns a validated document into one executed turn.
//!
//! This crate replaces the upstream Nostr relay intake wholesale. The controlling
//! application is the trust boundary: it selects the runner, the working
//! directory, the credentials environment and the prompt. There is no author
//! gating, no membership discovery and no identity check beyond refusing a task
//! addressed to a different runner.
//!
//! Diagnostics stay on stderr. Stdout carries terminal records and, in a future
//! persistent mode, control frames, so a controlling application can parse it
//! unconditionally.

#![forbid(unsafe_code)]

pub mod input;
pub mod invocation;
pub mod record;
pub mod task;
pub mod transport;

pub use input::{TaskSource, read, read_bytes, read_path, read_stdin};
pub use invocation::{Invocation, InvocationConfig, invoke, invoke_with_probe};
pub use record::{ExitCode, RECORD_VERSION, Status, TerminalRecord};
pub use task::{TASK_VERSION, TaskDocument, TaskRejection, code, parse, parse_text};
pub use transport::{ControlTransport, StdioTransport};
