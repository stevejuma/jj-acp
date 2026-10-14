import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import {
  validateLedger,
  ledgerPath,
  needsEvidence,
} from "./check-progress.mjs";
import { dependencyErrors, excluded } from "./check-boundaries.mjs";

const root = fileURLToPath(new URL("..", import.meta.url));

const entry = {
  id: "E-TEST-001",
  tasks: ["Bootstrap"],
  date: "2026-09-30",
  kind: "governance",
  files: ["test.rs"],
  tests: ["test.rs#proof"],
  runs: [{ command: "cargo test proof", result: "pass", failures: 0 }],
  result: "pass",
  remaining: [],
  versions: "test",
  formats: "none",
};
function fixture(
  rows = "| Bootstrap | Done | - | JAC-GOV-001 | gate | continue | E-TEST-001 |",
  entries = [entry],
  extra = {},
) {
  return {
    source: `${rows}\n\n\`\`\`json\n${JSON.stringify(entries)}\n\`\`\``,
    read: (path) => {
      if (path === "test.rs") return "#[test] fn proof() {}";
      throw Error("missing");
    },
    requirements: new Set(["JAC-GOV-001"]),
    ...extra,
  };
}
test("accepts_verified_task", () =>
  assert.deepEqual(validateLedger(fixture()), []));
test("rejects_missing_evidence", () =>
  assert.match(
    validateLedger(fixture(undefined, [])).join(),
    /missing evidence/,
  ));
test("rejects_completed_parent_with_incomplete_child", () =>
  assert.match(
    validateLedger(
      fixture(
        "| Bootstrap | Done | - | JAC-GOV-001 | gate | next | E-TEST-001 |\n| Bootstrap.1 | Planned | - | JAC-GOV-001 | child gate | next | - |",
      ),
    ).join(),
    /incomplete child/,
  ));
test("rejects_missing_named_test", () =>
  assert.match(
    validateLedger(
      fixture(undefined, [{ ...entry, tests: ["test.rs#absent"] }]),
    ).join(),
    /missing named test/,
  ));
test("accepts_evidence_superseded_by_tracked_removals", () => {
  const retired = {
    ...entry,
    id: "E-TEST-002",
    files: ["removed.rs", "test.rs"],
    tests: ["removed.rs#old_proof", "test.rs#old_proof"],
  };
  const replacement = {
    ...entry,
    removed_files: ["removed.rs"],
    removed_tests: ["test.rs#old_proof"],
  };
  assert.deepEqual(
    validateLedger(
      fixture(
        "| Bootstrap | Done | - | JAC-GOV-001 | gate | continue | E-TEST-001,E-TEST-002 |",
        [retired, replacement],
      ),
    ),
    [],
  );
});
test("rejects_removed_evidence_that_still_exists", () =>
  assert.match(
    validateLedger(
      fixture(undefined, [{ ...entry, removed_files: ["test.rs"] }]),
    ).join(),
    /removed evidence file still exists/,
  ));
test("rejects_invented_test_mention", () =>
  assert.match(
    validateLedger(
      fixture(undefined, undefined, { read: () => "// proof is not a test" }),
    ).join(),
    /missing named test/,
  ));
test("rejects_dependency_cycle", () =>
  assert.match(
    validateLedger(
      fixture(
        "| Bootstrap | Planned | M1 | JAC-GOV-001 | gate | next | - |\n| M1 | Planned | Bootstrap | JAC-GOV-001 | gate | next | - |",
      ),
    ).join(),
    /cycle/,
  ));
test("rejects_invalid_task_id", () =>
  assert.match(
    validateLedger(
      fixture(
        "| Bootstrap-x | Planned | - | JAC-GOV-001 | gate | next | - |",
        [],
      ),
    ).join(),
    /invalid task ID/,
  ));
test("rejects_unchanged_ledger_for_code", () =>
  assert.match(
    validateLedger(
      fixture(undefined, undefined, {
        changed: ["crates/jj-acp-protocol/src/lib.rs"],
        previousEntries: [entry],
      }),
    ).join(),
    /same-change ledger/,
  ));
test("rejects_touch_without_new_evidence", () =>
  assert.match(
    validateLedger(
      fixture(undefined, undefined, {
        changed: ["crates/jj-acp-protocol/src/lib.rs", ledgerPath],
        previousEntries: [entry],
      }),
    ).join(),
    /new or updated evidence/,
  ));
test("automation_exempts_same_change_evidence_but_preserves_ledger_integrity", () => {
  const options = {
    changed: ["Cargo.lock"],
    previousEntries: [entry],
    sameChange: false,
  };
  assert.deepEqual(validateLedger(fixture(undefined, undefined, options)), []);
  assert.match(
    validateLedger(fixture(undefined, [], options)).join(),
    /missing evidence/,
  );
});
test("allows_documentation_without_execution_ledger", () => {
  assert.equal(needsEvidence("docs/licenses/NOTICE.md"), false);
  assert.equal(needsEvidence("README.md"), false);
});
test("requires_evidence_for_protected_surfaces", () => {
  for (const path of [
    "crates/jj-acp-cli/src/main.rs",
    "tests/fixtures/task_agent.py",
    "scripts/check-boundaries.mjs",
    "Cargo.toml",
    "Cargo.lock",
    "rust-toolchain.toml",
    "rustfmt.toml",
    "package.json",
    "AGENTS.md",
    "spec-validator.config.mjs",
    "docs/upstream-lock.md",
    "spec/src/verification.md",
  ])
    assert.equal(needsEvidence(path), true, path);
});
test("rejects_false_passing_results", () =>
  assert.match(
    validateLedger(
      fixture(undefined, [
        { ...entry, runs: [{ command: "test", result: "pass", failures: 1 }] },
      ]),
    ).join(),
    /invalid run/,
  ));
