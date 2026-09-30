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

| ID            | Requirement                                                                                                                                                                                       |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| LAC-PROTO-001 | The protocol crate MUST encode and decode ACP JSON-RPC frames over newline-delimited stdio with an explicit per-frame byte cap and MUST fail closed on overflow.                                  |
| LAC-PROTO-002 | The client MUST request protocol version 2 at `initialize` and MUST fall back to version 1 when the agent reports it, recording the negotiated version.                                           |
| LAC-PROTO-003 | The client MUST support `session/new`, `session/prompt`, `session/cancel`, `session/set_model`, `session/set_config_option`, inbound `session/request_permission` and `session/update`.           |
| LAC-PROTO-004 | A turn MUST terminate in exactly one stop reason drawn from `end_turn`, `max_tokens`, `max_turn_requests`, `refusal` and `cancelled`.                                                             |
| LAC-PROTO-005 | An agent error MUST be surfaced as a typed protocol error preserving the agent's numeric code, and MUST substitute the whole error object when no message is present so provider detail survives. |
| LAC-PROTO-006 | The protocol crate MUST remain free of process spawning, scheduling, credential handling and transport policy.                                                                                    |

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
permission handling and legacy first-turn framing in one 215 KB file. Here those
concerns are twelve modules, the largest of which is 228 counted lines, and
`docs/upstream-lock.md` records which upstream region each came from so a
re-port can be compared region by region instead of file by file.

## Implemented surface

| Module          | Owns                                                                                       |
| --------------- | ------------------------------------------------------------------------------------------ |
| `frame`         | Bounded newline-delimited reader and writer, skip counting                                 |
| `envelope`      | Request, notification and response construction, inbound classification, id matching       |
| `error`         | Typed wire failures and agent error extraction                                             |
| `budget`        | Connection limits and the two-bound turn budget                                            |
| `initialize`    | Client identity, advertised client capabilities, version negotiation                       |
| `content`       | Text content blocks and MCP server declarations                                            |
| `session`       | `session/new`, `session/prompt` and `session/cancel` payloads and stop reason parsing      |
| `config_option` | Model and reasoning-effort discovery and selection                                         |
| `update`        | `session/update` classification with raw pass-through                                      |
| `permission`    | `session/request_permission` normalization and response construction                       |
| `handler`       | The inbound-traffic hook and a recording default                                           |
| `client`        | The connection: correlation, budgets and servicing agent traffic while awaiting a response |

`Connection` is generic over a reader and a writer rather than owning a child
process, which is what allows the whole surface to be tested over an in-memory
pipe. Process ownership, respawn and cleanup belong to
[session and turn lifecycle](runner.md).

A turn budget carries an idle bound that resets on every frame and an absolute
bound that never resets; the shorter remaining bound always wins. Both are
clamped to a nonzero floor so a zero value cannot produce a deadline that has
already passed and a confusing immediate timeout.

## Deliberate divergences

Three behaviours differ from upstream on purpose, and each is a boundary this
repository holds rather than an oversight in the port.

Approval is not automatic. Upstream answered every `session/request_permission`
by selecting the option whose kind is `allow_once`, falling back to
`reject_once`, because it ran unattended in an auto mode. Here the default
answer is cancellation and a caller that wants unattended approval must choose
it explicitly, select by semantic kind rather than by identifier, and record
that it did so. An unanswered approval request must never silently become an
approval.

Client capabilities are advertised as absent. This crate implements no
`fs/read_text_file`, `fs/write_text_file` or `terminal/*` method, so it
advertises all three as false. Upstream also advertised terminal-native
authentication and vendor-specific notification extensions; both are omitted
because this harness holds no credentials and publishes no telemetry, and
advertising a facility that does not exist would only produce requests that
fail.

Skipped frames are counted rather than only logged. A blank line is a frame
separator and is not counted; an unparseable line is counted and exposed through
`skipped_frames`, so a caller can distinguish an agent that emits stray
diagnostics from one that emits nothing but noise. Skipping is tolerance, not
acceptance: a stream of only garbage yields no frames and eventually trips the
idle bound.
