//! `lapis-acp` command line entry points.
//!
//! Owns argument parsing, launch configuration assembly, the one-shot
//! `run --task` invocation and process exit codes. Composition only: behaviour
//! lives in the owning crate, never here.
//!
//! Exit codes are a published contract and MUST NOT change meaning:
//! `0` normal completion, `1` startup/execution/output failure, `2` invalid
//! arguments, launch configuration or task input, `124` deadline exceeded,
//! `130` SIGINT and `143` SIGTERM after bounded cleanup.

pub fn run() -> i32 {
    // Slice 1 delivers governance and the crate skeleton only. The task runner
    // entry point lands with LAC-CTRL-002 and LAC-CLI-001.
    2
}
