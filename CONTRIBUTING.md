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
