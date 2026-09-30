# Architecture

`lapis-acp` is a Cargo workspace of layered crates. Each layer owns one concern
and may depend only on the layers beneath it. The controlling application sits
outside the workspace and owns everything `lapis-acp` deliberately does not:
identity, authorization, durable state, repository authority and prompt policy.

```
controlling app (jj-store, a script, an operator)
        │  versioned task document / control frames, newline-delimited JSON
        ▼
lapis-acp-cli ── lapis-acp-control ── lapis-acp-runner ── lapis-acp-agent
                                            │                   │
                                            └── lapis-acp-protocol ──┘
                                                        │ stdio JSON-RPC
                                                        ▼
                                                 ACP agent subprocess
```

The layering is not advisory. `scripts/check-boundaries.mjs` reads
`cargo metadata --no-deps --locked` and fails on an edge that is not in the
declared matrix, on any crate absent from the matrix, and on any excluded
dependency. The excluded set encodes the port's deliberate omissions, so a
future re-port that reintroduces Nostr, a transport, a TLS stack or a database
fails the gate instead of review.

## Requirements

| ID           | Requirement                                                                                                                                                                |
| ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| LAC-ARCH-001 | The repository MUST be a Cargo workspace of layered crates whose internal dependencies match the matrix enforced by `scripts/check-boundaries.mjs`.                        |
| LAC-ARCH-002 | `lapis-acp` MUST NOT depend on Nostr, transport, HTTP, TLS or database crates, and the boundary checker MUST fail closed when one appears.                                 |
| LAC-ARCH-003 | Production sources MUST stay at or below 300 counted lines and test sources at or below 500, with exceptions only as explicit baseline entries.                            |
| LAC-ARCH-004 | Production modules MUST NOT carry inline test attributes or import the testkit crate; unit tests use a `tests.rs` sibling and integration tests a crate `tests` directory. |
| LAC-ARCH-005 | The controlling application MUST own runner selection, working directory, credentials and prompt content; `lapis-acp` MUST NOT derive them on its own.                     |
| LAC-ARCH-006 | A crate MUST own its behaviour; the CLI crate MUST compose and MUST NOT reimplement protocol, scheduling or transport logic.                                               |

## Crate ownership

| Crate                | Owns                                                                                                          | Must not know                      |
| -------------------- | ------------------------------------------------------------------------------------------------------------- | ---------------------------------- |
| `lapis-acp-protocol` | Frame codec, correlation, capability negotiation, session methods, stop reasons, typed ACP payloads           | Processes, scheduling, credentials |
| `lapis-acp-agent`    | Runner definitions, catalog tiers, executable resolution, launch environment merge and reserved-key stripping | Turn lifecycle, transport          |
| `lapis-acp-runner`   | Subprocess launch, process-group ownership, session and turn lifecycle, deadlines, cancellation, cleanup      | How work arrives                   |
| `lapis-acp-control`  | Task document, terminal record, control transport abstraction and the stdio transport                         | ACP framing details                |
| `lapis-acp-cli`      | Argument parsing, launch configuration assembly, exit codes                                                   | Any behaviour it can delegate      |
| `lapis-acp-testkit`  | Deterministic ACP peer fixtures and failure modes for tests                                                   | Production use of any kind         |

## Counted-line budget

The budget in LAC-ARCH-003 is a deliberate divergence from jj-store, which
budgets only web and script sources and carries Rust files past a thousand
lines. Upstream `buzz-acp` concentrates its ACP client, pool, queue and relay in
single files of two hundred kilobytes or more. Porting those files intact would
make every future upstream comparison a whole-file diff, so decomposition is a
portability requirement rather than a style preference.

`scripts/rust-architecture.mjs` measures counted lines with blanks and comments
excluded, classifies each file as production, test or script, and compares
against `scripts/rust-architecture-baseline.json`. The baseline ships empty. An
entry may be added only when an extraction cannot lower a budget, and the gate
reports a stale entry whose file no longer exists, so the baseline cannot
quietly retain dead exceptions.

## Repository tooling

Validation is first-party Node automation under `scripts/`, not a crate. It sits
outside the Cargo workspace on purpose: the gates must be able to inspect the
workspace as data, read `cargo metadata`, and fail on a crate that does not
exist yet. Each checker exports its decision function so a governance test can
exercise it against synthetic input, and each has a thin command-line entry so a
lane can invoke it directly.

| Script                               | Owns                                                         |
| ------------------------------------ | ------------------------------------------------------------ |
| `scripts/check-boundaries.mjs`       | Crate layer matrix, excluded dependencies, testkit isolation |
| `scripts/rust-architecture.mjs`      | Counted-line budget, test placement, testkit imports         |
| `scripts/check-progress.mjs`         | Ledger parsing, evidence integrity, completion gates         |
| `scripts/upstream-lock.mjs`          | Port map and lock validation, digests, drift classification  |
| `scripts/upstream-sync.mjs`          | Pin self-test and the opt-in networked materialization       |
| `scripts/governance.test.mjs`        | Ledger and boundary regressions                              |
| `scripts/rust-architecture.test.mjs` | Budget and placement regressions                             |
| `scripts/upstream-lock.test.mjs`     | Port map, lock and drift regressions                         |

Tooling changes are protected and mapped to this chapter's governance owner, so
a gate cannot be weakened without a same-change specification edit. Adding a
checker means adding its decision function, its tests and its lane together.
