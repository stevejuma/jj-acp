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
