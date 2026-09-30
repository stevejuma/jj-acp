//! Pluggable ACP runner catalog and launch environment.
//!
//! Owns runner definitions (identifier, label, command, arguments, environment
//! floor, MCP transport, install hint), catalog tiering, executable resolution
//! and the environment merge that a spawned agent subprocess receives.
//!
//! Adding a runner MUST be a data change, never a code change: no install shell
//! commands, no network fetches and no credentials. Reserved environment keys
//! are stripped before a definition or user override is merged, so a runner
//! definition cannot claim harness identity.
//!
//! This crate MUST NOT spawn processes, schedule turns or open sockets.
