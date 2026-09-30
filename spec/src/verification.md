# Verification

Planned rows name a gate, not completed evidence. Mark Implemented only after
passing concrete tests linked from the progress ledger.

| ID             | Status      | Evidence                                                                                                                                             |
| -------------- | ----------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| LAC-ARCH-001   | Implemented | `scripts/check-boundaries.mjs`; `scripts/governance.test.mjs`; [bootstrap evidence](milestones.md)                                                   |
| LAC-ARCH-002   | Implemented | `scripts/check-boundaries.mjs`; `scripts/governance.test.mjs`; [bootstrap evidence](milestones.md)                                                   |
| LAC-ARCH-003   | Implemented | `scripts/rust-architecture.mjs`; `scripts/rust-architecture.test.mjs`; [bootstrap evidence](milestones.md)                                           |
| LAC-ARCH-004   | Implemented | `scripts/rust-architecture.mjs`; `scripts/rust-architecture.test.mjs`; [bootstrap evidence](milestones.md)                                           |
| LAC-ARCH-005   | Planned     | [Architecture](architecture.md); [milestone gate](milestones.md)                                                                                     |
| LAC-ARCH-006   | Planned     | [Architecture](architecture.md); [milestone gate](milestones.md)                                                                                     |
| LAC-PROTO-001  | Implemented | `crates/lapis-acp-protocol/src/frame.rs`; `crates/lapis-acp-protocol/tests/bounds.rs`; [M2 evidence](milestones.md)                                  |
| LAC-PROTO-002  | Implemented | `crates/lapis-acp-protocol/tests/wire.rs`; [M2 evidence](milestones.md)                                                                              |
| LAC-PROTO-003  | Implemented | `crates/lapis-acp-protocol/tests/wire.rs`; `crates/lapis-acp-protocol/tests/inbound.rs`; [M2 evidence](milestones.md)                                |
| LAC-PROTO-004  | Implemented | `crates/lapis-acp-protocol/tests/payload.rs`; `crates/lapis-acp-protocol/tests/wire.rs`; [M2 evidence](milestones.md)                                |
| LAC-PROTO-005  | Implemented | `crates/lapis-acp-protocol/tests/payload.rs`; `crates/lapis-acp-protocol/tests/wire.rs`; [M2 evidence](milestones.md)                                |
| LAC-PROTO-006  | Implemented | `scripts/governance.test.mjs`; `scripts/check-boundaries.mjs`; `crates/lapis-acp-protocol/Cargo.toml`; [M2 evidence](milestones.md)                  |
| LAC-AGENT-001  | Implemented | `crates/lapis-acp-agent/src/definition.rs`; `crates/lapis-acp-agent/tests/catalog.rs`; [M3 evidence](milestones.md)                                  |
| LAC-AGENT-002  | Implemented | `crates/lapis-acp-agent/src/config.rs`; `crates/lapis-acp-agent/tests/catalog.rs`; [M3 evidence](milestones.md)                                      |
| LAC-AGENT-003  | Implemented | `crates/lapis-acp-agent/src/environment.rs`; `crates/lapis-acp-agent/tests/catalog.rs`; [M3 evidence](milestones.md)                                 |
| LAC-AGENT-004  | Implemented | `crates/lapis-acp-agent/src/catalog.rs`; `crates/lapis-acp-agent/tests/catalog.rs`; [M3 evidence](milestones.md)                                     |
| LAC-AGENT-005  | Implemented | `crates/lapis-acp-agent/src/definition.rs`; `crates/lapis-acp-agent/tests/catalog.rs`; [M3 evidence](milestones.md)                                  |
| LAC-AGENT-006  | Implemented | `crates/lapis-acp-agent/src/environment.rs`; `crates/lapis-acp-agent/tests/catalog.rs`; [M3 evidence](milestones.md)                                 |
| LAC-RUN-001    | Planned     | [Session and turn lifecycle](runner.md); [milestone gate](milestones.md)                                                                             |
| LAC-RUN-002    | Planned     | [Session and turn lifecycle](runner.md); [milestone gate](milestones.md)                                                                             |
| LAC-RUN-003    | Planned     | [Session and turn lifecycle](runner.md); [milestone gate](milestones.md)                                                                             |
| LAC-RUN-004    | Planned     | [Session and turn lifecycle](runner.md); [milestone gate](milestones.md)                                                                             |
| LAC-RUN-005    | Planned     | [Session and turn lifecycle](runner.md); [milestone gate](milestones.md)                                                                             |
| LAC-RUN-006    | Planned     | [Session and turn lifecycle](runner.md); [milestone gate](milestones.md)                                                                             |
| LAC-RUN-007    | Planned     | [Session and turn lifecycle](runner.md); [milestone gate](milestones.md)                                                                             |
| LAC-CTRL-001   | Planned     | [Controlling application boundary](control.md); [milestone gate](milestones.md)                                                                      |
| LAC-CTRL-002   | Planned     | [Controlling application boundary](control.md); [milestone gate](milestones.md)                                                                      |
| LAC-CTRL-003   | Planned     | [Controlling application boundary](control.md); [milestone gate](milestones.md)                                                                      |
| LAC-CTRL-004   | Planned     | [Controlling application boundary](control.md); [milestone gate](milestones.md)                                                                      |
| LAC-CTRL-005   | Planned     | [Controlling application boundary](control.md); [milestone gate](milestones.md)                                                                      |
| LAC-CTRL-006   | Planned     | [Controlling application boundary](control.md); [milestone gate](milestones.md)                                                                      |
| LAC-CTRL-007   | Planned     | [Controlling application boundary](control.md); [milestone gate](milestones.md)                                                                      |
| LAC-CLI-001    | Planned     | [CLI and exit codes](cli.md); [milestone gate](milestones.md)                                                                                        |
| LAC-CLI-002    | Planned     | [CLI and exit codes](cli.md); [milestone gate](milestones.md)                                                                                        |
| LAC-CLI-003    | Planned     | [CLI and exit codes](cli.md); [milestone gate](milestones.md)                                                                                        |
| LAC-CLI-004    | Planned     | [CLI and exit codes](cli.md); [milestone gate](milestones.md)                                                                                        |
| LAC-CLI-005    | Planned     | [CLI and exit codes](cli.md); [milestone gate](milestones.md)                                                                                        |
| LAC-CLI-006    | Planned     | [CLI and exit codes](cli.md); [milestone gate](milestones.md)                                                                                        |
| LAC-SEC-001    | Implemented | `scripts/check-boundaries.mjs`; `scripts/governance.test.mjs`; `docs/upstream/disposition.json`; [M1 evidence](milestones.md)                        |
| LAC-SEC-002    | Planned     | [Security and trust](security.md); [milestone gate](milestones.md)                                                                                   |
| LAC-SEC-003    | Planned     | [Security and trust](security.md); [milestone gate](milestones.md)                                                                                   |
| LAC-SEC-004    | Planned     | [Security and trust](security.md); [milestone gate](milestones.md)                                                                                   |
| LAC-SEC-005    | Planned     | [Security and trust](security.md); [milestone gate](milestones.md)                                                                                   |
| LAC-SEC-006    | Planned     | [Security and trust](security.md); [milestone gate](milestones.md)                                                                                   |
| LAC-SEC-007    | Planned     | [Security and trust](security.md); [milestone gate](milestones.md)                                                                                   |
| LAC-COMPAT-001 | Implemented | `docs/upstream-lock.md`; `docs/upstream/disposition.json`; `docs/upstream/lock.json`; `scripts/upstream-lock.test.mjs`; [M1 evidence](milestones.md) |
| LAC-COMPAT-002 | Implemented | `docs/licenses/NOTICE.md`; `docs/upstream-lock.md`; `scripts/upstream-lock.test.mjs`; [M1 evidence](milestones.md)                                   |
| LAC-COMPAT-003 | Implemented | `scripts/upstream-sync.mjs`; `scripts/upstream-lock.test.mjs`; [M1 evidence](milestones.md)                                                          |
| LAC-COMPAT-004 | Implemented | `rust-toolchain.toml`; `rustfmt.toml`; `spec-validator.config.mjs`; [bootstrap evidence](milestones.md)                                              |
| LAC-COMPAT-005 | Implemented | `tests/fixtures/fixture_agent.py`; `crates/lapis-acp-testkit/tests/fixture.rs`; [M3 evidence](milestones.md)                                         |
| LAC-COMPAT-006 | Planned     | [Compatibility and upstream port](compatibility.md); [milestone gate](milestones.md)                                                                 |
| LAC-COMPAT-007 | Implemented | `docs/upstream/disposition.json`; `scripts/upstream-lock.mjs`; `scripts/upstream-lock.test.mjs`; [M1 evidence](milestones.md)                        |
| LAC-GOV-001    | Implemented | `scripts/governance.test.mjs`; `scripts/check-progress.mjs`; `spec-validator.config.mjs`; [bootstrap evidence](milestones.md)                        |
| LAC-GOV-002    | Implemented | `scripts/governance.test.mjs`; `scripts/check-progress.mjs`; `spec-validator.config.mjs`; [bootstrap evidence](milestones.md)                        |
| LAC-GOV-003    | Implemented | `scripts/governance.test.mjs`; `scripts/check-progress.mjs`; `spec-validator.config.mjs`; [bootstrap evidence](milestones.md)                        |
| LAC-GOV-004    | Implemented | `scripts/governance.test.mjs`; `scripts/check-progress.mjs`; `spec-validator.config.mjs`; [bootstrap evidence](milestones.md)                        |
| LAC-GOV-005    | Implemented | `scripts/governance.test.mjs`; `scripts/check-progress.mjs`; `spec-validator.config.mjs`; [bootstrap evidence](milestones.md)                        |
| LAC-GOV-006    | Implemented | `rustfmt.toml`; `package.json`; `spec-validator.config.mjs`; [bootstrap evidence](milestones.md)                                                     |
| LAC-GOV-007    | Implemented | `scripts/rust-architecture.mjs`; `scripts/rust-architecture.test.mjs`; [bootstrap evidence](milestones.md)                                           |
