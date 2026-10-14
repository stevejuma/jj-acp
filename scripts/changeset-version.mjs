import { copyFileSync, writeFileSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { join, resolve } from "node:path";
import { crates, version } from "./release-version.mjs";
import {
  checkRelease,
  json,
  pendingChangesets,
  read,
} from "./release-state.mjs";
import { recordPreparation } from "./release-evidence.mjs";

export function synchronizeVersion(root, selected) {
  const previous = version(root);
  const newer = selected.split(".").map(BigInt);
  const older = previous.split(".").map(BigInt);
  const changed = newer.findIndex((value, index) => value !== older[index]);
  if (
    !/^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$/.test(selected) ||
    changed < 0 ||
    newer[changed] < older[changed]
  )
    throw Error("version preparation requires a newer stable version");
  let cargo = read(root, "Cargo.toml").replace(
    /^version = "[^"]+"$/m,
    `version = "${selected}"`,
  );
  for (const name of crates.filter((item) => item !== "jj-acp")) {
    const pattern = new RegExp(
      `(^${name} = \\{[^\\n]* version = ")[^"]+("[^\\n]*\\}$)`,
      "m",
    );
    if (!pattern.test(cargo))
      throw Error(`${name}: internal version declaration missing`);
    cargo = cargo.replace(pattern, `$1${selected}$2`);
  }
  writeFileSync(join(root, "Cargo.toml"), cargo);
  const readme = read(root, "README.md").replace(
    `cargo install jj-acp --version ${previous} --locked`,
    `cargo install jj-acp --version ${selected} --locked`,
  );
  writeFileSync(join(root, "README.md"), readme);
  copyFileSync(
    join(root, "README.md"),
    join(root, "crates/jj-acp-cli/README.md"),
  );
}

export async function prepareVersion(
  root = ".",
  execute = (command, args) =>
    execFileSync(command, args, { cwd: root, stdio: "inherit" }),
) {
  const original = checkRelease(root);
  const notes = await pendingChangesets(root);
  if (!notes.some((note) => note.releases.length))
    return { changed: false, version: original };
  const runs = [];
  const run = (command, args) => {
    execute(command, args);
    runs.push({
      command: [command, ...args].join(" "),
      result: "pass",
      failures: 0,
    });
  };
  // Fail before consuming notes if the cold-checkout dependency bootstrap is missing.
  try {
    run("cargo", ["fetch", "--locked", "--offline"]);
  } catch (cause) {
    throw Error("Run cargo fetch --locked before offline version preparation", {
      cause,
    });
  }
  run("pnpm", ["exec", "changeset", "version"]);
  const selected = json(root, "package.json").version;
  synchronizeVersion(root, selected);
  run("cargo", ["update", "--workspace", "--offline"]);
  run("pnpm", ["install", "--lockfile-only", "--offline", "--ignore-pnpmfile"]);
  run("pnpm", ["release:check"]);
  recordPreparation(
    root,
    selected,
    notes.map((note) => `.changeset/${note.id}.md`),
    runs,
  );
  run("pnpm", [
    "exec",
    "prettier",
    "--write",
    "package.json",
    "README.md",
    "CHANGELOG.md",
    "crates/jj-acp-cli/README.md",
    "spec/src/architecture.md",
    "spec/src/cli.md",
    "spec/src/release.md",
    "spec/src/milestones.md",
  ]);
  return { changed: true, version: selected };
}

if (
  process.argv[1] &&
  resolve(process.argv[1]) === resolve(new URL(import.meta.url).pathname)
)
  console.log(await prepareVersion());
