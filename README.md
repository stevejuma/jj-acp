# lapis-acp

A pluggable [Agent Client Protocol](https://agentclientprotocol.com/) harness
that runs coding agents on behalf of a controlling application.

`lapis-acp` spawns an ACP-speaking agent subprocess, drives it over stdio
JSON-RPC, and reports one sanitized terminal record per invocation. It has no
message bus, no relay, no listener, no authentication and no durable state. The
controlling application owns identity, authorization, storage and prompt policy;
`lapis-acp` owns the protocol, the runner catalog and the turn lifecycle.

```
controlling app ──task document──▶ lapis-acp ──stdio JSON-RPC──▶ ACP agent
       ▲                              │                              │
       └──────terminal record─────────┘                              │
                                             tools, files, side effects
```

## Provenance

This repository is a derived work of the `buzz-acp` crate in
[block/buzz](https://github.com/block/buzz), which is Apache-2.0. The ACP
client, turn pool, session scoping, prompt framing and one-shot task runner are
ported. The Nostr relay intake, author gating, owner resolution, key custody,
commit and tag signing, relay publication and onboarding are not ported in any
form. The pinned upstream commit and the per-region disposition of the port are
recorded in `docs/upstream-lock.md`, and attribution lives in
`docs/licenses/NOTICE.md`.

## Status

The one-shot task runner is implemented and exercised end to end against a
deterministic fixture agent: a prepared task document in, one sanitized terminal
record out, and a published exit code. What is not implemented is concurrency
and durability — one process runs exactly one turn — and no real installed
adapter has been qualified yet. The roadmap and the real evidence for each slice
live in the [progress ledger](spec/src/milestones.md).

```sh
lapis-acp run --task task.json --catalog runners.json --runner codex
# {"version":1,"taskId":"task-001","status":"completed","stopReason":"end_turn"}
lapis-acp agents   # one JSON object per line: id, label, availability, install hint
```

| Slice     | Scope                                                                 | Status   |
| --------- | --------------------------------------------------------------------- | -------- |
| Bootstrap | Specification, ledger, crate skeleton, boundary and line-budget gates | Done     |
| M1        | Pinned upstream lock, attribution and drift detection                 | Done     |
| M2        | ACP framing, negotiation and session methods                          | Done     |
| M3        | Runner catalog, launch environment and deterministic fixture peer     | Done     |
| M4        | One-shot `run --task` with sanitized terminal record and exit codes   | Done     |
| M5        | Descendant containment, crash reporting and no re-dispatch            | Done     |
| M6        | Real installed-runner qualification against `cursor-agent acp`        | Done     |
| M7        | Model, reasoning-effort and execution-mode application                | Planned  |
| M8        | Persistent control transport and standing session pool                | Deferred |
| M9        | Controlling-application consumer acceptance                           | Planned  |

## Requirements

Rust 1.98.1 (pinned by `rust-toolchain.toml`), Node 22 or newer and pnpm.
Integration tests need Python 3 for the deterministic fixture agent, and never
need a model provider, a network endpoint or a credential.

## Development

```sh
pnpm install
pnpm check          # spec validation, boundaries, line budget, ledger, fmt, clippy
pnpm test           # governance tests and cargo workspace tests
pnpm build          # cargo build --workspace --locked
pnpm spec:check     # canonical specification lanes
```

`pnpm format` runs `cargo fmt --all` and Prettier. Cargo work always passes
`--locked`; a dependency change is an explicit reviewed edit to `Cargo.lock`.

## Specification

Canonical behaviour lives in [`spec/src`](spec/src). Read
[architecture](spec/src/architecture.md),
[governance](spec/src/spec-governance.md) and
[verification](spec/src/verification.md) before implementing a slice. Build the
book with `pnpm spec:build`; generated output is ignored and non-normative.

## License

Apache-2.0. See [LICENSE.md](LICENSE.md).
