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

fn normalized_type_at<'db>(
    db: &'db TestModuleDb,
    module: ModuleInfo,
    source: &str,
    marker: &str,
    expression: &str,
) -> InferredTypeData<'db> {
    let range = marked_range(source, marker, expression);
    execute_type_inference_request(
        db,
        TypeInferenceCaller::new("test", "discriminants"),
        NormalizedExpressionTypeRequest::new(module, range),
    )
    .unwrap_or_else(|| panic!("expression at {marker} {range:?} must be inferred"))
}

/// Names each member of `ty`, sorted: an interface or class instance by its
/// name, an object type literal by its members, and any other type by its
/// formatted text on one line.
fn variant_names(db: &TestModuleDb, ty: InferredTypeData) -> Vec<String> {
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
            if first.starts_with("Object")
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

/// Shapes shared by the tests, as interfaces, so that a read names each of
/// its members.
const SHAPES: &str = r#"
    interface Circle { kind: "c"; r: number }
    interface Square { kind: "s"; w: number }
    type Shape = Circle | Square;
    interface Ok { ok: true; v: number }
    interface Err { ok: false; e: string }
    declare function assert(value: unknown): asserts value;
"#;

/// Asserts the type of each `(marker, expression, expected)` read in `source`,
/// which follows [`SHAPES`], by the names of its members, as [`variant_names`]
/// gives them.
fn assert_reads(source: &str, reads: &[(&str, &str, &[&str])]) {
    let source = format!("{SHAPES}{source}");
    let (db, module) = narrowing_db(&source);
    let mismatches: Vec<_> = reads
        .iter()
        .filter_map(|(marker, expression, expected)| {
            let ty = normalized_type_at(&db, module, &source, marker, expression);
            let actual = variant_names(&db, ty);
            (actual != *expected)
                .then(|| format!("{marker}: expected {expected:?}, got {actual:?}"))
        })
        .collect();
    assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
}

// TypeScript 6.0.3 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m3/discriminants.ts` L8 to L10, L23, L26, L28, L31, and
// `m3-l2/cases.ts` L14, L15, L28 to L30, L32).
#[test]
fn equality_and_truthiness_of_a_discriminant_narrow_the_object() {
    assert_reads(
        r#"
        function f1(s: Shape) { if (s.kind === "c") /*equal*/s; else /*equalElse*/s; }
        function f2(s: Shape) { if (s.kind === "c") /*property*/s.r; }
        function f3(r: Ok | Err) { if (r.ok) /*truthy*/r; else /*falsy*/r; }
        function f4(s: Shape) { if (s["kind"] === "c") /*element*/s; }
        function f5(s: Shape) { if (s.kind !== "c") /*notEqual*/s; }
        function f6(r: Ok | Err) { if (r.ok === true) /*true*/r; }
        function f7(s: Shape) { if (s.kind === "c" && s.r > 0) /*and*/s; }
        function f8(s: Shape) { if (!(s.kind === "c")) /*not*/s; }
        function f9(s: Shape) { s.kind === "c" && /*operand*/s; }
        function f10(s: Shape) { s.kind !== "c" || /*or*/s; }
        function f11(s: Shape) { if (s.kind === "c") return; /*return*/s; }
        function f12(s: Shape) { assert(s.kind === "c"); /*assert*/s; }
        function f13(s: Shape) { if (s.kind === "c") { s.r; } /*after*/s; }
        "#,
        &[
            ("equal", "s", &["Circle"]),
            ("equalElse", "s", &["Square"]),
            ("property", "s.r", &["number"]),
            ("truthy", "r", &["Ok"]),
            ("falsy", "r", &["Err"]),
            ("element", "s", &["Circle"]),
            ("notEqual", "s", &["Square"]),
            ("true", "r", &["Ok"]),
            ("and", "s", &["Circle"]),
            ("not", "s", &["Square"]),
            ("operand", "s", &["Circle"]),
            ("or", "s", &["Circle"]),
            ("return", "s", &["Square"]),
            ("assert", "s", &["Circle"]),
            ("after", "s", &["Circle", "Square"]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) gives `Circle` in the loop and `Square`
// after it (`m3-l2/cases.ts` L27).
#[test]
fn discriminants_narrow_loops_and_their_exits() {
    assert_reads(
        r#"
        function f(s: Shape) { while (s.kind === "c") { /*loop*/s; } /*exit*/s; }
        "#,
        &[("loop", "s", &["Circle"]), ("exit", "s", &["Square"])],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m3/discriminants.ts` L11, L14, L15, L24, L25, L29, L34,
// and `m3-l2/cases.ts` L10 to L12, L37, L45, L46, L55 to L58).
#[test]
fn discriminants_of_every_unit_type_narrow_the_object() {
    assert_reads(
        r#"
        function typeOf(o: { k: "a"; a: 1 } | { k: 1; b: 2 }) {
            if (typeof o.k === "string") /*typeof*/o; else /*typeofElse*/o;
        }
        function optional(o: { k?: string; a: 1 } | { k: "b"; b: 2 }) {
            if (o.k === "b") /*optional*/o; else /*optionalElse*/o;
            if (o.k === undefined) /*undefined*/o;
        }
        function notNull(o: { v: null; e: string } | { v: number }) { if (o.v !== null) /*notNull*/o; }
        function broad(s: { kind: "c"; r: number } | { kind: string; z: 1 }) {
            if (s.kind !== "c") /*broad*/s; else /*broadElse*/s;
        }
        function object(o: { k: { n: 1 }; a: 1 } | { k: null; b: 2 }) {
            if (o.k) /*object*/o; else /*objectElse*/o;
        }
        function boolean(o: { k: boolean; a: 1 } | { k: string; b: 2 }) { if (o.k === true) /*boolean*/o; }
        function looseNull(o: { k: null; a: 1 } | { k: "x"; b: 2 }) {
            if (o.k == null) /*looseNull*/o; else /*looseNullElse*/o;
        }
        function looseUndefined(o: { k?: "x"; a: 1 } | { k: "y"; b: 2 }) {
            if (o.k != undefined) /*looseUndefined*/o; else /*looseUndefinedElse*/o;
        }
        function optionalTrue(o: { ok?: true; v: number } | { ok: false; e: string }) {
            if (o.ok) /*optionalTrue*/o; else /*optionalTrueElse*/o;
        }
        function nullable(o: { v: string | null; a: 1 } | { v: null; b: 2 }) {
            if (o.v) /*nullable*/o; else /*nullableElse*/o;
        }
        function numbers(o: { k: 1 | 2; a: 1 } | { k: 3; b: 2 }) {
            if (o.k === 1) /*numbers*/o; else /*numbersElse*/o;
        }
        function three(o: { k: "a"; a: 1 } | { k: "b"; b: 2 } | { k: "c"; c: 3 }) {
            if (o.k !== "a" && o.k !== "b") /*three*/o;
        }
        function zero(o: { k: 0; a: 1 } | { k: 1; b: 2 }) { if (o.k) /*zero*/o; else /*zeroElse*/o; }
        function empty(o: { k: ""; a: 1 } | { k: string; b: 2 }) { if (!o.k) /*empty*/o; else /*emptyElse*/o; }
        function bigint(o: { k: 0n; a: 1 } | { k: 1n; b: 2 }) { if (o.k === 0n) /*bigint*/o; }
        function negative(o: { k: -1; a: 1 } | { k: 1; b: 2 }) {
            if (o.k === 1) /*negative*/o; else /*negativeElse*/o;
        }
        "#,
        &[
            ("typeof", "o", &[r#"["k": string: a, "a": number: 1]"#]),
            ("typeofElse", "o", &[r#"["k": number: 1, "b": number: 2]"#]),
            (
                "optional",
                "o",
                &[
                    r#"["k": string: b, "b": number: 2]"#,
                    r#"["k"?: string | undefined, "a": number: 1]"#,
                ],
            ),
            (
                "optionalElse",
                "o",
                &[
                    r#"["k": string: b, "b": number: 2]"#,
                    r#"["k"?: string | undefined, "a": number: 1]"#,
                ],
            ),
            (
                "undefined",
                "o",
                &[r#"["k"?: string | undefined, "a": number: 1]"#],
            ),
            ("notNull", "o", &[r#"["v": number]"#]),
            (
                "broad",
                "s",
                &[
                    r#"["kind": string, "z": number: 1]"#,
                    r#"["kind": string: c, "r": number]"#,
                ],
            ),
            (
                "broadElse",
                "s",
                &[
                    r#"["kind": string, "z": number: 1]"#,
                    r#"["kind": string: c, "r": number]"#,
                ],
            ),
            (
                "object",
                "o",
                &[
                    r#"[ "k": Object { prototype: No prototype members: ["n": number: 1] } , "a": number: 1 ]"#,
                ],
            ),
            ("objectElse", "o", &[r#"["k": null, "b": number: 2]"#]),
            ("boolean", "o", &[r#"["k": boolean, "a": number: 1]"#]),
            ("looseNull", "o", &[r#"["k": null, "a": number: 1]"#]),
            (
                "looseNullElse",
                "o",
                &[r#"["k": string: x, "b": number: 2]"#],
            ),
            (
                "looseUndefined",
                "o",
                &[
                    r#"["k": string: y, "b": number: 2]"#,
                    r#"["k"?: string: x | undefined, "a": number: 1]"#,
                ],
            ),
            (
                "looseUndefinedElse",
                "o",
                &[r#"["k"?: string: x | undefined, "a": number: 1]"#],
            ),
            (
                "optionalTrue",
                "o",
                &[r#"["ok"?: bool: true | undefined, "v": number]"#],
            ),
            (
                "optionalTrueElse",
                "o",
                &[
                    r#"["ok": bool: false, "e": string]"#,
                    r#"["ok"?: bool: true | undefined, "v": number]"#,
                ],
            ),
            (
                "nullable",
                "o",
                &[r#"["v": string | null, "a": number: 1]"#],
            ),
            (
                "nullableElse",
                "o",
                &[
                    r#"["v": null, "b": number: 2]"#,
                    r#"["v": string | null, "a": number: 1]"#,
                ],
            ),
            (
                "numbers",
                "o",
                &[r#"["k": number: 1 | number: 2, "a": number: 1]"#],
            ),
            (
                "numbersElse",
                "o",
                &[
                    r#"["k": number: 1 | number: 2, "a": number: 1]"#,
                    r#"["k": number: 3, "b": number: 2]"#,
                ],
            ),
            ("three", "o", &[r#"["k": string: c, "c": number: 3]"#]),
            ("zero", "o", &[r#"["k": number: 1, "b": number: 2]"#]),
            ("zeroElse", "o", &[r#"["k": number: 0, "a": number: 1]"#]),
            (
                "empty",
                "o",
                &[
                    r#"["k": string, "b": number: 2]"#,
                    r#"["k": string: , "a": number: 1]"#,
                ],
            ),
            (
                "emptyElse",
                "o",
                &[
                    r#"["k": string, "b": number: 2]"#,
                    r#"["k": string: , "a": number: 1]"#,
                ],
            ),
            ("bigint", "o", &[r#"["k": bigint: 0n, "a": number: 1]"#]),
            ("negative", "o", &[r#"["k": number: 1, "b": number: 2]"#]),
            (
                "negativeElse",
                "o",
                &[r#"["k": number: -1, "a": number: 1]"#],
            ),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) keeps every read below unchanged
// (`narrowing-probes/m3/discriminants.ts` L12, L13, L27, L30, L32, L33, L36,
// `m3/predicate_discriminant.ts` L8, and `m3-l2/cases.ts` L48 and L51), unless
// a comment gives its type. L30, L36, and L51 are errors (TS2339 and TS18048).
#[test]
fn tests_of_properties_that_are_not_discriminants_keep_the_object() {
    assert_reads(
        r#"
        declare function isC(value: string): value is "c";
        function nonLiteral(o: { k: string; a: 1 } | { k: number; b: 2 }) { if (typeof o.k === "string") /*nonLiteral*/o; }
        function uniform(o: { k: string; a: 1 } | { k: string; b: 2 }) { if (o.k === "x") /*uniform*/o; }
        function sameLiterals(o: { k: "a" | "b"; x: 1 } | { k: "a" | "b"; y: 2 }) { if (o.k === "a") /*sameLiterals*/o; }
        function missing(o: { k: "a"; a: 1 } | { b: 2 }) { if (o.k === "a") /*missing*/o; }
        function partial(o: Circle | Square | { r: string }) { if (o.kind === "c") /*partial*/o; }
        function other(s: Shape, t: Shape) { if (s.kind === t.kind) /*other*/s; }
        function generic<T extends "c" | "s">(o: { kind: T; a: 1 } | { kind: "z"; b: 2 }) { if (o.kind === "z") /*generic*/o; }
        function noUnit(o: { kind: "a"; v: string | null } | { kind: "b"; v: number }) { if (o.v !== null) /*noUnit*/o; }
        function possiblyUndefined(s: Shape | undefined) { if (s.kind === "c") /*possiblyUndefined*/s; }
        function predicate(s: Shape) { if (isC(s.kind)) /*predicate*/s; }
        function single(o: Circle) { if (o.kind === "c") /*single*/o; }
        // TypeScript gives `Circle`: loose equality with a literal does not
        // narrow yet.
        function loose(s: Shape) { if (s.kind == "c") /*loose*/s; }
        "#,
        &[
            (
                "nonLiteral",
                "o",
                &[
                    r#"["k": number, "b": number: 2]"#,
                    r#"["k": string, "a": number: 1]"#,
                ],
            ),
            (
                "uniform",
                "o",
                &[
                    r#"["k": string, "a": number: 1]"#,
                    r#"["k": string, "b": number: 2]"#,
                ],
            ),
            (
                "sameLiterals",
                "o",
                &[
                    r#"["k": string: a | string: b, "x": number: 1]"#,
                    r#"["k": string: a | string: b, "y": number: 2]"#,
                ],
            ),
            (
                "missing",
                "o",
                &[r#"["b": number: 2]"#, r#"["k": string: a, "a": number: 1]"#],
            ),
            ("partial", "o", &["Circle", "Square", r#"["r": string]"#]),
            ("other", "s", &["Circle", "Square"]),
            (
                "generic",
                "o",
                &[
                    r#"["kind": instanceof T, "a": number: 1]"#,
                    r#"["kind": string: z, "b": number: 2]"#,
                ],
            ),
            (
                "noUnit",
                "o",
                &[
                    r#"["kind": string: a, "v": string | null]"#,
                    r#"["kind": string: b, "v": number]"#,
                ],
            ),
            ("possiblyUndefined", "s", &["Circle", "Square", "undefined"]),
            ("predicate", "s", &["Circle", "Square"]),
            ("single", "o", &["Circle"]),
            ("loose", "s", &["Circle", "Square"]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m3/discriminants.ts` L16 and L35, `m3-l2/cases.ts` L8,
// L9, L43, L44, L49, and `m3-l2/aliases_paths.ts` L7 and L12). A test through
// an optional chain keeps the `null` and `undefined` members of the object
// where the property reads as `undefined`.
#[test]
fn discriminants_read_through_optional_chains_narrow_the_object() {
    assert_reads(
        r#"
        function chain(s: Shape | undefined) { if (s?.kind === "c") /*chain*/s; else /*chainElse*/s; }
        function notEqual(s: Shape | null) { if (s?.kind !== "c") /*notEqual*/s; }
        function undefinedKind(s: Shape | undefined) {
            if (s?.kind === undefined) /*undefinedKind*/s; else /*undefinedKindElse*/s;
        }
        function nullObject(s: Shape | null) { if (s?.kind === "c") /*nullObject*/s; else /*nullObjectElse*/s; }
        function defined(s: Shape) { if (s?.kind === "c") /*defined*/s; }
        function path(x: { a: Shape }) { if (x.a?.kind === "c") /*path*/x.a; }
        function nullMember(o: { kind: "a"; v: null } | { kind: "b"; v: number } | null) {
            if (o?.v !== null) /*nullMember*/o;
        }
        function typeOf(s: Shape | undefined) { if (typeof s?.kind === "string") /*typeOf*/s; else /*typeOfElse*/s; }
        function inner(o: { s: Shape } | undefined) { if (o?.s.kind === "c") /*inner*/o.s; }
        "#,
        &[
            ("chain", "s", &["Circle"]),
            ("chainElse", "s", &["Square", "undefined"]),
            ("notEqual", "s", &["Square", "null"]),
            ("undefinedKind", "s", &["undefined"]),
            ("undefinedKindElse", "s", &["Circle", "Square"]),
            ("nullObject", "s", &["Circle"]),
            ("nullObjectElse", "s", &["Square", "null"]),
            ("defined", "s", &["Circle"]),
            ("path", "x.a", &["Circle"]),
            (
                "nullMember",
                "o",
                &[r#"["kind": string: b, "v": number]"#, "null"],
            ),
            ("typeOf", "s", &["Circle", "Square"]),
            ("typeOfElse", "s", &["undefined"]),
            ("inner", "o.s", &["Circle"]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m3/discriminants.ts` L17 and L18, `m3/literals.ts` L10,
// `m3-l2/cases.ts` L39 and L50): only the object that holds the property
// narrows, and a read of the property starts from the narrowed object.
#[test]
fn only_the_object_that_holds_the_discriminant_narrows() {
    assert_reads(
        r#"
        function nested(o: { a: { kind: "x"; v: 1 } } | { a: { kind: "y"; v: 2 } }) {
            if (o.a.kind === "x") { /*outer*/o; /*inner*/o.a; }
        }
        function path(w: { s: Shape }) { if (w.s.kind === "c") { /*path*/w.s; /*pathProperty*/w.s.r; } }
        function kind(s: Shape) { if (s.kind === "c") /*kind*/s.kind; else /*kindElse*/s.kind; }
        class K {
            s!: Shape;
            m() { if (this.s.kind === "c") /*this*/this.s; }
        }
        "#,
        &[
            (
                "outer",
                "o",
                &[
                    r#"[ "a": Object { prototype: No prototype members: ["kind": string: x, "v": number: 1] } ]"#,
                    r#"[ "a": Object { prototype: No prototype members: ["kind": string: y, "v": number: 2] } ]"#,
                ],
            ),
            ("inner", "o.a", &[r#"["kind": string: x, "v": number: 1]"#]),
            ("path", "w.s", &["Circle"]),
            ("pathProperty", "w.s.r", &["number"]),
            ("kind", "s.kind", &["string: c"]),
            ("kindElse", "s.kind", &["string: s"]),
            ("this", "this.s", &["Circle"]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m3/discriminants.ts` L21, `m3/literals.ts` L8, and
// `m3-l2/cases.ts` L33, L52, L53), unless a comment gives its type. A write
// to the discriminant keeps the object narrowed, because TypeScript checks
// the written value against the narrowed object's property.
#[test]
fn writes_to_the_discriminant_keep_the_object_narrowed() {
    assert_reads(
        r#"
        interface A2 { kind: "a" | "b"; x: 1 }
        interface B2 { kind: "c"; y: 2 }
        function written(s: A2 | B2) {
            if (s.kind === "a") {
                s.kind = "b";
                /*written*/s;
                // TypeScript gives `"b"`: a write narrows the property, which
                // Biome does not model yet.
                /*writtenKind*/s.kind;
            }
        }
        function same(s: Shape) { if (s.kind === "c") { s.kind = "c"; /*same*/s; } }
        function other(s: Shape) { if (s.kind === "c") { s.r = 2; /*other*/s; } }
        // TypeScript gives `Square`: a write narrows the written object, which
        // Biome does not model yet, so the written path keeps its declared
        // type.
        function object(o: { a: Shape }) { if (o.a.kind === "c") { o.a = { kind: "s", w: 1 }; /*object*/o.a; } }
        "#,
        &[
            ("written", "s", &["A2"]),
            ("writtenKind", "s.kind", &["string: a", "string: b"]),
            ("same", "s", &["Circle"]),
            ("other", "s", &["Circle"]),
            ("object", "o.a", &["Circle", "Square"]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m3/tuples.ts` L3 to L6, and `m3-l2/cases.ts` L40 and
// L41).
#[test]
fn tuple_elements_are_discriminants() {
    assert_reads(
        r#"
        function f1(t: ["a", number] | ["b", string]) { if (t[0] === "a") /*tuple*/t; else /*tupleElse*/t; }
        function f2(t: ["a", number] | ["b", string]) { if (t[0] === "a") /*element*/t[1]; }
        function f3(t: ["a", number] | ["b", string, boolean]) { if (t[0] === "b") /*longer*/t; }
        function f4(t: ["a", number?] | ["b"]) { if (t[0] === "a") /*optionalElement*/t; }
        function f5(t: readonly ["a", number] | readonly ["b", string]) { if (t[0] === "a") /*readonly*/t; }
        function f6(t: ["a", number] | ["b", string]) { if (t["0"] === "a") /*stringKey*/t; }
        "#,
        &[
            ("tuple", "t", &["[string: a, number]"]),
            ("tupleElse", "t", &["[string: b, string]"]),
            ("element", "t[1]", &["number"]),
            ("longer", "t", &["[string: b, string, boolean]"]),
            ("optionalElement", "t", &["[string: a, number?]"]),
            ("readonly", "t", &["Readonly [string: a, number]"]),
            ("stringKey", "t", &["[string: a, number]"]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m3-l2/cases.ts` L18 and L22).
#[test]
fn discriminants_of_classes_and_inherited_properties_narrow() {
    assert_reads(
        r#"
        class A { kind = "a" as const; a = 1 }
        class B { kind = "b" as const; b = 2 }
        function classes(x: A | B) { if (x.kind === "a") /*classA*/x; else /*classB*/x; }
        interface Base { id: number }
        interface D1 extends Base { kind: "d1" }
        interface D2 extends Base { kind: "d2" }
        function extended(x: D1 | D2) { if (x.kind === "d1") /*extended*/x; }
        "#,
        &[
            ("classA", "x", &["instanceof A"]),
            ("classB", "x", &["instanceof B"]),
            ("extended", "x", &["D1"]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) gives `{ ok: true; value: number }`
// (`narrowing-probes/m3-l2/cases.ts` L25). Biome keeps the declared type: the
// declared type of `r` is an instance of the generic alias, not a union, and
// no runtime test narrows such an instance yet.
#[test]
fn instances_of_generic_aliases_keep_their_declared_type() {
    const SOURCE: &str = r#"
        type Result<T> = { ok: true; value: T } | { ok: false; error: string };
        function result(r: Result<number>) { /*declared*/r; if (r.ok) /*narrowed*/r; }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    assert_eq!(
        normalized_type_at(&db, module, SOURCE, "narrowed", "r"),
        normalized_type_at(&db, module, SOURCE, "declared", "r")
    );
}

// TypeScript 6.0.3 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m3/readonly.ts` L25, `m3/aliases.ts` L13, L17, and L18,
// `m3-l2/cases.ts` L38, and `m3-l2/aliases_paths.ts` L6, L9, and L10). An
// aliased condition narrows the object when the object is a variable that
// cannot change, but never a property path such as `w.s`, which TypeScript
// narrows only when its properties are `readonly`.
#[test]
fn aliased_conditions_narrow_the_object_of_a_discriminant() {
    assert_reads(
        r#"
        function alias(s: Shape) { const ok = s.kind === "c"; if (ok) /*alias*/s; }
        function twoAliases(o: { k: "a"; v: 1 } | { k: "b"; v: 2 }) {
            const ok = o.k === "a"; const ok2 = ok; if (ok2) /*twoAliases*/o;
        }
        function negated(s: Shape) { const ok = s.kind === "c"; const ok2 = !ok; if (ok2) /*negated*/s; }
        function either(s: Shape) { if (s.kind === "c" || s.kind === "s") /*either*/s; }
        class Cl {
            kind: "a" | "b" = "a";
            m(o: { kind: "x"; v: 1 } | { kind: "y"; v: 2 }) { if (o.kind === "x") /*method*/o; }
        }
        function path(w: { s: Shape }) { const ok = w.s.kind === "c"; if (ok) /*path*/w.s; }
        function chain(s: Shape) { const ok = s?.kind === "c"; if (ok) /*chain*/s; }
        function nullable(s: Shape | undefined) { const ok = s?.kind === "c"; if (ok) /*nullable*/s; }
        "#,
        &[
            ("alias", "s", &["Circle"]),
            ("twoAliases", "o", &[r#"["k": string: a, "v": number: 1]"#]),
            ("negated", "s", &["Square"]),
            ("either", "s", &["Circle", "Square"]),
            ("method", "o", &[r#"["kind": string: x, "v": number: 1]"#]),
            ("path", "w.s", &["Circle", "Square"]),
            ("chain", "s", &["Circle"]),
            ("nullable", "s", &["Circle"]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) gives each read below the type `never`
// (`narrowing-probes/m3/aliases.ts` L13, and `m3-l2/cases.ts` L6, L7, L13,
// L36, and L54): no value of the property passes the test, so the branch is
// impossible, and Biome reads it as Unknown.
#[test]
fn impossible_discriminant_tests_read_as_unknown() {
    let source = format!(
        "{SHAPES}{}",
        r#"
        function nested(s: Shape) { if (s.kind === "c") { if (s.kind === "s") /*nested*/s; } }
        function exhausted(s: Shape) { if (s.kind === "c") {} else if (s.kind === "s") {} else /*exhausted*/s; }
        function neither(s: Shape) { if (s.kind === "c" || s.kind === "s") {} else /*neither*/s; }
        function typeofUndefined(o: { k: "a"; a: 1 } | { k: "b"; b: 2 }) { if (typeof o.k === "undefined") /*typeofUndefined*/o; }
        function truthy(s: Shape) { if (s.kind) {} else /*truthy*/s; }
        function typeofString(s: Shape) { if (typeof s.kind === "string") {} else /*typeofString*/s; }
        "#
    );
    let (db, module) = narrowing_db(&source);
    for (marker, expression) in [
        ("nested", "s"),
        ("exhausted", "s"),
        ("neither", "s"),
        ("typeofUndefined", "o"),
        ("truthy", "s"),
        ("typeofString", "s"),
    ] {
        let ty = normalized_type_at(&db, module, &source, marker, expression);
        assert_eq!(
            ty,
            InferredTypeData::Unknown,
            "{marker}: {}",
            format_inferred_type(&db, ty)
        );
    }
}

// TypeScript 6.0.3 (`tsc --strict`) keeps `o` as declared: a test of a
// property narrows only a union. Collection drops the first read, whose
// annotation shows that `o` is not a union, and the second runs one solve.
#[test]
fn property_tests_of_objects_that_are_not_unions_skip_the_solve() {
    for (source, solves) in [
        (
            "function f(o: { v: string | null }) { if (o.v) { /*read*/o; } }",
            0,
        ),
        (
            r#"function f(o: { k: "a" } | { k: "b" }) { if (o.k === "a") { /*read*/o; } }"#,
            1,
        ),
    ] {
        let (db, module) = narrowing_db(source);
        db.clear_salsa_events();
        normalized_type_at(&db, module, source, "read", "o");
        let events = db.take_salsa_events();
        assert_eq!(
            function_query_will_execute_count_by_name(&db, "infer_flow_subject_states", &events),
            solves,
            "{source}"
        );
    }
}

// TypeScript 6.0.3 (`tsc --strict`) gives every read below the expected type
// (`m3-l2/cost.ts` L10, L11, and `cases.ts` L18 with `Shape`). The parameter's
// declared type is an alias of this module, and the variables initialized
// from calls have inferred types.
#[test]
fn complete_module_inference_and_lookups_agree_on_discriminants() {
    let source = format!(
        "{SHAPES}{}",
        r#"
        declare function make(): Shape;
        function e(s: Shape) { if (s.kind === "c") /*alias*/s; else /*aliasElse*/s; }
        function f() { const s = make(); if (s.kind === "c") /*inferred*/s; else /*inferredElse*/s; }
        function g(x: string | null) { const s = make(); if (s.kind === "c" && x) /*shared*/(x && s); }
        "#
    );
    let reads: [(&str, &str, &[&str]); 5] = [
        ("alias", "s", &["Circle"]),
        ("aliasElse", "s", &["Square"]),
        ("inferred", "s", &["Circle"]),
        ("inferredElse", "s", &["Square"]),
        ("shared", "(x && s)", &["Circle", "string:"]),
    ];
    let lookups = {
        let (db, module) = narrowing_db(&source);
        reads.map(|(marker, expression, _)| {
            let range = marked_range(&source, marker, expression);
            infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range))
                .map(|ty| format_inferred_type(&db, ty))
        })
    };
    let (db, module) = narrowing_db(&source);
    let complete = infer_module_types(&db, module).unwrap();
    for ((marker, expression, expected), lookup) in reads.into_iter().zip(lookups) {
        let range = marked_range(&source, marker, expression);
        let ty = complete.expressions[&range];
        assert_eq!(Some(format_inferred_type(&db, ty)), lookup, "{marker}");
        let normalized = normalized_type_at(&db, module, &source, marker, expression);
        assert_eq!(variant_names(&db, normalized), expected, "{marker}");
    }
}

// TypeScript 6.0.3 (`tsc --strict`) keeps `o` as declared and narrows `p`
// (`m3-l2/cost.ts` L12 and L15). A test of a property of `o` can't narrow it,
// because its declared type is a single object type, so neither complete
// module inference nor lookups solve its flow. Finding out that the
// declared type of `p` is a union costs complete module inference no lookup
// of the alias, which it has already resolved.
#[test]
fn reads_that_property_tests_cannot_narrow_skip_the_solve() {
    let source = r#"
        type Pair = { k: "a"; a: 1 } | { k: "b"; b: 2 };
        declare function plain(): { v: string | null };
        declare function use(...values: unknown[]): void;
        function f() { const o = plain(); if (o.v === "a") { /*read*/o; /*call*/use(o); } }
        function g(p: Pair) { if (p.k === "a") /*pair*/p; }
    "#;
    let (db, module) = narrowing_db(source);
    db.clear_salsa_events();
    let complete = infer_module_types(&db, module).unwrap();
    let events = db.take_salsa_events();
    let pair = normalized_type_at(&db, module, source, "pair", "p");
    assert_eq!(
        format_inferred_type(
            &db,
            complete.expressions[&marked_range(source, "pair", "p")]
        ),
        format_inferred_type(&db, pair),
    );
    assert_eq!(
        variant_names(&db, pair),
        [r#"["k": string: a, "a": number: 1]"#]
    );
    for (query, count) in [
        ("infer_flow_subject_states", 1),
        ("infer_flow_binding_baseline", 0),
        ("infer_local_type", 0),
    ] {
        assert_eq!(
            function_query_will_execute_count_by_name(&db, query, &events),
            count,
            "complete module inference: {query}"
        );
    }

    let (db, module) = narrowing_db(source);
    db.clear_salsa_events();
    for (marker, expression) in [("read", "o"), ("call", "use(o)")] {
        let range = marked_range(source, marker, expression);
        infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range));
    }
    let events = db.take_salsa_events();
    for (query, count) in [
        ("infer_flow_subject_states", 0),
        ("infer_flow_subject_type", 0),
        ("infer_flow_binding_baseline", 1),
    ] {
        assert_eq!(
            function_query_will_execute_count_by_name(&db, query, &events),
            count,
            "lookups: {query}"
        );
    }
}

// TypeScript 6.0.3 (`tsc --strict`) removes the member whose property type
// can't share a value with what the test leaves (`m3-l2/objects.ts` L7 to
// L11), and narrows `s` by a `const` of a literal type (L10). Biome keeps
// both members whenever a property type is an object type and the test
// leaves a value other than `null` or `undefined`, because it doesn't compare
// object types with other types, and a comparison narrows only with a
// literal, so these reads are wider.
#[test]
fn tests_that_biome_cannot_compare_keep_more_members() {
    assert_reads(
        r#"
        type M = { k: { a: 1 }; x: 1 } | { k: "x"; y: 2 };
        function equal(o: M) { if (o.k === "x") /*equal*/o; }
        function typeOf(o: M) { if (typeof o.k === "object") /*typeof*/o; }
        function truthy(o: { k: { a: 1 }; x: 1 } | { k: 0; y: 2 }) { if (o.k) /*truthy*/o; }
        function constant(s: Shape) { const k = "c"; if (s.kind === k) /*constant*/s; }
        function notNull(o: { k: { a: 1 }; x: 1 } | { k: null; y: 2 }) { if (o.k !== null) /*notNull*/o; }
        "#,
        &[
            // TypeScript gives `{ k: "x"; y: 2 }`.
            (
                "equal",
                "o",
                &[
                    r#"[ "k": Object { prototype: No prototype members: ["a": number: 1] } , "x": number: 1 ]"#,
                    r#"["k": string: x, "y": number: 2]"#,
                ],
            ),
            // TypeScript gives `{ k: { a: 1 }; x: 1 }`.
            (
                "typeof",
                "o",
                &[
                    r#"[ "k": Object { prototype: No prototype members: ["a": number: 1] } , "x": number: 1 ]"#,
                    r#"["k": string: x, "y": number: 2]"#,
                ],
            ),
            // TypeScript gives `{ k: { a: 1 }; x: 1 }`.
            (
                "truthy",
                "o",
                &[
                    r#"[ "k": Object { prototype: No prototype members: ["a": number: 1] } , "x": number: 1 ]"#,
                    r#"["k": number: 0, "y": number: 2]"#,
                ],
            ),
            // TypeScript gives `Circle`.
            ("constant", "s", &["Circle", "Square"]),
            (
                "notNull",
                "o",
                &[
                    r#"[ "k": Object { prototype: No prototype members: ["a": number: 1] } , "x": number: 1 ]"#,
                ],
            ),
        ],
    );
}
