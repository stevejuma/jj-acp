# CLI and exit codes

The `lapis-acp` binary is the only executable in this repository. It composes
the crates and owns nothing else: argument parsing, launch configuration
assembly, signal handling and the process exit code.

The first delivery is the one-shot prepared task runner. A persistent service
mode is specified but not implemented, and the ledger records it as deferred
rather than implying parity.

## Requirements

| ID          | Requirement                                                                                                                                                                                             |
| ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| LAC-CLI-001 | The binary MUST expose `run --task <path\|->` executing exactly one prepared task in one fresh agent process and session, then exit.                                                                    |
| LAC-CLI-002 | Exit codes MUST be `0` normal completion, `1` startup, execution or output failure, `2` invalid arguments, launch configuration or task input, `124` deadline exceeded, `130` SIGINT and `143` SIGTERM. |
| LAC-CLI-003 | SIGINT and SIGTERM MUST trigger bounded cleanup before exit and MUST NOT leave an unreaped child when cleanup succeeds.                                                                                 |
| LAC-CLI-004 | Every launch option MUST be settable by a command-line flag and by a matching environment variable, with the flag taking precedence.                                                                    |
| LAC-CLI-005 | The binary MUST NOT write diagnostics, prompts or agent output to stdout on any path, including startup failure and signal handling.                                                                    |
| LAC-CLI-006 | A task run MUST NOT start a subscription, a standing pool, a heartbeat or any other service facility, and inherited service-only options MUST NOT enable one.                                           |

## Commands

| Command                     | Role                                                      | Status   |
| --------------------------- | --------------------------------------------------------- | -------- |
| `lapis-acp run --task -`    | Execute one prepared task document read from stdin        | Planned  |
| `lapis-acp run --task PATH` | Execute one prepared task document read from a file       | Planned  |
| `lapis-acp agents`          | List resolved runners with availability and install hints | Planned  |
| `lapis-acp serve`           | Persistent control transport with a standing session pool | Deferred |

Exit code `0` means the turn returned normally. It is not proof that the task
succeeded; the caller MUST inspect `stopReason`. A refusal is a normal
completion with `stopReason` set to `refusal`, which is why the two are separate
signals.

## Working directory and inheritance

The working directory is the launch working directory. A task document cannot
change it, and no option in the document is consulted for it. This keeps the
untrusted input away from the one decision that determines which files an agent
can touch.

The inherited environment is preserved and extended, never replaced. Existing
variables that the harness needs to control are overridden deliberately and the
override is recorded; everything else passes through so a controlling
application can configure provider credentials the way it already does.

## Implemented surface

| Module     | Owns                                                                          |
| ---------- | ----------------------------------------------------------------------------- |
| `args`     | Flag and environment parsing, defaults, usage text, refusals                  |
| `run_task` | Catalog assembly, prompt file read, launch configuration, one invocation      |
| `agents`   | One JSON object per line describing each resolved runner and its availability |
| `signals`  | SIGINT and SIGTERM installation and the cancellation watch                    |
| `lib`      | Dispatch, runtime construction, emitting one record, resolving the exit code  |

Every launch option has a flag and a matching `LAPIS_ACP_*` variable, and the
flag wins. Defaults for the idle window and the absolute cap are 620 and 7200
seconds, inherited from the upstream harness; the cancellation grace and reap
bounds default to five seconds each.

Installing a signal handler replaces the default terminate-immediately
behaviour, so an interruption now produces bounded cleanup and a record instead
of an orphaned agent and silence. A failure to install is reported rather than
ignored, because running without cancellation handling would silently weaken
that guarantee.

## Stdout contract

Stdout carries terminal records, and for `--help`, `--version` and `agents` the
output those commands exist to produce. Nothing else is ever written there.
Usage for a malformed command line goes to stderr, so a controlling application
that mis-invokes the binary still sees empty stdout rather than prose it cannot
parse.

A record is emitted on every path that reaches an invocation, including a spawn
failure and an agent error. A broken stdout is the one case with no record, and
it exits `1`; the record is not a durable ledger and a killed harness can leave
none.

## Failure classification

The exit code distinguishes causes a controlling application must treat
differently. `1` with `agent_exited` means the agent is gone and a new one can
safely be started. `124` means a bound elapsed and the agent's state is unknown,
so starting another one risks two agents working the same task. `2` means
nothing was started at all, so the same document can be corrected and
resubmitted.

`130` and `143` mean the run was interrupted and bounded cleanup ran. Cleanup
being bounded is not the same as cleanup being complete, and the record says
`cancelled` with `interrupted` rather than claiming the agent stopped.

## Subcommand option handling

`agents` parses the same launch options as `run`, because it inspects a catalog.
Building it from defaults instead reported a smaller runner set than the
operator configured and looked like a correct answer, which is the worst shape
for this class of bug: a catalog file that silently does nothing.

Arguments that no subcommand would read are refused rather than ignored.
`--help --catalog x` is a mistake, and accepting it teaches an operator that
unparsed flags are harmless.

## Session configuration flags

`--model`, `--effort` and `--mode` request session settings, each with a
matching `LAPIS_ACP_MODEL`, `LAPIS_ACP_EFFORT` and `LAPIS_ACP_MODE` variable and
the flag winning. All three are requests, not requirements: a value the adapter
does not offer is reported on stderr and the adapter's own default is kept, and
the run still completes normally with exit `0`.

That asymmetry is intentional. Failing a task because an adapter lacks a
preference would make the flags dangerous to pass, and a controlling application
that targets several adapters could not set a preference at all. The record
schema is unchanged by configuration, so shortfalls travel on stderr where an
operator reads them and never in the record a machine parses.

## The serve command

`lapis-acp serve` parses the same launch options as `run` and refuses `--task`,
because a task arrives per control frame and a launch-time document would be
meaningless. Sharing the option parser is what stops a flag from meaning one
thing in `run` and something else in `serve`; the trusted launch configuration
is built by one function both commands call.

A signal ends the loop rather than cancelling one task. A controlling
application that has stopped reading has nobody to receive a record, so the
honest outcome is a bounded drain followed by the signal's own exit code.
Reaching end of input is a clean exit `0`, because a closed stdin is how a
controlling application says it is finished.

Unusable launch configuration exits `2` with the reason on stderr and nothing on
stdout. There is no loop to serve and no peer to frame a refusal for, so the
exit code is the only machine-readable signal available.
