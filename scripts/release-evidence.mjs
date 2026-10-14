import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { parseLedger } from "./check-progress.mjs";
import { read } from "./release-state.mjs";

export function recordPreparation(root, selected, removed, runs) {
  const start = "<!-- changesets-candidate:start -->";
  const end = "<!-- changesets-candidate:end -->";
  for (const chapter of ["architecture", "cli", "release"]) {
    const path = `spec/src/${chapter}.md`;
    const section = `${start}\n\n## Prepared release candidate\n\nChangesets prepared coordinated version ${selected}. This updates package\nmetadata and installation examples without changing runtime contracts.\nPreparation does not establish product qualification or registry publication;\nthe required CI and tagged release workflow own those gates.\n\n${end}\n`;
    const source = read(root, path);
    const next = source.includes(start)
      ? source.replace(new RegExp(`${start}[\\s\\S]*?${end}\\n?`), section)
      : `${source.trimEnd()}\n\n${section}`;
    writeFileSync(join(root, path), next);
  }
  const source = read(root, "spec/src/milestones.md");
  const ledger = parseLedger(source);
  const id = `E-VERSION-${selected.replaceAll(".", "-")}`;
  if (ledger.entries.some((entry) => entry.id === id))
    throw Error("this version already has preparation evidence");
  ledger.entries.push({
    id,
    tasks: ["M15"],
    date: new Date().toISOString().slice(0, 10),
    kind: "changesets version preparation",
    versions: `Coordinated candidate ${selected}; Rust toolchain from rust-toolchain.toml`,
    formats:
      "Cargo manifests/lock; private Changesets coordinator; existing protocols unchanged",
    files: [
      "package.json",
      "pnpm-lock.yaml",
      "Cargo.toml",
      "Cargo.lock",
      "CHANGELOG.md",
      "README.md",
      "crates/jj-acp-cli/README.md",
      "spec/src/architecture.md",
      "spec/src/cli.md",
      "spec/src/release.md",
      "spec/src/milestones.md",
    ],
    removed_files: removed,
    tests: [
      "scripts/changesets.test.mjs#changesets_prepare_coordinated_cargo_version_and_consume_notes",
    ],
    runs,
    result: "pass",
    remaining: [],
    note: "Only version metadata preparation was validated here. Required product/native CI and immutable tagged publication remain separate gates; existing release evidence is retained.",
  });
  writeFileSync(
    join(root, "spec/src/milestones.md"),
    source.replace(
      /```json\n[\s\S]*?\n```/,
      `\`\`\`json\n${JSON.stringify(ledger.entries, null, 2)}\n\`\`\``,
    ),
  );
}
