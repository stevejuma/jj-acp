import { existsSync, readFileSync, readdirSync } from "node:fs";
export const crates = [
  "jj-acp-protocol",
  "jj-acp-agent",
  "jj-acp-testkit",
  "jj-acp-runner",
  "jj-acp-control",
  "jj-acp",
];
export function version(root = ".") {
  const value = readFileSync(`${root}/Cargo.toml`, "utf8").match(
    /^version = "([^"]+)"$/m,
  )?.[1];
  if (!/^\d+\.\d+\.\d+$/.test(value))
    throw Error("invalid workspace release version");
  return value;
}
export function validateTag(tag, selectedVersion) {
  if (tag !== `v${selectedVersion}`)
    throw Error("tag does not match workspace version");
}

export function validateReleaseSource(root = ".") {
  const directory = `${root}/.changeset`;
  if (
    existsSync(directory) &&
    readdirSync(directory, { withFileTypes: true }).some(
      (entry) =>
        entry.isDirectory() ||
        entry.name === "pre.json" ||
        (entry.name.endsWith(".md") && entry.name !== "README.md"),
    )
  )
    throw Error("tagged release source contains pending changesets");
}
