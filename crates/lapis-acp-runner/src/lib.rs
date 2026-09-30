//! ACP session and turn lifecycle.
//!
//! Owns agent subprocess launch with process-group ownership, `initialize`
//! handshake, session creation and rotation, single-in-flight turn execution
//! per scope, idle and absolute turn deadlines, cooperative cancellation,
//! bounded kill and reap, stop-reason capture and crash respawn.
//!
//! This crate MUST NOT know how work arrives. Intake, batching and controlling
//! app transport belong to `lapis-acp-control`; this crate executes one prepared
//! turn at a time and reports a terminal outcome.
//!
//! Cancellation is bounded and cleanup is best-effort: a killed harness or an
//! unresponsive descendant cannot be guaranteed to stop. Timeouts are unknown
//! outcomes, never proven success or proven failure.
