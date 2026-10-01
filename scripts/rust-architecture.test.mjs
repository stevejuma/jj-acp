import test from "node:test";
import assert from "node:assert/strict";
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import {
  architectureErrors,
  countedLines,
  fileKind,
  limits,
} from "./rust-architecture.mjs";

test("counted_lines_exclude_blanks_and_line_comments", () => {
  assert.equal(countedLines("//! doc\n\nfn main() {}\n   // note\n"), 1);
});

test("counted_lines_exclude_block_comments", () => {
  assert.equal(
    countedLines("/* one\n   two */\nfn a() {}\n/** doc */\nfn b() {}\n"),
    2,
  );
});

test("counted_lines_keep_trailing_comment_bearing_code", () => {
  assert.equal(countedLines("fn a() {} // why\n"), 1);
});

test("file_kind_classifies_tests_scripts_and_production", () => {
  assert.equal(fileKind("crates/jj-acp-cli/tests/run_task.rs"), "test");
  assert.equal(fileKind("crates/jj-acp-protocol/src/frame/tests.rs"), "test");
  assert.equal(fileKind("tests/fixtures/task_agent.py"), "test");
  assert.equal(fileKind("scripts/governance.test.mjs"), "test");
  assert.equal(fileKind("scripts/check-progress.mjs"), "script");
  assert.equal(fileKind("crates/jj-acp-protocol/src/frame.rs"), "production");
});

/** Build a throwaway repository root holding the supplied relative files. */
function root(files) {
  const dir = mkdtempSync(join(tmpdir(), "jj-acp-arch-"));
  for (const leaf of ["crates", "scripts", "tests"])
    mkdirSync(join(dir, leaf), { recursive: true });
  for (const [path, source] of Object.entries(files)) {
    const absolute = join(dir, path);
    mkdirSync(join(absolute, ".."), { recursive: true });
    writeFileSync(absolute, source);
  }
  return dir;
}

test("accepts_sources_within_budget", () => {
  const dir = root({ "crates/jj-acp-protocol/src/lib.rs": "fn a() {}\n" });
  assert.deepEqual(architectureErrors(dir, { files: {} }), []);
  rmSync(dir, { recursive: true, force: true });
});

test("rejects_production_source_over_budget", () => {
  const body = Array.from(
    { length: limits.production + 1 },
    (_, i) => `fn f${i}() {}`,
  ).join("\n");
  const dir = root({ "crates/jj-acp-runner/src/turn.rs": `${body}\n` });
  assert.match(
    architectureErrors(dir, { files: {} }).join(),
    new RegExp(`exceeds ${limits.production}`),
  );
  rmSync(dir, { recursive: true, force: true });
});

test("test_files_use_the_larger_budget", () => {
  const body = Array.from(
    { length: limits.production + 1 },
    (_, i) => `fn f${i}() {}`,
  ).join("\n");
  const dir = root({ "crates/jj-acp-cli/tests/run_task.rs": `${body}\n` });
  assert.deepEqual(architectureErrors(dir, { files: {} }), []);
  rmSync(dir, { recursive: true, force: true });
});

test("baseline_grants_an_explicit_exception", () => {
  const body = Array.from(
    { length: limits.production + 1 },
    (_, i) => `fn f${i}() {}`,
  ).join("\n");
  const path = "crates/jj-acp-runner/src/turn.rs";
  const dir = root({ [path]: `${body}\n` });
  assert.deepEqual(
    architectureErrors(dir, { files: { [path]: limits.production + 1 } }),
    [],
  );
  rmSync(dir, { recursive: true, force: true });
});

test("rejects_stale_baseline_entry", () => {
  const dir = root({ "crates/jj-acp-protocol/src/lib.rs": "fn a() {}\n" });
  assert.match(
    architectureErrors(dir, { files: { "crates/gone.rs": 300 } }).join(),
    /stale architecture baseline/,
  );
  rmSync(dir, { recursive: true, force: true });
});

test("rejects_production_import_of_the_testkit", () => {
  const dir = root({
    "crates/jj-acp-runner/src/turn.rs": "use jj_acp_testkit::peer;\n",
  });
  assert.match(
    architectureErrors(dir, { files: {} }).join(),
    /production source imports the testkit crate/,
  );
  rmSync(dir, { recursive: true, force: true });
});

test("rejects_inline_tests_in_a_production_module", () => {
  const dir = root({
    "crates/jj-acp-protocol/src/frame.rs":
      "fn a() {}\n\n#[cfg(test)]\nmod inline {\n    #[test]\n    fn proof() {}\n}\n",
  });
  assert.match(
    architectureErrors(dir, { files: {} }).join(),
    /defines tests inline/,
  );
  rmSync(dir, { recursive: true, force: true });
});

test("allows_a_cfg_test_sibling_module_declaration", () => {
  const dir = root({
    "crates/jj-acp-protocol/src/frame.rs":
      "fn a() {}\n\n#[cfg(test)]\nmod tests;\n",
    "crates/jj-acp-protocol/src/frame/tests.rs": "#[test]\nfn proof() {}\n",
  });
  assert.deepEqual(architectureErrors(dir, { files: {} }), []);
  rmSync(dir, { recursive: true, force: true });
});

test("allows_testkit_use_from_a_test_module", () => {
  const dir = root({
    "crates/jj-acp-cli/tests/run_task.rs": "use jj_acp_testkit::peer;\n",
  });
  assert.deepEqual(architectureErrors(dir, { files: {} }), []);
  rmSync(dir, { recursive: true, force: true });
});

test("allows_the_testkit_crate_to_name_itself", () => {
  // The testkit's own documentation examples legitimately refer to it by crate
  // name; the rule exists to keep production crates away from test helpers, not
  // to stop the helper describing itself.
  const dir = root({
    "crates/jj-acp-testkit/src/lib.rs": "//! use jj_acp_testkit::Fixture;\n",
  });
  assert.deepEqual(architectureErrors(dir, { files: {} }), []);
  rmSync(dir, { recursive: true, force: true });
});

test("tolerates_absent_source_roots", () => {
  const dir = root({});
  for (const leaf of ["crates", "tests"])
    rmSync(join(dir, leaf), { recursive: true, force: true });
  assert.deepEqual(architectureErrors(dir, { files: {} }), []);
  rmSync(dir, { recursive: true, force: true });
});
