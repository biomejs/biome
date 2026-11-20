use super::*;
use biome_module_graph::{
    ExpressionTypeInput, infer_expression_type,
    type_inference::{
        NormalizedExpressionTypeRequest, TypeInferenceCaller, execute_type_inference_request,
    },
};

// Expected types are those that TypeScript 7.0.2 (`tsc --noEmit --strict
// --target es2022`) gives the same reads, unless a comment says otherwise.
// TypeScript adds `undefined` to the return type that it infers from a
// function's body when some `return` has a value and a path reaches the end
// of the body, and gives `void` to a function whose returns have no value.

fn marked_range(source: &str, marker: &str, expression: &str) -> TextRange {
    let marker = format!("/*{marker}*/");
    let mut occurrences = source.match_indices(&marker);
    let (offset, _) = occurrences.next().expect("source marker must exist");
    assert!(occurrences.next().is_none(), "source marker must be unique");
    let start = offset + marker.len();
    assert!(source[start..].starts_with(expression));
    TextRange::at(
        start.try_into().unwrap(),
        expression.len().try_into().unwrap(),
    )
}

fn return_types_db(source: &str) -> (TestModuleDb, ModuleInfo) {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), source);
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    (db, module)
}

/// Names each member of `ty`, sorted: Biome's Unknown as `Unknown`, and any
/// other type by its formatted text on one line, such as `number`,
/// `undefined`, or `number: 1`.
fn variant_names(db: &TestModuleDb, ty: InferredTypeData) -> Vec<String> {
    if ty == InferredTypeData::Unknown {
        return vec!["Unknown".to_owned()];
    }
    let variants = match ty {
        InferredTypeData::Union(union) => union.types(db).to_vec(),
        _ => vec![ty],
    };
    let mut names: Vec<_> = variants
        .into_iter()
        .map(|ty| {
            let text = format_inferred_type(db, ty);
            let lines: Vec<_> = text.lines().map(str::trim).collect();
            lines.join(" ")
        })
        .collect();
    names.sort();
    names
}

/// Asserts the type of each `(marker, expression, expected)` read in `source`
/// by the names of its members (see [`variant_names`]). Also asserts that
/// complete module inference gives each read the type that a lookup gives it.
fn assert_reads(source: &str, reads: &[(&str, &str, &[&str])]) {
    let lookups: Vec<_> = {
        let (db, module) = return_types_db(source);
        reads
            .iter()
            .map(|(marker, expression, _)| {
                let range = marked_range(source, marker, expression);
                infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range))
                    .map(|ty| format_inferred_type(&db, ty))
            })
            .collect()
    };
    let (db, module) = return_types_db(source);
    let complete = infer_module_types(&db, module).unwrap();
    let mismatches: Vec<_> = reads
        .iter()
        .zip(lookups)
        .flat_map(|((marker, expression, expected), lookup)| {
            let range = marked_range(source, marker, expression);
            let ty = execute_type_inference_request(
                &db,
                TypeInferenceCaller::new("test", "return_types"),
                NormalizedExpressionTypeRequest::new(module, range),
            )
            .unwrap_or_else(|| panic!("expression at {marker} must be inferred"));
            let actual = variant_names(&db, ty);
            let complete = Some(format_inferred_type(&db, complete.expressions[&range]));
            [
                (actual != *expected)
                    .then(|| format!("{marker}: expected {expected:?}, got {actual:?}")),
                (complete != lookup).then(|| {
                    format!("{marker}: module inference gives {complete:?}, lookups {lookup:?}")
                }),
            ]
        })
        .flatten()
        .collect();
    assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
}

const DECLARATIONS: &str = r#"
    declare function fail(): never;
    declare function assert(v: unknown): asserts v;
    declare const c: boolean;
    declare const s: "a" | "b";
    const failArrow = (): never => { throw 0; };
"#;

/// Asserts the reads of `source` after [`DECLARATIONS`].
fn assert_reads_after_declarations(source: &str, reads: &[(&str, &str, &[&str])]) {
    assert_reads(&format!("{DECLARATIONS}{source}"), reads);
}

