//! WASM plugin: json-naming
//!
//! Single-rule plugin: `keyNamingConvention`.
//!
//! Checks that JSON object keys follow a naming convention.
//! Default convention is `camelCase`.
//!
//! Options: `{ "convention": "camelCase" | "snake_case" | "kebab-case" | "PascalCase" | "SCREAMING_SNAKE_CASE" }`

use biome_plugin_sdk::json_kinds;

biome_plugin_sdk::generate_plugin!();

use biome::plugin::host;
use biome::plugin::types::Severity;

use std::sync::OnceLock;

/// Stored as `(convention_name, regex_pattern)`.
static CONVENTION: OnceLock<(String, String)> = OnceLock::new();

const DEFAULT_CONVENTION: &str = "camelCase";
const DEFAULT_PATTERN: &str = r"^[a-z][a-zA-Z0-9]*$";

fn convention_to_pattern(convention: &str) -> &'static str {
    match convention {
        "camelCase" => r"^[a-z][a-zA-Z0-9]*$",
        "snake_case" => r"^[a-z][a-z0-9]*(_[a-z0-9]+)*$",
        "kebab-case" => r"^[a-z][a-z0-9]*(-[a-z0-9]+)*$",
        "PascalCase" => r"^[A-Z][a-zA-Z0-9]*$",
        "SCREAMING_SNAKE_CASE" => r"^[A-Z][A-Z0-9]*(_[A-Z0-9]+)*$",
        _ => DEFAULT_PATTERN,
    }
}

fn get_convention() -> (&'static str, &'static str) {
    CONVENTION
        .get()
        .map(|(name, pat)| (name.as_str(), pat.as_str()))
        .unwrap_or((DEFAULT_CONVENTION, DEFAULT_PATTERN))
}

struct JsonNaming;

impl Guest for JsonNaming {
    fn target_language() -> String {
        "json".into()
    }

    fn rule_names() -> Vec<String> {
        vec!["keyNamingConvention".into()]
    }

    fn query_kinds_for_rule(_rule: String) -> Vec<u32> {
        vec![json_kinds::JSON_MEMBER_NAME]
    }

    fn source_triggers_for_rule(_rule: String) -> Vec<String> {
        vec![]
    }

    fn configure(_rule: String, options_json: String) {
        if let Some(convention) = biome_plugin_sdk::options::get_string(&options_json, "convention")
        {
            let pattern = convention_to_pattern(&convention).to_string();
            let _ = CONVENTION.set((convention, pattern));
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

    fn check(node: u32, _rule: String) -> Vec<CheckResult> {
        let raw_text = host::node_trimmed_text(node);

        // JSON member name includes surrounding quotes from the lexer.
        let Some(stripped) = raw_text.strip_prefix('"').and_then(|s| s.strip_suffix('"')) else {
            return vec![];
        };

        // Single-word keys that are all lowercase always pass (they're valid
        // in every convention except SCREAMING_SNAKE_CASE, but we still let
        // single lowercase words pass for usability).
        // Actually, let's just check against the pattern.

        let (convention_name, pattern) = get_convention();

        if host::regex_matches(stripped, pattern) {
            return vec![];
        }

        let (start, end) = host::node_range(node);

        vec![CheckResult {
            start,
            end,
            message: format!("Key `{stripped}` does not follow {convention_name} convention"),
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
}

export!(JsonNaming);
