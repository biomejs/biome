use super::tests::{load_test_plugin_from_source, render_diagnostics, services, snap_diagnostics};
use super::*;
use biome_diagnostics::{Error, PrintDescription, print_diagnostic_to_string};
use biome_fs::{BiomePath, MemoryFileSystem};
use biome_js_parser::{JsParserOptions, parse};
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_js_syntax::JsSyntaxKind;
use biome_languages::JsFileSource;
use biome_module_graph::{
    ModuleInfo, ModuleInfoKind, TypeInferenceMode, resolve_js_module_with_inference_mode,
};
use biome_rowan::AstNode;
use biome_service::db::WorkspaceDb;
use std::sync::Arc;

fn services_with_mode(
    path: &str,
    source: &str,
    source_type: JsFileSource,
    inference_mode: TypeInferenceMode,
) -> ServiceBag {
    let fs = MemoryFileSystem::default();
    fs.insert(path.into(), source);
    let db = WorkspaceDb::new(Arc::new(fs));

    let parsed = parse(source, source_type, JsParserOptions::default());
    let root = parsed.tree();
    let semantic = Arc::new(semantic_model(
        &root,
        SemanticModelOptions::from(&source_type),
    ));
    let (info, _, _) = resolve_js_module_with_inference_mode(
        &db,
        root,
        &BiomePath::new(path),
        Arc::clone(&semantic),
        inference_mode,
    );
    let module = ModuleInfo::new(&db, path.into(), ModuleInfoKind::Js(info));
    db.modules.pin().insert(path.into(), module);

    let mut services = ServiceBag::default();
    services.insert_service(source_type);
    services.insert_service(db.rc_module_db());
    services.insert_service((*semantic).clone());
    services
}

fn typed_services(path: &str, source: &str, source_type: JsFileSource) -> ServiceBag {
    services_with_mode(path, source, source_type, TypeInferenceMode::RawTypesOnly)
}

fn analyze_root(
    plugin: &AnalyzerJsPlugin,
    path: &str,
    source: &str,
    bag: &ServiceBag,
) -> Vec<String> {
    let parsed = parse(source, JsFileSource::ts(), JsParserOptions::default());
    plugin
        .evaluate(parsed.syntax().into(), path.into(), bag)
        .entries
        .iter()
        .map(|entry| PrintDescription(&entry.diagnostic).to_string())
        .collect()
}

fn evaluate_nodes(
    plugin: &AnalyzerJsPlugin,
    path: &str,
    source: &str,
    bag: &ServiceBag,
    kind: JsSyntaxKind,
) -> PluginEvalResult {
    let parsed = parse(source, JsFileSource::ts(), JsParserOptions::default());
    let mut entries = Vec::new();
    for node in parsed
        .syntax()
        .descendants()
        .filter(|node| node.kind() == kind)
    {
        entries.extend(plugin.evaluate(node.into(), path.into(), bag).entries);
    }
    PluginEvalResult { entries }
}

fn analyze_nodes(
    plugin: &AnalyzerJsPlugin,
    path: &str,
    source: &str,
    bag: &ServiceBag,
    kind: JsSyntaxKind,
) -> Vec<String> {
    evaluate_nodes(plugin, path, source, bag, kind)
        .entries
        .iter()
        .map(|entry| PrintDescription(&entry.diagnostic).to_string())
        .collect()
}

fn evaluate_root(
    plugin: &AnalyzerJsPlugin,
    path: &str,
    source: &str,
    bag: &ServiceBag,
) -> PluginEvalResult {
    let parsed = parse(source, JsFileSource::ts(), JsParserOptions::default());
    plugin.evaluate(parsed.syntax().into(), path.into(), bag)
}

#[test]
fn ast_types_option_provides_type_information() {
    let plugin = load_test_plugin_from_source(
        "/plugin.js",
        r#"import { ast, defineRule, registerDiagnostic } from "@biomejs/runtime/plugin";
        export const rule = defineRule({
            query: ast("JS_IDENTIFIER_EXPRESSION", { types: true }),
            run(node, context) {
                const ty = context.types.typeOf(node);
                registerDiagnostic(node, "information",
                    `${ty?.kind()}:${ty?.isStringLike()}:${"model" in context}`);
            },
        });"#,
        None,
    );

    assert_eq!(
        analyze_nodes(
            &plugin,
            "/file.ts",
            "const value = \"hello\"; value;",
            &typed_services(
                "/file.ts",
                "const value = \"hello\"; value;",
                JsFileSource::ts()
            ),
            JsSyntaxKind::JS_IDENTIFIER_EXPRESSION,
        ),
        ["string:true:true"]
    );
}

