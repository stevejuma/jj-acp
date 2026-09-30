//! The compiled-in runner tier.
//!
//! These identifiers are reserved: a catalog file or a caller-supplied
//! definition cannot shadow one, so an operator always knows what `codex` means.
//! The adapter package names come from the published ACP adapter set; the exact
//! entrypoint of each is still subject to the real installed-runner
//! qualification recorded in the ledger, and an unqualified default is a
//! convenience rather than a claim.

use crate::definition::{CommandSpec, McpTransport, RunnerDefinition};
use crate::error::Result;

const ACP_DOCUMENTATION: &str = "https://agentclientprotocol.com/";

/// Identifiers reserved for the compiled-in tier.
pub const RESERVED_IDS: [&str; 4] = ["codex", "claude", "opencode", "goose"];

/// Report whether an identifier is reserved for the compiled-in tier.
pub fn is_reserved(id: &str) -> bool {
    RESERVED_IDS.contains(&id)
}

/// The Codex adapter.
pub fn codex() -> Result<RunnerDefinition> {
    Ok(RunnerDefinition::try_from_parts(
        "codex".to_owned(),
        "Codex".to_owned(),
        vec![CommandSpec::new("codex-acp")],
        Vec::new(),
        McpTransport::Stdio,
    )?
    .with_install(
        "npm install -g @agentclientprotocol/codex-acp",
        ACP_DOCUMENTATION,
    ))
}

/// The Claude adapter, accepting both current and legacy command names.
///
/// Both names describe the same zero-argument runtime, so they are candidates on
/// one definition rather than two runners. An operator with an older install
/// still resolves `claude`.
pub fn claude() -> Result<RunnerDefinition> {
    Ok(RunnerDefinition::try_from_parts(
        "claude".to_owned(),
        "Claude Code".to_owned(),
        vec![
            CommandSpec::new("claude-agent-acp"),
            CommandSpec::new("claude-code-acp"),
        ],
        Vec::new(),
        McpTransport::Stdio,
    )?
    .with_install(
        "npm install -g @agentclientprotocol/claude-agent-acp",
        ACP_DOCUMENTATION,
    ))
}

/// The OpenCode adapter.
pub fn opencode() -> Result<RunnerDefinition> {
    Ok(RunnerDefinition::try_from_parts(
        "opencode".to_owned(),
        "OpenCode".to_owned(),
        vec![
            CommandSpec::with_args("opencode", &["acp"]),
            CommandSpec::new("opencode"),
        ],
        Vec::new(),
        McpTransport::Stdio,
    )?
    .with_install("see the OpenCode documentation", ACP_DOCUMENTATION))
}

/// The Goose adapter.
pub fn goose() -> Result<RunnerDefinition> {
    Ok(RunnerDefinition::try_from_parts(
        "goose".to_owned(),
        "Goose".to_owned(),
        vec![CommandSpec::with_args("goose", &["acp"])],
        Vec::new(),
        McpTransport::Stdio,
    )?
    .with_install("see the Goose documentation", ACP_DOCUMENTATION))
}

/// Every compiled-in runner, in display order.
pub fn built_in() -> Result<Vec<RunnerDefinition>> {
    Ok(vec![codex()?, claude()?, opencode()?, goose()?])
}
