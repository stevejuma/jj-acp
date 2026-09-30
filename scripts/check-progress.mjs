import { readFileSync } from "node:fs";
import { resolve, isAbsolute } from "node:path";
import { fileURLToPath } from "node:url";
import { execFileSync } from "node:child_process";

export const ledgerPath = "spec/src/milestones.md";
const statuses = new Set([
  "Planned",
  "In progress",
  "Blocked",
  "Partial",
  "Done",
  "Deferred",
]);
export const needsEvidence = (path) =>
  /^(?:rustfmt\.toml$|crates\/|tests\/|scripts\/|Cargo\.(?:toml|lock)$|package\.json$|pnpm-lock\.yaml$|rust-toolchain\.toml$|AGENTS\.md$|spec-validator\.config\.mjs$|docs\/upstream-lock|spec\/src\/verification\.md$)/.test(
    path,
  );
const list = (s) => (s === "-" ? [] : s.split(",").map((v) => v.trim()));

export function parseLedger(source) {
  const tasks = source
    .split("\n")
    .filter((line) => /^\| (?:Bootstrap|M\d)/.test(line))
    .map((line) => {
      const cells = line
        .trim()
        .slice(1, -1)
        .split("|")
        .map((c) => c.trim());
      if (cells.length !== 7) throw new Error(`invalid ledger row: ${line}`);
      const [id, status, depends, requirements, gate, next, evidence] = cells;
      return {
        id,
        status,
        depends: list(depends),
        requirements: list(requirements),
        gate,
        next,
        evidence: list(evidence),
      };
    });
  const blocks = [...source.matchAll(/```json\s*\n([\s\S]*?)\n```/g)];
  if (blocks.length !== 1)
    throw new Error("ledger requires exactly one JSON evidence block");
  const entries = JSON.parse(blocks[0][1]);
  if (!Array.isArray(entries)) throw new Error("evidence must be an array");
  return { tasks, entries };
}

