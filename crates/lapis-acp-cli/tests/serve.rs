//! The persistent control protocol, driven over the real binary's stdio.
//!
//! These tests are the controlling application. They write frames, read frames,
//! and assert that stdout never carries anything else — which is the property that
//! makes the protocol parseable rather than merely documented.

mod common;

use std::process::Stdio;
use std::time::Duration;

use common::{Scratch, task, write_catalog};
use lapis_acp_control::protocol::{
    CONTROL_VERSION, cancel_frame, runners_frame, shutdown_frame, submit_frame,
};
use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};

const READ_TIMEOUT: Duration = Duration::from_secs(45);

struct Server {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl Server {
    async fn start(scratch: &Scratch, mode: &str, extra: &[&str]) -> Self {
        let catalog = write_catalog(scratch, mode, &[]);
        let mut command = Command::new(common::binary());
        command
            .arg("serve")
            .arg("--catalog")
            .arg(&catalog)
            .arg("--runner")
            .arg("fixture")
            .arg("--idle-timeout")
            .arg("30")
            .arg("--max-turn-duration")
            .arg("60")
            .arg("--cancel-grace")
            .arg("2")
            .arg("--reap-timeout")
            .arg("2")
            .args(extra)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);
        let mut child = command.spawn().expect("serve starts");
        let stdin = child.stdin.take().expect("stdin");
        let stdout = BufReader::new(child.stdout.take().expect("stdout"));
        let mut server = Self {
            child,
            stdin,
            stdout,
        };
        // Every session begins with the ready frame, so consuming it here means a
        // test that reads next sees its own reply rather than the greeting.
        let ready = server.recv().await;
        assert_eq!(ready["type"], json!("ready"), "{ready}");
        assert_eq!(ready["version"], json!(CONTROL_VERSION));
        server
    }

    async fn send(&mut self, frame: Value) {
        let mut line = frame.to_string();
        line.push('\n');
        self.stdin.write_all(line.as_bytes()).await.expect("write");
        self.stdin.flush().await.expect("flush");
    }

    async fn recv(&mut self) -> Value {
        let mut line = String::new();
        let read = tokio::time::timeout(READ_TIMEOUT, self.stdout.read_line(&mut line))
            .await
            .expect("harness stopped answering")
            .expect("stdout read");
        assert!(read > 0, "the control stream closed unexpectedly");
        serde_json::from_str(line.trim())
            .unwrap_or_else(|error| panic!("stdout carried a non-frame line ({error}): {line:?}"))
    }

    /// Read frames until one of the wanted type arrives, collecting the rest.
    async fn recv_type(&mut self, wanted: &str) -> (Value, Vec<Value>) {
        let mut skipped = Vec::new();
        loop {
            let frame = self.recv().await;
            if frame["type"] == json!(wanted) {
                return (frame, skipped);
            }
            skipped.push(frame);
        }
    }

    async fn stop(self) -> (Option<i32>, String) {
        drop(self.stdin);
        let output = self.child.wait_with_output().await.expect("exit");
        (
            output.status.code(),
            String::from_utf8_lossy(&output.stderr).to_string(),
        )
    }
}

#[tokio::test]
async fn startup_announces_the_resolved_runners() {
    let scratch = Scratch::new("serve-ready");
    let catalog = write_catalog(&scratch, "normal", &[]);
    let mut command = Command::new(common::binary());
    command
        .args(["serve", "--catalog"])
        .arg(&catalog)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    let mut child = command.spawn().expect("serve starts");
    let mut stdout = BufReader::new(child.stdout.take().expect("stdout"));
    let mut line = String::new();
    tokio::time::timeout(READ_TIMEOUT, stdout.read_line(&mut line))
        .await
        .expect("ready frame")
        .expect("read");
    let ready: Value = serde_json::from_str(line.trim()).expect("json frame");
    assert_eq!(ready["type"], json!("ready"));
    assert_eq!(ready["version"], json!(1));
    let runners = ready["runners"].as_array().expect("runners");
    // Five compiled-in runners plus the catalog's fixture entry.
    assert_eq!(runners.len(), 6, "{runners:?}");
    let ids: Vec<&str> = runners
        .iter()
        .map(|runner| runner["id"].as_str().expect("id"))
        .collect();
    assert!(ids.contains(&"fixture"), "{ids:?}");
    assert!(ids.contains(&"cursor"), "{ids:?}");
    drop(stdout);
    child.kill().await.ok();
}

#[tokio::test]
async fn a_submitted_task_returns_exactly_one_record() {
    let scratch = Scratch::new("serve-submit");
    let mut server = Server::start(&scratch, "normal", &[]).await;
    server
        .send(submit_frame(
            "req-1",
            &task("task-serve-1", "do the work", 60_000),
        ))
        .await;
    let (record, skipped) = server.recv_type("record").await;
    assert_eq!(record["requestId"], json!("req-1"));
    assert_eq!(record["record"]["status"], json!("completed"));
    assert_eq!(record["record"]["taskId"], json!("task-serve-1"));
    assert_eq!(record["record"]["stopReason"], json!("end_turn"));
    assert_eq!(record["version"], json!(CONTROL_VERSION));
    // Diagnostics are frames too, so nothing else may share the stream.
    for frame in &skipped {
        assert_eq!(frame["type"], json!("diagnostic"), "{frame}");
    }
    server.send(shutdown_frame("req-2")).await;
    let (ack, _) = server.recv_type("ack").await;
    assert_eq!(ack["requestId"], json!("req-2"));
    let (code, stderr) = server.stop().await;
    assert_eq!(code, Some(0), "stderr: {stderr}");
}

