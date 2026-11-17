use super::*;
use biome_module_graph::{
    BindingTypeInput, ExpressionTypeInput, infer_binding_type, infer_expression_type,
    type_inference::{
        NormalizedExpressionTypeRequest, TypeInferenceCaller, execute_type_inference_request,
    },
};

// Expected types are those that TypeScript 7.0.2 (`tsc --noEmit --strict
// --target es2022`) gives the same reads, unless a comment says otherwise.
// TypeScript infers a type predicate for a function without a return type
// annotation whose body returns a boolean test of a parameter: the
// parameter's type where the test holds becomes the target when it differs
// from the declared type and the test failing leaves nothing of it.

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

fn predicates_db(source: &str) -> (TestModuleDb, ModuleInfo) {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), source);
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    (db, module)
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

/// Asserts the type of each `(marker, expression, expected)` read in
/// [`DECLARATIONS`] followed by `source`, by the names of its members (see
/// [`variant_names`]). Also asserts that complete module inference gives
/// each read the type that a lookup gives it.
fn assert_reads(source: &str, reads: &[(&str, &str, &[&str])]) {
    let source = format!("{DECLARATIONS}{source}");
    let source = source.as_str();
    let lookups: Vec<_> = {
        let (db, module) = predicates_db(source);
        reads
            .iter()
            .map(|(marker, expression, _)| {
                let range = marked_range(source, marker, expression);
                infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range))
                    .map(|ty| format_inferred_type(&db, ty))
            })
            .collect()
    };
    let (db, module) = predicates_db(source);
    let complete = infer_module_types(&db, module).unwrap();
    let mismatches: Vec<_> = reads
        .iter()
        .zip(lookups)
        .flat_map(|((marker, expression, expected), lookup)| {
            let range = marked_range(source, marker, expression);
            let ty = execute_type_inference_request(
                &db,
                TypeInferenceCaller::new("test", "inferred_predicates"),
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

const DECLARATIONS: &str = r#"
    interface Foo { a: number }
    declare function isFoo(v: unknown): v is Foo;
    const isStr = (v: unknown) => typeof v === "string";
    function isNum(v: unknown) { return typeof v === "number"; }
    const notNull = (v: string | null) => v !== null;
    function isDefined<T>(v: T | undefined) { return v !== undefined; }
    const isFooish = (v: unknown) => isFoo(v);
    const isDate = (v: unknown) => v instanceof Date;
    const hasA = (v: { a: 1 } | { b: 2 }) => "a" in v;
    const isTruthy = (v: string | null | undefined) => !!v;
    const isC = (s: { kind: "c"; r: number } | { kind: "s"; w: number }) => s.kind === "c";
    const isStrOrNum = (v: unknown) => typeof v === "string" || typeof v === "number";
    const isPositive = (v: string | number) => typeof v === "number" && v > 0;
    const isNotStr = (v: string | number) => typeof v !== "string";
    const isStrBlock = (v: unknown) => { return typeof v === "string"; };
    function twoReturns(v: unknown) { if (typeof v === "string") return true; return false; }
    function mutates(v: string | null) { v = "a"; return v !== null; }
    const isNullish = (v: string | null | undefined) => v == null;
    const isA = (v: "a" | "b") => v === "a";
    const isObj = (v: unknown) => typeof v === "object" && v !== null;
    const truthyUnknown = (v: unknown) => !!v;
    const second = (a: unknown, b: unknown) => typeof b === "string";
    const isStrAsync = async (v: unknown) => typeof v === "string";
    class K { isS(v: unknown) { return typeof v === "string"; } }
    const obj = { isS(v: unknown) { return typeof v === "string"; } };
"#;

#[test]
fn calls_narrow_by_inferred_type_predicates() {
    assert_reads(
        r#"
        function f1(x: string | number) { if (isStr(x)) /*L30_t*/x; else /*L30_f*/x; }
        function f2(x: string | number) { if (isNum(x)) /*L31_t*/x; else /*L31_f*/x; }
        function f3(x: string | null) { if (notNull(x)) /*L32_t*/x; else /*L32_f*/x; }
        function f4(x: string | undefined) { if (isDefined(x)) /*L33_t*/x; else /*L33_f*/x; }
        function f5(x: Foo | string) { if (isFooish(x)) /*L34_t*/x; else /*L34_f*/x; }
        function f6(x: Date | string) { if (isDate(x)) /*L35_t*/x; else /*L35_f*/x; }
        function f7(x: { a: 1 } | { b: 2 }) { if (hasA(x)) /*L36_t*/x; else /*L36_f*/x; }
        function f9(s: { kind: "c"; r: number } | { kind: "s"; w: number }) { if (isC(s)) /*L38_t*/s; else /*L38_f*/s; }
        function f10(x: string | number | boolean) { if (isStrOrNum(x)) /*L39_t*/x; else /*L39_f*/x; }
        function f12(x: string | number) { if (isNotStr(x)) /*L41_t*/x; else /*L41_f*/x; }
        function f13(x: string | number) { if (isStrBlock(x)) /*L42_t*/x; }
        function f16(x: string | null | undefined) { if (isNullish(x)) /*L45_t*/x; else /*L45_f*/x; }
        function f17(x: "a" | "b") { if (isA(x)) /*L46_t*/x; else /*L46_f*/x; }
        function f20(x: unknown, y: string | number) { if (second(x, y)) /*L49_t*/y; }
        function f21(x: string | number) { if (new K().isS(x)) /*L50_t*/x; }
        function f22(x: string | number) { if (obj.isS(x)) /*L51_t*/x; }
        "#,
        &[
            ("L30_t", "x", &["string"]),
            ("L30_f", "x", &["number"]),
            ("L31_t", "x", &["number"]),
            ("L31_f", "x", &["string"]),
            ("L32_t", "x", &["string"]),
            ("L32_f", "x", &["null"]),
            // TypeScript gives `string` and `undefined`. Biome infers no type
            // predicate for `isDefined`: where `v !== undefined` holds, `v`
            // is `T & ({} | null)`, and a strict comparison with `undefined`
            // keeps an intersection with a type parameter as it is, so the
            // comparison failing doesn't leave nothing of it.
            ("L33_t", "x", &["string", "undefined"]),
            ("L33_f", "x", &["string", "undefined"]),
            (
                "L34_t",
                "x",
                &["interface \"Foo\" { extends: [] type_args: [] members: [\"a\": number] }"],
            ),
            ("L34_f", "x", &["string"]),
            ("L35_t", "x", &["instanceof Date"]),
            ("L35_f", "x", &["string"]),
            ("L36_t", "x", &["[\"a\": number: 1]"]),
            ("L36_f", "x", &["[\"b\": number: 2]"]),
            ("L38_t", "s", &["[\"kind\": string: c, \"r\": number]"]),
            ("L38_f", "s", &["[\"kind\": string: s, \"w\": number]"]),
            ("L39_t", "x", &["number", "string"]),
            ("L39_f", "x", &["boolean"]),
            ("L41_t", "x", &["number"]),
            ("L41_f", "x", &["string"]),
            ("L42_t", "x", &["string"]),
            ("L45_t", "x", &["null", "undefined"]),
            ("L45_f", "x", &["string"]),
            ("L46_t", "x", &["string: a"]),
            ("L46_f", "x", &["string: b"]),
            ("L49_t", "y", &["string"]),
            // TypeScript gives `string`. A call point needs a callee written
            // as a dotted name, which `new K().isS` isn't.
            ("L50_t", "x", &["number", "string"]),
            ("L51_t", "x", &["string"]),
        ],
    );
}

// TypeScript infers no type predicate for these functions, so the calls keep
// the declared types, unless a comment says otherwise.
#[test]
fn functions_that_infer_no_type_predicate_narrow_nothing() {
    assert_reads(
        r#"
        function* genStr(v: unknown) { return typeof v === "string"; }
        const restStr = (...v: unknown[]) => typeof v[0] === "string";
        function written(v: string | null) { v = v; return v !== null; }
        const isTrue = (v: boolean) => v === true;
        type B = boolean;
        const isTrueAlias = (v: B) => v === true;
        const defaulted = (v: string | null = null) => v !== null;
        function prior(x: string | null | number) { if (x === null) throw 0; return typeof x === "string" || x === null; }
        function unrelated(x: string | number) { const k = 1; return typeof x === "string"; }
        const firstOf = (a: string | null, b: string | null) => a !== null && b !== null;
        function f8(x: string | null | undefined) { if (isTruthy(x)) /*L37_t*/x; else /*L37_f*/x; }
        function f11(x: string | number) { if (isPositive(x)) /*L40_t*/x; else /*L40_f*/x; }
        function f14(x: string | number) { if (twoReturns(x)) /*L43_t*/x; }
        function f15(x: string | null) { if (mutates(x)) /*L44_t*/x; }
        function f18(x: unknown) { if (isObj(x)) /*L47_t*/x; else /*L47_f*/x; }
        function f19(x: unknown) { if (truthyUnknown(x)) /*L48_t*/x; else /*L48_f*/x; }
        function h1(x: string | number) { if (genStr(x)) /*generator*/x; }
        function h2(x: string | number) { if (restStr(x)) /*rest*/x; }
        function h3(x: string | null) { if (written(x)) /*written*/x; }
        function h4(x: boolean) { if (isTrue(x)) /*boolean*/x; }
        function h5(x: boolean) { if (isTrueAlias(x)) /*alias*/x; }
        function h6(x: string | null) { if (defaulted(x)) /*defaulted*/x; }
        function h7(y: string | null | number) { if (prior(y)) /*prior*/y; }
        function h8(y: string | number) { if (unrelated(y)) /*unrelated*/y; }
        function h9(x: string | null, y: string | null) { if (firstOf(x, y)) /*first*/y; }
        async function h10(x: string | number) { if (await isStrAsync(x)) /*async*/x; }
        "#,
        &[
            ("L37_t", "x", &["null", "string", "undefined"]),
            ("L37_f", "x", &["null", "string", "undefined"]),
            ("L40_t", "x", &["number", "string"]),
            ("L40_f", "x", &["number", "string"]),
            ("L43_t", "x", &["number", "string"]),
            ("L44_t", "x", &["null", "string"]),
            // TypeScript gives `object` and `unknown`. Biome keeps `object`
            // where `typeof v !== "object"`, because a function is an
            // `object` whose `typeof` is `"function"`, so `isObj` gets no
            // type predicate.
            ("L47_t", "x", &["unknown"]),
            ("L47_f", "x", &["unknown"]),
            ("L48_t", "x", &["unknown"]),
            ("L48_f", "x", &["unknown"]),
            ("generator", "x", &["number", "string"]),
            ("rest", "x", &["number", "string"]),
            ("written", "x", &["null", "string"]),
            ("boolean", "x", &["boolean"]),
            ("alias", "x", &["boolean"]),
            // TypeScript gives `string`. Biome infers no type predicate for
            // a parameter with a default value.
            ("defaulted", "x", &["null", "string"]),
            // TypeScript gives `string`. Biome infers no type predicate when
            // a statement comes before the `return`, because such a
            // statement can narrow the parameter, as `x === null` does here.
            ("prior", "y", &["null", "number", "string"]),
            ("unrelated", "y", &["number", "string"]),
            ("first", "y", &["null", "string"]),
            ("async", "x", &["number", "string"]),
        ],
    );
}

#[test]
fn inferred_type_predicates_are_part_of_function_types() {
    assert_reads(
        r#"
        const isTrueOrUndefined = (v: boolean | undefined) => v === true;
        const isDef = (v?: string) => v !== undefined;
        const neverStr = (v: string) => typeof v === "number";
        const onlySecond = (a: unknown, b: string | null) => b !== null;
        /*L58*/isStr;
        /*L59*/isPositive;
        /*L61*/isTruthy;
        /*L62*/isStrAsync;
        /*true*/isTrueOrUndefined;
        /*optional*/isDef;
        /*never*/neverStr;
        /*second*/onlySecond;
        function h12(x?: string) { if (isDef(x)) /*optional_t*/x; }
        function h13(x: string) { if (neverStr(x)) /*never_t*/x; else /*never_f*/x; }
        function h14(x: boolean | undefined) { if (isTrueOrUndefined(x)) /*true_t*/x; else /*true_f*/x; }
        "#,
        &[
            (
                "L58",
                "isStr",
                &[
                    "sync Function { accepts: { params: [ required v: unknown ] type_args: [] } returns: predicate v: string }",
                ],
            ),
            (
                "L59",
                "isPositive",
                &[
                    "sync Function { accepts: { params: [ required v: string | number ] type_args: [] } returns: boolean }",
                ],
            ),
            (
                "L61",
                "isTruthy",
                &[
                    "sync Function { accepts: { params: [ required v: string | null | undefined ] type_args: [] } returns: boolean }",
                ],
            ),
            (
                "L62",
                "isStrAsync",
                &[
                    "async Function { accepts: { params: [ required v: unknown ] type_args: [] } returns: instanceof Promise<boolean> }",
                ],
            ),
            (
                "true",
                "isTrueOrUndefined",
                &[
                    "sync Function { accepts: { params: [ required v: boolean | undefined ] type_args: [] } returns: predicate v: bool: true }",
                ],
            ),
            (
                "optional",
                "isDef",
                &[
                    "sync Function { accepts: { params: [ optional v: string | undefined ] type_args: [] } returns: predicate v: string }",
                ],
            ),
            (
                "never",
                "neverStr",
                &[
                    "sync Function { accepts: { params: [ required v: string ] type_args: [] } returns: predicate v: never }",
                ],
            ),
            (
                "second",
                "onlySecond",
                &[
                    "sync Function { accepts: { params: [ required a: unknown required b: string | null ] type_args: [] } returns: predicate b: string }",
                ],
            ),
            // TypeScript gives `string`. A call point narrows no optional
            // parameter.
            ("optional_t", "x", &["string", "undefined"]),
            // TypeScript gives `never`, which Biome reads as Unknown.
            ("never_t", "x", &["Unknown"]),
            ("never_f", "x", &["string"]),
            ("true_t", "x", &["bool: true"]),
            ("true_f", "x", &["bool: false", "undefined"]),
        ],
    );
}

#[test]
fn callbacks_with_inferred_type_predicates_bind_filter_find_and_every() {
    assert_reads(
        r#"
        function g1(xs: (string | undefined)[]) { /*L53*/xs.filter((x) => x !== undefined); }
        function g2(xs: (string | number)[]) { /*L54*/xs.filter((x) => typeof x === "string"); }
        function g3(xs: (string | number)[]) { if (xs.every((x) => typeof x === "string")) /*L55*/xs; }
        function g4(xs: (string | null)[]) { /*L56*/xs.find((x) => x !== null); }
        function g5(xs: (string | number)[]) { /*L57*/xs.filter(isStr); }
        function g11(xs: (string | number)[]) { /*L63*/xs.filter((x): x is string => typeof x === "string"); }
        function g12(xs: string[]) { /*L64*/xs.filter((x) => x.length > 0); }
        function g13(xs: (string | number)[]) { /*L65*/xs.filter(isPositive); }
        function g14(xs: (string | number)[]) { /*L66*/xs.find((x) => x === 1); }
        function g15(ys: boolean[]) { /*booleans*/ys.filter((y) => y === true); }
        function g16(xs: ({ kind: "a"; v: 1 } | { kind: "b"; w: 2 })[]) { /*kinds*/xs.filter((x) => x.kind === "a"); }
        function g17(xs: ({ kind: "a"; v: 1 } | { kind: "b"; w: 2 })[]) { /*index*/xs.find((x, i) => i > 0 && x.kind === "a"); }
        function g18(xs: (string | number)[]) { if (xs.every(isStr)) /*declared*/xs; }
        "#,
        &[
            (
                "L53",
                "xs.filter((x) => x !== undefined)",
                &["instanceof Array<string>"],
            ),
            (
                "L54",
                "xs.filter((x) => typeof x === \"string\")",
                &["instanceof Array<string>"],
            ),
            // TypeScript gives `string[]`. Selecting the signature of
            // `every` that narrows compares the callback's function type,
            // whose parameter without a type annotation Biome doesn't type
            // from the call.
            ("L55", "xs", &["instanceof Array<string | number>"]),
            (
                "L56",
                "xs.find((x) => x !== null)",
                &["string", "undefined"],
            ),
            ("L57", "xs.filter(isStr)", &["instanceof Array<string>"]),
            (
                "L63",
                "xs.filter((x): x is string => typeof x === \"string\")",
                &["instanceof Array<string>"],
            ),
            (
                "L64",
                "xs.filter((x) => x.length > 0)",
                &["instanceof Array<string>"],
            ),
            (
                "L65",
                "xs.filter(isPositive)",
                &["instanceof Array<string | number>"],
            ),
            (
                "L66",
                "xs.find((x) => x === 1)",
                &["number: 1", "undefined"],
            ),
            (
                "booleans",
                "ys.filter((y) => y === true)",
                &["instanceof Array<boolean>"],
            ),
            (
                "kinds",
                "xs.filter((x) => x.kind === \"a\")",
                &[
                    "instanceof Array<Object { prototype: No prototype members: [\"kind\": string: a, \"v\": number: 1] }>",
                ],
            ),
            (
                "index",
                "xs.find((x, i) => i > 0 && x.kind === \"a\")",
                &[
                    "[\"kind\": string: a, \"v\": number: 1]",
                    "[\"kind\": string: b, \"w\": number: 2]",
                    "undefined",
                ],
            ),
            ("declared", "xs", &["instanceof Array<string>"]),
        ],
    );
}

// A function that calls another one with an inferred type predicate gets
// the type predicate too.
#[test]
fn inferred_type_predicates_follow_calls_to_other_ones() {
    assert_reads(
        r#"
        const viaOther = (v: unknown) => isStr(v);
        function h17(x: string | number) { if (viaOther(x)) /*then*/x; else /*else*/x; }
        "#,
        &[("then", "x", &["string"]), ("else", "x", &["number"])],
    );
}

// A lookup of a narrowed read, or of a call's result, infers the type
// predicate of the function that it calls once, and that of no other
// function, without inferring the module completely.
#[test]
fn lookups_infer_the_type_predicate_of_each_callee_once() {
    const SOURCE: &str = r#"
        const isStr = (v: unknown) => typeof v === "string";
        const isNum = (v: unknown) => typeof v === "number";
        function f(x: string | number, xs: (string | number)[]) {
            if (isStr(x)) { /*first*/x; }
            if (isStr(x)) { /*second*/x; }
            /*filter*/xs.filter(isStr);
        }
    "#;
    let (db, module) = predicates_db(SOURCE);
    db.clear_salsa_events();
    for (marker, expression) in [
        ("first", "x"),
        ("second", "x"),
        ("filter", "xs.filter(isStr)"),
    ] {
        let range = marked_range(SOURCE, marker, expression);
        infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range));
    }
    let events = db.take_salsa_events();
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "infer_flow_inferred_predicate", &events),
        1
    );
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "infer_module_types", &events),
        0
    );
}

