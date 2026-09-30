//! Parsing payloads captured from a real installed adapter.
//!
//! The fixture is a verbatim `initialize` and `session/new` result from
//! `cursor-agent acp` version 2026.09.26-dd393fe, captured with the client
//! capabilities this crate actually advertises. It exists because a synthetic
//! payload only proves the parser agrees with itself: every assertion here was
//! first a question about what a shipping adapter really sends.

use lapis_acp_protocol::config_option::{self, ModelSwitch};
use lapis_acp_protocol::initialize;
use serde_json::Value;

fn captured() -> Value {
    serde_json::from_str(include_str!("fixtures/cursor-acp-session.json"))
        .expect("captured fixture parses")
}

#[test]
fn a_real_adapter_selects_protocol_version_one_and_is_accepted() {
    // The client asked for 2. The adapter chose 1, which is its right, and the
    // negotiated version is recorded rather than assumed.
    let result = initialize::parse_result(&captured()["initialize"]).expect("parses");
    assert_eq!(result.protocol_version, 1);
    assert_eq!(result.agent_name, None, "this adapter sends no agentInfo");
    assert!(result.advertises(&["loadSession"]));
    assert!(result.advertises(&["promptCapabilities", "image"]));
    assert!(!result.advertises(&["promptCapabilities", "audio"]));
    assert!(result.advertises(&["mcpCapabilities", "http"]));
    // MCP over stdio is not offered, so a caller must not hand it a stdio server.
    assert!(!result.advertises(&["mcpCapabilities", "stdio"]));
}

#[test]
fn a_real_adapter_names_its_option_id_rather_than_config_id() {
    let session = &captured()["sessionNew"];
    let options = config_option::model_options(session);
    assert_eq!(options.len(), 1);
    // The ACP schema says `configId`; this adapter sends `id`. Accepting only the
    // schema spelling would make every model here unselectable.
    assert!(options[0].get("configId").is_none());
    assert_eq!(options[0].get("id").and_then(Value::as_str), Some("model"));
    assert_eq!(
        config_option::resolve_model_switch(session, "default[]"),
        Some(ModelSwitch::ConfigOption {
            config_id: "model".to_owned(),
            option_value: "default[]".to_owned(),
        })
    );
}

#[test]
fn model_candidates_are_parameterized_and_matched_exactly() {
    let session = &captured()["sessionNew"];
    let parameterized = "grok-4.7[context=256k,reasoning_effort=high,fast=true]";
    assert_eq!(
        config_option::resolve_model_switch(session, parameterized),
        Some(ModelSwitch::ConfigOption {
            config_id: "model".to_owned(),
            option_value: parameterized.to_owned(),
        })
    );
    // The adapter's own current selection is also a selectable candidate.
    assert!(config_option::offers_model(
        session,
        "composer-2.5[fast=true]"
    ));
    // A bare name without its bracketed overrides is not a real candidate, and
    // guessing at one would send a value the adapter has never advertised.
    assert!(!config_option::offers_model(session, "grok-4.7"));
    assert!(!config_option::offers_model(session, "composer"));
    assert!(!config_option::offers_model(session, ""));
}

#[test]
fn the_stable_surface_wins_when_both_surfaces_offer_the_model() {
    let session = &captured()["sessionNew"];
    // This adapter populates configOptions and models with the same catalogue.
    assert!(config_option::model_state(session).is_some());
    let available = config_option::model_state(session).unwrap();
    assert_eq!(
        available["availableModels"].as_array().unwrap().len(),
        config_option::model_options(session)[0]["options"]
            .as_array()
            .unwrap()
            .len()
    );
    assert!(matches!(
        config_option::resolve_model_switch(session, "default[]"),
        Some(ModelSwitch::ConfigOption { .. })
    ));
    // The cached-catalogue mirror agrees with full resolution on real data.
    let options = config_option::model_options(session);
    let models = config_option::model_state(session);
    assert!(config_option::model_in_catalog(
        &options,
        models.as_ref(),
        "default[]"
    ));
    assert!(!config_option::model_in_catalog(
        &options,
        models.as_ref(),
        "grok-4.7"
    ));
}

#[test]
fn a_model_only_on_the_unstable_surface_falls_back_to_set_model() {
    let mut session = captured()["sessionNew"].clone();
    // Remove the stable surface so only `models` remains, which is what an
    // adapter that implements just the unstable path would send.
    session["configOptions"] = serde_json::json!([]);
    assert_eq!(
        config_option::resolve_model_switch(&session, "default[]"),
        Some(ModelSwitch::SetModel {
            model_id: "default[]".to_owned(),
        })
    );
}

#[test]
fn this_adapter_exposes_no_reasoning_effort_option_of_its_own() {
    let session = &captured()["sessionNew"];
    // Effort travels inside the parameterized model value here, so there is no
    // thought_level option to discover and nothing to set separately. A caller
    // that asked for an effort would be told the adapter does not offer one
    // rather than having a guessed identifier sent to it.
    assert_eq!(config_option::thought_level_config_id(session), None);
    assert!(config_option::options_in_category(session, "thought_level").is_empty());
    // It does expose a mode option, which this crate does not model.
    assert_eq!(
        config_option::options_in_category(session, "mode")[0]
            .get("id")
            .and_then(Value::as_str),
        Some("mode")
    );
}
