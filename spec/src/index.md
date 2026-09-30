# lapis-acp specification

This book is the canonical behavior contract for `lapis-acp`, a pluggable Agent
Client Protocol harness that runs coding agents on behalf of a controlling
application. Read [architecture](architecture.md),
[governance](spec-governance.md), [milestones](milestones.md) and
[verification](verification.md) before implementing a slice.

`lapis-acp` is a port of the upstream `buzz-acp` harness with its Nostr intake,
identity, author gating, key custody and relay publication removed. The
controlling application is the trust boundary. The pinned upstream source and
the per-file disposition of the port are recorded in `docs/upstream-lock.md`,
and adopted-source attribution lives in `docs/licenses/NOTICE.md`.

The first delivery is a one-shot prepared task runner. A persistent control
transport, a standing agent pool and the `jj-store` agent-runner integration
remain explicitly deferred and are tracked in the ledger rather than assumed.
