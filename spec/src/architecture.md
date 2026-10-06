# Architecture

`jj-acp` is a Cargo workspace of layered crates. Each layer owns one concern and
may depend only on the layers beneath it. The controlling application sits
outside the workspace and owns everything `jj-acp` deliberately does not:
identity, authorization, durable state, repository authority and prompt policy.

```
controlling app (jj-store, a script, an operator)
        │  versioned task document / control frames, newline-delimited JSON
        ▼
jj-acp-cli ── jj-acp-control ── jj-acp-runner ── jj-acp-agent
                                            │                   │
                                            └── jj-acp-protocol ──┘
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

| ID           | Requirement                                                                                                                                                                                     |
| ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JAC-ARCH-001 | The repository MUST be a Cargo workspace of layered crates whose internal dependencies match the matrix enforced by `scripts/check-boundaries.mjs`.                                             |
| JAC-ARCH-002 | `jj-acp` MUST NOT depend on Nostr, transport, HTTP, TLS or database crates, and the boundary checker MUST fail closed when one appears.                                                         |
| JAC-ARCH-003 | Production sources MUST stay at or below 300 counted lines and test sources at or below 500, with exceptions only as explicit baseline entries.                                                 |
| JAC-ARCH-004 | Production modules MUST NOT define test functions inline or import the testkit crate; unit tests MUST use a `tests.rs` sibling module and integration tests MUST use a crate `tests` directory. |
| JAC-ARCH-005 | The controlling application MUST own runner selection, working directory, credentials and prompt content; `jj-acp` MUST NOT derive them on its own.                                             |
| JAC-ARCH-006 | A crate MUST own its behaviour; the CLI crate MUST compose and MUST NOT reimplement protocol, scheduling or transport logic.                                                                    |

## Crate ownership

| Crate             | Owns                                                                                                          | Must not know                      |
| ----------------- | ------------------------------------------------------------------------------------------------------------- | ---------------------------------- |
| `jj-acp-protocol` | Frame codec, correlation, capability negotiation, session methods, stop reasons, typed ACP payloads           | Processes, scheduling, credentials |
| `jj-acp-agent`    | Runner definitions, catalog tiers, executable resolution, launch environment merge and reserved-key stripping | Turn lifecycle, transport          |
| `jj-acp-runner`   | Subprocess launch, process-group ownership, session and turn lifecycle, deadlines, cancellation, cleanup      | How work arrives                   |
| `jj-acp-control`  | Task document, terminal record, control transport abstraction and the stdio transport                         | ACP framing details                |
| `jj-acp-cli`      | Argument parsing, launch configuration assembly, exit codes                                                   | Any behaviour it can delegate      |
| `jj-acp-testkit`  | Deterministic ACP peer fixtures and failure modes for tests                                                   | Production use of any kind         |

## Counted-line budget

The budget in JAC-ARCH-003 is a deliberate divergence from jj-store, which
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

## Dependency declaration

Third-party versions are declared once in the workspace dependency table and
referenced by name from each crate, so a version change is one reviewed edit
rather than a sweep across manifests. `Cargo.lock` is committed and every cargo
invocation in a check lane passes `--locked`.

Each crate enables only the facilities it owns. `jj-acp-protocol` takes
`io-util`, `sync` and `time` from the async runtime and nothing else, because it
speaks over a caller-supplied byte stream and has no business spawning a process
or opening a socket. `jj-acp-agent` takes no async runtime at all: resolving a
runner is a synchronous filesystem question. Process ownership, signal handling
and scheduling arrive with the runner and CLI crates, which is where the
corresponding facilities are enabled.

That split is enforced rather than documented. A governance test reads the
protocol crate's manifest and fails if a process or network facility appears in
it, and the boundary checker fails on any crate absent from the layer matrix and
on any excluded dependency, so a crate cannot quietly acquire a capability its
layer does not own.

## Layer status

All six crates carry implementation except where the ledger says otherwise. The
protocol, agent, runner, control and CLI crates are populated; the testkit crate
owns fixture location, launch configuration and log parsing. The dependency
matrix in `scripts/check-boundaries.mjs` is unchanged from the bootstrap slice,
which is the point: filling five crates with real code required no new layer
edge, so the original decomposition was adequate rather than merely tidy.

The one dependency the matrix does express and the bootstrap could not exercise
is the testkit's. It depends on the protocol and agent crates because it
configures a fixture through the same types a production caller uses, and it
must never depend on the runner, control or CLI crates, or a test helper would
become a composition path around them.

The layering held under implementation without amendment. Five crates were
filled with real code across four slices and the dependency matrix recorded at
bootstrap needed no new edge, which is the evidence that the decomposition was
adequate rather than merely tidy. The one refinement was to the testkit's own
row: it depends on the protocol and agent crates because it configures a fixture
through the same types a production caller uses, and it must never depend on the
runner, control or CLI crates, or a test helper would become a composition path
around them.

Qualifying a real adapter exercised every layer at once and found three defects
that no fixture test had reached, in the protocol crate's capability probing,
the CLI's subcommand option handling and the compiled-in runner declarations.
That is the argument for keeping the real-runner gate separate from the fixture
suite rather than treating the fixture as sufficient: the fixture proves this
code agrees with itself, and only a shipping adapter proves it agrees with
anyone else.

## Repository identity

The repository, its crates, its binary and its requirement identifiers are named
`jj-acp` and `JAC-`, after the repository it exists to serve. The npm scope is
unchanged: `@lapismd/jj-acp` sits alongside `@lapismd/jj-store`, and the crate
layer names keep their `acp-<layer>` suffixes so the owning chapter of any crate
is readable from its path alone.

Renaming is a governance event rather than a search-and-replace, because three
contracts travel with the name. The launch-option environment namespace moved to
`JJ_ACP_*`, the harness-reserved task identity to `JJ_ACP_TASK_*`, and the
requirement identifier prefix to `JAC-`. All three are read by other programs: a
controlling application that set `LAPIS_ACP_MODEL` would silently get the
default instead, which is why the rename belongs in one atomic change with its
ledger entry rather than being spread across slices.

## Interactive session integration

Interactive control version 2 is implemented within the existing protocol,
runner and control layers. It adds no network, database or identity dependency
and retains the one-shot interface.
