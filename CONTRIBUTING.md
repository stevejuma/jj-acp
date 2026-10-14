# Contributing

Read AGENTS.md and the owning canonical requirement under spec/src. Use the
README's pinned development prerequisites and frozen dependency installation.
Run the narrow regression first, then the applicable source gates; shared
changes and releases require all canonical check, test and build lanes.

Update the governing chapter, verification map and milestone evidence with
protected changes. Record actual commands and failure counts, retain
qualification gaps, and keep provider-backed checks opt-in. Tests use packaged
Python fixtures.

Local work uses Jujutsu commits for verified slices. GitHub CI also validates
ordinary Git checkouts with an explicit base/head range. Submit a pull request
against main; passing CI is required. Do not commit credentials, generated
books, local configuration, or vendored upstream sources. See docs/releasing.md
for publication and partial-release recovery.

Add a `pnpm changeset` note for a change that should ship, selecting the private
`@lapismd/jj-acp` coordinator. All six Cargo crates share its release version.
Describe user-visible behavior and choose patch, minor or major intent. Keep
published history under docs/releases; a completed release must not be requeued
as an active note. Use `pnpm release:version` for local preparation so
Changesets and Cargo metadata remain synchronized. Maintainers review the
automated Version Packages PR before merging it to start tagged qualification
and publication.
