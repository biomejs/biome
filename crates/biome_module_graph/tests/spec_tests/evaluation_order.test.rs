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

/// Names each member of `ty`, sorted: an interface by its name, Biome's
/// Unknown as `Unknown`, and any other type by its formatted text on one
/// line, such as `string` or `null`.
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
            text.lines().map(str::trim).collect::<Vec<_>>().join(" ")
        })
        .collect();
    names.sort();
    names
}

/// Declarations shared by the tests.
const DECLARATIONS: &str = r#"
    declare function g(): void;
    declare function use(...values: unknown[]): void;
    declare function make<T>(): T;
    interface X { f(v: unknown): number; g: { h(v: unknown): number }; k: string; a?: X }
    interface F { f(v: unknown): void }
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
                TypeInferenceCaller::new("test", "evaluationOrder"),
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
// (`narrowing-probes/m4/chain.ts` L4 to L14, `m5/order.ts` L5 to L11,
// `m5/flow.ts` L26, and `m5-l1/extra.ts` L25 to L28): the key and the
// arguments after a `?.` run only when its object is not nullish, so they see
// the object as not nullish, not as truthy, while the code after the chain
// also runs when the object is nullish.
#[test]
fn receivers_are_not_nullish_inside_their_chain() {
    assert_reads(
        r#"
        interface G { f(v: unknown): { g(v: unknown): void } }
        interface R { [k: string]: number }
        interface Callable { (v: unknown): void }
        interface A { a: F | undefined }
        interface N { a: F | null }
        function c1(x: X | undefined) { x?.f(/*call*/x); }
        function c2(x: X | undefined) { x?.g.h(/*laterCall*/x); }
        function c3(x: X | undefined) { x?.[/*key*/x ? "k" : "k"]; }
        function c4(x: X | undefined) { x?.f(0); /*after*/x; }
        function c5(x: X | undefined) { x?.a?.f(/*twoLinks*/x, /*secondReceiver*/x.a); }
        function c6(x: X | undefined) { x?.f(x?.f(/*nested*/x)); }
        function c7(x: X | undefined) { (x?.g).h(/*parenthesized*/x); }
        function c8(x: X | undefined) { x?.f(0) ?? /*coalesced*/x; }
        function c9(o: { x?: X }) { o.x?.f(/*path*/o.x); }
        function c10(x: X | undefined) { x?.f(/*pathOfReceiver*/x.k); }
        function c11(x: X | null) { x?.f(/*callArgument*/x) && /*afterTruthyCall*/x; }
        function r1(n: 0 | 1 | undefined) { n?.toFixed(/*notTruthy*/n); }
        function r2(s: "" | "a" | undefined) { s?.charAt(/*falsyString*/s as any); }
        function r3(o: F | undefined) { o?.f(/*method*/o); /*methodAfter*/o; }
        function r4(o: A | undefined) { o?.a?.f(/*outer*/o, /*inner*/o.a); }
        function r5(o: G | undefined) { o?.f(1).g(/*laterLink*/o); }
        function r6(o: R | undefined) { o?.[/*elementKey*/o as any]; }
        function r7(c: Callable | undefined) { c?.(/*optionalCall*/c); }
        function r8(x: N | undefined) { x?.a?.f(/*nullReceiver*/x, /*nullPath*/x.a); }
        function r9(x: F | undefined, y: string | null) { x?.f(y && /*inArgument*/y); /*afterArgument*/y; }
        "#,
        &[
            ("call", "x", &["X"]),
            ("laterCall", "x", &["X"]),
            ("key", "x", &["X"]),
            ("after", "x", &["X", "undefined"]),
            ("twoLinks", "x", &["X"]),
            ("secondReceiver", "x.a", &["X"]),
            ("nested", "x", &["X"]),
            ("parenthesized", "x", &["X", "undefined"]),
            ("coalesced", "x", &["X", "undefined"]),
            ("path", "o.x", &["X"]),
            ("pathOfReceiver", "x.k", &["string"]),
            ("callArgument", "x", &["X"]),
            ("afterTruthyCall", "x", &["X"]),
            ("notTruthy", "n", &["number: 0", "number: 1"]),
            ("falsyString", "s", &["string:", "string: a"]),
            ("method", "o", &["F"]),
            ("methodAfter", "o", &["F", "undefined"]),
            ("outer", "o", &["A"]),
            ("inner", "o.a", &["F"]),
            ("laterLink", "o", &["G"]),
            ("elementKey", "o", &["R"]),
            ("optionalCall", "c", &["Callable"]),
            ("nullReceiver", "x", &["N"]),
            ("nullPath", "x.a", &["F"]),
            ("inArgument", "y", &["string"]),
            ("afterArgument", "y", &["null", "string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives the narrower type in the comment for
// each read below (`narrowing-probes/m5-l1/extra.ts` L12 to L14). Biome is
// wider and sound:
// - A chain whose root is a parenthesized optional chain tests the path of
//   its root, `o.v`, but not `o`, which TypeScript narrows too.
// - `!` ends a chain, so `x?.a!.b(x)` tests nothing, where TypeScript treats
//   `x?.a!` as part of the chain.
#[test]
fn chains_end_at_parentheses_and_non_null_assertions() {
    assert_reads(
        r#"
        interface W { w(v: unknown): void }
        interface V { v?: W }
        interface B { b(v: unknown): void }
        interface Q { a?: B }
        interface O { f?(v: unknown): void }
        function e1(o: V | undefined) { (o?.v)?.w(/*rootPath*/o?.v, /*rootObject*/o); }
        function e2(x: Q | undefined) { x?.a!.b(/*nonNull*/x); }
        function e3(x: O | undefined) { x?.f?.(/*optionalMethod*/x); }
        "#,
        &[
            ("rootPath", "o?.v", &["W"]),
            // TypeScript: `V`.
            ("rootObject", "o", &["V", "undefined"]),
            // TypeScript: `Q`.
            ("nonNull", "x", &["Q", "undefined"]),
            ("optionalMethod", "x", &["O"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m5/order.ts` L12 to L18, `m5/flow.ts` L28 and L29, and
// `m5-l1/extra.ts` L15 to L24 and L29 to L31): the defaults of destructuring
// patterns and parameters, computed keys, and the initializers of `using`
// declarations see the narrowing before them, and a default runs only when
// the value is `undefined`, so the code after it doesn't see its tests.
#[test]
fn defaults_see_the_narrowing_before_them() {
    assert_reads(
        r#"
        function d1(x: string | null, o: { a?: unknown }) { const { a = /*noTest*/x } = o; }
        function d2(x: string | null) { if (x) { const { a = /*declaration*/x }: { a?: unknown } = make(); } }
        function d3(x: string | null) { if (x) { const [a = /*arrayDeclaration*/x] = make<unknown[]>(); } }
        function d4(x: string | null) { if (x) { for (const { a = /*forOfHead*/x } of make<{ a?: unknown }[]>()) {} } }
        function d5(x: string | null) { if (x) { let a: unknown; [a = /*assignment*/x] = make<unknown[]>(); } }
        function d6(x: string | null, y = x ? /*parameter*/x : 0) {}
        function d7(x: string | null, { a = x && /*parameterPattern*/x }: { a?: unknown } = {}) {}
        function d8(x: string | null, o: { a?: unknown }) { const { a = x && /*withTest*/x } = o; /*afterDefault*/x; }
        function d9(x: string | null, [a = x && /*arrayParameter*/x]: unknown[] = []) { /*body*/x; }
        function d10(x: string | null, y = x ?? /*coalescingDefault*/x, z = /*nextParameter*/x) {}
        function d11(x: string | null, ...[a = x && /*restParameter*/x]: unknown[]) {}
        function d12(x: string | null) { if (x) { try { g(); } catch ({ a = /*catch*/x }: any) {} } }
        function d13(x: string | null, o: { [k: string]: number }) { if (x) { const { [/*computedKey*/x]: a } = o; } }
        function d14(x: string | null) { if (x) { const { a: { b = /*nestedDefault*/x } = {} }: { a?: { b?: unknown } } = make(); } }
        function d15(x: string | null) { if (x) { for (let { a = /*forHead*/x }: { a?: unknown } = make(); ;) break; } }
        function d16(x: F | undefined) { const { a = x?.f(/*chainInDefault*/x) }: { a?: unknown } = make(); }
        function d17(x: string | null) { if (x) { using d = /*using*/x as any; } }
        async function d18(x: string | null) { if (x) { await using d = /*awaitUsing*/x as any; } }
        class K { m(x: string | null, y = x && /*method*/x) {} constructor(x: string | null, public y = x && /*property*/x) {} }
        const d19 = (x: string | null, y = x && /*arrow*/x) => x;
        function d20(x: string | null) { if (x) { let a: unknown, b: unknown; ({ a, b = /*shorthand*/x } = make<{ a: 1; b?: 2 }>()); } }
        "#,
        &[
            ("noTest", "x", &["null", "string"]),
            ("declaration", "x", &["string"]),
            ("arrayDeclaration", "x", &["string"]),
            ("forOfHead", "x", &["string"]),
            ("assignment", "x", &["string"]),
            ("parameter", "x", &["string"]),
            ("parameterPattern", "x", &["string"]),
            ("withTest", "x", &["string"]),
            ("afterDefault", "x", &["null", "string"]),
            ("arrayParameter", "x", &["string"]),
            ("body", "x", &["null", "string"]),
            ("coalescingDefault", "x", &["null"]),
            ("nextParameter", "x", &["null", "string"]),
            ("restParameter", "x", &["string"]),
            ("catch", "x", &["string"]),
            ("computedKey", "x", &["string"]),
            ("nestedDefault", "x", &["string"]),
            ("forHead", "x", &["string"]),
            ("chainInDefault", "x", &["F"]),
            ("using", "x", &["string"]),
            ("awaitUsing", "x", &["string"]),
            ("method", "x", &["string"]),
            ("property", "x", &["string"]),
            ("arrow", "x", &["string"]),
            ("shorthand", "x", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m5-l1/extra.ts` L6 to L9): the right operand of a
// logical assignment runs where its target is falsy for `||=`, truthy for
// `&&=`, and nullish for `??=`, so a test of a discriminant narrows the object
// that holds it there, as the same test in `||`, `&&`, or `??` does.
#[test]
fn logical_assignments_test_their_target_before_their_right_operand() {
    assert_reads(
        r#"
        interface A { kind: ""; a: 1 }
        interface B { kind: "b"; b: 2 }
        interface C { kind?: undefined; c: 1 }
        interface D { kind: "d"; d: 1 }
        function k1(o: A | B) { o.kind ||= (use(/*or*/o), "b"); }
        function k2(o: A | B) { o.kind &&= (use(/*and*/o), ""); }
        function k3(o: C | D) { o.kind ??= (use(/*coalescing*/o), undefined); }
        function k4(o: A | B) { o["kind"] ||= (use(/*elementAccess*/o), "b"); }
        function k5(o: A | B) { o.kind ||= "b"; /*afterAssignment*/o; }
        "#,
        &[
            ("or", "o", &["A"]),
            ("and", "o", &["B"]),
            ("coalescing", "o", &["C"]),
            ("elementAccess", "o", &["A"]),
            ("afterAssignment", "o", &["A", "B"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives the narrower type in the comment for
// each read below (`narrowing-probes/m5/compound.ts` L25 to L27 and
// `m5/order.ts` L19 and L20). A logical assignment writes its target, and a
// variable or a property path that the execution root writes doesn't narrow
// yet, so the target keeps its declared type in the right operand, which is
// wider than TypeScript's.
#[test]
fn written_targets_of_logical_assignments_keep_their_declared_type() {
    assert_reads(
        r#"
        function l1(x: string | undefined) { x ??= make(/*coalescing*/x); }
        function l2(x: string | undefined) { x ||= make(/*or*/x); }
        function l3(x: string | undefined) { x &&= make(/*and*/x); }
        function l4(x: string | undefined) { x ??= /*right*/x as any; /*after*/x; }
        function l5(o: { v: string | undefined }) { o.v ??= make(/*path*/o.v); }
        "#,
        &[
            // TypeScript: `undefined`.
            ("coalescing", "x", &["string", "undefined"]),
            ("or", "x", &["string", "undefined"]),
            // TypeScript: `string`.
            ("and", "x", &["string", "undefined"]),
            // TypeScript: `undefined`.
            ("right", "x", &["string", "undefined"]),
            ("after", "x", &["string", "undefined"]),
            // TypeScript: `undefined`.
            ("path", "o.v", &["string", "undefined"]),
        ],
    );
}
