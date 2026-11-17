use super::*;
use biome_module_graph::{
    ExpressionTypeInput, infer_expression_type,
    type_inference::{
        NormalizedExpressionTypeRequest, TypeInferenceCaller, execute_type_inference_request,
    },
};

/// How [`variant_names`] names `{}`, the type literal without members.
const EMPTY: &str = "Object { prototype: No prototype members: [] }";

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

/// Names each member of `ty`, sorted: an interface by its name, Biome's
/// unknown type as `Unknown`, and any other type by its formatted text on one
/// line, such as `string`, `null`, or `instanceof Error`. A read on a path
/// that no value takes has the unknown type.
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

/// Asserts the reads in `source` as [`assert_reads_with`] does, without other
/// modules.
fn assert_reads(source: &str, reads: &[(&str, &str, &[&str])]) {
    assert_reads_with(source, &[], reads);
}

/// Asserts the type of each `(marker, expression, expected)` read in `source`,
/// the module at `/src/index.ts`, by the names of its members, from
/// [`variant_names`]. `modules` are the other modules, as `(path, source)`
/// pairs. Also asserts that complete module inference gives each read the
/// type that a lookup gives it.
fn assert_reads_with(source: &str, modules: &[(&str, &str)], reads: &[(&str, &str, &[&str])]) {
    let build = || {
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
    };
    let lookups: Vec<_> = {
        let (db, module) = build();
        reads
            .iter()
            .map(|(marker, expression, _)| {
                let range = marked_range(source, marker, expression);
                infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range))
                    .map(|ty| format_inferred_type(&db, ty))
            })
            .collect()
    };
    let (db, module) = build();
    let complete = infer_module_types(&db, module).unwrap();
    let mismatches: Vec<_> = reads
        .iter()
        .zip(lookups)
        .flat_map(|((marker, expression, expected), lookup)| {
            let range = marked_range(source, marker, expression);
            let ty = execute_type_inference_request(
                &db,
                TypeInferenceCaller::new("test", "unknownAndAny"),
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
// Truthiness, nullishness, `typeof`, and strict equality with `null` or
// `undefined` narrow `unknown` as `{} | null | undefined`, and a result that
// keeps all three is `unknown` again. `typeof x === "object"` gives
// `object | null`, and `"function"` gives `Function`. At `L24_83`, TypeScript
// gives `object | null`, which has the same values as Biome's type, because
// every function is an object.
#[test]
fn unknown_narrows_as_an_empty_object_null_or_undefined() {
    assert_reads(
        r#"
        export {};
        function u1(x: unknown) { if (x) /*L7_36*/x; else /*L7_58*/x; } // L7
        function u2(x: unknown) { if (x != null) /*L8_44*/x; else /*L8_66*/x; } // L8
        function u3(x: unknown) { if (x !== null) /*L9_45*/x; else /*L9_67*/x; } // L9
        function u4(x: unknown) { if (x !== undefined) /*L10_50*/x; else /*L10_72*/x; } // L10
        function u5(x: unknown) { if (x == undefined) /*L11_49*/x; else /*L11_71*/x; } // L11
        function u6(x: unknown) { if (!x) return; /*L12_45*/x; } // L12
        function u7(x: unknown) { x && /*L13_34*/x; } // L13
        function u8(x: unknown) { if (x !== null && x !== undefined) /*L14_64*/x; } // L14
        function t1(x: unknown) { if (typeof x === "object") /*L16_56*/x; else /*L16_78*/x; } // L16
        function t2(x: unknown) { if (typeof x === "function") /*L17_58*/x; else /*L17_80*/x; } // L17
        function t3(x: unknown) { if (typeof x === "object" && x !== null) /*L18_70*/x; } // L18
        function t4(x: unknown) { if (typeof x === "object" && x) /*L19_61*/x; } // L19
        function t5(x: unknown) { if (x && typeof x === "object") /*L20_61*/x; } // L20
        function t6(x: unknown) { if (typeof x !== "object") /*L21_56*/x; } // L21
        function t7(x: unknown) { if (typeof x === "string") /*L22_56*/x; else /*L22_78*/x; } // L22
        function t8(x: unknown) { if (x != null && typeof x === "object") /*L23_69*/x; } // L23
        function t9(x: unknown) { if (typeof x === "object" || typeof x === "function") /*L24_83*/x; } // L24
        function t10(x: unknown) { if (x) { if (typeof x === "object") /*L25_66*/x; } } // L25
        function t11(x: unknown) { if (x === undefined) /*L14_50*/x; else /*L14_72*/x; }
        function t12(x: unknown) { if (typeof x === "undefined") /*L19_59*/x; else /*L19_81*/x; }
        function t13(x: unknown) { if (typeof x === "number") {} else if (x) /*L20_72*/x; }
        function t14(x: unknown) { if (typeof x === "object") { if (x) /*L33_66*/x; else /*L33_88*/x; } }
        function t15(x: unknown) { if (x == null) /*L35_45*/x; else /*L35_67*/x; }
        "#,
        &[
            ("L7_36", "x", &[EMPTY]),
            ("L7_58", "x", &["unknown"]),
            ("L8_44", "x", &[EMPTY]),
            ("L8_66", "x", &["null", "undefined"]),
            ("L9_45", "x", &[EMPTY, "undefined"]),
            ("L9_67", "x", &["null"]),
            ("L10_50", "x", &[EMPTY, "null"]),
            ("L10_72", "x", &["undefined"]),
            ("L11_49", "x", &["null", "undefined"]),
            ("L11_71", "x", &[EMPTY]),
            ("L12_45", "x", &[EMPTY]),
            ("L13_34", "x", &[EMPTY]),
            ("L14_64", "x", &[EMPTY]),
            ("L16_56", "x", &["null", "object"]),
            ("L16_78", "x", &[EMPTY, "undefined"]),
            ("L17_58", "x", &["instanceof Function"]),
            ("L17_80", "x", &["unknown"]),
            ("L18_70", "x", &["object"]),
            ("L19_61", "x", &["object"]),
            ("L20_61", "x", &["object"]),
            ("L21_56", "x", &[EMPTY, "undefined"]),
            ("L22_56", "x", &["string"]),
            ("L22_78", "x", &["unknown"]),
            ("L23_69", "x", &["object"]),
            ("L24_83", "x", &["instanceof Function", "null", "object"]),
            ("L25_66", "x", &["object"]),
            ("L14_50", "x", &["undefined"]),
            ("L14_72", "x", &[EMPTY, "null"]),
            ("L19_59", "x", &["undefined"]),
            ("L19_81", "x", &[EMPTY, "null"]),
            ("L20_72", "x", &[EMPTY]),
            ("L33_66", "x", &["object"]),
            ("L33_88", "x", &["null"]),
            ("L35_45", "x", &["null", "undefined"]),
            ("L35_67", "x", &[EMPTY]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// except where a comment gives TypeScript's type. Strict equality with a
// literal gives the literal and keeps `unknown` in the `else` branch, and
// loose equality with a literal keeps `unknown` in both branches. A
// comparison with a value that is not a unit value narrows nothing yet.
#[test]
fn unknown_compares_with_literals_and_keeps_other_comparisons() {
    assert_reads(
        r#"
        export {};
        declare const o: { a: number };
        declare const s: string;
        declare const n: number | null;
        function q1(x: unknown) { if (x === "a") /*L27_44*/x; else /*L27_66*/x; } // L27
        function q2(x: unknown) { if (x == "a") /*L28_43*/x; else /*L28_65*/x; } // L28
        function q3(x: unknown) { if (x === o) /*L29_42*/x; else /*L29_64*/x; } // L29
        function q4(x: unknown) { if (x === s) /*L30_42*/x; } // L30
        function q5(x: unknown) { if (x === n) /*L31_42*/x; } // L31
        function q6(x: unknown) { if (x == o) /*L32_41*/x; } // L32
        function q7(x: unknown, y: unknown) { if (x === y) /*L33_54*/x; } // L33
        "#,
        &[
            ("L27_44", "x", &["string: a"]),
            ("L27_66", "x", &["unknown"]),
            ("L28_43", "x", &["unknown"]),
            ("L28_65", "x", &["unknown"]),
            // `object`
            ("L29_42", "x", &["unknown"]),
            ("L29_64", "x", &["unknown"]),
            // `string`
            ("L30_42", "x", &["unknown"]),
            ("L31_42", "x", &["unknown"]),
            ("L32_41", "x", &["unknown"]),
            ("L33_54", "x", &["unknown"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// except where a comment gives TypeScript's type. `instanceof` gives an
// instance of a generic class `any` type arguments, as `Map<any, any>`, and a
// type-guard function with a `null` or `undefined` target leaves the other
// two of `{} | null | undefined` in its `else` branch.
#[test]
fn unknown_narrows_through_properties_instanceof_and_type_predicates() {
    assert_reads(
        r#"
        export {};
        declare function isNull(v: unknown): v is null;
        declare function isObj(v: unknown): v is {};
        declare function assert(v: unknown): asserts v;
        class Box<T = string> { v!: T }
        function v1(x: { v: unknown }) { if (x.v) /*L35_45*/x.v; } // L35
        function v2(x: unknown) { if (x instanceof Error) /*L36_53*/x; else /*L36_75*/x; } // L36
        function v3(x: unknown) { if (x instanceof Map) /*L37_51*/x; } // L37
        function v4(x: unknown) { if (Array.isArray(x)) /*L38_51*/x; else /*L38_73*/x; } // L38
        function v5(x: unknown) { if (x) { if (Array.isArray(x)) /*L39_60*/x; } } // L39
        function v8(x: unknown) { if (x && typeof x === "object" && "a" in x) /*L42_73*/x.a; } // L42
        function w1(x: unknown) { if (x instanceof Box) /*L17_51*/x; }
        function w2(x: unknown) { if (isNull(x)) /*L21_45*/x; else /*L21_67*/x; }
        function w3(x: unknown) { if (isObj(x)) /*L22_44*/x; else /*L22_66*/x; }
        function w4(x: unknown) { assert(x); /*L8_54*/x; }
        "#,
        &[
            ("L35_45", "x.v", &[EMPTY]),
            ("L36_53", "x", &["instanceof Error"]),
            ("L36_75", "x", &["unknown"]),
            ("L37_51", "x", &["instanceof Map<any, any>"]),
            ("L38_51", "x", &["instanceof Array<any>"]),
            ("L38_73", "x", &["unknown"]),
            ("L39_60", "x", &["instanceof Array<any>"]),
            // `unknown`, the type of the property of
            // `object & Record<"a", unknown>`
            ("L42_73", "x.a", &["Unknown"]),
            ("L17_51", "x", &["instanceof Box<any>"]),
            ("L21_45", "x", &["null"]),
            ("L21_67", "x", &[EMPTY, "undefined"]),
            ("L22_44", "x", &[EMPTY]),
            // `null | undefined`
            ("L22_66", "x", &["unknown"]),
            ("L8_54", "x", &[EMPTY]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// `typeof x === "object"` replaces `{}`, written or inferred from `{}`, with
// `object`, but keeps an empty interface or class as it is.
#[test]
fn typeof_object_turns_an_empty_object_type_into_object() {
    assert_reads(
        r#"
        export {};
        type E = {};
        interface I {}
        class C {}
        function g1(x: E | string) { if (typeof x === "object") /*L6_59*/x; } // L6
        function g2() { const o = {}; if (typeof o === "object") /*L7_60*/o; else /*L7_82*/o; } // L7
        function g3(x: I) { if (typeof x === "object") /*L8_50*/x; } // L8
        function g4(x: C | number) { if (typeof x === "object") /*L9_59*/x; } // L9
        function g5(x: {} | undefined) { if (typeof x === "object") /*L10_63*/x; else /*L10_85*/x; } // L10
        function g6(x: object | {}) { if (typeof x === "object") /*L11_60*/x; } // L11
        function v6(x: {}) { if (typeof x === "object") /*L40_51*/x; else /*L40_73*/x; } // L40
        function v7(x: {} | null) { if (x) /*L41_38*/x; } // L41
        "#,
        &[
            ("L6_59", "x", &["object"]),
            ("L7_60", "o", &["object"]),
            ("L7_82", "o", &[EMPTY]),
            ("L8_50", "x", &["I"]),
            ("L9_59", "x", &["instanceof C"]),
            ("L10_63", "x", &["object"]),
            ("L10_85", "x", &[EMPTY, "undefined"]),
            ("L11_60", "x", &["object"]),
            ("L40_51", "x", &["object"]),
            ("L40_73", "x", &[EMPTY]),
            ("L41_38", "x", &[EMPTY]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// except where a comment gives TypeScript's type. Where no case of a `switch`
// on a value or on `typeof` of one matches, TypeScript removes the matched
// values from `unknown` as a whole, so `unknown` stays `unknown` there, while
// `switch (true)` tests each case as an `if` does. At `L10_73`, TypeScript
// gives `unknown`, which has the same values.
#[test]
fn switch_defaults_keep_unknown() {
    assert_reads(
        r#"
        export {};
        function s1(x: unknown) { switch (x) { case null: /*L3_53*/x; break; default: /*L3_86*/x; } } // L3
        function s2(x: unknown) { switch (x) { case null: case undefined: break; default: /*L4_85*/x; } } // L4
        function s3(x: unknown) { switch (x) { case null: return; } /*L5_63*/x; } // L5
        function s4(x: unknown) { switch (typeof x) { case "object": return; } /*L6_74*/x; } // L6
        function s5(x: unknown) { switch (true) { case x === null: break; default: /*L7_78*/x; } } // L7
        function s6(x: unknown) { switch (typeof x) { case "undefined": case "object": break; default: /*L8_98*/x; } } // L8
        function s7(x: unknown) { switch (x) { case "a": break; default: /*L9_68*/x; } } // L9
        function s8(x: unknown) { switch (typeof x) { case "string": default: /*L10_73*/x; } } // L10
        function s9(x: unknown) { if (!x) return; switch (typeof x) { case "object": break; default: /*L11_96*/x; } } // L11
        function s10(x: unknown) { switch (typeof x) { case "object": /*L12_65*/x; case "string": /*L12_97*/x; } } // L12
        function s11(o: { v: unknown }) { switch (o.v) { case null: return; } /*L13_73*/o.v; } // L13
        "#,
        &[
            ("L3_53", "x", &["null"]),
            ("L3_86", "x", &["unknown"]),
            ("L4_85", "x", &["unknown"]),
            ("L5_63", "x", &["unknown"]),
            ("L6_74", "x", &["unknown"]),
            ("L7_78", "x", &[EMPTY, "undefined"]),
            ("L8_98", "x", &["unknown"]),
            ("L9_68", "x", &["unknown"]),
            ("L10_73", "x", &["string", "unknown"]),
            ("L11_96", "x", &[EMPTY]),
            ("L12_65", "x", &["null", "object"]),
            ("L12_97", "x", &["null", "object", "string"]),
            ("L13_73", "o.v", &["unknown"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// `typeof` narrows `any` only to a primitive kind or `undefined`, and only in
// the true branch: `"object"` and `"function"` keep `any`, and so does every
// `else` branch.
#[test]
fn any_narrows_only_by_typeof_with_a_primitive_kind() {
    assert_reads(
        r#"
        export {};
        function a1(x: any) { if (typeof x === "string") /*L10_52*/x; else /*L10_74*/x; } // L10
        function a2(x: any) { if (typeof x === "object") /*L11_52*/x; else /*L11_74*/x; } // L11
        function a3(x: any) { if (typeof x === "function") /*L12_54*/x; else /*L12_76*/x; } // L12
        function a4(x: any) { if (typeof x === "undefined") /*L13_55*/x; else /*L13_77*/x; } // L13
        function a5(x: any) { if (typeof x !== "number") return; /*L14_60*/x; } // L14
        function e3(o: { v: any }) { if (typeof o.v === "string") /*L38_61*/o.v; } // L38
        function e4(x: any | string) { if (typeof x === "string") /*L39_61*/x; } // L39
        function e5(x: any) { switch (typeof x) { case "string": /*L40_60*/x; break; case "object": /*L40_99*/x; } } // L40
        function e6(x: any) { if (typeof x === "string" || typeof x === "number") /*L41_77*/x; } // L41
        function f14(x: any) { if (typeof x === "string" && x.length) /*L24_65*/x; }
        function f20(x: any) { if (x !== undefined && typeof x === "number") /*L30_72*/x; }
        function f28(x: any) { if (typeof x === "boolean") /*L38_54*/x; else if (typeof x === "symbol") /*L38_103*/x; else if (typeof x === "bigint") /*L38_152*/x; }
        function f32(x: any) { if (typeof x === "string") { if (x === "a") /*after_typeof*/x; } }
        "#,
        &[
            ("L10_52", "x", &["string"]),
            ("L10_74", "x", &["any"]),
            ("L11_52", "x", &["any"]),
            ("L11_74", "x", &["any"]),
            ("L12_54", "x", &["any"]),
            ("L12_76", "x", &["any"]),
            ("L13_55", "x", &["undefined"]),
            ("L13_77", "x", &["any"]),
            ("L14_60", "x", &["number"]),
            ("L38_61", "o.v", &["string"]),
            ("L39_61", "x", &["string"]),
            ("L40_60", "x", &["string"]),
            ("L40_99", "x", &["any"]),
            ("L41_77", "x", &["number", "string"]),
            ("L24_65", "x", &["string"]),
            ("L30_72", "x", &["number"]),
            ("L38_54", "x", &["boolean"]),
            ("L38_103", "x", &["symbol"]),
            ("L38_152", "x", &["BigInt"]),
            // Once `typeof` narrows `any`, other tests narrow the result.
            ("after_typeof", "x", &["string: a"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// except where a comment gives TypeScript's type. Truthiness, equality, `in`,
// discriminants, and the properties of `any` keep `any`.
#[test]
fn any_keeps_its_type_for_other_tests() {
    assert_reads(
        r#"
        export {};
        declare function assert(v: unknown): asserts v;
        function b1(x: any) { if (x) /*L16_32*/x; else /*L16_54*/x; } // L16
        function b2(x: any) { if (x === "a") /*L17_40*/x; else /*L17_62*/x; } // L17
        function b3(x: any) { if (x === null) /*L18_41*/x; else /*L18_63*/x; } // L18
        function b4(x: any) { if (x != null) /*L19_40*/x; else /*L19_62*/x; } // L19
        function b5(x: any) { if (x === undefined) /*L20_46*/x; } // L20
        function e1(x: any) { if ("a" in x) /*L36_39*/x; } // L36
        function e2(x: any) { if (x.kind === "a") /*L37_45*/x; } // L37
        function f1(x: any) { if (typeof x.v === "string") /*L11_54*/x.v; }
        function f6(x: any) { assert(x); /*L16_36*/x; }
        function f17(x: any) { if (x == null) /*L27_41*/x; }
        function f19(o: { v: any }) { if (o.v) /*L29_42*/o.v; }
        "#,
        &[
            ("L16_32", "x", &["any"]),
            ("L16_54", "x", &["any"]),
            ("L17_40", "x", &["any"]),
            ("L17_62", "x", &["any"]),
            ("L18_41", "x", &["any"]),
            ("L18_63", "x", &["any"]),
            ("L19_40", "x", &["any"]),
            ("L19_62", "x", &["any"]),
            ("L20_46", "x", &["any"]),
            ("L36_39", "x", &["any"]),
            ("L37_45", "x", &["any"]),
            // `any`: Biome doesn't type a property of `any`.
            ("L11_54", "x.v", &["Unknown"]),
            ("L16_36", "x", &["any"]),
            ("L27_41", "x", &["any"]),
            ("L29_42", "o.v", &["any"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// `instanceof` narrows `any` to the instance type in the true branch, with
// `any` type arguments for a generic class, except for `Object` and
// `Function`, and keeps `any` in the `else` branch.
#[test]
fn any_narrows_by_instanceof_in_the_true_branch() {
    assert_reads(
        r#"
        export {};
        class A { a = 1 }
        class Box<T = string> { v!: T }
        function c1(x: any) { if (x instanceof A) /*L22_45*/x; else /*L22_67*/x; } // L22
        function c2(x: any) { if (x instanceof Date) /*L23_48*/x; } // L23
        function c3(x: any) { if (x instanceof Object) /*L24_50*/x; } // L24
        function c4(x: any) { if (x instanceof Function) /*L25_52*/x; } // L25
        function c5(x: any) { if (x instanceof Map) /*L26_47*/x; } // L26
        function c6(x: any) { if (!(x instanceof A)) /*L27_48*/x; } // L27
        function f8(x: any) { if (x instanceof Error) /*L18_49*/x; else /*L18_71*/x; }
        function f24(x: any) { if (x instanceof A || typeof x === "string") /*L34_71*/x; }
        function f33(x: any) { if (x instanceof Box) /*box*/x; }
        "#,
        &[
            ("L22_45", "x", &["instanceof A"]),
            ("L22_67", "x", &["any"]),
            ("L23_48", "x", &["instanceof Date"]),
            ("L24_50", "x", &["any"]),
            ("L25_52", "x", &["any"]),
            ("L26_47", "x", &["instanceof Map<any, any>"]),
            ("L27_48", "x", &["any"]),
            ("L18_49", "x", &["instanceof Error"]),
            ("L18_71", "x", &["any"]),
            ("L34_71", "x", &["instanceof A", "string"]),
            ("box", "x", &["instanceof Box<any>"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// except where a comment gives TypeScript's type. A type-guard function and
// an assertion function give `any` their target, also through a call of the
// library, as `Array.isArray`, and the `else` branch keeps `any`.
#[test]
fn any_narrows_by_type_predicates_and_assertions() {
    assert_reads(
        r#"
        export {};
        declare function isFoo(v: unknown): v is Foo;
        declare function isString(v: unknown): v is string;
        declare function assertFoo(v: unknown): asserts v is Foo;
        declare function isAny(v: unknown): v is any;
        interface Foo { a: number }
        function d1(x: any) { if (isFoo(x)) /*L29_39*/x; else /*L29_61*/x; } // L29
        function d2(x: any) { if (isString(x)) /*L30_42*/x; else /*L30_64*/x; } // L30
        function d3(x: any) { assertFoo(x); /*L31_39*/x; } // L31
        function d4(x: any) { if (Array.isArray(x)) /*L32_47*/x; else /*L32_69*/x; } // L32
        function d5(x: any) { if (isAny(x)) /*L33_39*/x; } // L33
        function d6(x: any) { if (isFoo(x) && x.a) /*L34_46*/x; } // L34
        function f2(x: any) { if (!isAny(x)) /*L12_40*/x; }
        "#,
        &[
            ("L29_39", "x", &["Foo"]),
            ("L29_61", "x", &["any"]),
            ("L30_42", "x", &["string"]),
            ("L30_64", "x", &["any"]),
            ("L31_39", "x", &["Foo"]),
            ("L32_47", "x", &["instanceof Array<any>"]),
            ("L32_69", "x", &["any"]),
            ("L33_39", "x", &["any"]),
            ("L34_46", "x", &["Foo"]),
            // `never`
            ("L12_40", "x", &["any"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// except where a comment gives TypeScript's type. A variable whose type is
// `any` through its initializer narrows as one whose annotation is `any`.
#[test]
fn any_narrows_through_inferred_types() {
    assert_reads(
        r#"
        export {};
        declare const s: string;
        function e7(x: any) { let y: string | number = x; if (typeof y === "string") /*L42_80*/y; } // L42
        function e8(x: any) { const y = x; if (typeof y === "string") /*L43_65*/y; } // L43
        function e9(x: any) { if (typeof x === "string") { const y = x; /*L44_67*/y; } } // L44
        function f13() { const y = JSON.parse(s); if (typeof y === "string") /*L23_72*/y; else /*L23_94*/y; }
        "#,
        &[
            ("L42_80", "y", &["string"]),
            ("L43_65", "y", &["string"]),
            // `string`: Biome types `y` by the declared type of `x`, as for any
            // variable whose initializer reads a narrowed variable.
            ("L44_67", "y", &["any"]),
            ("L23_72", "y", &["string"]),
            ("L23_94", "y", &["any"]),
        ],
    );
}

/// The module at `/src/loose.ts`, which exports an alias of `any`, a variable
/// of type `any`, and a function that returns `any`.
const LOOSE: &str = r#"
export type Loose = any;
export declare const loose: any;
export declare function load(): any;
"#;

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A variable whose type is `any` through an alias, an import, or its
// initializer narrows by `instanceof` and by a type-guard function whose
// target is an object type as one whose annotation is `any`, also when no
// other test mentions it: the true branch gets the instance type or the
// target, and the `else` branch keeps `any`.
#[test]
fn any_through_aliases_imports_and_inference_narrows_by_instance_tests() {
    assert_reads_with(
        r#"
        import type { Loose as ImportedLoose } from "./loose";
        import { loose, load } from "./loose";
        declare const s: string;
        type Loose = any;
        class A { a = 1 }
        interface Foo { a: number }
        declare function isFoo(v: unknown): v is Foo;
        function a1(x: Loose) { if (x instanceof A) /*L9_47*/x; else /*L9_69*/x; } // L9
        function a2(x: Loose) { if (isFoo(x)) /*L10_41*/x; else /*L10_63*/x; } // L10
        function i1(x: ImportedLoose) { if (x instanceof A) /*L11_55*/x; else /*L11_77*/x; } // L11
        function i2(x: ImportedLoose) { if (isFoo(x)) /*L12_49*/x; else /*L12_71*/x; } // L12
        function i3() { const y = loose; if (y instanceof A) /*L13_56*/y; else /*L13_78*/y; } // L13
        function i4() { const y = load(); if (isFoo(y)) /*L14_51*/y; else /*L14_73*/y; } // L14
        function n1() { const d = JSON.parse(s); if (d instanceof A) /*L15_64*/d; else /*L15_86*/d; } // L15
        function n2() { const d = JSON.parse(s); if (isFoo(d)) /*L16_58*/d; else /*L16_80*/d; } // L16
        "#,
        &[("/src/loose.ts", LOOSE)],
        &[
            ("L9_47", "x", &["instanceof A"]),
            ("L9_69", "x", &["any"]),
            ("L10_41", "x", &["Foo"]),
            ("L10_63", "x", &["any"]),
            ("L11_55", "x", &["instanceof A"]),
            ("L11_77", "x", &["any"]),
            ("L12_49", "x", &["Foo"]),
            ("L12_71", "x", &["any"]),
            ("L13_56", "y", &["instanceof A"]),
            ("L13_78", "y", &["any"]),
            ("L14_51", "y", &["Foo"]),
            ("L14_73", "y", &["any"]),
            ("L15_64", "d", &["instanceof A"]),
            ("L15_86", "d", &["any"]),
            ("L16_58", "d", &["Foo"]),
            ("L16_80", "d", &["any"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// except where a comment gives TypeScript's type. A variable or a property
// whose type is `any`, and that only `instanceof`, type-guard functions, and
// tests of its shape mention, narrows as a variable whose annotation is `any`:
// `instanceof Object` and the tests of its shape keep `any`, a generic class
// gets `any` type arguments, and a closure sees the narrowed type. Biome reads
// a property of `any`, also through a destructuring pattern, as unknown, which
// no test narrows.
#[test]
fn instance_tested_any_narrows_as_annotated_any() {
    assert_reads(
        r#"
        export {};
        declare const s: string;
        type Loose = any;
        class A { a = 1 }
        interface Foo { a: number }
        declare function isFoo(v: unknown): v is Foo;
        function n3() { const d = JSON.parse(s); if (d instanceof Map) /*L17_66*/d; else /*L17_88*/d; } // L17
        function n4() { const d = JSON.parse(s); if (d instanceof Object) /*L18_69*/d; } // L18
        function n5() { const d = JSON.parse(s); if (!(d instanceof A)) /*L19_67*/d; else /*L19_89*/d; } // L19
        function n6() { const d = JSON.parse(s); if (d instanceof A || d.kind === "a") /*L20_82*/d; } // L20
        function n7() { const d = JSON.parse(s); if (d.kind === "a") /*L21_64*/d; } // L21
        function n8() { const d = JSON.parse(s); if (d instanceof A) {} /*L22_67*/d; } // L22
        function n9() { const d = JSON.parse(s); if ("a" in d && d instanceof A) /*L23_76*/d; } // L23
        function o1(o: { data: any }) { if (o.data instanceof A) /*L24_60*/o.data; else /*L24_87*/o.data; } // L24
        function o2(o: { data: Loose }) { if (isFoo(o.data)) /*L25_56*/o.data; else /*L25_83*/o.data; } // L25
        function c1() { const d = JSON.parse(s); if (d instanceof A) { const g = () => /*L26_82*/d; } } // L26
        function c2(o: { a: any }) { const { a } = o; if (a instanceof A) /*L27_69*/a; else /*L27_91*/a; } // L27
        function c3() { const { a } = JSON.parse(s); if (a instanceof A) /*L28_68*/a; } // L28
        function c4(xs: any[]) { for (const d of xs) { if (isFoo(d)) /*L29_64*/d; else /*L29_86*/d; } } // L29
        function c5(x: any) { if (x.v instanceof A) /*L30_47*/x.v; } // L30
        function c6(x: any) { const { a } = x; if (a instanceof A) /*L31_62*/a; } // L31
        "#,
        &[
            ("L17_66", "d", &["instanceof Map<any, any>"]),
            ("L17_88", "d", &["any"]),
            ("L18_69", "d", &["any"]),
            ("L19_67", "d", &["any"]),
            ("L19_89", "d", &["instanceof A"]),
            ("L20_82", "d", &["any"]),
            ("L21_64", "d", &["any"]),
            ("L22_67", "d", &["any"]),
            ("L23_76", "d", &["instanceof A"]),
            ("L24_60", "o.data", &["instanceof A"]),
            ("L24_87", "o.data", &["any"]),
            ("L25_56", "o.data", &["Foo"]),
            ("L25_83", "o.data", &["any"]),
            ("L26_82", "d", &["instanceof A"]),
            ("L27_69", "a", &["instanceof A"]),
            ("L27_91", "a", &["any"]),
            // `A`
            ("L28_68", "a", &["Unknown"]),
            ("L29_64", "d", &["Foo"]),
            ("L29_86", "d", &["any"]),
            // `any`
            ("L30_47", "x.v", &["Unknown"]),
            // `A`
            ("L31_62", "a", &["Unknown"]),
        ],
    );
}