#[test]
fn reachable_ends_add_undefined() {
    assert_reads_after_declarations(
        r#"
        function e1() { if (c) return 1; }
        const e9 = () => { if (c) return 1; };
        function e10() { if (c) return "a"; console.log(); }
        function e12() { for (const v of [1]) return v; }
        function e15() { label: { if (c) break label; return 1; } }
        async function e16() { if (c) return 1; }
        function e17() { if (c) return 1; failArrow(); }
        function e20() { if (c) return 1; if (false) return 2; }
        function e21() { switch (s) { case "a": return 1; } }
        function e25() { try { if (c) return 1; } catch { return 2; } }
        function e26() { try { return 1; } catch { } }
        function e27() { if (c) return null; }
        const e30 = function () { if (c) return 1; };
        function s3() { if (c) return 1; else if (c) return 2; }
        class K { m() { if (c) return 1; } get g() { if (c) return "a"; } }
        const o = { m() { if (c) return 1; } };
        /*e1*/e1();
        /*e9*/e9();
        /*e10*/e10();
        /*e12*/e12();
        /*e15*/e15();
        /*e16*/e16();
        /*e17*/e17();
        /*e20*/e20();
        /*e21*/e21();
        /*e25*/e25();
        /*e26*/e26();
        /*e27*/e27();
        /*e30*/e30();
        /*s3*/s3();
        /*method*/new K().m();
        /*getter*/new K().g;
        /*objectMethod*/o.m();
        "#,
        &[
            ("e1", "e1()", &["number: 1", "undefined"]),
            ("e9", "e9()", &["number: 1", "undefined"]),
            ("e10", "e10()", &["string: a", "undefined"]),
            ("e12", "e12()", &["number", "undefined"]),
            ("e15", "e15()", &["number: 1", "undefined"]),
            (
                "e16",
                "e16()",
                &["instanceof Promise<number: 1 | undefined>"],
            ),
            // The arrow's annotation isn't the variable's, so the call
            // doesn't end the path.
            ("e17", "e17()", &["number: 1", "undefined"]),
            // A `return` in code that no path reaches still counts.
            ("e20", "e20()", &["number: 1", "number: 2", "undefined"]),
            ("e21", "e21()", &["number: 1", "undefined"]),
            ("e25", "e25()", &["number: 1", "number: 2", "undefined"]),
            ("e26", "e26()", &["number: 1", "undefined"]),
            ("e27", "e27()", &["null", "undefined"]),
            ("e30", "e30()", &["number: 1", "undefined"]),
            ("s3", "s3()", &["number: 1", "number: 2", "undefined"]),
            ("method", "new K().m()", &["number: 1", "undefined"]),
            ("getter", "new K().g", &["string: a", "undefined"]),
            ("objectMethod", "o.m()", &["number: 1", "undefined"]),
        ],
    );
}

#[test]
fn unreachable_ends_add_nothing() {
    assert_reads_after_declarations(
        r#"
        function e2() { if (c) return 1; fail(); }
        function e3() { if (c) return 1; throw 0; }
        function e4() { switch (s) { case "a": return 1; case "b": return 2; } }
        function e5() { if (c) return 1; else return 2; }
        function e6() { while (true) { if (c) return 1; } }
        function e7() { if (c) return 1; return; }
        function e11() { try { return 1; } finally { } }
        function e13() { if (c) return 1; assert(false); }
        function e14() { do { return 1; } while (c); }
        function e18() { if (c) return 1; if (true) return 2; }
        function e24() { if (c) { return 1; } else { fail(); } }
        function e31(x: string | null) { if (x === null) return 1; switch (s) { case "a": return 2; case "b": return 3; } }
        function s1() { if (c) { return 1; } else { if (c) return 2; else throw 0; } }
        function s2() { { if (c) return 1; return 2; } }
        class K { get g() { if (c) return 1; return 2; } }
        /*e2*/e2();
        /*e3*/e3();
        /*e4*/e4();
        /*e5*/e5();
        /*e6*/e6();
        /*e7*/e7();
        /*e11*/e11();
        /*e13*/e13();
        /*e14*/e14();
        /*e18*/e18();
        /*e24*/e24();
        /*e31*/e31(null);
        /*s1*/s1();
        /*s2*/s2();
        /*getter*/new K().g;
        "#,
        &[
            // A single literal type widens.
            ("e2", "e2()", &["number"]),
            ("e3", "e3()", &["number"]),
            ("e4", "e4()", &["number: 1", "number: 2"]),
            ("e5", "e5()", &["number: 1", "number: 2"]),
            ("e6", "e6()", &["number"]),
            // A bare `return;` adds `undefined` when another `return` has a
            // value.
            ("e7", "e7()", &["number: 1", "undefined"]),
            ("e11", "e11()", &["number"]),
            ("e13", "e13()", &["number"]),
            ("e14", "e14()", &["number"]),
            ("e18", "e18()", &["number: 1", "number: 2"]),
            ("e24", "e24()", &["number"]),
            ("e31", "e31(null)", &["number: 1", "number: 2", "number: 3"]),
            ("s1", "s1()", &["number: 1", "number: 2"]),
            ("s2", "s2()", &["number: 1", "number: 2"]),
            ("getter", "new K().g", &["number: 1", "number: 2"]),
        ],
    );
}

