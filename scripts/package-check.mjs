import { readFileSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { crates, version } from "./release-version.mjs";
for (const name of crates) {
  const layer = name === "jj-acp" ? "cli" : name.slice("jj-acp-".length);
  const root = `crates/jj-acp-${layer}`;
  if (
    readFileSync(`${root}/LICENSE.md`, "utf8") !==
    readFileSync("LICENSE.md", "utf8")
  )
    throw Error(`${name}: stale license`);
  if (
    readFileSync(`${root}/NOTICE.md`, "utf8") !==
    readFileSync("docs/licenses/NOTICE.md", "utf8")
  )
    throw Error(`${name}: stale attribution`);
  const manifest = readFileSync(`${root}/Cargo.toml`, "utf8");
  if (
    !manifest.includes('publish = ["crates-io"]') ||
    !manifest.includes("repository.workspace = true")
  )
    throw Error(`${name}: publication metadata missing`);
  const list = execFileSync(
    "cargo",
    [
      "package",
      "-p",
      name,
      "--list",
      "--locked",
      ...(process.argv.includes("--allow-dirty") ? ["--allow-dirty"] : []),
    ],
    { encoding: "utf8" },
  );
  for (const required of ["README.md", "LICENSE.md", "NOTICE.md", "Cargo.toml"])
    if (!list.split("\n").includes(required))
      throw Error(`${name}: missing archive ${required}`);
  if (name === "jj-acp-testkit")
    for (const file of ["fixture", "interactive", "fleet"])
      if (!list.includes(`tests/fixtures/${file}_agent.py`))
        throw Error(`testkit: missing ${file} fixture`);
}
console.log(
  `Publication metadata and contents verified for ${crates.length} crates at ${version()}`,
);