#[test]
fn grit_types_option_provides_type_information() {
    let plugin = load_test_plugin_from_source(
        "/plugin.js",
        r#"import { defineRule, grit, registerDiagnostic } from "@biomejs/runtime/plugin";
        export const rule = defineRule({
            query: grit("`value + 1`", { types: true }),
            run(node, context) {
                const ty = context.types.typeOf(node);
                registerDiagnostic(node, "information", `${ty?.kind()}:${ty?.isStringLike()}`);
            },
        });"#,
        None,
    );

    assert_eq!(
        analyze_root(
            &plugin,
            "/file.ts",
            "const value = \"hello\"; value + 1;",
            &typed_services(
                "/file.ts",
                "const value = \"hello\"; value + 1;",
                JsFileSource::ts()
            ),
        ),
        ["string:true"]
    );
}

#[test]
fn type_service_queries_match_native_inference() {
    let plugin = load_test_plugin_from_source(
        "/plugin.js",
        r#"import { ast, defineRule, registerDiagnostic } from "@biomejs/runtime/plugin";
        export const rule = defineRule({
            query: ast("JS_CALL_EXPRESSION", { types: true }),
            run(node, context) {
                registerDiagnostic(node, "information", [
                    context.types.typeOf(node)?.kind(),
                    context.types.typeOfNamed(node, "result")?.kind(),
                    context.types.expectedArgumentType(node, 0)?.kind(),
                    context.types.classifiesAs(node, "promise"),
                ].join(":"));
            },
        });"#,
        None,
    );

    let source =
        "function schedule(task: () => void): void { task(); } const result = schedule(() => {});";
    assert_eq!(
        analyze_nodes(
            &plugin,
            "/file.ts",
            source,
            &typed_services("/file.ts", source, JsFileSource::ts()),
            JsSyntaxKind::JS_CALL_EXPRESSION,
        ),
        [
            "undefined:undefined::noMatch",
            "undefined:undefined:function:noMatch"
        ]
    );
}

#[test]
fn return_type_of_function_is_available() {
    let plugin = load_test_plugin_from_source(
        "/plugin.js",
        r#"import { ast, defineRule, registerDiagnostic } from "@biomejs/runtime/plugin";
        export const rule = defineRule({
            query: ast("JS_FUNCTION_DECLARATION", { types: true }),
            run(node, context) {
                registerDiagnostic(node, "information",
                    context.types.returnTypeOf(node)?.kind() ?? "missing");
            },
        });"#,
        None,
    );

    let source = "function greet(name: string): string { return name; }";
    assert_eq!(
        analyze_nodes(
            &plugin,
            "/file.ts",
            source,
            &typed_services("/file.ts", source, JsFileSource::ts()),
            JsSyntaxKind::JS_FUNCTION_DECLARATION,
        ),
        ["string"]
    );
}

const TYPE_ERROR_PLUGIN_SOURCE: &str = r#"import { ast, defineRule } from "@biomejs/runtime/plugin";
        export const rule = defineRule({
            query: ast("JS_IDENTIFIER_EXPRESSION", { types: true }),
            run() {},
        });"#;

fn render_type_error(bag: &ServiceBag) -> String {
    let plugin = load_test_plugin_from_source("/plugin.js", TYPE_ERROR_PLUGIN_SOURCE, None);

    let parsed = parse(
        "const value = 1; value;",
        JsFileSource::ts(),
        JsParserOptions::default(),
    );
    let root = parsed.tree();
    let node = root
        .syntax()
        .descendants()
        .find(|node| node.kind() == JsSyntaxKind::JS_IDENTIFIER_EXPRESSION)
        .expect("expected an identifier expression");
    let result = plugin.evaluate(node.into(), "/file.ts".into(), bag);
    let [entry] = result.entries.as_slice() else {
        panic!("{result:?}");
    };
    print_diagnostic_to_string(&Error::from(entry.diagnostic.clone()))
}

fn assert_type_error_points_to_the_types_domain(rendered: &str) {
    assert!(
        rendered.contains("Type information is unavailable for this file"),
        "{rendered}"
    );
    assert!(rendered.contains("linter.domains.types"), "{rendered}");
}

#[test]
fn classify_promise_call() {
    let plugin = load_test_plugin_from_source(
        "/plugin.js",
        r#"import { ast, defineRule, registerDiagnostic } from "@biomejs/runtime/plugin";
        export const rule = defineRule({
            query: ast("JS_CALL_EXPRESSION", { types: true }),
            run(node, context) {
                registerDiagnostic(node, "information",
                    context.types.classifiesAs(node, "promise"));
            },
        });"#,
        None,
    );

    let source = "async function load(): Promise<string> { return \"\"; } load();";
    assert_eq!(
        analyze_nodes(
            &plugin,
            "/file.ts",
            source,
            &typed_services("/file.ts", source, JsFileSource::ts()),
            JsSyntaxKind::JS_CALL_EXPRESSION,
        ),
        ["match"]
    );
}

