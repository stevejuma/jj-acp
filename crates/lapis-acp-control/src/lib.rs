//! Controlling application boundary.
//!
//! Owns the versioned task document, the terminal record emitted for every
//! attempted invocation, the control transport abstraction and the stdio
//! newline-delimited JSON transport that lets a controlling application drive
//! `lapis-acp` without sockets, ports, tokens or any other authentication.
//!
//! This crate replaces the upstream Nostr relay intake wholesale. The
//! controlling application is the trust boundary: it selects the runner, the
//! working directory, the credentials environment and the prompt. `lapis-acp`
//! performs no author gating, no membership discovery and no identity checks
//! beyond refusing to execute a task addressed to a different runner.
//!
//! Diagnostics MUST stay on stderr. Stdout carries control frames and terminal
//! records only, so a controlling application can parse it unconditionally.
