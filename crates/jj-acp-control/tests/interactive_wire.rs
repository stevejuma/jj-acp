//! Untrusted control input cannot broaden launch authority or lose partial frames.
use jj_acp_control::interactive_wire::parse;
use jj_acp_control::{FrameSource, LineSource};
use tokio::io::{AsyncWriteExt, BufReader, duplex};
#[test]
fn launch_paths_and_unadvertised_fields_are_refused() {
    for key in ["cwd", "executable", "env", "tools"] {
        let text = format!(r#"{{"version":2,"type":"open","requestId":"one","{key}":"unsafe"}}"#);
        assert_eq!(parse(&text).unwrap_err(), "invalid_request");
    }
    assert_eq!(
        parse(r#"{"version":3,"type":"open","requestId":"one"}"#).unwrap_err(),
        "unsupported_control_version"
    );
}
#[tokio::test]
async fn partial_frames_survive_cancelled_reads() {
    let (mut input, output) = duplex(256);
    let mut source = LineSource::new(BufReader::new(output));
    input.write_all(b"partial").await.unwrap();
    assert!(
        tokio::time::timeout(std::time::Duration::from_millis(10), source.next_line())
            .await
            .is_err()
    );
    input.write_all(b" frame\n").await.unwrap();
    assert_eq!(
        source.next_line().await.unwrap().unwrap(),
        "partial frame\n"
    );
}
#[tokio::test]
async fn endless_control_input_is_bounded_before_a_newline() {
    let (mut input, output) = duplex(8192);
    let mut source = LineSource::new(BufReader::new(output));
    let writer = tokio::spawn(async move {
        let _ = input.write_all(&vec![b'x'; 2_097_153]).await;
    });
    assert!(source.next_line().await.is_err());
    writer.abort();
}
