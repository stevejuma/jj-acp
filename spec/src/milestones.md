# Implementation progress

This is the single status ledger. Done requires original-gate evidence; a
fixture-peer result cannot satisfy a real installed-runner gate, and compilation
cannot satisfy a behavioural gate. Every implementation slice adds evidence here
in its enclosing Jujutsu change.

## Tasks and gates

| ID          | Status   | Depends   | Requirements                                                                                                                                                                                                                                                                                            | Gate                                                                                                                         | Next                                       | Evidence          |
| ----------- | -------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------ | ----------------- |
| Bootstrap   | Done     | -         | LAC-GOV-001,LAC-GOV-002,LAC-GOV-003,LAC-GOV-005,LAC-GOV-006,LAC-ARCH-001,LAC-ARCH-002,LAC-COMPAT-004                                                                                                                                                                                                    | Canonical specification, ledger, crate skeleton, boundary gate and pinned toolchain green on an empty product                | Maintain governance each slice             | E-BOOT-001        |
| Bootstrap.1 | Done     | -         | LAC-GOV-004,LAC-GOV-007,LAC-ARCH-003,LAC-ARCH-004                                                                                                                                                                                                                                                       | Boundary, counted-line and progress-checker negative tests pass                                                              | Maintain gates each slice                  | E-BOOT-002        |
| M1          | Done     | Bootstrap | LAC-COMPAT-001,LAC-COMPAT-002,LAC-COMPAT-003,LAC-COMPAT-007,LAC-SEC-001                                                                                                                                                                                                                                 | Pinned upstream lock, attribution and digest-verified drift detection                                                        | Implement M2 protocol crate                | E-M1-001          |
| M2          | Done     | Bootstrap | LAC-PROTO-001,LAC-PROTO-002,LAC-PROTO-003,LAC-PROTO-004,LAC-PROTO-005,LAC-PROTO-006                                                                                                                                                                                                                     | Bounded ACP framing, version negotiation and session methods under unit test                                                 | Implement M3 runner catalog                | E-M2-001          |
| M3          | Done     | M2        | LAC-AGENT-001,LAC-AGENT-002,LAC-AGENT-003,LAC-AGENT-004,LAC-AGENT-005,LAC-AGENT-006,LAC-COMPAT-005                                                                                                                                                                                                      | Tiered runner catalog, reserved-key stripping and deterministic fixture peer                                                 | Implement M4 task runner                   | E-M3-001          |
| M4          | Done     | M3        | LAC-CTRL-002,LAC-CTRL-003,LAC-CTRL-004,LAC-CTRL-005,LAC-CTRL-006,LAC-CTRL-007,LAC-CLI-001,LAC-CLI-002,LAC-CLI-003,LAC-CLI-004,LAC-CLI-005,LAC-CLI-006,LAC-RUN-001,LAC-RUN-002,LAC-RUN-003,LAC-RUN-004,LAC-ARCH-005,LAC-ARCH-006,LAC-SEC-002,LAC-SEC-003,LAC-SEC-004,LAC-SEC-005,LAC-SEC-006,LAC-SEC-007 | One-shot prepared task runner with sanitized record, published exit codes, signal cleanup and clean stdout                   | Qualify M5 process-group containment       | E-M4-001          |
| M5          | Done     | M4        | LAC-RUN-005,LAC-RUN-007                                                                                                                                                                                                                                                                                 | Descendant containment on both the forced and the graceful path, crash reporting distinct from a timeout, and no re-dispatch | Qualify M6 against a real installed runner | E-M5-001          |
| M6          | Done     | M5        | LAC-COMPAT-006,LAC-PROTO-002                                                                                                                                                                                                                                                                            | Real installed-runner qualification against cursor-agent acp recorded separately from fixture evidence                       | Implement M7 session configuration         | E-M6-001,E-M6-002 |
| M7          | Planned  | M6        | LAC-RUN-006                                                                                                                                                                                                                                                                                             | Model, reasoning-effort and execution-mode application against a real adapter catalogue                                      | Implement M8 persistent transport          | -                 |
| M8          | Deferred | M4        | LAC-CTRL-001                                                                                                                                                                                                                                                                                            | Persistent control transport and standing session pool                                                                       | Future controlling-application integration | -                 |
| M9          | Planned  | M8        | LAC-ARCH-005,LAC-CTRL-001                                                                                                                                                                                                                                                                               | Controlling-application consumer acceptance in a real host                                                                   | Future release                             | -                 |

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
      "crates/lapis-acp-protocol/src/lib.rs",
      "crates/lapis-acp-agent/src/lib.rs",
      "crates/lapis-acp-runner/src/lib.rs",
      "crates/lapis-acp-control/src/lib.rs",
      "crates/lapis-acp-cli/src/lib.rs",
      "crates/lapis-acp-testkit/src/lib.rs"
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
      "crates/lapis-acp-protocol/Cargo.toml",
      "crates/lapis-acp-protocol/src/lib.rs",
      "crates/lapis-acp-protocol/src/budget.rs",
      "crates/lapis-acp-protocol/src/client.rs",
      "crates/lapis-acp-protocol/src/config_option.rs",
      "crates/lapis-acp-protocol/src/content.rs",
      "crates/lapis-acp-protocol/src/envelope.rs",
      "crates/lapis-acp-protocol/src/error.rs",
      "crates/lapis-acp-protocol/src/frame.rs",
      "crates/lapis-acp-protocol/src/handler.rs",
      "crates/lapis-acp-protocol/src/initialize.rs",
      "crates/lapis-acp-protocol/src/permission.rs",
      "crates/lapis-acp-protocol/src/session.rs",
      "crates/lapis-acp-protocol/src/stop_reason.rs",
      "crates/lapis-acp-protocol/src/update.rs",
      "crates/lapis-acp-protocol/tests/common/mod.rs",
      "crates/lapis-acp-protocol/tests/payload.rs",
      "crates/lapis-acp-protocol/tests/wire.rs",
      "crates/lapis-acp-protocol/tests/inbound.rs",
      "crates/lapis-acp-protocol/tests/bounds.rs",
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
      "crates/lapis-acp-protocol/tests/wire.rs#initialize_negotiates_and_records_the_selected_version",
      "crates/lapis-acp-protocol/tests/wire.rs#initialize_rejects_a_version_the_client_cannot_speak",
      "crates/lapis-acp-protocol/tests/wire.rs#a_prompt_streams_updates_and_then_returns_one_stop_reason",
      "crates/lapis-acp-protocol/tests/wire.rs#request_ids_increase_and_stay_correlated",
      "crates/lapis-acp-protocol/tests/wire.rs#cancellation_is_a_notification_and_carries_no_id",
      "crates/lapis-acp-protocol/tests/inbound.rs#an_unanswered_permission_request_is_cancelled_not_approved",
      "crates/lapis-acp-protocol/tests/inbound.rs#an_explicit_approval_selects_the_option_by_kind",
      "crates/lapis-acp-protocol/tests/inbound.rs#an_unimplemented_agent_request_is_answered_not_ignored",
      "crates/lapis-acp-protocol/tests/bounds.rs#an_oversized_frame_is_refused_before_it_can_be_buffered",
      "crates/lapis-acp-protocol/tests/bounds.rs#the_idle_bound_reports_silence_while_the_absolute_bound_caps_noise",
      "crates/lapis-acp-protocol/tests/bounds.rs#an_agent_that_stops_reading_is_reported_as_a_write_timeout",
      "crates/lapis-acp-protocol/tests/bounds.rs#a_closed_agent_stream_is_reported_not_hung",
      "crates/lapis-acp-protocol/tests/payload.rs#a_model_match_comes_from_the_nested_candidates_not_the_current_value",
      "crates/lapis-acp-protocol/tests/payload.rs#stop_reason_parsing_tolerates_agent_casing",
      "scripts/governance.test.mjs#the_protocol_crate_declares_no_process_or_network_facility",
      "scripts/rust-architecture.test.mjs#allows_a_cfg_test_sibling_module_declaration"
    ],
    "runs": [
      {
        "command": "cargo test -p lapis-acp-protocol --locked",
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
      "crates/lapis-acp-agent/Cargo.toml",
      "crates/lapis-acp-agent/src/lib.rs",
      "crates/lapis-acp-agent/src/availability.rs",
      "crates/lapis-acp-agent/src/catalog.rs",
      "crates/lapis-acp-agent/src/config.rs",
      "crates/lapis-acp-agent/src/defaults.rs",
      "crates/lapis-acp-agent/src/definition.rs",
      "crates/lapis-acp-agent/src/environment.rs",
      "crates/lapis-acp-agent/src/error.rs",
      "crates/lapis-acp-agent/tests/catalog.rs",
      "crates/lapis-acp-testkit/Cargo.toml",
      "crates/lapis-acp-testkit/src/lib.rs",
      "crates/lapis-acp-testkit/src/fixture.rs",
      "crates/lapis-acp-testkit/src/log.rs",
      "crates/lapis-acp-testkit/tests/fixture.rs",
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
      "crates/lapis-acp-agent/tests/catalog.rs#a_runner_is_declared_entirely_as_data",
      "crates/lapis-acp-agent/tests/catalog.rs#identifiers_are_pattern_checked_and_length_bounded",
      "crates/lapis-acp-agent/tests/catalog.rs#the_compiled_in_tier_reserves_its_identifiers",
      "crates/lapis-acp-agent/tests/catalog.rs#no_tier_may_claim_an_identifier_twice",
      "crates/lapis-acp-agent/tests/catalog.rs#unknown_and_disabled_runners_fail_without_a_default_fallback",
      "crates/lapis-acp-agent/tests/catalog.rs#reserved_environment_keys_are_stripped_from_every_layer",
      "crates/lapis-acp-agent/tests/catalog.rs#environment_layers_apply_inherited_then_floor_then_overrides",
      "crates/lapis-acp-agent/tests/catalog.rs#harness_variables_are_produced_only_by_the_harness",
      "crates/lapis-acp-agent/tests/catalog.rs#a_catalog_document_declares_runners_without_executing_anything",
      "crates/lapis-acp-agent/tests/catalog.rs#availability_resolves_the_first_candidate_that_exists",
      "crates/lapis-acp-testkit/tests/fixture.rs#the_fixture_completes_a_turn_without_a_model_provider",
      "crates/lapis-acp-testkit/tests/fixture.rs#a_permission_request_is_answered_and_the_turn_completes",
      "crates/lapis-acp-testkit/tests/fixture.rs#an_agent_error_arrives_typed_with_its_marker_intact",
      "crates/lapis-acp-testkit/tests/fixture.rs#a_silent_agent_is_bounded_by_the_idle_window",
      "crates/lapis-acp-testkit/tests/fixture.rs#a_version_one_agent_is_accepted_and_recorded",
      "crates/lapis-acp-testkit/tests/fixture.rs#killing_the_agent_alone_leaves_its_descendant_running",
      "scripts/governance.test.mjs#the_testkit_may_depend_on_the_layers_it_exercises",
      "scripts/rust-architecture.test.mjs#allows_the_testkit_crate_to_name_itself"
    ],
    "runs": [
      {
        "command": "cargo test -p lapis-acp-agent --locked",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo test -p lapis-acp-testkit --locked",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo test -p lapis-acp-protocol --locked",
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
      "crates/lapis-acp-protocol/src/client.rs",
      "crates/lapis-acp-protocol/src/frame.rs",
      "crates/lapis-acp-protocol/tests/bounds.rs",
      "crates/lapis-acp-agent/src/config.rs",
      "crates/lapis-acp-agent/src/environment.rs",
      "crates/lapis-acp-agent/tests/catalog.rs",
      "crates/lapis-acp-runner/Cargo.toml",
      "crates/lapis-acp-runner/src/lib.rs",
      "crates/lapis-acp-runner/src/launch.rs",
      "crates/lapis-acp-runner/src/outcome.rs",
      "crates/lapis-acp-runner/src/process.rs",
      "crates/lapis-acp-runner/src/turn.rs",
      "crates/lapis-acp-control/Cargo.toml",
      "crates/lapis-acp-control/src/lib.rs",
      "crates/lapis-acp-control/src/input.rs",
      "crates/lapis-acp-control/src/invocation.rs",
      "crates/lapis-acp-control/src/record.rs",
      "crates/lapis-acp-control/src/task.rs",
      "crates/lapis-acp-control/src/transport.rs",
      "crates/lapis-acp-cli/Cargo.toml",
      "crates/lapis-acp-cli/src/lib.rs",
      "crates/lapis-acp-cli/src/main.rs",
      "crates/lapis-acp-cli/src/agents.rs",
      "crates/lapis-acp-cli/src/args.rs",
      "crates/lapis-acp-cli/src/run_task.rs",
      "crates/lapis-acp-cli/src/signals.rs",
      "crates/lapis-acp-cli/tests/common/mod.rs",
      "crates/lapis-acp-cli/tests/args.rs",
      "crates/lapis-acp-cli/tests/run_task.rs",
      "crates/lapis-acp-cli/tests/signals.rs",
      "crates/lapis-acp-cli/tests/stdio_contract.rs",
      "crates/lapis-acp-testkit/tests/fixture.rs",
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
      "crates/lapis-acp-cli/tests/run_task.rs#a_prepared_task_completes_with_one_terminal_record",
      "crates/lapis-acp-cli/tests/run_task.rs#file_and_stdin_input_agree",
      "crates/lapis-acp-cli/tests/run_task.rs#a_refusal_is_a_normal_completion_with_its_own_stop_reason",
      "crates/lapis-acp-cli/tests/run_task.rs#an_agent_error_never_echoes_the_agent_message",
      "crates/lapis-acp-cli/tests/run_task.rs#the_prompt_and_credentials_never_reach_the_terminal_record",
      "crates/lapis-acp-cli/tests/run_task.rs#the_fixture_receives_the_harness_reserved_variables",
      "crates/lapis-acp-cli/tests/run_task.rs#invalid_documents_are_refused_before_any_agent_starts",
      "crates/lapis-acp-cli/tests/run_task.rs#duplicate_and_trailing_documents_are_refused",
      "crates/lapis-acp-cli/tests/run_task.rs#an_oversized_document_is_refused_without_being_buffered",
      "crates/lapis-acp-cli/tests/run_task.rs#a_task_addressed_to_another_runner_is_refused",
      "crates/lapis-acp-cli/tests/run_task.rs#an_unknown_runner_is_refused_without_pinning_one",
      "crates/lapis-acp-cli/tests/run_task.rs#an_unresolvable_runner_reports_a_spawn_failure",
      "crates/lapis-acp-cli/tests/run_task.rs#a_deadline_produces_timed_out_and_exit_124",
      "crates/lapis-acp-cli/tests/run_task.rs#an_idle_window_that_never_sees_a_frame_times_out_separately",
      "crates/lapis-acp-cli/tests/signals.rs#sigterm_produces_bounded_cleanup_and_exit_143",
      "crates/lapis-acp-cli/tests/signals.rs#sigint_produces_bounded_cleanup_and_exit_130",
      "crates/lapis-acp-cli/tests/stdio_contract.rs#an_invalid_command_line_leaves_stdout_empty",
      "crates/lapis-acp-cli/tests/stdio_contract.rs#a_failed_invocation_still_emits_exactly_one_record",
      "crates/lapis-acp-cli/tests/stdio_contract.rs#the_agents_command_emits_one_json_object_per_line",
      "crates/lapis-acp-cli/tests/args.rs#a_flag_overrides_the_matching_environment_variable",
      "crates/lapis-acp-cli/tests/args.rs#every_option_is_settable_by_environment",
      "crates/lapis-acp-protocol/tests/bounds.rs#a_read_cancelled_mid_frame_resumes_instead_of_losing_bytes",
      "crates/lapis-acp-testkit/tests/fixture.rs#killing_the_agent_alone_leaves_its_descendant_running"
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
      "crates/lapis-acp-runner/src/process.rs",
      "crates/lapis-acp-cli/tests/cleanup.rs",
      "crates/lapis-acp-testkit/src/fixture.rs",
      "crates/lapis-acp-testkit/src/log.rs",
      "tests/fixtures/fixture_agent.py",
      "spec/src/runner.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md"
    ],
    "tests": [
      "crates/lapis-acp-cli/tests/cleanup.rs#a_forced_termination_takes_the_agents_descendant_with_it",
      "crates/lapis-acp-cli/tests/cleanup.rs#a_completed_turn_still_does_not_leave_a_descendant_behind",
      "crates/lapis-acp-cli/tests/cleanup.rs#an_agent_that_crashes_mid_turn_is_reported_and_never_re_dispatched",
      "crates/lapis-acp-cli/tests/cleanup.rs#an_agent_that_crashes_during_startup_never_opens_a_session",
      "crates/lapis-acp-cli/tests/cleanup.rs#a_crash_is_distinguished_from_a_bound_that_elapsed",
      "crates/lapis-acp-cli/tests/cleanup.rs#the_helpers_resolve_the_real_fixture_paths"
    ],
    "runs": [
      {
        "command": "cargo test -p lapis-acp-cli --test cleanup --locked",
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
      "crates/lapis-acp-agent/src/defaults.rs",
      "crates/lapis-acp-agent/tests/catalog.rs",
      "crates/lapis-acp-cli/src/args.rs",
      "crates/lapis-acp-cli/src/lib.rs",
      "crates/lapis-acp-cli/tests/args.rs",
      "crates/lapis-acp-protocol/src/initialize.rs",
      "crates/lapis-acp-protocol/tests/payload.rs",
      "crates/lapis-acp-protocol/tests/real_adapter.rs",
      "crates/lapis-acp-protocol/tests/fixtures/cursor-acp-session.json",
      "spec/src/agents.md",
      "spec/src/protocol.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md"
    ],
    "tests": [
      "crates/lapis-acp-protocol/tests/real_adapter.rs#a_real_adapter_selects_protocol_version_one_and_is_accepted",
      "crates/lapis-acp-protocol/tests/real_adapter.rs#a_real_adapter_names_its_option_id_rather_than_config_id",
      "crates/lapis-acp-protocol/tests/real_adapter.rs#model_candidates_are_parameterized_and_matched_exactly",
      "crates/lapis-acp-protocol/tests/real_adapter.rs#the_stable_surface_wins_when_both_surfaces_offer_the_model",
      "crates/lapis-acp-protocol/tests/real_adapter.rs#a_model_only_on_the_unstable_surface_falls_back_to_set_model",
      "crates/lapis-acp-protocol/tests/real_adapter.rs#this_adapter_exposes_no_reasoning_effort_option_of_its_own",
      "crates/lapis-acp-protocol/tests/payload.rs#an_explicit_false_capability_is_not_advertised",
      "crates/lapis-acp-agent/tests/catalog.rs#cursor_is_declared_with_the_transport_it_actually_advertises",
      "crates/lapis-acp-cli/tests/args.rs#the_agents_command_parses_the_same_launch_options_as_run",
      "crates/lapis-acp-cli/tests/args.rs#arguments_no_subcommand_would_read_are_refused",
      "crates/lapis-acp-cli/tests/args.rs#agents_refuses_a_task_flag"
    ],
    "runs": [
      {
        "command": "cursor-agent acp \u2014 initialize and session/new handshake probe, no prompt",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "lapis-acp run --task <scratch>/task.json --runner cursor --idle-timeout 60 --max-turn-duration 180 (reply READY, no tools)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "lapis-acp run --task <scratch>/task.json --runner cursor (read notes.md, report first line, no modification)",
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
      "crates/lapis-acp-protocol/tests/real_adapter.rs",
      "crates/lapis-acp-protocol/tests/fixtures/codex-acp-session.json",
      "spec/src/agents.md",
      "spec/src/protocol.md",
      "spec/src/compatibility.md"
    ],
    "tests": [
      "crates/lapis-acp-protocol/tests/real_adapter.rs#a_second_adapter_also_selects_version_one_but_does_report_agent_info",
      "crates/lapis-acp-protocol/tests/real_adapter.rs#explicit_false_capabilities_are_not_advertised_by_a_real_adapter",
      "crates/lapis-acp-protocol/tests/real_adapter.rs#adapters_disagree_about_whether_model_values_are_parameterized",
      "crates/lapis-acp-protocol/tests/real_adapter.rs#a_real_adapter_does_expose_a_reasoning_effort_option",
      "crates/lapis-acp-protocol/tests/real_adapter.rs#adapters_emit_notifications_that_are_not_part_of_the_exchange",
      "crates/lapis-acp-protocol/tests/real_adapter.rs#both_adapters_expose_modes_this_crate_does_not_yet_model"
    ],
    "runs": [
      {
        "command": "codex-acp 1.13.1 handshake probe (initialize + session/new), no prompt",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "PATH=<npx cache bin>:$PATH lapis-acp run --task <scratch>/task.json --runner codex --idle-timeout 150 --max-turn-duration 240 (read notes.md, report first line, no modification)",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo test -p lapis-acp-protocol --test real_adapter --locked",
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
    "note": "One real read-only turn completed with status completed, stopReason end_turn and exit 0; notes.md was unmodified and no file was created. The codex CLI itself has no acp subcommand, so the ACP path is the adapter that acpx launches through npx on demand and that never appears on PATH. This adapter does expose a thought_level option with the adapter-defined identifier reasoning_effort, which Cursor does not, so LAC-RUN-006 stays Planned and the discovery helper is proven against real data rather than implemented end to end. No permission request was raised, so the default deny policy was not exercised against this adapter."
  }
]
```

## Recorded gaps

The one-shot task runner is delivered and exercised end to end through the real
binary: a prepared document in, one sanitized terminal record out, and the
published exit code. Concurrency and durability are not delivered. There is no
standing pool, no persistent control transport, no inbound batching and no
session rotation, so one process runs exactly one turn and then exits. That is
M7 and it stays deferred.

Descendant containment is proven on macOS ARM64 for both the forced and the
graceful path, but only on Unix. On other platforms cleanup signals the direct
child and reports the reduced guarantee rather than approximating containment,
and no non-Unix platform has been qualified at all.

One residual case is documented rather than closed: a descendant that ignores
`SIGTERM`, whose leader then exits voluntarily inside the grace window, is not
reached, because the leader is reaped by then and the group id is no longer
safely ours. Containment is guaranteed for the fatal path and best-effort for
the cooperative one.

Model selection is implemented but reasoning effort and execution mode are not
applied at all, so LAC-RUN-006 stays Planned and now owns M7. The captured
Cursor payload shows the generic shape does not transfer: effort travels inside
parameterized model values and mode is its own option, while upstream discovered
a separate `thought_level` option for a different adapter. Neither is a superset
of the other.

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

Nothing has consumed this harness. The `jj-store` agent-runner integration is M9
and depends on M8, so no controlling application has yet driven a task document
over a transport other than the one-shot stdio path. Wiring it in is not a
packaging change either: `jj-store` records controller dispatch as deferred in
its own canonical specification, so that slice needs a spec change in that
repository under its own governance.
