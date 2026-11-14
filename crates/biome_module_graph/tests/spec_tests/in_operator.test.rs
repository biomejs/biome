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

/// Shapes shared by the tests, as interfaces, so that a read names each of
/// its members.
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
// (`narrowing-probes/m3-l5/spec.ts` L27 to L47, L72, L74, and
// `narrowing-probes/m3/in.ts` L8, L9, L13 to L15, L17 to L20, L28). Where
// every member declares the property as required, the `else` branch is
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

// TypeScript 6.0.3 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m3-l5/spec.ts` L48, L50, L51, L58, L61 to L63, L66 to L68,
// and `narrowing-probes/m3/in.ts` L11, L12, L23, L27), except where a comment
// gives TypeScript's type. Biome keeps a member when it can't tell that the
// member lacks the property.
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

// Biome keeps the declared type of every read below. A comment gives the type
// that TypeScript 6.0.3 (`tsc --strict`) gives where it differs
// (`narrowing-probes/m3-l5/spec.ts` L26, L49, L52 to L57, L59, L60, L64, L65,
// L69 to L71, L73, and `narrowing-probes/m3/in.ts` L10, L16, L21, L22, L24,
// L25).
#[test]
fn in_keeps_the_declared_type_where_biome_cannot_tell() {
    assert_reads(
        r##"
        function object(x: object) { if ("a" in x) /*object*/x; }
        function undeclared(x: A | B) { if ("c" in x) /*undeclared*/x; }
        function numberDictName(x: NumberDict | B) { if ("a" in x) /*numberDictName*/x; }
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
            // `object & Record<"a", unknown>`
            ("object", "x", &["object"]),
            // `(A | B) & Record<"c", unknown>`
            ("undeclared", "x", &["A", "B"]),
            // `(B | NumberDict) & Record<"a", unknown>`
            ("numberDictName", "x", &["B", "NumberDict"]),
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
