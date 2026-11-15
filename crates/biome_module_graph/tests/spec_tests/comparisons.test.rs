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
/// Biome's Unknown, which a read on a path that no value takes has, as
/// `Unknown`, and any other type by its formatted text on one line.
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
"#;

/// The module that [`CONSTANTS`] imports.
const MORE: &str = r#"
    export const EK = "c";
"#;

/// Asserts the type of each `(marker, expression, expected)` read in
/// `source`, which follows [`PRELUDE`] in `/src/index.ts` and can import
/// [`CONSTANTS`] from `./constants.ts`, by the names of its members, as
/// [`variant_names`] gives them. Each read has the same type in a lookup and
/// in complete module inference.
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

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/fix-e/equality.ts` L22 to L30, L34, L35, L47 to L49,
// L54): a comparison with a `const` whose initializer is a literal narrows as
// a comparison with the literal does, also through another such `const`, in
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

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/fix-e/equality.ts` L25, L26, L31 to L33, L52, L55): `-`
// or `+` directly before a number literal, `-` directly before a bigint
// literal, and a template literal without substitutions have literal types,
// and `void` gives `undefined`, so a comparison with them narrows. Biome
// gives `-1n` the type `bigint`, so it keeps `bigint` at `e31t`, where
// TypeScript gives `-1n`, which is wider.
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

// TypeScript 7.0.2 (`tsc --strict`) keeps the declared type at the first
// eight reads (`narrowing-probes/fix-e/equality.ts` L36, L37, L39 to L41,
// L51): `-(1)` and `-(-1)` are `number`, `1 === 1` is `boolean`, a `typeof`
// expression is the union of the kinds, and a `let` widens its literal, so
// none of them is a unit type. It narrows at the other reads (L42 to L46,
// L50, L53) to `"a"`, `"a" | "b"`, and `"b" | "c" | null` where Biome keeps
// the declared type, which is wider: Biome doesn't compare with a variable
// that has a type annotation, a call, `as const`, a template literal with
// substitutions, or a literal with loose equality yet.
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
            ("e21t", "x", &["string: a", "string: b", "string: c"]),
            ("e21f", "x", &["string: a", "string: b", "string: c"]),
            ("e22t", "x", &["string: a", "string: b", "string: c"]),
            ("e23t", "x", &["string: a", "string: b", "string: c"]),
            ("e24t", "x", &["string: a", "string: b", "string: c"]),
            ("e25t", "x", &["string: a", "string: b", "string: c"]),
            (
                "e29t",
                "x",
                &["null", "string: a", "string: b", "string: c"],
            ),
            (
                "e29f",
                "x",
                &["null", "string: a", "string: b", "string: c"],
            ),
            ("e32t", "x", &["string: a", "string: b", "string: c"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/fix-e/equality.ts` L56 to L58): a comparison of `x?.y`
// with a unit value that isn't `undefined` shows, where they are equal, that
// `x` is not nullish, and a comparison with `undefined` shows it where they
// differ.
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

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/fix-e/imports.ts` L6 to L13): an imported `const`
// narrows as a local one does, through the `const`s and imports of the
// module that declares it and through a re-export, while one whose value is
// `-(1)`, `1 === 1`, or a `let` is no unit value. A comparison with an
// imported variable narrows no receiver of an optional chain, where
// TypeScript removes `undefined` from `o` at `receiver`, which is wider.
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

// TypeScript 7.0.2 (`tsc --strict`) narrows `x` to `"a"` and `"b" | "c"` at
// the first four reads, to `"c"` and `"a" | "b"` at the next two, and `x` to
// `0` and `1` at the last two (`narrowing-probes/fix-e/imports.ts` L14 to
// L16, L19, L23). Biome keeps the declared type, which is wider: it doesn't
// compare with an imported variable that has a type annotation, an export
// written as an expression, a member of a namespace import, or an enum
// member, whose type Biome doesn't infer yet.
#[test]
fn comparisons_with_other_imported_values_keep_the_type() {
    assert_reads(
        r#"
        import D, { EA } from "./constants.ts";
        import * as C from "./constants.ts";
        enum E { A, B }
        function i9(x: ABC) { if (x === EA) /*i9t*/x; else /*i9f*/x; }
        function i10(x: ABC) { if (x === D) /*i10t*/x; else /*i10f*/x; }
        function i11(x: ABC) { if (x === C.EK) /*i11t*/x; else /*i11f*/x; }
        function i14(x: 0 | 1) { if (x === E.A) /*i14t*/x; else /*i14f*/x; }
        "#,
        &[
            ("i9t", "x", &["string: a", "string: b", "string: c"]),
            ("i9f", "x", &["string: a", "string: b", "string: c"]),
            ("i10t", "x", &["string: a", "string: b", "string: c"]),
            ("i10f", "x", &["string: a", "string: b", "string: c"]),
            ("i11t", "x", &["string: a", "string: b", "string: c"]),
            ("i11f", "x", &["string: a", "string: b", "string: c"]),
            ("i14t", "x", &["number: 0", "number: 1"]),
            ("i14f", "x", &["number: 0", "number: 1"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/fix-e/switch.ts` L39, L41, L50 to L52, L55): a
// comparison of a discriminant with a `const` narrows the object, loose
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

// TypeScript 7.0.2 (`tsc --strict`) gives `boolean` at both reads
// (`narrowing-probes/fix-e/switch.ts` L45, L46): `D` is `never` where the
// comparisons run, so they compare with no value and narrow nothing. Biome
// takes `D` as `false` there, so it gives `false`, which is narrower than
// TypeScript in code that TypeScript types as impossible, as the Limits in
// `NARROWING.md` describe.
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
