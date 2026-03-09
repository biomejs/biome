//! Integration tests for `WasmPluginEngine` using compiled WASM plugin fixtures.

use biome_analyze::PluginTargetLanguage;
use biome_css_parser::CssParserOptions;
use biome_css_syntax::{CssFileSource, CssSyntaxKind};
use biome_js_parser::JsParserOptions;
use biome_js_syntax::{JsFileSource, JsSyntaxKind};
use biome_json_parser::JsonParserOptions;
use biome_json_syntax::JsonSyntaxKind;
use biome_rowan::AstNode;
use biome_wasm_plugin::WasmPluginEngine;
use std::path::Path;

/// Shared fixtures directory (under biome_plugin_loader).
fn fixture_path(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../biome_plugin_loader/tests/fixtures")
        .join(name)
}

fn load_fixture(name: &str) -> Vec<u8> {
    std::fs::read(fixture_path(name)).unwrap_or_else(|e| {
        panic!("Could not read fixture {name}: {e}. Run `just build-test-wasm-plugins` to build.")
    })
}

fn parse_js(source: &str) -> biome_js_syntax::JsSyntaxNode {
    biome_js_parser::parse(
        source,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    )
    .tree()
    .syntax()
    .clone()
}

/// Walk the tree depth-first and return the first node matching `kind`.
fn find_node(
    root: &biome_js_syntax::JsSyntaxNode,
    kind: JsSyntaxKind,
) -> biome_js_syntax::JsSyntaxNode {
    for node in root.descendants() {
        if node.kind() == kind {
            return node;
        }
    }
    panic!("Could not find node of kind {kind:?}");
}

// ---------------------------------------------------------------
// Engine basics
// ---------------------------------------------------------------

#[test]
fn engine_loads_valid_wasm() {
    let bytes = load_fixture("boolean_naming.wasm");
    WasmPluginEngine::new(&bytes).expect("should load valid WASM");
}

#[test]
fn engine_rejects_invalid_bytes() {
    let result = WasmPluginEngine::new(&[0, 1, 2, 3]);
    assert!(result.is_err());
}

