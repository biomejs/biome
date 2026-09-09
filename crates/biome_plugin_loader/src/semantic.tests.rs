use super::tests::{load_test_plugin_from_source, render_diagnostics, services, snap_diagnostics};
use super::*;
use biome_analyze::{AnalysisFilter, AnalyzerOptions, ControlFlow, Never};
use biome_diagnostics::PrintDescription;
use biome_js_analyze::JsAnalyzerServices;
use biome_js_parser::{JsParserOptions, parse};
use biome_js_semantic::{SemanticModel, SemanticModelOptions, semantic_model};
use biome_js_syntax::JsSyntaxKind;
use biome_rowan::AstNode;
use serde_json::{Value, json};

fn native_summary(model: &SemanticModel) -> Value {
    let mut exported = model
        .all_exported_bindings()
        .map(|binding| binding.syntax().text_trimmed().to_string())
        .collect::<Vec<_>>();
    exported.sort();
    let reference = |reference: biome_js_semantic::Reference| {
        json!([
            format!("{:?}", reference.syntax().kind()),
            reference.syntax().text_trimmed().to_string(),
            reference.is_read(),
            reference.is_write(),
            reference
                .binding()
                .map(|binding| binding.syntax().text_trimmed().to_string()),
        ])
    };
    json!({
        "bindings": model.all_bindings().map(|binding| json!([
            format!("{:?}", binding.syntax().kind()), binding.syntax().text_trimmed().to_string(),
            binding.all_references().map(reference).collect::<Vec<_>>(),
            binding.all_reads().map(reference).collect::<Vec<_>>(),
            binding.all_writes().map(reference).collect::<Vec<_>>(),
            binding.is_imported(), binding.is_exported(),
            binding.is_imported(), model.is_exported(&binding.tree()),
            format!("{:?}", binding.declaration_kind()).char_indices()
                .map(|(index, ch)| if index == 0 { ch.to_ascii_lowercase() } else { ch }).collect::<String>(),
            binding.exports().map(|node| json!([
                format!("{:?}", node.kind()), node.text_trimmed().to_string(),
            ])).collect::<Vec<_>>(),
            binding.export_ranges().iter().map(|range| json!({
                "start": u32::from(range.start()), "end": u32::from(range.end()),
            })).collect::<Vec<_>>(),
        ])).collect::<Vec<_>>(),
        "exported": exported,
        "hasExports": model.has_exports(),
        "globals": model.all_global_references().map(|reference| json!([
            format!("{:?}", reference.syntax().kind()), reference.syntax().text_trimmed().to_string(),
            reference.is_read(), reference.is_write(),
        ])).collect::<Vec<_>>(),
        "unresolved": model.all_unresolved_references().map(|reference| json!([
            format!("{:?}", reference.syntax().kind()), reference.syntax().text_trimmed().to_string(),
            model.is_unresolved_reference(&reference.tree()),
        ])).collect::<Vec<_>>(),
    })
}

