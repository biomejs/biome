//! Integration tests for `AnalyzerWasmPlugin` using compiled WASM fixtures.
//!
//! These tests exercise the `biome_plugin_loader` layer that wraps
//! `WasmPluginEngine` with analyzer-compatible types.

#![cfg(feature = "wasm_plugin")]

use biome_analyze::{AnalyzerPlugin, Phases, PluginTargetLanguage, ServiceBag};
use biome_css_parser::CssParserOptions;
use biome_css_syntax::{CssFileSource, CssSyntaxKind};
use biome_js_parser::JsParserOptions;
use biome_js_syntax::{JsFileSource, JsSyntaxKind};
use biome_json_parser::JsonParserOptions;
use biome_json_syntax::JsonSyntaxKind;
use biome_plugin_loader::AnalyzerWasmPlugin;
use biome_rowan::{AstNode, RawSyntaxKind};
use camino::Utf8Path;
use std::path::Path;
use std::sync::Arc;

fn fixture_dir() -> camino::Utf8PathBuf {
    let p = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .canonicalize()
        .unwrap();
    camino::Utf8PathBuf::try_from(p).unwrap()
}

fn fixture_path(name: &str) -> camino::Utf8PathBuf {
    fixture_dir().join(name)
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

fn find_node(
    root: &biome_js_syntax::JsSyntaxNode,
    kind: JsSyntaxKind,
) -> biome_js_syntax::JsSyntaxNode {
    root.descendants()
        .find(|n| n.kind() == kind)
        .unwrap_or_else(|| panic!("Could not find node of kind {kind:?}"))
}

// ---------------------------------------------------------------
// Loading
// ---------------------------------------------------------------

#[test]
fn load_wasm_plugin() {
    let path = fixture_path("boolean_naming.wasm");
    let plugins = AnalyzerWasmPlugin::load(path.as_ref(), "boolean_naming", None).expect("should load WASM plugin");
    assert_eq!(plugins.len(), 1, "Expected 1 rule from boolean_naming");
}

#[test]
fn load_invalid_path() {
    let result = AnalyzerWasmPlugin::load(Utf8Path::new("/nonexistent/plugin.wasm"), "nonexistent", None);
    assert!(result.is_err());
}

// ---------------------------------------------------------------
// Metadata / trait methods
// ---------------------------------------------------------------

#[test]
fn phase_is_semantic_for_js() {
    let plugins =
        AnalyzerWasmPlugin::load(fixture_path("boolean_naming.wasm").as_ref(), "boolean_naming", None).unwrap();
    assert_eq!(plugins[0].phase(), Phases::Semantic);
}

#[test]
fn language_is_javascript() {
    let plugins =
        AnalyzerWasmPlugin::load(fixture_path("boolean_naming.wasm").as_ref(), "boolean_naming", None).unwrap();
    assert_eq!(plugins[0].language(), PluginTargetLanguage::JavaScript);
}

#[test]
fn query_returns_correct_kinds() {
    let plugins =
        AnalyzerWasmPlugin::load(fixture_path("boolean_naming.wasm").as_ref(), "boolean_naming", None).unwrap();
    let kinds = plugins[0].query();
    assert!(
        kinds.contains(&RawSyntaxKind(JsSyntaxKind::JS_VARIABLE_DECLARATOR as u16)),
        "Expected JS_VARIABLE_DECLARATOR in query kinds: {kinds:?}",
    );
}

// ---------------------------------------------------------------
// Evaluate
// ---------------------------------------------------------------

#[test]
fn evaluate_matching_node() {
    let plugins =
        AnalyzerWasmPlugin::load(fixture_path("boolean_naming.wasm").as_ref(), "boolean_naming", None).unwrap();

    let root = parse_js("const enabled = true;");
    let declarator = find_node(&root, JsSyntaxKind::JS_VARIABLE_DECLARATOR);

    let services = ServiceBag::default();
    let path = Arc::new(camino::Utf8PathBuf::from("test.js"));
    let result = plugins[0].evaluate(declarator.into(), path, &services);
    assert_eq!(result.diagnostics.len(), 1, "Expected 1 diagnostic");
    assert!(
        result.diagnostics[0].actions.is_empty(),
        "booleanNaming should not produce an autofix"
    );
}

#[test]
fn evaluate_non_matching_node() {
    let plugins =
        AnalyzerWasmPlugin::load(fixture_path("boolean_naming.wasm").as_ref(), "boolean_naming", None).unwrap();

    let root = parse_js("const isEnabled = true;");
    let declarator = find_node(&root, JsSyntaxKind::JS_VARIABLE_DECLARATOR);

    let services = ServiceBag::default();
    let path = Arc::new(camino::Utf8PathBuf::from("test.js"));
    let result = plugins[0].evaluate(declarator.into(), path, &services);
    assert!(result.diagnostics.is_empty(), "Should not flag 'isEnabled'");
}

#[test]
fn evaluate_with_options() {
    let plugins = AnalyzerWasmPlugin::load(
        fixture_path("boolean_naming.wasm").as_ref(),
        "boolean_naming",
        Some(r#"{"pattern": "^(is)[A-Z]"}"#.to_string()),
    )
    .unwrap();

    // "hasFeature" matches default but not ^(is)[A-Z]
    let root = parse_js("const hasFeature = true;");
    let declarator = find_node(&root, JsSyntaxKind::JS_VARIABLE_DECLARATOR);

    let services = ServiceBag::default();
    let path = Arc::new(camino::Utf8PathBuf::from("test.js"));
    let result = plugins[0].evaluate(declarator.into(), path, &services);
    assert_eq!(result.diagnostics.len(), 1);
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

#[test]
fn load_css_plugin() {
    let path = fixture_path("css_style_conventions.wasm");
    let plugins = AnalyzerWasmPlugin::load(path.as_ref(), "css_style_conventions", None).expect("should load CSS plugin");
    assert_eq!(
        plugins.len(),
        2,
        "Expected 2 rules from css-style-conventions"
    );
}

#[test]
fn css_plugin_metadata() {
    let plugins =
        AnalyzerWasmPlugin::load(fixture_path("css_style_conventions.wasm").as_ref(), "css_style_conventions", None)
            .unwrap();

    // Check that both rules have correct language and phase
    for plugin in &plugins {
        assert_eq!(plugin.language(), PluginTargetLanguage::Css);
        assert_eq!(plugin.phase(), Phases::Syntax);
    }

    // Check that query kinds include CSS_DECLARATION
    let kinds = plugins[0].query();
    assert!(
        kinds.contains(&RawSyntaxKind(CssSyntaxKind::CSS_DECLARATION as u16)),
        "Expected CSS_DECLARATION in query kinds: {kinds:?}",
    );
}

#[test]
fn css_plugin_evaluate() {
    let plugins =
        AnalyzerWasmPlugin::load(fixture_path("css_style_conventions.wasm").as_ref(), "css_style_conventions", None)
            .unwrap();

    // Find the customPropertyPattern rule
    let pattern_rule = plugins
        .iter()
        .find(|p| p.rule_name() == "customPropertyPattern")
        .expect("should have customPropertyPattern rule");

    let root = parse_css(":root { --camelCase: red; }");
    let decl = find_css_node(&root, CssSyntaxKind::CSS_DECLARATION);

    let services = ServiceBag::default();
    let path = Arc::new(camino::Utf8PathBuf::from("test.css"));
    let result = pattern_rule.evaluate(decl.into(), path, &services);
    assert_eq!(result.diagnostics.len(), 1, "Expected 1 diagnostic");
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
fn load_json_plugin() {
    let path = fixture_path("json_naming.wasm");
    let plugins = AnalyzerWasmPlugin::load(path.as_ref(), "json_naming", None).expect("should load JSON plugin");
    assert_eq!(plugins.len(), 1, "Expected 1 rule from json-naming");
}

#[test]
fn json_plugin_evaluate() {
    let plugins =
        AnalyzerWasmPlugin::load(fixture_path("json_naming.wasm").as_ref(), "json_naming", None).unwrap();

    let root = parse_json(r#"{ "another_key": "value" }"#);
    let member_name = find_json_node(&root, JsonSyntaxKind::JSON_MEMBER_NAME);

    let services = ServiceBag::default();
    let path = Arc::new(camino::Utf8PathBuf::from("test.json"));
    let result = plugins[0].evaluate(member_name.into(), path, &services);
    assert_eq!(
        result.diagnostics.len(),
        1,
        "Expected 1 diagnostic for snake_case key"
    );
}
