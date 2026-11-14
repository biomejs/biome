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
        TypeInferenceCaller::new("test", "propertyNarrowing"),
        NormalizedExpressionTypeRequest::new(module, range),
    )
    .unwrap_or_else(|| panic!("expression at {marker} {range:?} must be inferred"))
}

fn assert_variants<'db>(
    db: &'db TestModuleDb,
    ty: InferredTypeData<'db>,
    expected: &[InferredTypeData<'db>],
    marker: &str,
) {
    let actual = match ty {
        InferredTypeData::Union(union) => union.types(db).to_vec(),
        _ => vec![ty],
    };
    assert!(
        actual.len() == expected.len() && expected.iter().all(|ty| actual.contains(ty)),
        "{marker}: expected {expected:?}, got {}",
        format_inferred_type(db, ty)
    );
}

/// Asserts the type of each `(marker, expression, expected)` read in `source`.
fn assert_reads(source: &str, reads: &[(&str, &str, &[InferredTypeData<'static>])]) {
    let (db, module) = narrowing_db(source);
    for (marker, expression, expected) in reads {
        let ty = normalized_type_at(&db, module, source, marker, expression);
        assert_variants(&db, ty, expected, marker);
    }
}

// TypeScript 6.0.3 (`tsc --strict`) gives every read below the expected type.
#[test]
fn property_paths_narrow_like_variables() {
    use InferredTypeData::{Null, Number, String};
    const SOURCE: &str = r#"
        function truthy(o: { v: string | null }) {
            if (o.v) { /*truthy*/o.v; }
        }
        function notNull(o: { v: string | null }) {
            if (o.v !== null) { /*notNull*/o.v; }
        }
        function typeOf(o: { v: string | null }) {
            if (typeof o.v === "string") { /*typeof*/o.v; }
        }
        function nested(a: { b: { c: string | null } }) {
            if (a.b.c) { /*nested*/a.b.c; }
        }
        function earlyReturn(o: { v: string | null }) {
            if (o.v) {} else { return; }
            /*early*/o.v;
        }
        function branches(o: { v: string | number | null }) {
            if (o.v !== null) {
                if (typeof o.v === "string") { /*string*/o.v; } else { /*number*/o.v; }
            }
        }
        function nullish(o: { v: string | null }) {
            o.v ?? /*nullish*/o.v;
        }
        function joined(o: { v: string | null }) {
            if (o.v !== null) { /*inside*/o.v; return; }
            /*after*/o.v;
        }
        function spelling(o: { v: string | null }, x: { y: string | null } | undefined) {
            if (o.v) { /*parenthesized*/(o).v; /*optional*/o?.v; }
            if (o.\u0076) { /*escaped*/o.v; }
            if (x?.y) { /*chain*/x?.y; }
        }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("truthy", "o.v", &[String]),
            ("notNull", "o.v", &[String]),
            ("typeof", "o.v", &[String]),
            ("nested", "a.b.c", &[String]),
            ("early", "o.v", &[String]),
            ("string", "o.v", &[String]),
            ("number", "o.v", &[Number]),
            ("nullish", "o.v", &[Null]),
            ("inside", "o.v", &[String]),
            ("after", "o.v", &[Null]),
            ("parenthesized", "(o).v", &[String]),
            ("optional", "o?.v", &[String]),
            ("escaped", "o.v", &[String]),
            ("chain", "x?.y", &[String]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) gives every read below the expected type:
// a path read starts from the property's type on the object's narrowed type.
#[test]
fn path_reads_start_from_the_narrowed_object() {
    use InferredTypeData::{Number, String};
    const SOURCE: &str = r#"
        type A = { v: string | null };
        type B = { v: number };
        function nullable(o: { v: string | null } | null) {
            if (o && o.v) { /*nullable*/o.v; }
        }
        function union(o: A | B | null) {
            if (o && o.v) { /*union*/o.v; }
        }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("nullable", "o.v", &[String]),
            ("union", "o.v", &[String, Number]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) narrows each read below to `string`: a call
// that may write `o.v`, a write through another variable, a write inside a
// callback, and a write to a longer path never cancel property narrowing.
// Biome copies this unsound narrowing; `NARROWING.md` lists it among the
// deliberate choices.
#[test]
fn calls_aliases_and_callbacks_keep_property_narrowing() {
    use InferredTypeData::String;
    const SOURCE: &str = r#"
        function alias(o: { v: string | null }) {
            if (o.v) { const a = o; a.v = null; /*alias*/o.v; }
        }
        function closureCall(o: { v: string | null }) {
            const reset = () => { o.v = null; };
            if (o.v) { reset(); /*closure*/o.v; }
        }
        function callback(o: { v: string | null }) {
            if (o.v) { [1].forEach(() => { o.v = null; }); /*callback*/o.v; }
        }
        function method(o: { v: string | null }) {
            if (o.v) { class C { m() { o.v = null; } } /*method*/o.v; }
        }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("alias", "o.v", &[String]),
            ("closure", "o.v", &[String]),
            ("callback", "o.v", &[String]),
            ("method", "o.v", &[String]),
        ],
    );

    // TypeScript gives `{ w: string | null }`: writing `o.v.w` leaves `o.v`.
    const EXTENSION: &str = r#"
        function extension(o: { v: { w: string | null } | null }) {
            /*declared*/o.v;
            if (o.v !== null) { o.v.w = null; /*extension*/o.v; }
        }
    "#;
    let (db, module) = narrowing_db(EXTENSION);
    let declared = normalized_type_at(&db, module, EXTENSION, "declared", "o.v");
    assert!(contains_inferred_null(&db, declared));
    let narrowed = normalized_type_at(&db, module, EXTENSION, "extension", "o.v");
    assert_ne!(narrowed, InferredTypeData::Unknown);
    assert!(
        !contains_inferred_null(&db, narrowed),
        "{}",
        format_inferred_type(&db, narrowed)
    );
}

// TypeScript 6.0.3 (`tsc --strict`) narrows the read below to `string`,
// although a getter can return `null` the second time it runs. Biome copies
// this unsound narrowing; `NARROWING.md` lists it among the deliberate choices.
#[test]
fn getters_narrow_like_fields() {
    use InferredTypeData::String;
    const SOURCE: &str = r#"
        class G {
            get v(): string | null { return Math.random() ? "a" : null; }
        }
        function classGetter(g: G) {
            if (g.v) { /*class*/g.v; }
        }
    "#;
    assert_reads(SOURCE, &[("class", "g.v", &[String])]);
}

// Each expected type is `string | null` (or `number | null`, or
// `string | undefined`). TypeScript 6.0.3 (`tsc --strict`) gives the type in
// the comment after each read. Where it differs, TypeScript narrows the path
// at the write itself, which Biome does not model yet: a write anywhere in the
// function stops the path from narrowing, so Biome is deliberately wider. A
// write through a key that is not a literal (`unionKey`) also stops it, where
// TypeScript ignores the write; `NARROWING.md` lists this among the deliberate
// choices, and so does the `async` function called where it is defined.
#[test]
fn writes_to_the_path_a_prefix_or_any_property_cancel_narrowing() {
    use InferredTypeData::{Null, Number, String, Undefined};
    const SOURCE: &str = r#"
        declare function get(): string | null;
        function assigned(o: { v: string | null }) {
            if (o.v) { o.v = get(); /*assigned*/o.v; } // string | null
        }
        function literal(o: { v: string | null }) {
            if (o.v) { o.v = "a"; /*literal*/o.v; } // string
        }
        function updated(o: { v: number | null }) {
            if (o.v) { o.v++; /*updated*/o.v; } // number
        }
        function deleted(o: { v?: string }) {
            if (o.v) { delete o.v; /*deleted*/o.v; } // undefined
        }
        function prefix(o: { v: { w: string | null } }, q: { w: string | null }) {
            if (o.v.w) { o.v = q; /*prefix*/o.v.w; } // string | null
        }
        function variable(o: { v: string | null }, q: { v: string | null }) {
            if (o.v) { o = q; /*variable*/o.v; } // string | null
        }
        function arrowIife(o: { v: string | null }) {
            if (o.v) { (() => { o.v = null; })(); /*arrowIife*/o.v; } // null
        }
        function functionIife(o: { v: string | null }) {
            if (o.v) { (function () { o.v = null; })(); /*functionIife*/o.v; } // null
        }
        function asyncIife(o: { v: string | null }) {
            if (o.v) { (async () => { o.v = null; })(); /*asyncIife*/o.v; } // string
        }
        function staticBlock(o: { v: string | null }) {
            if (o.v) { class C { static { o.v = null; } } /*staticBlock*/o.v; } // null
        }
        function constKey(o: { v: string | null; w: string | null }) {
            const k = "v";
            if (o.v) { o[k] = null; /*constKey*/o.v; } // null
        }
        function unionKey(o: { v: string | null; w: string | null }, k: "v" | "w") {
            if (o.v) { o[k] = null; /*unionKey*/o.v; } // string
        }
        function laterWrite(o: { v: string | null }) {
            if (o.v) { /*beforeWrite*/o.v; }
            o.v = null;
        }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("assigned", "o.v", &[String, Null]),
            ("literal", "o.v", &[String, Null]),
            ("updated", "o.v", &[Number, Null]),
            ("deleted", "o.v", &[String, Undefined]),
            ("prefix", "o.v.w", &[String, Null]),
            ("variable", "o.v", &[String, Null]),
            ("arrowIife", "o.v", &[String, Null]),
            ("functionIife", "o.v", &[String, Null]),
            ("asyncIife", "o.v", &[String, Null]),
            ("staticBlock", "o.v", &[String, Null]),
            ("constKey", "o.v", &[String, Null]),
            ("unionKey", "o.v", &[String, Null]),
            // TypeScript narrows this read to `string`; the write after it
            // still stops Biome from narrowing the path in the whole function.
            ("beforeWrite", "o.v", &[String, Null]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) gives `string | null` at `arrow` and
// `declaration`: property narrowing never reaches a function created later.
// It gives `string` at `iife` and `enclosing`, where Biome is wider: the body
// of a function called where it is defined does not continue the outer flow
// yet, and a path narrows only in the execution root that declares its
// variable.
#[test]
fn reads_in_nested_functions_see_no_property_facts() {
    use InferredTypeData::{Null, String};
    const SOURCE: &str = r#"
        function f(o: { v: string | null }) {
            if (o.v) {
                const g = () => /*arrow*/o.v;
                function h() { return /*declaration*/o.v; }
                (() => /*iife*/o.v)();
            }
            const k = function () { if (o.v) { /*enclosing*/o.v; } };
        }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("arrow", "o.v", &[String, Null]),
            ("declaration", "o.v", &[String, Null]),
            ("iife", "o.v", &[String, Null]),
            ("enclosing", "o.v", &[String, Null]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) gives every read below the expected type.
#[test]
fn type_guards_and_assertions_narrow_properties() {
    use InferredTypeData::{Number, String};
    const SOURCE: &str = r#"
        function isString(v: unknown): v is string { return typeof v === "string"; }
        function assertString(v: unknown): asserts v is string {}
        function assert(c: unknown): asserts c {}
        function guard(o: { v: string | number }) {
            if (isString(o.v)) { /*guard*/o.v; } else { /*guardElse*/o.v; }
        }
        function assertType(o: { v: string | number }) {
            assertString(o.v);
            /*assertType*/o.v;
        }
        function assertCondition(o: { v: string | null }) {
            assert(o.v !== null);
            /*assertCondition*/o.v;
        }
        function assertTruthy(o: { v: string | null }) {
            assert(o.v);
            /*assertTruthy*/o.v;
        }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("guard", "o.v", &[String]),
            ("guardElse", "o.v", &[Number]),
            ("assertType", "o.v", &[String]),
            ("assertCondition", "o.v", &[String]),
            ("assertTruthy", "o.v", &[String]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) gives every call below the expected type:
// a narrowed method is called with the narrowed type.
#[test]
fn narrowed_callees_resolve_their_calls() {
    use InferredTypeData::{Number, String};
    const SOURCE: &str = r#"
        class Box<T> { get?: () => T; }
        function optional(o: { f?: () => string; g(): number }) {
            if (o.f) { /*optional*/o.f(); }
            if (o.g) { /*method*/o.g(); }
        }
        function generic(b: Box<string>) {
            if (b.get) { /*generic*/b.get(); }
        }
        function nullable(o: { f(): string } | null) {
            if (o && o.f) { /*nullable*/o.f(); }
        }
        function chained(o: { v: { n: number } | null }) {
            if (o.v) { /*chained*/o.v.n; }
        }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("optional", "o.f()", &[String]),
            ("method", "o.g()", &[Number]),
            ("generic", "b.get()", &[String]),
            ("nullable", "o.f()", &[String]),
            ("chained", "o.v.n", &[Number]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) gives every read below the expected type:
// an element access with a string or integer literal key reads the same
// property as a named access, so `o["a"]` and `o.a` share their narrowing, and
// so do `t[0]` and `t["0"]`.
#[test]
fn literal_keys_read_the_same_property_as_names() {
    use InferredTypeData::{Null, Number, String};
    const SOURCE: &str = r#"
        function isString(v: unknown): v is string { return typeof v === "string"; }
        function names(o: { a: string | null }) {
            if (o["a"]) { /*element*/o["a"]; /*fromElement*/o.a; }
            if (o.a) { /*fromName*/o["a"]; }
            if (o['a']) { /*quotes*/o["a"]; }
            if (o?.["a"]) { /*optional*/o.a; }
            if (o["a"] !== null) { /*notNull*/o["a"]; } else { /*null*/o["a"]; }
            o["a"] ?? /*nullish*/o["a"];
        }
        function tests(o: { a: string | number }) {
            if (typeof o["a"] === "string") { /*typeof*/o["a"]; }
            if (isString(o["a"])) { /*guard*/o["a"]; } else { /*guardElse*/o.a; }
        }
        function nested(o: { a: { b: string | null } }) {
            if (o["a"]["b"]) { /*nested*/o.a.b; /*mixed*/o["a"].b; }
        }
        function tuple(t: [string | null, number]) {
            if (t[0]) { /*index*/t[0]; }
            if (t["0"]) { /*fromString*/t[0]; }
        }
        function array(a: (string | null)[]) {
            if (a[0]) { /*array*/a[0]; }
        }
        function record(r: Record<string, string | null>) {
            if (r[1]) { /*fromNumber*/r["1"]; }
        }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("element", r#"o["a"]"#, &[String]),
            ("fromElement", "o.a", &[String]),
            ("fromName", r#"o["a"]"#, &[String]),
            ("quotes", r#"o["a"]"#, &[String]),
            ("optional", "o.a", &[String]),
            ("notNull", r#"o["a"]"#, &[String]),
            ("null", r#"o["a"]"#, &[Null]),
            ("nullish", r#"o["a"]"#, &[Null]),
            ("typeof", r#"o["a"]"#, &[String]),
            ("guard", r#"o["a"]"#, &[String]),
            ("guardElse", "o.a", &[Number]),
            ("nested", "o.a.b", &[String]),
            ("mixed", r#"o["a"].b"#, &[String]),
            ("index", "t[0]", &[String]),
            ("fromString", "t[0]", &[String]),
            ("array", "a[0]", &[String]),
            ("fromNumber", r#"r["1"]"#, &[String]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) narrows every read below to `string`: a
// write through a literal key writes only the property it names, and a call
// such as `a.shift()` never cancels narrowing.
#[test]
fn literal_key_writes_keep_other_properties() {
    use InferredTypeData::String;
    const SOURCE: &str = r#"
        function otherKey(o: { a: string | null; b: string | null }) {
            if (o.a) { o["b"] = null; /*otherKey*/o.a; }
        }
        function otherName(o: { a: string | null; b: string | null }) {
            if (o["a"]) { o.b = null; /*otherName*/o["a"]; }
        }
        function otherIndex(t: [string | null, string | null]) {
            if (t[0]) { t[1] = null; /*otherIndex*/t[0]; }
        }
        function shift(a: (string | null)[]) {
            if (a[0]) { a.shift(); /*shift*/a[0]; }
        }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("otherKey", "o.a", &[String]),
            ("otherName", r#"o["a"]"#, &[String]),
            ("otherIndex", "t[0]", &[String]),
            ("shift", "a[0]", &[String]),
        ],
    );

    // TypeScript gives `{ b: string | null }`: writing `o["a"]["b"]` leaves
    // `o["a"]`.
    const EXTENSION: &str = r#"
        function extension(o: { a: { b: string | null } | null }) {
            /*declared*/o["a"];
            if (o.a !== null) { o["a"]["b"] = null; /*extension*/o["a"]; }
        }
    "#;
    let (db, module) = narrowing_db(EXTENSION);
    let declared = normalized_type_at(&db, module, EXTENSION, "declared", r#"o["a"]"#);
    assert!(contains_inferred_null(&db, declared));
    let narrowed = normalized_type_at(&db, module, EXTENSION, "extension", r#"o["a"]"#);
    assert_ne!(narrowed, InferredTypeData::Unknown);
    assert!(
        !contains_inferred_null(&db, narrowed),
        "{}",
        format_inferred_type(&db, narrowed)
    );
}

// Each expected type keeps `null` or `undefined`. TypeScript 6.0.3
// (`tsc --strict`) gives the type in the comment after each read. Where it
// differs, TypeScript narrows the path at the write itself, which Biome does
// not model yet, or ignores the write: a write through a key that is not a
// literal (`anyIndex` and `negativeZero`; `NARROWING.md` lists this among the
// deliberate choices) and `delete` through an element access (`deleted`).
// Biome's declared type of `a[0]` includes `undefined`.
#[test]
fn literal_key_writes_cancel_their_path() {
    use InferredTypeData::{Null, String, Undefined};
    const SOURCE: &str = r#"
        function sameKey(o: { a: string | null }) {
            if (o.a) { o["a"] = null; /*sameKey*/o.a; } // null
        }
        function stringKey(t: [string | null]) {
            if (t[0]) { t["0"] = null; /*stringKey*/t[0]; } // null
        }
        function numberKey(r: Record<string, string | null>) {
            if (r["1"]) { r[1] = null; /*numberKey*/r["1"]; } // null
        }
        function prefix(o: { a: { b: string | null } }) {
            if (o.a.b) { o["a"] = { b: null }; /*prefix*/o.a.b; } // string | null
        }
        function nestedWrite(o: { a: { b: string | null } }) {
            if (o.a.b) { o["a"]["b"] = null; /*nestedWrite*/o.a.b; } // null
        }
        function anyIndex(a: (string | null)[], i: number) {
            if (a[0]) { a[i] = null; /*anyIndex*/a[0]; } // string
        }
        function negativeZero(t: [string | null]) {
            if (t[0]) { t[-0] = null; /*negativeZero*/t[0]; } // string
        }
        function deleted(o: { v?: string }) {
            if (o.v) { delete o["v"]; /*deleted*/o.v; } // string
        }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("sameKey", "o.a", &[String, Null]),
            ("stringKey", "t[0]", &[String, Null]),
            ("numberKey", r#"r["1"]"#, &[String, Null]),
            ("prefix", "o.a.b", &[String, Null]),
            ("nestedWrite", "o.a.b", &[String, Null]),
            ("anyIndex", "a[0]", &[String, Null, Undefined]),
            ("negativeZero", "t[0]", &[String, Null]),
            ("deleted", "o.v", &[String, Undefined]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) gives the type in the comment after each
// read. Two keys match only when they name the same property: `-0` is not a
// literal, `"01"` names another property than `1`, and an integer literal
// above 2^53 - 1 names the property of the nearest number, so
// `9007199254740993` names `"9007199254740992"`. TypeScript also matches other
// spellings of a key, such as `0x0`, `1e0`, a template literal, and an escape;
// Biome does not, so it is wider there.
#[test]
fn keys_that_name_other_properties_do_not_match() {
    use InferredTypeData::{Null, String};
    const SOURCE: &str = r#"
        function negativeZero(t: [string | null]) {
            if (t[-0]) { /*negativeZero*/t[0]; } // string | null
        }
        function leadingZero(o: { "01": string | null; 1: string | null }) {
            if (o[1]) { /*leadingZero*/o["01"]; } // string | null
            if (o["01"]) { /*withoutZero*/o["1"]; } // string | null
        }
        function unsafeInteger(r: Record<string, string | null>) {
            if (r[9007199254740993]) { /*unsafe*/r["9007199254740993"]; } // string | null
            if (r[12345678901234567890]) { /*large*/r["12345678901234567890"]; } // string | null
        }
        function otherSpellings(o: { a: string | null; 1: string | null }, t: [string | null]) {
            if (t[0x0]) { /*hex*/t[0]; } // string
            if (o[1e0]) { /*exponent*/o["1"]; } // string
            if (o[`a`]) { /*template*/o.a; } // string
            if (o["\u0061"]) { /*escape*/o.a; } // string
        }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("negativeZero", "t[0]", &[String, Null]),
            ("leadingZero", r#"o["01"]"#, &[String, Null]),
            ("withoutZero", r#"o["1"]"#, &[String, Null]),
            ("unsafe", r#"r["9007199254740993"]"#, &[String, Null]),
            ("large", r#"r["12345678901234567890"]"#, &[String, Null]),
            ("hex", "t[0]", &[String, Null]),
            ("exponent", r#"o["1"]"#, &[String, Null]),
            ("template", "o.a", &[String, Null]),
            ("escape", "o.a", &[String, Null]),
        ],
    );
}

#[test]
fn reads_of_a_path_with_one_declared_type_share_one_solve() {
    const SOURCE: &str = r#"
        function f(o: { v: string | null }) {
            if (o.v !== null) {
                /*first*/o.v;
                /*second*/o.v;
                /*element*/o["v"];
            }
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    db.clear_salsa_events();
    for (marker, expression) in [
        ("first", "o.v"),
        ("second", "o.v"),
        ("element", r#"o["v"]"#),
    ] {
        assert_eq!(
            normalized_type_at(&db, module, SOURCE, marker, expression),
            InferredTypeData::String
        );
    }
    let events = db.take_salsa_events();
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "infer_flow_subject_states", &events),
        1
    );
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "infer_flow_binding_baseline", &events),
        0
    );
    assert_function_query_was_not_run(&db, infer_module_types, module, &events);
}

#[test]
fn complete_module_tables_agree_with_path_reads() {
    const SOURCE: &str = r#"
        function f(o: { v: string | null } | null) {
            if (o && o.v !== null) {
                /*read*/o.v;
                /*element*/o["v"];
            }
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    for (marker, expression) in [("read", "o.v"), ("element", r#"o["v"]"#)] {
        let range = marked_range(SOURCE, marker, expression);
        let ty = infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range)).unwrap();
        assert_eq!(normalize_type(&db, module, ty), InferredTypeData::String);
        assert_eq!(
            infer_module_types(&db, module).unwrap().expressions[&range],
            ty
        );
    }
}

// TypeScript 6.0.3 (`tsc --strict`) gives every read below the expected type:
// paths on `this` narrow like paths on a variable in class methods and
// accessors, in object literal methods, and in arrow functions inside them or
// inside a field initializer.
#[test]
fn paths_on_this_narrow_like_paths_on_variables() {
    use InferredTypeData::{Number, String};
    const SOURCE: &str = r#"
        function isString(v: unknown): v is string { return typeof v === "string"; }
        class K {
            x: string | null = null;
            #y: string | null = null;
            n: string | number = 0;
            get g(): string | null { return null; }
            m() {
                if (this.x) { /*field*/this.x; }
                if (this.#y) { /*private*/this.#y; }
                if (this.g) { /*getter*/this.g; }
                if (this["x"]) { /*element*/this.x; }
                if (typeof this.n === "string") { /*typeof*/this.n; } else { /*typeofElse*/this.n; }
                if (isString(this.n)) { /*guard*/this.n; }
            }
            get accessor(): string {
                if (this.x !== null) { return /*accessor*/this.x; }
                return "";
            }
            arrow() {
                const g = () => { if (this.x) { /*arrow*/this.x; } };
            }
            fieldArrow = () => { if (this.x) { /*fieldArrow*/this.x; } };
        }
        const obj = {
            x: null as string | null,
            m() { if (this.x) { /*object*/this.x; } },
        };
    "#;
    assert_reads(
        SOURCE,
        &[
            ("field", "this.x", &[String]),
            ("private", "this.#y", &[String]),
            ("getter", "this.g", &[String]),
            ("element", "this.x", &[String]),
            ("typeof", "this.n", &[String]),
            ("typeofElse", "this.n", &[Number]),
            ("guard", "this.n", &[String]),
            ("accessor", "this.x", &[String]),
            ("arrow", "this.x", &[String]),
            ("fieldArrow", "this.x", &[String]),
            ("object", "this.x", &[String]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) gives the expected type at each read:
// narrowing never reaches an arrow function created after the test, and an
// object literal method or class method inside the method has its own `this`.
#[test]
fn reads_in_nested_functions_see_no_facts_about_this() {
    use InferredTypeData::{Null, Number, String};
    const SOURCE: &str = r#"
        class K {
            x: string | null = null;
            m() {
                if (this.x) {
                    const g = () => /*later*/this.x;
                    const o = { x: 1 as number | null, n() { return /*object*/this.x; } };
                    class F { x: number | null = null; n() { return /*class*/this.x; } }
                }
            }
        }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("later", "this.x", &[String, Null]),
            ("object", "this.x", &[Number, Null]),
            ("class", "this.x", &[Number, Null]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) gives the type in the comment after each
// read. In a constructor, and in an arrow function called where the
// constructor defines it, TypeScript reads a field without an initializer at
// its declared type after a test that leaves `undefined` possible (TS2565).
// Biome narrows no path on `this` in a constructor or in an arrow function
// created there, so it is wider where TypeScript narrows.
#[test]
fn constructors_keep_paths_on_this_unnarrowed() {
    use InferredTypeData::{Null, String};
    const SOURCE: &str = r#"
        class WithInitializer {
            x: string | null = null;
            constructor() {
                if (this.x !== null) { /*initializer*/this.x; } // string
                const g = () => { if (this.x) { /*arrow*/this.x; } }; // string
            }
        }
        class WithoutInitializer {
            x: string | null;
            constructor() {
                if (this.x !== null) { /*withoutInitializer*/this.x; } // string | null
                (() => { if (this.x !== null) { /*iife*/this.x; } })(); // string | null
                this.x = null;
            }
        }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("initializer", "this.x", &[String, Null]),
            ("arrow", "this.x", &[String, Null]),
            ("withoutInitializer", "this.x", &[String, Null]),
            ("iife", "this.x", &[String, Null]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) narrows `staticMethod` and `staticBlock` to
// `string`, and reports TS2339 at `instanceField`, because `this` in a static
// method or static block is the class. Biome types `this` there as an instance
// of the class, so it cannot read the type of `this.s`, and it reads `this.x`
// from the instance. It narrows no path on `this` there.
#[test]
fn static_members_keep_paths_on_this_unnarrowed() {
    use InferredTypeData::{Null, String, Unknown};
    const SOURCE: &str = r#"
        class S {
            static s: string | null = null;
            x: string | null = null;
            static m() {
                if (this.s) { /*staticMethod*/this.s; }
                if (this.x) { /*instanceField*/this.x; }
            }
            static { if (this.s) { /*staticBlock*/this.s; } }
        }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("staticMethod", "this.s", &[Unknown]),
            ("instanceField", "this.x", &[String, Null]),
            ("staticBlock", "this.s", &[Unknown]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) narrows every read below to `string`: an
// object literal method takes `this` from the object, or from its contextual
// type, and a function or method with a `this` parameter from the parameter.
// Biome finds the object or class only through a variable it initializes or a
// class name. Otherwise it types `this` from the enclosing class or object
// literal, so `argument` and `classArgument` read the field of `K` and
// `nested` the field of `o`, or not at all, and it ignores `this` parameters.
// It narrows no path on `this` there.
#[test]
fn paths_on_this_stay_unnarrowed_where_biome_types_this_otherwise() {
    use InferredTypeData::{Null, Number, String, Unknown};
    const SOURCE: &str = r#"
        declare function use(o: { x: string | null; n(): void }): void;
        declare function take(c: unknown): void;
        class K {
            x: number | null = null;
            m() {
                use({ x: null, n() { if (this.x) { /*argument*/this.x; } } });
                const o = { x: null as number | null, inner: { x: null as string | null, n() { if (this.x) { /*nested*/this.x; } } } };
                take(class { x: string | null = null; n() { if (this.x) { /*classArgument*/this.x; } } });
            }
        }
        use({ x: null, n() { if (this.x) { /*module*/this.x; } } });
        function f(this: { x: string | null }) { if (this.x) { /*function*/this.x; } }
        class T { x: string | null = null; m(this: T) { if (this.x) { /*thisParameter*/this.x; } } }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("argument", "this.x", &[Number, Null]),
            ("nested", "this.x", &[Number, Null]),
            ("classArgument", "this.x", &[Number, Null]),
            ("module", "this.x", &[Unknown]),
            ("function", "this.x", &[Unknown]),
            ("thisParameter", "this.x", &[String, Null]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) narrows every read below to `string`: a
// write to `this.x` in another method, in a callback, or in a function called
// through `.call(this)` never cancels its narrowing. Biome copies this unsound
// narrowing; `NARROWING.md` lists it among the deliberate choices.
#[test]
fn writes_to_this_in_other_functions_keep_narrowing() {
    use InferredTypeData::String;
    const SOURCE: &str = r#"
        class K {
            x: string | null = null;
            reset() { this.x = null; }
            method() { if (this.x) { this.reset(); /*method*/this.x; } }
            callback() { if (this.x) { [1].forEach(() => { this.x = null; }); /*callback*/this.x; } }
            call() {
                if (this.x) { (function (this: K) { this.x = null; }).call(this); /*call*/this.x; }
            }
        }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("method", "this.x", &[String]),
            ("callback", "this.x", &[String]),
            ("call", "this.x", &[String]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) gives the type in the comment after each
// read. Every write cancels the narrowing of the path it writes, so Biome reads
// it at its declared type. Where TypeScript differs, it narrows the path at the
// write itself, which Biome does not model yet, or ignores a write through
// `super`, which can set `this.x` through a setter of the parent class. A
// write to another path, at `other` and `superOther`, keeps the narrowing.
#[test]
fn writes_to_this_in_the_same_root_cancel_narrowing() {
    use InferredTypeData::{Null, String};
    const SOURCE: &str = r#"
        declare function get(): string | null;
        class K {
            x: string | null = null;
            #y: string | null = null;
            p: { y: string | null } = { y: null };
            assigned() { if (this.x) { this.x = get(); /*assigned*/this.x; } } // string | null
            privateName() { if (this.#y) { this.#y = get(); /*privateName*/this.#y; } } // string | null
            prefix() { if (this.p.y) { this.p = { y: null }; /*prefix*/this.p.y; } } // string | null
            other() { if (this.x) { this.p.y = null; this.#y = null; /*other*/this.x; } } // string
            arrowIife() { if (this.x) { (() => { this.x = null; })(); /*arrowIife*/this.x; } } // null
            functionIife() {
                if (this.x) { (function (this: any) { this.x = null; })(); /*functionIife*/this.x; } // null
            }
        }
        class P {
            get x(): string | null { return null; }
            set x(v: string | null) {}
        }
        class C extends P {
            y: string | null = null;
            m() { if (this.x) { super.x = null; /*super*/this.x; } } // string
            n() { if (this.y) { super.x = null; /*superOther*/this.y; } } // string
        }
    "#;
    assert_reads(
        SOURCE,
        &[
            ("assigned", "this.x", &[String, Null]),
            ("privateName", "this.#y", &[String, Null]),
            ("prefix", "this.p.y", &[String, Null]),
            ("arrowIife", "this.x", &[String, Null]),
            ("functionIife", "this.x", &[String, Null]),
            ("super", "this.x", &[String, Null]),
            ("other", "this.x", &[String]),
            ("superOther", "this.y", &[String]),
        ],
    );
}

// TypeScript 6.0.3 (`tsc --strict`) gives the type in the comment after each
// read. A private name narrows on any object, as `o.#v` does. TypeScript also
// treats `this["#x"]` as the same reference as `this.#x`, although they are
// different properties; Biome keeps them apart, so it is wider there.
#[test]
fn private_names_narrow_and_differ_from_string_keys() {
    use InferredTypeData::{Null, String};
    const SOURCE: &str = r##"
        class P {
            #v: string | null = null;
            static f(o: P | null) {
                if (o && o.#v) { /*narrowedObject*/o.#v; } // string
            }
        }
        class Q {
            #x: string | null = null;
            "#x": string | null = null;
            m() {
                if (this.#x) { /*fromPrivate*/this["#x"]; } // string
                if (this["#x"]) { /*fromString*/this.#x; } // string
            }
        }
    "##;
    assert_reads(
        SOURCE,
        &[
            ("narrowedObject", "o.#v", &[String]),
            ("fromPrivate", r##"this["#x"]"##, &[String, Null]),
            ("fromString", "this.#x", &[String, Null]),
        ],
    );
}

#[test]
fn complete_module_tables_agree_with_reads_of_paths_on_this() {
    const SOURCE: &str = r#"
        class K {
            x: string | null = null;
            #y: string | null = null;
            m() {
                if (this.x !== null && this.#y !== null) {
                    /*read*/this.x;
                    /*private*/this.#y;
                }
            }
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    for (marker, expression) in [("read", "this.x"), ("private", "this.#y")] {
        let range = marked_range(SOURCE, marker, expression);
        let ty = infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range)).unwrap();
        assert_eq!(normalize_type(&db, module, ty), InferredTypeData::String);
        assert_eq!(
            infer_module_types(&db, module).unwrap().expressions[&range],
            ty
        );
    }
}