#[test]
fn return_type_of_member_is_available() {
    let plugin = load_test_plugin_from_source(
        "/plugin.js",
        r#"import { ast, defineRule, registerDiagnostic } from "@biomejs/runtime/plugin";
        export const rule = defineRule({
            query: ast("JS_METHOD_OBJECT_MEMBER", { types: true }),
            run(node, context) {
                registerDiagnostic(node, "information",
                    context.types.returnTypeOfMember(node, "greet")?.kind() ?? "missing");
            },
        });"#,
        None,
    );

    let source = "const obj = { greet(name: string): string { return name; } };";
    assert_eq!(
        analyze_nodes(
            &plugin,
            "/file.ts",
            source,
            &typed_services("/file.ts", source, JsFileSource::ts()),
            JsSyntaxKind::JS_METHOD_OBJECT_MEMBER,
        ),
        ["string"]
    );
}

#[test]
fn missing_module_graph_points_to_the_types_domain() {
    let source_type = JsFileSource::ts();
    let parsed = parse(
        "const value = 1; value;",
        source_type,
        JsParserOptions::default(),
    );
    let root = parsed.tree();
    let mut bag = services(source_type);
    bag.insert_service(semantic_model(
        &root,
        SemanticModelOptions::from(&source_type),
    ));
    assert_type_error_points_to_the_types_domain(&render_type_error(&bag));
}

#[test]
fn disabled_type_inference_points_to_the_types_domain() {
    let source = "const value = 1; value;";
    let bag = services_with_mode(
        "/file.ts",
        source,
        JsFileSource::ts(),
        TypeInferenceMode::Disabled,
    );
    assert_type_error_points_to_the_types_domain(&render_type_error(&bag));
}

#[test]
fn types_option_requires_the_semantic_model() {
    let plugin = load_test_plugin_from_source(
        "/plugin.js",
        r#"import { ast, defineRule } from "@biomejs/runtime/plugin";
        export const rule = defineRule({
            query: ast("JS_IDENTIFIER_EXPRESSION", { types: true }),
            run() {},
        });"#,
        None,
    );

    assert!(plugin.requires_semantic_model());
}

#[test]
fn types_ast_query_snapshot() {
    let plugin_source = r#"import { ast, defineRule, registerDiagnostic } from "@biomejs/runtime/plugin";
        export const rule = defineRule({
            query: ast("JS_VARIABLE_DECLARATOR", { types: true }),
            run(node, { types }) {
                const expression = node.initializer?.expression;
                const ty = expression && types.typeOf(expression);
                registerDiagnostic(node, "information", [
                    ty?.toString(),
                    ty?.kind(),
                    `string=${ty?.isStringLike()}`,
                    `promise=${String(ty?.isPromise())}`,
                ].join(" | "));
            },
        });"#;
    let plugin = load_test_plugin_from_source("/plugin.js", plugin_source, None);

    let source = "const text = \"hello\";\nconst amount = 42;\nconst values = [1, 2, 3];\nconst pending = Promise.resolve(\"done\");";
    let bag = typed_services("/file.ts", source, JsFileSource::ts());
    let result = evaluate_nodes(
        &plugin,
        "/file.ts",
        source,
        &bag,
        JsSyntaxKind::JS_VARIABLE_DECLARATOR,
    );
    snap_diagnostics(
        "types_ast_query",
        "/plugin.js",
        plugin_source,
        &[("/file.ts", source)],
        render_diagnostics("/file.ts", source, result),
    );
}

#[test]
fn types_grit_query_snapshot() {
    let plugin_source = r#"import { defineRule, grit, registerDiagnostic } from "@biomejs/runtime/plugin";
        export const rule = defineRule({
            query: grit("`load()`", { types: true }),
            run(node, { types }) {
                const ty = types.typeOf(node);
                registerDiagnostic(node, "information", [
                    ty?.toString(),
                    ty?.kind(),
                    types.classifiesAs(node, "promise"),
                ].join(" | "));
            },
        });"#;
    let plugin = load_test_plugin_from_source("/plugin.js", plugin_source, None);

    let source = "async function load(): Promise<string> { return \"done\"; }\nload();";
    let bag = typed_services("/file.ts", source, JsFileSource::ts());
    let result = evaluate_root(&plugin, "/file.ts", source, &bag);
    snap_diagnostics(
        "types_grit_query",
        "/plugin.js",
        plugin_source,
        &[("/file.ts", source)],
        render_diagnostics("/file.ts", source, result),
    );
}

#[test]
fn types_unavailable_snapshot() {
    let source = "const value = 1; value;";
    let bag = services_with_mode(
        "/file.ts",
        source,
        JsFileSource::ts(),
        TypeInferenceMode::Disabled,
    );
    let rendered = render_type_error(&bag);
    assert_type_error_points_to_the_types_domain(&rendered);
    snap_diagnostics(
        "types_unavailable",
        "/plugin.js",
        TYPE_ERROR_PLUGIN_SOURCE,
        &[("/file.ts", source)],
        rendered,
    );
}
