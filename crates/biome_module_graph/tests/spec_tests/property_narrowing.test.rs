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

#[test]
fn reads_of_a_path_with_one_declared_type_share_one_solve() {
    const SOURCE: &str = r#"
        function f(o: { v: string | null }) {
            if (o.v !== null) {
                /*first*/o.v;
                /*second*/o.v;
            }
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    db.clear_salsa_events();
    for marker in ["first", "second"] {
        assert_eq!(
            normalized_type_at(&db, module, SOURCE, marker, "o.v"),
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
            }
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    let range = marked_range(SOURCE, "read", "o.v");
    let ty = infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range)).unwrap();
    assert_eq!(normalize_type(&db, module, ty), InferredTypeData::String);
    assert_eq!(
        infer_module_types(&db, module).unwrap().expressions[&range],
        ty
    );
}