#[test]
fn semantic_results_match_the_native_model() {
    let plugin_source = r#"import { defineRule, semantic, registerDiagnostic } from "@biomejs/runtime/plugin";
        export const inspect = defineRule({
            query: semantic("JS_MODULE", "TS_DECLARATION_MODULE"),
            run(root, { model }) {
                const reference = ref => [ref.syntax().kind, ref.syntax().text,
                    ref.isRead(), ref.isWrite(), ref.binding()?.syntax().text];
                registerDiagnostic(root, "information", JSON.stringify({
                    bindings: model.allBindings().map(binding => {
                        const node = binding.syntax();
                        const found = model.asBinding(node);
                        return [found.syntax().kind, found.syntax().text,
                            binding.allReferences().map(reference),
                            binding.allReads().map(reference), binding.allWrites().map(reference),
                            binding.isImported(), binding.isExported(),
                            model.isImported(node), model.isExported(node),
                            binding.declarationKind(),
                            binding.exports().map(site => [site.kind, site.text]),
                            binding.exportRanges()];
                    }),
                    exported: model.allExportedBindings().map(binding => binding.syntax().text).sort(),
                    hasExports: model.hasExports(),
                    globals: model.allGlobalReferences().map(ref => [ref.syntax().kind,
                        ref.syntax().text, ref.isRead(), ref.isWrite()]),
                    unresolved: model.allUnresolvedReferences().map(ref => [ref.syntax().kind,
                        ref.syntax().text, model.isUnresolvedReference(ref.syntax())]),
                }, 0, 2));
                const nodes = root.children().flatMap(function visit(node) {
                    return [node, ...node.children().flatMap(visit)];
                });
                for (const node of nodes) {
                    if (!["JS_REFERENCE_IDENTIFIER", "JS_IDENTIFIER_ASSIGNMENT", "JSX_REFERENCE_IDENTIFIER"].includes(node.kind)) continue;
                    registerDiagnostic(node, "information", JSON.stringify([
                        model.binding(node)?.syntax().text ?? null,
                        model.isImported(node), model.isExported(node),
                        model.isGlobalReference(node), model.isUnresolvedReference(node),
                    ], 0, 2));
                }
            },
        });"#;
    let plugin = load_test_plugin_from_source("/plugin.js", plugin_source, None);
    let mut inputs = Vec::new();
    let mut rendered = String::new();
    for (path, source, source_type) in [
        (
            "/file.js",
            "import { external as imported } from 'pkg'; export let outer = 1; function f(outer) { outer = 2; return outer; } outer++; imported; configured; configured = 1; missing;",
            JsFileSource::js_module(),
        ),
        (
            "/file.ts",
            "import dependency = require('pkg'); import type TypeDependency = require('types'); interface T {} type U<T> = T; let value: T; value; dependency; type D = TypeDependency;",
            JsFileSource::ts(),
        ),
        (
            "/file.jsx",
            "const Café = () => null; let café = 1; café; <Café />; <Missing />;",
            JsFileSource::jsx(),
        ),
        (
            "/file.d.ts",
            "declare const value: number;",
            JsFileSource::d_ts(),
        ),
        (
            "/malformed.js",
            "let broken = ; broken;",
            JsFileSource::js_module(),
        ),
        (
            "/exports.js",
            "export const café = 1; const local = 2; export { local as renamed, café as renamedCafé, local as again }; export default local; export * from 'pkg'; export { remote } from 'other';",
            JsFileSource::js_module(),
        ),
        (
            "/reexports.js",
            "export * from 'pkg'; export { remote as alias } from 'other'; export default 42; export {};",
            JsFileSource::js_module(),
        ),
        (
            "/kinds.ts",
            "var hoisted; using resource = acquire(); export class C {} export enum E { Member } namespace N { export const x = 1; } module M {} export type Alias<T> = T; declare module 'pkg' { export const external: number; }",
            JsFileSource::ts(),
        ),
    ] {
        let parsed = parse(source, source_type, JsParserOptions::default());
        let model = semantic_model(
            &parsed.tree(),
            SemanticModelOptions {
                globals: ["configured".to_string()].into_iter().collect(),
                ..SemanticModelOptions::from(&source_type)
            },
        );
        let mut bag = services(source_type);
        bag.insert_service(model.clone());
        let result = plugin.evaluate(parsed.syntax().into(), path.into(), &bag);
        let (summary, references) = result
            .entries
            .split_first()
            .expect("expected semantic output");
        let actual = PrintDescription(&summary.diagnostic).to_string();
        assert_eq!(
            serde_json::from_str::<Value>(&actual).expect(&actual),
            native_summary(&model),
            "{source}"
        );
        let expected: Vec<_> = parsed
            .syntax()
            .descendants()
            .filter_map(biome_js_syntax::AnyJsIdentifierReference::cast)
            .map(|node| {
                json!([
                    model
                        .binding(&node)
                        .map(|binding| binding.syntax().text_trimmed().to_string()),
                    model
                        .binding(&node)
                        .is_some_and(|binding| binding.is_imported()),
                    model.is_exported(&node).unwrap_or(false),
                    model.is_global_reference(&node),
                    model.is_unresolved_reference(&node),
                ])
            })
            .collect();
        let actual: Vec<Value> = references
            .iter()
            .map(|entry| {
                let message = PrintDescription(&entry.diagnostic).to_string();
                serde_json::from_str(&message).expect(&message)
            })
            .collect();
        assert_eq!(actual, expected, "{source}");
        inputs.push((path, source));
        rendered.push_str(&render_diagnostics(path, source, result));
    }
    snap_diagnostics(
        "semantic_model",
        "/plugin.js",
        plugin_source,
        &inputs,
        rendered,
    );
}

