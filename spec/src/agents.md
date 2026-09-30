# Runner catalog

A runner is an external ACP-speaking agent that `lapis-acp` can spawn. Runners
are data, not code: a catalog entry names an executable, its arguments, an
environment floor, an MCP transport and an install hint. Adding support for a
new agent MUST be a catalog change.

The catalog is tiered so that trust decreases as flexibility increases, matching
the model the upstream harness uses and the model `@lapismd/ai-host` already
exposes to Lapis consumers.

| Tier | Source                                                | Editable | Auto-install |
| ---- | ----------------------------------------------------- | -------- | ------------ |
| 1    | Compiled-in defaults                                  | No       | Never        |
| 2    | Catalog file supplied by launch configuration         | No       | Never        |
| 3    | Caller-supplied definition in a task or control frame | No       | Never        |

No tier may auto-install. Availability is determined by resolving the declared
executable, and an unavailable runner reports its install hint rather than
attempting a download or executing a shell command.

## Requirements

| ID            | Requirement                                                                                                                                  |
| ------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| LAC-AGENT-001 | A runner MUST be declared as data carrying identifier, label, command, arguments, environment floor, MCP transport and install hint.         |
| LAC-AGENT-002 | Adding a runner MUST NOT require a code change, an install shell command or a network fetch.                                                 |
| LAC-AGENT-003 | Reserved harness environment keys MUST be stripped before a definition or caller override is merged, and MUST NOT be settable by any tier.   |
| LAC-AGENT-004 | A disabled or unknown runner identifier MUST fail resolution and MUST NOT fall back to a default runner.                                     |
| LAC-AGENT-005 | Runner identifiers MUST be validated against a fixed pattern and MUST be unique within a resolved catalog.                                   |
| LAC-AGENT-006 | The launch environment MUST be assembled from the inherited environment, the definition floor and caller overrides in that precedence order. |

## Reserved environment

The harness injects identity and correlation variables into a spawned agent so
that tools it provides can attribute work correctly. Those names are reserved. A
runner definition, a catalog file or a caller-supplied override that sets one is
stripped before merge, and the strip is recorded so the operator can see that a
definition attempted to claim harness identity.

This is the only place `lapis-acp` performs anything resembling an authorization
check, and it is a namespace guard rather than an identity system. There is no
keypair, no signature, no token and no peer credential anywhere in this
repository. The controlling application decided to run this runner with this
environment; `lapis-acp` only refuses to let the runner definition contradict
that decision.

## Default catalog

The compiled-in tier names the adapters that are already in use elsewhere in the
workspace, so an operator who has one of them installed needs no configuration.
Identifiers are stable and reserved: a lower tier cannot shadow them.

| Identifier | Adapter                                      | MCP transport |
| ---------- | -------------------------------------------- | ------------- |
| `codex`    | `@agentclientprotocol/codex-acp`             | stdio         |
| `claude`   | `claude-agent-acp`, legacy `claude-code-acp` | stdio         |
| `opencode` | `opencode`                                   | stdio         |
| `goose`    | `goose acp`                                  | stdio         |

A runner that speaks ACP over stdio and satisfies the protocol requirements in
[ACP wire protocol](protocol.md) works without being listed here. The catalog
exists for discovery and labelling, not as an allowlist.
