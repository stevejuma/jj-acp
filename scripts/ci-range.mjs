import { execFileSync } from "node:child_process";
import { appendFileSync } from "node:fs";
const git = (args, input) =>
  execFileSync("git", args, { encoding: "utf8", input }).trim();
const head = git(["rev-parse", process.env.CI_HEAD_SHA || "HEAD"]);
let base = process.env.CI_BASE_SHA;
if (base && /^0+$/.test(base)) {
  base = git(["hash-object", "-t", "tree", "--stdin"], "");
} else if (!base) {
  try {
    base = git(["rev-parse", `${head}^`]);
  } catch {
    base = git(["hash-object", "-t", "tree", "--stdin"], "");
  }
} else if (process.env.CI_EVENT === "pull_request") {
  base = git(["merge-base", base, head]);
}
if (!/^[a-f0-9]{40,64}$/.test(base) || !/^[a-f0-9]{40,64}$/.test(head))
  throw Error("invalid CI revision");
appendFileSync(
  process.env.GITHUB_ENV,
  `JJ_ACP_CHECK_BASE=${base}\nJJ_ACP_CHECK_HEAD=${head}\n`,
);
console.log(`Checking ${base}..${head}`);