export function validateLedger({
  source,
  read,
  requirements,
  changed = [],
  previousEntries = [],
}) {
  const errors = [];
  let parsed;
  try {
    parsed = parseLedger(source);
  } catch (err) {
    return [err.message];
  }
  const { tasks, entries } = parsed;
  const byId = new Map(tasks.map((t) => [t.id, t]));
  const evidence = new Map(entries.map((e) => [e.id, e]));
  const removedFiles = new Set(entries.flatMap((e) => e.removed_files ?? []));
  const removedTests = new Set(entries.flatMap((e) => e.removed_tests ?? []));
  if (!tasks.length) errors.push("ledger has no tasks");
  if (byId.size !== tasks.length) errors.push("duplicate task ID");
  if (evidence.size !== entries.length) errors.push("duplicate evidence ID");
  const checkFile = (path) => {
    if (isAbsolute(path) || path.split(/[\\/]/).includes("..")) {
      errors.push(`unsafe evidence path: ${path}`);
      return;
    }
    try {
      return read(path);
    } catch {
      if (!removedFiles.has(path))
        errors.push(`missing evidence file: ${path}`);
    }
  };
  for (const entry of entries) {
    if (!/^E-[A-Z0-9-]+$/.test(entry.id ?? ""))
      errors.push("invalid evidence ID");
    if (!Array.isArray(entry.tasks) || !entry.tasks.length)
      errors.push(`${entry.id}: missing task references`);
    for (const task of entry.tasks ?? [])
      if (!byId.has(task)) errors.push(`${entry.id}: unknown task ${task}`);
    for (const key of ["date", "kind", "versions", "formats"])
      if (!entry[key]) errors.push(`${entry.id}: missing ${key}`);
    if (!Array.isArray(entry.files) || !entry.files.length)
      errors.push(`${entry.id}: missing changed files`);
    for (const file of entry.files ?? []) checkFile(file);
    for (const file of entry.removed_files ?? []) {
      if (isAbsolute(file) || file.split(/[\\/]/).includes("..")) {
        errors.push(`unsafe removed evidence path: ${file}`);
      } else {
        try {
          read(file);
          errors.push(
            `${entry.id}: removed evidence file still exists: ${file}`,
          );
        } catch {}
      }
    }
    for (const test of entry.removed_tests ?? []) {
      const [path, name, extra] = test.split("#");
      if (!path || !name || extra)
        errors.push(`${entry.id}: invalid removed test reference ${test}`);
    }
    for (const test of entry.tests ?? []) {
      const [path, name, extra] = test.split("#");
      const text = checkFile(path);
      if (!name || extra)
        errors.push(`${entry.id}: invalid test reference ${test}`);
      else if (text !== undefined && !removedTests.has(test)) {
        const escaped = name.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
        const definition = new RegExp(
          `(?:\\bfn\\s+${escaped}\\b|\\bdef\\s+${escaped}\\b|\\btest\\(["']${escaped}["'])`,
        );
        if (!definition.test(text))
          errors.push(`${entry.id}: missing named test ${test}`);
      }
    }
    if (!Array.isArray(entry.runs) || !entry.runs.length)
      errors.push(`${entry.id}: missing actual runs`);
    for (const run of entry.runs ?? []) {
      if (
        !run.command ||
        !["pass", "fail"].includes(run.result) ||
        !Number.isInteger(run.failures) ||
        run.failures < 0 ||
        (run.result === "pass" && run.failures !== 0) ||
        (run.result === "fail" && run.failures === 0)
      )
        errors.push(`${entry.id}: invalid run result`);
    }
    if (!["pass", "pending", "blocked", "fail"].includes(entry.result))
      errors.push(`${entry.id}: invalid result`);
    if (!Array.isArray(entry.remaining))
      errors.push(`${entry.id}: missing remaining gaps`);
    if (
      entry.result === "pass" &&
      (entry.remaining?.length ||
        !entry.runs?.some((run) => run.result === "pass"))
    )
      errors.push(`${entry.id}: pass requires successful runs and no gaps`);
  }
  for (const task of tasks) {
    if (!/^(?:Bootstrap|M\d+)(?:\.\d+)?$/.test(task.id))
      errors.push(`invalid task ID ${task.id}`);
    if (!statuses.has(task.status)) errors.push(`${task.id}: invalid status`);
    if (!task.gate || !task.next || !task.requirements.length)
      errors.push(`${task.id}: missing gate, next action, or requirements`);
    for (const id of task.requirements)
      if (!requirements.has(id))
        errors.push(`${task.id}: unknown requirement ${id}`);
    for (const id of task.depends)
      if (!byId.has(id)) errors.push(`${task.id}: unknown dependency ${id}`);
    for (const id of task.evidence)
      if (!evidence.has(id) || !evidence.get(id).tasks?.includes(task.id))
        errors.push(`${task.id}: missing evidence ${id}`);
    if (
      ["In progress", "Blocked", "Partial", "Done"].includes(task.status) &&
      !task.evidence.length
    )
      errors.push(`${task.id}: missing evidence`);
    if (task.status === "Done") {
      const successful = task.evidence
        .map((id) => evidence.get(id))
        .filter((e) => e?.result === "pass");
      if (!successful.length)
        errors.push(`${task.id}: Done without passing gate evidence`);
      for (const dep of task.depends)
        if (byId.get(dep)?.status !== "Done")
          errors.push(`${task.id}: incomplete dependency ${dep}`);
      for (const child of tasks.filter((t) => t.id.startsWith(`${task.id}.`)))
        if (child.status !== "Done")
          errors.push(`${task.id}: incomplete child ${child.id}`);
      if (task.id === "M6" && !successful.some((e) => e.kind === "real-runner"))
        errors.push("M6: real installed-runner evidence required");
    }
  }
  const visiting = new Set(),
    visited = new Set();
  function visit(id) {
    if (visiting.has(id)) {
      errors.push(`dependency cycle at ${id}`);
      return;
    }
    if (visited.has(id) || !byId.has(id)) return;
    visiting.add(id);
    for (const dep of byId.get(id).depends) visit(dep);
    visiting.delete(id);
    visited.add(id);
  }
  for (const task of tasks) visit(task.id);
  if (changed.some(needsEvidence)) {
    if (!changed.includes(ledgerPath))
      errors.push("protected change requires a same-change ledger update");
    const old = new Map(previousEntries.map((e) => [e.id, JSON.stringify(e)]));
    if (!entries.some((e) => old.get(e.id) !== JSON.stringify(e)))
      errors.push("protected change requires a new or updated evidence entry");
  }
  return [...new Set(errors)];
}

export function run(root) {
  const read = (path) => readFileSync(resolve(root, path), "utf8");
  const jj = (args) =>
    execFileSync("jj", ["--no-pager", "--color=never", ...args], {
      cwd: root,
      encoding: "utf8",
      stdio: ["ignore", "pipe", "pipe"],
    });
  const rev = process.env.LAPIS_ACP_CHECK_REVISION ?? "@";
  const changed = jj(["diff", "--name-only", "--from", `${rev}-`, "--to", rev])
    .trim()
    .split("\n")
    .filter(Boolean);
  let previousEntries = [];
  const parentFiles = jj(["file", "list", "-r", `${rev}-`]).split("\n");
  if (parentFiles.includes(ledgerPath))
    previousEntries = parseLedger(
      jj(["file", "show", "-r", `${rev}-`, ledgerPath]),
    ).entries;
  const paths = jj(["file", "list"])
    .split("\n")
    .filter(
      (p) =>
        /^spec\/src\/.+\.md$/.test(p) &&
        !/(?:milestones|verification)\.md$/.test(p),
    );
  const requirements = new Set(
    paths.flatMap((p) =>
      [...read(p).matchAll(/^\| (LAC-[A-Z]+-\d{3}) \|/gm)].map((m) => m[1]),
    ),
  );
  const errors = validateLedger({
    source: read(ledgerPath),
    read,
    requirements,
    changed,
    previousEntries,
  });
  if (errors.length) throw new Error(errors.join("\n"));
  console.log(
    `progress ok (${parseLedger(read(ledgerPath)).tasks.length} tasks; evidence bookkeeping only)`,
  );
}

if (
  process.argv[1] &&
  resolve(process.argv[1]) === fileURLToPath(import.meta.url)
) {
  try {
    run(fileURLToPath(new URL("..", import.meta.url)));
  } catch (err) {
    console.error(err.message);
    process.exitCode = 1;
  }
}
