import test from "node:test";
import assert from "node:assert/strict";
import { crates, validateTag } from "./release-version.mjs";
test("release_refuses_mismatched_or_nonversion_tags", () => {
  for (const tag of ["v0.1.1", "main", "v0.1.0-rc.1"])
    assert.throws(() => validateTag(tag, "0.1.0"), /does not match/);
  assert.doesNotThrow(() => validateTag("v0.1.0", "0.1.0"));
});
test("release_orders_testkit_before_dependent_test_packages", () => {
  for (const name of ["jj-acp-runner", "jj-acp-control", "jj-acp"])
    assert.ok(crates.indexOf("jj-acp-testkit") < crates.indexOf(name));
});
