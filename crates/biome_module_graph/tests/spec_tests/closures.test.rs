use super::*;
use biome_module_graph::{
    ExpressionTypeInput, infer_expression_type,
    type_inference::{
        NormalizedExpressionTypeRequest, TypeInferenceCaller, execute_type_inference_request,
    },
};

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

fn narrowing_db(source: &str) -> (TestModuleDb, ModuleInfo) {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), source);
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    (db, module)
}

/// Names each member of `ty`, sorted. An interface is named by its name, and
/// any other type by its formatted text on one line, such as `string`,
/// `null`, or `string: a`. Biome's Unknown, the type of a read on a path that
/// no value takes, is named `Unknown`.
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
            let first = text.lines().next().unwrap_or_default();
            if let Some(name) = first
                .strip_prefix("interface \"")
                .and_then(|name| name.split('"').next())
            {
                return name.to_owned();
            }
            let lines: Vec<_> = text.lines().map(str::trim).collect();
            lines.join(" ")
        })
        .collect();
    names.sort();
    names
}

/// Declarations that [`assert_reads`] puts before each test source.
const DECLARATIONS: &str = r#"
    declare function f<T>(): T;
    declare const c: boolean;
    declare function reset(): void;
    declare function fail(): never;
    interface Circle { kind: "c"; r: number }
    interface Square { kind: "s"; w: number }
"#;

