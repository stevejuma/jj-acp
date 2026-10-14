# Releasing jj-acp

Use Changesets for one coordinated version across the six Cargo crates. Add a
note with `pnpm changeset`, targeting the private @lapismd/jj-acp coordinator.
Choose patch, minor or major and describe the release-worthy behavior. Inspect
the queue with `pnpm release:status` or `pnpm release:plan`.

After feature notes merge into main, the Changesets workflow creates or updates
a Version Packages PR. It runs `pnpm release:version`: Changesets bumps the
coordinator and generates CHANGELOG.md, then the wrapper synchronizes the Cargo
workspace/internal versions, Cargo.lock, pnpm lockfile, README installation
example and packaged CLI README. Recognized Changesets and Dependabot PRs are
exempt from same-change specification and ledger updates. Canonical validation,
ledger integrity, tests, builds and packaging still run; ordinary PRs retain the
bookkeeping requirements. All required CI checks and a Pages build are
explicitly dispatched for the bot PR. Review and merge only after they pass.

The merged candidate is qualified again before a missing immutable v<version>
tag is created. The workflow dispatches release.yml for that exact tag,
preserving the configured crates.io trusted publishers and native archive
pipeline. Existing tags are retained; ordinary main updates at an already
published version do not publish again. The npm coordinator is never published
or tagged by Changesets.

For local preparation use `pnpm release:version`, followed by
`pnpm release:check`. Raw `changeset version` leaves Cargo metadata stale and is
not the repository's version command. With no pending notes the wrapper is a
no-op. Refresh packaged licenses/notices when their authoritative files change.

On a cold checkout run `cargo fetch --locked` once before preparation. The
wrapper checks that locked dependencies are available offline before consuming
notes. CI and the Changesets workflow perform this dependency bootstrap before
the offline metadata checks; a missing cache cannot leave a half-prepared bump.

Run the exact affected regressions, then pnpm spec:check, pnpm check, pnpm test,
pnpm build, pnpm check:workflow and pnpm release:dry-run. Packaging verification
must pass without --no-verify. Before committing, local dry runs may use
--allow-dirty solely to validate a prepared change; publication requires the
clean committed source. CI supplies JJ_ACP_CHECK_BASE and JJ_ACP_CHECK_HEAD to
validate committed ranges. Commit verified slices with Jujutsu.

Submit prepared changes through a pull request and wait for canonical/native CI
and Pages. The normal path uses the automatic tag handoff after the Version
Packages merge. An explicit manual tag uses jj tag set v<version> -r main and jj
git push --remote origin --tag v<version> only for qualified source with no
pending notes. Tags are immutable release identities. Manual release reruns take
the existing tag; they never choose a new source revision.

GitHub must allow Actions to create pull requests. The Changesets job grants
only the contents, pull-request and workflow-dispatch permissions needed for
that handoff. No personal GitHub or long-lived registry token is required.

The published 0.1.0 release is recorded in CHANGELOG.md and releases/0.1.0.md.
It has no active Changeset and must not be backfilled as a new pending bump.

## Initial crates.io bootstrap

crates.io requires an existing crate before trusted publishing can be
configured. Authenticate locally with cargo login using a short-lived token
authorized to publish the new crate names. Keep its value out of chat, logs and
repository files. On the exact clean release revision run cargo package
--workspace --locked and node scripts/release-registry.mjs --publish. Packages
are ordered protocol, agent, testkit, runner, control, jj-acp. Verify a fresh
installation with cargo install jj-acp --version <version> --locked --root
<temporary-directory>.

Configure every crate's trusted publisher: owner stevejuma, repository jj-acp,
workflow release.yml and environment release. Enable trusted-publishing-only
once configured, and revoke the bootstrap token. Subsequent release runs use the
Rust crates.io authentication Action and temporary OIDC credentials.

## Partial failure

Keep the tag on its original source. Rerun the release workflow for that tag. It
verifies the checksum of every existing .crate against the locally built
package, publishes only missing versions, and refuses any mismatch. Revoke an
expired/failed bootstrap token and authenticate locally if the initial release
is incomplete. Never bump or overwrite an existing version to hide a failure. If
a published package is defective, fix its owner and publish a new version;
yanking is an explicit maintainer decision.

Native archives include the executable, README, complete Apache-2.0 license and
upstream notice. SHA256SUMS lists all four archives. GitHub assets are finalized
only after registry verification and installation pass. The hosted specification
tracks main and can include development changes beyond the latest release. Keep
M12's live-adapter gaps visible; release fixture results do not close them.
