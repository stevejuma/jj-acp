//! Parsing a catalog document.
//!
//! A catalog document is data. Parsing it cannot execute anything, and an invalid
//! entry is reported with the identifier it failed on rather than being skipped
//! silently, because a quietly smaller runner set is worse than a loud failure.

use lapis_acp_protocol::EnvVar;
use serde_json::Value;

use crate::definition::{CommandSpec, McpTransport, RunnerDefinition};
use crate::error::{Error, Result};

/// The array member holding runner declarations.
pub const RUNNERS_KEY: &str = "runners";

fn string_field(value: &Value, key: &str) -> Option<String> {
    value.get(key).and_then(Value::as_str).map(str::to_owned)
}

/// Parse one argv, which may be a bare command or a complete argument vector.
fn command_spec(value: &Value, id: &str) -> Result<CommandSpec> {
    match value {
        Value::String(command) => Ok(CommandSpec::new(command.clone())),
        Value::Array(parts) => {
            let mut parts = parts.iter().map(|part| {
                part.as_str()
                    .map(str::to_owned)
                    .ok_or_else(|| Error::Parse(format!("{id}: command argv must be strings")))
            });
            let command = parts
                .next()
                .ok_or_else(|| Error::Parse(format!("{id}: command argv is empty")))??;
            let args = parts.collect::<Result<Vec<String>>>()?;
            Ok(CommandSpec { command, args })
        }
        _ => Err(Error::Parse(format!(
            "{id}: command must be a string or an array of strings"
        ))),
    }
}

fn arguments(value: &Value, id: &str) -> Result<Vec<String>> {
    let Some(args) = value.get("args") else {
        return Ok(Vec::new());
    };
    let Value::Array(args) = args else {
        return Err(Error::Parse(format!("{id}: args must be an array")));
    };
    args.iter()
        .map(|arg| {
            arg.as_str()
                .map(str::to_owned)
                .ok_or_else(|| Error::Parse(format!("{id}: args must contain only strings")))
        })
        .collect()
}

fn environment(value: &Value, id: &str) -> Result<Vec<EnvVar>> {
    let Some(env) = value.get("env") else {
        return Ok(Vec::new());
    };
    let Value::Object(env) = env else {
        return Err(Error::Parse(format!("{id}: env must be an object")));
    };
    let mut variables = Vec::new();
    for (name, value) in env {
        let value = value
            .as_str()
            .ok_or_else(|| Error::Parse(format!("{id}: env entry {name} must be a string")))?;
        variables.push(EnvVar::new(name.clone(), value.to_owned()));
    }
    variables.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(variables)
}

/// Parse one runner declaration.
pub fn parse_definition(value: &Value) -> Result<RunnerDefinition> {
    let id = string_field(value, "id")
        .ok_or_else(|| Error::Parse("a runner declaration needs a string id".to_owned()))?;
    let label = string_field(value, "label")
        .ok_or_else(|| Error::Parse(format!("{id}: label must be a string")))?;
    let command_value = value
        .get("command")
        .ok_or_else(|| Error::Parse(format!("{id}: command is required")))?;
    let mut commands = vec![command_spec(command_value, &id)?];
    if let Some(alternates) = value.get("alternateCommands") {
        let Value::Array(alternates) = alternates else {
            return Err(Error::Parse(format!(
                "{id}: alternateCommands must be an array"
            )));
        };
        for alternate in alternates {
            commands.push(command_spec(alternate, &id)?);
        }
    }
    if commands.len() == 1 {
        let args = arguments(value, &id)?;
        if !args.is_empty() {
            let command = commands[0].command.clone();
            commands[0] = CommandSpec { command, args };
        }
    } else if value.get("args").is_some() {
        return Err(Error::Parse(format!(
            "{id}: args cannot be combined with alternateCommands"
        )));
    }
    let transport = match string_field(value, "mcpTransport") {
        None => McpTransport::Stdio,
        Some(value) => {
            McpTransport::parse(&value).ok_or_else(|| Error::InvalidTransport(id.clone()))?
        }
    };
    let mut definition = RunnerDefinition::try_from_parts(
        id.clone(),
        label,
        commands,
        environment(value, &id)?,
        transport,
    )?;
    if value.get("enabled") == Some(&Value::Bool(false)) {
        definition = definition.disabled();
    }
    // Each is honoured on its own: a definition with a hint and no link, or a
    // link and no hint, is still useful guidance for an operator.
    definition.install_hint = string_field(value, "installHint").or(definition.install_hint);
    definition.install_instructions_url =
        string_field(value, "installInstructionsUrl").or(definition.install_instructions_url);
    Ok(definition)
}

/// Parse a complete catalog document.
pub fn parse_catalog(value: &Value) -> Result<Vec<RunnerDefinition>> {
    let runners = value
        .get(RUNNERS_KEY)
        .ok_or_else(|| Error::Parse(format!("{RUNNERS_KEY} array is required")))?;
    let Value::Array(runners) = runners else {
        return Err(Error::Parse(format!("{RUNNERS_KEY} must be an array")));
    };
    if runners.is_empty() {
        return Err(Error::Parse(format!(
            "{RUNNERS_KEY} must declare at least one runner"
        )));
    }
    runners.iter().map(parse_definition).collect()
}

/// Parse a catalog document from its JSON text.
pub fn parse_catalog_text(text: &str) -> Result<Vec<RunnerDefinition>> {
    let value: Value =
        serde_json::from_str(text).map_err(|err| Error::Parse(format!("invalid JSON: {err}")))?;
    parse_catalog(&value)
}
