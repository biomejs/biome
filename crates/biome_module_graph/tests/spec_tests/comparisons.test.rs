use super::*;
use biome_module_graph::type_inference::{
    NormalizedExpressionTypeRequest, TypeInferenceCaller, execute_type_inference_request,
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

/// Names each member of `ty`, sorted: an object type literal by its members,
/// Biome's Unknown as `Unknown`, and any other type by its formatted text on
/// one line. A read on a path that no value takes has the type Unknown.
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
            if lines.first().is_some_and(|line| line.starts_with("Object"))
                && let Some(start) = lines.iter().position(|line| line.starts_with("members: "))
            {
                // Skip `members: ` and the closing brace of the object.
                let members = lines[start..lines.len() - 1].join(" ");
                return members["members: ".len()..].to_owned();
            }
            lines.join(" ")
        })
        .collect();
    names.sort();
    names
}

/// Types and values shared by the tests.
const PRELUDE: &str = r#"
    type ABC = "a" | "b" | "c";
    const K = "a";
    const N = 1;
    const M = -1;
    const B = true;
    const Z = null;
    const U = undefined;
    const T = `a`;
    const K2 = K;
"#;

/// The module that `/src/index.ts` imports in [`assert_reads`].
const CONSTANTS: &str = r#"
    export const EK = "a";
    export const EN = -1;
    export const EV = void 0;
    export const ET = `b`;
    export const EC = EK;
    export const EP = -(1);
    export const EB = 1 === 1;
    export let EL = "a";
    export const EA: "a" = "a";
    export const EI = EK2;
    import { EK as EK2 } from "./more.ts";
    export { EK as EX } from "./more.ts";
    export default "c";
    export enum EE { A, B }
    export enum ES { A = "a", B = "b" }
    export { EM } from "./more.ts";
    import { EM as EM2 } from "./more.ts";
    export const EMC = EM2.C;
"#;

/// The module that [`CONSTANTS`] imports.
const MORE: &str = r#"
    export const EK = "c";
    export enum EM { C = "c" }
"#;

/// Asserts the type of each `(marker, expression, expected)` read in `source`
/// by the names of its members, as [`variant_names`] gives them, and asserts
/// that a lookup and complete module inference give each read the same type.
/// `source` follows [`PRELUDE`] in `/src/index.ts` and can import
/// [`CONSTANTS`] from `./constants.ts`.
fn assert_reads(source: &str, reads: &[(&str, &str, &[&str])]) {
    let source = format!("{PRELUDE}{source}");
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), source.as_str());
    fs.insert("/src/constants.ts".into(), CONSTANTS);
    fs.insert("/src/more.ts".into(), MORE);
    let db = build_js_test_module_db(
        &fs,
        &["/src/index.ts", "/src/constants.ts", "/src/more.ts"],
        true,
    );
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    let mut lookups = Vec::new();
    let mismatches: Vec<_> = reads
        .iter()
        .filter_map(|(marker, expression, expected)| {
            let range = marked_range(&source, marker, expression);
            let ty = execute_type_inference_request(
                &db,
                TypeInferenceCaller::new("test", "comparisons"),
                NormalizedExpressionTypeRequest::new(module, range),
            )
            .unwrap_or_else(|| panic!("expression at {marker} must be inferred"));
            lookups.push((marker, range, infer_expression_type_at(&db, module, range)));
            let actual = variant_names(&db, ty);
            (actual != *expected)
                .then(|| format!("{marker}: expected {expected:?}, got {actual:?}"))
        })
        .collect();
    assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
    let complete = infer_module_types(&db, module).unwrap();
    for (marker, range, ty) in lookups {
        assert_eq!(complete.expressions.get(&range).copied(), ty, "{marker}");
    }
}