#[test]
fn returns_without_values_give_void() {
    assert_reads_after_declarations(
        r#"
        function e8() { }
        function e23() { throw 0; }
        function e28() { return; }
        function e29() { if (c) return; }
        function b1() { if (c) return; fail(); }
        const b2 = () => { if (c) return; };
        const e22 = () => { throw 0; };
        /*e8*/e8();
        /*e23*/e23();
        /*e28*/e28();
        /*e29*/e29();
        /*b1*/b1();
        /*b2*/b2();
        /*e22*/e22();
        "#,
        &[
            ("e8", "e8()", &["void"]),
            ("e23", "e23()", &["void"]),
            ("e28", "e28()", &["void"]),
            ("e29", "e29()", &["void"]),
            ("b1", "b1()", &["void"]),
            ("b2", "b2()", &["void"]),
            // TypeScript gives `never` to a function or arrow expression
            // without a `return` whose end no path reaches. Biome gives
            // `void`, which is wider.
            ("e22", "e22()", &["void"]),
        ],
    );
}

#[test]
fn returns_of_nested_functions_and_members_do_not_count() {
    assert_reads_after_declarations(
        r#"
        function e19() { const o = { m() { return "x"; } }; return 1; }
        function n1() { class K { m() { return "x"; } get g() { return true; } } return 1; }
        function n2() { function inner() { return "x"; } const a = () => "y"; return 1; }
        /*e19*/e19();
        /*n1*/n1();
        /*n2*/n2();
        "#,
        &[
            ("e19", "e19()", &["number"]),
            ("n1", "n1()", &["number"]),
            ("n2", "n2()", &["number"]),
        ],
    );
}

// The end of `e` is reachable unless `box.fail()` never returns, which
// depends on the return type of `e` itself. Resolving that return type asks
// whether the end of `e` is reachable again, a query cycle, which keeps the
// `undefined` of the end.
#[test]
fn a_cycle_through_the_reachability_of_the_end_keeps_undefined() {
    assert_reads(
        r#"
        declare const c: boolean;
        declare const box: { fail: typeof e };
        function e() { if (c) return 1; box.fail(); }
        /*e*/e();
        "#,
        &[("e", "e()", &["number: 1", "undefined"])],
    );
}

// A call to a local function without a return type annotation gives a write
// the function's inferred return type, when Biome types each value that the
// function returns as TypeScript does, following calls to other such
// functions up to four levels deep.
#[test]
fn writes_of_calls_to_local_functions_use_their_inferred_return_types() {
    assert_reads_after_declarations(
        r#"
        function g() { if (c) return 1; }
        function h() { return 1; }
        function k() { if (c) return 1; fail(); }
        const m = { get() { if (c) return 1; } };
        const arrow = () => 1;
        const expr = function () { if (c) return 1; return 2; };
        function f5() { return 1; }
        function f4() { return f5(); }
        function f3() { return f4(); }
        function f2() { return f3(); }
        function f1() { return f2(); }
        function ev() { let v = null; if (c) v = "a"; return v; }
        function* gen() { return 1; }
        function w1() { let x: number | undefined = 0; x = g(); /*w1*/x; }
        function w2() { let x: number | undefined = 0; x = h(); /*w2*/x; }
        function w3() { let x: number | undefined = 0; x = k(); /*w3*/x; }
        function w4() { let x: number | undefined = 0; const y = g(); x = y; /*w4*/x; }
        function w5() { let x: number | undefined = 0; x = m.get(); /*w5*/x; }
        function w6() { let x: number | string | undefined = "a"; x = c ? g() : "b"; /*w6*/x; }
        function w7() { let x; x = g(); /*w7*/x; }
        function w8() { const xs = []; xs.push(g()); /*w8*/xs; }
        function w9() { let x: number | undefined = 0; x = arrow(); /*w9*/x; }
        function w10() { let x: number | undefined = 0; x = expr(); /*w10*/x; }
        function w11() { let x: number | undefined = 0; x = f4(); /*w11*/x; }
        function w12() { let x: number | undefined = 0; x = f1(); /*w12*/x; }
        function w13() { let x: string | null | undefined = "b"; x = ev(); /*w13*/x; }
        function w14() { let x: number | object | undefined = 0; x = gen(); /*w14*/x; }
        function w15() { let x = null; x = h(); /*w15*/x; }
        "#,
        &[
            ("w1", "x", &["number", "undefined"]),
            ("w2", "x", &["number"]),
            ("w3", "x", &["number"]),
            ("w4", "x", &["number", "undefined"]),
            // A method without a return type annotation stays undetermined.
            ("w5", "x", &["number", "undefined"]),
            ("w6", "x", &["number", "string", "undefined"]),
            ("w7", "x", &["number", "undefined"]),
            ("w8", "xs", &["instanceof Array<number | undefined>"]),
            ("w9", "x", &["number"]),
            ("w10", "x", &["number"]),
            ("w11", "x", &["number"]),
            // TypeScript gives `number`. `f1` reaches `f5` through four
            // other functions, beyond the levels that Biome follows.
            ("w12", "x", &["number", "undefined"]),
            // TypeScript gives `string | null`. Biome types the value of `v`
            // as `null`, the type of its initializer, so `ev` stays
            // undetermined.
            ("w13", "x", &["null", "string", "undefined"]),
            // TypeScript gives `object`. A generator returns a generator
            // object, which Biome doesn't infer, so `gen` stays undetermined.
            ("w14", "x", &["number", "object", "undefined"]),
            // A variable whose type evolves with its writes takes the type
            // of the call.
            ("w15", "x", &["number"]),
        ],
    );
}

