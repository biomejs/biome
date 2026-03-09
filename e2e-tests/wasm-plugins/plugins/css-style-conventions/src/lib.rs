//! WASM plugin: css-style-conventions
//!
//! Multi-rule plugin for CSS custom property conventions.
//!
//! ## Rules
//!
//! **`customPropertyPattern`** — Checks that CSS custom property names (e.g.
//! `--my-color`) match a configurable regex pattern. Defaults to kebab-case:
//! `^--[a-z][a-z0-9]*(-[a-z0-9]+)*$`.
//!
//! Options: `{ "pattern": "^--my-prefix-" }`
//!
//! **`noImportantInCustomProperties`** — Disallows `!important` on custom
//! property declarations.

use biome_plugin_sdk::css_kinds;

biome_plugin_sdk::generate_plugin!();

use biome::plugin::host;
use biome::plugin::types::Severity;

use std::cell::RefCell;

/// Default regex: kebab-case after the `--` prefix.
const DEFAULT_PATTERN: &str = r"^--[a-z][a-z0-9]*(-[a-z0-9]+)*$";

// Configured pattern for `customPropertyPattern`.
// Uses `RefCell` so `configure()` can update it (WASM is single-threaded).
thread_local! {
    static PATTERN: RefCell<Option<String>> = const { RefCell::new(None) };
}

fn get_pattern() -> String {
    PATTERN.with(|p| {
        p.borrow()
            .as_deref()
            .unwrap_or(DEFAULT_PATTERN)
            .to_string()
    })
}

/// Given a `CSS_DECLARATION` node, extract the custom property name if the
/// declaration uses a dashed identifier (`--*`).
///
/// Returns `None` when the declaration is for a regular property.
fn get_custom_property_name(node: u32) -> Option<String> {
    // CSS_DECLARATION children (non-token): [property, ?important]
    // property is CSS_GENERIC_PROPERTY whose first child is the name node.
    let property = host::node_child_by_kind(node, css_kinds::CSS_GENERIC_PROPERTY)?;
    let name = host::node_child_by_kind(property, css_kinds::CSS_DASHED_IDENTIFIER)?;
    let text = host::node_trimmed_text(name);
    if text.starts_with("--") {
        Some(text)
    } else {
        None
    }
}

struct CssStyleConventions;

impl Guest for CssStyleConventions {
    fn target_language() -> String {
        "css".into()
    }

    fn rule_names() -> Vec<String> {
        vec![
            "customPropertyPattern".into(),
            "noImportantInCustomProperties".into(),
        ]
    }

    fn query_kinds_for_rule(_rule: String) -> Vec<u32> {
        // Both rules operate on CSS_DECLARATION nodes.
        vec![css_kinds::CSS_DECLARATION]
    }

    fn source_triggers_for_rule(_rule: String) -> Vec<String> {
        vec![]
    }

    fn configure(rule: String, options_json: String) {
        if rule == "customPropertyPattern" {
            if let Some(pattern) = biome_plugin_sdk::options::get_string(&options_json, "pattern") {
                PATTERN.with(|p| *p.borrow_mut() = Some(pattern));
            }
        }
    }

    fn rule_metadata(_rule: String) -> RuleMetadata {
        RuleMetadata {
            version: "0.0.0".into(),
            sources: vec![],
            recommended: false,
            fix_kind: None,
            category: None,
            domains: vec![],
            deprecated: None,
            severity: None,
            issue_number: None,
        }
    }

    fn check(node: u32, rule: String) -> Vec<CheckResult> {
        match rule.as_str() {
            "customPropertyPattern" => check_custom_property_pattern(node),
            "noImportantInCustomProperties" => check_no_important_in_custom(node),
            _ => vec![],
        }
    }
}

fn check_custom_property_pattern(node: u32) -> Vec<CheckResult> {
    let Some(name) = get_custom_property_name(node) else {
        return vec![];
    };

    let pattern = get_pattern();
    if host::regex_matches(&name, &pattern) {
        return vec![];
    }

    let (start, end) = host::node_range(node);

    vec![CheckResult {
        start,
        end,
        message: format!("Custom property `{name}` does not match pattern: {pattern}"),
        sev: Severity::Warning,
        actions: vec![],
        labels: vec![],
        notes: vec![],
        warnings: vec![],
        unnecessary: None,
        deprecated: None,
        verbose: None,
    }]
}

fn check_no_important_in_custom(node: u32) -> Vec<CheckResult> {
    let Some(name) = get_custom_property_name(node) else {
        return vec![];
    };

    // Check whether the declaration has a CSS_DECLARATION_IMPORTANT child.
    if host::node_child_by_kind(node, css_kinds::CSS_DECLARATION_IMPORTANT).is_none() {
        return vec![];
    }

    let (start, end) = host::node_range(node);

    vec![CheckResult {
        start,
        end,
        message: format!("Avoid using `!important` on custom property `{name}`"),
        sev: Severity::Warning,
        actions: vec![],
        labels: vec![],
        notes: vec![],
        warnings: vec![],
        unnecessary: None,
        deprecated: None,
        verbose: None,
    }]
}

export!(CssStyleConventions);
