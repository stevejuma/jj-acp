# Changelog

## 0.1.0

### Minor Changes

Published 14 October 2026:
[GitHub release](https://github.com/stevejuma/jj-acp/releases/tag/v0.1.0). All
six Cargo crates share this version: jj-acp-protocol, jj-acp-agent,
jj-acp-testkit, jj-acp-runner, jj-acp-control and jj-acp.

Initial public release of the layered jj-acp harness. Includes one-shot tasks,
scoped version-1 scheduling, interactive version-2 sessions, bounded subprocess
cleanup, separate prepared context and trusted stdio MCP tool declarations.

Control version 3 provides warm workers with caller-scoped sessions and remains
experimental. M12 retains live-adapter permission/cancellation, cold native
consumer and broader platform qualification gaps. Fixture tests do not close
those gates. Prebuilt binaries target macOS 15 and Ubuntu 22.04 or newer, on
ARM64 and x86-64. macOS archives have no Developer ID notarization.

Source, CI, GitHub Pages specification, trusted Cargo publication and the four
native archives were qualified. Trusted MCP declarations include the integrated
local-compute work. This entry was backfilled for Changesets; the immutable
release source remains 07a53851d857e16f426ac0645c56904e8850de17. The
[historical record](docs/releases/0.1.0.md) is outside the pending release
queue.
