# Publication and release

The public repository is stevejuma/jj-acp. The initial version is 0.1.0. The CLI
package is jj-acp and installs the jj-acp executable; the existing CLI source
directory and jj_acp_cli library name remain. The layered libraries and dev-only
testkit are released at the same version. The npm package remains a private
development entry point.

## Requirements

| ID          | Requirement                                                                                                                                                                              |
| ----------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JAC-REL-001 | Published crates MUST declare versioned registry dependencies, portable metadata, packaged fixtures where applicable, and Apache-2.0 license and upstream notices.                       |
| JAC-REL-002 | CI MUST check committed base/head ranges in Git checkouts, preserve local Jujutsu checks, and run canonical and native platform qualification without model credentials.                 |
| JAC-REL-003 | GitHub Pages MUST publish the canonical mdBook from main with the repository site prefix; pull requests MUST build without deployment authority.                                         |
| JAC-REL-004 | Releases MUST match workspace versions and main ancestry, validate package contents and native executable archives, and include SHA-256 checksums.                                       |
| JAC-REL-005 | Initial crate publication MUST use explicitly configured token authentication; subsequent automation MUST use scoped trusted publishing and refuse mismatched existing package contents. |

## Supported initial profile

Native CI covers Ubuntu 22.04 and macOS 15 on ARM64 and x86-64. GNU/Linux
archives require an Ubuntu 22.04-compatible userspace. macOS distributions do
not include Developer ID notarization. Windows has no initial support claim.
Control v3 remains experimental; release tooling cannot close M12's live-adapter
gates. Deterministic fixtures remain separate from installed-runner evidence.

## Release gates and recovery

Release tags are `v<workspace-version>` on main history. The release workflow
qualifies the source, builds native archives, verifies registry package
identity, publishes missing versions and tests registry installation before
publishing GitHub assets. All published packages retain the same source
revision. Existing versions are accepted only when their archive checksum
matches the intended package; a different checksum blocks publication. Partial
publication may resume only for missing packages. Published versions and release
tags are never replaced.

The initial bootstrap uses local Cargo authentication. Each crate then
configures stevejuma/jj-acp, release.yml and the release environment as its
trusted publisher. No provider or registry credentials belong in tracked files,
logs or artifacts. GitHub workflow tokens have minimum job permissions; private
reports use GitHub's private vulnerability reporting feature. Operational steps
are in docs/releasing.md.
