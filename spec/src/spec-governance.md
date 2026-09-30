# Specification governance

Use the parent workspace test escalation ladder. Requirement changes belong here
before or with implementation. Generated book output is ignored and
non-normative. Only `milestones.md` carries implementation status; evidence
entries hold immutable records rather than competing status summaries. Focused
test commands may prove a requested slice without running unrelated suites when
the user narrows test scope, but every directly affected boundary still needs
named evidence.

`lapis-acp` borrows the jj-store governance regime wholesale: canonical mdBook
chapters, unique requirement IDs with exactly one verification row each, a
single progress ledger, mapped spec-first enforcement, pinned toolchain and
Jujutsu commits per verified slice. It deliberately adds one gate jj-store does
not have for Rust, recorded as LAC-GOV-007, because the upstream files this
repository ports are far larger than jj-store's Rust budget would allow.

## Requirements

| ID          | Requirement                                                                                                                                                           |
| ----------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| LAC-GOV-001 | The canonical specification MUST live under `spec/src`, build with mdBook, and assign unique `LAC-<AREA>-NNN` requirement IDs with exactly one verification row each. |
| LAC-GOV-002 | Protected implementation, interface, tooling and agent-guidance changes MUST update their mapped canonical chapter in the same Jujutsu change.                        |
| LAC-GOV-003 | Every implementation slice MUST update `milestones.md` with actual commands, results, failures, affected files, versions, formats, gaps and next action.              |
| LAC-GOV-004 | Progress checks MUST validate task and dependency references, evidence links and named tests, completion gates and same-change ledger updates.                        |
| LAC-GOV-005 | Verified slices MUST be committed with Jujutsu while unrelated work and local workspace configuration remain uncommitted.                                             |
| LAC-GOV-006 | Rust formatting MUST use the pinned toolchain component and tracked `rustfmt.toml`, and MUST be checked by both normal and specification checks.                      |
| LAC-GOV-007 | Rust and first-party script sources MUST stay within the recorded counted-line budget, and exceptions MUST be explicit baseline entries that do not grow.             |

## Change map

| Protected area                                                                                                    | Required chapter     |
| ----------------------------------------------------------------------------------------------------------------- | -------------------- |
| `^crates/lapis-acp-protocol/`                                                                                     | `protocol.md`        |
| `^crates/lapis-acp-agent/`                                                                                        | `agents.md`          |
| `^crates/lapis-acp-runner/`                                                                                       | `runner.md`          |
| `^crates/lapis-acp-control/`                                                                                      | `control.md`         |
| `^crates/lapis-acp-cli/`                                                                                          | `cli.md`             |
| `^crates/lapis-acp-testkit/`                                                                                      | `compatibility.md`   |
| `^(?:tests/\|docs/)`                                                                                              | `compatibility.md`   |
| `^(?:scripts/\|AGENTS\.md$\|spec-validator\.config\.mjs$\|spec/book\.toml$\|\.gitignore$\|\.prettier)`            | `spec-governance.md` |
| `^rustfmt\.toml$`                                                                                                 | `spec-governance.md` |
| `^(?:Cargo\.(?:toml\|lock)$\|rust-toolchain\.toml$\|package\.json$\|pnpm-lock\.yaml$\|README\.md$\|LICENSE\.md$)` | `architecture.md`    |

A protected path that matches no rule above fails closed as unmapped rather than
passing silently. A path that matches more than one rule requires every mapped
chapter in the same change, so the rules stay disjoint on purpose.

Passing bookkeeping MUST NOT substitute for passing product tests. A ledger
entry records the commands actually run and their real failure counts; it never
converts a fixture-peer result into a real-runner qualification or a compile
into a behavioral proof.