#[test]
fn semantic_rules_use_the_supplied_model_and_syntax_rules_keep_their_context() {
    let plugin_source = r#"import { ast, createMutation, defineRule, factory, semantic, registerDiagnostic } from "@biomejs/runtime/plugin";
        export const aSyntax = defineRule({
            query: ast("JS_REFERENCE_IDENTIFIER"),
            run(node, context) {
                registerDiagnostic(node, "information", `syntax:${"model" in context}`);
            },
        });
        export const bSemantic = defineRule({
            query: semantic("JS_REFERENCE_IDENTIFIER"),
            run(node, context) {
                const binding = context.model.binding(node);
                const reference = binding.allReads()[0].syntax();
                const mutation = createMutation(binding.syntax());
                mutation.replaceToken(reference.token("valueToken"), factory.token("IDENT", "renamed"));
                registerDiagnostic(reference, "information",
                    `semantic:${binding.syntax().text}:${binding.allReads().length}:${Reflect.set(context, "model", null)}`,
                    { mutation, message: "Rename the reference.", kind: "unsafe" });
            },
        });"#;
    let plugin = load_test_plugin_from_source("/plugin.js", plugin_source, None);
    let source_type = JsFileSource::js_module();
    let parsed = parse(
        "let value = 1; value;",
        source_type,
        JsParserOptions::default(),
    );
    let model = semantic_model(&parsed.tree(), SemanticModelOptions::from(&source_type));
    let options = AnalyzerOptions::default();
    let plugins: Vec<Arc<Box<dyn AnalyzerPlugin>>> = vec![Arc::new(Box::new(plugin))];
    let mut messages = Vec::new();
    let (_, diagnostics) = biome_js_analyze::analyze(
        &parsed.tree(),
        AnalysisFilter {
            enabled_rules: Some(&[]),
            ..AnalysisFilter::default()
        },
        &options,
        &plugins,
        JsAnalyzerServices::default()
            .with_source_type(source_type)
            .with_semantic_model(&model),
        |signal| {
            if let Some(diagnostic) = signal.diagnostic() {
                messages.push(PrintDescription(&diagnostic).to_string());
            }
            ControlFlow::<Never>::Continue(())
        },
    );
    assert!(diagnostics.is_empty(), "{diagnostics:?}");
    messages.sort();
    assert_eq!(messages, ["semantic:value:1:false", "syntax:false"]);

    let node = parsed
        .syntax()
        .descendants()
        .find(|node| node.kind() == JsSyntaxKind::JS_REFERENCE_IDENTIFIER)
        .unwrap();
    let mut bag = services(source_type);
    bag.insert_service(model);
    let result = plugins[0].evaluate(node.clone().into(), "/file.js".into(), &bag);
    let actions: Vec<_> = result
        .entries
        .iter()
        .filter_map(|entry| entry.action.as_ref())
        .collect();
    let [action] = actions.as_slice() else {
        panic!("expected one fix: {result:?}")
    };
    assert_eq!(
        action.text_edit.new_string("let value = 1; value;"),
        "let value = 1; renamed;"
    );
    let mut rendered = render_diagnostics("/file.js", "let value = 1; value;", result);
    let result = plugins[0].evaluate(node.into(), "/file.js".into(), &services(source_type));
    let messages: Vec<_> = result
        .entries
        .iter()
        .map(|entry| PrintDescription(&entry.diagnostic).to_string())
        .collect();
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0], "syntax:false");
    assert!(
        messages[1].contains("The semantic model is unavailable."),
        "{messages:?}"
    );
    rendered.push_str(&render_diagnostics(
        "/file.js",
        "let value = 1; value;",
        result,
    ));
    snap_diagnostics(
        "semantic_model_context_and_fixes",
        "/plugin.js",
        plugin_source,
        &[("/file.js", "let value = 1; value;")],
        rendered,
    );
}

#[test]
fn semantic_handles_retain_their_source() {
    let plugin_source = r#"import { defineRule, semantic, registerDiagnostic } from "@biomejs/runtime/plugin";
        let saved;
        export const retained = defineRule({
            query: semantic("JS_MODULE"),
            run(root, { model }) {
                const bindings = model.allBindings();
                const binding = bindings[0];
                bindings.length = 0;
                if (saved) {
                    const node = saved.reference.syntax();
                    registerDiagnostic(root, "information", JSON.stringify([
                        binding.syntax().text, saved.model.binding(node).syntax().text,
                        saved.binding.allReads().length, model.allBindings().length,
                        saved.scope.bindings().map(binding => binding.syntax().text),
                    ], 0, 2));
                    model.binding(node);
                } else {
                    saved = { model, binding, reference: binding.allReads()[0], scope: model.globalScope() };
                }
            },
        });"#;
    let plugin = load_test_plugin_from_source("/plugin.js", plugin_source, None);
    let source_type = JsFileSource::js_module();
    let inputs = [
        ("/file.js", "let first = 1; first;"),
        ("/file.js", "let second = 2; second;"),
    ];
    let mut rendered = String::new();
    for (index, (path, source)) in inputs.into_iter().enumerate() {
        let parsed = parse(source, source_type, JsParserOptions::default());
        let mut bag = services(source_type);
        bag.insert_service(semantic_model(
            &parsed.tree(),
            SemanticModelOptions::default(),
        ));
        let result = plugin.evaluate(parsed.syntax().into(), "/file.js".into(), &bag);
        if index == 0 {
            assert!(result.entries.is_empty());
        } else {
            let [retained, error] = result.entries.as_slice() else {
                panic!("{result:?}")
            };
            assert_eq!(
                serde_json::from_str::<Value>(&PrintDescription(&retained.diagnostic).to_string())
                    .unwrap(),
                json!(["second", "first", 1, 1, ["first"]])
            );
            assert!(
                PrintDescription(&error.diagnostic)
                    .to_string()
                    .contains("The node belongs to a different source")
            );
        }
        rendered.push_str(&render_diagnostics(path, source, result));
    }
    snap_diagnostics(
        "semantic_model_retained_handles",
        "/plugin.js",
        plugin_source,
        &inputs,
        rendered,
    );
}

