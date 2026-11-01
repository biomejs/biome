use super::tests::{load_test_plugin_from_source, render_diagnostics, services, snap_diagnostics};
use super::*;
use biome_diagnostics::PrintDescription;
use biome_fs::MemoryFileSystem;
use biome_js_parser::{JsParserOptions, parse};

fn evaluate(plugin: &AnalyzerJsPlugin, source: &str) -> PluginEvalResult {
    let parse = parse(
        source,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );
    assert!(!parse.has_errors(), "{source}");
    plugin.evaluate(
        parse.syntax().into(),
        "/file.js".into(),
        &services(JsFileSource::js_module()),
    )
}

fn messages(result: &PluginEvalResult) -> Vec<String> {
    result
        .entries
        .iter()
        .map(|entry| PrintDescription(&entry.diagnostic).to_string())
        .collect()
}

#[test]
fn kind_sets_match_nodes_and_tokens() {
    let plugin_source = r#"import { ast, defineRule, nodeKinds, registerDiagnostic, tokenKinds } from "@biomejs/runtime/plugin";
        const CALLS = nodeKinds("JS_CALL_EXPRESSION", "JS_NEW_EXPRESSION");
        const LOOSE_EQUALITY = tokenKinds("EQ2", "NEQ");
        export const kindSets = defineRule({
            query: ast("JS_EXPRESSION_STATEMENT"),
            run(statement) {
                const expression = statement.expression;
                if (CALLS.has(expression)) {
                    registerDiagnostic(expression, "information", `${expression.kind} is in CALLS.`);
                }
                for (const element of expression.childrenWithTokens()) {
                    if (LOOSE_EQUALITY.has(element)) {
                        registerDiagnostic(expression, "warning", `${element.text} is in LOOSE_EQUALITY.`);
                    }
                }
            },
        });"#;
    let source = "foo();\nnew Bar();\na == b;\nc === d;\ne != f;\n";
    let plugin = load_test_plugin_from_source("/plugin.js", plugin_source, None);
    let parse = parse(
        source,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );
    let mut diagnostics = String::new();
    for statement in parse.syntax().descendants() {
        if statement.kind() == biome_js_syntax::JsSyntaxKind::JS_EXPRESSION_STATEMENT {
            let result = plugin.evaluate(
                statement.into(),
                "/file.js".into(),
                &services(JsFileSource::js_module()),
            );
            diagnostics.push_str(&render_diagnostics("/file.js", source, result));
        }
    }

    snap_diagnostics(
        "kind_sets_match_nodes_and_tokens",
        "/plugin.js",
        plugin_source,
        &[("/file.js", source)],
        diagnostics,
    );
}

