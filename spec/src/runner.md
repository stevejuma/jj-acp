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