#[test]
fn semantic_scopes_preserve_nesting_bindings_and_hoisting() {
    // Boa parses this fixture on the test thread's stack; avoid deeply nested expressions.
    let plugin_source = r#"import { defineRule, semantic, registerDiagnostic } from "@biomejs/runtime/plugin";
        function describe(scope) {
            if (!scope) return null;
            const names = [];
            for (const binding of scope.bindings()) {
                names.push(binding.syntax().text);
            }
            return [scope.syntax().kind, names];
        }
        export const scopes = defineRule({
            query: semantic("JS_MODULE"),
            run(root, { model }) {
                const scopes = model.scopes();
                const global = model.globalScope();
                const children = global.children();
                children.length = 0;
                const scopeDetails = [];
                for (const scope of scopes) {
                    const lookup = [];
                    for (const name of ["outer", "value", "hoisted", "café", "caf\\u00e9", "Value", "missing"]) {
                        const binding = scope.getBinding(name);
                        const node = binding?.syntax();
                        lookup.push([name, node ? [node.kind, node.text] : null]);
                    }
                    const detail = {
                        scope: describe(scope),
                        global: scope.isGlobalScope(),
                        parent: describe(scope.parent()),
                        ancestors: scope.ancestors().map(describe),
                        lookup,
                    };
                    scopeDetails.push(detail);
                }
                const bindingDetails = [];
                for (const binding of model.allBindings()) {
                    const references = [];
                    for (const reference of binding.allReferences()) {
                        const detail = {
                            scope: describe(reference.scope()),
                            lookup: describe(model.scope(reference.syntax())),
                        };
                        references.push(detail);
                    }
                    const detail = {
                        name: binding.syntax().text,
                        scope: describe(binding.scope()),
                        hoisted: describe(model.scopeHoistedTo(binding.syntax())),
                        references,
                    };
                    bindingDetails.push(detail);
                }
                const result = {
                    global: describe(global),
                    rootScope: describe(model.scope(root)),
                    children: global.children().map(describe),
                    scopes: scopeDetails,
                    bindings: bindingDetails,
                };
                registerDiagnostic(root, "information", JSON.stringify(result, 0, 2));
            },
        });"#;
    let plugin = load_test_plugin_from_source("/plugin.js", plugin_source, None);
    let mut inputs = Vec::new();
    let mut rendered = String::new();
    for (path, source, source_type) in [
        (
            "/scopes.js",
            "const outer = 0; { let value = 1; function inner(param) { { var hoisted = param; let café = hoisted; café; outer; } } }",
            JsFileSource::js_module(),
        ),
        (
            "/scopes.ts",
            "interface Value {} const Value = 0; { type Local = Value; const café = 1; café; }",
            JsFileSource::ts(),
        ),
        ("/empty.js", "", JsFileSource::js_module()),
        ("/comments.js", "/* scope */", JsFileSource::js_module()),
        (
            "/malformed.js",
            "function broken( {",
            JsFileSource::js_module(),
        ),
    ] {
        let parsed = parse(source, source_type, JsParserOptions::default());
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::from(&source_type));
        let mut bag = services(source_type);
        bag.insert_service(model);
        let result = plugin.evaluate(parsed.syntax().into(), path.into(), &bag);
        let [entry] = result.entries.as_slice() else {
            panic!("{path}: {result:?}")
        };
        let message = PrintDescription(&entry.diagnostic).to_string();
        let value: Value = serde_json::from_str(&message).expect(&message);
        if parsed.syntax().text_trimmed_range().is_empty() {
            assert_eq!(value["rootScope"], Value::Null);
            assert_eq!(value["global"], json!(["JS_MODULE", []]));
        }
        inputs.push((path, source));
        rendered.push_str(&render_diagnostics(path, source, result));
    }
    snap_diagnostics(
        "semantic_model_scopes",
        "/plugin.js",
        plugin_source,
        &inputs,
        rendered,
    );
}
