# ACP wire protocol

`lapis-acp-protocol` owns the Agent Client Protocol surface that `lapis-acp`
speaks to an agent subprocess over stdin and stdout. It is a hand-rolled
JSON-RPC implementation, matching upstream: there is no ACP crate dependency to
vendor, and no network transport of any kind.

Framing is newline-delimited JSON with an explicit per-frame byte cap. A frame
that exceeds the cap is a protocol error, never a silent truncation and never an
unbounded allocation. Bounded reads are what allow the harness to survive a
malfunctioning agent that emits an endless line.

## Requirements

| ID            | Requirement                                                                                                                                                                             |
| ------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| LAC-PROTO-001 | The protocol crate MUST encode and decode ACP JSON-RPC frames over newline-delimited stdio with an explicit per-frame byte cap and MUST fail closed on overflow.                        |
| LAC-PROTO-002 | The client MUST request protocol version 2 at `initialize` and MUST fall back to version 1 when the agent reports it, recording the negotiated version.                                 |
| LAC-PROTO-003 | The client MUST support `session/new`, `session/prompt`, `session/cancel`, `session/set_model`, `session/set_config_option`, inbound `session/request_permission` and `session/update`. |
| LAC-PROTO-004 | A turn MUST terminate in exactly one stop reason drawn from `end_turn`, `max_tokens`, `max_turn_requests`, `refusal` and `cancelled`.                                                   |
| LAC-PROTO-005 | An agent error MUST be surfaced as a typed protocol error with the agent's code and message, and MUST NOT be echoed as raw text into a terminal record.                                 |
| LAC-PROTO-006 | The protocol crate MUST remain free of process spawning, scheduling, credential handling and transport policy.                                                                          |

## Method direction

| Method                       | Direction    | Role                                                      |
| ---------------------------- | ------------ | --------------------------------------------------------- |
| `initialize`                 | client→agent | Negotiate protocol version and read agent capabilities    |
| `session/new`                | client→agent | Open a session with working directory and MCP server list |
| `session/prompt`             | client→agent | Submit one prepared turn                                  |
| `session/cancel`             | client→agent | Request cooperative cancellation of an in-flight turn     |
| `session/set_model`          | client→agent | Select a model when the agent advertises the capability   |
| `session/set_config_option`  | client→agent | Select a configuration option such as a reasoning effort  |
| `session/update`             | agent→client | Stream turn progress, tool calls and content chunks       |
| `session/request_permission` | agent→client | Ask the controlling side to approve or refuse an action   |

Capability negotiation is advisory in one direction only. When an agent does not
advertise a capability, a request for it is rejected locally with a reason
naming the absent capability rather than being sent and timing out. When an
agent advertises a capability the harness does not model, the harness ignores it
and records that the session is narrower than the agent.

## Port relationship

This chapter governs the ported, decomposed derivative of the upstream ACP
client. Upstream concentrates framing, correlation, session setup, model and
permission handling and legacy first-turn framing in one file. Here those
concerns are separate modules under the 300-counted-line budget, and
`docs/upstream-lock.md` records which upstream region each module came from so a
re-port can be compared region by region instead of file by file.
