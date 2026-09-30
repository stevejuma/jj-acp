# Session and turn lifecycle

`lapis-acp-runner` executes prepared turns. It does not decide what to run, when
to run it, or what to do with the result; intake and disposition belong to
`lapis-acp-control` and to the controlling application above it.

A session scope is the unit of serialization. One scope has at most one turn in
flight. Distinct scopes may run concurrently when more than one agent subprocess
exists. This mirrors the upstream per-channel guarantee without inheriting its
channel concept: the controlling application names the scope, and `lapis-acp`
treats it as an opaque key.

## Requirements

| ID          | Requirement                                                                                                                                                              |
| ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| LAC-RUN-001 | The runner MUST execute one prepared turn per session scope with at most one turn in flight per scope.                                                                   |
| LAC-RUN-002 | Cancellation MUST be cooperative and bounded: request cancellation, wait a bounded grace period, then kill the process group and reap within a bounded interval.         |
| LAC-RUN-003 | An idle timeout reset by agent output and an absolute wall-clock turn deadline MUST both apply, and the shorter remaining bound MUST win.                                |
| LAC-RUN-004 | A timeout MUST be reported as an unknown outcome and MUST NOT be recorded as success or as a proven failure.                                                             |
| LAC-RUN-005 | A crashed agent subprocess MUST be reported as a distinct outcome, and a restart MUST NOT silently re-dispatch a turn that was already sent.                             |
| LAC-RUN-006 | Session creation MUST apply the configured model, effort and permission settings, and an unsupported setting MUST retain the adapter fallback rather than fail the turn. |
| LAC-RUN-007 | On Unix, cleanup MUST kill the process group so ordinary descendants cannot outlive a cancelled turn.                                                                    |

## Deadline composition

Two independent bounds apply to a turn. The idle timeout resets on any agent
stdout activity, so a long-running but visibly progressing turn is not killed.
The absolute turn duration never resets, so a turn that emits output forever
still ends. A task document may lower the absolute bound; it can never raise it
above the launch configuration cap, because the document is untrusted input and
the cap is trusted policy.

Deadline accounting covers adapter startup, session setup and the turn itself.
There is no separate unbounded startup phase. A launch configuration read has
its own short bound and reports a distinct outcome, because a blocked
configuration read must not be indistinguishable from a blocked agent.

## Cancellation and cleanup

Cancellation is a sequence, not an event. The runner requests cooperative
cancellation through the protocol, waits a bounded grace period for the agent to
return a stop reason, then kills the process group and waits a further bounded
interval to reap it. Each stage reports what actually happened.

Cleanup is best-effort and this is stated rather than hidden. A harness killed
with SIGKILL, or a machine that loses power, cannot run cleanup at all. A
descendant that deliberately escapes its process group is outside the guarantee.
An unreaped child is reported, not assumed stopped.

Descendant containment is a Unix property. On other platforms cleanup addresses
the direct child only, and the reduced guarantee is reported rather than
silently assumed to be equivalent.

## Recovery

A turn that ends without a stop reason is an unknown outcome. The runner records
what it observed and stops. It does not retry, because a retry can execute the
same side effects twice, and it does not fabricate a terminal state to make the
record tidier. Recovery from an unknown outcome is the controlling application's
decision, made with its own durable state.

## Implemented surface

| Module    | Owns                                                                               |
| --------- | ---------------------------------------------------------------------------------- |
| `launch`  | `LaunchSpec`, spawning with piped stdio, inherited stderr and a Unix process group |
| `process` | `ProcessGuard`, bounded grace, process-group kill, bounded reap, `Termination`     |
| `turn`    | `Agent`, `TurnRequest`, and `run_turn` for one negotiated, prompted, stopped turn  |
| `outcome` | `TurnOutcome`, `TurnStatus`, `Failure` and the protocol error classification       |

The child's stderr is inherited rather than piped, so agent diagnostics reach
the operator directly and can never be mistaken for protocol frames on stdout.

`Failure` carries a classified `kind` and an untrusted `detail`. Only the kind
and the agent's numeric error code may be published; the detail is diagnostic
text for stderr and can quote prompt content or a credential the agent was
given. The split is structural, so sanitization cannot be forgotten by a later
caller.

A timeout is classified by which bound elapsed, because the two mean different
things. An idle timeout says the agent went silent. An absolute timeout says the
cap was reached, possibly while the agent was still visibly working. Both are
unknown outcomes, and neither is retried.

## The descendant hazard

Killing the agent's direct child does not stop a child the agent spawned. The
fixture peer's descendant mode proves this rather than asserting it: after the
direct child is killed the recorded descendant pid is still alive. That is why
`ProcessGuard` kills the process group on Unix, and why `Termination` reports
whether the stop was confirmed instead of assuming it.

`Termination::confirmed_stopped` is false when the reap bound elapsed first. A
caller that needs certainty must treat that as an unknown outcome and report it,
which is what the control layer does through its diagnostics rather than through
the terminal record.

## Orderly shutdown before a kill

Termination is the fallback, not the first move. When a turn ends the agent's
stdin is released before the grace window starts, because closing a pipe write
end is what produces the end of input a well-behaved agent waits for. Flushing
is not sufficient: an async shutdown on a child's stdin leaves the descriptor
open, so an agent blocked reading it never learns the exchange is over.

The observable difference is that a completed turn now stops its agent without a
kill. That matters for a real adapter that flushes session state on exit, and it
removes a misleading diagnostic from the normal path: a group kill is reported
because it is worth knowing about, and reporting it after every successful task
would train an operator to ignore it.

Releasing the connection also means the process guard is the only remaining
owner, so the cleanup sequence cannot race against a further protocol write.
