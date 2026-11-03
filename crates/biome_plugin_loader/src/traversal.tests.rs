use super::tests::{load_test_plugin_from_source, render_diagnostics, services, snap_diagnostics};
use super::*;
use biome_diagnostics::PrintDescription;
use biome_js_parser::{JsParserOptions, parse};

fn evaluate(plugin: &AnalyzerJsPlugin, source: &str) -> PluginEvalResult {
    let parse = parse(source, JsFileSource::jsx(), JsParserOptions::default());
    assert!(!parse.has_errors(), "{source}");
    plugin.evaluate(
        parse.syntax().into(),
        "/file.jsx".into(),
        &services(JsFileSource::jsx()),
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
fn traversals_visit_matching_descendants() {
    let plugin_source = r#"import { ast, defineRule, nodeKinds, registerDiagnostic } from "@biomejs/runtime/plugin";
        const CALLS = nodeKinds("JS_CALL_EXPRESSION");
        const FUNCTIONS_AND_CALLS = nodeKinds("JS_FUNCTION_DECLARATION", "JS_ARROW_FUNCTION_EXPRESSION", "JS_CALL_EXPRESSION");
        export const calls = defineRule({
            query: ast("JS_MODULE"),
            run(root) {
                for (const call of root.descendants(CALLS)) {
                    registerDiagnostic(call, "information", `descendants() found ${call.callee.text}().`);
                }
            },
        });
        export const enclosingFunctions = defineRule({
            query: ast("JS_MODULE"),
            run(root) {
                const functions = [];
                root.walk(
                    FUNCTIONS_AND_CALLS,
                    (node) => {
                        if (node.kind === "JS_CALL_EXPRESSION") {
                            const enclosing = functions.at(-1) ?? "the module";
                            registerDiagnostic(node, "information", `walk() found ${node.callee.text}() in ${enclosing}.`);
                        } else if (node.id?.text === "skipped") {
                            return false;
                        } else {
                            functions.push(node.id?.text ?? "an arrow function");
                        }
                    },
                    (node) => {
                        if (node.kind !== "JS_CALL_EXPRESSION") {
                            functions.pop();
                        }
                    },
                );
            },
        });"#;
    let source = r#"setup();
function outer() {
    first();
    const inner = () => second(third());
    fourth();
}
function skipped() {
    hidden();
}
last();
"#;
    let plugin = load_test_plugin_from_source("/plugin.js", plugin_source, None);
    let parse = parse(
        source,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );
    let result = plugin.evaluate(
        parse.syntax().into(),
        "/file.js".into(),
        &services(JsFileSource::js_module()),
    );

    snap_diagnostics(
        "traversals_visit_matching_descendants",
        "/plugin.js",
        plugin_source,
        &[("/file.js", source)],
        render_diagnostics("/file.js", source, result),
    );
}

/// Compares the native traversals with a walk over `children()` that filters
/// every node in JavaScript, which is what plugins do without them.
#[test]
fn traversals_match_a_children_walk() {
    let plugin = load_test_plugin_from_source(
        "/plugin.js",
        r#"import { ast, defineRule, nodeKinds, registerDiagnostic } from "@biomejs/runtime/plugin";
        const SETS = [
            ["JS_CALL_EXPRESSION"],
            ["JSX_ELEMENT", "JSX_OPENING_ELEMENT", "JSX_SELF_CLOSING_ELEMENT", "JSX_ATTRIBUTE", "JSX_EXPRESSION_CHILD"],
            ["JS_MODULE", "JS_MODULE_ITEM_LIST", "JS_CALL_ARGUMENT_LIST", "JS_IDENTIFIER_EXPRESSION"],
            ["JS_FUNCTION_DECLARATION", "JS_ARROW_FUNCTION_EXPRESSION", "JS_RETURN_STATEMENT"],
            [],
        ];
        // Skips functions named `skipped` in the walks below.
        const isSkipped = (node) => node.kind === "JS_FUNCTION_DECLARATION" && node.id?.text === "skipped";

        function childrenWalk(node, kinds, events, skip) {
            for (const child of node.children()) {
                const matches = kinds.includes(child.kind);
                if (matches) {
                    events.push(`enter ${child.kind} ${child.text}`);
                    if (skip && isSkipped(child)) continue;
                }
                childrenWalk(child, kinds, events, skip);
                if (matches) events.push(`exit ${child.kind} ${child.text}`);
            }
            return events;
        }

        export const equivalence = defineRule({
            query: ast("JS_MODULE"),
            run(root) {
                const starts = [root, ...root.descendants(nodeKinds("JS_FUNCTION_DECLARATION", "JSX_ELEMENT", "JS_CALL_EXPRESSION"))];
                for (const start of starts) {
                    for (const names of SETS) {
                        const kinds = nodeKinds(...names);
                        const label = `${start.kind} ${names.join(",")}`;

                        const expected = childrenWalk(start, names, [], false);
                        const found = start.descendants(kinds).map((node) => `enter ${node.kind} ${node.text}`);
                        const entered = expected.filter((event) => event.startsWith("enter "));
                        if (JSON.stringify(found) !== JSON.stringify(entered)) {
                            throw new Error(`descendants() ${label}: ${JSON.stringify(found)} !== ${JSON.stringify(entered)}`);
                        }

                        for (const skip of [false, true]) {
                            const events = [];
                            start.walk(
                                kinds,
                                (node) => {
                                    events.push(`enter ${node.kind} ${node.text}`);
                                    if (skip && isSkipped(node)) return false;
                                },
                                (node) => {
                                    events.push(`exit ${node.kind} ${node.text}`);
                                },
                            );
                            const reference = childrenWalk(start, names, [], skip);
                            if (JSON.stringify(events) !== JSON.stringify(reference)) {
                                throw new Error(`walk() ${label} skip=${skip}: ${JSON.stringify(events)} !== ${JSON.stringify(reference)}`);
                            }
                        }
                    }
                }
                registerDiagnostic(root, "information", `Compared ${starts.length} starting nodes`);
            },
        });"#,
        None,
    );

    let result = evaluate(
        &plugin,
        r#"import { T } from "gt-next";
function outer(a) {
    return <T id="x"><b>{a}</b>{f(g(a), <Var value={h()} />)}</T>;
}
function skipped() {
    const nested = () => hidden();
    return nested;
}
const arrow = () => [call(), (() => deeper())()];
"#,
    );
    assert_eq!(messages(&result), ["Compared 12 starting nodes"]);
}

#[test]
fn traversal_contract() {
    let plugin = load_test_plugin_from_source(
        "/plugin.js",
        r#"import { ast, defineRule, nodeKinds, registerDiagnostic, tokenKinds } from "@biomejs/runtime/plugin";
        const FUNCTIONS = nodeKinds("JS_FUNCTION_DECLARATION");
        const CALLS = nodeKinds("JS_CALL_EXPRESSION");
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
                const [outer] = root.descendants(FUNCTIONS);
                assert(outer.id.text === "outer", "first function");

                // The receiver is excluded even when its kind is in the set.
                const nested = outer.descendants(FUNCTIONS);
                assert(nested.length === 1 && nested[0].id.text === "inner", "receiver excluded from descendants()");
                const walked = [];
                outer.walk(FUNCTIONS, (node) => walked.push(`enter ${node.id.text}`), (node) => walked.push(`exit ${node.id.text}`));
                assert(walked.join() === "enter inner,exit inner", `receiver excluded from walk(): ${walked}`);

                // descendants() returns a fresh, ordinary array.
                const first = root.descendants(CALLS);
                const second = root.descendants(CALLS);
                assert(Array.isArray(first) && first !== second, "fresh array");
                first.length = 0;
                assert(root.descendants(CALLS).length === 3, "independent arrays");
                assert(root.descendants(nodeKinds()).length === 0, "empty set");
                assert(outer.id.descendants(CALLS).length === 0, "leaf node");

                // exit receives the object that enter received.
                const entered = new WeakSet();
                let exits = 0;
                root.walk(
                    CALLS,
                    (node) => {
                        entered.add(node);
                    },
                    (node) => {
                        assert(entered.has(node), "exit identity");
                        exits++;
                    },
                );
                assert(exits === 3, "every exit");

                // Only `false` skips a subtree; other falsy values continue.
                for (const value of [undefined, null, 0, "", NaN, true]) {
                    let count = 0;
                    root.walk(FUNCTIONS, () => {
                        count++;
                        return value;
                    });
                    assert(count === 2, `${String(value)} continues`);
                }
                let skippedExits = 0;
                let visited = 0;
                root.walk(FUNCTIONS, () => {
                    visited++;
                    return false;
                }, () => {
                    skippedExits++;
                });
                assert(visited === 1 && skippedExits === 0, "false skips the subtree and exit");

                // An omitted or undefined exit is accepted.
                let enters = 0;
                root.walk(CALLS, () => { enters++; });
                root.walk(CALLS, () => { enters++; }, undefined);
                assert(enters === 6, "optional exit");

                // Callbacks run with an undefined receiver and can start nested traversals.
                const inner = [];
                root.walk(FUNCTIONS, function (node) {
                    assert(this === undefined, "callback receiver");
                    inner.push(node.descendants(CALLS).length);
                    node.walk(CALLS, () => {});
                });
                assert(inner.join() === "2,1", `nested traversals: ${inner}`);

                // Exceptions stop the walk and propagate unchanged.
                const sentinel = new Error("sentinel");
                for (const callbacks of [[() => { throw sentinel; }], [() => {}, () => { throw sentinel; }]]) {
                    let calls = 0;
                    try {
                        root.walk(CALLS, (node) => { calls++; return callbacks[0](node); }, callbacks[1]);
                        throw new Error("walk() did not throw");
                    } catch (error) {
                        assert(error === sentinel, `propagated: ${error}`);
                    }
                    assert(calls === 1, "stopped after the exception");
                }

                throwsTypeError(() => root.descendants(), "node.descendants() received the wrong number of arguments.");
                throwsTypeError(() => root.descendants(CALLS, CALLS), "node.descendants() received the wrong number of arguments.");
                throwsTypeError(() => root.descendants("JS_CALL_EXPRESSION"), "The kinds argument to node.descendants() is not a node kind set.");
                throwsTypeError(() => root.descendants(["JS_CALL_EXPRESSION"]), "The kinds argument to node.descendants() is not a node kind set.");
                throwsTypeError(() => root.descendants(Object.create(CALLS)), "The kinds argument to node.descendants() is not a node kind set.");
                throwsTypeError(() => root.descendants(tokenKinds("IDENT")), "node.descendants() received a set of token kinds, but it only visits nodes.");
                throwsTypeError(() => root.descendants.call({}, CALLS), "descendants() was called on a value that is not a node.");
                throwsTypeError(() => root.walk(CALLS), "node.walk() received the wrong number of arguments.");
                throwsTypeError(() => root.walk(CALLS, () => {}, () => {}, () => {}), "node.walk() received the wrong number of arguments.");
                throwsTypeError(() => root.walk("JS_CALL_EXPRESSION", () => {}), "The kinds argument to node.walk() is not a node kind set.");
                throwsTypeError(() => root.walk(tokenKinds("IDENT"), () => {}), "node.walk() received a set of token kinds, but it only visits nodes.");
                throwsTypeError(() => root.walk(CALLS, undefined), "The enter argument to node.walk() is not a function.");
                throwsTypeError(() => root.walk(CALLS, () => {}, null), "The exit argument to node.walk() is not a function.");
                throwsTypeError(() => root.walk.call(CALLS, CALLS, () => {}), "walk() was called on a value that is not a node.");
                registerDiagnostic(root, "information", "Traversal contract holds");
            },
        });"#,
        None,
    );

    let result = evaluate(
        &plugin,
        r#"function outer() {
    a();
    function inner() {
        b();
    }
}
c();
"#,
    );
    assert_eq!(messages(&result), ["Traversal contract holds"]);
}
