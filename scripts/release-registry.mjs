import { readFileSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { crates, version } from "./release-version.mjs";
const selectedVersion = version();
const missing = [];
for (const name of crates) {
  const response = await fetch(
    `https://crates.io/api/v1/crates/${name}/${selectedVersion}/download`,
    {
      headers: {
        "User-Agent":
          "jj-acp release verification (github.com/stevejuma/jj-acp)",
      },
    },
  );
  if (response.status === 404) {
    missing.push(name);
    continue;
  }
  if (!response.ok) throw Error(`registry ${name}: HTTP ${response.status}`);
  const data = Buffer.from(await response.arrayBuffer());
  const local = readFileSync(`target/package/${name}-${selectedVersion}.crate`);
  const digest = (bytes) => createHash("sha256").update(bytes).digest("hex");
  if (digest(data) !== digest(local))
    throw Error(
      `${name} already exists with different package contents; stop publication`,
    );
}
if (process.argv.includes("--publish") && missing.length) {
  execFileSync(
    "cargo",
    ["publish", ...missing.flatMap((name) => ["-p", name]), "--locked"],
    { stdio: "inherit" },
  );
} else if (missing.length && !process.argv.includes("--status")) {
  throw Error(`missing registry releases: ${missing.join(", ")}`);
}
if (process.env.GITHUB_OUTPUT)
  writeFileSync(process.env.GITHUB_OUTPUT, `missing=${missing.length}\n`, {
    flag: "a",
  });
console.log(
  `${crates.length - missing.length}/${crates.length} existing releases verified`,
);
