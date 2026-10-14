# Implementation progress

This is the single status ledger. Done requires original-gate evidence; a
fixture-peer result cannot satisfy a real installed-runner gate, and compilation
cannot satisfy a behavioural gate. Every implementation slice adds evidence here
in its enclosing Jujutsu change.

## Tasks and gates

| ID          | Status      | Depends   | Requirements                                                                                                                                                                                                                                                                                            | Gate                                                                                                                                                | Next                                                | Evidence                                    |
| ----------- | ----------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------- | ------------------------------------------- |
| Bootstrap   | Done        | -         | JAC-GOV-001,JAC-GOV-002,JAC-GOV-003,JAC-GOV-005,JAC-GOV-006,JAC-ARCH-001,JAC-ARCH-002,JAC-COMPAT-004                                                                                                                                                                                                    | Canonical specification, ledger, crate skeleton, boundary gate and pinned toolchain green on an empty product                                       | Maintain governance each slice                      | E-BOOT-001                                  |
| Bootstrap.1 | Done        | -         | JAC-GOV-004,JAC-GOV-007,JAC-ARCH-003,JAC-ARCH-004                                                                                                                                                                                                                                                       | Boundary, counted-line and progress-checker negative tests pass                                                                                     | Maintain gates each slice                           | E-BOOT-002                                  |
| Bootstrap.2 | Done        | -         | JAC-GOV-001,JAC-GOV-002,JAC-GOV-003,JAC-ARCH-001                                                                                                                                                                                                                                                        | Repository, crates, binary, requirement prefix and environment namespace renamed from lapis-acp to jj-acp with the change map keyed on crate layer  | Maintain governance each slice                      | E-BOOT-003                                  |
| M1          | Done        | Bootstrap | JAC-COMPAT-001,JAC-COMPAT-002,JAC-COMPAT-003,JAC-COMPAT-007,JAC-SEC-001                                                                                                                                                                                                                                 | Pinned upstream lock, attribution and digest-verified drift detection                                                                               | Implement M2 protocol crate                         | E-M1-001                                    |
| M2          | Done        | Bootstrap | JAC-PROTO-001,JAC-PROTO-002,JAC-PROTO-003,JAC-PROTO-004,JAC-PROTO-005,JAC-PROTO-006                                                                                                                                                                                                                     | Bounded ACP framing, version negotiation and session methods under unit test                                                                        | Implement M3 runner catalog                         | E-M2-001                                    |
| M3          | Done        | M2        | JAC-AGENT-001,JAC-AGENT-002,JAC-AGENT-003,JAC-AGENT-004,JAC-AGENT-005,JAC-AGENT-006,JAC-COMPAT-005                                                                                                                                                                                                      | Tiered runner catalog, reserved-key stripping and deterministic fixture peer                                                                        | Implement M4 task runner                            | E-M3-001                                    |
| M4          | Done        | M3        | JAC-CTRL-002,JAC-CTRL-003,JAC-CTRL-004,JAC-CTRL-005,JAC-CTRL-006,JAC-CTRL-007,JAC-CLI-001,JAC-CLI-002,JAC-CLI-003,JAC-CLI-004,JAC-CLI-005,JAC-CLI-006,JAC-RUN-001,JAC-RUN-002,JAC-RUN-003,JAC-RUN-004,JAC-ARCH-005,JAC-ARCH-006,JAC-SEC-002,JAC-SEC-003,JAC-SEC-004,JAC-SEC-005,JAC-SEC-006,JAC-SEC-007 | One-shot prepared task runner with sanitized record, published exit codes, signal cleanup and clean stdout                                          | Qualify M5 process-group containment                | E-M4-001                                    |
| M5          | Done        | M4        | JAC-RUN-005,JAC-RUN-007                                                                                                                                                                                                                                                                                 | Descendant containment on both the forced and the graceful path, crash reporting distinct from a timeout, and no re-dispatch                        | Qualify M6 against a real installed runner          | E-M5-001                                    |
| M6          | Done        | M5        | JAC-COMPAT-006,JAC-PROTO-002                                                                                                                                                                                                                                                                            | Real installed-runner qualification against cursor-agent acp recorded separately from fixture evidence                                              | Implement M7 session configuration                  | E-M6-001,E-M6-002                           |
| M7          | Done        | M6        | JAC-RUN-006                                                                                                                                                                                                                                                                                             | Model, effort and mode resolved per session with transport-versus-rejection classification, qualified live on two adapters                          | Implement M8 persistent transport                   | E-M7-001                                    |
| M8          | Done        | M7        | JAC-CTRL-001                                                                                                                                                                                                                                                                                            | Persistent versioned control protocol over stdio with concurrent cancellation, frame-level refusals and one shape for every task outcome            | Implement M9 standing pool                          | E-M8-001                                    |
| M9          | Done        | M8        | JAC-RUN-001,JAC-RUN-005                                                                                                                                                                                                                                                                                 | Caller-named scopes with one turn in flight each, concurrent distinct scopes, bounded queue admission and slot release on crash without re-dispatch | Implement M10 consumer acceptance                   | E-M9-001                                    |
| M10         | Done        | M9        | JAC-ARCH-005                                                                                                                                                                                                                                                                                            | Controlling-application consumer acceptance in a real host                                                                                          | Maintain qualified macOS jj-store profile           | E-M10-001                                   |
| M11         | Done        | M9        | JAC-CTRL-008,JAC-RUN-008,JAC-PROTO-007,JAC-CLI-007                                                                                                                                                                                                                                                      | Persistent interactive sessions, streamed activity, permission decisions and bounded cleanup                                                        | Maintain interactive session contract               | E-M11-001,E-M11-002,E-M11-003               |
| M12         | In progress | M11       | JAC-CTRL-009,JAC-RUN-009,JAC-PROTO-008,JAC-CLI-008                                                                                                                                                                                                                                                      | Warm worker and scoped sessions with real installed-runner acceptance                                                                               | Qualify installed adapters and jj-store integration | E-M12-001,E-M12-004,E-TOOLS-001,E-TOOLS-002 |
| M13         | Done        | M11       | JAC-REL-001,JAC-REL-002,JAC-REL-003,JAC-REL-004,JAC-REL-005                                                                                                                                                                                                                                             | Portable crate packaging, Git CI governance, Pages and native release automation                                                                    | Validate source and archives before publication     | E-RELEASE-001                               |
| M14         | Done        | M13       | JAC-REL-003,JAC-REL-004,JAC-REL-005                                                                                                                                                                                                                                                                     | Public source, hosted CI/specification, registry installation and four native archives                                                              | Publish the verified source and version             | E-GITHUB-001,E-PUBLISH-001                  |

## Evidence

Each entry records the commands actually run, their real failure counts, the
files touched, the named tests that prove the gate, the versions and formats in
play, and the gaps that remain. An entry with `result: "pass"` and a non-empty
`remaining` list is rejected by the progress checker, so a passing claim cannot
coexist with an acknowledged gap.

