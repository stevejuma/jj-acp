# Controlling application boundary

`lapis-acp` has no message bus, no relay, no subscription and no author gate.
Work arrives from a controlling application that spawned the process, over
stdin, and results leave over stdout. There is no listener, no port, no token
and no authentication, because there is no peer: the parent process is the trust
boundary and it already decided to run this executable with this environment.

This chapter replaces the upstream relay intake entirely. The upstream author
gate, membership discovery, owner resolution and workflow-provenance checking
have no analogue here and are not partially reimplemented.

## Requirements

| ID           | Requirement                                                                                                                                                                                    |
| ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| LAC-CTRL-001 | The controlling application MUST drive `lapis-acp` over a versioned newline-delimited JSON control protocol with no authentication and no network listener.                                    |
| LAC-CTRL-002 | `lapis-acp` MUST accept one prepared task document of at most 1 MiB from a file or stdin and MUST reject unknown fields, duplicate fields, wrong types, trailing documents and malformed JSON. |
| LAC-CTRL-003 | Every attempted invocation MUST emit exactly one terminal record on stdout and MUST NOT include the prompt, credentials, agent final text or raw adapter errors.                               |
| LAC-CTRL-004 | Diagnostics and child stderr MUST stay on stderr so that stdout remains unconditionally parseable by the controlling application.                                                              |
| LAC-CTRL-005 | The task document MUST NOT change the working directory, executable, tools or credentials, which come only from trusted launch configuration.                                                  |
| LAC-CTRL-006 | A control frame MUST carry an explicit integer version, and an unsupported version MUST be rejected before any agent subprocess is started.                                                    |
| LAC-CTRL-007 | Reading a task document from a file or stdin MUST have its own bounded interval, separate from the execution deadline.                                                                         |

## Task document, version 1

One UTF-8 JSON object, at most 1 MiB including whitespace, supplied as a path or
as `-` for stdin. Stdin MUST be closed after the document.

```json
{
  "version": 1,
  "taskId": "example-001",
  "agent": "codex",
  "prompt": "Inspect the working copy and report findings.",
  "maxDurationMs": 120000
}
```

| Field           | Contract                                                                                                                 |
| --------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `version`       | Integer `1`. Any other version fails before agent launch.                                                                |
| `taskId`        | Nonblank string, 1 to 256 UTF-8 bytes, no control characters. Correlation only; not a deduplication key and not a claim. |
| `agent`         | Runner identifier that MUST resolve to an enabled catalog entry. This is a mis-execution guard, not an authorship check. |
| `prompt`        | Nonblank prepared task text. Context, evidence and reporting instructions belong here.                                   |
| `maxDurationMs` | Positive integer. The shorter of this value and the trusted absolute cap wins.                                           |

All fields are required. The document is executable task input rather than a
sandboxed message, so only supply tasks trusted with that runner's capabilities.

## Terminal record

One JSON line per attempted invocation. `taskId` is null until the document
validates. `status` is `completed`, `failed`, `invalid`, `timed_out` or
`cancelled`. `stopReason` appears only on normal completion. Failures carry a
stable sanitized `error` code such as `invalid_task_json`, `agent_spawn_failed`,
`runner_mismatch` or `configuration_timeout`.

```json
{
  "version": 1,
  "taskId": "example-001",
  "status": "completed",
  "stopReason": "end_turn"
}
```

A record is not a durable ledger. A killed harness or a broken stdout can leave
no record at all, and launching the same task twice can execute it twice.
Durability belongs to the controlling application.

## Transport pluggability

The control transport is a trait with one implementation in the first delivery:
newline-delimited JSON over the inherited stdin and stdout. A controlling
application that wants a Unix socket, a local HTTP endpoint or an in-process
link adds a transport rather than editing the runner. The trait boundary is the
reason the one-shot task runner and a future persistent service can share the
same session and turn machinery.

## Implemented surface

| Module       | Owns                                                                                  |
| ------------ | ------------------------------------------------------------------------------------- |
| `task`       | Document version 1, strict validation, the stable rejection code vocabulary           |
| `input`      | Bounded reads from a path or stdin, capped while reading rather than after            |
| `record`     | `TerminalRecord`, `Status`, `ExitCode` and the record-level outcome codes             |
| `invocation` | Resolving a runner, launching, executing one turn, stopping, and reducing to a record |
| `transport`  | The `ControlTransport` trait and the stdio implementation                             |

Rejection happens before any agent is spawned. A document that names an unknown
runner, a version this build does not accept, or a runner other than the one
this process was launched as never reaches a launch attempt, and the fixture log
stays empty for every refusal case.

## Reserved namespace

Only the task identity the harness injects is reserved, under the
`LAPIS_ACP_TASK_` prefix. An earlier design reserved the whole `LAPIS_ACP_` tree
and the fixture peer caught the consequence: it also stripped the launch options
an operator sets on purpose, such as `LAPIS_ACP_RUNNER` and `LAPIS_ACP_MODEL`,
breaking configuration in the name of protecting identity. Reserving narrowly is
still sufficient, because the only thing a runner definition must not be able to
forge is which task the harness says it is running.

## Exit code resolution

The invocation reports the exit code for every outcome except a signal-driven
cancellation, where it sets a flag instead. The CLI owns signal handling and
therefore owns the choice between `130` and `143`, because only it observed
which signal arrived. Keeping that decision out of the control crate is what
lets a controlling application drive the same invocation over a different
transport and apply its own interruption semantics.
