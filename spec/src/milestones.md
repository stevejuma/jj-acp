# Implementation progress

This is the single status ledger. Done requires original-gate evidence; a
fixture-peer result cannot satisfy a real installed-runner gate, and compilation
cannot satisfy a behavioural gate. Every implementation slice adds evidence here
in its enclosing Jujutsu change.

## Tasks and gates

| ID          | Status   | Depends   | Requirements                                                                                                                                                                                                                                                        | Gate                                                                                                          | Next                                       | Evidence   |
| ----------- | -------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- | ------------------------------------------ | ---------- |
| Bootstrap   | Done     | -         | LAC-GOV-001,LAC-GOV-002,LAC-GOV-003,LAC-GOV-005,LAC-GOV-006,LAC-ARCH-001,LAC-ARCH-002,LAC-COMPAT-004                                                                                                                                                                | Canonical specification, ledger, crate skeleton, boundary gate and pinned toolchain green on an empty product | Maintain governance each slice             | E-BOOT-001 |
| Bootstrap.1 | Done     | -         | LAC-GOV-004,LAC-GOV-007,LAC-ARCH-003,LAC-ARCH-004                                                                                                                                                                                                                   | Boundary, counted-line and progress-checker negative tests pass                                               | Maintain gates each slice                  | E-BOOT-002 |
| M1          | Done     | Bootstrap | LAC-COMPAT-001,LAC-COMPAT-002,LAC-COMPAT-003,LAC-COMPAT-007,LAC-SEC-001                                                                                                                                                                                             | Pinned upstream lock, attribution and digest-verified drift detection                                         | Implement M2 protocol crate                | E-M1-001   |
| M2          | Done     | Bootstrap | LAC-PROTO-001,LAC-PROTO-002,LAC-PROTO-003,LAC-PROTO-004,LAC-PROTO-005,LAC-PROTO-006                                                                                                                                                                                 | Bounded ACP framing, version negotiation and session methods under unit test                                  | Implement M3 runner catalog                | E-M2-001   |
| M3          | Planned  | M2        | LAC-AGENT-001,LAC-AGENT-002,LAC-AGENT-003,LAC-AGENT-004,LAC-AGENT-005,LAC-AGENT-006,LAC-COMPAT-005                                                                                                                                                                  | Tiered runner catalog, reserved-key stripping and deterministic fixture peer                                  | Implement M4 task runner                   | -          |
| M4          | Planned  | M3        | LAC-CTRL-002,LAC-CTRL-003,LAC-CTRL-004,LAC-CTRL-005,LAC-CTRL-006,LAC-CTRL-007,LAC-CLI-001,LAC-CLI-002,LAC-CLI-004,LAC-CLI-005,LAC-CLI-006,LAC-RUN-001,LAC-RUN-003,LAC-RUN-004,LAC-ARCH-005,LAC-ARCH-006,LAC-SEC-002,LAC-SEC-003,LAC-SEC-004,LAC-SEC-005,LAC-SEC-007 | One-shot prepared task runner with sanitized terminal record, published exit codes and clean stdout           | Qualify M4.1 lifecycle bounds              | -          |
| M4.1        | Planned  | M4        | LAC-RUN-002,LAC-RUN-005,LAC-RUN-007,LAC-CLI-003,LAC-SEC-006                                                                                                                                                                                                         | Bounded cancellation, process-group cleanup, crash reporting and signal handling                              | Qualify M4.2 real runner                   | -          |
| M4.2        | Planned  | M4.1      | LAC-COMPAT-006,LAC-RUN-006,LAC-PROTO-002                                                                                                                                                                                                                            | Real installed-runner qualification recorded separately from fixture evidence                                 | Implement M5 persistent transport          | -          |
| M5          | Deferred | M4        | LAC-CTRL-001                                                                                                                                                                                                                                                        | Persistent control transport and standing session pool                                                        | Future controlling-application integration | -          |
| M6          | Planned  | M5        | LAC-ARCH-005,LAC-CTRL-001                                                                                                                                                                                                                                           | Controlling-application consumer acceptance in a real host                                                    | Future release                             | -          |

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
  }
]
```

## Recorded gaps

No product behaviour exists yet. The `lapis-acp` binary returns exit code `2`
for every invocation, and every crate is a documented skeleton. The ACP protocol
surface, runner catalog, turn lifecycle, task document and control transport are
specified but unimplemented, and their verification rows are Planned rather than
Implemented.

The upstream pin is recorded and digest-verified, but no region has been ported
yet. Every disposition in `docs/upstream/disposition.json` describes intent; the
18 ported-with-changes regions and the single ported region are claims about
work that M2 through M4 still have to do.

No real installed runner has been exercised. Every gate so far is a governance
or static-analysis gate; nothing here is evidence that a shipping ACP adapter
interoperates with this port.