```json
[
  {
    "id": "E-BOOT-001",
    "tasks": ["Bootstrap"],
    "date": "2026-09-30",
    "kind": "governance",
    "files": [
      "spec-validator.config.mjs",
      "spec/book.toml",
      "spec/src/SUMMARY.md",
      "spec/src/index.md",
      "spec/src/architecture.md",
      "spec/src/protocol.md",
      "spec/src/agents.md",
      "spec/src/runner.md",
      "spec/src/control.md",
      "spec/src/cli.md",
      "spec/src/security.md",
      "spec/src/compatibility.md",
      "spec/src/spec-governance.md",
      "spec/src/milestones.md",
      "spec/src/verification.md",
      "Cargo.toml",
      "Cargo.lock",
      "rust-toolchain.toml",
      "rustfmt.toml",
      "package.json",
      "AGENTS.md",
      "README.md",
      "crates/jj-acp-protocol/src/lib.rs",
      "crates/jj-acp-agent/src/lib.rs",
      "crates/jj-acp-runner/src/lib.rs",
      "crates/jj-acp-control/src/lib.rs",
      "crates/jj-acp-cli/src/lib.rs",
      "crates/jj-acp-testkit/src/lib.rs"
    ],
    "tests": ["scripts/governance.test.mjs#accepts_verified_task"],
    "runs": [
      {
        "command": "cargo fmt --all -- --check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo clippy --workspace --all-targets --locked -- -D warnings",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "node scripts/check-boundaries.mjs",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "spec-validator validate",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Rust 1.98.1; cargo 1.98.1; Node v24.10.0; pnpm 10.34.5; @lapismd/spec-validator 0.1.0",
    "formats": "requirement table; verification table; ledger task table and JSON evidence"
  },
  {
    "id": "E-BOOT-002",
    "tasks": ["Bootstrap.1"],
    "date": "2026-09-30",
    "kind": "governance",
    "files": [
      "scripts/check-boundaries.mjs",
      "scripts/check-progress.mjs",
      "scripts/governance.test.mjs",
      "scripts/rust-architecture.mjs",
      "scripts/rust-architecture.test.mjs",
      "scripts/rust-architecture-baseline.json"
    ],
    "tests": [
      "scripts/governance.test.mjs#rejects_excluded_nostr_transport_and_database_dependencies",
      "scripts/governance.test.mjs#testkit_is_dev_only",
      "scripts/governance.test.mjs#rejects_crate_outside_the_boundary_matrix",
      "scripts/governance.test.mjs#rejects_fixture_peer_qualification",
      "scripts/governance.test.mjs#rejects_unchanged_ledger_for_code",
      "scripts/rust-architecture.test.mjs#rejects_production_source_over_budget",
      "scripts/rust-architecture.test.mjs#rejects_inline_tests_in_a_production_module",
      "scripts/rust-architecture.test.mjs#rejects_production_import_of_the_testkit",
      "scripts/rust-architecture.test.mjs#rejects_stale_baseline_entry"
    ],
    "runs": [
      {
        "command": "node --test scripts/governance.test.mjs scripts/rust-architecture.test.mjs",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "node scripts/rust-architecture.mjs",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "node scripts/check-progress.mjs",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Node v24.10.0; cargo 1.98.1 for cargo metadata --no-deps --locked",
    "formats": "counted-line budget; crate dependency matrix; ledger JSON evidence"
  },
  {
    "id": "E-M1-001",
    "tasks": ["M1"],
    "date": "2026-09-30",
    "kind": "upstream-audit",
    "files": [
      "docs/upstream-lock.md",
      "docs/licenses/NOTICE.md",
      "docs/upstream/disposition.json",
      "docs/upstream/lock.json",
      "scripts/upstream-lock.mjs",
      "scripts/upstream-sync.mjs",
      "scripts/upstream-lock.test.mjs",
      "spec/src/compatibility.md",
      "spec/src/spec-governance.md",
      "spec/src/architecture.md",
      "spec-validator.config.mjs",
      "package.json"
    ],
    "tests": [
      "scripts/upstream-lock.test.mjs#the_curated_port_map_is_valid",
      "scripts/upstream-lock.test.mjs#the_recorded_lock_matches_the_curated_map",
      "scripts/upstream-lock.test.mjs#validate_lock_detects_drift_from_the_curated_map",
      "scripts/upstream-lock.test.mjs#drift_report_classifies_every_observed_file",
      "scripts/upstream-lock.test.mjs#build_lock_refuses_a_classified_region_without_a_digest",
      "scripts/upstream-lock.test.mjs#dropped_regions_carry_a_reason_and_no_owner",
      "scripts/upstream-lock.test.mjs#rejects_an_owner_disposition_mismatch",
      "scripts/upstream-lock.test.mjs#scratch_materialization_stays_in_ignored_storage"
    ],
    "runs": [
      {
        "command": "node --test scripts/upstream-lock.test.mjs",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "node scripts/upstream-sync.mjs --self-test",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "node scripts/upstream-sync.mjs --write",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "node scripts/upstream-sync.mjs --fetch",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "block/buzz@2664d14316790a57ea44b3f97c57440b70e43436; Node v24.10.0; SHA-256",
    "formats": "upstream disposition map; generated digest lock; drift report"
  },
  {
    "id": "E-M2-001",
    "tasks": ["M2"],
    "date": "2026-09-30",
    "kind": "protocol",
    "files": [
      "crates/jj-acp-protocol/Cargo.toml",
      "crates/jj-acp-protocol/src/lib.rs",
      "crates/jj-acp-protocol/src/budget.rs",
      "crates/jj-acp-protocol/src/client.rs",
      "crates/jj-acp-protocol/src/config_option.rs",
      "crates/jj-acp-protocol/src/content.rs",
      "crates/jj-acp-protocol/src/envelope.rs",
      "crates/jj-acp-protocol/src/error.rs",
      "crates/jj-acp-protocol/src/frame.rs",
      "crates/jj-acp-protocol/src/handler.rs",
      "crates/jj-acp-protocol/src/initialize.rs",
      "crates/jj-acp-protocol/src/permission.rs",
      "crates/jj-acp-protocol/src/session.rs",
      "crates/jj-acp-protocol/src/stop_reason.rs",
      "crates/jj-acp-protocol/src/update.rs",
      "crates/jj-acp-protocol/tests/common/mod.rs",
      "crates/jj-acp-protocol/tests/payload.rs",
      "crates/jj-acp-protocol/tests/wire.rs",
      "crates/jj-acp-protocol/tests/inbound.rs",
      "crates/jj-acp-protocol/tests/bounds.rs",
      "Cargo.toml",
      "Cargo.lock",
      "scripts/governance.test.mjs",
      "scripts/rust-architecture.mjs",
      "scripts/rust-architecture.test.mjs",
      "docs/upstream/disposition.json",
      "docs/upstream/lock.json",
      "spec/src/protocol.md",
      "spec/src/architecture.md",
      "spec/src/compatibility.md",
      "spec/src/spec-governance.md",
      "AGENTS.md"
    ],
    "tests": [
      "crates/jj-acp-protocol/tests/wire.rs#initialize_negotiates_and_records_the_selected_version",
      "crates/jj-acp-protocol/tests/wire.rs#initialize_rejects_a_version_the_client_cannot_speak",
      "crates/jj-acp-protocol/tests/wire.rs#a_prompt_streams_updates_and_then_returns_one_stop_reason",
      "crates/jj-acp-protocol/tests/wire.rs#request_ids_increase_and_stay_correlated",
      "crates/jj-acp-protocol/tests/wire.rs#cancellation_is_a_notification_and_carries_no_id",
      "crates/jj-acp-protocol/tests/inbound.rs#an_unanswered_permission_request_is_cancelled_not_approved",
      "crates/jj-acp-protocol/tests/inbound.rs#an_explicit_approval_selects_the_option_by_kind",
      "crates/jj-acp-protocol/tests/inbound.rs#an_unimplemented_agent_request_is_answered_not_ignored",
      "crates/jj-acp-protocol/tests/bounds.rs#an_oversized_frame_is_refused_before_it_can_be_buffered",
      "crates/jj-acp-protocol/tests/bounds.rs#the_idle_bound_reports_silence_while_the_absolute_bound_caps_noise",
      "crates/jj-acp-protocol/tests/bounds.rs#an_agent_that_stops_reading_is_reported_as_a_write_timeout",
      "crates/jj-acp-protocol/tests/bounds.rs#a_closed_agent_stream_is_reported_not_hung",
      "crates/jj-acp-protocol/tests/payload.rs#a_model_match_comes_from_the_nested_candidates_not_the_current_value",
      "crates/jj-acp-protocol/tests/payload.rs#stop_reason_parsing_tolerates_agent_casing",
      "scripts/governance.test.mjs#the_protocol_crate_declares_no_process_or_network_facility",
      "scripts/rust-architecture.test.mjs#allows_a_cfg_test_sibling_module_declaration"
    ],
    "runs": [
      {
        "command": "cargo test -p jj-acp-protocol --locked",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo clippy --workspace --all-targets --locked -- -D warnings",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "node scripts/rust-architecture.mjs",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "node --test scripts/governance.test.mjs scripts/rust-architecture.test.mjs scripts/upstream-lock.test.mjs",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Rust 1.98.1; tokio 1.53.1; serde 1.0.229; serde_json 1.0.151; thiserror 2.0.21",
    "formats": "ACP JSON-RPC protocol version 2 with version 1 fallback; newline-delimited frames capped at 10 MB"
  },
  {
    "id": "E-M3-001",
    "tasks": ["M3"],
    "date": "2026-09-30",
    "kind": "fixture-peer",
    "files": [
      "crates/jj-acp-agent/Cargo.toml",
      "crates/jj-acp-agent/src/lib.rs",
      "crates/jj-acp-agent/src/availability.rs",
      "crates/jj-acp-agent/src/catalog.rs",
      "crates/jj-acp-agent/src/config.rs",
      "crates/jj-acp-agent/src/defaults.rs",
      "crates/jj-acp-agent/src/definition.rs",
      "crates/jj-acp-agent/src/environment.rs",
      "crates/jj-acp-agent/src/error.rs",
      "crates/jj-acp-agent/tests/catalog.rs",
      "crates/jj-acp-testkit/Cargo.toml",
      "crates/jj-acp-testkit/src/lib.rs",
      "crates/jj-acp-testkit/src/fixture.rs",
      "crates/jj-acp-testkit/src/log.rs",
      "crates/jj-acp-testkit/tests/fixture.rs",
      "tests/fixtures/fixture_agent.py",
      "Cargo.toml",
      "Cargo.lock",
      "scripts/check-boundaries.mjs",
      "scripts/governance.test.mjs",
      "scripts/rust-architecture.mjs",
      "scripts/rust-architecture.test.mjs",
      "docs/upstream/disposition.json",
      "docs/upstream/lock.json",
      "spec/src/agents.md",
      "spec/src/architecture.md",
      "spec/src/compatibility.md",
      "spec/src/spec-governance.md"
    ],
    "tests": [
      "crates/jj-acp-agent/tests/catalog.rs#a_runner_is_declared_entirely_as_data",
      "crates/jj-acp-agent/tests/catalog.rs#identifiers_are_pattern_checked_and_length_bounded",
      "crates/jj-acp-agent/tests/catalog.rs#the_compiled_in_tier_reserves_its_identifiers",
      "crates/jj-acp-agent/tests/catalog.rs#no_tier_may_claim_an_identifier_twice",
      "crates/jj-acp-agent/tests/catalog.rs#unknown_and_disabled_runners_fail_without_a_default_fallback",
      "crates/jj-acp-agent/tests/catalog.rs#reserved_environment_keys_are_stripped_from_every_layer",
      "crates/jj-acp-agent/tests/catalog.rs#environment_layers_apply_inherited_then_floor_then_overrides",
      "crates/jj-acp-agent/tests/catalog.rs#harness_variables_are_produced_only_by_the_harness",
      "crates/jj-acp-agent/tests/catalog.rs#a_catalog_document_declares_runners_without_executing_anything",
      "crates/jj-acp-agent/tests/catalog.rs#availability_resolves_the_first_candidate_that_exists",
      "crates/jj-acp-testkit/tests/fixture.rs#the_fixture_completes_a_turn_without_a_model_provider",
      "crates/jj-acp-testkit/tests/fixture.rs#a_permission_request_is_answered_and_the_turn_completes",
      "crates/jj-acp-testkit/tests/fixture.rs#an_agent_error_arrives_typed_with_its_marker_intact",
      "crates/jj-acp-testkit/tests/fixture.rs#a_silent_agent_is_bounded_by_the_idle_window",
      "crates/jj-acp-testkit/tests/fixture.rs#a_version_one_agent_is_accepted_and_recorded",
      "crates/jj-acp-testkit/tests/fixture.rs#killing_the_agent_alone_leaves_its_descendant_running",
      "scripts/governance.test.mjs#the_testkit_may_depend_on_the_layers_it_exercises",
      "scripts/rust-architecture.test.mjs#allows_the_testkit_crate_to_name_itself"
    ],
    "runs": [
      {
        "command": "cargo test -p jj-acp-agent --locked",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo test -p jj-acp-testkit --locked",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo test -p jj-acp-protocol --locked",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo clippy --workspace --all-targets --locked -- -D warnings",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "node --test scripts/governance.test.mjs scripts/rust-architecture.test.mjs scripts/upstream-lock.test.mjs",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "node scripts/upstream-sync.mjs --self-test",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Rust 1.98.1; Python 3.14.7; tokio 1.53.1; serde_json 1.0.151",
    "formats": "runner catalog document; fixture record log; ACP protocol versions 1 and 2"
  },
  {
    "id": "E-M4-001",
    "tasks": ["M4"],
    "date": "2026-09-30",
    "kind": "task-runner",
    "files": [
      "crates/jj-acp-protocol/src/client.rs",
      "crates/jj-acp-protocol/src/frame.rs",
      "crates/jj-acp-protocol/tests/bounds.rs",
      "crates/jj-acp-agent/src/config.rs",
      "crates/jj-acp-agent/src/environment.rs",
      "crates/jj-acp-agent/tests/catalog.rs",
      "crates/jj-acp-runner/Cargo.toml",
      "crates/jj-acp-runner/src/lib.rs",
      "crates/jj-acp-runner/src/launch.rs",
      "crates/jj-acp-runner/src/outcome.rs",
      "crates/jj-acp-runner/src/process.rs",
      "crates/jj-acp-runner/src/turn.rs",
      "crates/jj-acp-control/Cargo.toml",
      "crates/jj-acp-control/src/lib.rs",
      "crates/jj-acp-control/src/input.rs",
      "crates/jj-acp-control/src/invocation.rs",
      "crates/jj-acp-control/src/record.rs",
      "crates/jj-acp-control/src/task.rs",
      "crates/jj-acp-control/src/transport.rs",
      "crates/jj-acp-cli/Cargo.toml",
      "crates/jj-acp-cli/src/lib.rs",
      "crates/jj-acp-cli/src/main.rs",
      "crates/jj-acp-cli/src/agents.rs",
      "crates/jj-acp-cli/src/args.rs",
      "crates/jj-acp-cli/src/run_task.rs",
      "crates/jj-acp-cli/src/signals.rs",
      "crates/jj-acp-cli/tests/common/mod.rs",
      "crates/jj-acp-cli/tests/args.rs",
      "crates/jj-acp-cli/tests/run_task.rs",
      "crates/jj-acp-cli/tests/signals.rs",
      "crates/jj-acp-cli/tests/stdio_contract.rs",
      "crates/jj-acp-testkit/tests/fixture.rs",
      "tests/fixtures/fixture_agent.py",
      "Cargo.toml",
      "Cargo.lock",
      "docs/upstream/disposition.json",
      "docs/upstream/lock.json",
      "spec/src/architecture.md",
      "spec/src/protocol.md",
      "spec/src/agents.md",
      "spec/src/runner.md",
      "spec/src/control.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md"
    ],
    "tests": [
      "crates/jj-acp-cli/tests/run_task.rs#a_prepared_task_completes_with_one_terminal_record",
      "crates/jj-acp-cli/tests/run_task.rs#file_and_stdin_input_agree",
      "crates/jj-acp-cli/tests/run_task.rs#a_refusal_is_a_normal_completion_with_its_own_stop_reason",
      "crates/jj-acp-cli/tests/run_task.rs#an_agent_error_never_echoes_the_agent_message",
      "crates/jj-acp-cli/tests/run_task.rs#the_prompt_and_credentials_never_reach_the_terminal_record",
      "crates/jj-acp-cli/tests/run_task.rs#the_fixture_receives_the_harness_reserved_variables",
      "crates/jj-acp-cli/tests/run_task.rs#invalid_documents_are_refused_before_any_agent_starts",
      "crates/jj-acp-cli/tests/run_task.rs#duplicate_and_trailing_documents_are_refused",
      "crates/jj-acp-cli/tests/run_task.rs#an_oversized_document_is_refused_without_being_buffered",
      "crates/jj-acp-cli/tests/run_task.rs#a_task_addressed_to_another_runner_is_refused",
      "crates/jj-acp-cli/tests/run_task.rs#an_unknown_runner_is_refused_without_pinning_one",
      "crates/jj-acp-cli/tests/run_task.rs#an_unresolvable_runner_reports_a_spawn_failure",
      "crates/jj-acp-cli/tests/run_task.rs#a_deadline_produces_timed_out_and_exit_124",
      "crates/jj-acp-cli/tests/run_task.rs#an_idle_window_that_never_sees_a_frame_times_out_separately",
      "crates/jj-acp-cli/tests/signals.rs#sigterm_produces_bounded_cleanup_and_exit_143",
      "crates/jj-acp-cli/tests/signals.rs#sigint_produces_bounded_cleanup_and_exit_130",
      "crates/jj-acp-cli/tests/stdio_contract.rs#an_invalid_command_line_leaves_stdout_empty",
      "crates/jj-acp-cli/tests/stdio_contract.rs#a_failed_invocation_still_emits_exactly_one_record",
      "crates/jj-acp-cli/tests/stdio_contract.rs#the_agents_command_emits_one_json_object_per_line",
      "crates/jj-acp-cli/tests/args.rs#a_flag_overrides_the_matching_environment_variable",
      "crates/jj-acp-cli/tests/args.rs#every_option_is_settable_by_environment",
      "crates/jj-acp-protocol/tests/bounds.rs#a_read_cancelled_mid_frame_resumes_instead_of_losing_bytes",
      "crates/jj-acp-testkit/tests/fixture.rs#killing_the_agent_alone_leaves_its_descendant_running"
    ],
    "runs": [
      {
        "command": "cargo test --workspace --locked",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo clippy --workspace --all-targets --locked -- -D warnings",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "node --test scripts/governance.test.mjs scripts/rust-architecture.test.mjs scripts/upstream-lock.test.mjs",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "node scripts/rust-architecture.mjs",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "node scripts/check-boundaries.mjs",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Rust 1.98.1; Python 3.14.7; tokio 1.53.1; nix 0.31; serde_json 1.0.151",
    "formats": "task document version 1; terminal record version 1; exit codes 0/1/2/124/130/143"
  },
  {
    "id": "E-M5-001",
    "tasks": ["M5"],
    "date": "2026-09-30",
    "kind": "lifecycle",
    "files": [
      "crates/jj-acp-runner/src/process.rs",
      "crates/jj-acp-cli/tests/cleanup.rs",
      "crates/jj-acp-testkit/src/fixture.rs",
      "crates/jj-acp-testkit/src/log.rs",
      "tests/fixtures/fixture_agent.py",
      "spec/src/runner.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md"
    ],
    "tests": [
      "crates/jj-acp-cli/tests/cleanup.rs#a_forced_termination_takes_the_agents_descendant_with_it",
      "crates/jj-acp-cli/tests/cleanup.rs#a_completed_turn_still_does_not_leave_a_descendant_behind",
      "crates/jj-acp-cli/tests/cleanup.rs#an_agent_that_crashes_mid_turn_is_reported_and_never_re_dispatched",
      "crates/jj-acp-cli/tests/cleanup.rs#an_agent_that_crashes_during_startup_never_opens_a_session",
      "crates/jj-acp-cli/tests/cleanup.rs#a_crash_is_distinguished_from_a_bound_that_elapsed",
      "crates/jj-acp-cli/tests/cleanup.rs#the_helpers_resolve_the_real_fixture_paths"
    ],
    "runs": [
      {
        "command": "cargo test -p jj-acp-cli --test cleanup --locked",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo test --workspace --locked",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo clippy --workspace --all-targets --locked -- -D warnings",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "node --test scripts/governance.test.mjs scripts/rust-architecture.test.mjs scripts/upstream-lock.test.mjs",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Rust 1.98.1; nix 0.31; Python 3.14.7; tokio 1.53.1; macOS ARM64",
    "formats": "termination report fields group_signalled/group_killed/reaped; fixture crash markers; dispatch counts",
    "note": "The containment test for a gracefully completing turn failed before process.rs signalled the group ahead of the grace window, which is the fix this entry records. Descendant containment on non-Unix platforms is unqualified: the direct child is signalled and the reduced guarantee is reported rather than approximated."
  },
  {
    "id": "E-M6-001",
    "tasks": ["M6"],
    "date": "2026-09-30",
    "kind": "real-runner",
    "files": [
      "crates/jj-acp-agent/src/defaults.rs",
      "crates/jj-acp-agent/tests/catalog.rs",
      "crates/jj-acp-cli/src/args.rs",
      "crates/jj-acp-cli/src/lib.rs",
      "crates/jj-acp-cli/tests/args.rs",
      "crates/jj-acp-protocol/src/initialize.rs",
      "crates/jj-acp-protocol/tests/payload.rs",
      "crates/jj-acp-protocol/tests/real_adapter.rs",
      "crates/jj-acp-protocol/tests/fixtures/cursor-acp-session.json",
      "spec/src/agents.md",
      "spec/src/protocol.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md"
    ],
    "tests": [
      "crates/jj-acp-protocol/tests/real_adapter.rs#a_real_adapter_selects_protocol_version_one_and_is_accepted",
      "crates/jj-acp-protocol/tests/real_adapter.rs#a_real_adapter_names_its_option_id_rather_than_config_id",
      "crates/jj-acp-protocol/tests/real_adapter.rs#model_candidates_are_parameterized_and_matched_exactly",
      "crates/jj-acp-protocol/tests/real_adapter.rs#the_stable_surface_wins_when_both_surfaces_offer_the_model",
      "crates/jj-acp-protocol/tests/real_adapter.rs#a_model_only_on_the_unstable_surface_falls_back_to_set_model",
      "crates/jj-acp-protocol/tests/real_adapter.rs#this_adapter_exposes_no_reasoning_effort_option_of_its_own",
      "crates/jj-acp-protocol/tests/payload.rs#an_explicit_false_capability_is_not_advertised",
      "crates/jj-acp-agent/tests/catalog.rs#cursor_is_declared_with_the_transport_it_actually_advertises",
      "crates/jj-acp-cli/tests/args.rs#the_agents_command_parses_the_same_launch_options_as_run",
      "crates/jj-acp-cli/tests/args.rs#arguments_no_subcommand_would_read_are_refused",
      "crates/jj-acp-cli/tests/args.rs#agents_refuses_a_task_flag"
    ],
    "runs": [
      {
        "command": "cursor-agent acp — initialize and session/new handshake probe, no prompt",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "jj-acp run --task <scratch>/task.json --runner cursor --idle-timeout 60 --max-turn-duration 180 (reply READY, no tools)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "jj-acp run --task <scratch>/task.json --runner cursor (read notes.md, report first line, no modification)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo test --workspace --locked",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo clippy --workspace --all-targets --locked -- -D warnings",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "node --test scripts/governance.test.mjs scripts/rust-architecture.test.mjs scripts/upstream-lock.test.mjs",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "cursor-agent 2026.09.26-dd393fe; ACP protocolVersion 1 negotiated from a request for 2; Rust 1.98.1; macOS ARM64",
    "formats": "captured initialize and session/new payloads; parameterized model values such as composer-2.5[fast=true]; configOption id rather than configId",
    "note": "Two real turns completed with status completed and stopReason end_turn, exit 0, no stderr noise, and the read target left unmodified. Three defects were found by the real adapter and none by the fixture suite: advertises() treated an explicit false as advertised, agents ignored --catalog, and trailing arguments after help were silently dropped. Qualification covers cursor only; codex-acp, claude-agent-acp, opencode and goose remain unlaunched, and an adapter that cannot reuse an existing login cannot be driven because authentication is deliberately unimplemented."
  },
  {
    "id": "E-M6-002",
    "tasks": ["M6"],
    "date": "2026-09-30",
    "kind": "real-runner",
    "files": [
      "crates/jj-acp-protocol/tests/real_adapter.rs",
      "crates/jj-acp-protocol/tests/fixtures/codex-acp-session.json",
      "spec/src/agents.md",
      "spec/src/protocol.md",
      "spec/src/compatibility.md"
    ],
    "tests": [
      "crates/jj-acp-protocol/tests/real_adapter.rs#a_second_adapter_also_selects_version_one_but_does_report_agent_info",
      "crates/jj-acp-protocol/tests/real_adapter.rs#explicit_false_capabilities_are_not_advertised_by_a_real_adapter",
      "crates/jj-acp-protocol/tests/real_adapter.rs#adapters_disagree_about_whether_model_values_are_parameterized",
      "crates/jj-acp-protocol/tests/real_adapter.rs#a_real_adapter_does_expose_a_reasoning_effort_option",
      "crates/jj-acp-protocol/tests/real_adapter.rs#adapters_emit_notifications_that_are_not_part_of_the_exchange",
      "crates/jj-acp-protocol/tests/real_adapter.rs#both_adapters_expose_modes_this_crate_does_not_yet_model"
    ],
    "runs": [
      {
        "command": "codex-acp 1.13.1 handshake probe (initialize + session/new), no prompt",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "PATH=<npx cache bin>:$PATH jj-acp run --task <scratch>/task.json --runner codex --idle-timeout 150 --max-turn-duration 240 (read notes.md, report first line, no modification)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo test -p jj-acp-protocol --test real_adapter --locked",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo test --workspace --locked",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "@agentclientprotocol/codex-acp 1.13.1 resolved from the local npx cache; codex-cli 0.156.1 present but not an ACP agent; ACP protocolVersion 1 negotiated from a request for 2; ChatGPT login reused from ~/.codex/auth.json",
    "formats": "captured initialize and session/new payloads; bare model values such as gpt-6-astra; configOption categories mode, collaboration_mode, model, thought_level and model_config; _auth/status_update notification",
    "note": "One real read-only turn completed with status completed, stopReason end_turn and exit 0; notes.md was unmodified and no file was created. The codex CLI itself has no acp subcommand, so the ACP path is the adapter that acpx launches through npx on demand and that never appears on PATH. This adapter does expose a thought_level option with the adapter-defined identifier reasoning_effort, which Cursor does not, so JAC-RUN-006 stays Planned and the discovery helper is proven against real data rather than implemented end to end. No permission request was raised, so the default deny policy was not exercised against this adapter."
  },
  {
    "id": "E-M7-001",
    "tasks": ["M7"],
    "date": "2026-09-30",
    "kind": "real-runner",
    "files": [
      "crates/jj-acp-protocol/src/thinking.rs",
      "crates/jj-acp-protocol/src/config_option.rs",
      "crates/jj-acp-protocol/src/lib.rs",
      "crates/jj-acp-protocol/tests/session_config.rs",
      "crates/jj-acp-runner/src/configuration.rs",
      "crates/jj-acp-runner/src/outcome.rs",
      "crates/jj-acp-runner/src/turn.rs",
      "crates/jj-acp-runner/src/lib.rs",
      "crates/jj-acp-control/src/invocation.rs",
      "crates/jj-acp-cli/src/args.rs",
      "crates/jj-acp-cli/src/run_task.rs",
      "crates/jj-acp-cli/tests/args.rs",
      "crates/jj-acp-cli/tests/configuration.rs",
      "tests/fixtures/fixture_agent.py",
      "spec/src/protocol.md",
      "spec/src/runner.md",
      "spec/src/control.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md"
    ],
    "tests": [
      "crates/jj-acp-protocol/tests/session_config.rs#effort_resolves_through_a_discovered_thought_level_option",
      "crates/jj-acp-protocol/tests/session_config.rs#effort_levels_outside_the_portable_vocabulary_stay_selectable",
      "crates/jj-acp-protocol/tests/session_config.rs#effort_resolves_through_parameterized_model_ids_when_no_option_exists",
      "crates/jj-acp-protocol/tests/session_config.rs#a_base_model_with_no_matching_effort_is_reported_honestly",
      "crates/jj-acp-protocol/tests/session_config.rs#a_bare_positional_bracket_is_not_read_as_an_effort",
      "crates/jj-acp-protocol/tests/session_config.rs#a_keyed_effort_wins_over_a_thinking_marker_on_the_same_identifier",
      "crates/jj-acp-protocol/tests/session_config.rs#mode_resolution_uses_both_advertised_surfaces",
      "crates/jj-acp-protocol/tests/session_config.rs#a_mode_offered_under_another_category_is_not_substituted",
      "crates/jj-acp-protocol/tests/session_config.rs#a_mode_present_in_the_option_but_not_in_available_modes_is_refused",
      "crates/jj-acp-cli/tests/configuration.rs#a_requested_model_effort_and_mode_all_reach_the_adapter",
      "crates/jj-acp-cli/tests/configuration.rs#configuration_is_applied_before_the_turn_is_dispatched",
      "crates/jj-acp-cli/tests/configuration.rs#a_value_offered_under_another_category_is_not_substituted",
      "crates/jj-acp-cli/tests/configuration.rs#an_unknown_model_is_reported_and_the_turn_still_completes",
      "crates/jj-acp-cli/tests/configuration.rs#configuration_arrives_by_environment_variable_too",
      "crates/jj-acp-cli/tests/args.rs#every_option_is_settable_by_environment"
    ],
    "runs": [
      {
        "command": "jj-acp run --runner cursor --effort high (live; effort resolved through a parameterized model identifier and accepted)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "jj-acp run --runner cursor --mode plan (live; applied with no shortfall)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "jj-acp run --runner cursor --model composer-2.5 --effort high (live; both reported unavailable, exit 0)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "jj-acp run --runner codex --mode read-only (live; applied)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "jj-acp run --runner codex --mode plan (live; refused naming collaboration_mode)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "jj-acp run --runner codex --effort high (live; reported unsupported because the adapter no longer advertises thought_level for the current model)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo test --workspace --locked",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo clippy --workspace --all-targets --locked -- -D warnings",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "node --test scripts/governance.test.mjs scripts/rust-architecture.test.mjs scripts/upstream-lock.test.mjs",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "cursor-agent 2026.09.26-dd393fe; @agentclientprotocol/codex-acp 1.13.1; Rust 1.98.1; macOS ARM64",
    "formats": "normalized thinking levels off/low/medium/high/xhigh for matching only; effort attribute keys effort, reasoning_effort, variant, thinking; parameterized model identifiers",
    "note": "Three defects were found by asserting against real captures rather than synthetic payloads: advertised_effort returned the first attribute in document order so a thinking marker masked a level, the effort-surface decision was made after narrowing to one model so a model without effort reported the adapter as having no effort control at all, and the configuration tests initially resolved against the fixture envelope instead of its session/new result. A live codex session also stopped advertising thought_level between captures at the same adapter version, which corrected an earlier claim in compatibility.md; the thought_level path is fixture-proven but not live-proven. A bare positional bracket such as [medium] is deliberately not read as an effort."
  },
  {
    "id": "E-M8-001",
    "tasks": ["M8"],
    "date": "2026-09-30",
    "kind": "control-protocol",
    "files": [
      "crates/jj-acp-control/src/protocol.rs",
      "crates/jj-acp-control/src/serve.rs",
      "crates/jj-acp-control/src/invocation.rs",
      "crates/jj-acp-control/src/lib.rs",
      "crates/jj-acp-cli/src/serve.rs",
      "crates/jj-acp-cli/src/args.rs",
      "crates/jj-acp-cli/src/lib.rs",
      "crates/jj-acp-cli/src/run_task.rs",
      "crates/jj-acp-cli/tests/serve.rs",
      "crates/jj-acp-cli/tests/signals.rs",
      "spec/src/control.md",
      "spec/src/cli.md"
    ],
    "tests": [
      "crates/jj-acp-cli/tests/serve.rs#startup_announces_the_resolved_runners",
      "crates/jj-acp-cli/tests/serve.rs#a_submitted_task_returns_exactly_one_record",
      "crates/jj-acp-cli/tests/serve.rs#a_second_submission_while_one_is_in_flight_is_refused",
      "crates/jj-acp-cli/tests/serve.rs#cancelling_a_submission_that_is_not_in_flight_is_refused",
      "crates/jj-acp-cli/tests/serve.rs#a_runner_listing_is_answered_without_starting_an_agent",
      "crates/jj-acp-cli/tests/serve.rs#malformed_and_unsupported_frames_are_refused_and_the_loop_continues",
      "crates/jj-acp-cli/tests/serve.rs#an_invalid_task_document_is_reported_as_a_record_not_an_error",
      "crates/jj-acp-cli/tests/serve.rs#end_of_input_ends_the_loop_cleanly"
    ],
    "runs": [
      {
        "command": "cargo test -p jj-acp-cli --test serve --locked",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo test --workspace --locked (three consecutive clean runs)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo clippy --workspace --all-targets --locked -- -D warnings",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "node --test scripts/governance.test.mjs scripts/rust-architecture.test.mjs scripts/upstream-lock.test.mjs",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "control protocol version 1; Rust 1.98.1; tokio 1.53.1",
    "formats": "newline-delimited control frames; requests submit/cancel/runners/shutdown; responses ready/record/runners/ack/error/diagnostic",
    "note": "The signals test was intermittently failing under parallel workspace load. The cause was the test, not the product: it raised the idle bound to 90 seconds but left the absolute cap at 60, so a deadline could end the turn before the signal arrived and report exit 124 instead of the signal code. Both bounds now exceed any plausible startup delay and three consecutive full workspace runs are clean. Execution is serial in this slice; a standing pool with caller-named scopes is M9."
  },
  {
    "id": "E-M9-001",
    "tasks": ["M9"],
    "date": "2026-10-01",
    "kind": "scheduler",
    "files": [
      "crates/jj-acp-control/src/scheduler.rs",
      "crates/jj-acp-control/src/serve.rs",
      "crates/jj-acp-control/src/transport.rs",
      "crates/jj-acp-control/src/protocol.rs",
      "crates/jj-acp-control/src/lib.rs",
      "crates/jj-acp-control/tests/serve.rs",
      "crates/jj-acp-cli/src/args.rs",
      "crates/jj-acp-cli/src/serve.rs",
      "crates/jj-acp-cli/tests/serve.rs",
      "spec/src/control.md",
      "spec/src/cli.md",
      "spec/src/milestones.md",
      "spec/src/verification.md"
    ],
    "tests": [
      "crates/jj-acp-control/tests/serve.rs#the_ready_frame_announces_sizing_and_the_catalogue",
      "crates/jj-acp-control/tests/serve.rs#one_scope_serializes_while_distinct_scopes_overlap",
      "crates/jj-acp-control/tests/serve.rs#a_crashed_agent_releases_its_slot_without_re_dispatch",
      "crates/jj-acp-control/tests/serve.rs#a_malformed_frame_never_stops_the_loop",
      "crates/jj-acp-control/tests/serve.rs#a_submission_completes_and_the_loop_keeps_serving",
      "crates/jj-acp-cli/tests/serve.rs#submissions_in_one_scope_serialize_rather_than_overlap",
      "crates/jj-acp-cli/tests/serve.rs#submissions_in_distinct_scopes_run_concurrently",
      "crates/jj-acp-cli/tests/serve.rs#a_full_queue_is_refused_rather_than_grown",
      "crates/jj-acp-cli/tests/serve.rs#a_crashed_agent_releases_its_slot_and_is_never_re_dispatched",
      "crates/jj-acp-cli/tests/args.rs#malformed_command_lines_are_refused_before_any_work"
    ],
    "runs": [
      {
        "command": "cargo test -p jj-acp-control --locked",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo test -p jj-acp-cli --test serve --locked",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo test --workspace --locked",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo clippy --workspace --all-targets --locked -- -D warnings",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "node scripts/rust-architecture.mjs",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "node --test scripts/governance.test.mjs scripts/rust-architecture.test.mjs scripts/upstream-lock.test.mjs",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "control protocol version 1 with scopes; Rust 1.98.1; tokio 1.53.1",
    "formats": "requests submit/cancel/runners/status/shutdown; responses ready/queued/record/runners/status/ack/error/diagnostic",
    "note": "Two defects were found while building this. Emitting a record directly from the worker task let a peer observe a record and then a status still listing that submission as running, so completions now route through the loop and release the scope first. And serve.rs reached 334 counted lines, which the budget gate rejected; the scheduler was extracted rather than baselined. The M8 contract changed deliberately: a second submission in the same scope is now queued and acknowledged rather than refused with busy, because serializing a scope means the second turn waits. Warm agent reuse across turns is not implemented; each turn spawns and tears down its own agent process. The M8 test asserting a busy refusal was replaced by scope-serialization coverage and the replacement is recorded as a tracked removal.",
    "removed_tests": [
      "crates/jj-acp-cli/tests/serve.rs#a_second_submission_while_one_is_in_flight_is_refused"
    ]
  },
  {
    "id": "E-BOOT-003",
    "tasks": ["Bootstrap.2"],
    "date": "2026-10-01",
    "kind": "governance",
    "files": [
      "Cargo.toml",
      "Cargo.lock",
      "package.json",
      "spec-validator.config.mjs",
      "spec/book.toml",
      "AGENTS.md",
      "README.md",
      "scripts/check-boundaries.mjs",
      "scripts/check-progress.mjs",
      "scripts/rust-architecture.mjs",
      "scripts/governance.test.mjs",
      "scripts/rust-architecture.test.mjs",
      "docs/upstream/disposition.json",
      "docs/upstream/lock.json",
      "spec/src/architecture.md",
      "spec/src/spec-governance.md",
      "crates/jj-acp-cli/tests/signals.rs"
    ],
    "tests": [
      "scripts/governance.test.mjs#accepts_verified_task",
      "scripts/governance.test.mjs#rejects_excluded_nostr_transport_and_database_dependencies",
      "scripts/governance.test.mjs#the_testkit_may_depend_on_the_layers_it_exercises",
      "scripts/governance.test.mjs#the_protocol_crate_declares_no_process_or_network_facility",
      "scripts/rust-architecture.test.mjs#allows_the_testkit_crate_to_name_itself",
      "crates/jj-acp-cli/tests/signals.rs#sigterm_produces_bounded_cleanup_and_exit_143",
      "crates/jj-acp-cli/tests/signals.rs#sigint_produces_bounded_cleanup_and_exit_130"
    ],
    "runs": [
      {
        "command": "cargo build --workspace --locked",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo test --workspace --locked (three consecutive full runs)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo clippy --workspace --all-targets --locked -- -D warnings",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "node --test scripts/governance.test.mjs scripts/rust-architecture.test.mjs scripts/upstream-lock.test.mjs",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "node scripts/check-boundaries.mjs && node scripts/rust-architecture.mjs && node scripts/check-progress.mjs && node scripts/upstream-sync.mjs --self-test",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "spec-validator validate && spec-validator first && spec-validator check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "jj-acp --version && jj-acp agents",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Rust 1.98.1; Node v24.10.0; pnpm 10.34.5; @lapismd/spec-validator 0.1.0; binary jj-acp 0.1.0",
    "formats": "requirement identifiers JAC-<AREA>-NNN; environment namespace JJ_ACP_*; reserved task identity JJ_ACP_TASK_*; crate names jj-acp-<layer>",
    "note": "Renamed the repository, six crates, the binary, the npm package, the requirement prefix and the environment namespace from lapis-acp to jj-acp in one atomic change. The rename exposed a change-map defect: crate rules keyed on the literal product name, so deletions at the old paths matched the protected list but no rule and the rename commit failed as unmapped. The rules now key on the crate layer, which is what actually determines the owning chapter, and still fail closed for a crate whose layer is not listed. It also exposed that the signals test startup bound was tuned to an idle machine; a full workspace run starts roughly fifteen binaries in parallel and the 30 second wait tripped intermittently, so the bound is now set so that only a genuine failure trips it. Three consecutive full workspace runs are clean."
  },
  {
    "id": "E-M11-001",
    "tasks": ["M11"],
    "date": "2026-10-06",
    "kind": "interactive-control",
    "files": [
      "README.md",
      "crates/jj-acp-cli/src/args.rs",
      "crates/jj-acp-cli/src/control_version.rs",
      "crates/jj-acp-cli/src/lib.rs",
      "crates/jj-acp-cli/src/serve.rs",
      "crates/jj-acp-cli/tests/interactive.rs",
      "crates/jj-acp-control/src/interactive.rs",
      "crates/jj-acp-control/src/interactive_handler.rs",
      "crates/jj-acp-control/src/interactive_session.rs",
      "crates/jj-acp-control/src/interactive_wire.rs",
      "crates/jj-acp-control/src/interactive_worker.rs",
      "crates/jj-acp-control/src/lib.rs",
      "crates/jj-acp-control/src/transport.rs",
      "crates/jj-acp-control/tests/interactive_wire.rs",
      "crates/jj-acp-protocol/src/client.rs",
      "crates/jj-acp-protocol/src/handler.rs",
      "crates/jj-acp-runner/src/launch.rs",
      "crates/jj-acp-runner/src/lib.rs",
      "crates/jj-acp-runner/src/persistent.rs",
      "crates/jj-acp-runner/src/process.rs",
      "spec/src/architecture.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/control.md",
      "spec/src/index.md",
      "spec/src/milestones.md",
      "spec/src/protocol.md",
      "spec/src/runner.md",
      "spec/src/verification.md",
      "tests/fixtures/interactive_agent.py"
    ],
    "tests": [
      "crates/jj-acp-cli/tests/interactive.rs#follow_up_reuses_the_session_and_streams_activity",
      "crates/jj-acp-cli/tests/interactive.rs#permissions_validate_exact_options_and_expire_after_resolution",
      "crates/jj-acp-cli/tests/interactive.rs#cancellation_remains_serviceable_during_permission_wait",
      "crates/jj-acp-cli/tests/interactive.rs#permission_wait_is_inside_the_absolute_deadline",
      "crates/jj-acp-cli/tests/interactive.rs#unsupported_modes_block_prompts_and_duplicates_never_run_again",
      "crates/jj-acp-cli/tests/interactive.rs#resume_is_explicit_and_does_not_send_a_prompt",
      "crates/jj-acp-control/tests/interactive_wire.rs#partial_frames_survive_cancelled_reads"
    ],
    "runs": [
      {
        "command": "cargo test -p jj-acp-cli --test serve --locked (before)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo test -p jj-acp-cli --test interactive -p jj-acp-control --test interactive_wire --locked",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo clippy --workspace --all-targets --locked -- -D warnings (initial)",
        "result": "fail",
        "failures": 4
      },
      {
        "command": "cargo clippy --workspace --all-targets --locked -- -D warnings",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "node scripts/rust-architecture.mjs (initial)",
        "result": "fail",
        "failures": 1
      },
      {
        "command": "node scripts/rust-architecture.mjs",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm test",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm build",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm spec:check",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Rust 1.98.1; macOS ARM64; control v2; deterministic Python peer",
    "formats": "Explicit interactive control v2; existing task/record/v1 control contracts retained",
    "note": "Before: v1 serve 11/11 and no interactive control. New target: six binary and three transport/input tests pass. Two compile/format repair attempts preceded the passing target; four clippy findings and one 301-line budget failure were repaired without suppressions. Signal/drop containment has a process-group fallback. Cross-layer changes require the full source check/test/build/spec lanes. All four required wider gates passed. Real interactive host qualification remains a distinct M10 gate."
  },
  {
    "id": "E-M11-002",
    "tasks": ["M11"],
    "date": "2026-10-06",
    "kind": "interactive-overflow-regression",
    "files": [
      "crates/jj-acp-control/src/interactive.rs",
      "crates/jj-acp-control/src/interactive_handler.rs",
      "crates/jj-acp-control/src/interactive_worker.rs",
      "crates/jj-acp-control/tests/interactive_overflow.rs",
      "spec/src/control.md",
      "spec/src/milestones.md",
      "spec/src/verification.md"
    ],
    "tests": [
      "crates/jj-acp-control/tests/interactive_overflow.rs#output_overflow_never_reports_completed_or_reusable_session"
    ],
    "runs": [
      {
        "command": "cargo test -p jj-acp-control --test interactive_overflow --locked (initial)",
        "result": "fail",
        "failures": 1
      },
      {
        "command": "cargo test -p jj-acp-control --test interactive_overflow --locked",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm check (initial, before ledger update)",
        "result": "fail",
        "failures": 2
      },
      {
        "command": "pnpm check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm test",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm build",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm spec:check",
        "result": "pass",
        "failures": 0
      }
    ],
    "remaining": [],
    "versions": "Rust 1.98.1; control version 2",
    "formats": "control/v2; terminal record/v1 unchanged",
    "note": "Consumer integration uncovered a completed-turn race after event output overflow. Coverage loss now remains latched after queue recovery and closes the session; terminal records report outcome_unknown/activity_coverage_lost. watch::send_replace also retains cancellation without a current subscriber. M11 review: fixture gate remains Done after all four required source gates passed; M10 real host acceptance stays separate.",
    "result": "pass"
  },
  {
    "id": "E-M11-003",
    "tasks": ["M11"],
    "date": "2026-10-06",
    "kind": "queued-cancellation-regression",
    "files": [
      "crates/jj-acp-control/src/interactive_worker.rs",
      "crates/jj-acp-cli/tests/interactive.rs",
      "spec/src/control.md",
      "spec/src/milestones.md",
      "spec/src/verification.md",
      "spec/src/cli.md",
      "spec/src/runner.md"
    ],
    "tests": [
      "crates/jj-acp-cli/tests/interactive.rs#cancelling_a_queued_prompt_keeps_the_warm_session_usable"
    ],
    "runs": [
      {
        "command": "cargo test -p jj-acp-cli --test interactive --locked cancelling_a_queued_prompt_keeps_the_warm_session_usable (before)",
        "result": "fail",
        "failures": 1
      },
      {
        "command": "cargo test -p jj-acp-cli --test interactive --locked cancelling_a_queued_prompt_keeps_the_warm_session_usable",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm check (before ledger update)",
        "result": "fail",
        "failures": 2
      },
      {
        "command": "pnpm check (partial was not an accepted ledger result)",
        "result": "fail",
        "failures": 1
      },
      {
        "command": "pnpm check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm test",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm build",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm spec:check (before mapped CLI chapter update)",
        "result": "fail",
        "failures": 1
      },
      {
        "command": "pnpm spec:check",
        "result": "pass",
        "failures": 0
      }
    ],
    "remaining": [],
    "versions": "Rust 1.98.1; control version 2",
    "formats": "control/v2; terminal record/v1 unchanged",
    "note": "Enclosing revision mkmlwqzt. Queued cancellation previously omitted sessionUsable even while retaining the warm session; jj-store would mark that live session interrupted. The terminal frame now reports usability and the next explicit prompt succeeds. M11 review: remains Done after its required wider gates were rerun; M10 real host acceptance remains separate.",
    "result": "pass"
  },
  {
    "id": "E-M10-001",
    "tasks": ["M10"],
    "date": "2026-10-06",
    "kind": "real-consumer",
    "files": [
      "README.md",
      "spec/src/architecture.md",
      "spec/src/verification.md",
      "spec/src/milestones.md"
    ],
    "tests": [],
    "runs": [
      {
        "command": "In jj-store: PATH=/Users/stevejuma/.npm/_npx/bb64e4387f65cc68/node_modules/.bin:$PATH JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 JJ_STORE_AGENT_TEST_ACP=/Users/stevejuma/code/lapismd/jj-acp/target/debug/jj-acp cargo test -p jj-store-cli --test agent_execution --locked real::installed_codex_and_cursor_edit_isolated_workspaces_and_publish_history -- --ignored --exact",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "In jj-store: PATH=/Users/stevejuma/.npm/_npx/bb64e4387f65cc68/node_modules/.bin:$PATH JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 python3 scripts/agent-execution-acceptance.py --server target/debug/jj-store --jj target/debug/jj --rustfs runtime/embedded-rustfs/target/debug/jj-store-rustfs --acp /Users/stevejuma/code/lapismd/jj-acp/target/debug/jj-acp --web web/dist --browser",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "In jj-store: JJ_STORE_DESKTOP_TEST_RESOURCES=$PWD/desktop/resources JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 cargo test --manifest-path desktop/Cargo.toml --test managed_runtime --locked real_managed_runtime_restarts_preserves_origin_and_rejects_occupied_port -- --ignored --exact",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm check:progress",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm spec:first",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm spec:validate",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "macOS ARM64; Rust 1.98.1; JJ 0.45.1; codex-acp 1.13.1 from existing npx cache; Cursor 2026.10.01-e373342; source binary mkmlwqzt/5f7e3fe1; host revision zontovyv with E-EXEC-001/002.",
    "formats": "Interactive control/v2 and terminal record/v1 unchanged. The host owns private device journals, exact JJ workspace selection and sanitized AgentStore publication; no harness persistence or authority was added.",
    "note": "Enclosing source change somnxvvl. M10 is Done after real jj-store consumer qualification: both installed adapters edited isolated ordinary Git-backed JJ and native real-RustFS workspaces, preserved dirty source files and published sanitized repository history. Cold production Chromium saved a named setup, launched/streamed/ended a native session, opened retained review and passed Axe including color contrast. Cold host restart retained exact session/event sequences, and explicit capability-gated resume or labelled saved-context continuation dispatched no prompt. Managed owner-EOF/restart lifetime passed separately. The host repaired newline-bearing Cursor projection aliases, independent saved-setup revisions, stream error lifetime and caption contrast in its owning modules; fixture cancellation usability was repaired in source E-M11-003 and passed all four source gates. 0 live permission requests were raised by these real edits; fixture permission/cancellation/deadline evidence remains distinct. Signed app/DMG/WKWebView, other platforms/adapters, live permission qualification and distribution remain outside this consumer profile. Optional unrelated host clippy diagnostics are recorded in E-EXEC-001, not claimed as a passing lane. M11 stays Done; no milestone status is inferred from compilation."
  },
  {
    "id": "E-M12-001",
    "tasks": ["M12"],
    "date": "2026-10-07",
    "kind": "warm worker control and session fencing",
    "versions": "macOS ARM64; Rust 1.98.1; JJ 0.45.1; installed Codex ACP 1.13.1 from existing npx cache; enclosing change yxwxpvkv",
    "formats": "control version 3; existing versions 1/2 unchanged; terminal record version 1",
    "files": [
      "crates/jj-acp-cli/src/args.rs",
      "crates/jj-acp-cli/src/control_version.rs",
      "crates/jj-acp-cli/src/serve.rs",
      "crates/jj-acp-cli/tests/fleet.rs",
      "crates/jj-acp-control/src/fleet.rs",
      "crates/jj-acp-control/src/fleet_wire.rs",
      "crates/jj-acp-control/src/fleet_worker.rs",
      "crates/jj-acp-control/src/interactive.rs",
      "crates/jj-acp-control/src/interactive_handler.rs",
      "crates/jj-acp-control/src/interactive_worker.rs",
      "crates/jj-acp-control/src/lib.rs",
      "crates/jj-acp-control/src/warm_worker.rs",
      "crates/jj-acp-control/tests/interactive_overflow.rs",
      "crates/jj-acp-protocol/src/client.rs",
      "crates/jj-acp-protocol/src/handler.rs",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/control.md",
      "spec/src/protocol.md",
      "spec/src/runner.md",
      "spec/src/verification.md",
      "tests/fixtures/fleet_agent.py",
      "spec/src/milestones.md"
    ],
    "tests": [
      "crates/jj-acp-cli/tests/fleet.rs#warm_worker_opens_distinct_sessions_without_reinitializing"
    ],
    "runs": [
      {
        "command": "CARGO_TARGET_DIR=../jj-acp/target cargo test -p jj-acp-cli --test fleet --locked (before)",
        "result": "fail",
        "failures": 1
      },
      {
        "command": "CARGO_TARGET_DIR=../jj-acp/target cargo test -p jj-acp-cli --test interactive --locked (before)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "CARGO_TARGET_DIR=../jj-acp/target cargo test -p jj-acp-cli --test fleet --test interactive --locked",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "CARGO_TARGET_DIR=../jj-acp/target cargo test -p jj-acp-cli --test fleet --locked (scope fencing)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo check -p jj-acp-cli --locked (temporary borrow fixes)",
        "result": "fail",
        "failures": 2
      },
      {
        "command": "cargo test -p jj-acp-cli --test fleet --locked (fixture configuration correction)",
        "result": "fail",
        "failures": 1
      },
      {
        "command": "pnpm spec:check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm test",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm check (queue Result lint)",
        "result": "fail",
        "failures": 1
      },
      {
        "command": "pnpm check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm build",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo test -p jj-acp-cli --test fleet --locked (scoped responses and release cancellation)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "python3 /tmp/jj-acp-fleet-real.py (initial two-session installed adapter run)",
        "result": "fail",
        "failures": 1,
        "note": "Second turn invalidated by delayed update from the known inactive first adapter session; no uncertain prompt replay."
      },
      {
        "command": "CARGO_TARGET_DIR=../jj-acp/target cargo test --locked -p jj-acp-cli --test fleet (known inactive session regression)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "python3 /tmp/jj-acp-fleet-real.py",
        "result": "pass",
        "failures": 0,
        "note": "Installed Codex ACP 1.13.1 warm without session/prompt; two distinct application sessions completed read-only turns on one initialized worker; exact scope; close reaped; zero permission requests."
      },
      {
        "command": "PATH=<existing adapter cache>/node_modules/.bin:$PATH JJ_STORE_AGENT_TEST_ACP=<rebuilt jj-acp> CARGO_TARGET_DIR=../jj-store/target cargo test --locked -p jj-store-cli --test agent_execution real::installed_fleet_warms_general_chat_then_reuses_an_attached_workspace -- --ignored --exact --nocapture",
        "result": "pass",
        "failures": 0,
        "note": "Real jj-store consumer: general turn, attached isolated workspace edit, preserved dirty source, canonical publication and exact duplicate without replay; zero permissions."
      },
      {
        "command": "CARGO_TARGET_DIR=../jj-acp/target pnpm spec:check and pnpm check (before final Markdown formatting)",
        "result": "fail",
        "failures": 2,
        "note": "control/protocol Markdown required Prettier; formatted in place."
      },
      {
        "command": "CARGO_TARGET_DIR=../jj-acp/target pnpm spec:check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "CARGO_TARGET_DIR=../jj-acp/target pnpm check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "CARGO_TARGET_DIR=../jj-acp/target pnpm test",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "CARGO_TARGET_DIR=../jj-acp/target pnpm build",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pending",
    "remaining": [
      "Cold native jj-store integration, other version-3 adapters/platforms and real permission/cancellation exchange are separate qualification gates."
    ],
    "note": "Enclosing change yxwxpvkv. Warm initialization sends neither session/new nor a prompt. Multiple session directories share one initialized adapter. Exact application scopes fence permissions, cancellation and events. Release cancels active/queued turns in its own scope. The installed Codex regression exposed delayed known-inactive session updates; these are now discarded and stale permissions cancelled, while unknown identities fail the worker. No uncertain prompt is replayed. M12 remains In progress with fixture and installed Codex gates passing; native cold consumer and other adapters remain separately qualified."
  },
  {
    "id": "E-M12-002",
    "tasks": ["M12"],
    "date": "2026-10-07",
    "kind": "separate prepared context blocks",
    "versions": "macOS ARM64; Rust 1.98.1; JJ 0.45.1; enclosing change ylkuyxzx; fixture evidence only",
    "formats": "additive optional context in controls v2/v3; ordered ACP text/resource blocks; terminal record v1 unchanged",
    "files": [
      "crates/jj-acp-control/src/interactive_wire.rs",
      "crates/jj-acp-control/src/interactive_worker.rs",
      "crates/jj-acp-control/src/fleet_worker.rs",
      "crates/jj-acp-runner/src/persistent.rs",
      "crates/jj-acp-protocol/src/session.rs",
      "crates/jj-acp-protocol/tests/payload.rs",
      "crates/jj-acp-control/tests/interactive_wire.rs",
      "crates/jj-acp-cli/tests/fleet.rs",
      "tests/fixtures/fleet_agent.py",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/runner.md",
      "spec/src/control.md",
      "spec/src/protocol.md",
      "spec/src/verification.md",
      "spec/src/milestones.md"
    ],
    "tests": [
      "crates/jj-acp-protocol/tests/payload.rs#context_blocks_are_capability_gated_and_preserve_authored_bytes",
      "crates/jj-acp-control/tests/interactive_wire.rs#prompt_context_is_optional_bounded_data_without_launch_authority",
      "crates/jj-acp-cli/tests/fleet.rs#warm_worker_opens_distinct_sessions_without_reinitializing"
    ],
    "runs": [
      {
        "command": "cargo test -p jj-acp-protocol --test payload -p jj-acp-control --test interactive_wire --locked",
        "result": "pass",
        "detail": "27 protocol tests and 4 control tests pass.",
        "failures": 0
      },
      {
        "command": "pnpm test",
        "result": "pass",
        "detail": "Full source tests passed before the later fixture assertion addition.",
        "failures": 0
      },
      {
        "command": "pnpm build",
        "result": "pass",
        "detail": "All source binaries built before package cache cleanup.",
        "failures": 0
      },
      {
        "command": "CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo test -p jj-acp-cli --test fleet --locked",
        "result": "pass",
        "failures": 0,
        "note": "Exact resource/authored block assertions and scope fencing pass after cache cleanup."
      },
      {
        "command": "pnpm check",
        "result": "pass",
        "failures": 0,
        "detail": "Full source formatting, types, boundaries, architecture and clippy all targets pass."
      },
      {
        "command": "pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "detail": "All 11 specification lanes pass after the runner, CLI and compatibility topic updates."
      },
      {
        "command": "CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 pnpm test",
        "result": "fail",
        "failures": 1,
        "detail": "The forced descendant cleanup fixture missed its PID under load; no product assertion was loosened."
      },
      {
        "command": "CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo test -p jj-acp-cli --test cleanup a_forced_termination_takes_the_agents_descendant_with_it --locked -- --exact",
        "result": "pass",
        "failures": 0,
        "detail": "The exact cleanup regression passes unchanged."
      },
      {
        "command": "CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 pnpm test",
        "result": "pass",
        "failures": 0,
        "detail": "Final complete source test and doctest rerun passes, including resource/authored block fixture assertions."
      },
      {
        "command": "CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 pnpm build",
        "result": "pass",
        "failures": 0,
        "detail": "Final locked build passes."
      },
      {
        "command": "pnpm spec:check",
        "result": "fail",
        "failures": 1,
        "detail": "Progress validation requires pending while a live interpretation gap remains; restored the scoped pending result without removing that gate."
      }
    ],
    "result": "pending",
    "remaining": [
      "Installed adapter interpretation of embedded context resources remains an independent live qualification gate; these results are fixture and source evidence."
    ],
    "note": "Prepared context is separate from authored text. Embedded resources require advertised support, otherwise context uses an independent text block in the same turn. No preliminary turn or replay. Initial check refused the missing evidence update; the first fixture assertion used a text-only prompt and was repaired to send context. Its next compilation ran out of disk space; generated package caches were cleared. M12 remains In progress and real installed-adapter qualification is not inferred from fixture block assertions. Exact cleanup retry and the final full test rerun pass unchanged. Full check, all specification lanes and final build pass. No package was published, installed binary replaced or real prompt sent."
  },
  {
    "id": "E-M12-003",
    "tasks": ["M12"],
    "date": "2026-10-09",
    "kind": "comments-only prepared prompt context",
    "versions": "macOS ARM64; Rust 1.98.1; enclosing Jujutsu change lwlxyuwu",
    "formats": "control versions 2/3; optional prepared context; record v1 unchanged",
    "files": [
      "crates/jj-acp-control/src/interactive_wire.rs",
      "crates/jj-acp-control/tests/interactive_wire.rs",
      "crates/jj-acp-cli/tests/fleet.rs",
      "spec/src/control.md",
      "spec/src/verification.md",
      "spec/src/milestones.md",
      "spec/src/cli.md"
    ],
    "tests": [
      "crates/jj-acp-control/tests/interactive_wire.rs#context_only_prompt_requires_nonempty_prepared_content",
      "crates/jj-acp-cli/tests/fleet.rs#warm_worker_opens_distinct_sessions_without_reinitializing"
    ],
    "runs": [
      {
        "command": "cargo test -p jj-acp-control --test interactive_wire context_only_prompt_requires_nonempty_prepared_content --locked -- --exact (initial test)",
        "result": "fail",
        "failures": 1,
        "detail": "Test initially referenced a private module; removed that access and kept version-3 coverage in the CLI fleet test."
      },
      {
        "command": "cargo test -p jj-acp-control --test interactive_wire context_only_prompt_requires_nonempty_prepared_content --locked -- --exact",
        "result": "fail",
        "failures": 1,
        "detail": "Empty authored text with nonempty prepared context was refused."
      },
      {
        "command": "cargo test -p jj-acp-control --test interactive_wire --locked",
        "result": "pass",
        "failures": 0,
        "detail": "5 tests."
      },
      {
        "command": "pnpm build",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm test",
        "result": "pass",
        "failures": 0,
        "detail": "Full source tests, governance and doctests including version-3 context-only fixture."
      },
      {
        "command": "pnpm check",
        "result": "fail",
        "failures": 1,
        "detail": "Evidence test references needed file paths; corrected the references."
      },
      {
        "command": "pnpm spec:check",
        "result": "fail",
        "failures": 1,
        "detail": "Evidence test references needed file paths; corrected the references."
      },
      {
        "command": "pnpm check (corrected evidence)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm spec:check (corrected evidence)",
        "result": "fail",
        "failures": 1,
        "detail": "Spec-first requires the owning CLI chapter for its fleet-test change; added comments-only CLI contract."
      },
      {
        "command": "pnpm spec:check (CLI chapter updated)",
        "result": "pass",
        "failures": 0,
        "detail": "11 lanes, including full tests, formatting and spec-first."
      }
    ],
    "result": "pending",
    "remaining": [
      "Other v3 adapters/platforms and real permission/cancellation exchanges remain separate M12 gates."
    ],
    "note": "The jj-store consumer exposed an old staged binary rejecting the optional context field and the current parser rejecting comments-only sends. Rebuilt source accepts nonblank context with empty authored text, still refuses fully empty prompts, and preserves the exact text/context blocks without manufactured message content. M12 remains In progress; no added credentials, launch authority or retries. The rebuilt helper was staged only in the jj-store-chat-body ignored development runtime. Its installed Codex consumer completed a read-only sample prompt and a comments-only turn answering three comments at two anchors, with two committed answer links. Consumer evidence is E-CHAT-LIVE-007; no release or installed CLI replacement."
  },
  {
    "id": "E-M12-004",
    "tasks": ["M12"],
    "date": "2026-10-10",
    "kind": "request-scoped session rejection",
    "versions": "macOS ARM64; Rust 1.98.1; enclosing Jujutsu change mtvyswyw",
    "formats": "control v3 unchanged; version-1 records unchanged",
    "files": [
      "crates/jj-acp-control/src/fleet_worker.rs",
      "crates/jj-acp-cli/tests/fleet.rs",
      "spec/src/control.md",
      "spec/src/runner.md",
      "spec/src/cli.md",
      "spec/src/verification.md",
      "spec/src/milestones.md"
    ],
    "tests": [
      "crates/jj-acp-cli/tests/fleet.rs#warm_worker_opens_distinct_sessions_without_reinitializing"
    ],
    "runs": [
      {
        "command": "cargo test -p jj-acp-cli --test fleet warm_worker_opens_distinct_sessions_without_reinitializing --locked -- --exact (before)",
        "result": "fail",
        "failures": 1,
        "detail": "A valid rejected load invalidated another configured scope; next configure reported session_not_open."
      },
      {
        "command": "cargo test -p jj-acp-cli --test fleet warm_worker_opens_distinct_sessions_without_reinitializing --locked -- --exact",
        "result": "pass",
        "failures": 0,
        "detail": "Rejected load leaves existing configured scope usable; foreign-session uncertainty still invalidates it."
      },
      {
        "command": "pnpm check (initial)",
        "result": "fail",
        "failures": 1,
        "detail": "Three edited canonical Markdown files needed formatting."
      },
      {
        "command": "pnpm exec prettier --write spec/src/control.md spec/src/runner.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm check (formatted)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm test",
        "result": "pass",
        "failures": 0,
        "detail": "Full governance, crate tests and doctests."
      },
      {
        "command": "pnpm build",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "jj-store cold UI Send with staged rebuilt helper",
        "result": "pass",
        "failures": 0,
        "detail": "Exact MIT prompt completed, request 2481f6b2b8cd46fcb747e5860b281a55; one LICENSE change, published 435/435. Consumer E-AGENT-RESUME-001."
      },
      {
        "command": "pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "detail": "All 11 source lanes including full tests, mdBook and spec-first."
      }
    ],
    "result": "pending",
    "remaining": [
      "Other adapters/platforms and independent live permission/cancellation gates remain."
    ],
    "note": "M12 remains In progress. Valid RPC open/configure rejection preserves the initialized worker and unrelated scopes, invalidating only the rejected scope. Uncertain prompt execution, timeout, protocol failure and process exit retain worker-wide invalidation. No prompt retry or raw diagnostic publication."
  },
  {
    "id": "E-TOOLS-001",
    "tasks": ["M12"],
    "requirements": [
      "JAC-CLI-008",
      "JAC-PROTO-008",
      "JAC-RUN-009",
      "JAC-CTRL-009"
    ],
    "date": "2026-10-07",
    "files": [
      "crates/jj-acp-cli/src/tools.rs",
      "crates/jj-acp-control/src/invocation.rs",
      "crates/jj-acp-cli/tests/fleet.rs"
    ],
    "tests": [
      "crates/jj-acp-cli/src/tools/tests.rs#trusted_tools_refuse_duplicate_names_relative_commands_and_reserved_identity",
      "crates/jj-acp-cli/tests/fleet.rs#warm_worker_opens_distinct_sessions_without_reinitializing",
      "crates/jj-acp-cli/src/tools/tests.rs#trusted_tools_are_bounded_and_diagnostics_do_not_echo_values"
    ],
    "runs": [
      {
        "command": "CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-acp/target pnpm check",
        "result": "fail",
        "failures": 1,
        "note": "Progress checker correctly rejected missing same-change evidence. Added this record before rerunning."
      },
      {
        "command": "CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-acp/target pnpm check",
        "result": "fail",
        "failures": 1,
        "note": "Evidence schema required kind/versions/formats and actual test file paths. Corrected record."
      },
      {
        "command": "CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-acp/target pnpm check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-acp/target pnpm test",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-acp/target pnpm build",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm spec:check (first run)",
        "result": "fail",
        "failures": 1,
        "note": "All executable lanes passed. Spec-first required compatibility chapter for the modified fleet fixture; added owning explanation."
      },
      {
        "command": "CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-acp/target pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "11 lanes, including cold source tests and mapped canonical chapter check."
      }
    ],
    "result": "pass",
    "remaining": [],
    "note": "Enclosing change mkopntkz. Trusted bounded launch declarations extend control v2/v3 and one-shot session creation/load. Task/control input never selects tools. M12 remains In progress; new source fixtures do not close installed adapter/platform qualification.",
    "kind": "verification",
    "versions": "macOS ARM64; Rust 1.98.1; ACP 1; stdio control v2/v3",
    "formats": "Trusted tool JSON; no durable/public format change"
  },
  {
    "id": "E-TOOLS-002",
    "tasks": ["M12"],
    "date": "2026-10-14",
    "kind": "workspace integration",
    "versions": "Rust 1.98.1; macOS ARM64; JJ 0.45.1",
    "formats": "task/record v1; control v2/v3; trusted tool JSON unchanged",
    "files": [
      "crates/jj-acp-cli/tests/fleet.rs",
      "crates/jj-acp-cli/src/tools.rs",
      "spec/src/cli.md",
      "spec/src/milestones.md"
    ],
    "tests": [
      "crates/jj-acp-cli/src/tools/tests.rs#trusted_tools_refuse_duplicate_names_relative_commands_and_reserved_identity",
      "crates/jj-acp-cli/tests/fleet.rs#warm_worker_opens_distinct_sessions_without_reinitializing"
    ],
    "runs": [
      {
        "command": "cargo test -p jj-acp-cli --test fleet --locked (before incorporation)",
        "result": "pass",
        "failures": 0,
        "detail": "1 test passed; main still omitted trusted tool support."
      },
      {
        "command": "target/debug/jj-acp agents --tools /dev/null (before incorporation)",
        "result": "fail",
        "failures": 1,
        "detail": "Exit 2: unknown option --tools."
      },
      {
        "command": "cargo test -p jj-acp-cli --test fleet --test interactive --lib --locked",
        "result": "pass",
        "failures": 0,
        "detail": "2 tools tests, 1 fleet test and 7 interactive tests passed after combining both lineages."
      },
      {
        "command": "pnpm spec:check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm test",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm build",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pass",
    "remaining": [],
    "note": "Rebased local-compute change mkopntkz onto mtvyswyw; preserved prepared context, comments-only prompts and request-scoped RPC rejection. Historical evidence retained. Live v3 qualification remains separate."
  },
  {
    "id": "E-RELEASE-001",
    "tasks": ["M13"],
    "date": "2026-10-14",
    "kind": "release preparation",
    "versions": "Rust 1.98.1; Node 26.9.0 locally / Node 22 in CI; pnpm 10.34.5; mdBook 0.5.4; macOS ARM64",
    "formats": "coordinated crate 0.1.0; task/record v1; controls v1/v2/v3 unchanged",
    "files": [
      "Cargo.toml",
      "Cargo.lock",
      "README.md",
      "LICENSE.md",
      "scripts/progress-vcs.mjs",
      "scripts/progress-vcs.test.mjs",
      "scripts/release.test.mjs",
      "scripts/package-check.mjs",
      ".github/workflows/ci.yml",
      ".github/workflows/release.yml",
      ".github/workflows/publish-spec-pages.yml",
      "spec/src/release.md",
      "spec/src/verification.md",
      "crates/jj-acp-cli/tests/common/mod.rs",
      "crates/jj-acp-cli/tests/scratch.rs",
      "scripts/release-assets.mjs"
    ],
    "removed_files": [
      "tests/fixtures/fixture_agent.py",
      "tests/fixtures/fleet_agent.py",
      "tests/fixtures/interactive_agent.py"
    ],
    "tests": [
      "scripts/progress-vcs.test.mjs#explicit_git_range_wins_over_local_jj",
      "scripts/progress-vcs.test.mjs#incomplete_range_fails_closed",
      "scripts/progress-vcs.test.mjs#initial_git_commit_uses_empty_tree",
      "scripts/release.test.mjs#release_refuses_mismatched_or_nonversion_tags",
      "crates/jj-acp-testkit/tests/fixture.rs#the_fixture_completes_a_turn_without_a_model_provider",
      "crates/jj-acp-cli/tests/scratch.rs#concurrent_scratch_directories_survive_peer_teardown",
      "crates/jj-acp-cli/tests/signals.rs#sigterm_produces_bounded_cleanup_and_exit_143"
    ],
    "runs": [
      {
        "command": "node scripts/check-progress.mjs (Git-only checkout before fix)",
        "result": "fail",
        "failures": 1,
        "detail": "Required jj metadata absent from ordinary Git checkout."
      },
      {
        "command": "node --test scripts/progress-vcs.test.mjs scripts/release.test.mjs",
        "result": "pass",
        "failures": 0,
        "detail": "6 tests passed."
      },
      {
        "command": "pnpm spec:validate (initial release chapter)",
        "result": "fail",
        "failures": 5,
        "detail": "Verification rows were accidentally separated from their table; fixed the canonical map."
      },
      {
        "command": "pnpm package:check (uncommitted preparation)",
        "result": "fail",
        "failures": 1,
        "detail": "Cargo correctly required a clean source or explicit validation-only --allow-dirty."
      },
      {
        "command": "pnpm package:check --allow-dirty",
        "result": "pass",
        "failures": 0,
        "detail": "All six portable manifests/archive file lists verified."
      },
      {
        "command": "cargo publish --workspace --locked --dry-run --allow-dirty",
        "result": "pass",
        "failures": 0,
        "detail": "All six archives compiled from packaged sources; no upload."
      },
      {
        "command": "pnpm spec:check; pnpm check (release preparation)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm test (release preparation)",
        "result": "fail",
        "failures": 1,
        "detail": "SIGTERM fixture log removed by parallel timestamp collision in shared Scratch allocator."
      },
      {
        "command": "cargo test -p jj-acp --test scratch --locked -- --exact concurrent_scratch_directories_survive_peer_teardown (before allocator fix)",
        "result": "fail",
        "failures": 1,
        "detail": "32 callers obtained 31 unique paths."
      },
      {
        "command": "cargo test -p jj-acp --test scratch --test signals --locked",
        "result": "pass",
        "failures": 0,
        "detail": "Collision regression and both unchanged signal contracts pass."
      },
      {
        "command": "node scripts/check-progress.mjs (real Git-only checkout after fix)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm check:workflow; pnpm spec:build",
        "result": "pass",
        "failures": 0,
        "detail": "Actionlint passes; mdBook builds without warnings."
      },
      {
        "command": "retained-history credential-pattern audit",
        "result": "pass",
        "failures": 0,
        "detail": "21 commits and 455 distinct blobs; no known credential/private-key patterns."
      },
      {
        "command": "pnpm spec:check (after parallel scratch fix)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm check (after parallel scratch fix)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm test (after parallel scratch fix)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm build (after parallel scratch fix)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo build -p jj-acp --release --locked --target aarch64-apple-darwin; node scripts/release-archive.mjs aarch64-apple-darwin",
        "result": "pass",
        "failures": 0,
        "detail": "Native release binary and licensed checksum archive verified."
      },
      {
        "command": "cold jj-store developmentAgentEnvironment with candidate ACP executable",
        "result": "pass",
        "failures": 0,
        "detail": "Control v3 and text/context-only preflight; no provider prompt, service restart or consumer edits."
      },
      {
        "command": "README task-file/stdin/catalog/trusted-tools fixture acceptance",
        "result": "pass",
        "failures": 0,
        "detail": "Both task inputs complete with exactly one matching sanitized terminal record."
      }
    ],
    "result": "pass",
    "remaining": [],
    "note": "M12 remains In progress. Package metadata and bundled fixtures change; runtime launch authority and protocol contracts do not. Initial Cargo authentication supplied locally by the user. Full source rerun: 180 Rust tests/doctests and 59 governance tests passed; zero failures. All source lanes passed after repairing the parallel directory collision."
  },
  {
    "id": "E-GITHUB-001",
    "tasks": ["M14"],
    "date": "2026-10-14",
    "kind": "hosted publication",
    "versions": "GitHub source 273b6651; Rust 1.98.1; actionlint 1.7.12; mdBook 0.5.4",
    "formats": "public main; Actions Pages deployment; coordinated crate 0.1.0 pending",
    "files": [
      ".github/workflows/ci.yml",
      "spec/src/spec-governance.md",
      "spec/src/release.md",
      "scripts/release-registry.mjs",
      "scripts/registry-check.mjs",
      "scripts/registry-check.test.mjs",
      "package.json",
      "spec-validator.config.mjs"
    ],
    "tests": [
      "scripts/progress-vcs.test.mjs#explicit_git_range_wins_over_local_jj",
      "scripts/registry-check.test.mjs#missing_version_checks_metadata_before_cdn_download",
      "scripts/registry-check.test.mjs#existing_version_must_match_the_verified_package"
    ],
    "runs": [
      {
        "command": "jj git push --remote origin --bookmark main",
        "result": "pass",
        "failures": 0,
        "detail": "Published signed source history; automatic push signatures updated commit IDs without source changes."
      },
      {
        "command": "GitHub Pages run 38601008185",
        "result": "pass",
        "failures": 0,
        "detail": "Build/deploy succeeded; index, control, release and verification pages return HTTP 200."
      },
      {
        "command": "GitHub CI run 38601008085 canonical setup",
        "result": "fail",
        "failures": 1,
        "detail": "taiki-e/install-action does not support actionlint; native ARM64 Linux/macOS jobs passed."
      },
      {
        "command": "workspace incorporation and cleanup audit",
        "result": "pass",
        "failures": 0,
        "detail": "No substantive changes outside main in either workspace, clean working copies, no extra files to preserve. Forgot and removed both leftover workspaces; default remains."
      },
      {
        "command": "GitHub CI run 38601625239",
        "result": "pass",
        "failures": 0,
        "detail": "Canonical checks and all three additional native targets pass after tool setup correction."
      },
      {
        "command": "node scripts/release-registry.mjs --publish (initial registry existence check)",
        "result": "fail",
        "failures": 1,
        "detail": "No upload attempted: nonexistent CDN archive returned HTTP 403. Existence now checked through version metadata."
      },
      {
        "command": "node --test scripts/registry-check.test.mjs; node scripts/release-registry.mjs --status",
        "result": "pass",
        "failures": 0,
        "detail": "3 retry/existence regressions pass; live metadata confirms all six versions absent."
      },
      {
        "command": "pnpm spec:check; pnpm check; pnpm test; pnpm build (registry preflight correction)",
        "result": "pass",
        "failures": 0,
        "detail": "All source gates including 62 governance tests and 180 Rust tests/doctests pass."
      }
    ],
    "result": "pending",
    "remaining": [
      "Corrected hosted CI, registry bootstrap and native release artifacts pending."
    ],
    "note": "No source qualification was waived. M12 retains independent live-adapter gaps."
  },
  {
    "id": "E-PUBLISH-001",
    "tasks": ["M14"],
    "date": "2026-10-14",
    "kind": "public release acceptance",
    "versions": "All six crates 0.1.0; release tag v0.1.0 at 07a53851d857e16f426ac0645c56904e8850de17; Rust 1.98.1; Node 22 hosted; mdBook 0.5.4",
    "formats": "task/record v1; control v1/v2/v3; GNU/Linux and Darwin ARM64/x86-64 tar.gz; SHA256SUMS",
    "files": [
      "spec/src/milestones.md",
      "spec/src/release.md",
      "README.md",
      ".github/workflows/release.yml"
    ],
    "tests": [
      "scripts/registry-check.test.mjs#existing_version_must_match_the_verified_package",
      "crates/jj-acp-testkit/tests/fixture.rs#the_fixture_completes_a_turn_without_a_model_provider"
    ],
    "runs": [
      {
        "command": "GitHub CI run 38602685961 and release qualification run 38603867724",
        "result": "pass",
        "failures": 0,
        "detail": "Canonical checks and all native targets passed; Git-only checks, metadata, package compile and dry run passed."
      },
      {
        "command": "node scripts/release-registry.mjs --publish (first new-crate batch)",
        "result": "fail",
        "failures": 1,
        "detail": "Five libraries published; crates.io returned 429 for sixth new crate until 2026-10-14 17:12:50 UTC. No version overwritten."
      },
      {
        "command": "node scripts/release-registry.mjs --publish (after specified quota reset)",
        "result": "pass",
        "failures": 0,
        "detail": "Verified existing five package checksums and published only the missing CLI from the same source revision."
      },
      {
        "command": "node scripts/release-registry.mjs; trusted-publisher configuration and public metadata verification",
        "result": "pass",
        "failures": 0,
        "detail": "6/6 archive checksums match; all six enforce stevejuma/jj-acp release.yml with release environment and trusted-publishing-only."
      },
      {
        "command": "cargo install jj-acp --version 0.1.0 --locked --root .local/publication/registry-install",
        "result": "pass",
        "failures": 0,
        "detail": "Fresh registry installation reports jj-acp 0.1.0."
      },
      {
        "command": "published testkit archive standalone cargo +1.98.1 test --locked",
        "result": "pass",
        "failures": 0,
        "detail": "Seven tests and one doctest passed outside the checkout using published protocol/agent dependencies and packaged Python fixtures."
      },
      {
        "command": "GitHub release run 38603867724",
        "result": "pass",
        "failures": 0,
        "detail": "Four native archives tested and published; registry install and package identity gates passed; public v0.1.0 release finalized."
      },
      {
        "command": "downloaded public native archives and SHA256SUMS acceptance",
        "result": "pass",
        "failures": 0,
        "detail": "All four checksums match; downloaded macOS ARM64 binary passes --version and --help."
      },
      {
        "command": "docs.rs API documentation and hosted specification acceptance",
        "result": "pass",
        "failures": 0,
        "detail": "All six Rust API documents and canonical specification pages return HTTP 200."
      }
    ],
    "result": "pass",
    "remaining": [],
    "note": "Publication gate complete. Source remains at tagged immutable version; main may advance for evidence. M12 remains In progress: no new live-adapter qualification claim. Scoped bootstrap-token API revocation returned 403; local Cargo credentials were removed. Server-side token cleanup remains an account operation, separate from the completed publication gate."
  }
]
```