#[tokio::test]
async fn a_second_submission_while_one_is_in_flight_is_refused() {
    let scratch = Scratch::new("serve-busy");
    let mut server = Server::start(&scratch, "turn-hang", &[]).await;
    server
        .send(submit_frame("req-a", &task("task-a", "hang", 60_000)))
        .await;
    // Give the first submission time to be registered as in flight.
    tokio::time::sleep(Duration::from_millis(1500)).await;
    server
        .send(submit_frame("req-b", &task("task-b", "hang", 60_000)))
        .await;
    let (error, _) = server.recv_type("error").await;
    assert_eq!(error["requestId"], json!("req-b"));
    assert_eq!(error["code"], json!("busy"));

    server.send(cancel_frame("req-c", "req-a")).await;
    let (ack, _) = server.recv_type("ack").await;
    assert_eq!(ack["requestId"], json!("req-c"));
    let (record, _) = server.recv_type("record").await;
    assert_eq!(record["requestId"], json!("req-a"));
    assert_eq!(record["record"]["status"], json!("cancelled"));
    let _ = server.stop().await;
}

#[tokio::test]
async fn cancelling_a_submission_that_is_not_in_flight_is_refused() {
    let scratch = Scratch::new("serve-cancel-unknown");
    let mut server = Server::start(&scratch, "normal", &[]).await;
    server.send(cancel_frame("req-x", "no-such-request")).await;
    let (error, _) = server.recv_type("error").await;
    assert_eq!(error["requestId"], json!("req-x"));
    assert_eq!(error["code"], json!("not_in_flight"));
    // The loop survives a refused cancel and still serves work.
    server
        .send(submit_frame("req-y", &task("task-y", "still here", 60_000)))
        .await;
    let (record, _) = server.recv_type("record").await;
    assert_eq!(record["record"]["status"], json!("completed"));
    let _ = server.stop().await;
}

#[tokio::test]
async fn a_runner_listing_is_answered_without_starting_an_agent() {
    let scratch = Scratch::new("serve-runners");
    let mut server = Server::start(&scratch, "normal", &[]).await;
    server.send(runners_frame("req-list")).await;
    let (listing, _) = server.recv_type("runners").await;
    assert_eq!(listing["requestId"], json!("req-list"));
    let runners = listing["runners"].as_array().expect("runners");
    assert!(runners.len() >= 6, "{runners:?}");
    assert!(
        runners
            .iter()
            .all(|runner| runner["available"].is_boolean()),
        "{runners:?}"
    );
    assert!(
        !scratch.log().exists(),
        "listing runners must not start an agent"
    );
    let _ = server.stop().await;
}

#[tokio::test]
async fn malformed_and_unsupported_frames_are_refused_and_the_loop_continues() {
    let scratch = Scratch::new("serve-bad-frames");
    let mut server = Server::start(&scratch, "normal", &[]).await;

    // A frame with no version at all is malformed rather than merely unsupported,
    // because there is nothing to negotiate against.
    server
        .send(json!({"type": "runners", "requestId": "no-version"}))
        .await;
    let (error, _) = server.recv_type("error").await;
    assert_eq!(error["code"], json!("invalid_frame"));
    assert_eq!(error["requestId"], json!("no-version"));

    server
        .send(json!({"version": 99, "type": "runners", "requestId": "wrong-version"}))
        .await;
    let (error, _) = server.recv_type("error").await;
    assert_eq!(error["code"], json!("unsupported_control_version"));
    assert_eq!(error["requestId"], json!("wrong-version"));

    server
        .send(json!({"version": 1, "type": "teleport", "requestId": "unknown-kind"}))
        .await;
    let (error, _) = server.recv_type("error").await;
    assert_eq!(error["code"], json!("unknown_request"));

    server
        .send(json!({"version": 1, "type": "cancel", "requestId": "no-target"}))
        .await;
    let (error, _) = server.recv_type("error").await;
    assert_eq!(error["code"], json!("invalid_request"));

    // Raw garbage that is not JSON at all is refused rather than crashing the loop.
    server
        .stdin
        .write_all(b"this is not json\n")
        .await
        .expect("write");
    server.stdin.flush().await.expect("flush");
    let (error, _) = server.recv_type("error").await;
    assert_eq!(error["code"], json!("invalid_frame"));

    // And the loop still serves real work afterwards.
    server.send(runners_frame("req-after")).await;
    let (listing, _) = server.recv_type("runners").await;
    assert_eq!(listing["requestId"], json!("req-after"));
    let _ = server.stop().await;
}

#[tokio::test]
async fn an_invalid_task_document_is_reported_as_a_record_not_an_error() {
    let scratch = Scratch::new("serve-bad-task");
    let mut server = Server::start(&scratch, "normal", &[]).await;
    // One shape for every task outcome, so a controlling application does not have
    // to handle a refusal and a failure as different frame types.
    let mut document = task("task-bad", "p", 60_000);
    document["cwd"] = json!("/etc");
    server.send(submit_frame("req-bad", &document)).await;
    let (record, _) = server.recv_type("record").await;
    assert_eq!(record["requestId"], json!("req-bad"));
    assert_eq!(record["record"]["status"], json!("invalid"));
    assert_eq!(record["record"]["error"], json!("invalid_task_json"));
    assert!(
        !scratch.log().exists(),
        "an invalid document must not start an agent"
    );
    let _ = server.stop().await;
}

#[tokio::test]
async fn end_of_input_ends_the_loop_cleanly() {
    let scratch = Scratch::new("serve-eof");
    let server = Server::start(&scratch, "normal", &[]).await;
    // Closing stdin is how a controlling application says it is finished.
    let (code, stderr) = server.stop().await;
    assert_eq!(code, Some(0), "stderr: {stderr}");
}
