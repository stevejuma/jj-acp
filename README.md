# jj-acp

A pluggable [Agent Client Protocol](https://agentclientprotocol.com/) harness
that runs coding agents on behalf of a controlling application.

`jj-acp` spawns an ACP-speaking agent subprocess, drives it over stdio JSON-RPC,
and reports one sanitized terminal record per invocation. It has no message bus,
no relay, no listener, no authentication and no durable state. The controlling
application owns identity, authorization, storage and prompt policy; `jj-acp`
owns the protocol, the runner catalog and the turn lifecycle.

```
controlling app ──task document──▶ jj-acp ──stdio JSON-RPC──▶ ACP agent
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

The one-shot runner and version-1 persistent scoped scheduler are implemented.
Interactive version 2 retains a session across turns and streams activity and
permission requests. The deterministic consumer suite qualifies its control
behavior; real interactive Codex/Cursor consumer acceptance remains outstanding.
Two installed adapters have separate read-only one-shot qualification.

```sh
jj-acp run --task task.json --catalog runners.json --runner codex
# {"version":1,"taskId":"task-001","status":"completed","stopReason":"end_turn"}
jj-acp agents   # one JSON object per line: id, label, availability, install hint
```

| Slice     | Scope                                                                 | Status |
| --------- | --------------------------------------------------------------------- | ------ |
| Bootstrap | Specification, ledger, crate skeleton, boundary and line-budget gates | Done   |
| M1        | Pinned upstream lock, attribution and drift detection                 | Done   |
| M2        | ACP framing, negotiation and session methods                          | Done   |
| M3        | Runner catalog, launch environment and deterministic fixture peer     | Done   |
| M4        | One-shot `run --task` with sanitized terminal record and exit codes   | Done   |
| M5        | Descendant containment, crash reporting and no re-dispatch            | Done   |
| M6        | Real installed-runner qualification against `cursor-agent acp`        | Done   |
| M7        | Model, reasoning-effort and execution-mode application                | Done   |
| M8        | Persistent control transport                                          | Done   |
| M9        | Scoped concurrent scheduler and bounded queues                        | Done   |
| M10       | Controlling-application consumer acceptance                           | Done   |
| M11       | Interactive version-2 session lifecycle                               | Done   |

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

## Interactive controlling application

`jj-acp serve --control-version 2 --runner codex --cwd /approved/workspace`
opens the explicit version-2 stdio interface. Send open, configure and prompt
frames; activity and permission frames arrive before the terminal record. The
parent sanitizes and persists content. The runner requires an applied execution
mode before prompting, and never retries a turn. See the controlling application
chapter for request fields and lifecycle semantics.