// A lookup of a call asks whether a path reaches the end of the function
// that it calls, and of no other function, and a second lookup of a call to
// the same function reuses the answer.
#[test]
fn return_types_ask_the_reachability_of_their_own_function_once() {
    const SOURCE: &str = r#"
        declare const c: boolean;
        function asked() { if (c) return 1; }
        function other() { if (c) return 2; }
        /*first*/asked();
        /*second*/asked();
    "#;
    let (db, module) = return_types_db(SOURCE);
    db.clear_salsa_events();
    for marker in ["first", "second"] {
        let range = marked_range(SOURCE, marker, "asked()");
        infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range));
    }
    let events = db.take_salsa_events();
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "infer_flow_reachability", &events),
        1
    );
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "infer_module_types", &events),
        0
    );
}

// Collection adds no marker of the end when the syntax of the body shows
// that no path reaches it. A `switch` statement with `default` reaches its
// end only through its last clause or a `break`, because each clause falls
// through to the next one. A `try` statement with `catch` reaches its end
// only when the `try` block or the `catch` clause completes. Declarations
// that run no code don't count as the last statement. The other functions
// keep the marker, and the module's flow decides their end.
#[test]
fn ends_that_syntax_decides_match_the_flow() {
    assert_reads(
        r#"
        declare const c: boolean;
        declare const x: number;
        declare function log(): void;
        function a1() { switch (x) { case 1: return 1; default: return 2; } }
        function a2() { switch (x) { default: return 0; case 1: return 1; } }
        function a3() { switch (x) { case 1: if (c) return 1; default: return 2; } }
        function a4() { switch (x) { case 1: return 1; default: for (const v of [1]) { if (v) break; } return 2; } }
        function a5() { switch (x) { default: switch (x) { case 1: break; default: return 1; } return 2; } }
        function a6() { switch (x) { case 1: return 1; default: return 2; function g() {} } }
        function n1() { switch (x) { case 1: break; default: return 2; } }
        function n2() { switch (x) { default: if (c) break; return 1; } }
        function n3() { switch (x) { case 1: return 1; default: return 2; case 2: } }
        function n4() { switch (x) { case 1: return 1; default: return 2; case 2: log(); } }
        function n5() { switch (x) { case 1: return 1; case 2: return 2; } }
        function n6() { l: switch (x) { case 1: return 1; default: return 2; } }
        function t1() { try { return 1; } catch { return 2; } }
        function t2() { try { if (c) return 1; return 2; } catch (e) { return 3; } finally { log(); } }
        function t3() { try { return 1; } catch { throw 0; } }
        function m1() { try { return 1; } finally { log(); } }
        function m2() { try { return 1; } catch { log(); } }
        function m3() { try { log(); } catch { return 1; } }
        function d1() { if (c) return 1; return 2; function g() {} }
        function d2() { if (c) return 1; throw 0; type T = string; interface I {} function h() {} }
        function d3() { if (c) return 1; log(); function g() {} }
        /*a1*/a1();
        /*a2*/a2();
        /*a3*/a3();
        /*a4*/a4();
        /*a5*/a5();
        /*a6*/a6();
        /*n1*/n1();
        /*n2*/n2();
        /*n3*/n3();
        /*n4*/n4();
        /*n5*/n5();
        /*n6*/n6();
        /*t1*/t1();
        /*t2*/t2();
        /*t3*/t3();
        /*m1*/m1();
        /*m2*/m2();
        /*m3*/m3();
        /*d1*/d1();
        /*d2*/d2();
        /*d3*/d3();
        "#,
        &[
            ("a1", "a1()", &["number: 1", "number: 2"]),
            ("a2", "a2()", &["number: 0", "number: 1"]),
            ("a3", "a3()", &["number: 1", "number: 2"]),
            ("a4", "a4()", &["number: 1", "number: 2"]),
            ("a5", "a5()", &["number: 1", "number: 2"]),
            ("a6", "a6()", &["number: 1", "number: 2"]),
            ("n1", "n1()", &["number: 2", "undefined"]),
            ("n2", "n2()", &["number: 1", "undefined"]),
            ("n3", "n3()", &["number: 1", "number: 2", "undefined"]),
            ("n4", "n4()", &["number: 1", "number: 2", "undefined"]),
            ("n5", "n5()", &["number: 1", "number: 2", "undefined"]),
            ("n6", "n6()", &["number: 1", "number: 2"]),
            ("t1", "t1()", &["number: 1", "number: 2"]),
            ("t2", "t2()", &["number: 1", "number: 2", "number: 3"]),
            ("t3", "t3()", &["number"]),
            ("m1", "m1()", &["number"]),
            ("m2", "m2()", &["number: 1", "undefined"]),
            ("m3", "m3()", &["number: 1", "undefined"]),
            ("d1", "d1()", &["number: 1", "number: 2"]),
            ("d2", "d2()", &["number"]),
            ("d3", "d3()", &["number: 1", "undefined"]),
        ],
    );
}

