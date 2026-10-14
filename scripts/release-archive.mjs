import {
  mkdtempSync,
  mkdirSync,
  copyFileSync,
  chmodSync,
  readFileSync,
  writeFileSync,
  rmSync,
} from "node:fs";
import { join, resolve } from "node:path";
import { tmpdir } from "node:os";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { version } from "./release-version.mjs";
const target = process.argv[2];
if (!/^(x86_64|aarch64)-(apple-darwin|unknown-linux-gnu)$/.test(target ?? ""))
  throw Error("unsupported release target");
const temp = mkdtempSync(join(tmpdir(), "jj-acp-archive-"));
try {
  const name = `jj-acp-${version()}-${target}`;
  const stage = join(temp, name);
  mkdirSync(stage);
  for (const [source, destination] of [
    [`target/${target}/release/jj-acp`, "jj-acp"],
    ["README.md", "README.md"],
    ["LICENSE.md", "LICENSE.md"],
    ["docs/licenses/NOTICE.md", "NOTICE.md"],
  ])
    copyFileSync(source, join(stage, destination));
  chmodSync(join(stage, "jj-acp"), 0o755);
  const reported = execFileSync(join(stage, "jj-acp"), ["--version"], {
    encoding: "utf8",
  }).trim();
  if (reported !== `jj-acp ${version()}`)
    throw Error(`wrong binary version: ${reported}`);
  execFileSync(join(stage, "jj-acp"), ["--help"]);
  mkdirSync("dist", { recursive: true });
  const archive = `${name}.tar.gz`;
  const epoch = execFileSync("git", ["show", "-s", "--format=%ct", "HEAD"], {
    encoding: "utf8",
  }).trim();
  execFileSync("python3", [
    "-c",
    `
import gzip, tarfile, sys
archive, stage, name, epoch = sys.argv[1:]
def normalize(info):
    info.uid = info.gid = 0
    info.uname = info.gname = ""
    info.mtime = int(epoch)
    info.mode = 0o755 if info.isdir() or info.name.endswith("/jj-acp") else 0o644
    return info
with open(archive, "wb") as output:
    with gzip.GzipFile(filename="", mode="wb", fileobj=output, mtime=int(epoch)) as zipped:
        with tarfile.open(fileobj=zipped, mode="w", format=tarfile.USTAR_FORMAT) as bundle:
            bundle.add(stage, arcname=name, filter=normalize)
`,
    resolve("dist", archive),
    stage,
    name,
    epoch,
  ]);
  const hash = createHash("sha256")
    .update(readFileSync(join("dist", archive)))
    .digest("hex");
  writeFileSync(join("dist", `${archive}.sha256`), `${hash}  ${archive}\n`);
  console.log(archive);
} finally {
  rmSync(temp, { recursive: true, force: true });
}