/// Asserts the type of each `(marker, expression, expected)` read in `source`
/// by the names of its members (see [`variant_names`]). Also asserts that
/// complete module inference gives each read the type that a lookup gives
/// it.
fn assert_reads(source: &str, reads: &[(&str, &str, &[&str])]) {
    let source = format!("{DECLARATIONS}{source}");
    let lookups: Vec<_> = {
        let (db, module) = narrowing_db(&source);
        reads
            .iter()
            .map(|(marker, expression, _)| {
                let range = marked_range(&source, marker, expression);
                infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range))
                    .map(|ty| format_inferred_type(&db, ty))
            })
            .collect()
    };
    let (db, module) = narrowing_db(&source);
    let complete = infer_module_types(&db, module).unwrap();
    let mismatches: Vec<_> = reads
        .iter()
        .zip(lookups)
        .flat_map(|((marker, expression, expected), lookup)| {
            let range = marked_range(&source, marker, expression);
            let ty = execute_type_inference_request(
                &db,
                TypeInferenceCaller::new("test", "closures"),
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

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A read in a function or arrow expression, or in a method or accessor of an
// object literal or a class expression, continues the flow where the closure
// is created. It always does for a `const`, and for a parameter or a `let`
// past its last assignment.
#[test]
fn closures_continue_the_flow_where_they_are_created() {
    assert_reads(
        r#"
        function k1() { const x: string | null = f(); if (x) { const g = () => /*k1*/x; } }
        function k2() { let x: string | null = f(); if (x) { const g = () => /*k2*/x; } }
        function k4(x: string | null) { if (x) [1].forEach(() => /*k4*/x); }
        function k6(x: string | null) { x = x ?? "d"; return () => /*k6*/x; }
        function k9() { const x: string | null = f(); if (x) () => () => /*k9*/x; }
        function k11() { const x: string | null = f(); if (x) ({ m() { /*k11*/x; } }); }
        function k12() { const x: string | null = f(); if (x) (class { m() { /*k12*/x; } }); }
        function k14() { const x: string | null = f(); if (x) ({ get g() { /*k14*/x; return 1; } }); }
        function k19() { let x: string | number = "a"; const g = () => /*k19*/x; }
        function k20() { let x: string | number = "a"; x = 1; const g = () => /*k20*/x; }
        function k21() { let x: string | null = f(); if (x) { reset(); /*k21*/x; } }
        function k22() { let x: string | null = f(); if (x) { [1].forEach(() => { x = null; }); /*k22*/x; } }
        function k23() { let x: string | null = f(); if (!x) return; x; const g = () => /*k23*/x; }
        function k25() { const x: string | null = f(); const g = () => { if (!x) return; return () => /*k25*/x; }; }
        function k26() { const x: string | null = f(); if (x) { const g = function () { /*k26*/x; }; } }
        function k27(x: string | null) { if (x) { const g = (y = /*k27*/x) => y; } }
        function k28(x: string | null) { if (!x) return; const g = () => { if (c) /*k28*/x; }; }
        function k31({ a }: { a: string | null }) { if (a) () => /*k31*/a; }
        function k32() { const x: string | null = f(); if (x) { const g = async () => /*k32*/x; } }
        function k33() { const x: string | null = f(); if (x) { const g = function* () { /*k33*/x; }; } }
        function k35() { const x: string | null = f(); if (x) ({ m: () => /*k35*/x }); }
        function k40() { let x: string | null = f(); if (x) { (() => /*k40*/x)(); } }
        "#,
        &[
            ("k1", "x", &["string"]),
            ("k2", "x", &["string"]),
            ("k4", "x", &["string"]),
            ("k6", "x", &["string"]),
            ("k9", "x", &["string"]),
            ("k11", "x", &["string"]),
            ("k12", "x", &["string"]),
            ("k14", "x", &["string"]),
            ("k19", "x", &["string"]),
            ("k20", "x", &["number"]),
            ("k21", "x", &["string"]),
            ("k22", "x", &["string"]),
            ("k23", "x", &["string"]),
            ("k25", "x", &["string"]),
            ("k26", "x", &["string"]),
            ("k27", "x", &["string"]),
            ("k28", "x", &["string"]),
            ("k31", "a", &["string"]),
            ("k32", "x", &["string"]),
            ("k33", "x", &["string"]),
            ("k35", "x", &["string"]),
            ("k40", "x", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below its declared type.
// A read in a closure starts from the declared type when it reads a `var`, a
// property path, or a parameter or a `let` that its function assigns after
// the read or that another function assigns. So does a read in a function
// declaration, a method of a class declaration, or a field initializer, and
// a read in a closure created before the test.
#[test]
fn closures_keep_the_declared_type_where_typescript_does() {
    assert_reads(
        r#"
        function k3() { let x: string | null = f(); if (x) { const g = () => /*k3*/x; } x = null; }
        function k5(x: string | null) { if (x) { [1].map(() => /*k5*/x); } x = null; }
        function k7() { let x: string | null = f(); if (x) { const g = () => { /*k7*/x; x = null; }; } }
        function k8() { let x: string | null = f(); if (x) { const g = () => /*k8*/x; } const h = () => { x = null; }; }
        function k10() { const x: string | null = f(); if (x) { function g() { /*k10*/x; } } }
        function k13() { const x: string | null = f(); if (x) { class D { m() { /*k13*/x; } } } }
        function k15(o: { v: string | null }) { if (o.v) () => /*k15*/o.v; }
        function k16() { var x: string | null = f(); if (x) () => /*k16*/x; }
        function k17() { let x: string | null = f(); for (let i = 0; i < 2; i++) { x = f<string | null>(); if (x) () => /*k17*/x; } }
        function k18() { let x: string | null = f(); if (x) { x = f<string | null>(); const h = () => /*k18*/x; } }
        function k24() { let x: string | null = f(); const g = () => /*k24*/x; if (!x) return; }
        function k29() { let x: string | null = f(); x = "a"; if (c) { x = null; } const g = () => /*k29*/x; }
        function k30() { let x: string | null = f(); if (x) { const g = () => /*k30*/x; } if (c) { x = "b"; } }
        function k34(x: string | null) { if (x) { const g = () => /*k34*/x; x = null; g(); } }
        function k38() { const x: string | null = f(); if (x) { class E { y = /*k38*/x; } } }
        function k39() { const x: string | null = f(); if (x) (class { y = /*k39*/x; }); }
        function k42() { const x: string | null = f(); if (x) { class E { [(/*k42*/x, "k")]() {} } } }
        "#,
        &[
            ("k3", "x", &["null", "string"]),
            ("k5", "x", &["null", "string"]),
            ("k7", "x", &["null", "string"]),
            ("k8", "x", &["null", "string"]),
            ("k10", "x", &["null", "string"]),
            ("k13", "x", &["null", "string"]),
            ("k15", "o.v", &["null", "string"]),
            ("k16", "x", &["null", "string"]),
            ("k17", "x", &["null", "string"]),
            ("k18", "x", &["null", "string"]),
            ("k24", "x", &["null", "string"]),
            ("k29", "x", &["null", "string"]),
            ("k30", "x", &["null", "string"]),
            ("k34", "x", &["null", "string"]),
            ("k38", "x", &["null", "string"]),
            ("k39", "x", &["null", "string"]),
            ("k42", "x", &["null", "string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives the narrower type in the comment for
// the read below. Biome evaluates no computed key of a class member.
#[test]
fn closures_that_biome_narrows_less() {
    assert_reads(
        r#"
        function k43() { const x: string | null = f(); if (x) { (class { [(/*k43*/x, "k")]() {} }); } }
        "#,
        &[
            // TypeScript: `string`.
            ("k43", "x", &["null", "string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// The elements of a destructuring pattern, a discriminated union, an aliased
// condition, and a capture through other closures continue too. A test inside
// a closure narrows further from the type where the closure is created.
#[test]
fn closures_narrow_patterns_unions_and_their_own_tests() {
    assert_reads(
        r#"
        type S = { kind: "c"; v: number } | { kind: "s"; v: string };
        function m1(s: S) { const { kind, v } = s; if (kind === "c") () => /*m1*/v; }
        function m2(s: Circle | Square) { if (s.kind === "c") () => /*m2*/s; }
        function m3(x: string | null) { const ok = x !== null; () => { if (ok) /*m3*/x; }; }
        function m4(x: string | null) { if (x === null) return; function g() { return () => /*m4*/x; } }
        function m5(x: string | null) { const g = () => { if (x === null) return; const h = () => /*m5*/x; }; }
        function m6(x: string | null) { if (x) { const o = { m() { return () => /*m6*/x; } }; } }
        function m7(xs: (string | null)[]) { for (const v of xs) { if (v) () => /*m7*/v; } }
        function m8(xs: (string | null)[]) { for (let v of xs) { if (v) () => /*m8*/v; } }
        function m9(x: string | null) { try { if (!x) throw 0; () => /*m9*/x; } catch {} }
        function m10(x: string | null) { switch (x) { case null: return; default: () => /*m10*/x; } }
        export let ex: string | null = f();
        if (ex) () => /*m19*/ex;
        function m11(x: string | null) { if (x) { const g = () => /*m20*/x; } else { x = "d"; } }
        function m12(x: string | null) { if (x) { const g = () => /*m21*/x; return g; } x = "d"; }
        function m13(x?: string) { if (x) () => /*m22*/x; }
        function m14(x: string | null = null) { if (x) () => /*m23*/x; }
        function m15(...[x]: [string | null]) { if (x) () => /*m24*/x; }
        function m16() { const x: string | number = f(); x satisfies string | number; if (typeof x === "string") () => /*m25*/x; }
        function m17(x: string | null) { x ??= "d"; () => /*m26*/x; }
        function own() { let x: string | null = f(); if (x) { const g = () => { if (x === "a") return; /*own*/x; }; } }
        function ownWritten() { let x: string | null = f(); if (x) { const g = () => { if (x === "a") return; /*ownWritten*/x; }; } x = null; }
        function nested(x: string | number | null) { if (x !== null) { const g = () => { if (typeof x === "string") return; /*nested*/x; }; } }
        "#,
        &[
            ("m1", "v", &["number"]),
            ("m2", "s", &["Circle"]),
            ("m3", "x", &["string"]),
            ("m4", "x", &["null", "string"]),
            ("m5", "x", &["string"]),
            ("m6", "x", &["string"]),
            ("m7", "v", &["string"]),
            ("m8", "v", &["string"]),
            ("m9", "x", &["string"]),
            ("m10", "x", &["string"]),
            ("m19", "ex", &["null", "string"]),
            ("m20", "x", &["null", "string"]),
            ("m21", "x", &["null", "string"]),
            ("m22", "x", &["string"]),
            ("m23", "x", &["string"]),
            ("m24", "x", &["string"]),
            ("m25", "x", &["string"]),
            ("m26", "x", &["string"]),
            ("own", "x", &["string"]),
            ("ownWritten", "x", &["null", "string"]),
            ("nested", "x", &["number"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// An assignment counts until the end of the outermost variable or expression
// statement, `if`, loop, `switch`, `try`, or class declaration around it that
// starts after the declaration, so a closure created inside that statement
// keeps the declared type.
#[test]
fn assignments_count_until_the_end_of_their_statement() {
    assert_reads(
        r#"
        function n1() { let x: string | null = f(); if (c) { x = "a"; const h = () => /*n1*/x; } }
        function n2() { let x: string | null = f(); x = "a"; const h = () => /*n2*/x; }
        function n3() { let x: string | null = f(); { x = "a"; } const h = () => /*n3*/x; }
        function n4() { let x: string | null = f(); { x = "a"; const h = () => /*n4*/x; } }
        function n5() { let x: string | null = f(); const h = (x = "a", () => /*n5*/x); }
        function n6(x: string | null) { if (x === null) x = "d"; return () => /*n6*/x; }
        function n7(x: string | null) { while (c) { if (x === null) x = "d"; } return () => /*n7*/x; }
        function n8(x: string | null) { x = "a"; for (const v of [1]) { const h = () => /*n8*/x; } }
        "#,
        &[
            ("n1", "x", &["null", "string"]),
            ("n2", "x", &["string"]),
            ("n3", "x", &["string"]),
            ("n4", "x", &["string"]),
            ("n5", "x", &["null", "string"]),
            ("n6", "x", &["string"]),
            ("n7", "x", &["null", "string"]),
            ("n8", "x", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A `catch` variable and a destructured `let` follow the rule of a `let`.
// An assignment in a `return` statement or a labeled block counts from its
// own position or its expression statement, a `for` statement's update
// from its own position, and an assignment in any function, even one called
// where it is defined, stops the narrowing in every closure.
#[test]
fn closures_follow_typescripts_rule_in_every_statement_kind() {
    assert_reads(
        r#"
        function a3() { try { f(); } catch (e) { if (typeof e === "string") () => /*a3*/e; } }
        function a4() { try { f(); } catch (e) { e = 1; if (typeof e === "string") () => /*a4*/e; } }
        function a5() { let { a } = f<{ a: string | null }>(); if (a) () => /*a5*/a; }
        function a6() { let { a } = f<{ a: string | null }>(); if (a) () => /*a6*/a; a = null; }
        function a7(x: string | null) { return (x = "a", () => /*a7*/x); }
        function a8(x: string | null) { lbl: { x = "a"; } const h = () => /*a8*/x; }
        function a9() { const x: string | null = f(); if (x) (class { y = () => /*a9*/x; }); }
        function a11() { for (let i: number | null = 0; i !== null && i < 3; i++) { () => /*a11*/i; } }
        function a12(x: string | null) { while (c) { if (!x) return; const h = () => /*a12*/x; x = f<string | null>(); } }
        function a13(x: string | null) { (() => { x = null; })(); if (x) () => /*a13*/x; }
        function a14(x: string | null) { x = "a"; x += "b"; const g = () => /*a14*/x; }
        function a15(x: string | null) { [x] = ["a"]; const g = () => /*a15*/x; }
        function a16(x: string | null) { for (x of ["a"]) {} if (x) () => /*a16*/x; }
        function a17(x: string | null) { switch (c) { case true: x = "a"; } if (x) () => /*a17*/x; }
        function a18(x: string | null) { try { x = "a"; } finally {} if (x) () => /*a18*/x; }
        function a19() { let x: string | null; x = "a"; const g = () => /*a19*/x; }
        namespace N { const x: string | null = f(); if (x) () => /*a29*/x; }
        const mc: string | null = f();
        function a20() { if (mc) () => /*a20*/mc; }
        function a21() { const x: string | null = f(); if (x) (class { get g() { /*a21*/x; return 1; } }); }
        function a22() { const x: string | null = f(); try { if (!x) return; } finally { () => /*a22*/x; } }
        function a23(o: { f?(v: unknown): void }) { const x: string | null = f(); if (x) o.f?.(() => /*a23*/x); }
        function a24() { const x: string | null = f(); x && (() => /*a24*/x); }
        function a25() { const x: string | null = f(); x ? () => /*a25*/x : null; }
        const a26 = (y: string | null) => { y = y ?? "d"; return () => /*a26*/y; };
        function a27(x: string | null) { if (x) { const g = () => () => /*a27*/x; } x = null; }
        function a30(x: string | null) { if (x === null) return; function g() { const h = () => /*a30*/x; } }
        function a31(x: string | null) { if (x === null) return; const g = function named() { return /*a31*/x; }; }
        function a32() { const x: string | null = f(); if (x) { const o = { m: function () { return () => /*a32*/x; } }; } }
        function a33() { const x: string | null = f(); if (x) { const o = { set s(v: number) { /*a33*/x; } }; } }
        function a35() { const x: string | null = f(); if (!x) throw 0; const g = () => /*a35*/x; }
        function a36() { const x: string | null = f(); if (x) { const g = () => { const y = /*a36*/x; }; } }
        function a37() { const x: string | null = f(); if (x !== null) { const g = () => { if (typeof x === "string") /*a37*/x; }; } }
        "#,
        &[
            ("a3", "e", &["string"]),
            ("a4", "e", &["string"]),
            ("a5", "a", &["string"]),
            ("a6", "a", &["null", "string"]),
            ("a7", "x", &["string"]),
            ("a8", "x", &["string"]),
            ("a9", "x", &["null", "string"]),
            ("a11", "i", &["number"]),
            ("a12", "x", &["null", "string"]),
            ("a13", "x", &["null", "string"]),
            ("a14", "x", &["string"]),
            ("a15", "x", &["string"]),
            ("a16", "x", &["string"]),
            ("a17", "x", &["string"]),
            ("a18", "x", &["string"]),
            ("a19", "x", &["string"]),
            ("a29", "x", &["string"]),
            ("a20", "mc", &["string"]),
            ("a21", "x", &["string"]),
            ("a22", "x", &["null", "string"]),
            ("a23", "x", &["string"]),
            ("a24", "x", &["string"]),
            ("a25", "x", &["string"]),
            ("a26", "y", &["string"]),
            ("a27", "x", &["null", "string"]),
            ("a30", "x", &["null", "string"]),
            ("a31", "x", &["string"]),
            ("a32", "x", &["string"]),
            ("a33", "x", &["string"]),
            ("a35", "x", &["string"]),
            ("a36", "x", &["string"]),
            ("a37", "x", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A closure nested in another one continues the flow of each closure around
// it, so the tests of every one of them narrow it, and an element of a
// destructuring pattern continues the narrowing of its pattern.
#[test]
fn chains_of_closures_resolve_outward() {
    assert_reads(
        r#"
        type S = { kind: "c"; v: number } | { kind: "s"; v: string };
        function ch1(x: string | number | null) { if (x !== null) { const g = () => { if (typeof x === "string") return; const h = () => /*ch1*/x; }; } }
        function ch2(x: string | number | null) { if (x !== null) { const g = () => { const h = () => { if (typeof x === "string") return; /*ch2*/x; }; }; } }
        function ch3(x: string | number | boolean | null) { if (x !== null) { const g = () => { if (typeof x === "string") return; const h = () => { if (typeof x === "number") return; /*ch3*/x; }; }; } }
        function pt1(s: S) { const { kind, v } = s; if (kind === "c") { const g = () => { if (typeof v === "number") /*pt1*/v; }; } }
        function pt2(s: S) { const { kind, v } = s; if (kind === "c") { const g = () => () => /*pt2*/v; } }
        function ch4(x: string | number | null) { if (x !== null) { const g = () => { if (typeof x === "string") return; /*ch4a*/x; const h = () => /*ch4b*/x; }; } }
        function ch5(x: string | null) { if (x) { const g = () => { /*ch5*/x; if (x === "a") {} }; } }
        "#,
        &[
            ("ch1", "x", &["number"]),
            ("ch2", "x", &["number"]),
            ("ch3", "x", &["boolean"]),
            ("pt1", "v", &["number"]),
            ("pt2", "v", &["number"]),
            ("ch4a", "x", &["number"]),
            ("ch4b", "x", &["number"]),
            ("ch5", "x", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A `let` without an initializer starts with `undefined` where it is
// declared, but TypeScript assumes it assigned there for a read in a nested
// function, a closure included, when any code assigns it (one of the unsound
// results that `NARROWING.md` lists as copied). A `let` that nothing assigns
// keeps `undefined`, and a read where `undefined` remains, although the
// declared type lacks it, gets the declared type (TS2454).
#[test]
fn closures_read_uninitialized_variables_from_their_start() {
    assert_reads(
        r#"
        function u1() { let x: string | null; if (c) x = "a"; const g = () => { if (x !== null) /*u1*/x; }; }
        function u3() { let x: string | number; if (typeof x === "string") { const g = () => /*u3*/x; } }
        function u4() { let x: string | null; if (c) x = "a"; if (typeof x === "string") { const g = () => /*u4*/x; } }
        function u5() { let x: string | null; if (c) x = "a"; const g = () => () => { if (x !== null) /*u5*/x; }; }
        function u6() { let x: string | null; if (c) { x = "a"; } else { x = null; } const g = () => /*u6*/x; }
        function u7() { let x: string | null; if (c) x = "a"; const g = () => /*u7*/x; }
        function u8() { let x: string | null; const g = () => { if (x !== null) /*u8*/x; }; }
        function u9() { let x: string | null; if (c) x = "a"; if (x !== null) { const g = () => /*u9*/x; } }
        function u10() { let x: string | null; if (c) x = "a"; if (x === undefined) { const g = () => /*u10*/x; } }
        function u11() { let x: string | undefined; if (c) x = "a"; const g = () => /*u11*/x; }
        function u12() { let x: string | null; if (c) x = "a"; if (x !== null) { const g = () => { if (x === "a") return; /*u12*/x; }; } }
        function u13() { let x: string | null; if (c) x = "a"; function h() { if (x !== null) /*u13*/x; } }
        "#,
        &[
            ("u1", "x", &["string"]),
            ("u3", "x", &["string"]),
            ("u4", "x", &["string"]),
            ("u5", "x", &["string"]),
            ("u6", "x", &["null", "string"]),
            ("u7", "x", &["null", "string"]),
            ("u8", "x", &["null", "string"]),
            ("u9", "x", &["string"]),
            // TypeScript: `never`.
            ("u10", "x", &["Unknown"]),
            ("u11", "x", &["string", "undefined"]),
            ("u12", "x", &["string"]),
            ("u13", "x", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A read in a closure created in unreachable code, or in unreachable code of
// a closure, gets the declared type, as a read in unreachable code does.
#[test]
fn closures_in_unreachable_code_keep_the_declared_type() {
    assert_reads(
        r#"
        function b3() { const x: string | null = f(); if (x) { return; /*b3*/x; } }
        function b5() { const x: string | null = f(); if (x) { const g = () => { if (c) return; else return; const h = () => /*b5*/x; }; } }
        function b6() { const x: string | null = f(); if (x) { const g = () => { while (true) {} const h = () => /*b6*/x; }; } }
        function b7() { const x: string | null = f(); if (x) { const g = () => { throw 0; const h = () => /*b7*/x; }; } }
        function b8() { const x: string | null = f(); if (x) { const g = () => { if (!c) return; const h = () => /*b8*/x; }; } }
        function b9() { const x: string | null = f(); if (x) { const g = () => { for (;;) { break; } const h = () => /*b9*/x; }; } }
        function b10() { const x: string | null = f(); if (x) { const g = () => { return () => /*b10*/x; }; } }
        function b11() { const x: string | null = f(); if (x) { const g = function () { return; const h = () => /*b11*/x; }; } }
        function b12() { let x: string | null = f(); if (x) { const g = () => { return; const h = () => /*b12*/x; }; } }
        function b13() { const x: string | null = f(); if (x) { const g = () => { return; if (x) { const h = () => /*b13*/x; } }; } }
        function b14() { const x: string | null = f(); if (x) { const g = () => { return; /*b14*/x; }; } }
        function b15() { const x: string | null = f(); if (x) { const g = () => () => { return; const h = () => /*b15*/x; }; } }
        "#,
        &[
            ("b3", "x", &["null", "string"]),
            ("b5", "x", &["null", "string"]),
            ("b6", "x", &["null", "string"]),
            ("b7", "x", &["null", "string"]),
            ("b8", "x", &["string"]),
            ("b9", "x", &["string"]),
            ("b10", "x", &["string"]),
            ("b11", "x", &["null", "string"]),
            ("b12", "x", &["null", "string"]),
            ("b13", "x", &["null", "string"]),
            ("b14", "x", &["null", "string"]),
            ("b15", "x", &["null", "string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `string | null` at every read
// below, because it finds the code after a call that returns `never` and the
// branch of `if (false)` unreachable. It gives a read there, and in a closure
// created there, the declared type.
#[test]
fn closures_in_code_that_typescript_finds_unreachable() {
    assert_reads(
        r#"
        function b1() { const x: string | null = f(); if (x) { fail(); /*b1*/x; } }
        function b2() { const x: string | null = f(); if (x) { fail(); const g = () => /*b2*/x; } }
        function b4() { const x: string | null = f(); if (x) { const g = () => { fail(); const h = () => /*b4*/x; }; } }
        function d1() { const x: string | null = f(); if (x) { if (false) { /*d1*/x; } } }
        function d5() { const x: string | null = f(); if (x) { if (false) { const g = () => /*d5*/x; } } }
        "#,
        &[
            ("b1", "x", &["null", "string"]),
            ("b2", "x", &["null", "string"]),
            ("b4", "x", &["null", "string"]),
            ("d1", "x", &["null", "string"]),
            ("d5", "x", &["null", "string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A loop whose test is not the literal `true`, as `while (1)`, has an exit,
// so a closure that a root doesn't analyze continues the flow past it, while
// `while (true)` ends every path. A closure created after a call to a method
// that returns `never` starts from that point, so its reads keep the
// declared type, unless another path reaches it.
#[test]
fn closures_after_loops_and_calls_that_end_the_path() {
    assert_reads(
        r#"
        declare const log: { fatal(): never };
        function w1() { const x: string | null = f(); if (x) { const g = () => { while (1) { if (c) break; } /*w1*/x; }; } }
        function w2() { const x: string | null = f(); if (x) { const g = () => { while (true) {} /*w2*/x; }; } }
        function w3() { const x: string | null = f(); if (x) { log.fatal(); const g = () => /*w3*/x; } }
        function w4() { const x: string | null = f(); if (x) { if (c) log.fatal(); const g = () => /*w4*/x; } }
        function w5() { const x: string | null = f(); if (x) { const g = () => { do { if (c) break; } while (1); /*w5*/x; }; } }
        "#,
        &[
            ("w1", "x", &["string"]),
            ("w2", "x", &["null", "string"]),
            ("w3", "x", &["null", "string"]),
            ("w4", "x", &["string"]),
            ("w5", "x", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `string` at every read below.
// Biome keeps the declared type of a `let` declared at the top level of a
// file, because it can't tell a module from a script, whose top-level `let`
// another script can write.
#[test]
fn closures_at_the_top_level_of_a_module() {
    assert_reads(
        r#"
        export const ec: string | null = f();
        if (ec) () => /*exported*/ec;
        const tc: string | null = f();
        if (tc) () => /*topConst*/tc;
        let top: string | null = f();
        if (top) () => /*top*/top;
        "#,
        &[
            ("exported", "ec", &["string"]),
            ("topConst", "tc", &["string"]),
            // TypeScript: `string`.
            ("top", "top", &["null", "string"]),
        ],
    );
}

// The reads in a closure narrow from the type where the closure is created,
// which the solve of the enclosing root gives at the capture point. That
// solve is the one that the enclosing root's own reads share, so the
// closure's reads add no solve of it, and lookups after complete module
// inference reuse every solve. An edit inside the closure replaces the
// module's collected flow, which every flow query reads, so the next lookup
// solves the enclosing root once more, once for all the reads of the
// closure.
#[test]
fn closures_share_the_solve_of_the_enclosing_root() {
    const SOURCE: &str = r#"
        declare function get(): string | number | null;
        function outer(x: string | number | null) {
            if (x !== null) {
                /*outer*/x;
                const g = () => { if (typeof x === "string") return; /*tested*/x; };
                const h = () => /*plain*/x;
                const k = () => /*other*/x;
            }
        }
    "#;
    let edited = SOURCE.replace("\"string\"", "\"number\"");
    let solves = |db: &TestModuleDb, events: &[salsa::Event]| {
        function_query_will_execute_count_by_name(db, "infer_flow_subject_states", events)
    };
    let read = |db: &TestModuleDb, module, source: &str, marker| {
        let range = marked_range(source, marker, "x");
        let ty = infer_expression_type(db, ExpressionTypeInput::new(db, module, range)).unwrap();
        variant_names(db, ty)
    };

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);
    fs.insert("/src/unrelated.ts".into(), "export const unused = 1;");
    let mut db = build_js_test_module_db(&fs, &["/src/index.ts", "/src/unrelated.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    let unrelated = db
        .module_for_path(Utf8Path::new("/src/unrelated.ts"))
        .unwrap();
    db.clear_salsa_events();
    assert_eq!(read(&db, module, SOURCE, "outer"), ["number", "string"]);
    assert_eq!(read(&db, module, SOURCE, "plain"), ["number", "string"]);
    assert_eq!(read(&db, module, SOURCE, "other"), ["number", "string"]);
    assert_eq!(read(&db, module, SOURCE, "tested"), ["number"]);
    let events = db.take_salsa_events();
    assert_eq!(
        solves(&db, &events),
        2,
        "one solve of `outer` for its read and the capture points, and one of `g`"
    );
    assert_function_query_was_not_run(&db, infer_module_types, module, &events);

    fs.insert("/src/unrelated.ts".into(), "export const unused = false;");
    let kind = resolve_js_module_kind_for_test(&fs, "/src/unrelated.ts", true);
    salsa::Setter::to(unrelated.set_kind(&mut db), kind);
    db.clear_salsa_events();
    assert_eq!(read(&db, module, SOURCE, "tested"), ["number"]);
    let events = db.take_salsa_events();
    assert_eq!(
        solves(&db, &events),
        0,
        "an edit of another module re-solves nothing"
    );

    fs.insert("/src/index.ts".into(), edited.as_str());
    let kind = resolve_js_module_kind_for_test(&fs, "/src/index.ts", true);
    salsa::Setter::to(module.set_kind(&mut db), kind);
    db.clear_salsa_events();
    assert_eq!(read(&db, module, &edited, "tested"), ["string"]);
    assert_eq!(read(&db, module, &edited, "plain"), ["number", "string"]);
    let events = db.take_salsa_events();
    assert_eq!(
        solves(&db, &events),
        2,
        "after an edit inside `g`, `outer` is solved once for both closures"
    );

    let (db, module) = narrowing_db(SOURCE);
    db.clear_salsa_events();
    let complete = infer_module_types(&db, module).unwrap();
    let range = marked_range(SOURCE, "tested", "x");
    assert_eq!(complete.expressions[&range], InferredTypeData::Number);
    db.clear_salsa_events();
    for marker in ["outer", "plain", "other", "tested"] {
        read(&db, module, SOURCE, marker);
    }
    let events = db.take_salsa_events();
    assert_eq!(
        solves(&db, &events),
        0,
        "lookups after complete module inference must reuse its solves"
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A function or arrow expression that is neither `async` nor a generator and
// that a call invokes where it is defined, through parentheses, also with
// `?.()` or when the call is the operand of `!` or `void`, runs in the flow
// of the code around the call: its arguments run,
// then its parameters and body, and the code after the call continues from
// the end of the body joined with each `return`. So a write in it narrows the
// code after the call, and a `throw` ends the path, also inside a nested
// inline function or a loop. A call through `.call`, a class constructed with
// `new`, and a function called later run elsewhere, and keep the narrowing.
// The body of an optional call, as in `(() => {})?.(x)`, runs before its
// arguments, as TypeScript's binder runs it. A read after a function that
// always throws is unreachable and keeps its declared type. A generator
// called in place is analyzed on its own. TypeScript reports TS1345 for
// testing the `void` result at `i15`, and still gives `null`.
#[test]
fn inline_functions_run_in_the_flow_where_they_are_called() {
    assert_reads(
        r#"
        function i1(x: string | null) { if (x) (() => /*i1*/x)(); }
        function i2(x: string | null) { if (x) { (() => { x = null; })(); /*i2*/x; } }
        function i3(x: string | null) { (() => { if (x === null) throw 0; })(); /*i3*/x; }
        function i4(x: string | null) { (function () { if (x === null) throw 0; })(); /*i4*/x; }
        function i5(x: string | null) { (() => { if (x === null) return; })(); /*i5*/x; }
        function i6(x: string | null) { (async () => { x = null; })(); /*i6*/x; }
        function i9(x: string | null) { if (x) { (() => { x = "a"; })(); /*i9*/x; } }
        function i10(x: string | null) { const r = (() => { if (x === null) throw 0; return 1; })(); /*i10*/x; }
        function i11(x: string | null) { ((y: string | null) => { if (y === null) throw 0; })(x); /*i11*/x; }
        function i13(x: string | null) { (() => { x = null; })(); /*i13*/x; }
        function i14(x: string | null) { if (x) { (() => { if (c) x = null; })(); /*i14*/x; } }
        function i15(x: string | null) { if (x) { !(() => { x = null; })(); /*i15*/x; } }
        function i16(x: string | null) { if (x) { (() => { x = null; }).call(undefined); /*i16*/x; } }
        function i17(x: string | null) { if (x) { new (class { constructor() { x = null; } })(); /*i17*/x; } }
        function i18() { let x: string | null = f(); (() => { x = "a"; })(); /*i18*/x; }
        function i19(x: string | null) { if (x) { void (() => { x = null; })(); /*i19*/x; } }
        function i20(x: string | null) { (() => { while (true) { if (x !== null) break; } })(); /*i20*/x; }
        function i21(x: string | null) { (() => { try { if (x === null) throw 0; } finally {} })(); /*i21*/x; }
        function i22(x: string | null) { if (x) { const g = () => { x = null; }; g(); /*i22*/x; } }
        function i23(x: string | null) { (() => { if (!x) return; /*i23*/x; })(); }
        function i24(x: string | null) { if (x) { ((() => { x = null; }))(); /*i24*/x; } }
        function i25(x: string | null) { if (x) { (() => { x = null; })?.(); /*i25*/x; } }
        function i26(o: { v: string | null }) { if (o.v) { (() => { o.v = null; })(); /*i26*/o.v; } }
        function i27(x: string | null) { let y: string | null = null; (() => { y = x; })(); /*i27*/y; }
        function i28(x: string | null) { (() => { x = "a"; })(); /*i28*/x; }
        function k41() { let x: string | null = f(); if (x) { (() => { x = null; })(); /*k41*/x; } }
        function n1(x: string | null) { (() => { (() => { if (x === null) throw 0; })(); })(); /*n1*/x; }
        function n2(x: string | null) { if (x) { (() => (() => { x = null; })())(); /*n2*/x; } }
        function l1(x: string | null) { while (c) { (() => { for (;;) { if (x === null) break; return; } })(); /*l1*/x; } }
        function d1(x: string | null) { ((y: string | null = x) => { if (y === null) throw 0; /*d1a*/y; })(); /*d1b*/x; }
        function d2(x: string | null) { if (x) { ((y: string = x) => /*d2*/y)(); } }
        function a1(x: string | null) { if (x) { ((v?: unknown) => { x = null; })(/*a1*/x); } }
        function a2(x: string | null) { if (x) { ((v?: unknown) => { x = null; })?.(/*a2*/x); } }
        function u1(x: string | null) { if (x) { (() => { throw 0; })(); /*u1*/x; } }
        function u2(x: string | null) { const v = c ? (() => { throw 0; })() : 1; /*u2*/x; }
        function y1(x: string | null) { (function* () { if (x === null) return; /*y1a*/x; })(); /*y1b*/x; }
        export const ex = ((y: string | null) => { if (y) return /*e1*/y; })(f());
        "#,
        &[
            ("i1", "x", &["string"]),
            ("i2", "x", &["null"]),
            ("i3", "x", &["string"]),
            ("i4", "x", &["string"]),
            ("i5", "x", &["null", "string"]),
            ("i6", "x", &["null", "string"]),
            ("i9", "x", &["string"]),
            ("i10", "x", &["string"]),
            ("i11", "x", &["null", "string"]),
            ("i13", "x", &["null"]),
            ("i14", "x", &["null", "string"]),
            ("i15", "x", &["null"]),
            ("i16", "x", &["string"]),
            ("i17", "x", &["string"]),
            ("i18", "x", &["string"]),
            ("i19", "x", &["null"]),
            ("i20", "x", &["string"]),
            ("i21", "x", &["string"]),
            ("i22", "x", &["string"]),
            ("i23", "x", &["string"]),
            ("i24", "x", &["null"]),
            ("i25", "x", &["null"]),
            ("i26", "o.v", &["null"]),
            ("i27", "y", &["null", "string"]),
            ("i28", "x", &["string"]),
            ("k41", "x", &["null"]),
            ("n1", "x", &["string"]),
            ("n2", "x", &["null"]),
            ("l1", "x", &["null", "string"]),
            ("d1a", "y", &["string"]),
            ("d1b", "x", &["null", "string"]),
            ("d2", "y", &["string"]),
            ("a1", "x", &["string"]),
            ("a2", "x", &["null"]),
            ("u1", "x", &["null", "string"]),
            ("u2", "x", &["null", "string"]),
            ("y1a", "x", &["string"]),
            ("y1b", "x", &["null", "string"]),
            ("e1", "y", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A class static block runs in the flow of the code around its class, where
// the class is evaluated, in member order and after its `extends` clause,
// also inside a function called where it is defined and inside another
// static block. A field initializer doesn't continue the flow.
#[test]
fn static_blocks_run_where_their_class_is_evaluated() {
    assert_reads(
        r#"
        function s1(x: string | null) { if (x) { class C { static { /*s1*/x; } } } }
        function s2(x: string | null) { if (x) { class C { static { x = null; } } /*s2*/x; } }
        function s3(x: string | null) { class C { static { if (x === null) throw 0; } } /*s3*/x; }
        function s4(x: string | null) { if (x) { const C = class { static { /*s4*/x; } }; } }
        function s8(x: string | null) { if (x) { class C { static a = /*s8*/x; } } }
        function s10(x: string | null) { if (x) { class C { static { (() => { x = null; })(); } } /*s10*/x; } }
        function s11(x: string | null) { if (x) { (() => { class C { static { x = null; } } })(); /*s11*/x; } }
        function s12(x: string | null) { class C { static { class D { static { if (x === null) throw 0; } } } } /*s12*/x; }
        function k36() { const x: string | null = f(); if (x) (class { static { /*k36*/x; } }); }
        function k37() { const x: string | null = f(); if (x) { class E { static { /*k37*/x; } } } }
        "#,
        &[
            ("s1", "x", &["string"]),
            ("s2", "x", &["null"]),
            ("s3", "x", &["string"]),
            ("s4", "x", &["string"]),
            ("s8", "x", &["null", "string"]),
            ("s10", "x", &["null"]),
            ("s11", "x", &["null"]),
            ("s12", "x", &["string"]),
            ("k36", "x", &["string"]),
            ("k37", "x", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A closure created inside an inline function continues the flow where the
// inline function creates it, and a read inside an inline function in a
// closure continues the closure's flow. A write in an inline function still
// counts as a write in another function for a closure that reads the
// variable, so the closure at `c3` keeps the declared type while the read
// beside it narrows. A read after an inline function that always throws is
// unreachable and keeps its declared type, also in a closure.
#[test]
fn closures_inside_inline_functions_continue_their_flow() {
    assert_reads(
        r#"
        function c1(x: string | null) { if (x) { (() => { const g = () => /*c1*/x; g(); })(); } }
        function c2(x: string | null) { (() => { if (x) { [1].forEach(() => /*c2*/x); } })(); }
        function c3(x: string | null) { (() => { x = "a"; })(); if (x) { [1].forEach(() => /*c3*/x); /*c3b*/x; } }
        function c4() { let y: string | null = f(); (() => { if (y) { const h = () => /*c4*/y; h(); } })(); }
        function c5(x: string | null) { if (x) { const g = () => { (() => { /*c5*/x; })(); }; } }
        function c6(x: string | null) { if (x) { const g = () => { (() => { throw 0; })(); /*c6*/x; }; } }
        "#,
        &[
            ("c1", "x", &["string"]),
            ("c2", "x", &["string"]),
            ("c3", "x", &["null", "string"]),
            ("c3b", "x", &["string"]),
            ("c4", "y", &["string"]),
            ("c5", "x", &["string"]),
            ("c6", "x", &["null", "string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// An arrow function called where it is defined reads and writes the `this`
// of the code around it, and `arguments` inside a function expression is its
// own, so writing an element of it changes no parameter of the code around
// it. TypeScript never narrows through `arguments`.
#[test]
fn inline_functions_keep_their_own_this_and_arguments() {
    assert_reads(
        r#"
        class K {
            v: string | null = null;
            m1() { if (this.v) { (() => { /*t1*/this.v; })(); } }
            m2() { if (this.v) { (() => { this.v = null; })(); /*t2*/this.v; } }
            m3() { if (this.v) { (function () {})(); /*t3*/this.v; } }
        }
        function g1(x: string | null) { if (x) { (function () { arguments[0] = null; })(); /*g1*/x; } }
        function g2(x: string | null) { if (x) { (() => { arguments[0] = null; })(); /*g2*/x; } }
        "#,
        &[
            ("t1", "this.v", &["string"]),
            ("t2", "this.v", &["null"]),
            ("t3", "this.v", &["string"]),
            ("g1", "x", &["string"]),
            ("g2", "x", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A `return` in an inline function leaves only that function, through its
// own finally blocks, and never reaches a finally block around the call. An
// inline function in a finally block, which Biome builds twice, joins its
// returns in each build.
#[test]
fn inline_functions_join_their_returns_through_finally_blocks() {
    assert_reads(
        r#"
        function f1(x: string | null) { (() => { try { if (x === null) return; } finally { /*f1a*/x; } /*f1b*/x; })(); /*f1c*/x; }
        function f2(x: string | null) { try { reset(); } finally { (() => { if (x === null) throw 0; })(); /*f2a*/x; } /*f2b*/x; }
        function f3(x: string | null) { try { (() => { if (x === null) return; })(); /*f3a*/x; } finally { /*f3b*/x; } }
        function f4(x: string | null) { try { reset(); } finally { (() => { if (x === null) return; })(); /*f4a*/x; } /*f4b*/x; }
        function f5(x: string | null) { try { (() => { try { if (x === null) return; } finally { reset(); } })(); /*f5a*/x; } finally { /*f5b*/x; } /*f5c*/x; }
        "#,
        &[
            ("f1a", "x", &["null", "string"]),
            ("f1b", "x", &["string"]),
            ("f1c", "x", &["null", "string"]),
            ("f2a", "x", &["string"]),
            ("f2b", "x", &["string"]),
            ("f3a", "x", &["null", "string"]),
            ("f3b", "x", &["null", "string"]),
            ("f4a", "x", &["null", "string"]),
            ("f4b", "x", &["null", "string"]),
            ("f5a", "x", &["null", "string"]),
            ("f5b", "x", &["null", "string"]),
            ("f5c", "x", &["null", "string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives the narrower type in the comment for
// each read below. Biome is wider. A write in an `async` function or a
// generator called where it is defined, or in a field initializer, keeps the
// variable at its declared type in the whole function, where TypeScript
// ignores the write. A function called in place inside a field initializer is
// part of that initializer, which runs apart from the code around the class.
// The parameter of an inline function gets no type from its argument. A
// write in an inline function reaches a `catch` clause around the call, where
// TypeScript starts the clause from the flow before the `try` statement and
// the writes of the try block's own code.
#[test]
fn inline_functions_that_biome_narrows_less() {
    assert_reads(
        r#"
        function i7(x: string | null) { if (x) { (async () => { x = null; })(); /*i7*/x; } }
        function i8(x: string | null) { if (x) { (function* () { x = null; })(); /*i8*/x; } }
        function i12(x: string | number) { ((y) => { /*i12*/y; })(x); }
        function s5(x: string | null) { if (x) { class C { static f = (x = null); } /*s5*/x; } }
        function s6(x: string | null) { if (x) { class C { f = (x = null); } /*s6*/x; } }
        function k1(x: string | null) { if (x) { class C { f = (() => { x = null; })(); } /*k1*/x; } }
        function t5(x: string | null) { if (x) { try { (() => { x = null; throw 0; })(); } catch { /*t5*/x; } } }
        "#,
        &[
            // TypeScript: `string`.
            ("i7", "x", &["null", "string"]),
            // TypeScript: `string`.
            ("i8", "x", &["null", "string"]),
            // TypeScript: `string | number`.
            ("i12", "y", &["Unknown"]),
            // TypeScript: `string`.
            ("s5", "x", &["null", "string"]),
            // TypeScript: `string`.
            ("s6", "x", &["null", "string"]),
            // TypeScript: `string`.
            ("k1", "x", &["null", "string"]),
            // TypeScript: `string`.
            ("t5", "x", &["null", "string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the type in its
// comment. A function nested in the code that declares a `var` assumes it
// assigned, also when it has no initializer and nothing writes it. It doesn't
// assume so for such a `let`, and TypeScript reports TS2454 at its reads and
// gives them the declared type.
#[test]
fn functions_assume_a_var_of_the_code_around_them_assigned() {
    assert_reads(
        r#"
        var v: string | null;
        function g() { if (v !== null) /*v*/v; }
        function outer() {
            var w: string | null;
            function inner() { if (w !== null) /*w*/w; }
            const arrow = () => { if (w !== null) /*arrow*/w; };
            let l: string | null;
            function letInner() { if (l !== null) /*l*/l; }
        }
        export {};
        "#,
        &[
            // TypeScript: `string`.
            ("v", "v", &["string"]),
            // TypeScript: `string`.
            ("w", "w", &["string"]),
            // TypeScript: `string`.
            ("arrow", "w", &["string"]),
            // TypeScript: `string | null`, with TS2454.
            ("l", "l", &["null", "string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the type in its
// comment. A read in a closure assumes a `let` without an initializer assigned
// where it is declared only when an `=`, a logical assignment, a
// destructuring assignment, or the head of a `for...in` or `for...of` loop
// assigns it somewhere. `++`, `+=`, and an assignment through `as` read the
// variable first, so they don't count. TypeScript reports TS2454 at the reads
// that only such writes assign, and gives them the declared type. A `var` is
// always assumed assigned in a closure, so `incrementVar` narrows.
#[test]
fn closures_assume_a_let_assigned_only_after_a_definite_assignment() {
    assert_reads(
        r#"
        declare const xs: number[];
        function a1() { let n: number | null; const g = () => { if (n !== null) /*incrementAfter*/n; }; n++; } // number | null
        function a2() { let n: number | null; if (c) n++; const g = () => { if (n !== null) /*increment*/n; }; } // number | null
        function a3() { let n: number | null; if (c) n += 1; const g = () => { if (n !== null) /*compound*/n; }; } // number | null
        function a4() { let n: number | null; if (c) (n as any) = 1; const g = () => { if (n !== null) /*asserted*/n; }; } // number | null
        function a5() { let n: number | null; if (c) n++; const g = () => /*incrementRead*/n; } // number | null
        function a6() { let n: number | null; if (c) n = 1; const g = () => { if (n !== null) /*assigned*/n; }; } // number
        function a7() { let n: number | null; if (c) n ??= 1; const g = () => { if (n !== null) /*logical*/n; }; } // number
        function a8() { let n: number | null; if (c) [n] = [1]; const g = () => { if (n !== null) /*array*/n; }; } // number
        function a9() { let n: number | null; if (c) ({ n } = { n: 1 }); const g = () => { if (n !== null) /*object*/n; }; } // number
        function a10() { let n: number | null; for (n of xs); const g = () => { if (n !== null) /*forOf*/n; }; } // number
        function a11() { let n: number | null; const g = () => { if (n !== null) /*assignedAfter*/n; }; n = 1; } // number
        function a12() { let n: number | null; if (c) n--; n = 2; const g = () => { if (n !== null) /*decrementThenAssigned*/n; }; } // number
        function a13() { var v: number | null; v++; const g = () => { if (v !== null) /*incrementVar*/v; }; } // number
        "#,
        &[
            ("incrementAfter", "n", &["null", "number"]),
            ("increment", "n", &["null", "number"]),
            ("compound", "n", &["null", "number"]),
            ("asserted", "n", &["null", "number"]),
            ("incrementRead", "n", &["null", "number"]),
            ("assigned", "n", &["number"]),
            ("logical", "n", &["number"]),
            ("array", "n", &["number"]),
            ("object", "n", &["number"]),
            ("forOf", "n", &["number"]),
            ("assignedAfter", "n", &["number"]),
            ("decrementThenAssigned", "n", &["number"]),
            ("incrementVar", "v", &["number"]),
        ],
    );
}
