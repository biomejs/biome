//! WASM plugin: booleanNaming
//!
//! Checks that boolean variables follow a naming convention. By default,
//! boolean variable names must match `^(is|was|has|should|can|will|did|does)[A-Z]`.
//!
//! The pattern is configurable via the `options` field in `biome.json`:
//! ```json
//! { "path": "./boolean-naming.wasm", "options": { "pattern": "^(is|has)[A-Z]" } }
//! ```
//!
//! This rule fires when the initializer has a boolean type. Detection uses
//! type inference when available, with syntactic fallback for boolean literals,
//! comparison operators, `!`, `instanceof`, and `in` expressions.

use biome_plugin_sdk::js_kinds;

biome_plugin_sdk::generate_plugin!();

use biome::plugin::host;
use biome::plugin::types::Severity;

/// Default regex pattern for boolean variable names.
const DEFAULT_PATTERN: &str = r"^(is|was|has|should|can|will|did|does)[A-Z]";

/// Global state for the configured pattern string.
/// WASM plugins are single-threaded, so a `Cell`-like approach is fine.
/// We use `std::sync::OnceLock` to avoid `static mut`.
use std::sync::OnceLock;

static PATTERN: OnceLock<String> = OnceLock::new();

fn get_pattern() -> &'static str {
    PATTERN.get().map(|s| s.as_str()).unwrap_or(DEFAULT_PATTERN)
}

struct BooleanNaming;

impl Guest for BooleanNaming {
    fn target_language() -> String {
        "javascript".into()
    }

    fn rule_names() -> Vec<String> {
        vec!["booleanNaming".into()]
    }

    fn query_kinds_for_rule(_rule: String) -> Vec<u32> {
        vec![js_kinds::JS_VARIABLE_DECLARATOR]
    }

    fn configure(_rule: String, options_json: String) {
        if let Some(pattern) = biome_plugin_sdk::options::get_string(&options_json, "pattern") {
            let _ = PATTERN.set(pattern);
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
        // A JS_VARIABLE_DECLARATOR has children:
        //   - JS_IDENTIFIER_BINDING (the name)
        //   - JS_INITIALIZER_CLAUSE (optional, contains `= <expr>`)
        //
        // Boolean detection strategy:
        //   1. Use `expression-type` host function (type inference) — returns
        //      "boolean" for any expression the type system knows is boolean.
        //   2. When type inference returns "unknown", fall back to syntactic
        //      heuristics that detect structurally-boolean expressions.

        let children = host::node_children(node);

        let mut binding_name: Option<String> = None;
        let mut has_boolean_init = false;

        for child in &children {
            let kind = host::node_kind(*child);

            if kind == js_kinds::JS_IDENTIFIER_BINDING {
                binding_name = Some(host::node_trimmed_text(*child));
            }

            if kind == js_kinds::JS_INITIALIZER_CLAUSE {
                // Find the expression child of the initializer clause.
                let init_children = host::node_children(*child);
                for init_child in &init_children {
                    if !host::element_is_token(*init_child) {
                        // Primary: use type inference.
                        let expr_type = host::expression_type(*init_child);
                        if expr_type == "boolean" {
                            has_boolean_init = true;
                            break;
                        }
                        // Fallback: syntactic heuristics when type is unknown.
                        if expr_type == "unknown" {
                            has_boolean_init = is_syntactically_boolean(*init_child);
                        }
                        break;
                    }
                }
            }
        }

        let Some(name) = binding_name else {
            return vec![];
        };

        if !has_boolean_init {
            return vec![];
        }

        // Check the name against the pattern using host regex.
        let pattern = get_pattern();
        if host::regex_matches(&name, pattern) {
            return vec![];
        }

        let (start, end) = host::node_range(node);

        vec![CheckResult {
            start,
            end,
            message: format!("Boolean variable `{name}` should match naming pattern: {pattern}"),
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

/// Check whether an expression node is syntactically a boolean expression.
///
/// This covers cases where type inference is unavailable ("unknown"):
///   - Boolean literals (`true` / `false`)
///   - Comparison binary expressions (`==`, `===`, `!=`, `!==`, `<`, `>`, `<=`, `>=`)
///   - Logical NOT (`!expr`)
///   - `instanceof` expressions
///   - `in` expressions
fn is_syntactically_boolean(expr: u32) -> bool {
    let kind = host::node_kind(expr);

    // Boolean literals: `true` or `false`.
    if kind == js_kinds::JS_BOOLEAN_LITERAL_EXPRESSION {
        return true;
    }

    // `instanceof` and `in` always produce booleans.
    if kind == js_kinds::JS_INSTANCEOF_EXPRESSION || kind == js_kinds::JS_IN_EXPRESSION {
        return true;
    }

    // Unary `!` always produces a boolean.
    if kind == js_kinds::JS_UNARY_EXPRESSION {
        let children = host::node_children_with_tokens(expr);
        for child in &children {
            if host::element_is_token(*child) && host::node_kind(*child) == js_kinds::BANG {
                return true;
            }
        }
    }

    // Binary expressions with comparison operators produce booleans.
    if kind == js_kinds::JS_BINARY_EXPRESSION {
        let children = host::node_children_with_tokens(expr);
        for child in &children {
            if host::element_is_token(*child) {
                let tk = host::node_kind(*child);
                if tk == js_kinds::EQ2
                    || tk == js_kinds::EQ3
                    || tk == js_kinds::NEQ
                    || tk == js_kinds::NEQ2
                    || tk == js_kinds::L_ANGLE
                    || tk == js_kinds::R_ANGLE
                    || tk == js_kinds::LTEQ
                    || tk == js_kinds::GTEQ
                {
                    return true;
                }
            }
        }
    }

    false
}

export!(BooleanNaming);
