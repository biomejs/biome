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

/// Names each member of `ty`, sorted: an interface by its name, an object
/// type literal by its members, and any other type by its formatted text on
/// one line.
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

/// Asserts the type of each `(marker, expression, expected)` read in `source`
/// by comparing the sorted names of its members with `expected`.
///
/// The module under test is [`SHAPES`] followed by `source`, and
/// [`variant_names`] names the members.
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

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
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

// TypeScript 7.0.2 (`tsc --strict`) gives `Circle` in the loop and `Square`
// after it.
#[test]
fn discriminants_narrow_loops_and_their_exits() {
    assert_reads(
        r#"
        function f(s: Shape) { while (s.kind === "c") { /*loop*/s; } /*exit*/s; }
        "#,
        &[("loop", "s", &["Circle"]), ("exit", "s", &["Square"])],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
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

// TypeScript 7.0.2 (`tsc --strict`) keeps every read below unchanged, unless
// a comment gives its type. It reports an error in `missing` and `partial`
// (TS2339) and in `possiblyUndefined` (TS18048).
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

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A test through an optional chain keeps the `null` and `undefined` members of
// the object where the property reads as `undefined`.
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

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// Only the object that holds the property narrows, and a read of the property
// starts from the narrowed object.
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

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// unless a comment gives its type. A write to the discriminant keeps the
// object narrowed, because TypeScript checks the written value against the
// narrowed object's property.
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
                /*writtenKind*/s.kind;
            }
        }
        function same(s: Shape) { if (s.kind === "c") { s.kind = "c"; /*same*/s; } }
        function other(s: Shape) { if (s.kind === "c") { s.r = 2; /*other*/s; } }
        function object(o: { a: Shape }) { if (o.a.kind === "c") { o.a = { kind: "s", w: 1 }; /*object*/o.a; } }
        "#,
        &[
            ("written", "s", &["A2"]),
            ("writtenKind", "s.kind", &["string: b"]),
            ("same", "s", &["Circle"]),
            ("other", "s", &["Circle"]),
            ("object", "o.a", &["Square"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
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

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
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

// TypeScript 7.0.2 (`tsc --strict`) gives `{ ok: true; value: number }`.
// Biome keeps the declared type, because the declared type of `r` is an
// instance of the generic alias, not a union, and no runtime test narrows such
// an instance.
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

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// An aliased condition narrows the object when the object is a variable that
// cannot change, or a property path whose properties are all `readonly`, so
// not `w.s`.
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

// TypeScript 7.0.2 (`tsc --strict`) gives each read below the type `never`,
// because no value of the property passes the test. The branch is impossible,
// and Biome reads it as Unknown.
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

// TypeScript 7.0.2 (`tsc --strict`) keeps `o` as declared in the first source
// and narrows it to `{ k: "a" }` in the second, because a test of a property
// narrows only a union. The annotation of `o` in the first source shows that
// it is not a union, so collection drops the read. The second source runs one
// solve.
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

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// The parameter's declared type is an alias of this module, and the variables
// initialized from calls have inferred types.
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

// TypeScript 7.0.2 (`tsc --strict`) keeps `o` as declared and narrows `p`.
// A test of a property of `o` can't narrow it, because its declared type is a
// single object type, so neither complete module inference nor lookups solve
// its flow. Finding out that the declared type of `p` is a union costs
// complete module inference no lookup of the alias, which it has already
// resolved.
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

// TypeScript 7.0.2 (`tsc --strict`) removes the member whose property type
// can't share a value with what the test leaves, and narrows `s` by a `const`
// of a literal type to `Circle`, as Biome does. Biome keeps both members
// whenever a property type is an object type and the test leaves a value
// other than `null` or `undefined`, because it doesn't compare object types
// with other types, so the other reads are wider.
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
            ("constant", "s", &["Circle"]),
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

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A test of a `const` that holds a property of an object, as `k` holds
// `s.kind`, narrows the object as a test of the property does, and only that
// object.
#[test]
fn discriminants_held_in_constants_narrow_the_object() {
    assert_reads(
        r#"
        function equal(s: Shape) { const k = s.kind; if (k === "c") /*equal*/s; else /*equalElse*/s; }
        function property(s: Shape) { const k = s.kind; if (k === "c") /*property*/s.r; }
        function element(s: Shape) { const k = s["kind"]; if (k === "c") /*element*/s; }
        function parenthesized(s: Shape) { const k = (s.kind); if (k === "c") /*parenthesized*/s; }
        function object(s: Shape) { const k = (s).kind; if (k === "c") /*object*/s; }
        function escaped(s: Shape) { const k = s.\u006bind; if (k === "c") /*escaped*/s; }
        function path(w: { s: Shape }) { const k = w.s.kind; if (k === "c") /*path*/w.s; }
        function typeOf(o: { k: "a"; a: 1 } | { k: 1; b: 2 }) {
            const k = o.k; if (typeof k === "string") /*typeof*/o; else /*typeofElse*/o;
        }
        function truthy(r: Ok | Err) { const ok = r.ok; if (ok) /*truthy*/r; else /*falsy*/r; }
        function negated(r: Ok | Err) { const ok = r.ok; if (!ok) /*negated*/r; }
        function notEqual(s: Shape) { const k = s.kind; if (k !== "c") /*notEqual*/s; }
        function isNull(o: { k: null; a: 1 } | { k: "x"; b: 2 }) {
            const k = o.k; if (k === null) /*null*/o; else /*nullElse*/o;
        }
        function looseNull(o: { k?: "x"; a: 1 } | { k: "y"; b: 2 }) {
            const k = o.k; if (k == null) /*looseNull*/o; else /*looseNullElse*/o;
        }
        function notUndefined(o: { k?: "x"; a: 1 } | { k: "y"; b: 2 }) {
            const k = o.k; if (k !== undefined) /*notUndefined*/o; else /*notUndefinedElse*/o;
        }
        function tuple(t: ["a", number] | ["b", string]) {
            const k = t[0]; if (k === "a") /*tuple*/t; else /*tupleElse*/t;
        }
        function and(s: Shape, x: string | null) { const k = s.kind; if (k === "c" && x) /*and*/s; }
        function after(s: Shape) { const k = s.kind; if (k === "c") { s; } /*after*/s; }
        function loop(s: Shape) { const k = s.kind; while (k === "c") { /*loop*/s; } /*exit*/s; }
        function operand(s: Shape) { const k = s.kind; k === "c" && /*operand*/s; }
        function early(s: Shape) { const k = s.kind; if (k === "c") return; /*return*/s; }
        function kind(s: Shape) { const k = s.kind; if (k === "c") /*kind*/k; }
        function nested(o: { a: Shape } | { a: Circle; b: 1 }) { const k = o.a.kind; if (k === "s") /*outer*/o; }
        "#,
        &[
            ("equal", "s", &["Circle"]),
            ("equalElse", "s", &["Square"]),
            ("property", "s.r", &["number"]),
            ("element", "s", &["Circle"]),
            ("parenthesized", "s", &["Circle"]),
            ("object", "s", &["Circle"]),
            ("escaped", "s", &["Circle"]),
            ("path", "w.s", &["Circle"]),
            ("typeof", "o", &[r#"["k": string: a, "a": number: 1]"#]),
            ("typeofElse", "o", &[r#"["k": number: 1, "b": number: 2]"#]),
            ("truthy", "r", &["Ok"]),
            ("falsy", "r", &["Err"]),
            ("negated", "r", &["Err"]),
            ("notEqual", "s", &["Square"]),
            ("null", "o", &[r#"["k": null, "a": number: 1]"#]),
            ("nullElse", "o", &[r#"["k": string: x, "b": number: 2]"#]),
            (
                "looseNull",
                "o",
                &[r#"["k"?: string: x | undefined, "a": number: 1]"#],
            ),
            (
                "looseNullElse",
                "o",
                &[
                    r#"["k": string: y, "b": number: 2]"#,
                    r#"["k"?: string: x | undefined, "a": number: 1]"#,
                ],
            ),
            (
                "notUndefined",
                "o",
                &[
                    r#"["k": string: y, "b": number: 2]"#,
                    r#"["k"?: string: x | undefined, "a": number: 1]"#,
                ],
            ),
            (
                "notUndefinedElse",
                "o",
                &[r#"["k"?: string: x | undefined, "a": number: 1]"#],
            ),
            ("tuple", "t", &["[string: a, number]"]),
            ("tupleElse", "t", &["[string: b, string]"]),
            ("and", "s", &["Circle"]),
            ("after", "s", &["Circle", "Square"]),
            ("loop", "s", &["Circle"]),
            ("exit", "s", &["Square"]),
            ("operand", "s", &["Circle"]),
            ("return", "s", &["Square"]),
            ("kind", "k", &["string: c"]),
            (
                "outer",
                "o",
                &[
                    r#"[ "a": interface "Circle" { extends: [] type_args: [] members: ["kind": string: c, "r": number] } , "b": number: 1 ]"#,
                    r#"[ "a": interface "Circle" { extends: [] type_args: [] members: ["kind": string: c, "r": number] } | interface "Square" { extends: [] type_args: [] members: ["kind": string: s, "w": number] } ]"#,
                ],
            ),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A test of a `const` that holds `s?.kind` tests no receiver, so `s` keeps
// `undefined` wherever the property can be `undefined`. A truthiness test also
// inlines `s?.kind` as an aliased condition, which shows that `s` is not
// nullish.
#[test]
fn discriminants_held_through_optional_chains_keep_nullish_objects() {
    assert_reads(
        r#"
        function chain(s: Shape | undefined) { const k = s?.kind; if (k === "c") /*chain*/s; else /*chainElse*/s; }
        function defined(s: Shape) { const k = s?.kind; if (k === "c") /*defined*/s; }
        function notEqual(s: Shape | undefined) { const k = s?.kind; if (k !== "c") /*notEqual*/s; }
        function undefinedKind(s: Shape | undefined) {
            const k = s?.kind; if (k === undefined) /*undefinedKind*/s; else /*undefinedKindElse*/s;
        }
        function truthy(s: Shape | undefined) { const k = s?.kind; if (k) /*truthy*/s; else /*falsy*/s; }
        function receiver(w: { s: Shape } | undefined) {
            const k = w?.s.kind; if (k === "c") { /*receiver*/w; /*receiverPath*/w?.s; }
        }
        "#,
        &[
            ("chain", "s", &["Circle", "undefined"]),
            ("chainElse", "s", &["Square", "undefined"]),
            ("defined", "s", &["Circle"]),
            ("notEqual", "s", &["Square", "undefined"]),
            ("undefinedKind", "s", &["undefined"]),
            ("undefinedKindElse", "s", &["Circle", "Square", "undefined"]),
            ("truthy", "s", &["Circle", "Square"]),
            ("falsy", "s", &["undefined"]),
            (
                "receiver",
                "w",
                &[
                    r#"[ "s": interface "Circle" { extends: [] type_args: [] members: ["kind": string: c, "r": number] } | interface "Square" { extends: [] type_args: [] members: ["kind": string: s, "w": number] } ]"#,
                    "undefined",
                ],
            ),
            ("receiverPath", "w?.s", &["Circle", "undefined"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A test of an element of a destructuring `const` narrows the initializer as a
// test of the element's property does.
#[test]
fn destructured_discriminants_narrow_the_initializer() {
    assert_reads(
        r#"
        type S = { kind: "c"; v: number } | { kind: "s"; v: string };
        function shorthand(s: Shape) { const { kind } = s; if (kind === "c") /*shorthand*/s; else /*shorthandElse*/s; }
        function renamed(s: Shape) { const { kind: k } = s; if (k === "c") /*renamed*/s; }
        function others(s: S) { const { kind, v } = s; if (kind === "c") /*others*/s; }
        function otherDefault(s: S) { const { kind, v = 0 } = s; if (kind === "c") /*otherDefault*/s; }
        function parenthesized(s: Shape) { const { kind } = (s); if (kind === "c") /*parenthesized*/s; }
        function path(w: { s: Shape }) { const { kind } = w.s; if (kind === "c") /*path*/w.s; }
        function string(s: Shape) { const { "kind": k } = s; if (k === "c") /*string*/s; }
        function computed(s: Shape) { const { ["kind"]: k } = s; if (k === "c") /*computed*/s; }
        function escaped(s: Shape) { const { \u006bind } = s; if (kind === "c") /*escaped*/s; }
        function escapedKey(s: Shape) { const { \u006bind: k } = s; if (k === "c") /*escapedKey*/s; }
        function array(t: ["a", number] | ["b", string]) { const [k] = t; if (k === "a") /*array*/t; else /*arrayElse*/t; }
        function hole(t: [number, "a"] | [string, "b"]) { const [, k] = t; if (k === "a") /*hole*/t; }
        function arrayRest(t: ["a", number] | ["b", string]) { const [k, ...others] = t; if (k === "a") /*arrayRest*/t; }
        function index(t: ["a", number] | ["b", string]) { const { 0: k } = t; if (k === "a") /*index*/t; }
        function indexString(t: ["a", number] | ["b", string]) { const { "0": k } = t; if (k === "a") /*indexString*/t; }
        function truthy(r: Ok | Err) { const { ok } = r; if (ok) /*truthy*/r; else /*falsy*/r; }
        function typeOf(o: { k: "a"; a: 1 } | { k: 1; b: 2 }) { const { k } = o; if (typeof k === "number") /*typeof*/o; }
        function objectRest(s: Shape) { const { kind, ...others } = s; if (kind === "c") /*objectRest*/s; }
        function declarators(s: Shape) { const { kind } = s, k = s.kind; if (k === "s") /*declarators*/s; }
        function notEqual(s: Shape) { const { kind } = s; if (kind !== "c") /*notEqual*/s; }
        "#,
        &[
            ("shorthand", "s", &["Circle"]),
            ("shorthandElse", "s", &["Square"]),
            ("renamed", "s", &["Circle"]),
            ("others", "s", &[r#"["kind": string: c, "v": number]"#]),
            (
                "otherDefault",
                "s",
                &[r#"["kind": string: c, "v": number]"#],
            ),
            ("parenthesized", "s", &["Circle"]),
            ("path", "w.s", &["Circle"]),
            ("string", "s", &["Circle"]),
            ("computed", "s", &["Circle"]),
            ("escaped", "s", &["Circle"]),
            ("escapedKey", "s", &["Circle"]),
            ("array", "t", &["[string: a, number]"]),
            ("arrayElse", "t", &["[string: b, string]"]),
            ("hole", "t", &["[number, string: a]"]),
            ("arrayRest", "t", &["[string: a, number]"]),
            ("index", "t", &["[string: a, number]"]),
            ("indexString", "t", &["[string: a, number]"]),
            ("truthy", "r", &["Ok"]),
            ("falsy", "r", &["Err"]),
            ("typeof", "o", &[r#"["k": number: 1, "b": number: 2]"#]),
            ("objectRest", "s", &["Circle"]),
            ("declarators", "s", &["Square"]),
            ("notEqual", "s", &["Square"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) keeps every read below as declared. Only
// an unannotated `const` holds a discriminant, and only when its initializer
// reads the property directly or through a pattern that the declarator binds,
// without a default.
#[test]
fn constants_that_hold_no_discriminant_keep_the_object() {
    assert_reads(
        r#"
        declare function isC(value: string): value is "c";
        function annotated(s: Shape) { const k: string = s.kind; if (k === "c") /*annotated*/s; }
        function mutable(s: Shape) { let k = s.kind; if (k === "c") /*mutable*/s; }
        function asserted(s: Shape) { const k = s.kind as string; if (k === "c") /*asserted*/s; }
        function predicate(s: Shape) { const k = s.kind; if (isC(k)) /*predicate*/s; }
        function truthy(s: Shape) { const k = s.kind; if (k) /*truthy*/s; }
        function other(s: Shape, t: Shape) { const k = s.kind; if (k === "c") /*other*/t; }
        function ownDefault(s: Shape) { const { kind = "c" } = s; if (kind === "c") /*ownDefault*/s; }
        function nonNull(s: Shape | undefined) { const { kind } = s!; if (kind === "c") /*nonNull*/s; }
        function cast(s: Shape) { const { kind } = s as Shape; if (kind === "c") /*cast*/s; }
        function annotatedPattern(s: Shape) { const { kind }: Shape = s; if (kind === "c") /*annotatedPattern*/s; }
        function mutablePattern(s: Shape) { let { kind } = s; if (kind === "c") /*mutablePattern*/s; }
        function nested(w: { a: Shape }) { const { a: { kind } } = w; if (kind === "c") /*nested*/w.a; }
        function elementDefault(t: ["a", number] | ["b", string]) { const [k = "a"] = t; if (k === "a") /*elementDefault*/t; }
        function nestedProperty(s: Shape) { const { kind: { length } } = s; if (length === 1) /*nestedProperty*/s; }
        "#,
        &[
            ("annotated", "s", &["Circle", "Square"]),
            ("mutable", "s", &["Circle", "Square"]),
            ("asserted", "s", &["Circle", "Square"]),
            ("predicate", "s", &["Circle", "Square"]),
            ("truthy", "s", &["Circle", "Square"]),
            ("other", "t", &["Circle", "Square"]),
            ("ownDefault", "s", &["Circle", "Square"]),
            ("nonNull", "s", &["Circle", "Square", "undefined"]),
            ("cast", "s", &["Circle", "Square"]),
            ("annotatedPattern", "s", &["Circle", "Square"]),
            ("mutablePattern", "s", &["Circle", "Square"]),
            ("nested", "w.a", &["Circle", "Square"]),
            (
                "elementDefault",
                "t",
                &["[string: a, number]", "[string: b, string]"],
            ),
            ("nestedProperty", "s", &["Circle", "Square"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `Shape` at the first two reads and
// `Circle` at the others. An exported `const` holds no discriminant, but the
// object it reads can be exported, or a `let` at the top level of a file,
// because only the `const` must keep its value.
#[test]
fn discriminants_held_at_the_top_level_follow_exports() {
    assert_reads(
        r#"
        declare function make(): Shape;
        const s1: Shape = make();
        export const k1 = s1.kind;
        if (k1 === "c") /*exported*/s1;
        const s4: Shape = make();
        export const { kind: k4 } = s4;
        if (k4 === "c") /*exportedPattern*/s4;
        export const s3: Shape = make();
        const k3 = s3.kind;
        if (k3 === "c") /*exportedObject*/s3;
        let s5: Shape = make();
        const k5 = s5.kind;
        if (k5 === "c") /*topLevelLet*/s5;
        "#,
        &[
            ("exported", "s1", &["Circle", "Square"]),
            ("exportedPattern", "s4", &["Circle", "Square"]),
            ("exportedObject", "s3", &["Circle"]),
            ("topLevelLet", "s5", &["Circle"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) narrows every read below, giving
// `Circle` or the first tuple. A test inside a `switch`, a `for...of` loop,
// or a closure that reads the variables of its enclosing function narrows as
// anywhere else, so Biome also gives `Circle` at `inSwitch`, `forOf`,
// `closure`, and `patternClosure`. Biome keeps the declared type at the
// other reads, because it doesn't follow `s!` and keys that are spelled
// differently from the property, loose equality doesn't narrow an object,
// and a `const` exported by an `export { ... }` list holds nothing.
#[test]
fn discriminant_constants_that_biome_does_not_follow_keep_the_object() {
    assert_reads(
        r#"
        declare function make(): Shape;
        function nonNull(s: Shape) { const k = s!.kind; if (k === "c") /*nonNull*/s; }
        function loose(s: Shape) { const k = s.kind; if (k == "c") /*loose*/s; }
        function closure(s: Shape) { const k = s.kind; const g = () => { if (k === "c") /*closure*/s; }; }
        function escapedString(s: Shape) { const k = s["\u006bind"]; if (k === "c") /*escapedString*/s; }
        function hexadecimal(t: ["a", number] | ["b", string]) { const k = t[0x0]; if (k === "a") /*hexadecimal*/t; }
        function template(s: Shape) { const { [`kind`]: k } = s; if (k === "c") /*template*/s; }
        function escapedStringKey(s: Shape) { const { "\u006bind": k } = s; if (k === "c") /*escapedStringKey*/s; }
        function patternClosure(s: Shape) { const { kind } = s; const g = () => { if (kind === "c") /*patternClosure*/s; }; }
        function looseKey(s: Shape) { const { kind } = s; if (kind == "c") /*looseKey*/s; }
        function hexadecimalKey(t: ["a", number] | ["b", string]) { const { 0x0: k } = t; if (k === "a") /*hexadecimalKey*/t; }
        function forOf(s: Shape) { const k = s.kind; for (const x of [1]) { if (k === "c") /*forOf*/s; } }
        function inSwitch(s: Shape) { const k = s.kind; switch (true) { default: if (k === "c") /*inSwitch*/s; } }
        const s2: Shape = make();
        const k2 = s2.kind;
        export { k2 };
        if (k2 === "c") /*exportList*/s2;
        "#,
        &[
            ("nonNull", "s", &["Circle", "Square"]),
            ("loose", "s", &["Circle", "Square"]),
            ("closure", "s", &["Circle"]),
            ("escapedString", "s", &["Circle", "Square"]),
            (
                "hexadecimal",
                "t",
                &["[string: a, number]", "[string: b, string]"],
            ),
            ("template", "s", &["Circle", "Square"]),
            ("escapedStringKey", "s", &["Circle", "Square"]),
            ("patternClosure", "s", &["Circle"]),
            ("looseKey", "s", &["Circle", "Square"]),
            (
                "hexadecimalKey",
                "t",
                &["[string: a, number]", "[string: b, string]"],
            ),
            ("forOf", "s", &["Circle"]),
            ("inSwitch", "s", &["Circle"]),
            ("exportList", "s2", &["Circle", "Square"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `Ok`. It takes a rest element as
// the property with its name, so `ok` holds `r.ok`, although `ok` holds an
// object, which is always truthy. Biome keeps the declared type, because `r`
// can be `Err` there.
#[test]
fn rest_elements_hold_no_discriminant() {
    assert_reads(
        r#"
        function objectRest(r: Ok | Err) { const { ...ok } = r; if (ok) /*objectRest*/r; }
        "#,
        &[("objectRest", "r", &["Err", "Ok"])],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// Inside an aliased condition, the held discriminant narrows the object only
// when the object cannot change between the `const` and the test, so not a
// property path with a mutable property, such as `w.s`. Outside one, a
// `const` in a block or a loop, and one that holds a property of a path on
// `this`, narrow as elsewhere.
#[test]
fn held_discriminants_narrow_through_aliased_conditions() {
    assert_reads(
        r#"
        function access(s: Shape) { const k = s.kind; const ok = k === "c"; if (ok) /*access*/s; }
        function pattern(s: Shape) { const { kind } = s; const ok = kind === "c"; if (ok) /*pattern*/s; }
        function path(w: { s: Shape }) { const k = w.s.kind; const ok = k === "c"; if (ok) /*path*/w.s; }
        function patternPath(w: { s: Shape }) { const { kind } = w.s; const ok = kind === "c"; if (ok) /*patternPath*/w.s; }
        function block(s: Shape) { { const k = s.kind; if (k === "c") /*block*/s; } }
        function loop(s: Shape) { if (true) { const k = s.kind; } const k = s.kind; for (;;) { if (k === "c") /*loop*/s; } }
        class K {
            s!: Shape;
            m() { const k = this.s.kind; if (k === "c") /*thisPath*/this.s; }
            n() { const { kind } = this.s; if (kind === "c") /*thisPattern*/this.s; }
        }
        "#,
        &[
            ("access", "s", &["Circle"]),
            ("pattern", "s", &["Circle"]),
            ("path", "w.s", &["Circle", "Square"]),
            ("patternPath", "w.s", &["Circle", "Square"]),
            ("block", "s", &["Circle"]),
            ("loop", "s", &["Circle"]),
            ("thisPath", "this.s", &["Circle"]),
            ("thisPattern", "this.s", &["Circle"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `Circle` at every read below,
// although in the first six functions the write can make `s.kind` differ from
// the value the `const` holds, so `s` can be a `Square`, or not match any
// member, at the read. Biome keeps the declared type there, because a test of
// a `const` that holds a property, or an aliased condition that tests it,
// narrows the object only when the root's own code never writes the property.
// A test of the property itself reads its current value, and a call or a
// callback that writes the property keeps the narrowing, as in TypeScript.
#[test]
fn writes_to_a_held_discriminant_stop_the_narrowing() {
    assert_reads(
        r#"
        declare function reset(s: Shape): void;
        function access(s: Shape) { const k = s.kind; s.kind = "s"; if (k === "c") /*access*/s; }
        function pattern(s: Shape) { const { kind } = s; s.kind = "s"; if (kind === "c") /*pattern*/s; }
        function condition(s: Shape) { const ok = s.kind === "c"; s.kind = "s"; if (ok) /*condition*/s; }
        function path(w: { s: Shape }) { const k = w.s.kind; w.s.kind = "s"; if (k === "c") /*path*/w.s; }
        function anyKey(s: Shape, key: "kind") { const k = s.kind; s[key] = "s"; if (k === "c") /*anyKey*/s; }
        function destructuring(s: Shape) {
            const k = s.kind; ({ kind: s.kind } = { kind: "s" as const }); if (k === "c") /*destructuring*/s;
        }
        // The write follows the test, so the `const` still holds the
        // property's value at the test. Biome doesn't model writes yet, so
        // any write in the root stops the narrowing.
        function later(s: Shape) { const k = s.kind; if (k === "c") { s.kind = "c"; /*later*/s; } }
        function call(s: Shape) { const k = s.kind; reset(s); if (k === "c") /*call*/s; }
        function callback(s: Shape) { const k = s.kind; [1].forEach(() => { s.kind = "s"; }); if (k === "c") /*callback*/s; }
        function direct(s: Shape) { s.kind = "s"; if (s.kind === "c") /*direct*/s; }
        "#,
        &[
            ("access", "s", &["Circle", "Square"]),
            ("pattern", "s", &["Circle", "Square"]),
            ("condition", "s", &["Circle", "Square"]),
            ("path", "w.s", &["Circle", "Square"]),
            ("anyKey", "s", &["Circle", "Square"]),
            ("destructuring", "s", &["Circle", "Square"]),
            ("later", "s", &["Circle", "Square"]),
            ("call", "s", &["Circle"]),
            ("callback", "s", &["Circle"]),
            ("direct", "s", &["Circle"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) keeps `o` as declared, because a test of
// a property narrows only a union. A held discriminant makes its object a
// subject that only tests of its properties narrow, so collection drops the
// read of `o` when its annotation shows that it is not a union, and a read of
// a union runs one solve.
#[test]
fn held_discriminants_of_objects_that_are_not_unions_skip_the_solve() {
    for (source, solves) in [
        (
            "function f(o: { v: string | null }) { const { v } = o; if (v) { /*read*/o; } }",
            0,
        ),
        (
            "function f(o: { v: string | null }) { const v = o.v; if (v) { /*read*/o; } }",
            0,
        ),
        (
            r#"function f(o: { k: "a" } | { k: "b" }) { const { k } = o; if (k === "a") { /*read*/o; } }"#,
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

/// Discriminated unions shared by the tests of destructured elements.
const DESTRUCTURED: &str = r#"
    type S = { kind: "c"; v: number } | { kind: "s"; v: string };
    type R = { ok: true; v: number } | { ok: false; v: string };
    type N = { e: null; v: number } | { e: Error; v: undefined };
    type K = { k: "a"; v: 1 } | { k: 1; v: 2 };
    declare const g: S;
"#;

/// Asserts the reads of `source` with [`assert_reads`], after prepending
/// [`DESTRUCTURED`].
fn assert_element_reads(source: &str, reads: &[(&str, &str, &[&str])]) {
    assert_reads(&format!("{DESTRUCTURED}{source}"), reads);
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A test of one element of a destructuring pattern narrows the value the
// pattern destructures, and another element of the same pattern reads its
// property on the narrowed value.
#[test]
fn tests_of_destructured_elements_narrow_the_other_elements() {
    assert_element_reads(
        r#"
        interface IC { kind: "c"; v: number }
        interface IS { kind: "s"; v: string }
        declare function make(): S;
        function declarator(s: S) { const { kind, v } = s; if (kind === "c") /*declarator*/v; else /*declaratorElse*/v; }
        function parameter({ kind, v }: S) { if (kind === "c") /*parameter*/v; else /*parameterElse*/v; }
        function renamed(s: S) { const { kind: k, v } = s; if (k === "c") /*renamed*/v; }
        function nested(w: { a: S }) { const { a: { kind, v } } = w; if (kind === "c") /*nested*/v; }
        function array(t: ["a", number] | ["b", string]) { const [k, x] = t; if (k === "a") /*array*/x; }
        function hole(t: ["a", 0, number] | ["b", 0, string]) { const [k, , x] = t; if (k === "a") /*hole*/x; }
        function arrayInObject(w: { t: ["a", number] | ["b", string] }) { const { t: [k, x] } = w; if (k === "a") /*arrayInObject*/x; }
        function rest(...[k, x]: ["a", number] | ["b", string]) { if (k === "a") /*rest*/x; else /*restElse*/x; }
        function restArity(...[k, x, y]: ["a", number] | ["b", string, boolean]) { if (k === "b") /*restArity*/y; }
        function moduleConst() { const { kind, v } = g; if (kind === "c") /*moduleConst*/v; }
        function path(w: { s: S }) { const { kind, v } = w.s; if (kind === "c") /*path*/v; }
        function cast(s: S) { const { kind, v } = s as S; if (kind === "c") /*cast*/v; }
        function computed(s: S) { const { ["kind"]: kind, v } = s; if (kind === "c") /*computed*/v; }
        function annotated(s: S) { const { kind, v }: S = s; if (kind === "c") /*annotated*/v; }
        function parameterDefault({ kind, v }: S = g) { if (kind === "c") /*parameterDefault*/v; }
        function earlyReturn({ kind, v }: S = g) { if (kind !== "c") return; /*earlyReturn*/v; }
        function returned(s: S) { const { kind, v } = s; if (kind === "c") return; /*returned*/v; }
        function tested(s: S) { const { kind, v } = s; if (kind === "c") /*tested*/kind; }
        function truthy(r: R) { const { ok, v } = r; if (ok) /*truthy*/v; else /*falsy*/v; }
        function typeOf(o: K) { const { k, v } = o; if (typeof k === "string") /*typeof*/v; else /*typeofElse*/v; }
        function notEqual(s: S) { const { kind, v } = s; if (kind !== "c") /*notEqual*/v; else /*notEqualElse*/v; }
        function isNull(n: N) { const { e, v } = n; if (e === null) /*null*/v; else /*notNull*/v; }
        function looseNull(n: N) { const { e, v } = n; if (e == null) /*looseNull*/v; if (e != undefined) /*looseNotNull*/v; }
        function propertyWrite(s: S) { const { kind, v } = s; s.kind = "s"; if (kind === "c") /*propertyWrite*/v; }
        function objectWrite(s: S, o: S) { const { kind, v } = s; s = o; if (kind === "c") /*objectWrite*/v; }
        function writeBetween(s: S) { const { kind, v } = s; if (kind === "c") { s.v = "x" as any; /*writeBetween*/v; } }
        const arrow = ({ kind, v }: S) => kind === "c" ? /*arrow*/v : 0;
        function callback(xs: S[]) { xs.forEach(({ kind, v }) => { if (kind === "c") /*callback*/v; }); }
        function interfaces(s: IC | IS) { const { kind, v } = s; if (kind === "c") /*interfaces*/v; }
        function call() { const { kind, v } = make(); if (kind === "c") /*call*/v; }
        function and(s: S) { const { kind, v } = s; kind === "c" && /*and*/v; }
        function loop(s: S) { const { kind, v } = s; while (kind === "c") { /*loop*/v; } }
        function nestedDeclaration({ kind, v }: S) { function h() { kind; } if (kind === "c") /*nestedDeclaration*/v; }
        function forOf(shapes: S[]) { for (const { kind, v } of shapes) { kind === "c" && /*forOf*/v; } }
        function forOfIf(shapes: S[]) { for (const { kind, v } of shapes) { if (kind === "c") /*forOfIf*/v; } }
        export const { kind: exported, v: exportedValue } = g;
        if (exported === "c") /*exported*/exportedValue;
        "#,
        &[
            ("declarator", "v", &["number"]),
            ("declaratorElse", "v", &["string"]),
            ("parameter", "v", &["number"]),
            ("parameterElse", "v", &["string"]),
            ("renamed", "v", &["number"]),
            ("nested", "v", &["number"]),
            ("array", "x", &["number"]),
            ("hole", "x", &["number"]),
            ("arrayInObject", "x", &["number"]),
            ("rest", "x", &["number"]),
            ("restElse", "x", &["string"]),
            ("restArity", "y", &["boolean"]),
            ("moduleConst", "v", &["number"]),
            ("path", "v", &["number"]),
            ("cast", "v", &["number"]),
            ("computed", "v", &["number"]),
            ("annotated", "v", &["number"]),
            ("parameterDefault", "v", &["number"]),
            ("earlyReturn", "v", &["number"]),
            ("returned", "v", &["string"]),
            ("tested", "kind", &["string: c"]),
            ("truthy", "v", &["number"]),
            ("falsy", "v", &["string"]),
            ("typeof", "v", &["number: 1"]),
            ("typeofElse", "v", &["number: 2"]),
            ("notEqual", "v", &["string"]),
            ("notEqualElse", "v", &["number"]),
            ("null", "v", &["number"]),
            ("notNull", "v", &["undefined"]),
            ("looseNull", "v", &["number"]),
            ("looseNotNull", "v", &["undefined"]),
            ("propertyWrite", "v", &["number"]),
            ("objectWrite", "v", &["number"]),
            ("writeBetween", "v", &["number"]),
            ("arrow", "v", &["number"]),
            ("callback", "v", &["number"]),
            ("interfaces", "v", &["number"]),
            ("call", "v", &["number"]),
            ("and", "v", &["number"]),
            ("loop", "v", &["number"]),
            ("nestedDeclaration", "v", &["number"]),
            ("forOf", "v", &["number"]),
            ("forOfIf", "v", &["number"]),
            ("exported", "exportedValue", &["number"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// An aliased condition narrows the pattern when its declaration is a `const`
// or a parameter whose variables are never written.
#[test]
fn aliased_conditions_narrow_destructured_elements() {
    assert_element_reads(
        r#"
        function declarator(s: S) { const { kind, v } = s; const ok = kind === "c"; if (ok) /*declarator*/v; }
        function parameter({ kind, v }: S) { const ok = kind === "c"; if (ok) /*parameter*/v; }
        function twice({ kind, v }: S) { const ok = kind === "c"; const again = ok; if (again) /*twice*/v; }
        "#,
        &[
            ("declarator", "v", &["number"]),
            ("parameter", "v", &["number"]),
            ("twice", "v", &["number"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A destructured element that is tested itself starts from its property on the
// narrowed pattern, and so does an optional property, which keeps `undefined`.
// Destructuring throws on `null` and `undefined`, so the pattern narrows as if
// its initializer had neither.
#[test]
fn destructured_elements_start_from_the_narrowed_pattern() {
    assert_element_reads(
        r#"
        type I = { kind: "a"; v: string | null } | { kind: "b"; v: number };
        type O = { kind: "c"; v?: number } | { kind: "s"; v: string };
        function tested(i: I) { const { kind, v } = i; if (kind === "a" && v !== null) /*tested*/v; }
        function optional(o: O) { const { kind, v } = o; if (kind === "c") /*optional*/v; }
        function narrowedInitializer(s: S | undefined) { if (!s) return; const { kind, v } = s; if (kind === "c") /*narrowedInitializer*/v; }
        function nullish(s: S | undefined) { const { kind, v } = s ?? g; if (kind === "c") /*nullish*/v; }
        "#,
        &[
            ("tested", "v", &["string"]),
            ("optional", "v", &["number", "undefined"]),
            ("narrowedInitializer", "v", &["number"]),
            ("nullish", "v", &["number"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) keeps every read below as declared. Only
// a direct element of a pattern with at least two elements, without a default
// and not a rest element, narrows the other elements, and only when its
// declaration is a `const` or a parameter whose variables are never written,
// and when the tested element is a discriminant.
#[test]
fn destructured_elements_that_do_not_narrow_each_other() {
    assert_element_reads(
        r#"
        type W = { kind: "c"; a: { v: number } } | { kind: "s"; a: { v: string } };
        function mutable(s: S) { let { kind, v } = s; if (kind === "c") /*mutable*/v; }
        function readDefault(s: S) { const { kind, v = 0 } = s; if (kind === "c") /*readDefault*/v; }
        function testedDefault(s: S) { const { kind = "c", v } = s; if (kind === "c") /*testedDefault*/v; }
        function restElement(s: S) { const { kind, ...rest } = s; if (kind === "c") /*restElement*/rest; }
        function notDiscriminant(s: S) { const { kind, v } = s; if (typeof v === "number") /*notDiscriminant*/kind; }
        function written({ kind, v }: S, flag: boolean) { if (flag) kind = "s"; if (kind === "c") /*written*/v; }
        function writtenInClosure({ kind, v }: S) { const h = () => { v = "x"; }; if (kind === "c") /*writtenInClosure*/v; }
        function writtenAfter({ kind, v }: S) { if (kind === "c") /*writtenAfter*/v; v = 2; }
        function aliasedWritten({ kind, v }: S, w: boolean) { const ok = kind === "c"; if (w) v = 1; if (ok) /*aliasedWritten*/v; }
        function both(s: S) { const { kind, v } = s; if (kind === "c" || kind === "s") /*both*/v; }
        function sibling(w: { a: S; b: "x" | "y" }) { const { a: { kind, v }, b } = w; if (b === "x") /*sibling*/v; }
        function innerSingle(w: W) { const { kind, a: { v } } = w; if (kind === "c") /*innerSingle*/v; }
        function alias(s: S) { const { kind, v } = s; const k = kind; if (k === "c") /*alias*/v; }
        function arrayLiteral(s: S) { const [a, b] = [s.kind, s.v] as const; if (a === "c") /*arrayLiteral*/b; }
        function alwaysTruthy(s: S) { const { kind, v } = s; if (kind) /*alwaysTruthy*/v; }
        function typeofString(s: S) { const { kind, v } = s; if (typeof kind === "string") /*typeofString*/v; }
        function flag(s: S) { let ok = false; const { kind, v } = s; if (kind === "c") ok = true; if (ok) /*flag*/v; }
        function single(s: S) { const { kind } = s; const { v } = s; if (kind === "c") /*single*/v; }
        function notUnion(o: { a: string | null; b: number | string }) { const { a, b } = o; if (a) /*notUnion*/b; }
        "#,
        &[
            ("mutable", "v", &["number", "string"]),
            ("readDefault", "v", &["number", "string"]),
            ("testedDefault", "v", &["number", "string"]),
            // Biome types the rest of a union as an empty object, with or
            // without the test, where TypeScript gives a union.
            ("restElement", "rest", &["[]"]),
            ("notDiscriminant", "kind", &["string: c", "string: s"]),
            ("written", "v", &["number", "string"]),
            ("writtenInClosure", "v", &["number", "string"]),
            ("writtenAfter", "v", &["number", "string"]),
            ("aliasedWritten", "v", &["number", "string"]),
            ("both", "v", &["number", "string"]),
            ("sibling", "v", &["number", "string"]),
            ("innerSingle", "v", &["number", "string"]),
            ("alias", "v", &["number", "string"]),
            ("arrayLiteral", "b", &["number", "string"]),
            ("alwaysTruthy", "v", &["number", "string"]),
            ("typeofString", "v", &["number", "string"]),
            ("flag", "v", &["number", "string"]),
            ("single", "v", &["number", "string"]),
            ("notUnion", "b", &["number", "string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `number` at every read below. A
// test in a closure narrows the pattern of the enclosing function there, as
// in TypeScript, so `closure` gives `number`. Biome keeps the declared type
// at `loose` and `enclosingDefault`, because loose `==` with a literal
// doesn't narrow an object, and an element under a default of an
// enclosing pattern never narrows because its declared type ignores the
// default. Biome also gives the declared type `unknown`, which no test
// narrows, to an element destructured from `s!`, a parameter destructured in
// a function that a variable annotation types, and the parameters of a
// function typed by a rest parameter whose type is a union of tuples.
#[test]
fn destructured_elements_that_biome_does_not_narrow_yet() {
    assert_element_reads(
        r#"
        function loose(s: S) { const { kind, v } = s; if (kind == "c") /*loose*/v; }
        function enclosingDefault(w: { a?: S }) { const { a: { kind, v } = g } = w; if (kind === "c") /*enclosingDefault*/v; }
        function closure(s: S) { const { kind, v } = s; const h = () => { if (kind === "c") /*closure*/v; }; }

        function nonNull(s: S | undefined) { const { kind, v } = s!; if (kind === "c") /*nonNull*/v; }
        const contextual: (s: S) => void = ({ kind, v }) => { if (kind === "c") /*contextual*/v; };
        const dependent: (...args: ["a", number] | ["b", string]) => void = (k, x) => { if (k === "a") /*dependent*/x; };
        "#,
        &[
            ("loose", "v", &["number", "string"]),
            ("enclosingDefault", "v", &["number", "string"]),
            ("closure", "v", &["number"]),
            ("nonNull", "v", &["unknown"]),
            ("contextual", "v", &["unknown"]),
            ("dependent", "x", &["unknown"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives `number` at both reads, although
// a default or a computed key between the two elements can run code that
// changes `s` after `kind` is read and before `v` is, so `v` can be a string at
// the read.
// Biome narrows the same way, as `NARROWING.md` records.
#[test]
fn code_between_destructured_elements_keeps_the_narrowing() {
    assert_element_reads(
        r#"
        type SX = { kind: "c"; x?: number; v: number } | { kind: "s"; x?: number; v: string };
        declare function mutate(): number;
        declare function key(): "x";
        function between(s: SX) { const { kind, x = mutate(), v } = s; if (kind === "c") /*between*/v; }
        function computedBetween(s: SX) { const { kind, [key()]: x, v } = s; if (kind === "c") /*computedBetween*/v; }
        "#,
        &[
            ("between", "v", &["number"]),
            ("computedBetween", "v", &["number"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) reports the comparison and gives `never`
// at the read. No member is left, so Biome reads the element as Unknown.
#[test]
fn impossible_tests_of_destructured_elements_read_as_unknown() {
    let source = format!(
        "{DESTRUCTURED}{}",
        r#"
        function impossible(s: S) { const { kind, v } = s; if (kind === "c" && kind === "s") /*impossible*/v; }
        "#
    );
    let (db, module) = narrowing_db(&source);
    let ty = normalized_type_at(&db, module, &source, "impossible", "v");
    assert_eq!(
        ty,
        InferredTypeData::Unknown,
        "{}",
        format_inferred_type(&db, ty)
    );
}

// TypeScript 7.0.2 (`tsc --strict`) keeps `b` as declared in the first two
// sources, whose values are not unions, and gives `1` in the third, because a
// test of an element narrows only a pattern whose value is a union.
// Collection drops the read of `b` when the annotation shows that the value
// is not a union, without resolving anything. Otherwise the read checks the
// pattern's declared type once, and solves the pattern only when it is a
// union.
#[test]
fn destructured_elements_of_values_that_are_not_unions_skip_the_solve() {
    for (source, candidate, solves) in [
        (
            "function f({ a, b }: { a: string | null; b: number }) { if (a) { /*read*/b; } }",
            false,
            0,
        ),
        (
            "type O = { a: string | null; b: number }; function f({ a, b }: O) { if (a) { /*read*/b; } }",
            true,
            0,
        ),
        (
            r#"type S = { k: "a"; b: 1 } | { k: "b"; b: 2 }; function f({ k, b }: S) { if (k === "a") { /*read*/b; } }"#,
            true,
            1,
        ),
    ] {
        let (db, module) = narrowing_db(source);
        let range = marked_range(source, "read", "b");
        db.clear_salsa_events();
        let ty = infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range));
        let events = db.take_salsa_events();
        assert!(ty.is_some(), "{source}");
        assert_eq!(
            function_query_will_execute_count_by_name(
                &db,
                "infer_flow_expression_type_impl",
                &events
            ) > 0,
            candidate,
            "{source}"
        );
        assert_eq!(
            function_query_will_execute_count_by_name(&db, "infer_flow_subject_states", &events),
            solves,
            "{source}"
        );
    }
}

// TypeScript 7.0.2 (`tsc --strict`) gives `number` and `string`. Complete
// module inference narrows the elements from the pattern's declared type that
// its own context resolved, as lookups do, and both reads share one solve of
// the pattern.
#[test]
fn complete_module_inference_narrows_destructured_elements_as_lookups_do() {
    use InferredTypeData::{Number, String};
    const SOURCE: &str = r#"
        type S = { kind: "c"; v: number } | { kind: "s"; v: string };
        function f({ kind, v }: S) { if (kind === "c") /*number*/v; else /*string*/v; }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    let reads = [("number", Number), ("string", String)]
        .map(|(marker, expected)| (marked_range(SOURCE, marker, "v"), expected));

    db.clear_salsa_events();
    let complete = infer_module_types(&db, module).unwrap();
    for (range, expected) in reads {
        assert_eq!(complete.expressions[&range], expected);
    }
    let events = db.take_salsa_events();
    for query in [
        "infer_flow_expression_type_impl",
        "infer_flow_binding_baseline",
    ] {
        assert_eq!(
            function_query_will_execute_count_by_name(&db, query, &events),
            0,
            "{query}"
        );
    }
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "infer_flow_subject_states", &events),
        1
    );

    for (range, expected) in reads {
        assert_eq!(
            infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range)),
            Some(expected)
        );
    }
}
