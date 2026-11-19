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

/// Declarations shared by the tests. Each object type is an interface, so
/// [`variant_names`] names an object member of a read by its interface.
const TYPES: &str = r#"
    interface A { a: 1 }
    interface V { v?: A }
    interface P { v: A | null }
    interface Xs { xs: (string | null)[] }
    interface Circle { kind: "c"; r: number }
    interface Square { kind: "s"; w: number }
    type Shape = Circle | Square;
    interface Nc { kind: "c"; v: number }
    interface Ns { kind: "s"; v: string }
    interface Nv { v: string | null }
    interface R { r: number }
    interface W { w: number }
    declare function more(): boolean;
"#;

/// Asserts that the type of each `(marker, expression, expected)` read in
/// `source` has the sorted member names `expected`, and that complete module
/// inference gives each read the same type as a lookup of that read alone.
///
/// [`TYPES`] is prepended to `source`. See [`variant_names`] for how members
/// are named.
fn assert_reads(source: &str, reads: &[(&str, &str, &[&str])]) {
    let source = format!("{TYPES}{source}");
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
                TypeInferenceCaller::new("test", "loops"),
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
// Conditions in the body narrow the loop variable, its paths, and the
// elements of its pattern. A read at the top of the body sees the entry,
// where the variable has its declared type, joined with the end of the body.
#[test]
fn conditions_in_the_body_narrow_the_loop_variable() {
    assert_reads(
        r#"
        function f1(xs: (string | null)[]) { for (const v of xs) { if (v !== null) /*test*/v; } }
        function f2(xs: (string | null)[]) { for (const v of xs) { if (v === null) continue; /*afterContinue*/v; } }
        function f3(xs: (string | null)[]) { for (const v of xs) { /*beforeBreak*/v; if (v === null) break; } }
        function f15(xs: Shape[]) { for (const s of xs) { if (s.kind === "c") /*discriminant*/s; } }
        function f17(xs: (Nc | Ns)[]) { for (const { kind, v } of xs) { if (kind === "c") /*pattern*/v; } }
        function f18(xs: [string, number | null][]) { for (const [k, v] of xs) { if (v !== null) /*tuple*/v; } }
        function f23(xs: (string | null)[]) { for (let v of xs) { if (v === null) continue; /*let*/v; } }
        function f27(m: Map<string, number | null>) { for (const [k, v] of m) { if (v !== null) /*map*/v; } }
        function f28(o: Record<string, number>) { for (const k in o) { /*key*/k; } }
        function e16(xs: (string | null)[]) { for (const v of xs) { const w = v; if (w !== null) /*held*/w; /*heldAfter*/w; } }
        function e21(xs: (string | null)[]) { for (const v of xs) { if (v === null) { continue; } else { /*else*/v; } /*afterElse*/v; } }
        function e22(xs: Nv[]) { for (const o of xs) { if (o.v !== null) /*path*/o.v; } }
        async function e26(xs: (string | null)[]) { for await (const v of xs) { if (v === null) continue; /*awaitContinue*/v; } }
        function e27(o: A | null | undefined) { for (const k in o) { /*nullableKey*/k; } }
        function e28(xs: (string | null)[]) { for (const v of xs) { if (v !== null) { /*first*/v; } } for (const v of xs) { /*second*/v; } }
        "#,
        &[
            ("test", "v", &["string"]),
            ("afterContinue", "v", &["string"]),
            ("beforeBreak", "v", &["null", "string"]),
            ("discriminant", "s", &["Circle"]),
            ("pattern", "v", &["number"]),
            ("tuple", "v", &["number"]),
            ("let", "v", &["string"]),
            ("map", "v", &["number"]),
            ("key", "k", &["string"]),
            ("held", "w", &["string"]),
            ("heldAfter", "w", &["null", "string"]),
            ("else", "v", &["string"]),
            ("afterElse", "v", &["string"]),
            ("path", "o.v", &["string"]),
            ("awaitContinue", "v", &["string"]),
            ("nullableKey", "k", &["string"]),
            ("first", "v", &["string"]),
            ("second", "v", &["null", "string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// The facts from before the loop hold in the body and after it, and the
// expression runs once before the loop. The code after the loop continues
// from the loop's header and from the `break` statements that target the
// loop, so a test in the body never narrows that code.
#[test]
fn the_header_is_the_exit_of_the_loop() {
    assert_reads(
        r#"
        function f4(xs: number[], y: string | null) { if (y === null) return; for (const v of xs) { /*beforeInBody*/y; } /*beforeAfter*/y; }
        function f5(xs: number[], y: string | null) { for (const v of xs) { if (y === null) return; /*returnInBody*/y; } /*returnAfter*/y; }
        function f6(xs: number[], y: string | null) { for (const v of xs) { if (y !== null) break; } /*breakAfter*/y; }
        function f7(xs: number[], y: string | null) { for (const v of xs) { /*topOfBody*/y; if (y === null) return; } }
        function f8(xs: number[], y: string | null) { for (const v of xs) { if (y === null) continue; } /*continueAfter*/y; }
        function f20(xs: (string | null)[] | undefined) { for (const v of xs ?? []) { /*expression*/xs; } }
        function f21(o: Xs | undefined) { if (!o) return; for (const v of o.xs) { /*object*/o; } }
        function f22(xs: (string | null)[], y: string | null) { outer: for (const v of xs) { for (const w of xs) { if (y === null) continue outer; /*inner*/y; } /*afterInner*/y; } }
        function f24(xs: number[], y: string | null) { for (const v of xs) { if (y === null) throw 0; } /*throwAfter*/y; }
        function f25(xs: number[], y: string | null) { for (const v of (y === null ? [] : xs)) { /*conditionalExpression*/y; } }
        function e10(xs: (string | null)[], y: string | null) { outer: for (const v of xs) { for (const w of xs) { if (y === null) break outer; } /*afterLabeledBreak*/y; } /*labeledBreakAfter*/y; }
        function e11(xs: (string | null)[], y: string | null) { for (const v of xs) { if (y === null) break; /*afterBreak*/y; } /*breakExit*/y; }
        function e18(xs: (string | null)[], y: string | null) { for (const v of xs) { if (y === null) continue; /*continueInBody*/y; } }
        function e20(xs: (string | null)[], y: string | null) { for (const v of xs) /*bodyStatement*/y; if (y === null) return; /*afterLoop*/y; }
        function e24(xs: (string | null)[], y: string | null) { for (const v of xs) { if (y !== null) { /*beforeBreakOut*/y; break; } } /*breakOut*/y; }
        function e25(xs: (string | null)[], y: string | null) { lbl: for (const v of xs) { if (y === null) continue lbl; /*labeledContinue*/y; } }
        "#,
        &[
            ("beforeInBody", "y", &["string"]),
            ("beforeAfter", "y", &["string"]),
            ("returnInBody", "y", &["string"]),
            ("returnAfter", "y", &["null", "string"]),
            ("breakAfter", "y", &["null", "string"]),
            ("topOfBody", "y", &["null", "string"]),
            ("continueAfter", "y", &["null", "string"]),
            (
                "expression",
                "xs",
                &["instanceof Array<string | null>", "undefined"],
            ),
            ("object", "o", &["Xs"]),
            ("inner", "y", &["string"]),
            ("afterInner", "y", &["null", "string"]),
            ("throwAfter", "y", &["null", "string"]),
            ("conditionalExpression", "y", &["null", "string"]),
            ("afterLabeledBreak", "y", &["null", "string"]),
            ("labeledBreakAfter", "y", &["null", "string"]),
            ("afterBreak", "y", &["string"]),
            ("breakExit", "y", &["null", "string"]),
            ("continueInBody", "y", &["string"]),
            ("bodyStatement", "y", &["null", "string"]),
            ("afterLoop", "y", &["string"]),
            ("beforeBreakOut", "y", &["string"]),
            ("breakOut", "y", &["null", "string"]),
            ("labeledContinue", "y", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) narrows every read below to `string`. A
// loop neither stops the narrowing in its execution root nor loses the facts
// from before it.
#[test]
fn facts_from_before_a_loop_reach_its_body_and_the_code_after_it() {
    assert_reads(
        r#"
        declare const items: (string | null)[];
        declare function check(): boolean;
        function reads(value: string | null) { if (value === null) return; for (const item of items) { /*forOf*/value; } for (const key in items) { /*forIn*/value; } /*after*/value; }
        function loops(value: string | null) { for (const item of items) { check(); } if (value !== null) { /*elsewhere*/value; } }
        "#,
        &[
            ("forOf", "value", &["string"]),
            ("forIn", "value", &["string"]),
            ("after", "value", &["string"]),
            ("elsewhere", "value", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A `for...in` loop whose head declares a variable makes its expression, and
// the receivers of its optional chain, not nullish in the body, but not
// after the loop.
#[test]
fn for_in_makes_its_expression_not_nullish_in_the_body() {
    assert_reads(
        r#"
        function f9(o: A | undefined) { for (const k in o) { /*body*/o; } /*after*/o; }
        function f10(o: A | null) { for (const k in o) { /*null*/o; } }
        function f11(o: V) { for (const k in o.v) { /*path*/o.v; } }
        function f12(o: V | undefined) { for (const k in o?.v) { /*receiver*/o; /*chain*/o?.v; } }
        function f14(o: A | undefined) { for (var k in o) { /*var*/o; } }
        function e3(o: A | undefined) { for (let k in o) { /*let*/o; } }
        function e4(o: A | undefined) { for (const k in (o)) { /*parentheses*/o; } }
        function e6(s: Shape | undefined) { for (const k in s?.kind) { /*kindReceiver*/s; } }
        function e8(o: A | undefined, xs: number[]) { for (const k in o) { for (const v of xs) { /*innerLoop*/o; } /*afterInnerLoop*/o; } /*outerAfter*/o; }
        function e9(o: A | undefined) { for (const k in o) { if (more()) continue; /*afterContinue*/o; } }
        function e17(o: A | undefined, y: string | null) { for (const k in o) { if (y === null) return; } /*exit*/y; }
        function e19(o: P) { for (const k in o.v) { /*pathBody*/o.v; } /*pathAfter*/o.v; }
        "#,
        &[
            ("body", "o", &["A"]),
            ("after", "o", &["A", "undefined"]),
            ("null", "o", &["A"]),
            ("path", "o.v", &["A"]),
            ("receiver", "o", &["V"]),
            ("chain", "o?.v", &["A"]),
            ("var", "o", &["A"]),
            ("let", "o", &["A"]),
            ("parentheses", "o", &["A"]),
            ("kindReceiver", "s", &["Circle", "Square"]),
            ("innerLoop", "o", &["A"]),
            ("afterInnerLoop", "o", &["A"]),
            ("outerAfter", "o", &["A", "undefined"]),
            ("afterContinue", "o", &["A"]),
            ("exit", "y", &["null", "string"]),
            ("pathBody", "o.v", &["A"]),
            ("pathAfter", "o.v", &["A", "null"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) keeps the declared type at every read
// below. A `for...in` loop narrows nothing when its head is an assignment
// target or a destructuring pattern, which TypeScript rejects (TS2491). A
// test of a property's value narrows no object, so `for (const k in s.kind)`
// keeps `s`. A rest element never narrows, as elsewhere. TypeScript gives
// `rest` the type `{ r: number } | { w: number }` at both reads, while
// Biome's declared type of a rest element of an object pattern is an object
// without members.
#[test]
fn loops_that_narrow_nothing_keep_the_declared_type() {
    assert_reads(
        r#"
        function f13(o: A | undefined) { let k: string; for (k in o) { /*assigned*/o; } }
        function f16(xs: Shape[]) { for (const { kind, ...rest } of xs) { /*restDeclared*/rest; if (kind === "c") /*rest*/rest; } }
        function m9(o: A | undefined, k: string) { for (k in o) { /*parameter*/o; } }
        function e1(o: A | undefined) { for (const [c] in o) { /*arrayPattern*/o; } }
        function e2(o: A | undefined) { for (const { length } in o) { /*objectPattern*/o; } }
        function e7(s: { kind: "c" | null; r: 1 } | { kind: "s"; w: 2 }) { for (const k in s.kind) { /*property*/s; } }
        "#,
        &[
            ("assigned", "o", &["A", "undefined"]),
            // Biome's declared type of a rest element of an object pattern has
            // no members.
            (
                "restDeclared",
                "rest",
                &["Object { prototype: No prototype members: [] }"],
            ),
            (
                "rest",
                "rest",
                &["Object { prototype: No prototype members: [] }"],
            ),
            ("parameter", "o", &["A", "undefined"]),
            ("arrayPattern", "o", &["A", "undefined"]),
            ("objectPattern", "o", &["A", "undefined"]),
            (
                "property",
                "s",
                &[
                    r#"Object { prototype: No prototype members: ["kind": string: c | null, "r": number: 1] }"#,
                    r#"Object { prototype: No prototype members: ["kind": string: s, "w": number: 2] }"#,
                ],
            ),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A `switch` in the body narrows as anywhere else, `break` leaves the
// `switch`, and `continue` reaches the loop's header.
#[test]
fn switch_statements_in_the_body_narrow() {
    assert_reads(
        r#"
        function f25(xs: ("a" | "b")[], y: string | null) { for (const x of xs) { switch (x) { case "a": if (y === null) continue; /*afterContinue*/y; } } }
        function f4(xs: number[], y: string | null) { for (const n of xs) { switch (n) { case 1: if (y === null) break; /*afterBreak*/y; } /*afterSwitch*/y; } }
        function e15(xs: Shape[]) { for (const s of xs) { switch (s.kind) { case "c": /*case*/s; break; default: /*default*/s; } } }
        "#,
        &[
            ("afterContinue", "y", &["string"]),
            ("afterBreak", "y", &["string"]),
            ("afterSwitch", "y", &["null", "string"]),
            ("case", "s", &["Circle"]),
            ("default", "s", &["Square"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) narrows `o` to `A` at the first two reads
// below, and gives `string | null`, then `string`, at the last two. Biome
// keeps the declared type at the first, because `o!` is not a variable or a
// property path. A read in a closure created in the body sees the test of
// the loop, as in TypeScript. Biome doesn't infer the type of the elements
// of an `AsyncIterable`, so `v` is Unknown, which no test narrows.
#[test]
fn loops_that_biome_does_not_narrow_yet() {
    assert_reads(
        r#"
        function e5(o: A | undefined) { for (const k in o!) { /*nonNull*/o; } }
        function e23(o: A | undefined) { for (const k in o) { const f = () => /*closure*/o; } }
        async function f19(xs: AsyncIterable<string | null>) { for await (const v of xs) { /*asyncIterableDeclared*/v; if (v) /*asyncIterable*/v; } }
        "#,
        &[
            ("nonNull", "o", &["A", "undefined"]),
            ("closure", "o", &["A"]),
            ("asyncIterableDeclared", "v", &["Unknown"]),
            ("asyncIterable", "v", &["Unknown"]),
        ],
    );
}
