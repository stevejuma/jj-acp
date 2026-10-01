import { execFileSync } from "node:child_process";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

/**
 * Internal crate layering. A crate may depend only on the crates listed here.
 * Mirrors the jj-store boundary gate so an invalid edge fails before review.
 */
export const layers = {
  "jj-acp-protocol": [],
  "jj-acp-agent": ["jj-acp-protocol"],
  "jj-acp-runner": ["jj-acp-protocol", "jj-acp-agent"],
  "jj-acp-control": ["jj-acp-protocol", "jj-acp-agent", "jj-acp-runner"],
  "jj-acp-cli": [
    "jj-acp-protocol",
    "jj-acp-agent",
    "jj-acp-runner",
    "jj-acp-control",
  ],
  "jj-acp-testkit": ["jj-acp-protocol", "jj-acp-agent"],
};

/**
 * Fail-closed dependency exclusions. These encode the port's deliberate
 * omissions: no Nostr, no authentication, no network client, no database and
 * no upstream Nostr Git signing helper. A future re-port that reintroduces one
 * fails this gate rather than review.
 */
export const excluded =
  /^(?:nostr|nostr-.*|reqwest|hyper|axum|tungstenite|tokio-tungstenite|rustls|native-tls|openssl|evalexpr|git-credential-nostr|git-sign-nostr|turso|rusqlite|sqlx|sqlx-.*|libsql|redis|postgres)$/;

export function dependencyErrors(packages) {
  const errors = [];
  const known = new Set(Object.keys(layers));
  for (const pkg of packages) {
    if (!known.has(pkg.name)) {
      errors.push(`${pkg.name}: crate is absent from the boundary matrix`);
      continue;
    }
    for (const dep of pkg.dependencies) {
      if (excluded.test(dep.name))
        errors.push(`${pkg.name}: excluded dependency ${dep.name}`);
      if (dep.name === "jj-acp-testkit" && dep.kind !== "dev")
        errors.push(`${pkg.name}: testkit is a dev-only dependency`);
      if (!known.has(dep.name)) continue;
      if (!layers[pkg.name].includes(dep.name) && dep.kind !== "dev")
        errors.push(`${pkg.name}: invalid layer dependency ${dep.name}`);
    }
  }
  return errors;
}

export function run(root) {
  const metadata = JSON.parse(
    execFileSync(
      "cargo",
      ["metadata", "--no-deps", "--format-version", "1", "--locked"],
      { cwd: root, encoding: "utf8" },
    ),
  );
  const errors = dependencyErrors(metadata.packages);
  if (errors.length) throw new Error(errors.join("\n"));
  console.log(`crate boundaries ok (${metadata.packages.length} packages)`);
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
