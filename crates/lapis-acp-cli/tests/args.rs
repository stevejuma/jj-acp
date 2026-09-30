//! Argument and environment parsing for the `run` command.

use std::time::Duration;

use lapis_acp_cli::args::{
    Command, DEFAULT_CANCEL_GRACE, DEFAULT_IDLE_TIMEOUT, DEFAULT_MAX_TURN_DURATION,
    DEFAULT_REAP_TIMEOUT, LaunchOptions, parse,
};
use lapis_acp_control::TaskSource;
use lapis_acp_protocol::session::SystemPromptTransport;
use std::collections::BTreeMap;

/// Build an environment lookup from fixed pairs.
fn env(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
    let map: BTreeMap<String, String> = pairs
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect();
    move |name: &str| map.get(name).cloned()
}

fn argv(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| item.to_string()).collect()
}

fn run_options(command: Command) -> LaunchOptions {
    match command {
        Command::Run(command) => command.launch,
        other => panic!("expected a run command, got {other:?}"),
    }
}

#[test]
fn defaults_match_the_documented_upstream_bounds() {
    let options = run_options(parse(argv(&["run", "--task", "-"]), &env(&[])).expect("parse"));
    assert_eq!(options.idle_timeout, DEFAULT_IDLE_TIMEOUT);
    assert_eq!(options.idle_timeout, Duration::from_secs(620));
    assert_eq!(options.max_turn_duration, DEFAULT_MAX_TURN_DURATION);
    assert_eq!(options.max_turn_duration, Duration::from_secs(7200));
    assert_eq!(options.cancel_grace, DEFAULT_CANCEL_GRACE);
    assert_eq!(options.reap_timeout, DEFAULT_REAP_TIMEOUT);
    assert_eq!(options.prompt_transport, SystemPromptTransport::Field);
    assert!(options.runner.is_none());
    assert!(options.cwd.is_none());
}

#[test]
fn every_option_is_settable_by_environment() {
    let options = run_options(
        parse(
            argv(&["run", "--task", "-"]),
            &env(&[
                ("LAPIS_ACP_RUNNER", "codex"),
                ("LAPIS_ACP_CATALOG", "/etc/runners.json"),
                ("LAPIS_ACP_MODEL", "chosen"),
                ("LAPIS_ACP_IDLE_TIMEOUT", "30"),
                ("LAPIS_ACP_MAX_TURN_DURATION", "90"),
                ("LAPIS_ACP_CANCEL_GRACE", "3"),
                ("LAPIS_ACP_REAP_TIMEOUT", "4"),
                ("LAPIS_ACP_SYSTEM_PROMPT_FILE", "/etc/prompt.md"),
                ("LAPIS_ACP_PROMPT_TRANSPORT", "meta-append"),
                ("LAPIS_ACP_CWD", "/work"),
            ]),
        )
        .expect("parse"),
    );
    assert_eq!(options.runner.as_deref(), Some("codex"));
    assert_eq!(options.catalog.as_deref(), Some("/etc/runners.json"));
    assert_eq!(options.model.as_deref(), Some("chosen"));
    assert_eq!(options.idle_timeout, Duration::from_secs(30));
    assert_eq!(options.max_turn_duration, Duration::from_secs(90));
    assert_eq!(options.cancel_grace, Duration::from_secs(3));
    assert_eq!(options.reap_timeout, Duration::from_secs(4));
    assert_eq!(
        options.system_prompt_file.as_deref(),
        Some("/etc/prompt.md")
    );
    assert_eq!(options.prompt_transport, SystemPromptTransport::MetaAppend);
    assert_eq!(options.cwd.as_deref(), Some(std::path::Path::new("/work")));
}

#[test]
fn a_flag_overrides_the_matching_environment_variable() {
    let options = run_options(
        parse(
            argv(&[
                "run",
                "--task",
                "/tmp/task.json",
                "--runner",
                "from-flag",
                "--idle-timeout",
                "11",
                "--prompt-transport",
                "meta",
            ]),
            &env(&[
                ("LAPIS_ACP_RUNNER", "from-env"),
                ("LAPIS_ACP_IDLE_TIMEOUT", "99"),
                ("LAPIS_ACP_PROMPT_TRANSPORT", "meta-append"),
            ]),
        )
        .expect("parse"),
    );
    assert_eq!(options.runner.as_deref(), Some("from-flag"));
    assert_eq!(options.idle_timeout, Duration::from_secs(11));
    assert_eq!(options.prompt_transport, SystemPromptTransport::Meta);
    // Untouched options still come from the environment.
    assert_eq!(options.model, None);
}

#[test]
fn the_task_source_accepts_a_path_and_the_stdin_spelling() {
    match parse(argv(&["run", "--task", "-"]), &env(&[])).expect("parse") {
        Command::Run(command) => assert_eq!(command.task, TaskSource::Stdin),
        other => panic!("expected a run command, got {other:?}"),
    }
    match parse(argv(&["run", "--task", "/tmp/t.json"]), &env(&[])).expect("parse") {
        Command::Run(command) => {
            assert_eq!(command.task, TaskSource::Path("/tmp/t.json".to_owned()))
        }
        other => panic!("expected a run command, got {other:?}"),
    }
    match parse(argv(&["run", "--task=/tmp/t.json"]), &env(&[])).expect("parse") {
        Command::Run(command) => {
            assert_eq!(command.task, TaskSource::Path("/tmp/t.json".to_owned()))
        }
        other => panic!("expected a run command, got {other:?}"),
    }
    assert_eq!(TaskSource::parse("-"), TaskSource::Stdin);
}

#[test]
fn subcommands_that_take_no_task_are_recognised() {
    assert_eq!(parse(argv(&[]), &env(&[])).expect("parse"), Command::Help);
    assert_eq!(
        parse(argv(&["--help"]), &env(&[])).expect("parse"),
        Command::Help
    );
    assert_eq!(
        parse(argv(&["--version"]), &env(&[])).expect("parse"),
        Command::Version
    );
    assert_eq!(
        parse(argv(&["agents"]), &env(&[])).expect("parse"),
        Command::Agents
    );
}

#[test]
fn malformed_command_lines_are_refused_before_any_work() {
    let cases: Vec<(&[&str], &str)> = vec![
        (&["bogus"], "unknown command"),
        (&["run"], "needs --task"),
        (&["run", "--task"], "needs a value"),
        (&["run", "--task", "-", "--nope"], "unknown option"),
        (
            &["run", "--task", "-", "--idle-timeout", "abc"],
            "whole number",
        ),
        (
            &["run", "--task", "-", "--idle-timeout", "0"],
            "must be positive",
        ),
        (
            &["run", "--task", "-", "--prompt-transport", "sideways"],
            "meta-append",
        ),
    ];
    for (arguments, expected) in cases {
        let error = parse(argv(arguments), &env(&[])).expect_err("should refuse");
        assert!(
            error.to_string().contains(expected),
            "{arguments:?} produced {error}, expected {expected}"
        );
    }
}

#[test]
fn an_unusable_environment_value_is_refused_like_a_bad_flag() {
    let error = parse(
        argv(&["run", "--task", "-"]),
        &env(&[("LAPIS_ACP_MAX_TURN_DURATION", "soon")]),
    )
    .expect_err("should refuse");
    assert!(error.to_string().contains("whole number"), "{error}");
}
