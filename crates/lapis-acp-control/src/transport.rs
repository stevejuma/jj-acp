//! The control transport abstraction.
//!
//! One implementation ships in this slice: newline-delimited JSON over the
//! inherited stdin and stdout. A controlling application that wants a Unix
//! socket, a local HTTP endpoint or an in-process link implements this trait
//! rather than editing the runner, which is why the one-shot task runner and a
//! future persistent service can share one session and turn machinery.
//!
//! No transport may add authentication. There is no peer to authenticate: the
//! process that spawned this one already decided to run it with this environment.

use std::io;

use crate::record::TerminalRecord;
use crate::task::TaskDocument;

/// Carries task documents in and terminal records out.
pub trait ControlTransport {
    /// Read the next prepared task, or `None` when the caller is finished.
    fn next_task(
        &mut self,
    ) -> impl std::future::Future<Output = io::Result<Option<TaskDocument>>> + Send;

    /// Emit exactly one terminal record for one attempted invocation.
    fn emit(
        &mut self,
        record: &TerminalRecord,
    ) -> impl std::future::Future<Output = io::Result<()>> + Send;

    /// Write an operator diagnostic. Implementations must route this away from
    /// the channel `emit` uses.
    fn diagnose(
        &mut self,
        message: &str,
    ) -> impl std::future::Future<Output = io::Result<()>> + Send;
}

/// The stdio transport: documents in on stdin, records out on stdout.
///
/// Stderr carries diagnostics so stdout stays unconditionally parseable. A
/// controlling application that mixes its own output into this process's stdout
/// breaks that guarantee, which is why the CLI never writes anything else there.
#[derive(Debug, Default)]
pub struct StdioTransport;

impl StdioTransport {
    /// Build the stdio transport.
    pub fn new() -> Self {
        Self
    }
}

impl ControlTransport for StdioTransport {
    async fn next_task(&mut self) -> io::Result<Option<TaskDocument>> {
        match crate::input::read_stdin().await {
            Ok(document) => Ok(Some(document)),
            // A rejected document is not an I/O failure; the caller reports the
            // rejection through `emit` and continues or exits.
            Err(rejection) => Err(io::Error::other(rejection.reason)),
        }
    }

    async fn emit(&mut self, record: &TerminalRecord) -> io::Result<()> {
        use tokio::io::{AsyncWriteExt, Stdout, stdout};
        let mut handle: Stdout = stdout();
        handle.write_all(record.to_line().as_bytes()).await?;
        handle.write_all(b"\n").await?;
        handle.flush().await
    }

    async fn diagnose(&mut self, message: &str) -> io::Result<()> {
        use tokio::io::{AsyncWriteExt, Stderr, stderr};
        let mut handle: Stderr = stderr();
        handle.write_all(message.as_bytes()).await?;
        handle.write_all(b"\n").await?;
        handle.flush().await
    }
}
