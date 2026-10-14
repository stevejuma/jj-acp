# Changesets

Run `pnpm changeset` for a change that should ship. Select the private
`@lapismd/jj-acp` coordinator and choose patch, minor or major. This versions
all six Cargo crates together. The npm package stays private.

`pnpm release:status` shows pending notes. The main workflow creates or updates
a **Version Packages** PR using `pnpm release:version`. Review its changelog,
coordinated Cargo metadata and canonical preparation evidence, then merge after
all required checks pass. The workflow qualifies the merged source, creates its
immutable `v<version>` tag and dispatches the existing Cargo/native release.

For local preparation, use `pnpm release:version`; do not run raw
`changeset version`, which would leave Cargo metadata unsynchronized. This
command never publishes or creates a tag. Run `pnpm release:check` to verify
version consistency and follow [release guidance](../docs/releasing.md).

The already published `0.1.0` release is recorded in
[CHANGELOG.md](../CHANGELOG.md) and [the archive](../docs/releases/0.1.0.md).
Archived notes are historical records and are not pending changesets. There is
no pending bump from adding this workflow.
