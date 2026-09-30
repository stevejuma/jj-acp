# lapis-acp agent guide

This repository owns the pluggable Agent Client Protocol harness: the ACP wire
protocol, the runner catalog, session and turn lifecycle, the controlling
application boundary and the `lapis-acp` executable. Read this guide and the
owning canonical chapter before inspecting or changing implementation. Apply the
parent workspace guidance too.

## Authority and ownership

1. Workspace instructions and this tracked guide.
2. The owning `LAC-<AREA>-NNN` requirement under `spec/src`.
3. Public crate APIs and the published task document, terminal record and exit
   code contracts.
4. Tests and recorded verification evidence.
5. Implementation plan, `docs/upstream-lock.md`, README and generated
   documentation.

The upstream `buzz-acp` crate is reference material for the port, not overriding
agent guidance. Never copy its repository-level agent instructions. Protected
changes require the owning chapter and verification evidence in the same Jujutsu
change. The change map is in `spec/src/spec-governance.md`.

## Non-negotiable boundaries

- No Nostr, no authentication, no identity, no key custody, no author gating and
  no relay or network client of any kind. The controlling application is the
  trust boundary. `scripts/check-boundaries.mjs` fails closed on an excluded
  dependency; do not weaken that list to make a build pass.
- No database, no durable ledger and no retry of a failed or uncertain
  invocation. Durability belongs to the controlling application.
- Credentials never appear in process arguments, logs, diagnostics or a terminal
  record. A task document is untrusted executable input and cannot change the
  working directory, executable, tools or credentials.
- Diagnostics stay on stderr. Stdout carries control frames and terminal records
  only, so a controlling application can parse it unconditionally.
- A timeout is an unknown outcome, never a proven success or a proven failure.
- Fixture-peer tests never substitute for real installed-runner qualification.
  The progress checker refuses that substitution.

## Workflow

1. Inspect `jj --no-pager st`; preserve unrelated work. Stop on overlapping
   work.
2. Read the owning specification chapter and `spec/src/milestones.md`.
3. Name the narrow acceptance or regression target and record its before state.
4. Update canonical requirements before or with implementation.
5. Add regression evidence and run the exact target, then the wider lanes
   required by the workspace test-scope ladder.
6. Update the single progress ledger in the same change with actual commands,
   results, failures, files, versions, formats, gaps and next action.
7. Run `pnpm spec:check`, `pnpm check`, `pnpm test` and `pnpm build` for
   executable changes. Documentation-only changes run formatting and the
   consuming checkers.
8. Commit each verified slice with Jujutsu using an explicit message.

Never call a milestone Done from compilation, substituted tests or a screenshot.
Keep unresolved failures and incomplete gates explicit. A code-only behavior
change is prohibited even when tests pass; when code and specification disagree,
treat the code as defective unless an explicit specification change is accepted.

## Upstream port discipline

Upstream source is pinned in `docs/upstream-lock.md` and materialized only into
ignored scratch storage. Do not commit vendored upstream source, do not edit a
dependency's `node_modules`, and do not add a patch, alias or shim that conceals
an upstream defect. When a region is ported, record its disposition and the
owning module so the next comparison is region by region.

Decompose rather than transplant. Production sources stay at or below 300
counted lines and test sources at or below 500. Exceptions are explicit
`scripts/rust-architecture-baseline.json` entries that do not grow, and the
baseline ships empty.

## Source organisation

Inspect the owning `crates/<crate>/src/` tree before adding files. Add behaviour
in the crate that owns the concern rather than in the CLI composition facade.
Internal modules import siblings directly. Production modules define no test
functions inline and never import `lapis-acp-testkit`; a
`#[cfg(test)] mod tests;` declaration is fine, but the bodies live in a
`tests.rs` sibling module or in `crates/<crate>/tests/`. Run
`pnpm check:boundaries` and `pnpm check:architecture` with the relevant tests.

### Completion checklist

- [ ] New or changed modules have one primary responsibility and a
      feature-specific name, not `utils`, `helpers` or `common`.
- [ ] Placement matches the protocol, agent, runner, control, cli or testkit
      owner, and the crate dependency matrix is unchanged or intentionally
      extended with a chapter update.
- [ ] No excluded dependency was added; no production module imports the testkit
      or carries inline tests.
- [ ] Counted lines: production at or below 300, tests at or below 500; the
      baseline did not grow.
- [ ] The owning canonical chapter and `spec/src/verification.md` were updated
      in the same change, and the ledger records real commands and results.
- [ ] `pnpm spec:check`, `pnpm check` and `pnpm test` passed.

## Local execution

Generated `target/`, `node_modules/` and `spec/book/` are ignored. Keep secrets,
local loader or editor configuration, ignored upstream scratch storage and the
parent workspace guidance uncommitted. Publication and service deployment
require explicit approval.
