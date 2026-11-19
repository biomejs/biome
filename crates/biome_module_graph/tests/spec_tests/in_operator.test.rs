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
        TypeInferenceCaller::new("test", "in_operator"),
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

/// Interfaces and classes shared by the tests. Their names identify the
/// members of a read's type.
const SHAPES: &str = r#"
    interface A { a: 1 }
    interface B { b: 2 }
    interface OptA { a?: 1 }
    interface AB { a: 2; b: 3 }
    interface I1 { a: 1 }
    interface I2 extends I1 { c: 2 }
    interface Zero { 0: "a" }
    interface One { 1: "b" }
    interface Method { a(): void }
    interface UndefinedA { a: undefined }
    interface Empty {}
    interface Dict { [key: string]: number }
    interface NumberDict { [key: number]: 1 }
    interface AtLike { at(): void }
    interface CauseLike { cause: 1 }
    interface Call { (): void; a: 1 }
    class P { p = 1 }
    class Q { q = 1 }
    class S { set a(v: number) {} }
    class Idx { [key: string]: unknown }
"#;

/// How [`variant_names`] names `() => void`.
const VOID_FUNCTION: &str = "sync Function { accepts: { params: [] type_args: [] } returns: void }";

/// Asserts the type of each `(marker, expression, expected)` read by the names
/// of its members, from [`variant_names`]. The program is [`SHAPES`] followed
/// by `source`.
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
// Where every member declares the property as required, the `else` branch is
// `never` in TypeScript, which Biome reads as Unknown. `"a" in x` with an
// `undefined` member is an error in TypeScript (TS18048).
#[test]
fn in_keeps_the_members_that_declare_the_property() {
    assert_reads(
        r#"
        function required(x: A | B) { if ("a" in x) /*required*/x; else /*requiredElse*/x; }
        function optional(x: OptA | B) { if ("a" in x) /*optional*/x; else /*optionalElse*/x; }
        function inherited(x: I2 | B) { if ("a" in x) /*inherited*/x; else /*inheritedElse*/x; }
        function integer(x: Zero | One) { if (0 in x) /*integer*/x; else /*integerElse*/x; }
        function integerString(x: Zero | One) { if ("0" in x) /*integerString*/x; }
        function path(o: { v: A | B }) { if ("a" in o.v) /*path*/o.v; else /*pathElse*/o.v; }
        function method(x: Method | B) { if ("a" in x) /*method*/x; else /*methodElse*/x; }
        function negated(x: A | B) { if (!("a" in x)) /*negated*/x; }
        function both(x: A | B | AB) { if ("a" in x && "b" in x) /*both*/x; }
        function undefinedProperty(x: UndefinedA | B) { if ("a" in x) /*undefinedProperty*/x; }
        function nullish(x: A | B | undefined) { if ("a" in x) /*nullish*/x; else /*nullishElse*/x; }
        function empty(x: Empty | A) { if ("a" in x) /*empty*/x; else /*emptyElse*/x; }
        function objectMember(x: A | B) { if ("toString" in x) /*objectMember*/x; else /*objectMemberElse*/x; }
        function indexedObjectMember(x: A | B | Dict) { if ("toString" in x) /*indexed*/x; else /*indexedElse*/x; }
        function parenthesized(x: A | B) { if (("a") in (x)) /*parenthesized*/x; }
        function property(x: A | B) { if ("a" in x) /*property*/x.a; }
        function operand(x: A | B) { "a" in x && /*operand*/x; }
        function conditional(x: A | B) { "a" in x ? /*conditional*/x : x; }
        function loop(x: A | B) { while ("a" in x) { /*loop*/x; } /*loopExit*/x; }
        function afterTruthy(x: A | B | null) { if (x && "a" in x) /*afterTruthy*/x; else /*afterTruthyElse*/x; }
        function aliased(x: A | B) { const ok = "a" in x; if (ok) /*aliased*/x; else /*aliasedElse*/x; }
        function element(w: { a: A | B; b: 1 }) { const { a, b } = w; if ("a" in a) /*element*/a; }
        interface M { a: 1 }
        interface M { m: 2 }
        interface Mm { m: 3 }
        function merged(x: M | Mm) { if ("m" in x) /*merged*/x; else /*mergedElse*/x; }
        "#,
        &[
            ("required", "x", &["A"]),
            ("requiredElse", "x", &["B"]),
            ("optional", "x", &["OptA"]),
            ("optionalElse", "x", &["B", "OptA"]),
            ("inherited", "x", &["I2"]),
            ("inheritedElse", "x", &["B"]),
            ("integer", "x", &["Zero"]),
            ("integerElse", "x", &["One"]),
            ("integerString", "x", &["Zero"]),
            ("path", "o.v", &["A"]),
            ("pathElse", "o.v", &["B"]),
            ("method", "x", &["Method"]),
            ("methodElse", "x", &["B"]),
            ("negated", "x", &["B"]),
            ("both", "x", &["AB"]),
            ("undefinedProperty", "x", &["UndefinedA"]),
            ("nullish", "x", &["A"]),
            ("nullishElse", "x", &["B", "undefined"]),
            ("empty", "x", &["A"]),
            ("emptyElse", "x", &["Empty"]),
            ("objectMember", "x", &["A", "B"]),
            ("objectMemberElse", "x", &["unknown"]),
            ("indexed", "x", &["A", "B", "Dict"]),
            ("indexedElse", "x", &["unknown"]),
            ("parenthesized", "x", &["A"]),
            ("property", "x.a", &["number: 1"]),
            ("operand", "x", &["A"]),
            ("conditional", "x", &["A"]),
            ("loop", "x", &["A"]),
            ("loopExit", "x", &["B"]),
            ("afterTruthy", "x", &["A"]),
            ("afterTruthyElse", "x", &["B", "null"]),
            ("aliased", "x", &["A"]),
            ("aliasedElse", "x", &["B"]),
            ("element", "a", &["A"]),
            ("merged", "x", &["M", "Mm"]),
            ("mergedElse", "x", &["unknown"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) merges the interfaces of one name in one
// scope into one type, and a class with them, and gives every read below the
// expected type. Where every member declares the property as required, the
// `else` branch is `never` in TypeScript, which Biome reads as Unknown.
#[test]
fn in_sees_the_members_of_every_merged_declaration() {
    assert_reads(
        r#"
        interface X2 { x: 1 }
        interface X2 { z: 2 }
        interface Xx { x: 3 }
        function mergedIn(q: X2 | Xx) { if ("x" in q) /*mergedIn*/q; else /*mergedInElse*/q; }
        interface Y1 { y: 1 } interface Y1 { z: 2 } interface Yy { y: 3 }
        function last(q: Y1 | Yy) { if ("z" in q) /*last*/q; else /*lastElse*/q; }
        function first(q: Y1 | Yy) { if ("y" in q) /*first*/q; else /*firstElse*/q; }
        class Kj { k = 1 } interface Kj { j: 1 }
        function classMember(q: Kj | A) { if ("j" in q) /*classMember*/q; else /*classMemberElse*/q; }
        interface Jk { j: 1 } class Jk { k = 1 }
        function interfaceFirst(q: Jk | A) { if ("j" in q) /*interfaceFirst*/q; else /*interfaceFirstElse*/q; }
        interface Ext extends A {} interface Ext { e: 1 }
        function inherited(q: Ext | Xx) { if ("a" in q) /*inherited*/q; else /*inheritedElse*/q; }
        "#,
        &[
            ("mergedIn", "q", &["X2", "Xx"]),
            ("mergedInElse", "q", &["unknown"]),
            ("last", "q", &["Y1"]),
            ("lastElse", "q", &["Yy"]),
            ("first", "q", &["Y1", "Yy"]),
            ("firstElse", "q", &["unknown"]),
            ("classMember", "q", &["instanceof Kj"]),
            ("classMemberElse", "q", &["A"]),
            ("interfaceFirst", "q", &["instanceof Jk"]),
            ("interfaceFirstElse", "q", &["A"]),
            ("inherited", "q", &["Ext"]),
            ("inheritedElse", "q", &["Xx"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// except where a comment gives TypeScript's type. Biome keeps a member when it
// can't tell that the member lacks the property.
#[test]
fn in_keeps_the_members_that_may_have_the_property() {
    assert_reads(
        r#"
        function dict(x: Dict | B) { if ("a" in x) /*dict*/x; else /*dictElse*/x; }
        function numberDict(x: NumberDict | B) { if (0 in x) /*numberDict*/x; else /*numberDictElse*/x; }
        function classes(x: P | Q) { if ("p" in x) /*classes*/x; else /*classesElse*/x; }
        function functions(x: A | (() => void)) { if ("a" in x) /*functions*/x; else /*functionsElse*/x; }
        function date(x: A | Date) { if ("a" in x) /*date*/x; else /*dateElse*/x; }
        function newerArray(x: string[] | AtLike) { if ("at" in x) /*newerArray*/x; else /*newerArrayElse*/x; }
        function newerError(x: Error | CauseLike) { if ("cause" in x) /*newerError*/x; else /*newerErrorElse*/x; }
        function primitive(x: string | A | B) { if ("a" in x) /*primitive*/x; else /*primitiveElse*/x; }
        function setter(x: S | A) { if ("a" in x) /*setter*/x; else /*setterElse*/x; }
        function classIndex(x: Idx | A) { if ("a" in x) /*classIndex*/x; else /*classIndexElse*/x; }
        "#,
        &[
            ("dict", "x", &["Dict"]),
            ("dictElse", "x", &["B", "Dict"]),
            ("numberDict", "x", &["NumberDict"]),
            ("numberDictElse", "x", &["B", "NumberDict"]),
            // TypeScript gives `P`.
            ("classes", "x", &["instanceof P", "instanceof Q"]),
            ("classesElse", "x", &["instanceof Q"]),
            // TypeScript gives `A`.
            ("functions", "x", &["A", VOID_FUNCTION]),
            ("functionsElse", "x", &[VOID_FUNCTION]),
            // TypeScript gives `A`.
            ("date", "x", &["A", "instanceof Date"]),
            ("dateElse", "x", &["instanceof Date"]),
            ("newerArray", "x", &["AtLike", "instanceof Array<string>"]),
            // TypeScript gives `never`.
            ("newerArrayElse", "x", &["instanceof Array<string>"]),
            ("newerError", "x", &["CauseLike", "instanceof Error"]),
            ("newerErrorElse", "x", &["instanceof Error"]),
            // TypeScript gives `A`, and reports TS2322.
            ("primitive", "x", &["A", "string"]),
            ("primitiveElse", "x", &["B", "string"]),
            ("setter", "x", &["A", "instanceof S"]),
            // TypeScript gives `never`.
            ("setterElse", "x", &["instanceof S"]),
            ("classIndex", "x", &["A", "instanceof Idx"]),
            ("classIndexElse", "x", &["instanceof Idx"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives each read the type in its comment.
// When every member lacks the property, the true branch is the intersection
// with `Record<"a", unknown>`, which Biome writes as an object type with the
// property `a: unknown`, and normalization merges into the object types of
// the members. The `else` branch keeps the type.
#[test]
fn in_intersects_the_type_when_no_member_has_the_property() {
    assert_reads(
        r##"
        function object(x: object) { if ("a" in x) /*object*/x; else /*objectElse*/x; } // object & Record<"a", unknown>; object
        function undeclared(x: A | B) { if ("c" in x) /*undeclared*/x; else /*undeclaredElse*/x; } // (A | B) & Record<"c", unknown>; A | B
        function numberDictName(x: NumberDict | B) { if ("a" in x) /*numberDictName*/x; } // (B | NumberDict) & Record<"a", unknown>
        function single(x: B) { if ("a" in x) /*single*/x; } // B & Record<"a", unknown>
        function prototypeMember(x: object) { if ("toString" in x) /*prototypeMember*/x; } // object
        "##,
        &[
            (
                "object",
                "x",
                &[r#"object & Object { prototype: No prototype members: ["a": unknown] }"#],
            ),
            ("objectElse", "x", &["object"]),
            (
                "undeclared",
                "x",
                &[
                    r#"["a": number: 1, "c": unknown]"#,
                    r#"["b": number: 2, "c": unknown]"#,
                ],
            ),
            ("undeclaredElse", "x", &["A", "B"]),
            (
                "numberDictName",
                "x",
                &[
                    r#"["b": number: 2, "a": unknown]"#,
                    r#"[[number]: number: 1, "a": unknown]"#,
                ],
            ),
            ("single", "x", &[r#"["b": number: 2, "a": unknown]"#]),
            ("prototypeMember", "x", &["object"]),
        ],
    );
}

// Biome keeps the declared type of every read below. A comment gives the type
// that TypeScript 7.0.2 (`tsc --strict`) gives where it differs.
#[test]
fn in_keeps_the_declared_type_where_biome_cannot_tell() {
    assert_reads(
        r##"
        function constKey(x: A | B) { const k = "a"; if (k in x) /*constKey*/x; }
        function template(x: A | B) { if (`a` in x) /*template*/x; }
        function escapedKey(x: A | B) { if ("\u0061" in x) /*escapedKey*/x; }
        function hex(x: A | B) { if (0x0 in x) /*hex*/x; }
        function functionMember(x: A | (() => void)) { if ("call" in x) /*functionMember*/x; else /*functionMemberElse*/x; }
        function tuples(x: [1] | []) { if (0 in x) /*tuples*/x; else /*tuplesElse*/x; }
        function escapedMember(x: { "\u0061": 1 } | B) { if ("a" in x) /*escapedMember*/x; }
        function numberMember(x: { 1.0: "a" } | B) { if (1 in x) /*numberMember*/x; }
        function callable(x: Call | B) { if ("call" in x) /*callable*/x; }
        function intersection(x: (A & { c: 3 }) | B) { if ("a" in x) /*intersection*/x; }
        function hashMember(x: { "#a": 1 } | B) { if ("#a" in x) /*hashMember*/x; }
        class K { #a = 1; m(x: K | B) { if ("#a" in x) /*privateName*/x; else /*privateNameElse*/x; } }
        function discriminant(s: { k: A | B; t: 1 } | { k: B; t: 2 }) { if ("a" in s.k) /*discriminant*/s; }
        "##,
        &[
            // `A`
            ("constKey", "x", &["A", "B"]),
            // `A`
            ("template", "x", &["A", "B"]),
            // `A`
            ("escapedKey", "x", &["A", "B"]),
            // `(A | B) & Record<0, unknown>`
            ("hex", "x", &["A", "B"]),
            // `() => void`, and `A` in the `else` branch
            ("functionMember", "x", &["A", VOID_FUNCTION]),
            ("functionMemberElse", "x", &["A", VOID_FUNCTION]),
            ("tuples", "x", &["[]", "[number: 1]"]),
            // `[]`
            ("tuplesElse", "x", &["[]", "[number: 1]"]),
            // `{ a: 1 }`
            ("escapedMember", "x", &["B", r#"["\u0061": number: 1]"#]),
            // `{ 1: "a" }`
            ("numberMember", "x", &["B", r#"["1.0": string: a]"#]),
            // `Call`
            ("callable", "x", &["B", "Call"]),
            // `A & { c: 3 }`
            (
                "intersection",
                "x",
                &["B", r#"["a": number: 1, "c": number: 3]"#],
            ),
            // `{ "#a": 1 }`
            ("hashMember", "x", &["B", r##"["#a": number: 1]"##]),
            // `(B | K) & Record<"#a", unknown>`
            ("privateName", "x", &["B", "instanceof K"]),
            ("privateNameElse", "x", &["B", "instanceof K"]),
            // `in` narrows no object that holds the tested value, as in
            // TypeScript.
            (
                "discriminant",
                "s",
                &[
                    r#"[ "k": interface "A" { extends: [] type_args: [] members: ["a": number: 1] } | interface "B" { extends: [] type_args: [] members: ["b": number: 2] } , "t": number: 1 ]"#,
                    r#"[ "k": interface "B" { extends: [] type_args: [] members: ["b": number: 2] } , "t": number: 2 ]"#,
                ],
            ),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// except where a comment gives TypeScript's type. Biome keeps an array, and
// an object type with a setter, a spread, or a computed key, in both
// branches, because it can't tell whether the member has the property. It
// lists none of the members of the setter's type and of the spread object.
#[test]
fn in_keeps_the_members_whose_properties_biome_cannot_list() {
    let computed = r#"[computed [string: a]: number: 1]"#;
    assert_reads(
        r#"
        function array(x: string[] | A) { if ("a" in x) /*array*/x; else /*arrayElse*/x; }
        function setterType(x: { set a(v: number) } | B) { if ("a" in x) /*setter*/x; else /*setterElse*/x; }
        const base = { a: 1 as const };
        const spread = { ...base };
        function spreadType(x: typeof spread | B) { if ("a" in x) /*spread*/x; else /*spreadElse*/x; }
        const key = "a";
        const computed = { [key]: 1 as const };
        function computedType(x: typeof computed | B) { if ("a" in x) /*computed*/x; else /*computedElse*/x; }
        "#,
        &[
            // TypeScript: `A`.
            ("array", "x", &["A", "instanceof Array<string>"]),
            ("arrayElse", "x", &["instanceof Array<string>"]),
            // TypeScript: `{ a: number }`.
            ("setter", "x", &["B", "[]"]),
            // TypeScript: `B`.
            ("setterElse", "x", &["B", "[]"]),
            // TypeScript: `{ a: 1 }`.
            ("spread", "x", &["B", "[]"]),
            // TypeScript: `B`.
            ("spreadElse", "x", &["B", "[]"]),
            // TypeScript: `{ a: 1 }`.
            ("computed", "x", &["B", computed]),
            // TypeScript: `B`.
            ("computedElse", "x", &["B", computed]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) merges the interfaces of one name in two
// blocks of a namespace, so `N.I` has `a` and `b`. Biome takes `N.I` as the
// first of them, which lacks `b`, so `"b" in x` intersects both members with
// an object type that has `b`, and `i.b` is Unknown.
#[test]
fn interfaces_in_two_blocks_of_a_namespace_dont_merge() {
    assert_reads(
        r#"
        namespace N { export interface I { a: 1 } }
        namespace N { export interface I { b: 2 } }
        function f(x: N.I | { c: 3 }) { if ("b" in x) /*merged*/x; else /*mergedElse*/x; }
        declare const i: N.I;
        /*member*/i.b;
        "#,
        &[
            // TypeScript: `N.I`.
            (
                "merged",
                "x",
                &[
                    r#"["a": number: 1, "b": unknown]"#,
                    r#"["c": number: 3, "b": unknown]"#,
                ],
            ),
            // TypeScript: `{ c: 3 }`.
            ("mergedElse", "x", &["I", r#"["c": number: 3]"#]),
            // TypeScript: `2`.
            ("member", "i.b", &["unknown"]),
        ],
    );
}
