use crate::analyzer_js_plugin::tests::{render_diagnostics, snap_diagnostics};

use super::tests::{load_test_plugin_from_source, services};
use super::*;
use biome_diagnostics::{Error, PrintDescription, print_diagnostic_to_string};
use biome_fs::MemoryFileSystem;
use biome_js_parser::{JsParserOptions, parse};
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_resolver::FsWithResolverProxy;
use std::sync::Arc;

fn analyze(plugin: &AnalyzerJsPlugin, source: &str, bag: &ServiceBag) -> Vec<String> {
    let parsed = parse(
        source,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );
    plugin
        .evaluate(parsed.syntax().into(), "/file.js".into(), bag)
        .entries
        .iter()
        .map(|entry| PrintDescription(&entry.diagnostic).to_string())
        .collect()
}

#[test]
fn grit_matches_separate_nodes() {
    let plugin = load_test_plugin_from_source(
        "/plugin.js",
        r#"import { defineRule, grit, registerDiagnostic } from "@biomejs/runtime/plugin";
        export const rule = defineRule({
            query: grit("`console.log($args)`"),
            run(node, context) {
                registerDiagnostic(node, "information", `${"model" in context}:${node.text}`);
            },
        });"#,
        None,
    );

    let messages = analyze(
        &plugin,
        "console.log('a'); console.log('b'); other();",
        &services(JsFileSource::js_module()),
    );

    assert_eq!(
        messages,
        ["false:console.log('a')", "false:console.log('b')"]
    );
}

#[test]
fn grit_matches_a_statement_list() {
    let plugin = load_test_plugin_from_source(
        "/plugin.js",
        r#"import { defineRule, grit, registerDiagnostic } from "@biomejs/runtime/plugin";
        export const rule = defineRule({
            query: grit("`foo(); bar()`"),
            run(node) {
                registerDiagnostic(node, "information", node.text);
            },
        });"#,
        None,
    );

    assert_eq!(
        analyze(
            &plugin,
            "foo(); bar();",
            &services(JsFileSource::js_module())
        ),
        ["foo();", "bar();"]
    );
    assert!(
        analyze(
            &plugin,
            "foo(); bar(); baz();",
            &services(JsFileSource::js_module())
        )
        .is_empty(),
        "a longer statement list must not match"
    );
}

#[test]
fn grit_matches_a_leaf_token() {
    let plugin = load_test_plugin_from_source(
        "/plugin.js",
        r#"import { defineRule, grit, registerDiagnostic } from "@biomejs/runtime/plugin";
        export const rule = defineRule({
            query: grit("`+`"),
            run(token) {
                registerDiagnostic(token.parent, "information", `${token.kind}:${token.text}`);
            },
        });"#,
        None,
    );

    assert_eq!(
        analyze(
            &plugin,
            "const value = 1 + 2;",
            &services(JsFileSource::js_module())
        ),
        ["PLUS:+"]
    );
}

#[test]
fn grit_can_request_the_semantic_model() {
    let plugin_source = r#"import { defineRule, grit, registerDiagnostic } from "@biomejs/runtime/plugin";
        export const rule = defineRule({
            query: grit("JsReferenceIdentifier()", { semantic: true }),
            run(node, { model }) {
                const binding = model.binding(node);
                registerDiagnostic(node, "information", `${binding.syntax().text}:${binding.allReads().length}`);
            },
        });"#;
    let plugin = load_test_plugin_from_source("/plugin.js", plugin_source, None);

    let source_type = JsFileSource::js_module();
    let source = "let value = 1; value;";
    let parsed = parse(source, source_type, JsParserOptions::default());
    let mut bag = services(source_type);
    bag.insert_service(semantic_model(
        &parsed.tree(),
        SemanticModelOptions::from(&source_type),
    ));

    let path = "/file.js";
    let result = plugin.evaluate(parsed.syntax().into(), path.into(), &bag);
    let messages: Vec<_> = result
        .entries
        .iter()
        .map(|entry| PrintDescription(&entry.diagnostic).to_string())
        .collect();

    assert_eq!(messages, ["value:1"]);

    let rendered = render_diagnostics(path, source, result);
    snap_diagnostics(
        "grit_can_request_the_semantic_model",
        "/plugin.js",
        plugin_source,
        &[(path, source)],
        rendered,
    );
}

#[test]
fn invalid_grit_pattern_is_rejected() {
    let plugin_source = r#"import { defineRule, grit } from "@biomejs/runtime/plugin";
        export const rule = defineRule({
            query: grit("`foo()"),
            run() {},
        });"#;

    let fs = MemoryFileSystem::default();
    fs.insert("/plugin.js".into(), plugin_source);

    let error = AnalyzerJsPlugin::load(
        Arc::new(fs) as Arc<dyn FsWithResolverProxy>,
        "/plugin.js".into(),
        None,
    )
    .expect_err("Grit rewrites cannot select nodes");
    let output = print_diagnostic_to_string(&Error::from(error));

    assert!(
        output.contains("Missing closing quote"),
        "unexpected error: {output}"
    );
    snap_diagnostics(
        "grit_invalid_pattern_is_rejected",
        "/plugin.js",
        plugin_source,
        &[],
        output,
    );
}

#[test]
fn invalid_grit_rewrite_is_rejected() {
    let plugin_source = r#"import { defineRule, grit } from "@biomejs/runtime/plugin";
        export const rule = defineRule({
            query: grit("`foo()` => `bar()`"),
            run() {},
        });"#;

    let fs = MemoryFileSystem::default();
    fs.insert("/plugin.js".into(), plugin_source);

    let error = AnalyzerJsPlugin::load(
        Arc::new(fs) as Arc<dyn FsWithResolverProxy>,
        "/plugin.js".into(),
        None,
    )
    .expect_err("Grit rewrites cannot select nodes");
    let output = print_diagnostic_to_string(&Error::from(error));

    assert!(
        output.contains("cannot contain rewrites"),
        "unexpected error: {output}"
    );
    snap_diagnostics(
        "grit_rewrite_pattern_is_rejected",
        "/plugin.js",
        plugin_source,
        &[],
        output,
    );
}