fn infer_expression_type_at<'db>(
    db: &'db TestModuleDb,
    module: ModuleInfo,
    range: TextRange,
) -> Option<InferredTypeData<'db>> {
    biome_module_graph::infer_expression_type(
        db,
        biome_module_graph::ExpressionTypeInput::new(db, module, range),
    )
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A comparison with a `const` whose initializer is a literal narrows as a
// comparison with the literal does, also through another such `const`, in
// either order, and in parentheses.
#[test]
fn comparisons_with_constants_narrow_both_branches() {
    assert_reads(
        r#"
        function e1(x: ABC) { if (x === K) /*e1t*/x; else /*e1f*/x; }
        function e2(x: ABC) { if (x !== K) /*e2t*/x; else /*e2f*/x; }
        function e3(x: 1 | 2) { if (x === N) /*e3t*/x; else /*e3f*/x; }
        function e6(x: -1 | 1) { if (x === M) /*e6t*/x; else /*e6f*/x; }
        function e7(x: boolean) { if (x === B) /*e7t*/x; else /*e7f*/x; }
        function e8(x: string | null) { if (x === Z) /*e8t*/x; else /*e8f*/x; }
        function e9(x: string | undefined) { if (x === U) /*e9t*/x; else /*e9f*/x; }
        function e13(x: ABC) { if (x === T) /*e13t*/x; else /*e13f*/x; }
        function e14(x: ABC) { if (x === K2) /*e14t*/x; else /*e14f*/x; }
        function e26(x: string) { if (x === K) /*e26t*/x; else /*e26f*/x; }
        function e27(x: unknown) { if (x === K) /*e27t*/x; else /*e27f*/x; }
        function e28(x: ABC) { if (K === x) /*e28t*/x; else /*e28f*/x; }
        function e33(x: ABC) { if (x === (K)) /*e33t*/x; else /*e33f*/x; }
        "#,
        &[
            ("e1t", "x", &["string: a"]),
            ("e1f", "x", &["string: b", "string: c"]),
            ("e2t", "x", &["string: b", "string: c"]),
            ("e2f", "x", &["string: a"]),
            ("e3t", "x", &["number: 1"]),
            ("e3f", "x", &["number: 2"]),
            ("e6t", "x", &["number: -1"]),
            ("e6f", "x", &["number: 1"]),
            ("e7t", "x", &["bool: true"]),
            ("e7f", "x", &["bool: false"]),
            ("e8t", "x", &["null"]),
            ("e8f", "x", &["string"]),
            ("e9t", "x", &["undefined"]),
            ("e9f", "x", &["string"]),
            ("e13t", "x", &["string: a"]),
            ("e13f", "x", &["string: b", "string: c"]),
            ("e14t", "x", &["string: a"]),
            ("e14f", "x", &["string: b", "string: c"]),
            ("e26t", "x", &["string: a"]),
            ("e26f", "x", &["string"]),
            ("e27t", "x", &["string: a"]),
            ("e27f", "x", &["unknown"]),
            ("e28t", "x", &["string: a"]),
            ("e28f", "x", &["string: b", "string: c"]),
            ("e33t", "x", &["string: a"]),
            ("e33f", "x", &["string: b", "string: c"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below but `e31t` the
// expected type. `-` or `+` directly before a number literal, `-` directly
// before a bigint literal, and a template literal without substitutions have
// literal types, and `void` gives `undefined`, so a comparison with them
// narrows. Biome gives `-1n` the type `bigint`, so it keeps `bigint` at
// `e31t`, which is wider than the `-1n` that TypeScript gives.
#[test]
fn comparisons_with_negative_numbers_void_and_templates_narrow() {
    assert_reads(
        r#"
        function e4(x: number) { if (x === -1) /*e4t*/x; else /*e4f*/x; }
        function e5(x: -1 | 1) { if (x === -1) /*e5t*/x; else /*e5f*/x; }
        function e10(x: string | undefined) { if (x === void 0) /*e10t*/x; else /*e10f*/x; }
        function e11(x: string | null | undefined) { if (x == void 0) /*e11t*/x; else /*e11f*/x; }
        function e12(x: ABC) { if (x === `a`) /*e12t*/x; else /*e12f*/x; }
        function e31(x: bigint) { if (x === -1n) /*e31t*/x; else /*e31f*/x; }
        function e34(x: 1 | 2) { if (x === +1) /*e34t*/x; else /*e34f*/x; }
        "#,
        &[
            ("e4t", "x", &["number: -1"]),
            ("e4f", "x", &["number"]),
            ("e5t", "x", &["number: -1"]),
            ("e5f", "x", &["number: 1"]),
            ("e10t", "x", &["undefined"]),
            ("e10f", "x", &["string"]),
            ("e11t", "x", &["null", "undefined"]),
            ("e11f", "x", &["string"]),
            ("e12t", "x", &["string: a"]),
            ("e12f", "x", &["string: b", "string: c"]),
            ("e31t", "x", &["BigInt"]),
            ("e31f", "x", &["BigInt"]),
            ("e34t", "x", &["number: 1"]),
            ("e34f", "x", &["number: 2"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) keeps the declared type at every read up
// to `e20f`, because none of the compared values has a unit type. `-(1)` and
// `-(-1)` are `number`, `1 === 1` is `boolean`, a `typeof` expression is the
// union of the kinds, and a `let` widens its literal. TypeScript gives every
// other read the expected type, except `e22t`, `e23t`, and `e32t`, where it
// gives `"a"` and Biome keeps the declared type, which is wider: Biome
// doesn't compare with a call, a `const` that holds an `as const`
// expression, or a template literal with substitutions, whose types it
// infers. A variable with a type annotation narrows by its declared type, and
// loose equality with a literal narrows the members of the literal's kind.
#[test]
fn comparisons_with_values_that_biome_does_not_compare_keep_the_type() {
    assert_reads(
        r#"
        const P = -(1);
        const C = 1 === 1;
        declare const v: string;
        const TY = typeof v;
        let L = "a";
        let LA: "a" = "a";
        declare function unit(): "a";
        const F = unit();
        const AC = "a" as const;
        function e15(x: -1 | 1) { if (x === P) /*e15t*/x; else /*e15f*/x; }
        function e16(x: number) { if (x === -(1)) /*e16t*/x; else /*e16f*/x; }
        function e30(x: 1 | 2 | 3) { if (x === -(-1)) /*e30t*/x; }
        function e18(x: boolean) { if (x === C) /*e18t*/x; else /*e18f*/x; }
        function e19(x: string) { if (x === TY) /*e19f*/x; }
        function e20(x: ABC) { if (x === L) /*e20t*/x; else /*e20f*/x; }
        function e21(x: ABC) { if (x === LA) /*e21t*/x; else /*e21f*/x; }
        function e22(x: ABC) { if (x === F) /*e22t*/x; }
        function e23(x: ABC) { if (x === AC) /*e23t*/x; }
        function e24(x: ABC, k: "a") { if (x === k) /*e24t*/x; }
        function e25(x: ABC, k: "a" | "b") { if (x === k) /*e25t*/x; }
        function e29(x: ABC | null) { if (x == K) /*e29t*/x; else /*e29f*/x; }
        function e32(x: ABC) { if (x === `${K}`) /*e32t*/x; }
        "#,
        &[
            ("e15t", "x", &["number: -1", "number: 1"]),
            ("e15f", "x", &["number: -1", "number: 1"]),
            ("e16t", "x", &["number"]),
            ("e16f", "x", &["number"]),
            ("e30t", "x", &["number: 1", "number: 2", "number: 3"]),
            ("e18t", "x", &["boolean"]),
            ("e18f", "x", &["boolean"]),
            ("e19f", "x", &["string"]),
            ("e20t", "x", &["string: a", "string: b", "string: c"]),
            ("e20f", "x", &["string: a", "string: b", "string: c"]),
            ("e21t", "x", &["string: a"]),
            ("e21f", "x", &["string: b", "string: c"]),
            ("e22t", "x", &["string: a", "string: b", "string: c"]),
            ("e23t", "x", &["string: a", "string: b", "string: c"]),
            ("e24t", "x", &["string: a"]),
            ("e25t", "x", &["string: a", "string: b"]),
            ("e29t", "x", &["string: a"]),
            ("e29f", "x", &["null", "string: b", "string: c"]),
            ("e32t", "x", &["string: a", "string: b", "string: c"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// Where `x?.y` equals a unit value other than `undefined`, `x` is not
// nullish, and the same holds where `x?.y` differs from `undefined`.
#[test]
fn comparisons_with_unit_values_narrow_the_receivers_of_an_optional_chain() {
    assert_reads(
        r#"
        function e35(x: { y: number } | undefined) { if (x?.y === -1) /*e35t*/x; else /*e35f*/x; }
        function e36(x: { y: string } | undefined) { if (x?.y === K) /*e36t*/x; else /*e36f*/x; }
        function e37(x: { y: string } | undefined) { if (x?.y !== void 0) /*e37t*/x; else /*e37f*/x; }
        "#,
        &[
            ("e35t", "x", &["[\"y\": number]"]),
            ("e35f", "x", &["[\"y\": number]", "undefined"]),
            ("e36t", "x", &["[\"y\": string]"]),
            ("e36f", "x", &["[\"y\": string]", "undefined"]),
            ("e37t", "x", &["[\"y\": string]"]),
            ("e37f", "x", &["[\"y\": string]", "undefined"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below but `receiver` the
// expected type. An imported `const` narrows as a local one does, through the
// `const`s and imports of the module that declares it and through a
// re-export. An imported `const` whose value is `-(1)` or `1 === 1` holds no
// unit value, and neither does an imported `let`. TypeScript removes
// `undefined` from `o` at `receiver`, where Biome keeps it, which is wider,
// because a comparison with an imported variable narrows no receiver of an
// optional chain.
#[test]
fn comparisons_with_imported_constants_narrow() {
    assert_reads(
        r#"
        import { EK, EN, EV, ET, EC, EP, EB, EL, EI, EX } from "./constants.ts";
        function i1(x: ABC) { if (x === EK) /*i1t*/x; else /*i1f*/x; }
        function i2(x: -1 | 1) { if (x === EN) /*i2t*/x; else /*i2f*/x; }
        function i3(x: string | undefined) { if (x === EV) /*i3t*/x; else /*i3f*/x; }
        function i4(x: ABC) { if (x === ET) /*i4t*/x; else /*i4f*/x; }
        function i5(x: ABC) { if (x === EC) /*i5t*/x; else /*i5f*/x; }
        function i6(x: -1 | 1) { if (x === EP) /*i6t*/x; else /*i6f*/x; }
        function i7(x: boolean) { if (x === EB) /*i7t*/x; else /*i7f*/x; }
        function i8(x: ABC) { if (x === EL) /*i8t*/x; else /*i8f*/x; }
        function i9(x: ABC) { if (x === EI) /*i9t*/x; else /*i9f*/x; }
        function i10(x: ABC) { if (x === EX) /*i10t*/x; else /*i10f*/x; }
        function i11(x: string | null | undefined) { if (x == EV) /*i11t*/x; else /*i11f*/x; }
        function i12(o: { k: string } | undefined) { if (o?.k === EK) /*receiver*/o; }
        "#,
        &[
            ("i1t", "x", &["string: a"]),
            ("i1f", "x", &["string: b", "string: c"]),
            ("i2t", "x", &["number: -1"]),
            ("i2f", "x", &["number: 1"]),
            ("i3t", "x", &["undefined"]),
            ("i3f", "x", &["string"]),
            ("i4t", "x", &["string: b"]),
            ("i4f", "x", &["string: a", "string: c"]),
            ("i5t", "x", &["string: a"]),
            ("i5f", "x", &["string: b", "string: c"]),
            ("i6t", "x", &["number: -1", "number: 1"]),
            ("i6f", "x", &["number: -1", "number: 1"]),
            ("i7t", "x", &["boolean"]),
            ("i7f", "x", &["boolean"]),
            ("i8t", "x", &["string: a", "string: b", "string: c"]),
            ("i8f", "x", &["string: a", "string: b", "string: c"]),
            ("i9t", "x", &["string: c"]),
            ("i9f", "x", &["string: a", "string: b"]),
            ("i10t", "x", &["string: c"]),
            ("i10f", "x", &["string: a", "string: b"]),
            ("i11t", "x", &["null", "undefined"]),
            ("i11f", "x", &["string"]),
            ("receiver", "o", &["[\"k\": string]", "undefined"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) narrows `x` to `"a"` and `"b" | "c"` in
// `i9` and `i11`, and to `"c"` and `"a" | "b"` in `i10`. An imported variable
// with a type annotation narrows by its declared type, as a local one does.
// Biome keeps the declared type in `i10` and `i11`, which is wider, because
// it doesn't compare with an export written as an expression or a member of
// a namespace import.
#[test]
fn comparisons_with_imported_values_that_hold_no_unit_value() {
    assert_reads(
        r#"
        import D, { EA } from "./constants.ts";
        import * as C from "./constants.ts";
        function i9(x: ABC) { if (x === EA) /*i9t*/x; else /*i9f*/x; }
        function i10(x: ABC) { if (x === D) /*i10t*/x; else /*i10f*/x; }
        function i11(x: ABC) { if (x === C.EK) /*i11t*/x; else /*i11f*/x; }
        "#,
        &[
            ("i9t", "x", &["string: a"]),
            ("i9f", "x", &["string: b", "string: c"]),
            ("i10t", "x", &["string: a", "string: b", "string: c"]),
            ("i10f", "x", &["string: a", "string: b", "string: c"]),
            ("i11t", "x", &["string: a", "string: b", "string: c"]),
            ("i11f", "x", &["string: a", "string: b", "string: c"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A comparison of a discriminant with a `const` narrows the object, loose
// equality with a `const` that holds `null` or `undefined` matches both, and
// a local variable named `undefined` is compared as any other `const`.
#[test]
fn comparisons_with_constants_narrow_discriminants_and_loose_nullish_tests() {
    assert_reads(
        r#"
        import { EV } from "./constants.ts";
        type S = { kind: "c"; r: number } | { kind: "s"; w: number };
        function k1(s: S) { if (s.kind === KC) /*k1t*/s; else /*k1f*/s; }
        const KC = "c";
        function k2(s: S) { switch (s.kind) { case KC: /*k2t*/s; break; default: /*k2f*/s; } }
        function l1(x: string | null | undefined) { if (x == Z) /*l1t*/x; else /*l1f*/x; }
        function l2(x: string | null | undefined) { if (x == U) /*l2t*/x; else /*l2f*/x; }
        function l3(x: string | null | undefined) { if (x != EV) /*l3t*/x; else /*l3f*/x; }
        function u1(x: string | undefined) { const undefined = "a"; if (x === undefined) /*u1t*/x; else /*u1f*/x; }
        "#,
        &[
            ("k1t", "s", &["[\"kind\": string: c, \"r\": number]"]),
            ("k1f", "s", &["[\"kind\": string: s, \"w\": number]"]),
            ("k2t", "s", &["[\"kind\": string: c, \"r\": number]"]),
            ("k2f", "s", &["[\"kind\": string: s, \"w\": number]"]),
            ("l1t", "x", &["null", "undefined"]),
            ("l1f", "x", &["string"]),
            ("l2t", "x", &["null", "undefined"]),
            ("l2f", "x", &["string"]),
            ("l3t", "x", &["string"]),
            ("l3f", "x", &["null", "undefined"]),
            ("u1t", "x", &["string: a"]),
            ("u1f", "x", &["string", "undefined"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `boolean` at both reads, because
// `D` is `never` where the comparisons run, so they compare with no value and
// narrow nothing. Biome takes `D` as `false` there, so it gives `false`,
// which is narrower than TypeScript in code that TypeScript types as
// impossible, as the deliberate choices in `NARROWING.md` describe.
#[test]
fn comparisons_with_constants_that_a_condition_makes_impossible_narrow() {
    assert_reads(
        r#"
        const D = false;
        function n1(x: boolean) { if (D) { if (x === D) /*n1*/x; } }
        function n2(x: boolean) { if (!D) return; if (x === D) /*n2*/x; }
        "#,
        &[("n1", "x", &["bool: false"]), ("n2", "x", &["bool: false"])],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the values of the
// expected type, but displays the literal type of an enum member by the
// member's name: `E.A` where Biome displays `0`, and `E` for the union of
// an enum's members. A comparison with a member narrows as a comparison with
// its value does, in both branches, through a discriminant, and through a
// `const` that holds the member. At `e6t`, `D.P` is a member of an ambient
// enum without an initializer, whose value the code that defines the enum
// chooses, so Biome types it as `number` and keeps the declared type of `d`,
// where TypeScript gives `D.P`. At `e15f`, `G.C` has a value that Biome
// doesn't compute, so `m` keeps `number`, where TypeScript gives
// `G.B | G.C`. These are wider. Loose `==` narrows the members of the
// member's kind as `===` does, so `s` is `"a"` at `e10t`, the value of
// `S.A`. At `e16t`, the discriminant's annotation `E.A` reads as an instance
// of `0`, which has the same values.
#[test]
fn comparisons_with_enum_members_narrow() {
    assert_reads(
        r#"
        enum E { A, B }
        enum S { A = "a", B = "b" }
        const enum C { X = 1, Y = 2 }
        declare enum D { P, Q }
        enum G { A = 1, B = A * 2, C = "c".length }
        function e1(e: E) { if (e === E.A) /*e1t*/e; else /*e1f*/e; }
        function e2(x: 0 | 1) { if (x === E.A) /*e2t*/x; else /*e2f*/x; }
        function e3(s: S) { if (s === S.A) /*e3t*/s; else /*e3f*/s; }
        function e4(x: "a" | "b") { if (x === S.A) /*e4t*/x; else /*e4f*/x; }
        function e5(c: C) { if (c === C.X) /*e5t*/c; else /*e5f*/c; }
        function e6(d: D) { if (d === D.P) /*e6t*/d; }
        function e8(e: E | undefined) { if (e) /*e8t*/e; }
        function e9(e: E) { if (e === 0) /*e9t*/e; }
        function e10(s: S | null) { if (s == S.A) /*e10t*/s; }
        function e11(e: E) { if (e !== E.A && e !== E.B) /*e11t*/e; }
        function e12(e: E.A | E.B) { const v = E.A; if (e === v) /*e12t*/e; }
        function e13(x: number) { if (x === E.B) /*e13t*/x; }
        function e15(m: G) { if (m === G.A) /*e15t*/m; else /*e15f*/m; }
        function e16(o: { k: E.A; a: 1 } | { k: E.B; b: 2 }) { if (o.k === E.A) /*e16t*/o; }
        function e17(x: "a" | "b") { if (S["B"] !== x) /*e17t*/x; }
        "#,
        &[
            ("e1t", "e", &["number: 0"]),
            ("e1f", "e", &["number: 1"]),
            ("e2t", "x", &["number: 0"]),
            ("e2f", "x", &["number: 1"]),
            ("e3t", "s", &["string: a"]),
            ("e3f", "s", &["string: b"]),
            ("e4t", "x", &["string: a"]),
            ("e4f", "x", &["string: b"]),
            ("e5t", "c", &["number: 1"]),
            ("e5f", "c", &["number: 2"]),
            ("e6t", "d", &["number"]),
            ("e8t", "e", &["number: 1"]),
            ("e9t", "e", &["number: 0"]),
            ("e10t", "s", &["string: a"]),
            ("e11t", "e", &["Unknown"]),
            ("e12t", "e", &["number: 0"]),
            ("e13t", "x", &["number: 1"]),
            ("e15t", "m", &["number: 1"]),
            ("e15f", "m", &["number"]),
            (
                "e16t",
                "o",
                &["[\"k\": instanceof number: 0, \"a\": number: 1]"],
            ),
            ("e17t", "x", &["string: a"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the values of the
// expected type, displaying `EE.A` and `EE.B` at `i1t` and `i1f`. A member of
// an imported enum narrows as a member of a local one, also when a re-export
// or a `const` of another module holds it, and also with loose `==`.
#[test]
fn comparisons_with_members_of_imported_enums_narrow() {
    assert_reads(
        r#"
        import { EE, ES, EM, EMC } from "./constants.ts";
        function i1(e: EE) { if (e === EE.A) /*i1t*/e; else /*i1f*/e; }
        function i2(x: ABC) { if (x === ES["A"]) /*i2t*/x; else /*i2f*/x; }
        function i3(x: ABC) { if (x === EM.C) /*i3t*/x; else /*i3f*/x; }
        function i4(x: ABC) { if (x === EMC) /*i4t*/x; else /*i4f*/x; }
        function i5(x: ABC | null) { if (x == ES.A) /*i5t*/x; }
        "#,
        &[
            ("i1t", "e", &["number: 0"]),
            ("i1f", "e", &["number: 1"]),
            ("i2t", "x", &["string: a"]),
            ("i2f", "x", &["string: b", "string: c"]),
            ("i3t", "x", &["string: c"]),
            ("i3f", "x", &["string: a", "string: b"]),
            ("i4t", "x", &["string: c"]),
            ("i4f", "x", &["string: a", "string: b"]),
            ("i5t", "x", &["string: a"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the values of the
// expected type, displaying a member's literal type by the member's name and
// the union of an enum's members by the enum's name, except where a comment
// says otherwise. A member has the literal type of its value: auto-incremented
// from the member before it, computed from literals, operators, and earlier
// members, or written as a template literal without substitutions.
//
// Biome is wider in these cases. `d` is `number` and `D.P` is `number`,
// because a member of an ambient enum without an initializer has a value
// that the code that defines the enum chooses; TypeScript gives `D` and
// `D.P`, whose values it doesn't know either. `G.C` is `number`, because
// Biome doesn't compute `"c".length`, so `m` is `number`; TypeScript gives
// `G.C` and `G`. `H` has two declarations, and `NE` a namespace of its name,
// which Biome doesn't merge, so `h`, `H.X`, and `NE.A` are Unknown;
// TypeScript gives `H`, `H.X`, which is `0`, and `NE.A`, which is `0`.
// `F.Y = E.B` references a member of another enum, which Biome doesn't
// follow, because it can be a string, so it is Unknown; TypeScript gives `1`.
// `F.W` is a template literal with a substitution, so it is `string`;
// TypeScript gives `"aw"`. `F.X = K` has the value of the `const` `K`, `"a"`.
// Inside an enum, a member's name reads the member, as `A` in `R.B = A`.
//
// TypeScript widens a member's literal type to the enum's type where it
// initializes a mutable location: `wn`, the field `f`, and the return type of
// `wr` are `E` and `S`. Biome widens it to its primitive type, `number` or
// `string`, which is wider. `const` and `readonly` keep the member's type.
#[test]
fn enums_have_the_types_of_their_values() {
    assert_reads(
        r#"
        enum E { A, B }
        enum S { A = "a", B = "b" }
        const enum C { X = 1, Y }
        declare enum D { P, Q = 2 }
        declare const enum DC { P, Q }
        enum G { A = 1, B = A * 2, C = "c".length }
        enum H { X }
        enum H { Y = 1 }
        enum NE { A }
        namespace NE { export const x = 1; }
        enum F { X = K, Y = E.B, Z = `z`, W = `${K}w` }
        enum R { A = "a", B = /*rb*/A }
        function t1(e: E, s: S, c: C, d: D, dc: DC, m: G, h: H) { /*e*/e; /*s*/s; /*c*/c; /*d*/d; /*dc*/dc; /*m*/m; /*h*/h; }
        function t2(a: E.B, b: S.A) {
            /*a*/a; /*b*/b; /*ea*/E.A; /*eb*/E["B"]; /*mb*/G.B; /*mc*/G.C; /*hx*/H.X; /*nea*/NE.A;
            /*fx*/F.X; /*fy*/F.Y; /*fz*/F.Z; /*fw*/F.W; /*dp*/D.P; /*dq*/D.Q; /*cy*/C.Y;
        }
        let wn = E.A;
        const wc = E.A;
        class WK { f = S.A; readonly r = S.A; }
        function wr() { return E.A; }
        function t3(k: WK) { /*wn*/wn; /*wc*/wc; /*wf*/k.f; /*wro*/k.r; /*wr*/wr(); }
        "#,
        &[
            ("e", "e", &["number: 0", "number: 1"]),
            ("s", "s", &["string: a", "string: b"]),
            ("c", "c", &["number: 1", "number: 2"]),
            ("d", "d", &["number"]),
            ("dc", "dc", &["number: 0", "number: 1"]),
            ("m", "m", &["number"]),
            ("h", "h", &["Unknown"]),
            ("a", "a", &["number: 1"]),
            ("b", "b", &["string: a"]),
            ("ea", "E.A", &["number: 0"]),
            ("eb", "E[\"B\"]", &["number: 1"]),
            ("mb", "G.B", &["number: 2"]),
            ("mc", "G.C", &["number"]),
            ("hx", "H.X", &["Unknown"]),
            ("nea", "NE.A", &["Unknown"]),
            ("fx", "F.X", &["string: a"]),
            ("fy", "F.Y", &["Unknown"]),
            ("fz", "F.Z", &["string: z"]),
            ("fw", "F.W", &["string"]),
            ("dp", "D.P", &["number"]),
            ("dq", "D.Q", &["number: 2"]),
            ("cy", "C.Y", &["number: 2"]),
            ("rb", "A", &["string: a"]),
            ("wn", "wn", &["number"]),
            ("wc", "wc", &["number: 0"]),
            ("wf", "k.f", &["string"]),
            ("wro", "k.r", &["string: a"]),
            ("wr", "wr()", &["number"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// except where a comment says otherwise. Loose `==` and `!=` with a literal
// narrow the members of the literal's kind as `===` does and remove `null`
// and `undefined`, and their false branch removes only the members strictly
// equal to the literal, as in TypeScript.
//
// Deliberate choice: TypeScript narrows loose equality as strict equality,
// which also removes members of other kinds that convert to the literal. Biome
// keeps primitive types and object types of other kinds, and removes a
// literal of another kind only when JavaScript's `==` is certainly false for
// it, as for `"a"` with a number. So Biome is wider than TypeScript at `l3t`
// (TS `1`), `l4t` (TS `"1"`), `l5t` (TS `0`), `l6t` (TS `true`), `l10t`
// (TS `1n`), `l12t` (TS `1`), `l13t` (TS `true`), `l14t` (TS `"a"`), `l16t`
// (TS `-1`), and `l20t` (TS `0`).
#[test]
fn loose_equality_with_literals_narrows_the_members_of_their_kind() {
    assert_reads(
        r#"
        function l1(x: "a" | "b" | null) { if (x == "a") /*l1t*/x; else /*l1f*/x; }
        function l2(x: "a" | "b" | null) { if (x != "a") /*l2t*/x; else /*l2f*/x; }
        function l3(x: string | number) { if (x == 1) /*l3t*/x; else /*l3f*/x; }
        function l4(x: string | number) { if (x == "1") /*l4t*/x; else /*l4f*/x; }
        function l5(x: boolean | number) { if (x == 0) /*l5t*/x; else /*l5f*/x; }
        function l6(x: boolean | string) { if (x == true) /*l6t*/x; else /*l6f*/x; }
        function l7(x: 0 | 1 | "a") { if (x == 0) /*l7t*/x; else /*l7f*/x; }
        function l8(x: string) { if (x == "a") /*l8t*/x; else /*l8f*/x; }
        function l9(x: number | undefined) { if (x == 1) /*l9t*/x; else /*l9f*/x; }
        function l10(x: bigint | number) { if (x == 1n) /*l10t*/x; else /*l10f*/x; }
        function l11(x: "a" | 1 | true) { if (x == "a") /*l11t*/x; else /*l11f*/x; }
        function l12(x: "1" | 1) { if (x == 1) /*l12t*/x; else /*l12f*/x; }
        function l13(x: 1 | "x" | true) { if (x == true) /*l13t*/x; else /*l13f*/x; }
        function l14(x: { a: 1 } | "a") { if (x == "a") /*l14t*/x; else /*l14f*/x; }
        function l15(x: "a" | "b") { if ("a" == x) /*l15t*/x; }
        function l16(x: string | number) { if (x == -1) /*l16t*/x; }
        function l17(x: string | undefined) { if (x == `a`) /*l17t*/x; }
        function l18(x: "a" | "b") { const K = "a"; if (x == K) /*l18t*/x; else /*l18f*/x; }
        function l19(x: boolean | null) { if (x == false) /*l19t*/x; else /*l19f*/x; }
        function l20(x: 0 | "" | false | null) { if (x == 0) /*l20t*/x; else /*l20f*/x; }
        function l21(x: unknown) { if (x == "a") /*l21t*/x; else /*l21f*/x; }
        function l22(x: "a" | "b" | null, s: "a" | "b") { if (x == s) /*l22t*/x; }
        "#,
        &[
            ("l1t", "x", &["string: a"]),
            ("l1f", "x", &["null", "string: b"]),
            ("l2t", "x", &["null", "string: b"]),
            ("l2f", "x", &["string: a"]),
            ("l3t", "x", &["number: 1", "string"]),
            ("l3f", "x", &["number", "string"]),
            ("l4t", "x", &["number", "string: 1"]),
            ("l4f", "x", &["number", "string"]),
            ("l5t", "x", &["boolean", "number: 0"]),
            ("l5f", "x", &["boolean", "number"]),
            ("l6t", "x", &["bool: true", "string"]),
            ("l6f", "x", &["bool: false", "string"]),
            ("l7t", "x", &["number: 0"]),
            ("l7f", "x", &["number: 1", "string: a"]),
            ("l8t", "x", &["string: a"]),
            ("l8f", "x", &["string"]),
            ("l9t", "x", &["number: 1"]),
            ("l9f", "x", &["number", "undefined"]),
            ("l10t", "x", &["bigint: 1n", "number"]),
            ("l10f", "x", &["BigInt", "number"]),
            ("l11t", "x", &["string: a"]),
            ("l11f", "x", &["bool: true", "number: 1"]),
            ("l12t", "x", &["number: 1", "string: 1"]),
            ("l12f", "x", &["string: 1"]),
            ("l13t", "x", &["bool: true", "number: 1"]),
            ("l13f", "x", &["number: 1", "string: x"]),
            ("l14t", "x", &["[\"a\": number: 1]", "string: a"]),
            ("l14f", "x", &["[\"a\": number: 1]"]),
            ("l15t", "x", &["string: a"]),
            ("l16t", "x", &["number: -1", "string"]),
            ("l17t", "x", &["string: a"]),
            ("l18t", "x", &["string: a"]),
            ("l18f", "x", &["string: b"]),
            ("l19t", "x", &["bool: false"]),
            ("l19f", "x", &["bool: true", "null"]),
            ("l20t", "x", &["bool: false", "number: 0", "string:"]),
            ("l20f", "x", &["bool: false", "null", "string:"]),
            ("l21t", "x", &["unknown"]),
            ("l21f", "x", &["unknown"]),
            ("l22t", "x", &["string: a", "string: b"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// except where a comment says otherwise. A comparison with a value that is
// not a unit value narrows by the value's declared type, when an annotation
// or `as const` gives it: the true branch keeps the members that may equal a
// value of that type and replaces a primitive type with its literals of that
// kind, and the false branch narrows only by a single unit type. Both
// operands of a comparison of two subjects narrow.
//
// TypeScript narrows by the value's narrowed type at the comparison, so it
// gives `"b"` at `c11t`, where Biome keeps `"a" | "b"` from the declared type
// of `y`, which is wider. Deliberate choice: loose equality keeps members of
// other kinds, so Biome gives `string | number` at `c8t`, where TypeScript
// gives `string`.
#[test]
fn comparisons_with_annotated_values_narrow_by_their_declared_types() {
    assert_reads(
        r#"
        type Circle = { kind: "c"; r: number };
        type Square = { kind: "s"; w: number };
        function c1(x: string | number, y: "a") { if (x === y) /*c1t*/x; else /*c1f*/x; }
        function c2(x: string | number, y: string) { if (x === y) /*c2t*/x; else /*c2f*/x; }
        function c3(x: string | null, y: string) { if (x === y) { /*c3x*/x; /*c3y*/y; } }
        function c4(x: string | undefined, y: string | null) { if (x === y) { /*c4x*/x; /*c4y*/y; } else /*c4f*/x; }
        function c5(x: "a" | "b" | "c", y: "a" | "b") { if (x === y) /*c5t*/x; else /*c5f*/x; }
        function c6(x: "a" | "b", y: "a" | "b") { if (x !== y) /*c6t*/x; else /*c6f*/x; }
        function c7(x: string | number, y: "a" | 1) { if (x === y) /*c7t*/x; }
        function c8(x: string | number, y: string | null) { if (x == y) /*c8t*/x; else /*c8f*/x; }
        function c9(x: number | null, y: number) { if (x !== y) /*c9t*/x; else /*c9f*/x; }
        function c10(x: "a" | "b", y: "a") { if (x !== y) /*c10t*/x; else /*c10f*/x; }
        function c11(x: "a" | "b", y: "a" | "b") { if (y === "a") { if (x !== y) /*c11t*/x; } }
        function c12(s: Circle | Square, k: "c") { if (s.kind === k) /*c12t*/s; else /*c12f*/s; }
        function c13(x: string | number, o: { k: "a" }) { if (x === o.k) /*c13t*/x; }
        function c14(x: { a: 1 } | null, y: { a: 1 }) { if (x === y) /*c14t*/x; }
        function c15(x: string | number) { const k: "a" = "a"; if (x === k) /*c15t*/x; }
        function c16(x: string | number) { if (x === ("a" as const)) /*c16t*/x; }
        function c18(x: string | number, y: unknown) { if (x === y) { /*c18x*/x; /*c18y*/y; } }
        function c19(x: number | null, y: number) { if (y === x) /*c19t*/x; }
        function c20(x: "a" | "b", y: "a" | null) { if (x === y) /*c20t*/x; else /*c20f*/x; }
        function c21(x: string | null, y: { length: number }) { if (x === y) /*c21t*/x; }
        function c22(x: boolean | string, y: boolean) { if (x === y) /*c22t*/x; }
        function c23(x: string | null, y: null) { if (x == y) /*c23t*/x; else /*c23f*/x; }
        function c24(x: string | number, y: "a" | 1) { if (x == y) /*c24t*/x; }
        "#,
        &[
            ("c1t", "x", &["string: a"]),
            ("c1f", "x", &["number", "string"]),
            ("c2t", "x", &["string"]),
            ("c2f", "x", &["number", "string"]),
            ("c3x", "x", &["string"]),
            ("c3y", "y", &["string"]),
            ("c4x", "x", &["string"]),
            ("c4y", "y", &["string"]),
            ("c4f", "x", &["string", "undefined"]),
            ("c5t", "x", &["string: a", "string: b"]),
            ("c5f", "x", &["string: a", "string: b", "string: c"]),
            ("c6t", "x", &["string: a", "string: b"]),
            ("c6f", "x", &["string: a", "string: b"]),
            ("c7t", "x", &["number: 1", "string: a"]),
            ("c8t", "x", &["number", "string"]),
            ("c8f", "x", &["number", "string"]),
            ("c9t", "x", &["null", "number"]),
            ("c9f", "x", &["number"]),
            ("c10t", "x", &["string: b"]),
            ("c10f", "x", &["string: a"]),
            ("c11t", "x", &["string: a", "string: b"]),
            ("c12t", "s", &["[\"kind\": string: c, \"r\": number]"]),
            ("c12f", "s", &["[\"kind\": string: s, \"w\": number]"]),
            ("c13t", "x", &["string: a"]),
            ("c14t", "x", &["[\"a\": number: 1]"]),
            ("c15t", "x", &["string: a"]),
            ("c16t", "x", &["string: a"]),
            ("c18x", "x", &["number", "string"]),
            ("c18y", "y", &["unknown"]),
            ("c19t", "x", &["number"]),
            ("c20t", "x", &["string: a"]),
            ("c20f", "x", &["string: a", "string: b"]),
            ("c21t", "x", &["string"]),
            ("c22t", "x", &["boolean"]),
            ("c23t", "x", &["null"]),
            ("c23f", "x", &["string"]),
            ("c24t", "x", &["number: 1", "string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// except where a comment says otherwise. A comparison of a condition with
// `true` or `false` narrows as the condition does where it is truthy or
// falsy, also through an aliased condition, and narrows nothing when the
// condition, without parentheses and `!`, is `&&` or `||`, as in TypeScript.
//
// Deliberate choice: TypeScript takes a condition that isn't `false` as
// truthy, and one that isn't `true` as falsy. Biome does so only when every
// falsy value of the condition is `false`, or every truthy value is `true`.
// So it keeps `string | null` at `d1t` (TS `string`), where `x` is `null`
// when it is `null`, because `null !== false`, and `{ a: 1 } | null` at
// `d3f` (TS `{ a: 1 }`). It also keeps `string | null` at `d4t` (TS
// `string`), where `y` is a boolean, which the syntax doesn't show.
#[test]
fn comparisons_with_booleans_narrow_by_the_compared_condition() {
    assert_reads(
        r#"
        function b1(x: string | number) { if ((typeof x === "string") === true) /*b1t*/x; else /*b1f*/x; }
        function b2(x: string | number) { if ((typeof x === "string") == false) /*b2t*/x; }
        function b3(x: string | null) { if ((x !== null) !== false) /*b3t*/x; }
        function b4(x: string | null) { const ok = x !== null; if (ok === true) /*b4t*/x; }
        function b5(x: string | number | null) { if ((x !== null && x !== 1) === false) /*b5t*/x; else /*b5f*/x; }
        function b6(x: string | null) { if ((x && x.length > 0) !== false) /*b6t*/x; }
        function b7(x: string | null) { if ((x !== null) != false) /*b7t*/x; else /*b7f*/x; }
        function b8(x: string | null) { if ((x !== null) != true) /*b8t*/x; else /*b8f*/x; }
        function b9(x: string | null) { if (true === (x !== null)) /*b9t*/x; else /*b9f*/x; }
        function b10(x: string | null, f: (v: unknown) => v is string) { if (f(x) === true) /*b10t*/x; }
        function b11(x: string | null) { const ok = x !== null; if (ok !== false) /*b11t*/x; else /*b11f*/x; }
        function b12(x: string | null) { const ok = x !== null; if (ok == false) /*b12t*/x; }
        function b13(x: string | null) { if (!(x !== null && x.length > 0) === true) /*b13t*/x; }
        function b14(x: string | number) { if (!(typeof x === "string") === true) /*b14t*/x; }
        function b15(x: string | null) { const ok = x !== null && x.length > 0; if (ok === false) /*b15t*/x; else /*b15f*/x; }
        function b16(x: string | null) { const ok = x !== null && x.length > 0; if (ok !== true) /*b16t*/x; else /*b16f*/x; }
        function d1(x: string | null) { const ok = x && x.length > 0; if (ok !== false) /*d1t*/x; else /*d1f*/x; }
        function d3(x: { a: 1 } | null) { const ok = x && x.a === 1; if (ok === false) /*d3t*/x; else /*d3f*/x; }
        function d4(x: string | null, y: boolean) { const ok = y && x !== null; if (ok !== false) /*d4t*/x; }
        "#,
        &[
            ("b1t", "x", &["string"]),
            ("b1f", "x", &["number"]),
            ("b2t", "x", &["number"]),
            ("b3t", "x", &["string"]),
            ("b4t", "x", &["string"]),
            ("b5t", "x", &["null", "number", "string"]),
            ("b5f", "x", &["null", "number", "string"]),
            ("b6t", "x", &["null", "string"]),
            ("b7t", "x", &["string"]),
            ("b7f", "x", &["null"]),
            ("b8t", "x", &["null"]),
            ("b8f", "x", &["string"]),
            ("b9t", "x", &["string"]),
            ("b9f", "x", &["null"]),
            ("b10t", "x", &["string"]),
            ("b11t", "x", &["string"]),
            ("b11f", "x", &["null"]),
            ("b12t", "x", &["null"]),
            ("b13t", "x", &["null", "string"]),
            ("b14t", "x", &["number"]),
            ("b15t", "x", &["null", "string"]),
            ("b15f", "x", &["string"]),
            ("b16t", "x", &["null", "string"]),
            ("b16f", "x", &["string"]),
            ("d1t", "x", &["null", "string"]),
            ("d1f", "x", &["null", "string"]),
            ("d3t", "x", &["[\"a\": number: 1]", "null"]),
            ("d3f", "x", &["[\"a\": number: 1]", "null"]),
            ("d4t", "x", &["null", "string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// except where a comment says otherwise. A subject compared with itself
// learns nothing. A value whose type Biome infers, such as a variable without
// a type annotation, `let y = "a" as const`, or a call, narrows nothing,
// because Biome may infer it narrower than TypeScript does: TypeScript gives
// `"a"` at `s3t` and `s4t`, where Biome keeps the declared type, which is
// wider. Where `x?.v` equals a value whose declared type excludes
// `undefined`, and `null` for a loose comparison, `x` is not nullish.
#[test]
fn comparisons_by_declared_types_need_an_annotation() {
    assert_reads(
        r#"
        declare function make(): "a";
        function s1(x: string | null) { if (x === x) /*s1t*/x; else /*s1f*/x; }
        function s2(o: { v: string | null }) { if (o.v === o.v) /*s2t*/o.v; }
        function s3(x: "a" | "b") { let y = "a" as const; if (x === y) /*s3t*/x; }
        function s4(x: string | number) { const y = make(); if (x === y) /*s4t*/x; }
        function s5(x: string | number, y = "a") { if (x === y) /*s5t*/x; }
        function r1(x: { v: string } | undefined, y: string) { if (x?.v === y) /*r1t*/x; else /*r1f*/x; }
        function r2(x: { v: string } | undefined, y: string | undefined) { if (x?.v === y) /*r2t*/x; }
        function r3(x: { v: string } | undefined, y: string | null) { if (x?.v == y) /*r3t*/x; }
        function r4(x: { v: string } | undefined, y: string) { if (x?.v == y) /*r4t*/x; }
        function r5(x: { v: string } | undefined, y: string | null) { if (x?.v === y) /*r5t*/x; }
        "#,
        &[
            ("s1t", "x", &["null", "string"]),
            ("s1f", "x", &["null", "string"]),
            ("s2t", "o.v", &["null", "string"]),
            ("s3t", "x", &["string: a", "string: b"]),
            ("s4t", "x", &["number", "string"]),
            ("s5t", "x", &["number", "string"]),
            ("r1t", "x", &["[\"v\": string]"]),
            ("r1f", "x", &["[\"v\": string]", "undefined"]),
            ("r2t", "x", &["[\"v\": string]", "undefined"]),
            ("r3t", "x", &["[\"v\": string]", "undefined"]),
            ("r4t", "x", &["[\"v\": string]"]),
            ("r5t", "x", &["[\"v\": string]"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `T & string` at every read below:
// a comparison keeps an intersection with a type parameter as it is when the
// intersection may equal the value, and removes it otherwise.
#[test]
fn comparisons_keep_intersections_with_type_parameters() {
    assert_reads(
        r#"
        function g1<T>(x: T) { if (typeof x === "string") { if (x === "a") /*g1*/x; } }
        function g2<T>(x: T, y: "a" | "b") { if (typeof x === "string") { if (x === y) /*g2*/x; } }
        function g3<T>(x: T) { if (typeof x === "string") { if (x == "a") /*g3*/x; } }
        function g4<T>(x: T, y: string) { if (typeof x === "string") { if (x === y) /*g4*/x; } }
        "#,
        &[
            ("g1", "x", &["instanceof T & string"]),
            ("g2", "x", &["instanceof T & string"]),
            ("g3", "x", &["instanceof T & string"]),
            ("g4", "x", &["instanceof T & string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `S.A` at `member`, `S.B` at
// `narrowed`, and `S` at `declared`, types that a rule can tell from
// `string`. Biome gives each member the literal type of its value, so a rule
// that asks whether a type is a string gets `true` for all three.
#[test]
fn members_of_string_enums_are_strings() {
    const SOURCE: &str = r#"
        enum S { A = "a", B = "b" }
        function f(s: S) { /*member*/S.A; if (s === S.B) /*narrowed*/s; /*declared*/s; }
    "#;
    let db = build_files_db(&[("/src/index.ts", SOURCE)]);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    for (marker, expression) in [("member", "S.A"), ("narrowed", "s"), ("declared", "s")] {
        let range = marked_range(SOURCE, marker, expression);
        let ty = execute_type_inference_request(
            &db,
            TypeInferenceCaller::new("test", "comparisons"),
            NormalizedExpressionTypeRequest::new(module, range),
        )
        .unwrap_or_else(|| panic!("expression at {marker} must be inferred"));
        assert!(
            InferredType::new(&db, ty).is_all_string_like(),
            "{marker}: {}",
            format_inferred_type(&db, ty)
        );
    }
}

// TypeScript 7.0.2 (`tsc --strict`) gives `"a"` at `equal`, and `string` at
// `truthy` and `typeOf`. Only the discriminant of a `switch` narrows through
// `!`, so Biome keeps the declared types here, which are wider.
#[test]
fn checks_of_non_null_assertions_keep_the_type() {
    assert_reads(
        r#"
        function e40(x: "a" | "b") { if (x! === "a") /*equal*/x; }
        function e41(x: string | null) { if (x!) /*truthy*/x; }
        function e42(x: string | number) { if (typeof x! === "string") /*typeOf*/x; }
        "#,
        &[
            // TypeScript: `"a"`.
            ("equal", "x", &["string: a", "string: b"]),
            // TypeScript: `string`.
            ("truthy", "x", &["null", "string"]),
            // TypeScript: `string`.
            ("typeOf", "x", &["number", "string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the values of the
// expected type, displaying a member's literal type by the member's name. A
// member initialized with a `const` declared before the enum without a type
// annotation has the `const`'s value, also through another `const`, an
// operator, or a template literal, and inside a function. A name in the
// initializer of such a `const` reads the variable, not a member of the enum,
// as `A` in `FM.B`. Comparisons with such a member narrow by its value.
#[test]
fn enum_members_initialized_by_constants_have_their_values() {
    assert_reads(
        r#"
        const KX = "k";
        const NX = 2;
        const KKX = KX;
        const NS = NX + 1;
        const A = "outer";
        const AA = A;
        const KT = `kt`;
        enum F { X = KX, Y = NX, Z = KKX, W = NS }
        enum FM { A = "member", B = AA }
        enum FT { X = KT, Y = 1 }
        function r() { /*fx*/F.X; /*fz*/F.Z; /*fw*/F.W; /*fmb*/FM.B; /*ftx*/FT.X; }
        function c1(x: "k" | "j") { if (x === F.X) /*c1t*/x; else /*c1f*/x; }
        function c2(f: F) { if (f === F.X) /*c2t*/f; else /*c2f*/f; }
        function c3(f: F) { switch (f) { case F.X: case F.Y: break; default: /*c3*/f; } }
        function o() { const L = "l"; enum FO { X = L } /*fox*/FO.X; }
        "#,
        &[
            ("fx", "F.X", &["string: k"]),
            ("fz", "F.Z", &["string: k"]),
            ("fw", "F.W", &["number: 3"]),
            ("fmb", "FM.B", &["string: outer"]),
            ("ftx", "FT.X", &["string: kt"]),
            ("c1t", "x", &["string: k"]),
            ("c1f", "x", &["string: j"]),
            ("c2t", "f", &["string: k"]),
            ("c2f", "f", &["number: 2", "number: 3"]),
            ("c3", "f", &["number: 3"]),
            ("fox", "FO.X", &["string: l"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) computes none of the members below, which
// it reports, and gives each its own type. A `const` with a type annotation,
// one initialized with `as const`, one declared after the enum, and a `let`
// give their members Biome's Unknown type, which is wider.
#[test]
fn enum_members_initialized_by_other_variables_are_unknown() {
    assert_reads(
        r#"
        const KA: string = "ka";
        const KC = "kc" as const;
        let KL = "kl";
        enum FA { X = KA }
        enum FC { X = KC }
        enum FB { X = LATER }
        enum FL { X = KL }
        const LATER = "later";
        function r() { /*fa*/FA.X; /*fc*/FC.X; /*fb*/FB.X; /*fl*/FL.X; }
        "#,
        &[
            ("fa", "FA.X", &["Unknown"]),
            ("fc", "FC.X", &["Unknown"]),
            ("fb", "FB.X", &["Unknown"]),
            ("fl", "FL.X", &["Unknown"]),
        ],
    );
}
