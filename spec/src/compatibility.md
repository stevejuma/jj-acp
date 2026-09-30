# Compatibility and upstream port

`lapis-acp` is a derived work. Most of its design and a substantial part of its
logic come from the upstream `buzz-acp` crate in the `block/buzz` repository,
which is Apache-2.0. This chapter governs how that relationship stays auditable
so the port can be updated rather than rewritten when upstream moves.

Upstream source is never committed here. It is pinned by commit, materialized
into ignored scratch storage on demand, verified against recorded digests and
compared through a per-file disposition map. References are not runtime
dependencies.

## Requirements

| ID             | Requirement                                                                                                                                                           |
| -------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| LAC-COMPAT-001 | The upstream port source MUST be pinned by repository, path and commit in `docs/upstream-lock.md` together with a per-file disposition map.                           |
| LAC-COMPAT-002 | Adopted upstream attribution MUST be recorded in `docs/licenses/NOTICE.md`, and upstream repository-level agent instructions MUST NOT be copied into this repository. |
| LAC-COMPAT-003 | Upstream drift MUST be detectable by a script that materializes the pinned source into ignored scratch storage and verifies recorded digests.                         |
| LAC-COMPAT-004 | The Rust toolchain MUST be pinned and formatting MUST be checked by both the normal and the specification check lanes.                                                |
| LAC-COMPAT-005 | Integration tests MUST reach a terminal outcome without a model provider, a network endpoint or a credential.                                                         |
| LAC-COMPAT-006 | Qualification against a real installed runner MUST be recorded separately and MUST NOT be inferred from fixture-peer tests.                                           |
| LAC-COMPAT-007 | Every disposition in the port map MUST be one of ported, ported-with-changes, dropped or not-ported, and a dropped region MUST name the reason.                       |

## Upstream lock

`docs/upstream-lock.md` records the pinned commit, the crate path, the license
and a per-region disposition. Because upstream concentrates logic in very large
files and this repository decomposes them under a counted-line budget, the map
is recorded per region rather than per file: a whole-file mapping would make
every upstream comparison a diff against a file that no longer exists here.

Each region names the owning `lapis-acp` module so a re-port has a target. A
region marked dropped names the reason, which for this port is almost always
that it is Nostr intake, identity, signing or relay publication. A region marked
not-ported is a deliberate deferral with a ledger task, not an oversight.

## Disposition summary

| Upstream region                                       | Disposition                                             |
| ----------------------------------------------------- | ------------------------------------------------------- |
| ACP client: framing, correlation, session methods     | ported-with-changes                                     |
| Turn pool, queue, scoping, prompt framing             | ported-with-changes                                     |
| Shared runtime, prompt context, isolated execution    | ported-with-changes                                     |
| One-shot task runner and task document contract       | ported-with-changes                                     |
| Deterministic ACP peer fixture                        | ported-with-changes                                     |
| Relay intake, subscriptions, recovery, REST discovery | dropped: Nostr intake                                   |
| Author gate, owner resolution, workflow provenance    | dropped: Nostr identity                                 |
| Git signing, credential helpers, ephemeral keyfile    | dropped: Nostr signing                                  |
| Memory fetch, observer publication, usage publication | dropped: relay publication                              |
| Setup and onboarding mode                             | dropped: relay onboarding                               |
| Base prompt and session model prose                   | ported-with-changes: rewritten without relay references |

## Toolchain and dependency pins

The toolchain is pinned by `rust-toolchain.toml` with the cargo, clippy and
rustfmt components, so formatting and lint results are reproducible and the
pinned component is what the check lanes invoke. `Cargo.lock` is committed and
every cargo invocation in a check lane passes `--locked`, so a dependency change
is an explicit reviewed edit rather than a silent resolution.

Third-party crates come from the registry under normal semver ranges. Nothing is
vendored as committed source, and no dependency is patched, aliased or shimmed
to work around a defect; a defect is fixed in its owning repository.

## Test qualification

Two levels of qualification are kept distinct and neither substitutes for the
other.

The fixture peer is a deterministic scripted ACP agent. It answers the protocol
methods with canned frames and can be told to hang at startup, hang during
session creation, refuse, exceed a deadline or spawn a long-lived descendant. It
proves framing, deadlines, cancellation, cleanup, sanitization and exit codes
with no model provider, no network and no credential, which is why every
automated lane uses it.

The real installed runner is an actual agent adapter on the operator's machine.
It proves that the ported protocol surface matches what a shipping adapter
actually speaks, including capability negotiation and configuration options the
fixture only approximates. That qualification is recorded as its own ledger task
with its own evidence kind, and the progress checker refuses to mark it done on
fixture evidence. A passing fixture suite is not a claim about any real agent.