// A `return` without a value adds `undefined` to the return type, so the end
// of the body can only add `undefined` again. Resolution gives the end
// `undefined` without asking whether a path reaches it, and the return type
// stays the one that the module's flow gives.
#[test]
fn ends_after_returns_without_values_match_the_flow() {
    assert_reads_after_declarations(
        r#"
        function u1() { if (c) return; if (c) return 1; fail(); }
        function u2() { if (c) return; if (c) return 1; }
        function u3() { if (c) return 1; else if (c) return; switch (s) { case "a": return 2; case "b": return 3; } }
        const u4 = () => { if (c) return "a"; if (c) return; };
        class K { m() { if (c) return 1; if (c) return; } }
        /*u1*/u1();
        /*u2*/u2();
        /*u3*/u3();
        /*u4*/u4();
        /*method*/new K().m();
        "#,
        &[
            ("u1", "u1()", &["number: 1", "undefined"]),
            ("u2", "u2()", &["number: 1", "undefined"]),
            (
                "u3",
                "u3()",
                &["number: 1", "number: 2", "number: 3", "undefined"],
            ),
            ("u4", "u4()", &["string: a", "undefined"]),
            ("method", "new K().m()", &["number: 1", "undefined"]),
        ],
    );
}

// A function decides the end of its body without the reachability query
// when its syntax shows that no path reaches the end, or when a `return`
// without a value already adds `undefined`. A `switch` without `default`
// still needs the query.
#[test]
fn ends_decided_from_syntax_ask_no_reachability_query() {
    const SOURCE: &str = r#"
        declare const c: boolean;
        declare const s: "a" | "b";
        function closed() { switch (s) { case "a": return 1; default: return 2; } }
        function bare() { if (c) return; if (c) return 1; }
        function asked() { switch (s) { case "a": return 1; } }
        /*closed*/closed();
        /*bare*/bare();
        /*asked*/asked();
    "#;
    let (db, module) = return_types_db(SOURCE);
    db.clear_salsa_events();
    for (marker, expression) in [("closed", "closed()"), ("bare", "bare()")] {
        let range = marked_range(SOURCE, marker, expression);
        infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range));
    }
    let events = db.take_salsa_events();
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "infer_flow_reachability", &events),
        0
    );
    let range = marked_range(SOURCE, "asked", "asked()");
    infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range));
    let events = db.take_salsa_events();
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "infer_flow_reachability", &events),
        1
    );
}
