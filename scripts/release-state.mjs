import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import readModule from "@changesets/read";
import toml from "@iarna/toml";
import { crates, version } from "./release-version.mjs";

export const coordinator = "@lapismd/jj-acp";
const readChangesets = readModule.default ?? readModule;
export const read = (root, path) => readFileSync(join(root, path), "utf8");
export const cratePath = (name) =>
  `crates/jj-acp-${name === "jj-acp" ? "cli" : name.slice(7)}`;
export const json = (root, path) => JSON.parse(read(root, path));
export const manifest = (root, path) => toml.parse(read(root, path));

export function checkRelease(root = ".") {
  const selected = version(root);
  const pkg = json(root, "package.json");
  if (
    pkg.name !== coordinator ||
    pkg.private !== true ||
    pkg.version !== selected
  )
    throw Error("private coordinator and Cargo versions must match");
  const config = json(root, ".changeset/config.json");
  if (
    config.privatePackages?.version !== true ||
    config.privatePackages?.tag !== false
  )
    throw Error(
      "Changesets must version the private coordinator without npm tags",
    );
  const workspace = manifest(root, "Cargo.toml");
  const locked = manifest(root, "Cargo.lock").package;
  for (const name of crates) {
    const pkg = manifest(root, `${cratePath(name)}/Cargo.toml`).package;
    if (pkg.name !== name || pkg.version?.workspace !== true)
      throw Error(`${name}: must inherit coordinated Cargo version`);
    const entry = locked.filter((item) => item.name === name);
    if (entry.length !== 1 || entry[0].version !== selected || entry[0].source)
      throw Error(`${name}: Cargo.lock version differs from workspace`);
    if (
      name !== "jj-acp" &&
      workspace.workspace.dependencies[name]?.version !== selected
    )
      throw Error(`${name}: internal registry version differs from workspace`);
  }
  if (!read(root, "CHANGELOG.md").includes(`\n## ${selected}\n`))
    throw Error("current release is missing from changelog");
  const readme = read(root, "README.md");
  if (!readme.includes(`cargo install jj-acp --version ${selected} --locked`))
    throw Error("installation example differs from workspace version");
  if (read(root, "crates/jj-acp-cli/README.md") !== readme)
    throw Error("packaged CLI README differs from root README");
  return selected;
}

export async function pendingChangesets(root = ".") {
  const notes = await readChangesets(resolve(root));
  for (const note of notes)
    for (const item of note.releases)
      if (
        item.name !== coordinator ||
        !["patch", "minor", "major"].includes(item.type)
      )
        throw Error("changesets must target the coordinated Cargo release");
  return notes;
}

export async function releaseState(root = ".") {
  const selected = checkRelease(root);
  const notes = await pendingChangesets(root);
  return {
    version: selected,
    tag: `v${selected}`,
    crates,
    pending: notes.length,
    changesets: notes.map(({ id, summary, releases }) => ({
      id,
      summary,
      releases,
    })),
  };
}

if (
  process.argv[1] &&
  resolve(process.argv[1]) === resolve(new URL(import.meta.url).pathname)
) {
  if (process.argv.includes("--plan"))
    console.log(JSON.stringify(await releaseState(), null, 2));
  else
    console.log(`Coordinated release metadata verified at ${checkRelease()}`);
}
