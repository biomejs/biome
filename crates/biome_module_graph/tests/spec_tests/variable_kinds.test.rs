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

/// Builds `/src/index.ts` from `source`, with each `(path, source)` of
/// `modules` as another module it can import.
fn narrowing_db_with(source: &str, modules: &[(&str, &str)]) -> (TestModuleDb, ModuleInfo) {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), source);
    let mut paths = vec!["/src/index.ts"];
    for (path, module) in modules {
        fs.insert((*path).into(), *module);
        paths.push(path);
    }
    let db = build_js_test_module_db(&fs, &paths, true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    (db, module)
}

/// Names each member of `ty`, sorted: an interface by its name, Biome's
/// Unknown as `Unknown`, and any other type by its formatted text on one
/// line, such as `string`, `null`, or `string: a`. A read on a path that no
/// value takes is Unknown.
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

/// Declarations shared by the tests.
const DECLARATIONS: &str = r#"
    declare function f<T>(): T;
    declare const c: boolean;
    declare function read(): string | number;
    declare function assertString(input: unknown): asserts input is string;
    declare function guard(input: unknown): input is string;
    declare const xs: (string | number)[];
    interface Circle { kind: "c"; r: number }
    interface Square { kind: "s"; w: number }
"#;

/// Asserts that [`variant_names`] gives `expected` for the type of each
/// `(marker, expression, expected)` read in `source`, and that complete
/// module inference gives each read the type that a lookup gives it.
/// `source` follows [`DECLARATIONS`] in the module.
fn assert_reads(source: &str, reads: &[(&str, &str, &[&str])]) {
    assert_reads_with(source, &[], reads);
}