## Recorded gaps

Both entry points are delivered and the persistent one is concurrent:
submissions carry a caller-named scope, one scope has at most one turn in
flight, distinct scopes overlap up to the worker limit, and a bounded queue
refuses admission rather than growing. A crashed agent releases its slot and is
never re-dispatched.

Version 1 does not reuse agents across scoped submissions. Explicit version 2
retains its agent and session across follow-ups and supports capability-gated
loading without prompting. Installed interactive consumer qualification is
recorded separately in M10; the fixture result does not establish
shipping-adapter behavior.

Inbound batching is also absent. Upstream drains all queued events for a scope
into one prompt; here each submission is one turn, so a controlling application
that wants batching must do it before submitting.

`ControlTransport` was replaced rather than wired in. Its old shape modelled the
one-shot path and nothing used it; the loop now depends on a `FrameSource` and a
shared `FrameSink`, and the scheduler is tested in process over an in-memory
pair as well as through the spawned binary.

Descendant containment is proven on macOS ARM64 for both the forced and the
graceful path, but only on Unix. On other platforms cleanup signals the direct
child and reports the reduced guarantee rather than approximating containment,
and no non-Unix platform has been qualified at all.

One residual case is documented rather than closed: a descendant that ignores
`SIGTERM`, whose leader then exits voluntarily inside the grace window, is not
reached, because the leader is reaped by then and the group id is no longer
safely ours. Containment is guaranteed for the fatal path and best-effort for
the cooperative one.

