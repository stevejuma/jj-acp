//! Model and configuration option selection.
//!
//! Adapters expose model choice through two surfaces: the stable
//! `configOptions` array and an unstable `models` object. Both are read here,
//! with the stable surface taking precedence. No identifier is hardcoded,
//! because adapters define their own option ids and their own model ids.

use serde_json::{Value, json};

/// The configuration category that carries model choices.
pub const MODEL_CATEGORY: &str = "model";

/// The configuration category that carries reasoning effort choices.
pub const THOUGHT_LEVEL_CATEGORY: &str = "thought_level";

/// Build `session/set_model` params.
pub fn set_model_params(session_id: &str, model_id: &str) -> Value {
    json!({ "sessionId": session_id, "modelId": model_id })
}

/// Build `session/set_config_option` params.
///
/// The wire key is always `configId` even though some adapters advertise their
/// option identifier as `id`.
pub fn set_config_option_params(session_id: &str, config_id: &str, value: &str) -> Value {
    json!({ "sessionId": session_id, "configId": config_id, "value": value })
}

/// Select the `configOptions` entries in a category.
pub fn options_in_category(result: &Value, category: &str) -> Vec<Value> {
    result
        .get("configOptions")
        .and_then(Value::as_array)
        .map(|options| {
            options
                .iter()
                .filter(|option| option.get("category").and_then(Value::as_str) == Some(category))
                .cloned()
                .collect()
        })
        .unwrap_or_default()
}

/// The model options an agent advertised at session creation.
pub fn model_options(result: &Value) -> Vec<Value> {
    options_in_category(result, MODEL_CATEGORY)
}

/// The unstable `models` object when the agent supplied one.
pub fn model_state(result: &Value) -> Option<Value> {
    result.get("models").cloned()
}

/// Read an option's own identifier, accepting either spelling.
///
/// The ACP schema uses `configId`; at least one shipping adapter uses `id`. Both
/// are accepted so the identifier is discovered from the session rather than
/// assumed per vendor.
pub fn option_id(option: &Value) -> Option<&str> {
    option
        .get("configId")
        .or_else(|| option.get("id"))
        .and_then(Value::as_str)
}

/// Discover the reasoning-effort option identifier an agent advertised.
pub fn thought_level_config_id(result: &Value) -> Option<String> {
    options_in_category(result, THOUGHT_LEVEL_CATEGORY)
        .iter()
        .find_map(|option| option_id(option))
        .map(str::to_owned)
}

/// How a model should be selected for a session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelSwitch {
    /// Use the stable `session/set_config_option` path.
    ConfigOption {
        /// The adapter-defined option identifier to set.
        config_id: String,
        /// The value within that option which selects the model.
        option_value: String,
    },
    /// Use the unstable `session/set_model` path.
    SetModel {
        /// The model identifier to request.
        model_id: String,
    },
}

/// Resolve how to select `model_id`, or `None` when the agent does not offer it.
///
/// The stable `configOptions` surface is searched first, matching a candidate
/// inside each option's nested `options` array rather than against the option's
/// own current value: the current value is what the session started with, not
/// the catalogue of what it could become. The unstable `models` object is the
/// fallback. The session result is always authoritative, so a model available in
/// an earlier session is never assumed available in this one.
pub fn resolve_model_switch(result: &Value, model_id: &str) -> Option<ModelSwitch> {
    for option in model_options(result) {
        let Some(config_id) = option_id(&option) else {
            continue;
        };
        let candidates = option.get("options").and_then(Value::as_array);
        let matched = candidates.is_some_and(|candidates| {
            candidates
                .iter()
                .any(|candidate| candidate.get("value").and_then(Value::as_str) == Some(model_id))
        });
        if matched {
            return Some(ModelSwitch::ConfigOption {
                config_id: config_id.to_owned(),
                option_value: model_id.to_owned(),
            });
        }
    }
    let available = model_state(result)?
        .get("availableModels")?
        .as_array()?
        .iter()
        .any(|model| model.get("modelId").and_then(Value::as_str) == Some(model_id));
    available.then_some(ModelSwitch::SetModel {
        model_id: model_id.to_owned(),
    })
}

/// Report whether a session result offers `model_id` at all.
pub fn offers_model(result: &Value, model_id: &str) -> bool {
    resolve_model_switch(result, model_id).is_some()
}

/// Report whether `model_id` appears in already-extracted catalogue halves.
///
/// This mirrors [`resolve_model_switch`] for a caller that cached the two
/// halves instead of retaining the whole session result, so a pre-turn guard can
/// check availability without re-parsing.
pub fn model_in_catalog(
    config_options: &[Value],
    available_models: Option<&Value>,
    model_id: &str,
) -> bool {
    let in_config_options = config_options.iter().any(|option| {
        option
            .get("options")
            .and_then(Value::as_array)
            .is_some_and(|candidates| {
                candidates.iter().any(|candidate| {
                    candidate.get("value").and_then(Value::as_str) == Some(model_id)
                })
            })
    });
    if in_config_options {
        return true;
    }
    available_models
        .and_then(|models| models.get("availableModels"))
        .and_then(Value::as_array)
        .is_some_and(|available| {
            available
                .iter()
                .any(|model| model.get("modelId").and_then(Value::as_str) == Some(model_id))
        })
}
