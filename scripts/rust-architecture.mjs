import { existsSync, readFileSync, readdirSync } from "node:fs";
import { extname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

/**
 * Counted-line budget. Production Rust and first-party scripts stay at or below
 * 300 counted lines; test files at or below 500. This is a deliberate
 * divergence from jj-store, which budgets only web and script sources: the
 * upstream `buzz-acp` files this repository ports are far larger, so the budget
 * is what forces decomposition and keeps a future re-port mechanical.
 */
export const limits = { production: 300, test: 500, script: 300 };

const ignored = new Set([".git", ".jj", "node_modules", "spec", "target"]);

/** Count lines that carry code, excluding blanks and comments. */
export function countedLines(source) {
  let blockComment = false;
  let count = 0;
  for (const line of source.split("\n")) {
    const value = line.trim();
    if (blockComment) {
      if (value.includes("*/")) blockComment = false;
      continue;
    }
    if (!value || value.startsWith("//")) continue;
    if (value.startsWith("/*")) {
      blockComment = !value.includes("*/");
      continue;
    }
    count += 1;
  }
  return count;
}

export function fileKind(path) {
  if (
    /(?:^|\/)tests?\//u.test(path) ||
    /(?:^|\/)tests?\.rs$/u.test(path) ||
    /\.(?:test|spec)\.mjs$/u.test(path)
  )
    return "test";
  if (path.startsWith("scripts/")) return "script";
  return "production";
}

export function sourceFiles(root) {
  const found = [];
  const visit = (directory) => {
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      if (ignored.has(entry.name)) continue;
      const absolute = join(directory, entry.name);
      if (entry.isDirectory()) visit(absolute);
      else if ([".mjs", ".rs"].includes(extname(entry.name)))
        found.push(relative(root, absolute).replaceAll("\\", "/"));
    }
  };
  for (const directory of ["crates", "scripts", "tests"]) {
    const absolute = join(root, directory);
    if (existsSync(absolute)) visit(absolute);
  }
  return found.sort();
}

export function architectureErrors(root, baseline) {
  const errors = [];
  const files = sourceFiles(root);
  const current = new Set(files);
  for (const path of Object.keys(baseline.files))
    if (!current.has(path)) errors.push(`${path}: stale architecture baseline`);
  for (const path of files) {
    const source = readFileSync(join(root, path), "utf8");
    const count = countedLines(source);
    const allowed = baseline.files[path] ?? limits[fileKind(path)];
    if (count > allowed)
      errors.push(`${path}: ${count} counted lines exceeds ${allowed}`);
    const isTest = fileKind(path) === "test";
    const inTestkit = path.startsWith("crates/lapis-acp-testkit/");
    if (
      !isTest &&
      !inTestkit &&
      path.endsWith(".rs") &&
      /lapis_acp_testkit/u.test(source)
    )
      errors.push(`${path}: production source imports the testkit crate`);
    if (
      !isTest &&
      path.endsWith(".rs") &&
      /#\[(?:tokio::)?test\]/u.test(source)
    )
      errors.push(
        `${path}: production module defines tests inline; use a tests.rs sibling or crates/*/tests`,
      );
  }
  return errors;
}

export function run(root = process.cwd()) {
  const resolved = resolve(root);
  const baseline = JSON.parse(
    readFileSync(
      join(resolved, "scripts/rust-architecture-baseline.json"),
      "utf8",
    ),
  );
  const errors = architectureErrors(resolved, baseline);
  if (errors.length) throw new Error(errors.join("\n"));
  console.log(`rust architecture ok (${sourceFiles(resolved).length} modules)`);
}

if (
  process.argv[1] &&
  resolve(process.argv[1]) === resolve(new URL(import.meta.url).pathname)
)
  run(resolve(fileURLToPath(new URL("..", import.meta.url))));