#[test]
fn metadata_returns_correct_values() {
    let bytes = load_fixture("boolean_naming.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();
    let meta = engine.metadata().unwrap();

    assert_eq!(meta.language, "javascript");
    assert_eq!(meta.rule_names, vec!["booleanNaming"]);
    let query_kinds = meta
        .query_kinds_by_rule
        .get("booleanNaming")
        .expect("should have query kinds for booleanNaming");
    assert!(
        query_kinds.contains(&(JsSyntaxKind::JS_VARIABLE_DECLARATOR as u32)),
        "query_kinds should contain JS_VARIABLE_DECLARATOR, got: {query_kinds:?}",
    );
}

// ---------------------------------------------------------------
// check_node: matching / non-matching
// ---------------------------------------------------------------

#[test]
fn check_node_matching_returns_diagnostic() {
    let bytes = load_fixture("boolean_naming.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    let root = parse_js("const enabled = true;");
    let declarator = find_node(&root, JsSyntaxKind::JS_VARIABLE_DECLARATOR);

    let results = engine
        .check_node(
            declarator.into(),
            "booleanNaming",
            PluginTargetLanguage::JavaScript,
            None,
            None,
            String::new(),
            None,
        )
        .unwrap();

    assert_eq!(
        results.len(),
        1,
        "Expected 1 diagnostic for 'enabled = true'"
    );
}

#[test]
fn check_node_non_matching_returns_empty() {
    let bytes = load_fixture("boolean_naming.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    let root = parse_js("const isEnabled = true;");
    let declarator = find_node(&root, JsSyntaxKind::JS_VARIABLE_DECLARATOR);

    let results = engine
        .check_node(
            declarator.into(),
            "booleanNaming",
            PluginTargetLanguage::JavaScript,
            None,
            None,
            String::new(),
            None,
        )
        .unwrap();

    assert!(results.is_empty(), "Expected no diagnostics for isEnabled");
}

#[test]
fn configure_then_check() {
    let bytes = load_fixture("boolean_naming.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    // "hasFeature" matches default pattern but not ^(is)[A-Z]
    let root = parse_js("const hasFeature = true;");
    let declarator = find_node(&root, JsSyntaxKind::JS_VARIABLE_DECLARATOR);

    let results = engine
        .check_node(
            declarator.into(),
            "booleanNaming",
            PluginTargetLanguage::JavaScript,
            None,
            None,
            String::new(),
            Some(r#"{"pattern": "^(is)[A-Z]"}"#),
        )
        .unwrap();

    assert_eq!(
        results.len(),
        1,
        "Expected 1 diagnostic: 'hasFeature' doesn't match ^(is)[A-Z]"
    );
}

// ---------------------------------------------------------------
// Boolean detection: literals
// ---------------------------------------------------------------

#[test]
fn boolean_naming_detects_bad_name() {
    let bytes = load_fixture("boolean_naming.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    let root = parse_js("const enabled = true;");
    let declarator = find_node(&root, JsSyntaxKind::JS_VARIABLE_DECLARATOR);

    let results = engine
        .check_node(
            declarator.into(),
            "booleanNaming",
            PluginTargetLanguage::JavaScript,
            None,
            None,
            String::new(),
            None,
        )
        .unwrap();

    assert_eq!(
        results.len(),
        1,
        "Expected 1 diagnostic for 'enabled = true'"
    );
}

#[test]
fn boolean_naming_detects_false_literal() {
    let bytes = load_fixture("boolean_naming.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    let root = parse_js("const disabled = false;");
    let declarator = find_node(&root, JsSyntaxKind::JS_VARIABLE_DECLARATOR);

    let results = engine
        .check_node(
            declarator.into(),
            "booleanNaming",
            PluginTargetLanguage::JavaScript,
            None,
            None,
            String::new(),
            None,
        )
        .unwrap();

    assert_eq!(
        results.len(),
        1,
        "Expected 1 diagnostic for 'disabled = false'"
    );
}

#[test]
fn boolean_naming_accepts_good_name() {
    let bytes = load_fixture("boolean_naming.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    let root = parse_js("const isEnabled = true;");
    let declarator = find_node(&root, JsSyntaxKind::JS_VARIABLE_DECLARATOR);

    let results = engine
        .check_node(
            declarator.into(),
            "booleanNaming",
            PluginTargetLanguage::JavaScript,
            None,
            None,
            String::new(),
            None,
        )
        .unwrap();

    assert!(results.is_empty(), "Should not flag 'isEnabled'");
}

// ---------------------------------------------------------------
// Boolean detection: comparisons and operators
// ---------------------------------------------------------------

#[test]
fn boolean_naming_detects_comparison() {
    let bytes = load_fixture("boolean_naming.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    let root = parse_js("const equal = a === b;");
    let declarator = find_node(&root, JsSyntaxKind::JS_VARIABLE_DECLARATOR);

    let results = engine
        .check_node(
            declarator.into(),
            "booleanNaming",
            PluginTargetLanguage::JavaScript,
            None,
            None,
            String::new(),
            None,
        )
        .unwrap();

    assert_eq!(
        results.len(),
        1,
        "Expected 1 diagnostic: 'equal = a === b' is a boolean comparison"
    );
}

#[test]
fn boolean_naming_detects_less_than() {
    let bytes = load_fixture("boolean_naming.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    let root = parse_js("const smaller = a < b;");
    let declarator = find_node(&root, JsSyntaxKind::JS_VARIABLE_DECLARATOR);

    let results = engine
        .check_node(
            declarator.into(),
            "booleanNaming",
            PluginTargetLanguage::JavaScript,
            None,
            None,
            String::new(),
            None,
        )
        .unwrap();

    assert_eq!(
        results.len(),
        1,
        "Expected 1 diagnostic: 'smaller = a < b' is boolean comparison"
    );
}

#[test]
fn boolean_naming_detects_negation() {
    let bytes = load_fixture("boolean_naming.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    let root = parse_js("const flipped = !value;");
    let declarator = find_node(&root, JsSyntaxKind::JS_VARIABLE_DECLARATOR);

    let results = engine
        .check_node(
            declarator.into(),
            "booleanNaming",
            PluginTargetLanguage::JavaScript,
            None,
            None,
            String::new(),
            None,
        )
        .unwrap();

    assert_eq!(
        results.len(),
        1,
        "Expected 1 diagnostic: 'flipped = !value' is boolean (negation)"
    );
}

#[test]
fn boolean_naming_detects_instanceof() {
    let bytes = load_fixture("boolean_naming.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    let root = parse_js("const check = a instanceof B;");
    let declarator = find_node(&root, JsSyntaxKind::JS_VARIABLE_DECLARATOR);

    let results = engine
        .check_node(
            declarator.into(),
            "booleanNaming",
            PluginTargetLanguage::JavaScript,
            None,
            None,
            String::new(),
            None,
        )
        .unwrap();

    assert_eq!(
        results.len(),
        1,
        "Expected 1 diagnostic: 'check = a instanceof B' is boolean"
    );
}

#[test]
fn boolean_naming_detects_in_expression() {
    let bytes = load_fixture("boolean_naming.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    let root = parse_js("const exists = 'key' in obj;");
    let declarator = find_node(&root, JsSyntaxKind::JS_VARIABLE_DECLARATOR);

    let results = engine
        .check_node(
            declarator.into(),
            "booleanNaming",
            PluginTargetLanguage::JavaScript,
            None,
            None,
            String::new(),
            None,
        )
        .unwrap();

    assert_eq!(
        results.len(),
        1,
        "Expected 1 diagnostic: \"exists = 'key' in obj\" is boolean"
    );
}

#[test]
fn boolean_naming_accepts_comparison_with_good_name() {
    let bytes = load_fixture("boolean_naming.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    let root = parse_js("const isEqual = a === b;");
    let declarator = find_node(&root, JsSyntaxKind::JS_VARIABLE_DECLARATOR);

    let results = engine
        .check_node(
            declarator.into(),
            "booleanNaming",
            PluginTargetLanguage::JavaScript,
            None,
            None,
            String::new(),
            None,
        )
        .unwrap();

    assert!(results.is_empty(), "Should not flag 'isEqual = a === b'");
}

// ---------------------------------------------------------------
// Non-boolean initializers
// ---------------------------------------------------------------

#[test]
fn boolean_naming_ignores_non_boolean_init() {
    let bytes = load_fixture("boolean_naming.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    let root = parse_js("const count = 42;");
    let declarator = find_node(&root, JsSyntaxKind::JS_VARIABLE_DECLARATOR);

    let results = engine
        .check_node(
            declarator.into(),
            "booleanNaming",
            PluginTargetLanguage::JavaScript,
            None,
            None,
            String::new(),
            None,
        )
        .unwrap();

    assert!(
        results.is_empty(),
        "Should not flag non-boolean initializer"
    );
}

#[test]
fn boolean_naming_ignores_no_initializer() {
    let bytes = load_fixture("boolean_naming.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    let root = parse_js("let active;");
    let declarator = find_node(&root, JsSyntaxKind::JS_VARIABLE_DECLARATOR);

    let results = engine
        .check_node(
            declarator.into(),
            "booleanNaming",
            PluginTargetLanguage::JavaScript,
            None,
            None,
            String::new(),
            None,
        )
        .unwrap();

    assert!(
        results.is_empty(),
        "Should not flag declarator without initializer"
    );
}

// ---------------------------------------------------------------
// Edge cases
// ---------------------------------------------------------------

#[test]
fn boolean_naming_rejects_prefix_only_name() {
    let bytes = load_fixture("boolean_naming.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    // "is" alone doesn't satisfy `^(is|...)[A-Z]` — the next char must be uppercase.
    let root = parse_js("const is = true;");
    let declarator = find_node(&root, JsSyntaxKind::JS_VARIABLE_DECLARATOR);

    let results = engine
        .check_node(
            declarator.into(),
            "booleanNaming",
            PluginTargetLanguage::JavaScript,
            None,
            None,
            String::new(),
            None,
        )
        .unwrap();

    assert_eq!(
        results.len(),
        1,
        "Expected 1 diagnostic: 'is' alone doesn't match ^(is)[A-Z]"
    );
}

#[test]
fn boolean_naming_with_custom_pattern() {
    let bytes = load_fixture("boolean_naming.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    // "hasFeature" matches default pattern but would NOT match "^(is)[A-Z]"
    let root = parse_js("const hasFeature = true;");
    let declarator = find_node(&root, JsSyntaxKind::JS_VARIABLE_DECLARATOR);

    let results = engine
        .check_node(
            declarator.into(),
            "booleanNaming",
            PluginTargetLanguage::JavaScript,
            None,
            None,
            String::new(),
            Some(r#"{"pattern": "^(is)[A-Z]"}"#),
        )
        .unwrap();

    assert_eq!(
        results.len(),
        1,
        "Expected 1 diagnostic: 'hasFeature' doesn't match ^(is)[A-Z]"
    );
}

// ===============================================================
// CSS plugin tests
// ===============================================================

fn parse_css(source: &str) -> biome_css_syntax::CssSyntaxNode {
    biome_css_parser::parse_css(source, CssFileSource::css(), CssParserOptions::default())
        .tree()
        .syntax()
        .clone()
}

fn find_css_node(
    root: &biome_css_syntax::CssSyntaxNode,
    kind: CssSyntaxKind,
) -> biome_css_syntax::CssSyntaxNode {
    root.descendants()
        .find(|n| n.kind() == kind)
        .unwrap_or_else(|| panic!("Could not find CSS node of kind {kind:?}"))
}

// ---------------------------------------------------------------
// customPropertyPattern
// ---------------------------------------------------------------

#[test]
fn css_custom_property_pattern_detects_bad_name() {
    let bytes = load_fixture("css_style_conventions.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    let root = parse_css(":root { --camelCase: red; }");
    let decl = find_css_node(&root, CssSyntaxKind::CSS_DECLARATION);

    let results = engine
        .check_node(
            decl.into(),
            "customPropertyPattern",
            PluginTargetLanguage::Css,
            None,
            None,
            String::new(),
            None,
        )
        .unwrap();

    assert_eq!(results.len(), 1, "Expected 1 diagnostic for --camelCase");
}

#[test]
fn css_custom_property_pattern_accepts_good_name() {
    let bytes = load_fixture("css_style_conventions.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    let root = parse_css(":root { --valid-name: red; }");
    let decl = find_css_node(&root, CssSyntaxKind::CSS_DECLARATION);

    let results = engine
        .check_node(
            decl.into(),
            "customPropertyPattern",
            PluginTargetLanguage::Css,
            None,
            None,
            String::new(),
            None,
        )
        .unwrap();

    assert!(results.is_empty(), "Should not flag --valid-name");
}

#[test]
fn css_custom_property_pattern_with_custom_option() {
    let bytes = load_fixture("css_style_conventions.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    // --valid-name matches default but not ^--my-prefix-
    let root = parse_css(":root { --valid-name: red; }");
    let decl = find_css_node(&root, CssSyntaxKind::CSS_DECLARATION);

    let results = engine
        .check_node(
            decl.into(),
            "customPropertyPattern",
            PluginTargetLanguage::Css,
            None,
            None,
            String::new(),
            Some(r#"{"pattern": "^--my-prefix-"}"#),
        )
        .unwrap();

    assert_eq!(
        results.len(),
        1,
        "Expected 1 diagnostic: --valid-name doesn't match ^--my-prefix-"
    );
}

#[test]
fn css_custom_property_pattern_ignores_regular_property() {
    let bytes = load_fixture("css_style_conventions.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    let root = parse_css(".foo { color: red; }");
    let decl = find_css_node(&root, CssSyntaxKind::CSS_DECLARATION);

    let results = engine
        .check_node(
            decl.into(),
            "customPropertyPattern",
            PluginTargetLanguage::Css,
            None,
            None,
            String::new(),
            None,
        )
        .unwrap();

    assert!(
        results.is_empty(),
        "Should not flag regular property 'color'"
    );
}

// ---------------------------------------------------------------
// noImportantInCustomProperties
// ---------------------------------------------------------------

#[test]
fn css_no_important_on_custom_property() {
    let bytes = load_fixture("css_style_conventions.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    let root = parse_css(".foo { --my-prop: red !important; }");
    let decl = find_css_node(&root, CssSyntaxKind::CSS_DECLARATION);

    let results = engine
        .check_node(
            decl.into(),
            "noImportantInCustomProperties",
            PluginTargetLanguage::Css,
            None,
            None,
            String::new(),
            None,
        )
        .unwrap();

    assert_eq!(
        results.len(),
        1,
        "Expected 1 diagnostic for !important on custom property"
    );
}

#[test]
fn css_no_important_allows_regular_property_important() {
    let bytes = load_fixture("css_style_conventions.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    let root = parse_css(".foo { color: red !important; }");
    let decl = find_css_node(&root, CssSyntaxKind::CSS_DECLARATION);

    let results = engine
        .check_node(
            decl.into(),
            "noImportantInCustomProperties",
            PluginTargetLanguage::Css,
            None,
            None,
            String::new(),
            None,
        )
        .unwrap();

    assert!(
        results.is_empty(),
        "Should not flag !important on regular property"
    );
}

#[test]
fn css_no_important_allows_custom_without_important() {
    let bytes = load_fixture("css_style_conventions.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    let root = parse_css(":root { --my-prop: red; }");
    let decl = find_css_node(&root, CssSyntaxKind::CSS_DECLARATION);

    let results = engine
        .check_node(
            decl.into(),
            "noImportantInCustomProperties",
            PluginTargetLanguage::Css,
            None,
            None,
            String::new(),
            None,
        )
        .unwrap();

    assert!(
        results.is_empty(),
        "Should not flag custom prop without !important"
    );
}

#[test]
fn css_multi_rule_returns_both_rules() {
    let bytes = load_fixture("css_style_conventions.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();
    let meta = engine.metadata().unwrap();

    assert_eq!(meta.language, "css");
    assert_eq!(meta.rule_names.len(), 2);
    assert!(meta.rule_names.contains(&"customPropertyPattern".into()));
    assert!(
        meta.rule_names
            .contains(&"noImportantInCustomProperties".into())
    );
}

// ===============================================================
// JSON plugin tests
// ===============================================================

fn parse_json(source: &str) -> biome_json_syntax::JsonSyntaxNode {
    biome_json_parser::parse_json(source, JsonParserOptions::default())
        .tree()
        .syntax()
        .clone()
}

fn find_json_node(
    root: &biome_json_syntax::JsonSyntaxNode,
    kind: JsonSyntaxKind,
) -> biome_json_syntax::JsonSyntaxNode {
    root.descendants()
        .find(|n| n.kind() == kind)
        .unwrap_or_else(|| panic!("Could not find JSON node of kind {kind:?}"))
}

#[test]
fn json_key_naming_detects_snake_case() {
    let bytes = load_fixture("json_naming.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    let root = parse_json(r#"{ "another_key": "value" }"#);
    let member_name = find_json_node(&root, JsonSyntaxKind::JSON_MEMBER_NAME);

    let results = engine
        .check_node(
            member_name.into(),
            "keyNamingConvention",
            PluginTargetLanguage::Json,
            None,
            None,
            String::new(),
            None,
        )
        .unwrap();

    assert_eq!(
        results.len(),
        1,
        "Expected 1 diagnostic for snake_case key with camelCase convention"
    );
}

#[test]
fn json_key_naming_accepts_camel_case() {
    let bytes = load_fixture("json_naming.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    let root = parse_json(r#"{ "validKey": "value" }"#);
    let member_name = find_json_node(&root, JsonSyntaxKind::JSON_MEMBER_NAME);

    let results = engine
        .check_node(
            member_name.into(),
            "keyNamingConvention",
            PluginTargetLanguage::Json,
            None,
            None,
            String::new(),
            None,
        )
        .unwrap();

    assert!(results.is_empty(), "Should not flag valid camelCase key");
}

#[test]
fn json_key_naming_with_snake_case_convention() {
    let bytes = load_fixture("json_naming.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    // "validKey" is camelCase, which should fail snake_case convention
    let root = parse_json(r#"{ "validKey": "value" }"#);
    let member_name = find_json_node(&root, JsonSyntaxKind::JSON_MEMBER_NAME);

    let results = engine
        .check_node(
            member_name.into(),
            "keyNamingConvention",
            PluginTargetLanguage::Json,
            None,
            None,
            String::new(),
            Some(r#"{"convention": "snake_case"}"#),
        )
        .unwrap();

    assert_eq!(
        results.len(),
        1,
        "Expected 1 diagnostic: 'validKey' doesn't match snake_case"
    );
}

#[test]
fn json_key_naming_accepts_single_word() {
    let bytes = load_fixture("json_naming.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    let root = parse_json(r#"{ "valid": "value" }"#);
    let member_name = find_json_node(&root, JsonSyntaxKind::JSON_MEMBER_NAME);

    let results = engine
        .check_node(
            member_name.into(),
            "keyNamingConvention",
            PluginTargetLanguage::Json,
            None,
            None,
            String::new(),
            None,
        )
        .unwrap();

    assert!(results.is_empty(), "Should not flag single lowercase word");
}

#[test]
fn json_key_naming_strips_quotes() {
    let bytes = load_fixture("json_naming.wasm");
    let engine = WasmPluginEngine::new(&bytes).unwrap();

    // "bad_key" is snake_case — should be flagged by camelCase default convention.
    // This also confirms that the plugin correctly strips surrounding quotes.
    let root = parse_json(r#"{ "bad_key": "value" }"#);
    let member_name = find_json_node(&root, JsonSyntaxKind::JSON_MEMBER_NAME);

    let results = engine
        .check_node(
            member_name.into(),
            "keyNamingConvention",
            PluginTargetLanguage::Json,
            None,
            None,
            String::new(),
            None,
        )
        .unwrap();

    assert_eq!(
        results.len(),
        1,
        "Expected 1 diagnostic for snake_case key 'bad_key'"
    );
}
