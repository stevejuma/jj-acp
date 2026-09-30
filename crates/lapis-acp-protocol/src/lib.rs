//! Agent Client Protocol wire contract.
//!
//! Owns the JSON-RPC framing, request/response correlation, capability
//! negotiation and session methods that `lapis-acp` speaks to an ACP agent
//! subprocess over stdio. This crate is the ported, decomposed derivative of
//! the upstream `buzz-acp` ACP client; the pin and per-file port map live in
//! `docs/upstream-lock.md`.
//!
//! This crate MUST stay free of transport policy, agent discovery, session
//! scheduling, credentials and network clients. It defines frames and typed
//! payloads only, so the owning layers can evolve independently and a future
//! upstream re-port stays mechanical.
