//! Signal handling: bounded cleanup and the published signal exit codes.

mod common;

use std::time::Duration;

use common::{Scratch, run_args, task, write_catalog, write_task};
use serde_json::Value;
use tokio::process::Command;

/// Send a signal to a pid without depending on a signal crate.
fn signal(pid: u32, name: &str) {
    let status = std::process::Command::new("kill")
        .args([name, &pid.to_string()])
        .status()
        .expect("kill runs");
    assert!(status.success(), "could not signal {pid} with {name}");
}

/// Wait until the fixture has started, so the signal arrives mid-turn.
///
/// The bound is generous on purpose. Every test binary in the workspace spawns
/// interpreter subprocesses and `cargo test` runs them in parallel, so startup can
/// be far slower under load than in isolation. A short wait here would make this
/// test flaky for a reason that has nothing to do with signal handling, and a
/// flaky lifecycle test is worse than a slow one because it trains reviewers to
/// re-run instead of investigate.
fn await_agent_start(log: &std::path::Path) {
    const ATTEMPTS: u32 = 1200;
    const INTERVAL: Duration = Duration::from_millis(25);
    for attempt in 0..ATTEMPTS {
        if let Ok(text) = std::fs::read_to_string(log)
            && !text.trim().is_empty()
        {
            return;
        }
        std::thread::sleep(INTERVAL);
        if attempt + 1 == ATTEMPTS {
            panic!(
                "the fixture agent wrote nothing to {} within {:?}",
                log.display(),
                INTERVAL * ATTEMPTS
            );
        }
    }
}

async fn interrupted(signal_name: &str, expected: i32) {
    let scratch = Scratch::new("signal");
    let catalog = write_catalog(&scratch, "turn-hang", &[]);
    let path = write_task(&scratch, &task("task-signal", "never answer", 120_000));
    let mut arguments = run_args(&catalog, &path);
    // Long bounds, so only the signal can end this turn.
    let position = arguments
        .iter()
        .position(|item| item == "--idle-timeout")
        .expect("flag");
    arguments[position + 1] = "90".to_owned();
    arguments.extend(["--cancel-grace".to_owned(), "5".to_owned()]);
    arguments.extend(["--reap-timeout".to_owned(), "5".to_owned()]);

    let child = Command::new(common::binary())
        .args(&arguments)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .expect("binary spawns");
    let pid = child.id().expect("pid");
    await_agent_start(&scratch.log());

    signal(pid, signal_name);
    let output = child.wait_with_output().await.expect("binary exits");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    assert_eq!(
        output.status.code(),
        Some(expected),
        "{signal_name} should exit {expected}; stderr: {stderr}"
    );
    // Stdout still carries exactly one record after a signal, which is what lets
    // a controlling application parse the output of an interrupted run.
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 1, "stdout: {stdout:?}");
    let record: Value = serde_json::from_str(lines[0]).expect("json record");
    assert_eq!(record["status"], Value::String("cancelled".to_owned()));
    assert_eq!(record["taskId"], Value::String("task-signal".to_owned()));
    assert_eq!(record["error"], Value::String("interrupted".to_owned()));
    assert!(record.get("stopReason").is_none(), "{record}");
}

#[tokio::test]
async fn sigterm_produces_bounded_cleanup_and_exit_143() {
    interrupted("-TERM", 143).await;
}

#[tokio::test]
async fn sigint_produces_bounded_cleanup_and_exit_130() {
    interrupted("-INT", 130).await;
}
