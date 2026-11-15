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

/// Names each member of `ty`, sorted: Biome's Unknown, which a read on a path
/// that no value takes has, as `Unknown`, and any other type by its
/// formatted text on one line, such as `string`, `null`, or
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
/// which follows [`DECLARATIONS`], by the names of its members, as
/// [`variant_names`] gives them, and that complete module inference gives
/// each read the type that a lookup gives it.
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

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m5/evolve.ts` L5, L6, L8, L9, L13 to L15, L18, L19, and
// `narrowing-probes/m5-l5/variables.ts`). A `let` or a `var` without a type
// annotation, declared without an initializer or with `null` or `undefined`,
// starts at `undefined` and takes the type of each value assigned to it, its
// literal types widened, in the function that declares it.
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

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m5/evolve.ts` L10 to L12, L16, L17, L20, L23, and
// `narrowing-probes/m5-l5/arrays.ts`). A variable initialized with an empty
// array literal starts with the element type `never` and joins the widened
// type of each value that `push`, `unshift`, or an assignment to an element
// with a number key adds to it.
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
// than Biome, in the comment after it (`narrowing-probes/m5/evolve.ts` L7,
// L21, and `narrowing-probes/m5-l5/declared.ts`). Biome keeps the declared
// type where a write that reaches the read assigns a value that Biome may
// type otherwise than TypeScript does, as a call without a return type
// annotation, a function, a compound assignment, or a value that reads such
// a variable, where nothing but `null`, or nothing but `undefined`, is
// assigned, in a closure or a nested function,
// where TypeScript reports an implicit `any` (TS7005), and where a function
// called where it is defined adds to an array.
#[test]
fn evolving_declarations_keep_their_declared_type_where_biome_cant_follow_them() {
    assert_reads(
        r#"
        function e3() { let x; /*e3*/x; } // undefined
        function e17() { let x; const g = () => /*e17*/x; x = 1; } // any
        function v2() { let x; x = 1; x++; /*v2*/x; } // number
        function v10() { let x; x = null; /*v10*/x; } // null
        function v16() { let x; function g() { x = 1; } g(); /*v16*/x; } // undefined
        function v23() { let x; x = []; x.push(1); /*v23*/x; } // number[]
        function v24() { let x; x = () => 1; /*v24*/x; } // () => number
        function v26() { let x; x = 1; const y = x; /*v26*/y; } // number
        function v42() { let x; x = 1; x = x + 1; /*v42*/x; } // number
        function v45() { let x; x = { m() { return 1; } }; /*v45*/x; } // { m(): number }
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
            ("v2", "x", &["Unknown"]),
            ("v10", "x", &["Unknown"]),
            ("v16", "x", &["Unknown"]),
            ("v23", "x", &["Unknown"]),
            ("v24", "x", &["Unknown"]),
            ("v26", "y", &["Unknown"]),
            ("v42", "x", &["Unknown"]),
            ("v45", "x", &["Unknown"]),
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

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m5/evolve.ts` L22 and `narrowing-probes/m5-l5/reads.ts`).
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
