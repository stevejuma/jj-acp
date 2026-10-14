import { existsSync } from "node:fs";
import { resolve } from "node:path";
import { execFileSync } from "node:child_process";

export function snapshot(
  root,
  env = process.env,
  execute,
  exists = existsSync,
) {
  const run =
    execute ??
    ((command, args, input) =>
      execFileSync(command, args, {
        cwd: root,
        encoding: "utf8",
        input,
        stdio: ["pipe", "pipe", "pipe"],
      }).trimEnd());
  const lines = (value) => value.split("\n").filter(Boolean);
  const base = env.JJ_ACP_CHECK_BASE;
  const head = env.JJ_ACP_CHECK_HEAD;
  if (Boolean(base) !== Boolean(head))
    throw Error("progress requires both base and head");
  if (!base && exists(resolve(root, ".jj"))) {
    const jj = (args) => run("jj", ["--no-pager", "--color=never", ...args]);
    const rev = env.JJ_ACP_CHECK_REVISION ?? "@";
    return {
      changed: lines(
        jj(["diff", "--name-only", "--from", `${rev}-`, "--to", rev]),
      ),
      paths: lines(jj(["file", "list", "-r", rev])),
      previous: (path) => jj(["file", "show", "-r", `${rev}-`, path]),
    };
  }
  const git = (args, input) => run("git", args, input);
  const selectedHead = head ?? "HEAD";
  let selectedBase = base;
  if (!selectedBase) {
    try {
      selectedBase = git(["rev-parse", "--verify", "HEAD^"]);
    } catch {
      selectedBase = git(["hash-object", "-t", "tree", "--stdin"], "");
    }
  }
  return {
    changed: git([
      "diff",
      "--name-only",
      "-z",
      selectedBase,
      selectedHead,
      "--",
    ])
      .split("\0")
      .filter(Boolean),
    paths: lines(git(["ls-tree", "-r", "--name-only", selectedHead])),
    previous: (path) => git(["show", `${selectedBase}:${path}`]),
  };
}
