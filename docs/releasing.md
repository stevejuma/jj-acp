# Releasing jj-acp

Prepare one coordinated version across the workspace. Update Cargo.lock,
internal dependency versions, the README installation version, changelog and
canonical release evidence. Refresh each crate's packaged license/notice and the
CLI README when their authoritative root files change.

Run the exact affected regressions, then pnpm spec:check, pnpm check, pnpm test,
pnpm build, pnpm check:workflow and pnpm release:dry-run. Packaging verification
must pass without --no-verify. Before committing, local dry runs may use
--allow-dirty solely to validate a prepared change; publication requires the
clean committed source. CI supplies JJ_ACP_CHECK_BASE and JJ_ACP_CHECK_HEAD to
validate committed ranges. Commit verified slices with Jujutsu.

Push main and wait for all canonical and native CI checks and the Pages build.
Use jj tag set v<version> -r main and jj git push --remote origin --tag
v<version> for the verified source. Tags are immutable release identities.
Manual release reruns take the existing tag; they never choose a new source
revision.

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
