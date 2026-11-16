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
// each read below. In TypeScript, a class static block continues the flow of
// the code around its class, and so does a function called where it is
// defined, so its writes change the reads after the call. Biome analyzes both
// on their own. Biome also evaluates no computed key of a class member.
#[test]
fn closures_that_biome_narrows_less() {
    assert_reads(
        r#"
        function k36() { const x: string | null = f(); if (x) (class { static { /*k36*/x; } }); }
        function k37() { const x: string | null = f(); if (x) { class E { static { /*k37*/x; } } } }
        function k41() { let x: string | null = f(); if (x) { (() => { x = null; })(); /*k41*/x; } }
        function k43() { const x: string | null = f(); if (x) { (class { [(/*k43*/x, "k")]() {} }); } }
        "#,
        &[
            // TypeScript: `string`.
            ("k36", "x", &["null", "string"]),
            // TypeScript: `string`.
            ("k37", "x", &["null", "string"]),
            // TypeScript: `null`.
            ("k41", "x", &["null", "string"]),
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
// function, a closure included, when any code assigns it (a deliberate-choice
// row of `NARROWING.md`). A `let` that nothing assigns keeps `undefined`, and
// a read where `undefined` remains, although the declared type lacks it, gets
// the declared type (TS2454).
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
