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
fn narrowing_db(source: &str, modules: &[(&str, &str)]) -> (TestModuleDb, ModuleInfo) {
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

/// Names each member of `ty`, sorted: Biome's Unknown as `Unknown`, and any
/// other type by its formatted text on one line, such as `string`, `null`, or
/// `instanceof Array<number>`.
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

/// Declarations shared by the tests.
const DECLARATIONS: &str = r#"
    declare function f<T>(): T;
    declare const c: boolean;
    declare const n: number;
"#;

/// Asserts the type of each `(marker, expression, expected)` read in `source`,
/// which follows [`DECLARATIONS`], by the names of its members from
/// [`variant_names`]. Also asserts that complete module inference gives each
/// read the type that a lookup gives it.
fn assert_reads(source: &str, reads: &[(&str, &str, &[&str])]) {
    assert_reads_with(source, &[], reads);
}

/// Asserts the reads of `source` as [`assert_reads`] does, where `source` can
/// import each `(path, source)` of `modules`.
fn assert_reads_with(source: &str, modules: &[(&str, &str)], reads: &[(&str, &str, &[&str])]) {
    let source = format!("{DECLARATIONS}{source}");
    let lookups: Vec<_> = {
        let (db, module) = narrowing_db(&source, modules);
        reads
            .iter()
            .map(|(marker, expression, _)| {
                let range = marked_range(&source, marker, expression);
                infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range))
                    .map(|ty| format_inferred_type(&db, ty))
            })
            .collect()
    };
    let (db, module) = narrowing_db(&source, modules);
    let complete = infer_module_types(&db, module).unwrap();
    let mismatches: Vec<_> = reads
        .iter()
        .zip(lookups)
        .flat_map(|((marker, expression, expected), lookup)| {
            let range = marked_range(&source, marker, expression);
            let ty = execute_type_inference_request(
                &db,
                TypeInferenceCaller::new("test", "evolving"),
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
// A `let` or a `var` without a type annotation, declared without an
// initializer or with `null` or `undefined`, starts at `undefined` and takes
// the type of each value assigned to it, its literal types widened, in the
// function that declares it.
#[test]
fn evolving_variables_take_the_types_of_their_writes() {
    assert_reads(
        r#"
        function e1() { let x; x = 1; /*e1*/x; }
        function e2() { let x; if (c) x = 1; else x = "s"; /*e2*/x; }
        function e4() { let x = null; x = "a"; /*e4*/x; }
        function e5() { let x = undefined; x = 1; /*e5*/x; }
        function e9() { let x; if (c) x = 1; /*e9*/x; }
        function e10() { let x; x = 1; x = "a"; /*e10*/x; }
        function e11() { let x = null; if (c) x = 1; /*e11*/x; }
        function e14() { let x; x = { k: 1 }; /*e14*/x.k; }
        function e15() { var x; x = 1; /*e15*/x; }
        function v1() { let x; x ??= 1; /*v1*/x; }
        function v4() { let x; [x] = [1]; /*v4*/x; }
        function v5() { let x; ({ x } = { x: 1 }); /*v5*/x; }
        function v6() { let x; for (x of [1, 2]) {} /*v6*/x; }
        function v7() { let x; for (x in { a: 1 }) {} /*v7*/x; }
        function v8() { let x; x = c ? 1 : "a"; /*v8*/x; }
        function v11() { let x; if (x === undefined) x = 1; /*v11*/x; }
        function v12() { let x; if (c) x = 1; if (typeof x === "number") /*v12*/x; }
        function v13() { let x; if (c) x = "a"; if (x) /*v13*/x; }
        function v14() { let x; while (c) { x = 1; } /*v14*/x; }
        function v15() { let x; try { x = 1; } catch { } /*v15*/x; }
        function v18() { let x, y = null; x = 1; y = "s"; /*v18*/x; /*v18b*/y; }
        function v20() { let x; x = f<string>(); /*v20*/x; }
        function v21() { let x; x = { k: 1 }; /*v21*/x; }
        function v22() { let x; x = [1]; /*v22*/x; }
        function v25() { let x; x = new Date(); /*v25*/x; }
        function v32() { /*v32*/x; var x = null; x = 1; }
        function v33() { let x; x = true; /*v33*/x; }
        function v36() { let x; x = 1; x = undefined; /*v36*/x; }
        function v38() { let x; x = n; /*v38*/x; }
        function v43() { let x = null; switch (n) { case 1: x = "a"; break; default: x = 1; } /*v43*/x; }
        function f1() { for (let x; ;) { x = 1; /*f1*/x; break; } }
        function f3() { let x = null; while (c) { /*f3*/x; x = 1; } }
        function g1() { let x = null; if (c) x = 1; if (x === null) return; /*g1*/x; }
        function g3() { let x; x = 1; if (x === 2) /*g3*/x; }
        function u1() { let x = null; x = undefined; /*u1*/x; }
        function u2() { let x = undefined; if (c) x = null; /*u2*/x; }
        "#,
        &[
            ("e1", "x", &["number"]),
            ("e2", "x", &["number", "string"]),
            ("e4", "x", &["string"]),
            ("e5", "x", &["number"]),
            ("e9", "x", &["number", "undefined"]),
            ("e10", "x", &["string"]),
            ("e11", "x", &["null", "number"]),
            ("e14", "x.k", &["number"]),
            ("e15", "x", &["number"]),
            ("v1", "x", &["number"]),
            ("v4", "x", &["number"]),
            ("v5", "x", &["number"]),
            ("v6", "x", &["number", "undefined"]),
            ("v7", "x", &["string", "undefined"]),
            ("v8", "x", &["number", "string"]),
            ("v11", "x", &["number"]),
            ("v12", "x", &["number"]),
            ("v13", "x", &["string"]),
            ("v14", "x", &["number", "undefined"]),
            ("v15", "x", &["number", "undefined"]),
            ("v18", "x", &["number"]),
            ("v18b", "y", &["string"]),
            ("v20", "x", &["string"]),
            (
                "v21",
                "x",
                &[r#"Object { prototype: No prototype members: ["k": number] }"#],
            ),
            ("v22", "x", &["instanceof Array<number>"]),
            ("v25", "x", &["instanceof Date"]),
            ("v32", "x", &["undefined"]),
            ("v33", "x", &["boolean"]),
            ("v36", "x", &["undefined"]),
            ("v38", "x", &["number"]),
            ("v43", "x", &["number", "string"]),
            ("f1", "x", &["number"]),
            ("f3", "x", &["null", "number"]),
            ("g1", "x", &["number"]),
            ("g3", "x", &["number: 2"]),
            ("u1", "x", &["undefined"]),
            ("u2", "x", &["null", "undefined"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A variable initialized with an empty array literal starts with the element
// type `never` and joins the widened type of each value that `push`,
// `unshift`, or an assignment to an element with a number key adds to it.
#[test]
fn evolving_arrays_take_the_types_of_their_elements() {
    assert_reads(
        r#"
        function e6() { let a = []; a.push(1); /*e6*/a; }
        function e7() { let a = []; a[0] = "s"; /*e7*/a; }
        function e8() { const a = []; a.push(1); a.push("s"); /*e8*/a; }
        function e12() { let a = []; a.unshift(true); /*e12*/a; }
        function e13() { let a = []; /*e13a*/a.length; a.push(1); /*e13*/a; }
        function e16() { let a = []; for (const v of [1, 2]) a.push(v); /*e16*/a; }
        function e19() { let x = []; x = [1]; /*e19*/x; }
        function a2() { const a = []; a.push(1, "s"); /*a2*/a; }
        function a6() { const a = []; a.push(1); a[0] += 1; [a[0]] = [2]; a[0]++; /*a6*/a; }
        function a9() { let a = []; a = [1]; a.push(2); /*a9*/a; }
        function a11() { const a = []; a?.push(1); /*a11*/a; }
        function a12() { const a = []; (a).push(1); a!.unshift(2); /*a12*/a; }
        function a16() { const a = []; if (c) a.push(1); else a.push("s"); /*a16*/a; }
        function a17() { const a = []; while (c) { a.push(1); } /*a17*/a; }
        function a20() { const a = []; a.push({ k: 1 }); /*a20*/a; }
        function a26() { let a = []; a.push(1); a = []; /*a26*/a.length; a.push("s"); /*a26b*/a; }
        function a29() { const a = []; a.push(1); if (a) /*a29*/a; }
        function a32() { let a = []; a = f<string[]>(); /*a32*/a; }
        function a36() { const a = []; a.push(1); const g = () => a.push(2); /*a36*/a; }
        function a42() { const a = []; a.push([1]); /*a42*/a; }
        function a44() { var a = []; a.push(1); /*a44*/a; }
        function a45() { const a = [], b = []; a.push(1); b.push("s"); /*a45*/a; /*a45b*/b; }
        function u5() { const a = []; a.push(null); a.push(undefined); /*u5*/a; }
        "#,
        &[
            ("e6", "a", &["instanceof Array<number>"]),
            ("e7", "a", &["instanceof Array<string>"]),
            ("e8", "a", &["instanceof Array<number | string>"]),
            ("e12", "a", &["instanceof Array<boolean>"]),
            ("e13a", "a.length", &["number"]),
            ("e13", "a", &["instanceof Array<number>"]),
            ("e16", "a", &["instanceof Array<number>"]),
            ("e19", "x", &["instanceof Array<number>"]),
            ("a2", "a", &["instanceof Array<number | string>"]),
            ("a6", "a", &["instanceof Array<number>"]),
            ("a9", "a", &["instanceof Array<number>"]),
            ("a11", "a", &["instanceof Array<number>"]),
            ("a12", "a", &["instanceof Array<number>"]),
            ("a16", "a", &["instanceof Array<number | string>"]),
            ("a17", "a", &["instanceof Array<number>"]),
            (
                "a20",
                "a",
                &[r#"instanceof Array<Object { prototype: No prototype members: ["k": number] }>"#],
            ),
            ("a26", "a.length", &["number"]),
            ("a26b", "a", &["instanceof Array<string>"]),
            ("a29", "a", &["instanceof Array<number>"]),
            ("a32", "a", &["instanceof Array<string>"]),
            ("a36", "a", &["instanceof Array<number>"]),
            ("a42", "a", &["instanceof Array<instanceof Array<number>>"]),
            ("a44", "a", &["instanceof Array<number>"]),
            ("a45", "a", &["instanceof Array<number>"]),
            ("a45b", "b", &["instanceof Array<string>"]),
            ("u5", "a", &["instanceof Array<null | undefined>"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives each read below a narrower type
// than Biome, in the comment after it. Biome keeps the declared type where
// only the initializer reaches the read, where nothing but `null`, or nothing
// but `undefined`, is assigned, in a closure or a nested function, where
// TypeScript reports an implicit `any` (TS7005), and for an array to which a
// write that reaches the read adds a value that Biome may type differently
// from TypeScript, or to which a function called where it is defined adds. A
// value that reads such a variable has Biome's type for it, which is its
// declared type.
#[test]
fn evolving_declarations_keep_their_declared_type_where_biome_cant_follow_them() {
    assert_reads(
        r#"
        function e3() { let x; /*e3*/x; } // undefined
        function e17() { let x; const g = () => /*e17*/x; x = 1; } // any
        function v10() { let x; x = null; /*v10*/x; } // null
        function v16() { let x; function g() { x = 1; } g(); /*v16*/x; } // undefined
        function v26() { let x; x = 1; const y = x; /*v26*/y; } // number
        function a1() { const a = []; a.push(...[1, 2]); /*a1*/a; } // number[]
        function a3() { const a = []; a[n] = 1; /*a3*/a; } // number[]
        function a8() { const a = []; a.push(1); (() => a.push("s"))(); /*a8*/a; } // (string | number)[]
        function a21() { const a = []; a.push(null); /*a21*/a; } // null[]
        function a43() { const a = []; a.push(() => 1); /*a43*/a; } // (() => number)[]
        function r1() { let e = null; e = "a"; let y: string | null = f(); y = e; /*r1*/y; } // string
        function r4() { let e; e = 1; let w: number | string = f(); w = e; /*r4*/w; } // number
        "#,
        &[
            ("e3", "x", &["Unknown"]),
            ("e17", "x", &["Unknown"]),
            ("v10", "x", &["Unknown"]),
            ("v16", "x", &["Unknown"]),
            ("v26", "y", &["Unknown"]),
            ("a1", "a", &["instanceof Array<unknown>"]),
            ("a3", "a", &["instanceof Array<unknown>"]),
            ("a8", "a", &["instanceof Array<unknown>"]),
            ("a21", "a", &["instanceof Array<unknown>"]),
            ("a43", "a", &["instanceof Array<unknown>"]),
            ("r1", "y", &["null", "string"]),
            ("r4", "w", &["number", "string"]),
        ],
    );
}

/// The module that the tests of writes import as `./lib`, with inferred and
/// annotated return types.
const LIBRARY: &str = r#"
    export function loadInferred() { return 1; }
    export function loadAnnotated(): number { return 1; }
    export async function fetchInferred() { return { id: 1 }; }
    export async function fetchAnnotated(): Promise<{ id: number }> { return { id: 1 }; }
"#;

/// Declarations that the tests of writes share, after [`DECLARATIONS`].
const WRITES: &str = r#"
    import { fetchAnnotated, fetchInferred, loadAnnotated, loadInferred } from "./lib";
    declare const s: string;
    declare const vs: number[];
    declare const tuple: [number];
    declare const map: Map<string, number>;
    declare function computeAnnotated(): number;
    function computeInferred() { return 1; }
    const computeArrow = () => 1;
    declare function fetchNumber(): Promise<number>;
    async function fetchLocal() { return 1; }
    const store = { load() { return 1; } };
    declare const literal: { method(): number };
    type Alias = { method(): number };
    declare const aliased: Alias;
    interface Service { method(): number }
    declare const service: Service;
    class K { annotated(): number { return 1; } inferred() { return 1; } }
    declare const k: K;
    class C { v = 1 }
"#;

/// Asserts the reads of `source`, which follows [`DECLARATIONS`] and
/// [`WRITES`] and can import [`LIBRARY`], as [`assert_reads`] does.
fn assert_writes(source: &str, reads: &[(&str, &str, &[&str])]) {
    assert_reads_with(
        &format!("{WRITES}{source}"),
        &[("/src/lib.ts", LIBRARY)],
        reads,
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A write whose value Biome types as TypeScript does gives the variable that
// value's type: a call to a function or a method with a return type
// annotation, including an imported one and one from the lib, a call to a
// local function without one whose returned values Biome types so, `await`
// of such a call, `new` with type arguments, and a conditional expression of
// such values.
#[test]
fn evolving_variables_take_the_types_of_writes_that_biome_types() {
    assert_writes(
        r#"
        function t01() { let x = null; x = computeAnnotated(); /*t01*/x; }
        function t02() { let x = null; x = computeInferred(); /*t02*/x; }
        function t03() { let x = null; x = computeArrow(); /*t03*/x; }
        function t04() { let x = null; x = loadAnnotated(); /*t04*/x; }
        function t08() { let x = null; x = service.method(); /*t08*/x; }
        function t09() { let x = null; x = k.annotated(); /*t09*/x; }
        function t11() { let x = null; x = map.get(s); /*t11*/x; }
        function t12() { let x = null; x = vs.find((v) => v > 1); /*t12*/x; }
        function t13() { let x = null; x = s.trim(); /*t13*/x; }
        async function t15() { let x = null; x = await fetchNumber(); /*t15*/x; }
        async function t16() { let x = null; x = await fetchLocal(); /*t16*/x; }
        async function t18() { let x = null; x = await fetchAnnotated(); /*t18*/x; }
        function t19() { let x = null; x = new C(); /*t19*/x; }
        function t20() { let x = null; x = new Map<string, number>(); /*t20*/x; }
        function t27() { let x; x = c ? "a" : 1; /*t27*/x; }
        function t28() { let x = null; x = c ? "a" : 1; /*t28*/x; }
        function t31() { let x = null; for (const v of vs) { x = v; } /*t31*/x; }
        function t48() { let x = null; x = 1; /*t48*/x; }
        function t59() { let x = null; x = store.load(); x = 1; /*t59*/x; }
        function t79() { let x = null; const g = () => { x = store.load(); }; x = 1; /*t79*/x; }
        function t91() { let x = null; /*t91*/x; }
        function t92() { let x = null; if (c) { x = 1; } x = computeArrow(); /*t92*/x; }
        "#,
        &[
            ("t01", "x", &["number"]),
            ("t02", "x", &["number"]),
            ("t03", "x", &["number"]),
            ("t04", "x", &["number"]),
            ("t08", "x", &["number"]),
            ("t09", "x", &["number"]),
            ("t11", "x", &["number", "undefined"]),
            ("t12", "x", &["number", "undefined"]),
            ("t13", "x", &["string"]),
            ("t15", "x", &["number"]),
            ("t16", "x", &["number"]),
            (
                "t18",
                "x",
                &[r#"Object { prototype: No prototype members: ["id": number] }"#],
            ),
            ("t19", "x", &["instanceof C"]),
            ("t20", "x", &["instanceof Map<string, number>"]),
            ("t27", "x", &["number", "string"]),
            ("t28", "x", &["number", "string"]),
            ("t31", "x", &["null", "number"]),
            ("t48", "x", &["number"]),
            ("t59", "x", &["number"]),
            ("t79", "x", &["number"]),
            ("t91", "x", &["null"]),
            ("t92", "x", &["number"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives each read below the type in the
// comment after it. A write that reaches the read assigns a value that Biome
// may type differently from TypeScript, and the variable's declared type
// doesn't include that value, so the read is Unknown: a call to an imported
// function or a method whose return type Biome infers, or `await` of one, the
// lib's `JSON.parse`, `new Map()` without type arguments, `[]`, a function,
// `typeof`, a compound or compound-like assignment, a value that reads a
// variable whose type evolves with its writes, destructuring, and the head of
// a `for...of` loop. The declared type includes the initializer and the values
// of `=` and logical assignments in scopes other than the declaration's, so
// not a write in the same block, in a body of `if` or `while` without braces,
// or in `c && (x = v)`.
#[test]
fn evolving_variables_are_unknown_after_writes_that_biome_cant_type() {
    assert_writes(
        r#"
        function t05() { let x = null; x = loadInferred(); /*t05*/x; } // number
        function t06() { let x = null; x = store.load(); /*t06*/x; } // number
        function t07() { let x = null; x = literal.method(); /*t07*/x; } // number
        function t10() { let x = null; x = k.inferred(); /*t10*/x; } // number
        function t14() { let x = null; x = JSON.parse(s); /*t14*/x; } // any
        async function t17() { let x = null; x = await fetchInferred(); /*t17*/x; } // { id: number; }
        function t21() { let x = null; x = new Map(); /*t21*/x; } // Map<any, any>
        function t22() { let x = null; x = []; /*t22*/x; } // any[]
        function t23() { let x = null; x = () => 1; /*t23*/x; } // () => number
        function t24() { let x = null; x = 0; x += 1; /*t24*/x; } // number
        function t25() { let x = null; x = 0; x++; /*t25*/x; } // number
        function t26() { let x = null; x = 0; x = x + 1; /*t26*/x; } // number
        function t29() { let x; x = c ? store.load() : 1; /*t29*/x; } // number
        function t30() { let x = null; x = c ? store.load() : 1; /*t30*/x; } // number
        function t35() { let x = null; while (c) x = store.load(); /*t35*/x; } // number | null
        function t36() { let x = null; if (c) x = store.load(); /*t36*/x; } // number | null
        function t39() { let x = undefined; x = store.load(); /*t39*/x; } // number
        function t40() { let x; x = store.load(); /*t40*/x; } // number
        function t41() { var x = null; x = store.load(); /*t41*/x; } // number
        function t42() { let x = null; if (c) x = 1; else x = store.load(); /*t42*/x; } // number
        function t44() { let x = null; x = store.load(); if (x) { /*t44*/x; } else { /*t44f*/x; } } // number, number
        function t45() { let x = null; x = loadInferred(); if (x !== null) { /*t45*/x; } } // number
        let m1 = null; m1 = store.load(); /*t46*/m1; // number
        function t47() { let x = null; if (c) { x = "a"; } x = store.load(); /*t47*/x; } // number
        function t49() { let x; if (c) { x = "a"; } x = store.load(); /*t49*/x; } // number
        function t51() { let x = null; x = aliased.method(); /*t51*/x; } // number
        function t53() { let x = null; x ??= store.load(); /*t53*/x; } // number
        function t55() { let x = null; c && (x = store.load()); /*t55*/x; } // number | null
        function t60() { let x = null; if (c) { x = typeof s; } /*t60*/x; } // "bigint" | ... | "undefined" | null
        function t61() { let y = null; y = "a"; let x = null; if (c) { x = y; } /*t61*/x; } // string | null
        function t62() { let x = null; x = 0; if (c) { x += 1; } /*t62*/x; } // number
        function t63() { let x = null; x = 0; if (c) { x++; } /*t63*/x; } // number
        function t65() { let x = null; if (c) { ({ x } = { x: () => 1 }); } /*t65*/x; } // (() => number) | null
        function t66() { let x = null; if (c) { for (x of [() => 1]) {} } /*t66*/x; } // (() => number) | null
        function t67() { let x = null; if (c) { x = n + 1; } /*t67*/x; } // number | null
        function t76() { { let x = null; x = store.load(); /*t76*/x; } } // number
        function t85() { let x = null; if (c) { [x] = tuple; } /*t85*/x; } // number | null
        function t86() { let x = null; x = store.load(); if (c) { x = 1; } /*t86*/x; } // number
        function t88() { let x = null; while (c) x = store.load(), /*t88*/x; } // number
        function t90() { let x = null; x = c ? store.load() : null; if (x !== null) { /*t90*/x; } } // number
        function v2() { let x; x = 1; x++; /*v2*/x; } // number
        function v23() { let x; x = []; x.push(1); /*v23*/x; } // number[]
        function v24() { let x; x = () => 1; /*v24*/x; } // () => number
        function v42() { let x; x = 1; x = x + 1; /*v42*/x; } // number
        function v45() { let x; x = { m() { return 1; } }; /*v45*/x; } // { m(): number }
        "#,
        &[
            ("t05", "x", &["Unknown"]),
            ("t06", "x", &["Unknown"]),
            ("t07", "x", &["Unknown"]),
            ("t10", "x", &["Unknown"]),
            ("t14", "x", &["Unknown"]),
            ("t17", "x", &["Unknown"]),
            ("t21", "x", &["Unknown"]),
            ("t22", "x", &["Unknown"]),
            ("t23", "x", &["Unknown"]),
            ("t24", "x", &["Unknown"]),
            ("t25", "x", &["Unknown"]),
            ("t26", "x", &["Unknown"]),
            ("t29", "x", &["Unknown"]),
            ("t30", "x", &["Unknown"]),
            ("t35", "x", &["Unknown"]),
            ("t36", "x", &["Unknown"]),
            ("t39", "x", &["Unknown"]),
            ("t40", "x", &["Unknown"]),
            ("t41", "x", &["Unknown"]),
            ("t42", "x", &["Unknown"]),
            ("t44", "x", &["Unknown"]),
            ("t44f", "x", &["Unknown"]),
            ("t45", "x", &["Unknown"]),
            ("t46", "m1", &["Unknown"]),
            ("t47", "x", &["Unknown"]),
            ("t49", "x", &["Unknown"]),
            ("t51", "x", &["Unknown"]),
            ("t53", "x", &["Unknown"]),
            ("t55", "x", &["Unknown"]),
            ("t60", "x", &["Unknown"]),
            ("t61", "x", &["Unknown"]),
            ("t62", "x", &["Unknown"]),
            ("t63", "x", &["Unknown"]),
            ("t65", "x", &["Unknown"]),
            ("t66", "x", &["Unknown"]),
            ("t67", "x", &["Unknown"]),
            ("t76", "x", &["Unknown"]),
            ("t85", "x", &["Unknown"]),
            ("t86", "x", &["Unknown"]),
            ("t88", "x", &["Unknown"]),
            ("t90", "x", &["Unknown"]),
            ("v2", "x", &["Unknown"]),
            ("v23", "x", &["Unknown"]),
            ("v24", "x", &["Unknown"]),
            ("v42", "x", &["Unknown"]),
            ("v45", "x", &["Unknown"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// or a narrower one in the comment after it. The declared type of a variable
// whose type evolves with its writes includes the values of `=` and logical
// assignments in a scope other than the declaration's, as a block, a loop
// body, a `try` block, or a `switch`. So such a write gives the variable the
// type that Biome gives the value, with its literal types widened, even where
// Biome may type the value differently from TypeScript.
#[test]
fn evolving_variables_take_the_types_of_writes_in_other_scopes() {
    assert_writes(
        r#"
        function t32() { let x = null; for (const v of vs) { x = store.load(); } /*t32*/x; }
        function t33() { let x = null; for (const v of vs) x = store.load(); /*t33*/x; }
        function t34() { let x = null; while (c) { x = store.load(); } /*t34*/x; }
        function t37() { let x = null; if (c) { x = store.load(); } /*t37*/x; }
        function t38() { let x = null; try { x = store.load(); } catch {} /*t38*/x; }
        function t43() { let x = null; if (c) { x = 1; } else { x = store.load(); } /*t43*/x; }
        function t54() { let x = null; switch (n) { case 1: x = store.load(); break; } /*t54*/x; }
        function t56() { let x = null; do { x = store.load(); } while (c); /*t56*/x; }
        function t57() { let x = null; for (let i = 0; i < 3; i++) x = store.load(); /*t57*/x; }
        async function t58() { let x = null; try { x = await fetchInferred(); } catch { return; } /*t58*/x; }
        function t64() { let x = null; if (c) { x = computeInferred(); } /*t64*/x; }
        function t68() { let x = null; if (c) { x = store.load(); } if (x) { /*t68*/x; } }
        function t70() { let x; if (c) { x = store.load(); } /*t70*/x; }
        function t71() { let x = null; if (c) { x ??= store.load(); } /*t71*/x; }
        function t73() { let x = null; if (c) { x = loadInferred(); } /*t73*/x; }
        function t75() { let x = null; if (c) { if (c) { x = store.load(); } } /*t75*/x; }
        function t77() { var x = null; if (c) { x = store.load(); } /*t77*/x; }
        function t78() { if (c) { var x = null; } x = store.load(); /*t78*/x; }
        function t80() { let x = null; x = 1; if (c) { x = store.load(); } /*t80*/x; }
        function t81() { let x = null; if (c) { x = store.load(); } else { x = "s"; } /*t81*/x; }
        function t82() { let x = null; if (c) { x = []; } /*t82*/x; } // any[] | null
        function t83() { let x = null; if (c) { x = () => 1; } /*t83*/x; }
        function t84() { let x = null; if (c) { x = JSON.parse(s); } /*t84*/x; }
        function t87() { let x = null; while (c) { /*t87*/x; x = store.load(); } }
        function t89() { let x = null; if (c) { x = new Map(); } /*t89*/x; }
        function t94() { let x = null; if (c) { x = { k: store.load() }; } /*t94*/x; }
        function t95() { let x = null; label: { x = store.load(); } /*t95*/x; }
        function t93() { let x = undefined; if (c) { x = store.load(); } x = undefined; /*t93*/x; }
        "#,
        &[
            ("t32", "x", &["null", "number"]),
            ("t33", "x", &["null", "number"]),
            ("t34", "x", &["null", "number"]),
            ("t37", "x", &["null", "number"]),
            ("t38", "x", &["null", "number"]),
            ("t43", "x", &["number"]),
            ("t54", "x", &["null", "number"]),
            ("t56", "x", &["number"]),
            ("t57", "x", &["null", "number"]),
            (
                "t58",
                "x",
                &[r#"Object { prototype: No prototype members: ["id": number] }"#],
            ),
            ("t64", "x", &["null", "number"]),
            ("t68", "x", &["number"]),
            ("t70", "x", &["number", "undefined"]),
            ("t71", "x", &["null", "number"]),
            ("t73", "x", &["null", "number"]),
            ("t75", "x", &["null", "number"]),
            ("t77", "x", &["null", "number"]),
            ("t78", "x", &["number"]),
            ("t80", "x", &["number"]),
            ("t81", "x", &["number", "string"]),
            ("t82", "x", &["instanceof Array<unknown>", "null"]),
            (
                "t83",
                "x",
                &[
                    "null",
                    "sync Function { accepts: { params: [] type_args: [] } returns: number }",
                ],
            ),
            ("t84", "x", &["any"]),
            ("t87", "x", &["null", "number"]),
            ("t89", "x", &["instanceof Map<any, any>", "null"]),
            (
                "t94",
                "x",
                &[
                    r#"Object { prototype: No prototype members: ["k": number] }"#,
                    "null",
                ],
            ),
            ("t95", "x", &["number"]),
            ("t93", "x", &["undefined"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A variable with a type annotation or an initializer other than `null`,
// `undefined`, or `[]`, including `[]` in parentheses, doesn't evolve, and
// neither does an exported one, which TypeScript types as an implicit `any`
// (TS7005), nor one that nothing writes, which narrows as any other variable.
#[test]
fn other_declarations_dont_evolve() {
    assert_reads(
        r#"
        export let x1; x1 = 1; /*x1*/x1; // any
        function e18() { let x: any; x = 1; /*e18*/x; }
        function v9() { let x = 1 as number | undefined; x = undefined; /*v9*/x; }
        function v19() { let x = null as null; x = "a" as any; /*v19*/x; }
        function n1() { let x = null; if (x !== null) /*n1*/x; } // never
        function n2() { const a = []; if (a.length === 0) /*n2*/a; } // any[]
        "#,
        &[
            ("x1", "x1", &["Unknown"]),
            ("e18", "x", &["any"]),
            ("v9", "x", &["undefined"]),
            ("v19", "x", &["null"]),
            ("n1", "x", &["Unknown"]),
            ("n2", "a", &["instanceof Array<unknown>"]),
        ],
    );
}
