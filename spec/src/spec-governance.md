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

## Check lanes

`spec-validator.config.mjs` declares the lanes that `pnpm spec:check` runs, and
`package.json` exposes the same checks as individual scripts so a narrowed slice
can run one of them. The lanes are rustfmt, prettier, crate boundaries, the
counted-line budget, the progress ledger, the upstream pin self-test, the
governance and lock test suites, and the cargo workspace tests.

The upstream pin lane is offline by design.
`scripts/upstream-sync.mjs --self-test` validates the curated port map and
reconciles it with the generated lock without touching the network, so a normal
check run never depends on upstream availability. The two networked modes,
`--write` and `--fetch`, are explicit opt-in lanes reached through
`pnpm upstream:write` and `pnpm upstream:fetch`. This mirrors the sibling
jj-store split between a harness self-test in the normal lane and a real-storage
qualification that stays opt-in, and it preserves the rule that a self-test is
not conformance evidence: passing `--self-test` proves the record is internally
consistent, not that the pinned upstream source still matches it.

A lane that needs the network, a credential or a model provider MUST NOT be part
of the normal check run. It belongs in an explicitly named opt-in lane whose
absent evidence is recorded in the ledger as a gap rather than inferred as a
pass.

## Test placement and the counted-line gate

The counted-line gate classifies a source file as production, test or script
from its path, and a production file that defines a test function inline is
rejected. A `#[cfg(test)] mod tests;` declaration is accepted, because it names
a sibling file rather than carrying test bodies; the bodies live in that sibling
or in the crate's `tests` directory. The distinction matters to the budget: an
inline test module inflates a production file's counted lines with content that
is not production, which would either force a spurious baseline exception or
push real logic out of the file it belongs in.

The gate exempts the testkit crate from the rule against importing the testkit,
because its own documentation examples legitimately name it. The rule exists to
keep production crates away from test helpers, not to stop a helper describing
itself, and a gate that fires on its own subject teaches reviewers to ignore it.

An over-budget file is extracted, never baselined. The baseline exists for a
file whose extraction genuinely cannot lower its budget, and a stale entry whose
file no longer exists is itself a failure, so exceptions cannot outlive their
justification. When the protocol crate's connection tests outgrew the test
budget, the response was to split them across three test files sharing one
harness module rather than to record an exception.

## Milestone shape

The ledger rejects a `Done` task that has an incomplete child, because a
`<ID>.<N>` suffix asserts that the child is part of the parent's gate rather
than work that follows it. Follow-on qualification is therefore a sibling
milestone with a dependency on the one it follows, not a sub-milestone.

That distinction was learned by being blocked rather than by anticipating it.
The one-shot task runner was originally recorded with process-group containment
and real installed-runner qualification as its children, which made the runner
impossible to call done until both were also done, even though its own gate was
complete and independently evidenced. Renumbering them as siblings with
dependencies preserved the ordering without conflating a delivered gate with the
hardening that follows it.

One task carries a special evidence rule: the real installed-runner
qualification cannot be marked done on evidence whose kind is anything other
than a real runner run. A fixture-peer suite is deterministic and model-free by
design, and that is exactly why it cannot prove interoperability with a shipping
adapter.
