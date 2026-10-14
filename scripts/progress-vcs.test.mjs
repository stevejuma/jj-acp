import test from "node:test";
import assert from "node:assert/strict";
import { snapshot } from "./progress-vcs.mjs";
import { needsEvidence } from "./check-progress.mjs";

test("explicit_git_range_wins_over_local_jj", () => {
  const calls = [];
  const run = (command, args) => {
    calls.push([command, args]);
    return args[0] === "diff"
      ? ".github/workflows/ci.yml\0README.md\0"
      : "spec/src/architecture.md\n";
  };
  const s = snapshot(
    "/example",
    { JJ_ACP_CHECK_BASE: "base", JJ_ACP_CHECK_HEAD: "head" },
    run,
    () => true,
  );
  assert.deepEqual(s.changed, [".github/workflows/ci.yml", "README.md"]);
  s.previous("spec/src/milestones.md");
  assert.ok(calls.every(([command]) => command === "git"));
  assert.deepEqual(calls[0][1], [
    "diff",
    "--name-only",
    "-z",
    "base",
    "head",
    "--",
  ]);
  assert.ok(needsEvidence(s.changed[0]));
});
test("local_jj_preserves_selected_revision", () => {
  const calls = [];
  const run = (command, args) => {
    calls.push([command, args]);
    return "spec/src/milestones.md";
  };
  snapshot("/example", { JJ_ACP_CHECK_REVISION: "change" }, run, () => true);
  assert.ok(calls.every(([command]) => command === "jj"));
  assert.ok(calls[0][1].includes("change-"));
});
test("incomplete_range_fails_closed", () => {
  assert.throws(
    () => snapshot("/example", { JJ_ACP_CHECK_BASE: "base" }),
    /both base and head/,
  );
});
test("initial_git_commit_uses_empty_tree", () => {
  const calls = [];
  snapshot(
    "/example",
    {},
    (_command, args, input) => {
      calls.push([args, input]);
      if (args[0] === "rev-parse") throw Error("no parent");
      if (args[0] === "hash-object") return "empty-tree";
      return "";
    },
    () => false,
  );
  assert.ok(
    calls.some(([args, input]) => args[0] === "hash-object" && input === ""),
  );
  assert.ok(
    calls.some(
      ([args]) => args.includes("empty-tree") && args.includes("HEAD"),
    ),
  );
});