// The parameter of a callback takes its type from the call that the
// callback is passed to, without the callback itself, so typing the
// parameter infers no type predicate, and inferring the callback's type
// predicate for the call's result passes through no query cycle.
#[test]
fn callbacks_infer_type_predicates_without_a_cycle() {
    const SOURCE: &str = r#"
        function g(xs: (string | undefined)[]) {
            return /*call*/xs.filter((/*parameter*/x) => x !== undefined);
        }
    "#;
    let (db, module) = predicates_db(SOURCE);
    let parameter = marked_range(SOURCE, "parameter", "x");
    db.clear_salsa_events();
    let ty = infer_binding_type(&db, BindingTypeInput::new(&db, module, parameter)).unwrap();
    assert_eq!(variant_names(&db, ty), ["string", "undefined"]);
    let events = db.take_salsa_events();
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "infer_flow_inferred_predicate", &events),
        0
    );
    let call = marked_range(
        SOURCE,
        "call",
        "xs.filter((/*parameter*/x) => x !== undefined)",
    );
    let ty = infer_expression_type(&db, ExpressionTypeInput::new(&db, module, call)).unwrap();
    assert_eq!(variant_names(&db, ty), ["instanceof Array<string>"]);
    let events = db.take_salsa_events();
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "infer_flow_inferred_predicate", &events),
        1
    );
}
