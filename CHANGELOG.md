# Changelog

## 0.1.0

Initial public release of the layered jj-acp harness. Includes one-shot tasks,
scoped version-1 scheduling, interactive version-2 sessions, bounded subprocess
cleanup, separate prepared context and trusted stdio MCP tool declarations.

Control version 3 provides warm workers with caller-scoped sessions and remains
experimental. M12 retains live-adapter permission/cancellation, cold native
consumer and broader platform qualification gaps. Fixture tests do not close
those gates. Prebuilt binaries target macOS 15 and Ubuntu 22.04 or newer, on
ARM64 and x86-64. macOS archives have no Developer ID notarization.