#[test]
fn kind_set_membership_contract() {
    let plugin = load_test_plugin_from_source(
        "/plugin.js",
        r#"import { ast, defineRule, nodeKinds, registerDiagnostic, tokenKinds } from "@biomejs/runtime/plugin";
        const NODES = nodeKinds("JS_MODULE", "JS_IDENTIFIER_BINDING", "JS_MODULE_ITEM_LIST", "JS_BOGUS");
        const TOKENS = tokenKinds("LET_KW", "SEMICOLON", "EOF");
        const EMPTY = nodeKinds();
        const assert = (condition, message) => {
            if (!condition) throw new Error(message);
        };
        const throwsTypeError = (callback, expected) => {
            try {
                callback();
            } catch (error) {
                assert(error instanceof TypeError, `${expected}: ${error}`);
                assert(error.message.includes(expected), `${expected}: ${error.message}`);
                return;
            }
            throw new Error(`Expected a TypeError: ${expected}`);
        };
        export const contract = defineRule({
            query: ast("JS_MODULE"),
            run(root) {
                const statement = root.items[0];
                const declaration = statement.declaration;
                const keyword = declaration.token("kindToken");
                const binding = declaration.declarators[0].id;
                const semicolon = statement.token("semicolonToken");

                assert(NODES.has(root), "root");
                assert(NODES.has(statement.parent), "list container");
                assert(NODES.has(binding), "binding");
                assert(!NODES.has(statement), "statement");
                assert(!NODES.has(keyword), "token in a node set");
                assert(!NODES.has(undefined), "undefined");
                assert(TOKENS.has(keyword), "keyword");
                assert(TOKENS.has(semicolon), "semicolon");
                assert(!TOKENS.has(root), "node in a token set");
                assert(TOKENS.has(root.token("eofToken")), "end of file");
                assert(!EMPTY.has(root), "empty set");
                assert(!EMPTY.has(keyword), "empty set with a token");

                const prototype = Object.getPrototypeOf(NODES);
                assert(prototype === Object.getPrototypeOf(EMPTY), "shared node set prototype");
                assert(prototype !== Object.getPrototypeOf(TOKENS), "separate token set prototype");
                assert(Object.keys(NODES).length === 0, "no enumerable node set state");
                assert(Object.keys(TOKENS).length === 0, "no enumerable token set state");
                throwsTypeError(() => new prototype.constructor(), "Node kind sets do not have a public constructor.");
                throwsTypeError(() => new (Object.getPrototypeOf(TOKENS).constructor)(), "Token kind sets do not have a public constructor.");
                throwsTypeError(() => NODES.has(), "set.has() received the wrong number of arguments.");
                throwsTypeError(() => NODES.has(root, root), "set.has() received the wrong number of arguments.");
                throwsTypeError(() => NODES.has("JS_MODULE"), "set.has() requires a Biome node or token");
                throwsTypeError(() => NODES.has(null), "set.has() requires a Biome node or token");
                throwsTypeError(() => NODES.has(Object.create(root)), "set.has() requires a Biome node or token");
                throwsTypeError(() => NODES.has.call({}, root), "has() was called on a value that is not a node kind set.");
                throwsTypeError(() => NODES.has.call(Object.create(NODES), root), "has() was called on a value that is not a node kind set.");
                throwsTypeError(() => NODES.has.call(TOKENS, root), "has() was called on a value that is not a node kind set.");
                throwsTypeError(() => TOKENS.has.call(NODES, keyword), "has() was called on a value that is not a token kind set.");
                throwsTypeError(() => TOKENS.has("LET_KW"), "set.has() requires a Biome node or token");
                registerDiagnostic(root, "information", "Kind set contract holds");
            },
        });"#,
        None,
    );

    // Sets built at module scope remain usable across evaluations.
    for _ in 0..2 {
        let result = evaluate(&plugin, "let value;");
        assert_eq!(messages(&result), ["Kind set contract holds"]);
    }
}

#[test]
fn kind_set_constructors_reject_invalid_kinds() {
    for (call, expected) in [
        (
            r#"nodeKinds("NOT_A_KIND")"#,
            r#"nodeKinds() received the unknown node kind "NOT_A_KIND". Pass a node kind listed in JsNodeByKind, such as "JS_CALL_EXPRESSION"."#,
        ),
        (
            r#"nodeKinds("JS_CALL_EXPRESSION", "js_call_expression")"#,
            r#"nodeKinds() received the unknown node kind "js_call_expression"."#,
        ),
        (
            r#"nodeKinds("IDENT")"#,
            r#"nodeKinds() received the token kind "IDENT", but it only accepts node kinds. Create a set of token kinds with tokenKinds("IDENT") instead."#,
        ),
        (
            r#"tokenKinds("NOT_A_KIND")"#,
            r#"tokenKinds() received the unknown token kind "NOT_A_KIND". Pass a kind that token.kind can return, such as "IDENT"."#,
        ),
        (
            r#"tokenKinds("JS_CALL_EXPRESSION")"#,
            r#"tokenKinds() received the node kind "JS_CALL_EXPRESSION", but it only accepts token kinds. Create a set of node kinds with nodeKinds("JS_CALL_EXPRESSION") instead."#,
        ),
        (
            r#"nodeKinds(["JS_CALL_EXPRESSION"])"#,
            r#"nodeKinds() requires node kind names as strings. Pass names such as "JS_CALL_EXPRESSION", not node objects."#,
        ),
        (
            r#"tokenKinds(new String("IDENT"))"#,
            r#"tokenKinds() requires token kind names as strings. Pass names such as "IDENT", not token objects."#,
        ),
    ] {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/plugin.js".into(),
            format!(
                r#"import {{ ast, defineRule, nodeKinds, tokenKinds }} from "@biomejs/runtime/plugin";
                const SET = {call};
                export const rule = defineRule({{ query: ast("JS_MODULE"), run() {{}} }});"#
            ),
        );
        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        let error = AnalyzerJsPlugin::load(fs, "/plugin.js".into(), None).unwrap_err();
        let message = PrintDescription(&error).to_string();
        assert!(message.contains(expected), "{call}: {message}");
    }
}