Model, reasoning effort and execution mode are all applied, but only against
what the current session advertises. Two gaps remain in that area. The
`thought_level` configuration path is proven against a committed capture and not
against a live adapter, because the adapter stopped advertising that option
between captures at the same version. And a bare positional bracket such as
`gpt-6.1-sol[medium]` is deliberately not read as an effort, so an adapter that
encodes effort that way reports no effort control rather than being guessed at.

Advertised configuration surfaces are not stable across sessions. A committed
capture is evidence about one session rather than a property of an adapter
version, so no test asserts that a live adapter will offer a given option.

Two real adapters have been qualified: `cursor-agent acp` 2026.09.26-dd393fe and
`@agentclientprotocol/codex-acp` 1.13.1, each driven through completed read-only
turns. `claude-agent-acp`, `opencode` and `goose` have never been launched, and
for `opencode` and `goose` a reported availability means only that an executable
with that name exists, not that its ACP entrypoint works.

Neither qualified run raised a permission request, so the default deny policy
has not been exercised against a real adapter. An adapter that does ask will be
refused and the turn will end cancelled, which is the intended safe default but
is not yet proven outside the fixture peer.

Codex was reached through an adapter already in the local npx cache rather than
a fresh download. The `codex` CLI itself is not an ACP agent and has no `acp`
subcommand, so a machine without a cached or globally installed `codex-acp`
needs one before the compiled-in `codex` runner resolves.

Authentication is deliberately unimplemented, so an adapter that cannot reuse an
existing operator login cannot be driven at all. Cursor worked because
`cursor-agent login` had already been run outside this harness. That is a
consequence of the trust boundary rather than a defect, and it bounds which
adapters are reachable without a controlling application supplying credentials
its own way.

The jj-store controlling-application consumer is qualified in M10 through the
real host evidence E-M10-001 and jj-store E-EXEC-001/002. The qualified profile
is macOS ARM64 LocalTrusted execution with installed Codex 1.13.1 and Cursor
2026.10.01-e373342, ordinary and native isolated workspace edits, sanitized
publication, cold Chromium/restart and managed owner lifetime. Fixture-only
permission/cancellation/deadline evidence remains distinct; these real turns
raised zero permission requests. Signed desktop/WKWebView distribution and other
platforms/adapters remain separate qualification.

E-M12-002 reviews separate prepared context blocks without expanding the real
adapter or lifecycle qualification. M12 stays In progress.