test("rejects_fixture_peer_qualification", () =>
  assert.match(
    validateLedger(
      fixture(
        "| M6 | Done | - | JAC-GOV-001 | real runner | next | E-TEST-001 |",
        [{ ...entry, tasks: ["M6"], kind: "fixture-peer" }],
      ),
    ).join(),
    /real installed-runner/,
  ));
test("rejects_unknown_requirement", () =>
  assert.match(
    validateLedger(
      fixture(undefined, undefined, { requirements: new Set() }),
    ).join(),
    /unknown requirement/,
  ));
test("rejects_layer_dependency_leaks", () => {
  assert.match(
    dependencyErrors([
      {
        name: "jj-acp-protocol",
        dependencies: [{ name: "jj-acp-runner" }],
      },
    ]).join(),
    /invalid layer dependency jj-acp-runner/,
  );
  assert.match(
    dependencyErrors([
      { name: "jj-acp-agent", dependencies: [{ name: "jj-acp" }] },
    ]).join(),
    /invalid layer dependency jj-acp/,
  );
});
test("accepts_owned_layer_dependencies", () =>
  assert.deepEqual(
    dependencyErrors([
      {
        name: "jj-acp-agent",
        dependencies: [{ name: "jj-acp-protocol" }],
      },
      {
        name: "jj-acp-control",
        dependencies: [
          { name: "jj-acp-protocol" },
          { name: "jj-acp-agent" },
          { name: "jj-acp-runner" },
        ],
      },
    ]),
    [],
  ));
test("rejects_excluded_nostr_transport_and_database_dependencies", () => {
  const errors = dependencyErrors([
    {
      name: "jj-acp",
      dependencies: [
        { name: "nostr" },
        { name: "reqwest" },
        { name: "tokio-tungstenite" },
        { name: "rustls" },
        { name: "git-sign-nostr" },
        { name: "rusqlite" },
      ],
    },
  ]);
  assert.equal(errors.length, 6);
  assert.match(errors.join(), /excluded dependency nostr/);
});
test("testkit_is_dev_only", () => {
  assert.deepEqual(
    dependencyErrors([
      {
        name: "jj-acp",
        dependencies: [{ name: "jj-acp-testkit", kind: "dev" }],
      },
    ]),
    [],
  );
  assert.match(
    dependencyErrors([
      {
        name: "jj-acp-runner",
        dependencies: [{ name: "jj-acp-testkit" }],
      },
    ]).join(),
    /testkit is a dev-only dependency/,
  );
});
test("rejects_crate_outside_the_boundary_matrix", () =>
  assert.match(
    dependencyErrors([{ name: "jj-acp-mystery", dependencies: [] }]).join(),
    /absent from the boundary matrix/,
  ));

test("the_protocol_crate_declares_no_process_or_network_facility", () => {
  const manifest = readFileSync(
    join(root, "crates/jj-acp-protocol/Cargo.toml"),
    "utf8",
  );
  // The protocol crate speaks over a caller-supplied byte stream, so it must
  // never gain the ability to spawn a process or open a socket itself.
  for (const facility of ["process", "net"])
    assert.ok(
      !new RegExp(`"${facility}"`).test(manifest),
      `tokio ${facility} feature leaked into the protocol crate`,
    );
  assert.deepEqual(
    dependencyErrors([
      {
        name: "jj-acp-protocol",
        dependencies: [
          { name: "tokio" },
          { name: "serde" },
          { name: "serde_json" },
          { name: "thiserror" },
        ],
      },
    ]),
    [],
  );
});

test("every_declared_crate_dependency_survives_the_exclusion_list", () => {
  const workspace = readFileSync(join(root, "Cargo.toml"), "utf8");
  const declared = [...workspace.matchAll(/^([a-z0-9_-]+) = /gm)].map(
    (match) => match[1],
  );
  assert.ok(
    declared.includes("serde"),
    "expected serde in the workspace manifest",
  );
  for (const name of declared)
    assert.ok(!excluded.test(name), `${name} is an excluded dependency`);
});

test("the_testkit_may_depend_on_the_layers_it_exercises", () => {
  assert.deepEqual(
    dependencyErrors([
      {
        name: "jj-acp-testkit",
        dependencies: [
          { name: "jj-acp-protocol" },
          { name: "jj-acp-agent" },
          { name: "tokio", kind: "dev" },
        ],
      },
    ]),
    [],
  );
  // The testkit sits below nothing: it must not reach the runner, control or CLI
  // layers, or a test helper would become a composition path around them.
  assert.match(
    dependencyErrors([
      {
        name: "jj-acp-testkit",
        dependencies: [{ name: "jj-acp-runner" }],
      },
    ]).join(),
    /invalid layer dependency jj-acp-runner/,
  );
});
