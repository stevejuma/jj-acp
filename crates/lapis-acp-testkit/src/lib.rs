//! Deterministic ACP peer fixtures for integration tests.
//!
//! Owns the scripted agent subprocess that answers `initialize`, `session/new`,
//! `session/prompt`, `session/set_config_option` and permission requests with
//! canned frames, plus the hang, refusal and descendant-spawn modes that prove
//! deadline, cancellation and process-group cleanup behaviour.
//!
//! Tests MUST reach a terminal outcome without a model provider, a network
//! endpoint or a credential. This crate is test infrastructure: no production
//! crate may depend on it, and `scripts/check-boundaries.mjs` fails closed when
//! one does.