/// Asserts the reads in `source` like [`assert_reads`], with each
/// `(path, source)` of `modules` as another module that `source` can import.
fn assert_reads_with(source: &str, modules: &[(&str, &str)], reads: &[(&str, &str, &[&str])]) {
    let source = format!("{DECLARATIONS}{source}");
    let lookups: Vec<_> = {
        let (db, module) = narrowing_db_with(&source, modules);
        reads
            .iter()
            .map(|(marker, expression, _)| {
                let range = marked_range(&source, marker, expression);
                infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range))
                    .map(|ty| format_inferred_type(&db, ty))
            })
            .collect()
    };
    let (db, module) = narrowing_db_with(&source, modules);
    let complete = infer_module_types(&db, module).unwrap();
    let mismatches: Vec<_> = reads
        .iter()
        .zip(lookups)
        .flat_map(|((marker, expression, expected), lookup)| {
            let range = marked_range(&source, marker, expression);
            let ty = execute_type_inference_request(
                &db,
                TypeInferenceCaller::new("test", "variableKinds"),
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

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected
// type. A `var` can be read before its declarator, where it is `undefined`.
// A read where `undefined` remains, although the declared type lacks it,
// gets the declared type (TS2454). Each declarator with an initializer
// writes the `var`, and so does each iteration of a `for...in` or `for...of`
// loop that declares it.
#[test]
fn var_declarations_narrow_from_their_hoisted_start() {
    assert_reads(
        r#"
        function v1() { var x: string | null = f(); if (x) /*v1*/x; }
        function v2() { var x: string | number = 1; /*v2*/x; }
        function v3() { /*v3*/x; var x: string | number = 1; }
        function v5() { if (c) { var x: string | null = f(); } if (x) /*v5*/x; }
        function v6() { for (var i: number | string = 0; i < 2; i++) /*v6*/i; }
        function v8() { var x: string | undefined; /*v8*/x; }
        function v9() { var x: string | null; if (x) /*v9*/x; }
        function v10() { if (c) { var x: string | null = "a"; } /*v10*/x; }
        function v11() { var x: string | null = f(); x = null; /*v11*/x; }
        function u6() { var x: string | null; if (x !== null) /*u6*/x; }
        function e1() { var other: string | number = read(); assertString(other); /*e1*/other; }
        function e2() { var hoisted: string | number = read(); if (guard(hoisted)) /*e2*/hoisted; }
        function e3() { for (var x of xs) {} /*e3*/x; }
        function e4() { for (var x of xs) {} if (x === undefined) /*e4*/x; }
        function e5() { x = 1; for (var x of xs) { /*e5*/x; } }
        function e6() { for (var k in xs) { if (k === "a") /*e6Body*/k; } /*e6After*/k; }
        function e18() { var { a } = f<{ a: string | null }>(); if (a) /*e18*/a; }
        function e19() { if (a === undefined) /*e19*/a; var { a } = f<{ a: string | null }>(); }
        function e22() { if (c) { var y: string | null = "a"; } else { y = null; } /*e22*/y; }
        function e23() { var z: string | null = null; if (c) { z = "a"; } if (z) /*e23*/z; }
        function e25() { if (m === null) { var m: string | null = "a"; /*e25*/m; } }
        function e30() { for (var i: number | string = 0; i < 2; i++) {} /*e30*/i; }
        function d1() { var x = f<string | null>(); if (x !== null) /*d1*/x; }
        function d2() { var { a } = f<{ a: string | null }>(); if (a !== null) /*d2*/a; }
        "#,
        &[
            ("v1", "x", &["string"]),
            ("v2", "x", &["number"]),
            ("v3", "x", &["number", "string"]),
            ("v5", "x", &["string"]),
            ("v6", "i", &["number"]),
            ("v8", "x", &["string", "undefined"]),
            ("v9", "x", &["string"]),
            ("v10", "x", &["null", "string"]),
            ("v11", "x", &["null"]),
            ("u6", "x", &["null", "string"]),
            ("e1", "other", &["string"]),
            ("e2", "hoisted", &["string"]),
            ("e3", "x", &["number", "string"]),
            ("e4", "x", &["number", "string"]),
            ("e5", "x", &["number", "string"]),
            ("e6Body", "k", &["string: a"]),
            ("e6After", "k", &["string"]),
            ("e18", "a", &["string"]),
            ("e19", "a", &["null", "string"]),
            ("e22", "y", &["null", "string"]),
            ("e23", "z", &["string"]),
            ("e25", "m", &["string"]),
            ("e30", "i", &["number"]),
            ("d1", "x", &["string"]),
            ("d2", "a", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives the narrower type in the comment
// for each read below, and `string | null` at `bothDeclared`. Biome keeps a
// `var` at its declared type when its scope declares its name twice or a
// parameter of its function declares it. The semantic model resolves every
// reference to the last declaration, while each declarator writes the one
// variable that they all declare, so narrowing would miss the write of the
// first declarator at `beforeLast`.
#[test]
fn redeclared_var_declarations_keep_their_declared_type() {
    assert_reads(
        r#"
        function v4() { var x: string | number = 1; var x: string | number = "a"; /*twice*/x; }
        function v7() { var x: string | number = 1; { var x: string | number = "a"; } /*block*/x; }
        function e7(x: string | null) { var x: string | null; if (x) /*parameter*/x; }
        function e24() { var q: string | null = f(); if (q) { var q: string | null = null; } /*bothDeclared*/q; }
        function r1() { if (x === null) { var x: string | null = "a"; /*beforeLast*/x; } var x: string | null; }
        "#,
        &[
            // TypeScript: `string`.
            ("twice", "x", &["number", "string"]),
            // TypeScript: `string`.
            ("block", "x", &["number", "string"]),
            // TypeScript: `string`.
            ("parameter", "x", &["null", "string"]),
            ("bothDeclared", "q", &["null", "string"]),
            // TypeScript: `string`.
            ("beforeLast", "x", &["null", "string"]),
        ],
    );
}

const IMPORTED: &str = r#"
    export declare const x: string | null;
    export declare let y: string | null;
    export declare const o: { v: string | null };
    declare const d: string | null;
    export default d;
    export declare function g(): void;
"#;

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m5/imports/main.ts` L5 to L14). An import narrows by the
// tests of the root that reads it, from the declared type of what it
// imports, a live `let` too, and a call never changes it. A test outside a
// closure doesn't reach a read of an import inside it.
#[test]
fn imported_bindings_narrow() {
    assert_reads_with(
        r#"
        import { x, y, o, g } from "./a";
        import d from "./a";
        import * as ns from "./a";
        function i1() { if (x) /*i1*/x; }
        function i2() { if (y) /*i2*/y; }
        function i3() { if (o.v) /*i3*/o.v; }
        function i4() { if (d) /*i4*/d; }
        function i5() { if (ns.x) /*i5*/ns.x; }
        function i6() { if (ns.o.v) /*i6*/ns.o.v; }
        function i7() { if (x) { g(); /*i7*/x; } }
        function i8() { if (x) () => /*i8*/x; }
        function i9() { if (y) () => /*i9*/y; }
        if (x) /*i14*/x;
        "#,
        &[("/src/a.ts", IMPORTED)],
        &[
            ("i1", "x", &["string"]),
            ("i2", "y", &["string"]),
            ("i3", "o.v", &["string"]),
            ("i4", "d", &["string"]),
            ("i5", "ns.x", &["string"]),
            ("i6", "ns.o.v", &["string"]),
            ("i7", "x", &["string"]),
            ("i8", "x", &["null", "string"]),
            ("i9", "y", &["null", "string"]),
            ("i14", "x", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected
// type, unless a comment gives TypeScript's type. A function narrows a
// variable that an enclosing root declares by its own tests and writes,
// from the declared type. Writes in other functions and in the enclosing
// root never change its type there. A `let` without an initializer that
// nothing writes may be read before it is assigned (TS2454). Biome reads
// such a `var` the same way, where TypeScript assumes it initialized.
#[test]
fn variables_of_enclosing_roots_narrow_by_the_reading_root() {
    assert_reads(
        r#"
        const config: { v: string | null } = f();
        let s: string | null = f();
        let h: { v: string | null } = f();
        var w: string | null = f();
        function o1() { if (config.v) /*o1*/config.v; }
        function o2() { if (s) /*o2*/s; }
        function o3() { if (h.v) /*o3*/h.v; }
        function o4() { if (w) /*o4*/w; }
        function o5() { if (s) { s = null; /*o5*/s; } }
        function o6() { s = "a"; /*o6*/s; }
        function o7(x: string | null) { return () => { if (x) /*o7*/x; }; }
        function o8() { if (config.v) { config.v = null; /*o8*/config.v; } }
        function o9() { h = { v: "a" }; /*o9*/h.v; }
        function o10() { if (h.v) { h = f(); /*o10*/h.v; } }
        function o11() { if (s) /*o11*/s; s = null; }
        function o12() { if (c) { s = "s"; } /*o12*/s; }
        const o13 = () => { if (config.v) /*o13*/config.v; };
        function o14(x: string | null) { function inner() { if (x) /*o14*/x; } }
        function u7() { let x: string | null; const g = () => { if (x !== null) /*u7*/x; }; }
        var w2: string | null; function e8() { if (w2 !== null) /*outerVar*/w2; }
        let g2: string | null; function e9() { g2 = "a"; } function e10() { if (g2 !== null) /*writtenElsewhere*/g2; }
        let g3: string | null = f(); g3 = null; function e11() { if (g3) /*writtenAround*/g3; }
        let g5: string | null = f(); function e13() { if (g5) { const h = () => { g5 = null; }; h(); /*callback*/g5; } }
        export let g6: string | null = f(); function e14() { if (g6) /*exported*/g6; }
        let g7: string | number; function e16() { if (typeof g7 === "string") /*typeofUnassigned*/g7; }
        let g8: string | null; function e17() { if (g8 !== null) /*writtenLater*/g8; g8 = "a"; }
        const { kind, v } = f<{ kind: "a"; v: string } | { kind: "b"; v: number }>(); function e20() { if (kind === "a") /*pattern*/v; }
        let g9: string | null = f(); class K { m() { if (g9) /*method*/g9; } }
        let g10: { v: string | null } = f(); function e26() { if (g10.v) { g10 = f(); /*prefixWritten*/g10.v; } }
        const g11 = f<string | null>(); const ok11 = g11 !== null; function e27() { if (ok11) /*moduleAlias*/g11; }
        function e28(x: string | null) { const ok = x !== null; const g = () => { if (ok) /*enclosingAlias*/x; }; }
        let g12: string | null = f(); function e29() { if (g12) { g12 = null; } /*joined*/g12; }
        function held(s: Circle | Square, t: Circle | Square) { const k = s.kind; s = t; const g = () => { if (k === "c") /*heldOutside*/s; }; }
        "#,
        &[
            ("o1", "config.v", &["string"]),
            ("o2", "s", &["string"]),
            ("o3", "h.v", &["string"]),
            ("o4", "w", &["string"]),
            ("o5", "s", &["null"]),
            ("o6", "s", &["string"]),
            ("o7", "x", &["string"]),
            ("o8", "config.v", &["null"]),
            ("o9", "h.v", &["null", "string"]),
            ("o10", "h.v", &["null", "string"]),
            ("o11", "s", &["string"]),
            ("o12", "s", &["null", "string"]),
            ("o13", "config.v", &["string"]),
            ("o14", "x", &["string"]),
            ("u7", "x", &["null", "string"]),
            // TypeScript: `string`.
            ("outerVar", "w2", &["null", "string"]),
            ("writtenElsewhere", "g2", &["string"]),
            ("writtenAround", "g3", &["string"]),
            ("callback", "g5", &["string"]),
            ("exported", "g6", &["string"]),
            ("typeofUnassigned", "g7", &["string"]),
            ("writtenLater", "g8", &["string"]),
            ("pattern", "v", &["string"]),
            ("method", "g9", &["string"]),
            ("prefixWritten", "g10.v", &["null", "string"]),
            ("moduleAlias", "g11", &["string"]),
            ("enclosingAlias", "x", &["string"]),
            ("joined", "g12", &["null", "string"]),
            ("heldOutside", "s", &["Circle"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives the narrower type in the comment for
// `invoked` (`narrowing-probes/m5-l3/extra.ts` L19), and `never` at
// `captured` (L22), which Biome reads as Unknown. Biome keeps the declared
// type of a variable that a function called where it is defined writes,
// because the graph has no write point there. A closure continues the flow
// where it is created, where `x` is `null`.
#[test]
fn variables_of_enclosing_roots_that_biome_narrows_less() {
    assert_reads(
        r#"
        let g4: string | null = f(); function e12() { if (g4) { (() => { g4 = null; })(); /*invoked*/g4; } }
        function e15(x: string | null) { x = null; return () => { if (x) /*captured*/x; }; }
        "#,
        &[
            // TypeScript: `null`.
            ("invoked", "g4", &["null", "string"]),
            ("captured", "x", &["Unknown"]),
        ],
    );
}

// Lookups resolve the type that narrowing of an import starts from once,
// through the `infer_flow_binding_baseline` query, and the reads in every
// function share it. Complete module inference resolves that type in its
// own context, from the import that it has resolved already. Later lookups
// of reads that start from the same type reuse its solves.
#[test]
fn imported_baselines_are_shared_by_the_roots_that_read_them() {
    const SOURCE: &str = r#"
        import { x } from "./a";
        function first() { if (x) { /*first*/x; } }
        function second() { if (x !== null) { /*second*/x; } }
    "#;
    let modules = [("/src/a.ts", IMPORTED)];
    let read = |db: &TestModuleDb, module, marker| {
        let range = marked_range(SOURCE, marker, "x");
        let ty = infer_expression_type(db, ExpressionTypeInput::new(db, module, range)).unwrap();
        variant_names(db, ty)
    };
    let baselines = |db: &TestModuleDb, events: &[salsa::Event]| {
        function_query_will_execute_count_by_name(db, "infer_flow_binding_baseline", events)
    };

    let (db, module) = narrowing_db_with(SOURCE, &modules);
    db.clear_salsa_events();
    assert_eq!(read(&db, module, "first"), ["string"]);
    assert_eq!(read(&db, module, "second"), ["string"]);
    let events = db.take_salsa_events();
    assert_eq!(
        baselines(&db, &events),
        1,
        "the reads of `x` in both functions share its baseline"
    );
    assert_function_query_was_not_run(&db, infer_module_types, module, &events);

    let (db, module) = narrowing_db_with(SOURCE, &modules);
    db.clear_salsa_events();
    let complete = infer_module_types(&db, module).unwrap();
    let range = marked_range(SOURCE, "first", "x");
    assert_eq!(complete.expressions[&range], InferredTypeData::String);
    let events = db.take_salsa_events();
    assert_eq!(
        baselines(&db, &events),
        0,
        "complete module inference must resolve the baseline itself"
    );
    db.clear_salsa_events();
    assert_eq!(read(&db, module, "first"), ["string"]);
    assert_eq!(read(&db, module, "second"), ["string"]);
    let events = db.take_salsa_events();
    assert_eq!(baselines(&db, &events), 1);
    for query in ["infer_flow_subject_states", "infer_flow_subject_type"] {
        assert_eq!(
            function_query_will_execute_count_by_name(&db, query, &events),
            0,
            "lookups starting from the same baseline must reuse the solves: {query}"
        );
    }
}
