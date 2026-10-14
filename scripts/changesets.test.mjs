import test from "node:test";
import assert from "node:assert/strict";
import {
  cpSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  rmSync,
  symlinkSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { execFileSync } from "node:child_process";
import { prepareVersion, synchronizeVersion } from "./changeset-version.mjs";
import {
  checkRelease,
  coordinator,
  json,
  pendingChangesets,
  releaseState,
} from "./release-state.mjs";
import { parseLedger } from "./check-progress.mjs";
import { validateReleaseSource } from "./release-version.mjs";

const source = resolve(new URL("..", import.meta.url).pathname);
function fixture(t) {
  const root = mkdtempSync(join(tmpdir(), "jj-acp-changesets-"));
  t.after(() => rmSync(root, { recursive: true, force: true }));
  for (const path of [
    "package.json",
    "pnpm-lock.yaml",
    "Cargo.toml",
    "Cargo.lock",
    "rust-toolchain.toml",
    "README.md",
    "CHANGELOG.md",
    "scripts",
    "spec/src",
    "crates",
  ])
    cpSync(join(source, path), join(root, path), { recursive: true });
  mkdirSync(join(root, ".changeset"));
  for (const path of ["config.json", "README.md"])
    cpSync(join(source, ".changeset", path), join(root, ".changeset", path));
  symlinkSync(join(source, "node_modules"), join(root, "node_modules"), "dir");
  return root;
}
function edit(root, path, transform) {
  const target = join(root, path);
  writeFileSync(target, transform(readFileSync(target, "utf8")));
}

test("released_history_is_not_a_pending_changeset", async (t) => {
  const root = fixture(t);
  const state = await releaseState(root);
  assert.equal(state.pending, 0);
  assert.doesNotThrow(() => validateReleaseSource(root));
  const history = readFileSync(join(source, "docs/releases/0.1.0.md"), "utf8");
  assert.match(history, /07a53851d857e16f426ac0645c56904e8850de17/);
  for (const name of state.crates) assert.ok(history.includes(`\`${name}\``));
});

test("version_preparation_without_notes_preserves_released_metadata", async (t) => {
  const root = fixture(t);
  const before = [
    "Cargo.toml",
    "Cargo.lock",
    "package.json",
    "CHANGELOG.md",
  ].map((path) => readFileSync(join(root, path), "utf8"));
  let calls = 0;
  const result = await prepareVersion(root, () => calls++);
  assert.equal(result.changed, false);
  assert.equal(calls, 0);
  assert.deepEqual(
    ["Cargo.toml", "Cargo.lock", "package.json", "CHANGELOG.md"].map((path) =>
      readFileSync(join(root, path), "utf8"),
    ),
    before,
  );
});

test("missing_cargo_cache_preserves_pending_notes_and_version_metadata", async (t) => {
  const root = fixture(t);
  const cache = mkdtempSync(join(tmpdir(), "jj-acp-empty-cargo-"));
  t.after(() => rmSync(cache, { recursive: true, force: true }));
  writeFileSync(
    join(root, ".changeset/cold-cache.md"),
    `---\n"${coordinator}": patch\n---\n\nCold dependency bootstrap.\n`,
  );
  const before = [
    "Cargo.toml",
    "Cargo.lock",
    "package.json",
    "CHANGELOG.md",
  ].map((path) => readFileSync(join(root, path), "utf8"));
  await assert.rejects(
    prepareVersion(root, (command, args) =>
      execFileSync(command, args, {
        cwd: root,
        stdio: "pipe",
        env: { ...process.env, CARGO_HOME: cache },
      }),
    ),
    /cargo fetch --locked before offline version preparation/,
  );
  assert.equal((await pendingChangesets(root)).length, 1);
  assert.deepEqual(
    ["Cargo.toml", "Cargo.lock", "package.json", "CHANGELOG.md"].map((path) =>
      readFileSync(join(root, path), "utf8"),
    ),
    before,
  );
});

test("changesets_prepare_coordinated_cargo_version_and_consume_notes", async (t) => {
  const root = fixture(t);
  const original = checkRelease(root);
  const [major, minor, patch] = original.split(".").map(Number);
  const next = `${major}.${minor}.${patch + 1}`;
  writeFileSync(
    join(root, ".changeset/fixture-release.md"),
    `---\n"${coordinator}": patch\n---\n\nQualify coordinated Cargo version preparation.\n`,
  );
  assert.throws(() => validateReleaseSource(root), /pending changesets/);
  const result = await prepareVersion(root, (command, args) =>
    execFileSync(command, args, { cwd: root, stdio: "pipe" }),
  );
  assert.deepEqual(result, { changed: true, version: next });
  assert.equal(checkRelease(root), next);
  assert.equal((await pendingChangesets(root)).length, 0);
  assert.doesNotThrow(() => validateReleaseSource(root));
  assert.equal(json(root, "package.json").private, true);
  assert.match(
    readFileSync(join(root, "CHANGELOG.md"), "utf8"),
    /Qualify coordinated Cargo version preparation/,
  );
  const evidence = parseLedger(
    readFileSync(join(root, "spec/src/milestones.md"), "utf8"),
  ).entries.find(
    (entry) => entry.id === `E-VERSION-${next.replaceAll(".", "-")}`,
  );
  assert.deepEqual(evidence.removed_files, [".changeset/fixture-release.md"]);
  for (const chapter of ["architecture", "cli", "release"])
    assert.ok(
      readFileSync(join(root, `spec/src/${chapter}.md`), "utf8").includes(
        `version ${next}`,
      ),
    );
  assert.equal((await prepareVersion(root)).changed, false);
});

test("release_metadata_refuses_stale_cargo_and_public_coordinator", (t) => {
  const root = fixture(t);
  edit(root, "Cargo.toml", (text) =>
    text.replace(
      /(jj-acp-protocol = .*version = ")[^"]+/,
      (_, prefix) => `${prefix}0.0.0`,
    ),
  );
  assert.throws(() => checkRelease(root), /internal registry version/);
  cpSync(join(source, "Cargo.toml"), join(root, "Cargo.toml"));
  edit(root, "package.json", (text) =>
    text.replace('"private": true', '"private": false'),
  );
  assert.throws(() => checkRelease(root), /private coordinator/);
});

test("changesets_refuse_unknown_release_targets", async (t) => {
  const root = fixture(t);
  writeFileSync(
    join(root, ".changeset/wrong-target.md"),
    '---\n"jj-acp-protocol": patch\n---\n\nWrong coordination target.\n',
  );
  await assert.rejects(prepareVersion(root), /coordinated Cargo release/);
});

test("version_preparation_refuses_replacing_a_stable_version", (t) => {
  const root = fixture(t);
  for (const selected of [checkRelease(root), "0.0.0", "0.1.1-rc.1"])
    assert.throws(() => synchronizeVersion(root, selected));
});
