use super::*;
use biome_module_graph::{
    BindingTypeInput, ExpressionTypeInput, infer_binding_type, infer_expression_is_promise,
    infer_expression_type,
    type_inference::{
        ArrayOfPromisesClassificationRequest, NormalizedExpressionTypeRequest,
        PromiseClassificationRequest, PromiseReturningFunctionClassificationRequest,
        TypeInferenceCaller, TypeInferenceClassification, execute_type_inference_request,
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

fn guard_db(source: &str) -> (TestModuleDb, ModuleInfo) {
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
        TypeInferenceCaller::new("test", "functionGuards"),
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

fn assert_no_whole_module_inference(db: &TestModuleDb, events: &[salsa::Event]) {
    assert_eq!(
        function_query_will_execute_count_by_name(db, "infer_module_types", events),
        0,
        "targeted guard requests must not infer complete modules"
    );
}

#[test]
fn direct_guards_refine_only_the_selected_occurrences_not_the_declaration() {
    use InferredTypeData::{Number, String};
    for declaration in [
        "function isString(input: unknown): input is string { return typeof input === 'string'; }",
        "declare function isString(input: unknown): input is string;",
    ] {
        let source = format!(
            r#"
                {declaration}
                function inspect(/*binding*/value: string | number) {{
                    /*before*/value;
                    if (isString(/*argument*/value)) {{
                        /*then*/value;
                    }} else {{
                        /*else*/value;
                    }}
                    /*join*/value;
                }}
            "#
        );
        let (db, module) = guard_db(&source);
        db.clear_salsa_events();
        for (marker, expected) in [
            ("then", &[String][..]),
            ("else", &[Number]),
            ("before", &[String, Number]),
            ("argument", &[String, Number]),
            ("join", &[String, Number]),
        ] {
            let ty = normalized_type_at(&db, module, &source, marker, "value");
            assert_variants(&db, ty, expected, marker);
        }
        let binding = marked_range(&source, "binding", "value");
        let ty = infer_binding_type(&db, BindingTypeInput::new(&db, module, binding)).unwrap();
        assert_variants(
            &db,
            normalize_type(&db, module, ty),
            &[String, Number],
            "binding",
        );
        assert_no_whole_module_inference(&db, &db.take_salsa_events());
    }
}

#[test]
fn primitive_keyword_guards_filter_both_branches_and_refine_explicit_unknown() {
    use InferredTypeData::{BigInt, Boolean, Null, Number, String, Symbol, Undefined};
    let cases = [
        ("string", String),
        ("number", Number),
        ("boolean", Boolean),
        ("bigint", BigInt),
        ("symbol", Symbol),
        ("null", Null),
        ("undefined", Undefined),
    ];
    for (target, expected) in cases {
        let source = format!(
            r#"
                declare function guard(input: unknown): input is {target};
                function inspect(value: string | number | boolean | bigint | symbol | null | undefined) {{
                    if (guard(value)) {{ /*then*/value; }} else {{ /*else*/value; }}
                }}
                function unknownValue(value: unknown) {{
                    if (guard(value)) {{ /*unknownThen*/value; }} else {{ /*unknownElse*/value; }}
                }}
            "#
        );
        let (db, module) = guard_db(&source);
        let complement = cases
            .iter()
            .map(|(_, ty)| *ty)
            .filter(|ty| *ty != expected)
            .collect::<Vec<_>>();
        for (marker, expected) in [
            ("then", &[expected][..]),
            ("else", complement.as_slice()),
            ("unknownThen", &[expected]),
            ("unknownElse", &[InferredTypeData::UnknownKeyword]),
        ] {
            let ty = normalized_type_at(&db, module, &source, marker, "value");
            assert_variants(&db, ty, expected, &format!("{target}: {marker}"));
        }
    }
}

#[test]
fn primitive_literal_guards_keep_the_complement_of_literal_unions() {
    for (target, other) in [
        (r#""ready""#, r#""""#),
        ("1", "0"),
        ("-1", "1"),
        ("true", "0"),
        ("false", "0"),
        ("1n", "0n"),
        ("-1n", "1n"),
        ("- /* sign trivia */ 1n", "1n"),
    ] {
        let source = format!(
            r#"
                declare function guard(input: unknown): input is {target};
                declare const expectedTarget: {target};
                declare const expectedOther: {other};
                /*target*/expectedTarget;
                /*other*/expectedOther;
                function inspect(value: {target} | {other}) {{
                    if (guard(value)) {{ /*then*/value; }} else {{ /*else*/value; }}
                }}
                function unknownValue(value: unknown) {{
                    if (guard(value)) {{ /*unknownThen*/value; }}
                }}
            "#
        );
        let (db, module) = guard_db(&source);
        let expected_target = normalized_type_at(&db, module, &source, "target", "expectedTarget");
        let expected_other = normalized_type_at(&db, module, &source, "other", "expectedOther");
        for (marker, expected) in [
            ("then", expected_target),
            ("else", expected_other),
            ("unknownThen", expected_target),
        ] {
            let actual = normalized_type_at(&db, module, &source, marker, "value");
            assert_eq!(
                actual,
                expected,
                "{target}: {marker}: actual {}, expected {}",
                format_inferred_type(&db, actual),
                format_inferred_type(&db, expected),
            );
        }
    }
}

// TypeScript 7.0.2 (`tsc --strict`) gives the target in the `then` branch and
// the broad primitive in the `else` branch. The exception is `boolean`, which
// is `true | false`, so the `else` branch of `input is true` gives `false`.
#[test]
fn literal_guards_do_not_exclude_individual_values_from_broad_primitives() {
    for (target, primitive, otherwise) in [
        (r#""ready""#, "string", "string"),
        ("true", "boolean", "false"),
        ("1", "number", "number"),
        ("1n", "bigint", "bigint"),
    ] {
        let source = format!(
            r#"
                declare function guard(input: unknown): input is {target};
                declare const expectedTarget: {target};
                /*target*/expectedTarget;
                declare const expectedOtherwise: {otherwise};
                /*otherwise*/expectedOtherwise;
                function inspect(value: {primitive}) {{
                    if (guard(value)) {{ /*then*/value; }} else {{ /*else*/value; }}
                }}
            "#
        );
        let (db, module) = guard_db(&source);
        let expected_target = normalized_type_at(&db, module, &source, "target", "expectedTarget");
        let expected_otherwise =
            normalized_type_at(&db, module, &source, "otherwise", "expectedOtherwise");
        assert_eq!(
            normalized_type_at(&db, module, &source, "then", "value"),
            expected_target,
            "{primitive}: then"
        );
        assert_eq!(
            normalized_type_at(&db, module, &source, "else", "value"),
            expected_otherwise,
            "{primitive}: else"
        );
    }
}

#[test]
fn negated_guards_and_early_returns_refine_the_surviving_path() {
    use InferredTypeData::{Number, String};
    const SOURCE: &str = r#"
        declare function isString(input: unknown): input is string;
        function negated(value: string | number) {
            if (!isString(value)) { /*negatedThen*/value; } else { /*negatedElse*/value; }
        }
        function positiveReturn(value: string | number) {
            if (isString(value)) return;
            /*number*/value;
        }
        function negativeReturn(value: string | number) {
            if (!isString(value)) return;
            /*string*/value;
        }
    "#;
    let (db, module) = guard_db(SOURCE);
    for (marker, expected) in [
        ("negatedThen", Number),
        ("negatedElse", String),
        ("number", Number),
        ("string", String),
    ] {
        assert_eq!(
            normalized_type_at(&db, module, SOURCE, marker, "value"),
            expected,
            "{marker}"
        );
    }
}

#[test]
fn guards_compose_with_logical_conditional_and_ordinary_loop_flow() {
    use InferredTypeData::{Number, String};
    const SOURCE: &str = r#"
        declare function isString(input: unknown): input is string;
        function expressions(value: string | number, flag: boolean) {
            isString(value) && /*and*/value;
            isString(value) || /*or*/value;
            isString(value) ? /*consequent*/value : /*alternate*/value;
            if (isString(value) && flag) { /*andThen*/value; }
            if (isString(value) || flag) { /*orThen*/value; } else { /*orElse*/value; }
            /*join*/value;
        }
        function whileLoop(value: string | number) {
            while (isString(value)) { /*whileBody*/value; }
            /*whileExit*/value;
        }
        function breakLoop(value: string | number, stop: boolean) {
            while (isString(value)) {
                if (stop) break;
                /*breakBody*/value;
            }
            /*breakExit*/value;
        }
        function forLoop(value: string | number) {
            for (; isString(value); /*update*/value) { /*forBody*/value; }
            /*forExit*/value;
        }
        function doLoop(value: string | number) {
            do { /*doBody*/value; } while (isString(value));
            /*doExit*/value;
        }
    "#;
    let (db, module) = guard_db(SOURCE);
    for (marker, expected) in [
        ("and", &[String][..]),
        ("or", &[Number]),
        ("consequent", &[String]),
        ("alternate", &[Number]),
        ("andThen", &[String]),
        ("orThen", &[String, Number]),
        ("orElse", &[Number]),
        ("join", &[String, Number]),
        ("whileBody", &[String]),
        ("whileExit", &[Number]),
        ("breakBody", &[String]),
        ("breakExit", &[String, Number]),
        ("update", &[String]),
        ("forBody", &[String]),
        ("forExit", &[Number]),
        ("doBody", &[String, Number]),
        ("doExit", &[Number]),
    ] {
        let ty = normalized_type_at(&db, module, SOURCE, marker, "value");
        assert_variants(&db, ty, expected, marker);
    }
}

#[test]
fn guarded_reads_reach_enclosing_calls_members_and_addition() {
    use InferredTypeData::{Number, String};
    const SOURCE: &str = r#"
        declare function isString(input: unknown): input is string;
        declare function isNull(input: unknown): input is null;
        declare function identity<T>(input: T): T;
        function primitive(value: string | number) {
            if (isString(value)) {
                /*stringCall*/identity(value);
                /*stringAddition*/value + 1;
            } else {
                /*numberCall*/identity(value);
                /*numberAddition*/value + 1;
            }
        }
        function members(value: { length: number; method(): string } | null) {
            if (isNull(value)) return;
            /*member*/value?.length;
            /*method*/value?.method();
            /*nestedCall*/identity(value?.length);
        }
    "#;
    let (db, module) = guard_db(SOURCE);
    for (marker, expression, expected) in [
        ("stringCall", "identity(value)", String),
        ("numberCall", "identity(value)", Number),
        ("stringAddition", "value + 1", String),
        ("numberAddition", "value + 1", Number),
        ("member", "value?.length", Number),
        ("method", "value?.method()", String),
        ("nestedCall", "identity(value?.length)", Number),
    ] {
        assert_eq!(
            normalized_type_at(&db, module, SOURCE, marker, expression),
            expected,
            "{marker}"
        );
    }
    assert_no_whole_module_inference(&db, &db.take_salsa_events());
}

#[test]
fn parameter_mapping_skips_synthetic_this_and_decodes_escaped_names() {
    use InferredTypeData::{Number, String};
    const SOURCE: &str = r#"
        declare function second(ignored: unknown, input: unknown): input is string;
        declare function withThis(this: void, input: unknown): input is string;
        declare function withThisSecond(this: void, ignored: unknown, input: unknown): input is number;
        declare function escapedFormal(\u0076alue: unknown): value is string;
        declare function escapedPredicate(value: unknown): \u0076alue is number;
        declare function escapedBoth(\u0076alue: unknown): \u{76}alue is string;
        function inspect(value: string | number, other: string | number) {
            if (second(value, other)) {
                /*firstArgument*/value;
                /*secondArgument*/other;
            } else { /*secondElse*/other; }
            if (withThis(value)) { /*thisArgument*/value; }
            if (withThisSecond(value, other)) {
                /*thisFirstArgument*/value;
                /*thisSecondArgument*/other;
            }
            if (escapedFormal(value)) { /*escapedFormal*/value; }
            if (escapedPredicate(value)) { /*escapedPredicate*/value; }
            if (escapedBoth(value)) { /*escapedBoth*/value; }
        }
        function escapedSubject(\u0076alue: string | number) {
            if (escapedFormal(value)) { /*escapedSubject*/\u0076alue; }
        }
    "#;
    let (db, module) = guard_db(SOURCE);
    for (marker, expression, expected) in [
        ("firstArgument", "value", &[String, Number][..]),
        ("secondArgument", "other", &[String]),
        ("secondElse", "other", &[Number]),
        ("thisArgument", "value", &[String]),
        ("thisFirstArgument", "value", &[String, Number]),
        ("thisSecondArgument", "other", &[Number]),
        ("escapedFormal", "value", &[String]),
        ("escapedPredicate", "value", &[Number]),
        ("escapedBoth", "value", &[String]),
        ("escapedSubject", r"\u0076alue", &[String]),
    ] {
        let ty = normalized_type_at(&db, module, SOURCE, marker, expression);
        assert_variants(&db, ty, expected, marker);
    }
}

#[test]
fn callee_and_subject_shadowing_use_semantic_binding_identity() {
    use InferredTypeData::{Number, String};
    const SOURCE: &str = r#"
        declare function guard(input: unknown): input is string;
        declare function read(): string | number;
        function shadowedSubject(value: string | number) {
            if (guard(value)) {
                {
                    const value = read();
                    /*innerBefore*/value;
                    if (guard(value)) { /*innerThen*/value; }
                    /*innerJoin*/value;
                }
                /*outer*/value;
            }
        }
        function shadowedDeclaration(value: string | number) {
            function guard(input: unknown): input is number { return typeof input === "number"; }
            if (guard(value)) { /*localCallee*/value; }
        }
        function shadowedParameter(value: string | number, guard: (input: unknown) => input is string) {
            if (guard(value)) { /*parameterCallee*/value; }
        }
    "#;
    let (db, module) = guard_db(SOURCE);
    for (marker, expected) in [
        ("innerBefore", &[String, Number][..]),
        ("innerThen", &[String]),
        ("innerJoin", &[String, Number]),
        ("outer", &[String]),
        ("localCallee", &[Number]),
        ("parameterCallee", &[String, Number]),
    ] {
        let ty = normalized_type_at(&db, module, SOURCE, marker, "value");
        assert_variants(&db, ty, expected, marker);
    }
}

#[test]
fn let_const_var_written_and_captured_subjects_narrow() {
    use InferredTypeData::{Number, String};
    const SOURCE: &str = r#"
        declare function guard(input: unknown): input is string;
        declare function read(): string | number;
        function locals() {
            const value = read();
            if (guard(value)) { /*const*/value; }
            let other: string | number = read();
            if (guard(other)) { /*let*/other; }
            var hoisted: string | number = read();
            if (guard(hoisted)) { /*var*/hoisted; }
        }
        function written(value: string | number) {
            if (guard(value)) { value = 0; /*written*/value; }
        }
        function captured(value: string | number) {
            if (guard(value)) { return () => /*captured*/value; }
        }
        function insideTry(value: string | number) {
            try { if (guard(value)) { /*try*/value; } } finally {}
        }
    "#;
    let (db, module) = guard_db(SOURCE);
    for (marker, expression, expected) in [
        ("const", "value", &[String][..]),
        ("let", "other", &[String]),
        // TypeScript 7.0.2 (`tsc --strict`) narrows a `var` too.
        ("var", "hoisted", &[String]),
        // TypeScript 7.0.2 (`tsc --strict`) gives `number` after the write.
        ("written", "value", &[Number]),
        // TypeScript 7.0.2 (`tsc --strict`) narrows a read in a closure
        // created after the test.
        ("captured", "value", &[String]),
        // TypeScript 7.0.2 (`tsc --strict`) narrows the read inside the try
        // block.
        ("try", "value", &[String]),
    ] {
        let ty = normalized_type_at(&db, module, SOURCE, marker, expression);
        assert_variants(&db, ty, expected, marker);
    }
}

#[test]
fn unsupported_callees_signatures_and_arguments_do_not_refine() {
    use InferredTypeData::{Number, String};
    for (name, declaration, call) in [
        (
            "overloaded",
            "declare function guard(input: string): input is string; declare function guard(input: unknown): input is number;",
            "guard(value)",
        ),
        (
            "generic",
            "declare function guard<T>(input: unknown): input is string;",
            "guard(value)",
        ),
        (
            "explicit type arguments",
            "declare function guard(input: unknown): input is string;",
            "guard<unknown>(value)",
        ),
        (
            "call method",
            "declare function guard(input: unknown): input is string;",
            "guard.call(undefined, value)",
        ),
        (
            "apply method",
            "declare function guard(input: unknown): input is string;",
            "guard.apply(undefined, [value])",
        ),
        (
            "aliased",
            "declare function original(input: unknown): input is string; const guard = original;",
            "guard(value)",
        ),
        (
            "written let",
            "let guard = (input: unknown): input is string => typeof input === 'string'; guard = (input): input is string => false;",
            "guard(value)",
        ),
        (
            "async arrow",
            "const guard = async (input: unknown): input is string => typeof input === 'string';",
            "guard(value)",
        ),
        (
            "generator expression",
            "const guard = function*(input: unknown): input is string { yield 0; return typeof input === 'string'; };",
            "guard(value)",
        ),
        (
            "default arrow formal",
            "const guard = (input: unknown = ''): input is string => typeof input === 'string';",
            "guard(value)",
        ),
        (
            "method",
            "const owner = { guard(input: unknown): input is string { return typeof input === 'string'; } };",
            "owner.guard(value)",
        ),
        (
            "optional call",
            "declare function guard(input: unknown): input is string;",
            "guard?.(value)",
        ),
        (
            "async",
            "async function guard(input: unknown): input is string { return typeof input === 'string'; }",
            "guard(value)",
        ),
        (
            "generator",
            "function* guard(input: unknown): input is string { yield 0; return typeof input === 'string'; }",
            "guard(value)",
        ),
        (
            "written callee",
            "function guard(input: unknown): input is string { return typeof input === 'string'; } guard = (input): input is string => false;",
            "guard(value)",
        ),
        (
            "implicit predicate",
            "function guard(input: unknown) { return typeof input === 'string'; }",
            "guard(value)",
        ),
        (
            "spread before target",
            "declare function guard(ignored: unknown, input: unknown): input is string;",
            "guard(...[], value)",
        ),
        (
            "spread target",
            "declare function guard(input: unknown): input is string;",
            "guard(...[value])",
        ),
        (
            "missing target argument",
            "declare function guard(ignored: unknown, input: unknown): input is string;",
            "guard(value)",
        ),
        (
            "missing all arguments",
            "declare function guard(input: unknown): input is string;",
            "guard()",
        ),
        (
            "non-target argument",
            "declare function guard(ignored: unknown, input: unknown): input is string;",
            "guard(value, other)",
        ),
        (
            "non-identifier argument",
            "declare function guard(input: unknown): input is string;",
            "guard(value + '')",
        ),
        (
            "default formal",
            "function guard(input: unknown = ''): input is string { return typeof input === 'string'; }",
            "guard(value)",
        ),
        (
            "rest formal",
            "function guard(...input: unknown[]): input is string { return false; }",
            "guard(value)",
        ),
        (
            "destructured formal",
            "function guard({ input }: { input: unknown }): input is string { return typeof input === 'string'; }",
            "guard(value)",
        ),
        (
            "assertion predicate",
            "declare function guard(input: unknown): asserts input is string;",
            "guard(value)",
        ),
        (
            "truthiness assertion",
            "declare function guard(input: unknown): asserts input;",
            "guard(value)",
        ),
        (
            "this predicate",
            "function guard(this: unknown, input: unknown): this is string { return false; }",
            "guard(value)",
        ),
    ] {
        let source = format!(
            r#"
                {declaration}
                function inspect(value: string | number, other: unknown) {{
                    if ({call}) {{ /*then*/value; }} else {{ /*else*/value; }}
                    /*join*/value;
                }}
            "#
        );
        let (db, module) = guard_db(&source);
        for marker in ["then", "else", "join"] {
            let ty = normalized_type_at(&db, module, &source, marker, "value");
            assert_variants(&db, ty, &[String, Number], &format!("{name}: {marker}"));
        }
        assert_no_whole_module_inference(&db, &db.take_salsa_events());
    }
}

#[test]
fn guards_bound_to_variables_and_untested_parameters_narrow() {
    use InferredTypeData::{Number, String};
    for (name, declaration, call) in [
        (
            "arrow",
            "const guard = (input: unknown): input is string => typeof input === 'string';",
            "guard(value)",
        ),
        (
            "function expression",
            "const guard = function(input: unknown): input is string { return typeof input === 'string'; };",
            "guard(value)",
        ),
        (
            "unwritten let",
            "let guard = (input: unknown): input is string => typeof input === 'string';",
            "guard(value)",
        ),
        (
            "spread after target",
            "declare function guard(input: unknown): input is string;",
            "guard(value, ...[])",
        ),
        (
            "default before target",
            "function guard(ignored: unknown = '', input: unknown): input is string { return typeof input === 'string'; }",
            "guard(other, value)",
        ),
        (
            "rest after target",
            "function guard(input: unknown, ...ignored: unknown[]): input is string { return typeof input === 'string'; }",
            "guard(value)",
        ),
        (
            "destructured before target",
            "function guard({ ignored }: { ignored: unknown }, input: unknown): input is string { return typeof input === 'string'; }",
            "guard(other, value)",
        ),
    ] {
        let source = format!(
            r#"
                {declaration}
                function inspect(value: string | number, other: unknown) {{
                    if ({call}) {{ /*then*/value; }} else {{ /*else*/value; }}
                    /*join*/value;
                }}
            "#
        );
        let (db, module) = guard_db(&source);
        for (marker, expected) in [
            ("then", &[String][..]),
            ("else", &[Number]),
            ("join", &[String, Number]),
        ] {
            let ty = normalized_type_at(&db, module, &source, marker, "value");
            assert_variants(&db, ty, expected, &format!("{name}: {marker}"));
        }
        assert_no_whole_module_inference(&db, &db.take_salsa_events());
    }
}

#[test]
fn long_signatures_names_and_parenthesized_callees_narrow() {
    let parameters = (0..128)
        .map(|index| format!("p{index}?: unknown"))
        .collect::<Vec<_>>()
        .join(", ");
    let long_name = "x".repeat(1025);
    let parenthesized = format!("{}guard{}(value)", "(".repeat(33), ")".repeat(33));
    for (declaration, call) in [
        (
            format!("declare function guard(input: unknown, {parameters}): input is string;"),
            "guard(value)".to_owned(),
        ),
        (
            format!("declare function guard({long_name}: unknown): {long_name} is string;"),
            "guard(value)".to_owned(),
        ),
        (
            "declare function guard(input: unknown): input is string;".to_owned(),
            parenthesized,
        ),
    ] {
        let source = format!(
            "{declaration} function f(value: string | number) {{ if ({call}) {{ /*read*/value; }} }}"
        );
        let (db, module) = guard_db(&source);
        let ty = normalized_type_at(&db, module, &source, "read", "value");
        assert_variants(&db, ty, &[InferredTypeData::String], &source);
        assert_no_whole_module_inference(&db, &db.take_salsa_events());
    }
}

// TypeScript 7.0.2 (`tsc --strict`) gives every `then` read the target and
// every `else` read `unknown`, except that a target of `{}` gives
// `null | undefined` and `void` gives `{} | null` in the `else` branch, where
// Biome keeps `unknown`.
#[test]
fn predicate_targets_narrow_unknown_to_the_target() {
    for target in [
        "{}",
        "{ name: string }",
        "object",
        "string | number",
        "string & {}",
        "Promise<void>",
        "string[]",
        "void",
    ] {
        let source = format!(
            r#"
                declare function guard(input: unknown): input is {target};
                declare const expected: {target};
                /*expected*/expected;
                function inspect(value: unknown) {{
                    if (guard(value)) {{ /*then*/value; }} else {{ /*else*/value; }}
                }}
            "#
        );
        let (db, module) = guard_db(&source);
        let expected = normalized_type_at(&db, module, &source, "expected", "expected");
        assert_eq!(
            normalized_type_at(&db, module, &source, "then", "value"),
            expected,
            "{target}: then"
        );
        assert_eq!(
            normalized_type_at(&db, module, &source, "else", "value"),
            InferredTypeData::UnknownKeyword,
            "{target}: else"
        );
        assert_no_whole_module_inference(&db, &db.take_salsa_events());
    }
    // No value reaches the `then` branch of a `never` target.
    let source = r#"
        declare function guard(input: unknown): input is never;
        function inspect(value: unknown) {
            if (guard(value)) { /*then*/value; } else { /*else*/value; }
        }
    "#;
    let (db, module) = guard_db(source);
    for (marker, expected) in [
        ("then", InferredTypeData::Unknown),
        ("else", InferredTypeData::UnknownKeyword),
    ] {
        assert_eq!(
            normalized_type_at(&db, module, source, marker, "value"),
            expected,
            "never: {marker}"
        );
    }
}

// TypeScript 7.0.2 (`tsc --strict`) gives `any` in the `then` branch of an
// `any` target and `unknown` in the `then` branch of an `unknown` target.
// Both `else` branches are `never`. Biome supports neither target, so both
// branches keep the declared type.
#[test]
fn any_and_unknown_predicate_targets_preserve_unknown_and_indeterminate_classifications() {
    for target in ["any", "unknown"] {
        let source = format!(
            r#"
                declare function guard(input: unknown): input is {target};
                function inspect(value: unknown) {{
                    if (guard(value)) {{ /*then*/value; }} else {{ /*else*/value; }}
                }}
            "#
        );
        let (db, module) = guard_db(&source);
        for marker in ["then", "else"] {
            assert_eq!(
                normalized_type_at(&db, module, &source, marker, "value"),
                InferredTypeData::UnknownKeyword,
                "{target}: {marker}"
            );
            assert_eq!(
                execute_type_inference_request(
                    &db,
                    TypeInferenceCaller::new("test", "unsupportedGuard"),
                    PromiseClassificationRequest::new(
                        module,
                        marked_range(&source, marker, "value"),
                    ),
                ),
                TypeInferenceClassification::Indeterminate,
                "{target}: {marker}"
            );
        }
        assert_no_whole_module_inference(&db, &db.take_salsa_events());
    }
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

/// Types and type-guard functions shared by the tests of targets that are not
/// primitives.
const OBJECT_TARGETS: &str = r#"
    interface Foo { a: number }
    interface Foo2 extends Foo { b: number }
    interface Bar { c: number }
    interface IA { x: number }
    interface Same { a: number }
    class A { a = 1 }
    class A2 { a = 1; b = 2 }
    class B extends A { b = 2 }
    class C { c = 3 }
    class K { k = 1 }
    class L { length = 0 }
    type OC = { x: number; y?: string };
    type FooT = { a: number };
    type Len = { length: number };
    declare function isFoo(v: unknown): v is Foo;
    declare function isFooT(v: unknown): v is FooT;
    declare function isA(v: unknown): v is A;
    declare function isA1(v: unknown): v is { a: 1 };
    declare function isK(v: unknown): v is K;
    declare function isL(v: unknown): v is L;
    declare function isSN(v: unknown): v is string | number;
    declare function isOC(v: unknown): v is OC;
    declare function isMap(v: unknown): v is Map<string, number>;
    declare function isFooOrBar(v: unknown): v is Foo | Bar;
    declare function isStrOrFoo(v: unknown): v is string | Foo;
    declare function isLen(v: unknown): v is Len;
    declare function isObj(v: unknown): v is object;
    declare function isEmpty(v: unknown): v is {};
    declare function isFn(v: unknown): v is () => void;
    declare function isArr(v: unknown): v is string[];
    declare function isPromise(v: unknown): v is Promise<void>;
    declare function isError(v: unknown): v is Error;
    declare function isTypeError(v: unknown): v is TypeError;
    declare function isString(v: unknown): v is string;
    declare function isNever(v: unknown): v is never;
    declare function isAdd(v: unknown): v is { type: "add"; payload: number };
"#;

/// How [`variant_names`] names `() => void`.
const VOID_FUNCTION: &str = "sync Function { accepts: { params: [] type_args: [] } returns: void }";

/// Asserts that the type of each `(marker, expression, expected)` read in
/// `source` has the member names `expected`, sorted and spelled as
/// [`variant_names`] spells them. `source` follows [`OBJECT_TARGETS`] in the
/// module.
fn assert_reads(source: &str, reads: &[(&str, &str, &[&str])]) {
    let source = format!("{OBJECT_TARGETS}{source}");
    let (db, module) = guard_db(&source);
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
    assert_no_whole_module_inference(&db, &db.take_salsa_events());
}

// Biome records `readonly`, so two object types that differ only by it are
// different types. Its subtype relation ignores `readonly`, so it can't tell
// which of the two TypeScript's strict subtype relation prefers, and the
// `else` branch keeps both members of each pair. A comment gives the type
// that TypeScript 7.0.2 (`tsc --strict`) gives where it differs.
#[test]
fn object_targets_that_differ_only_by_readonly_keep_both_members_in_the_else_branch() {
    assert_reads(
        r#"
        declare function isFrozenA1(v: unknown): v is { readonly a: 1 };
        function r1(x: { a: 1 } | Bar) { if (isFrozenA1(x)) /*r1*/x; else /*r1Else*/x; }
        function r2(x: { readonly a: 1 } | Bar) { if (isA1(x)) /*r2*/x; else /*r2Else*/x; }
        "#,
        &[
            ("r1", "x", &[r#"["a": number: 1]"#]),
            // `Bar`
            ("r1Else", "x", &["Bar", r#"["a": number: 1]"#]),
            // `{ a: 1 }`
            ("r2", "x", &[r#"[readonly "a": number: 1]"#]),
            ("r2Else", "x", &["Bar", r#"[readonly "a": number: 1]"#]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) merges the interfaces of one name in one
// scope into one type, and a class with them, and gives every read below the
// expected type.
#[test]
fn object_targets_see_the_members_of_every_merged_declaration() {
    assert_reads(
        r#"
        interface M { m: 1 } interface M { n: 2 }
        declare function isM(v: unknown): v is { m: 1 };
        function merged(x: M | { m: 1; y: 2 } | string) { if (isM(x)) /*merged*/x; else /*mergedElse*/x; }
        interface Y1 { y: 1 } interface Y1 { z: 2 }
        declare function isY(v: unknown): v is { y: 1 };
        function first(q: Y1 | { y: 1; w: 2 } | string) { if (isY(q)) /*first*/q; else /*firstElse*/q; }
        class Kj { k = 1 } interface Kj { j: string }
        declare function isJ(v: unknown): v is { j: string };
        function classMember(q: Kj | string) { if (isJ(q)) /*classMember*/q; else /*classMemberElse*/q; }
        interface Base0 { b: string }
        class Lb { l = 1 } interface Lb extends Base0 {}
        declare function isB(v: unknown): v is { b: string };
        function classBase(q: Lb | string) { if (isB(q)) /*classBase*/q; else /*classBaseElse*/q; }
        "#,
        &[
            ("merged", "x", &["M", r#"["m": number: 1, "y": number: 2]"#]),
            ("mergedElse", "x", &["string"]),
            ("first", "q", &["Y1", r#"["y": number: 1, "w": number: 2]"#]),
            ("firstElse", "q", &["string"]),
            ("classMember", "q", &["instanceof Kj"]),
            ("classMemberElse", "q", &["string"]),
            ("classBase", "q", &["instanceof Lb"]),
            ("classBaseElse", "q", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// A branch that no value reaches is `never` in TypeScript, which Biome reads
// as Unknown.
#[test]
fn object_targets_narrow_both_branches() {
    assert_reads(
        r#"
        function f1(x: Foo | string) { if (isFoo(x)) /*f1*/x; else /*f1Else*/x; }
        function f2(x: Foo2 | Bar) { if (isFoo(x)) /*f2*/x; else /*f2Else*/x; }
        function f3(x: string | number | boolean) { if (isSN(x)) /*f3*/x; else /*f3Else*/x; }
        function f4(x: B | C) { if (isA(x)) /*f4*/x; else /*f4Else*/x; }
        function f5(x: { a: 1; b: 2 } | { c: 3 }) { if (isFoo(x)) /*f5*/x; else /*f5Else*/x; }
        function f7(x: unknown) { if (isFoo(x)) /*f7*/x; else /*f7Else*/x; }
        function f8(x: IA | Bar) { if (isOC(x)) /*f8*/x; else /*f8Else*/x; }
        function f9(x: Foo | undefined) { if (isFoo(x)) /*f9*/x; else /*f9Else*/x; }
        function f11(x: Map<string, number> | string) { if (isMap(x)) /*f11*/x; else /*f11Else*/x; }
        function f14(x: { a: any } | Bar) { if (isFoo(x)) /*f14*/x; else /*f14Else*/x; }
        function f15(x: { a: number; b?: string } | Bar) { if (isFoo(x)) /*f15*/x; else /*f15Else*/x; }
        function f16(x: object) { if (isFoo(x)) /*f16*/x; else /*f16Else*/x; }
        function f17(x: {}) { if (isFoo(x)) /*f17*/x; else /*f17Else*/x; }
        function f19(x: Foo2 | string) { if (!isFoo(x)) /*f19*/x; }
        function f21(x: { a: 1 | 2 } | Bar) { if (isFoo(x)) /*f21*/x; else /*f21Else*/x; }
        function g1(x: Foo | Bar | string) { if (isFooOrBar(x)) /*g1*/x; else /*g1Else*/x; }
        function g2(x: K | string) { if (isK(x)) /*g2*/x; else /*g2Else*/x; }
        function g4(x: string | number | Foo) { if (isLen(x)) /*g4*/x; else /*g4Else*/x; }
        function g5(x: string | Foo | number) { if (isStrOrFoo(x)) /*g5*/x; else /*g5Else*/x; }
        function g7(x: FooT | Bar) { if (isFooT(x)) /*g7*/x; else /*g7Else*/x; }
        function g8(x: Error | string) { if (isError(x)) /*g8*/x; else /*g8Else*/x; }
        function g11(x: (() => void) | string) { if (isFn(x)) /*g11*/x; else /*g11Else*/x; }
        function g12(x: string[] | string) { if (isArr(x)) /*g12*/x; else /*g12Else*/x; }
        function g13(x: Promise<void> | null) { if (isPromise(x)) /*g13*/x; else /*g13Else*/x; }
        function g26(x: Foo | string) { if (isNever(x)) /*g26*/x; else /*g26Else*/x; }
        function g28(x: unknown) { if (isFooOrBar(x)) /*g28*/x; else /*g28Else*/x; }
        function g29(x: Foo | string) { if (isObj(x)) /*g29*/x; else /*g29Else*/x; }
        function g30(x: Foo | null | undefined) { if (isEmpty(x)) /*g30*/x; else /*g30Else*/x; }
        function g33() { const o = { a: 1, b: 2 }; if (isFoo(o)) /*g33*/o; else /*g33Else*/o; }
        function w3() { const action = { type: "add", payload: 1 }; if (isAdd(action)) /*w3*/action; else /*w3Else*/action; }
        "#,
        &[
            ("f1", "x", &["Foo"]),
            ("f1Else", "x", &["string"]),
            // Biome matches TypeScript here, although it is unsound. `Bar` is
            // removed, but a `Bar` that also has `a` is a `Foo`.
            ("f2", "x", &["Foo2"]),
            ("f2Else", "x", &["Bar"]),
            ("f3", "x", &["number", "string"]),
            ("f3Else", "x", &["boolean"]),
            ("f4", "x", &["instanceof B"]),
            ("f4Else", "x", &["instanceof C"]),
            ("f5", "x", &[r#"["a": number: 1, "b": number: 2]"#]),
            ("f5Else", "x", &[r#"["c": number: 3]"#]),
            ("f7", "x", &["Foo"]),
            ("f7Else", "x", &["unknown"]),
            // `OC` replaces `IA`, which lacks its optional property `y`.
            ("f8", "x", &[r#"["x": number, "y"?: string | undefined]"#]),
            ("f8Else", "x", &["Bar", "IA"]),
            ("f9", "x", &["Foo"]),
            ("f9Else", "x", &["undefined"]),
            ("f11", "x", &["instanceof Map<string, number>"]),
            ("f11Else", "x", &["string"]),
            // `any` is a subtype only of `any` and `unknown`, so `Foo`
            // replaces `{ a: any }`, which the `else` branch keeps.
            ("f14", "x", &["Foo"]),
            ("f14Else", "x", &["Bar", r#"["a": any]"#]),
            ("f15", "x", &[r#"["a": number, "b"?: string | undefined]"#]),
            ("f15Else", "x", &["Bar"]),
            ("f16", "x", &["Foo"]),
            ("f16Else", "x", &["object"]),
            ("f17", "x", &["Foo"]),
            ("f17Else", "x", &["[]"]),
            ("f19", "x", &["string"]),
            ("f21", "x", &[r#"["a": number: 1 | number: 2]"#]),
            ("f21Else", "x", &["Bar"]),
            ("g1", "x", &["Bar", "Foo"]),
            ("g1Else", "x", &["string"]),
            ("g2", "x", &["instanceof K"]),
            ("g2Else", "x", &["string"]),
            // Every string has `length`, so `string` is a subtype of `Len`.
            ("g4", "x", &["string"]),
            ("g4Else", "x", &["Foo", "number"]),
            ("g5", "x", &["Foo", "string"]),
            ("g5Else", "x", &["number"]),
            ("g7", "x", &[r#"["a": number]"#]),
            ("g7Else", "x", &["Bar"]),
            ("g8", "x", &["instanceof Error"]),
            ("g8Else", "x", &["string"]),
            ("g11", "x", &[VOID_FUNCTION]),
            ("g11Else", "x", &["string"]),
            ("g12", "x", &["instanceof Array<string>"]),
            ("g12Else", "x", &["string"]),
            ("g13", "x", &["instanceof Promise<void>"]),
            ("g13Else", "x", &["null"]),
            ("g26", "x", &["unknown"]),
            ("g26Else", "x", &["Foo", "string"]),
            ("g28", "x", &["Bar", "Foo"]),
            ("g28Else", "x", &["unknown"]),
            ("g29", "x", &["Foo"]),
            ("g29Else", "x", &["string"]),
            ("g30", "x", &["Foo"]),
            ("g30Else", "x", &["null", "undefined"]),
            // The object literal's properties have the widened literal types
            // of their values, so `o` is a subtype of `Foo`.
            ("g33", "o", &[r#"["a": number, "b": number]"#]),
            ("g33Else", "o", &["unknown"]),
            // The target is a subtype of `{ type: string; payload: number }`.
            (
                "w3",
                "action",
                &[r#"["type": string: add, "payload": number]"#],
            ),
            (
                "w3Else",
                "action",
                &[r#"["type": string, "payload": number]"#],
            ),
        ],
    );
}

// Biome keeps the declared type in both branches of every read below, or in
// the `else` branch where it narrows only the `then` branch. A comment gives
// the type that TypeScript 7.0.2 (`tsc --strict`) gives where it differs.
#[test]
fn object_targets_keep_the_declared_type_where_biome_cannot_tell() {
    assert_reads(
        r#"
        function f6(x: string | number) { if (isFoo(x)) /*f6*/x; else /*f6Else*/x; }
        function f12(x: { a: number } | Bar) { if (isFoo(x)) /*f12*/x; else /*f12Else*/x; }
        function f13(x: A | C) { if (isFoo(x)) /*f13*/x; else /*f13Else*/x; }
        function f18(x: Foo | Bar) { if (isSN(x)) /*f18*/x; else /*f18Else*/x; }
        function f20(x: Same | Bar) { if (isFoo(x)) /*f20*/x; else /*f20Else*/x; }
        function g3(x: L | string) { if (isL(x)) /*g3*/x; else /*g3Else*/x; }
        function g9(x: TypeError | string) { if (isError(x)) /*g9*/x; else /*g9Else*/x; }
        function g10(x: Error | string) { if (isTypeError(x)) /*g10*/x; else /*g10Else*/x; }
        function g27(x: { a: number } | string) { if (isA(x)) /*g27*/x; else /*g27Else*/x; }
        function w1(x: { a: 1 } | string) { if (isA(x)) /*w1*/x; else /*w1Else*/x; }
        function w2(x: A2 | string) { if (isA1(x)) /*w2*/x; else /*w2Else*/x; }
        "#,
        &[
            // `(string | number) & Foo`
            ("f6", "x", &["number", "string"]),
            ("f6Else", "x", &["number", "string"]),
            // TypeScript's strict subtype relation keeps `{ a: number }` and
            // `A` as they are. Biome can't tell, because that relation depends
            // on modifiers such as `readonly`, so the `else` branch keeps them.
            ("f12", "x", &[r#"["a": number]"#]),
            // `Bar`
            ("f12Else", "x", &["Bar", r#"["a": number]"#]),
            ("f13", "x", &["instanceof A"]),
            // `C`
            ("f13Else", "x", &["instanceof A", "instanceof C"]),
            // `(Foo | Bar) & (string | number)`
            ("f18", "x", &["Bar", "Foo"]),
            ("f18Else", "x", &["Bar", "Foo"]),
            ("f20", "x", &["Same"]),
            // `Bar`
            ("f20Else", "x", &["Bar", "Same"]),
            // A class can declare private members, so Biome can't tell
            // whether `string` is a subtype of `L`.
            ("g3", "x", &["instanceof L", "string"]),
            // `never`
            ("g3Else", "x", &["instanceof L", "string"]),
            // Biome compares instances of two built-in classes only as the
            // same class. `TypeError`
            ("g9", "x", &["instanceof TypeError", "string"]),
            // `string`
            ("g9Else", "x", &["instanceof TypeError", "string"]),
            // `Error`
            ("g10", "x", &["instanceof Error", "string"]),
            // `string`
            ("g10Else", "x", &["instanceof Error", "string"]),
            ("g27", "x", &[r#"["a": number]"#]),
            // `string`
            ("g27Else", "x", &[r#"["a": number]"#, "string"]),
            // Biome types the field of `A` and the properties of the object
            // literal `action` with the literal types of their initializers.
            // TypeScript widens those to `number` and `string`, so Biome can't
            // tell whether one type is a subtype of the other.
            // `{ a: 1 }`
            ("w1", "x", &[r#"["a": number: 1]"#, "string"]),
            // `string`
            ("w1Else", "x", &[r#"["a": number: 1]"#, "string"]),
            // `(string | A2) & { a: 1 }`
            ("w2", "x", &["instanceof A2", "string"]),
            ("w2Else", "x", &["instanceof A2", "string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
#[test]
fn object_targets_narrow_through_the_flow() {
    assert_reads(
        r#"
        const isFooArrow = (v: unknown): v is Foo => typeof v === "object";
        const isFooExpression = function (v: unknown): v is Foo { return true; };
        function arrow(x: Foo | string) { if (isFooArrow(x)) /*arrow*/x; else /*arrowElse*/x; }
        function expression(x: Foo | string) { if (isFooExpression(x)) /*expression*/x; else /*expressionElse*/x; }
        function path(o: { v: Foo | string }) { if (isFoo(o.v)) /*path*/o.v; else /*pathElse*/o.v; }
        function aliased(x: Foo | string) { const ok = isFoo(x); if (ok) /*aliased*/x; else /*aliasedElse*/x; }
        function operands(x: Foo | string) { isFoo(x) && /*and*/x; isFoo(x) || /*or*/x; }
        function conditional(x: Foo | string) { isFoo(x) ? /*conditional*/x : /*conditionalElse*/x; }
        function returned(x: Foo | string) { if (!isFoo(x)) return; /*returned*/x; }
        function loop(x: Foo | string) { while (isFoo(x)) { /*loop*/x; } /*loopExit*/x; }
        function both(x: Foo | Bar | string) { if (isFooOrBar(x) && !isFoo(x)) /*both*/x; }
        function either(x: Foo | Bar | string) { if (isFoo(x) || typeof x === "string") /*either*/x; else /*eitherElse*/x; }
        function nested(x: Foo | string | null) { if (x) { if (isFoo(x)) /*nested*/x; else /*nestedElse*/x; } }
        function two(x: Foo | string, y: B | C) { if (isFoo(x) && isA(y)) { /*twoX*/x; /*twoY*/y; } }
        function member(x: Foo | string) { if (isFoo(x)) /*member*/x.a; }
        "#,
        &[
            ("arrow", "x", &["Foo"]),
            ("arrowElse", "x", &["string"]),
            ("expression", "x", &["Foo"]),
            ("expressionElse", "x", &["string"]),
            ("path", "o.v", &["Foo"]),
            ("pathElse", "o.v", &["string"]),
            ("aliased", "x", &["Foo"]),
            ("aliasedElse", "x", &["string"]),
            ("and", "x", &["Foo"]),
            ("or", "x", &["string"]),
            ("conditional", "x", &["Foo"]),
            ("conditionalElse", "x", &["string"]),
            ("returned", "x", &["Foo"]),
            ("loop", "x", &["Foo"]),
            ("loopExit", "x", &["string"]),
            ("both", "x", &["Bar"]),
            ("either", "x", &["Foo", "string"]),
            ("eitherElse", "x", &["Bar"]),
            ("nested", "x", &["Foo"]),
            ("nestedElse", "x", &["string"]),
            ("twoX", "x", &["Foo"]),
            ("twoY", "y", &["instanceof B"]),
            ("member", "x.a", &["number"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type.
// Its union of `"a"` and `string`, the types that the `then` branch gets, is
// `string`, so the `else` branch keeps `"a"`.
#[test]
fn primitive_targets_keep_the_literals_that_the_then_branch_absorbs() {
    assert_reads(
        r#"
        function absorbed(x: "a" | Len) { if (isString(x)) /*absorbed*/x; else /*absorbedElse*/x; }
        function literals(x: "a" | number) { if (isString(x)) /*literals*/x; else /*literalsElse*/x; }
        "#,
        &[
            ("absorbed", "x", &["string"]),
            ("absorbedElse", "x", &[r#"["length": number]"#, "string: a"]),
            ("literals", "x", &["string: a"]),
            ("literalsElse", "x", &["number"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type,
// except where a comment gives TypeScript's type.
#[test]
fn object_targets_resolve_types_of_other_modules() {
    let files = [
        (
            "/src/shapes.ts",
            "export interface Foo { a: number }
export type Bar = { c: number };
export class K { k = 1 }
",
        ),
        (
            "/src/copy.ts",
            "export class K { k = 1 }
",
        ),
        (
            "/src/index.ts",
            r#"import type { Foo, Bar } from "./shapes";
import { K } from "./shapes";
import { K as KC } from "./copy";
declare function isFoo(v: unknown): v is Foo;
declare function isBar(v: unknown): v is Bar;
declare function isK(v: unknown): v is K;
declare function isCopy(v: unknown): v is KC;
function foo(x: Foo | string) { if (isFoo(x)) /*foo*/x; else /*fooElse*/x; }
function bar(x: Bar | string) { if (isBar(x)) /*bar*/x; else /*barElse*/x; }
function k(x: K | string) { if (isK(x)) /*k*/x; else /*kElse*/x; }
function copy(x: K | string) { if (isCopy(x)) /*copy*/x; else /*copyElse*/x; }
"#,
        ),
    ];
    let fs = MemoryFileSystem::default();
    for (path, source) in files {
        fs.insert(path.into(), source);
    }
    let paths: Vec<_> = files.iter().map(|(path, _)| *path).collect();
    let db = build_js_test_module_db(&fs, &paths, true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    let source = files[2].1;
    for (marker, expected) in [
        ("foo", &["Foo"][..]),
        ("fooElse", &["string"]),
        ("bar", &[r#"["c": number]"#]),
        ("barElse", &["string"]),
        ("k", &["instanceof K"]),
        ("kElse", &["string"]),
        // Two classes with the same name and members in two modules can't be
        // told from one class that Biome reaches through two modules, so the
        // read keeps its type. `K`
        ("copy", &["instanceof K", "string"]),
        // `string`
        ("copyElse", &["instanceof K", "string"]),
    ] {
        let ty = normalized_type_at(&db, module, source, marker, "x");
        assert_eq!(variant_names(&db, ty), expected, "{marker}");
    }
    assert_no_whole_module_inference(&db, &db.take_salsa_events());
}

// In Biome, a type predicate whose target has only object types can't narrow
// a value whose declared type has no object type. So the target `Local`,
// which extends `Foo` from another module, is never resolved for such a
// value, even when another test of the same value is solved.
#[test]
fn object_targets_are_not_resolved_for_values_without_object_types() {
    const SOURCE: &str = r#"
        import type { Foo } from "./foo.ts";
        interface Local extends Foo { b: number }
        type Primitive = string | number;
        declare function isLocal(input: unknown): input is Local;
        function only(value: Primitive) {
            if (isLocal(value)) { /*only*/value; }
        }
        function other(value: Primitive | null) {
            if (value !== null && isLocal(value)) { /*other*/value; }
        }
        function object(value: Local | string) {
            if (isLocal(value)) { /*object*/value; }
        }
    "#;
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);
    fs.insert("/src/foo.ts".into(), "export interface Foo { a: number }");
    let db = build_js_test_module_db(&fs, &["/src/index.ts", "/src/foo.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    use InferredTypeData::{Number, String};
    for (marker, solves) in [("only", 0), ("other", 1)] {
        db.clear_salsa_events();
        let ty = normalized_type_at(&db, module, SOURCE, marker, "value");
        assert_variants(&db, ty, &[String, Number], marker);
        let events = db.take_salsa_events();
        assert_eq!(
            function_query_will_execute_count_by_name(&db, "infer_flow_subject_states", &events),
            solves,
            "{marker}"
        );
        assert_eq!(
            function_query_will_execute_count_by_name(&db, "resolved_export_origin", &events),
            0,
            "{marker}"
        );
        assert_no_whole_module_inference(&db, &events);
    }
    let ty = normalized_type_at(&db, module, SOURCE, "object", "value");
    assert_eq!(variant_names(&db, ty), ["Local"]);
}

#[test]
fn alias_targets_narrow_like_the_types_they_name() {
    use InferredTypeData::{Number, String};
    for target in ["Alias", "Nested"] {
        let source = format!(
            r#"
                type Alias = string;
                type Nested = Alias;
                declare function guard(input: unknown): input is {target};
                function inspect(value: string | number) {{
                    if (guard(value)) {{ /*then*/value; }} else {{ /*else*/value; }}
                }}
            "#
        );
        let (db, module) = guard_db(&source);
        for (marker, expected) in [("then", String), ("else", Number)] {
            assert_eq!(
                normalized_type_at(&db, module, &source, marker, "value"),
                expected,
                "{target}: {marker}"
            );
        }
    }
}

#[test]
fn typeof_targets_narrow_like_the_values_they_query() {
    for (target, literal) in [
        ("typeof READY", r#""ready""#),
        ("typeof config.mode", r#""strict""#),
    ] {
        let source = format!(
            r#"
                const READY = "ready";
                const config = {{ mode: "strict" as const }};
                declare function guard(input: unknown): input is {target};
                declare const expectedTarget: {literal};
                /*target*/expectedTarget;
                function inspect(value: {literal} | null) {{
                    if (guard(value)) {{ /*then*/value; }} else {{ /*else*/value; }}
                }}
            "#
        );
        let (db, module) = guard_db(&source);
        let expected_target = normalized_type_at(&db, module, &source, "target", "expectedTarget");
        for (marker, expected) in [("then", expected_target), ("else", InferredTypeData::Null)] {
            let actual = normalized_type_at(&db, module, &source, marker, "value");
            assert_eq!(
                actual,
                expected,
                "{target}: {marker}: actual {}, expected {}",
                format_inferred_type(&db, actual),
                format_inferred_type(&db, expected),
            );
        }
    }
}

// TypeScript 7.0.2 (`tsc --strict`) narrows both reads to `string`, although
// a direct `eval` can reassign the guard or set `value` to `null`. Biome
// deliberately copies this unsound narrowing; `NARROWING.md` lists it among
// the deliberate choices.
#[test]
fn guards_narrow_in_modules_that_call_eval() {
    for declaration in [
        "let guard = (input: unknown): input is string => typeof input === 'string';",
        "function guard(input: unknown): input is string { return typeof input === 'string'; }",
    ] {
        let source = format!(
            r#"
                {declaration}
                function loosen() {{ eval("guard = () => true"); }}
                function inspect(value: string | null) {{
                    if (guard(value)) {{ /*then*/value; }}
                }}
                function dynamic(value: string | null) {{
                    if (guard(value)) {{ eval("value = null"); /*eval*/value; }}
                }}
            "#
        );
        let (db, module) = guard_db(&source);
        for marker in ["then", "eval"] {
            let ty = normalized_type_at(&db, module, &source, marker, "value");
            assert_variants(
                &db,
                ty,
                &[InferredTypeData::String],
                &format!("{declaration}: {marker}"),
            );
        }
    }
}

#[test]
fn imported_guards_do_not_refine_even_when_the_export_is_available() {
    use InferredTypeData::{Number, String};
    const SOURCE: &str = r#"
        import { guard } from "./guard.ts";
        function inspect(value: string | number) {
            if (guard(value)) { /*then*/value; } else { /*else*/value; }
        }
    "#;
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);
    fs.insert(
        "/src/guard.ts".into(),
        "export function guard(input: unknown): input is string { return typeof input === 'string'; }",
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts", "/src/guard.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    for marker in ["then", "else"] {
        let ty = normalized_type_at(&db, module, SOURCE, marker, "value");
        assert_variants(&db, ty, &[String, Number], marker);
    }
    assert_no_whole_module_inference(&db, &db.take_salsa_events());
}

#[test]
fn unchecked_predicate_calls_do_not_refine_following_reads() {
    const SOURCE: &str = r#"
        declare function isString(input: unknown): input is string;
        function inspect(value: string | number) {
            isString(value);
            /*unchecked*/value;
        }
    "#;
    let (db, module) = guard_db(SOURCE);
    let ty = normalized_type_at(&db, module, SOURCE, "unchecked", "value");
    assert_variants(
        &db,
        ty,
        &[InferredTypeData::String, InferredTypeData::Number],
        "unchecked",
    );
}

#[test]
fn promise_classifiers_observe_guarded_reads_and_nested_call_arguments() {
    use TypeInferenceClassification::{Match, NoMatch};
    const SOURCE: &str = r#"
        declare function isString(input: unknown): input is string;
        declare function isNull(input: unknown): input is null;
        declare function identity<T>(input: T): T;
        function promise(value: Promise<void> | string) {
            if (isString(value)) {
                /*string*/value;
                /*stringCall*/identity(value);
            } else {
                /*promise*/value;
                /*promiseCall*/identity(value);
            }
        }
        function array(value: Promise<void>[] | null) {
            if (isNull(value)) { /*nullArray*/value; } else { /*array*/value; }
        }
        function callback(value: (() => Promise<void>) | null) {
            if (isNull(value)) { /*nullCallback*/value; } else { /*callback*/value; }
        }
    "#;
    let (db, module) = guard_db(SOURCE);
    for (marker, expression, expected) in [
        ("string", "value", NoMatch),
        ("stringCall", "identity(value)", NoMatch),
        ("promise", "value", Match),
        ("promiseCall", "identity(value)", Match),
    ] {
        let range = marked_range(SOURCE, marker, expression);
        let input = ExpressionTypeInput::new(&db, module, range);
        for cold in [true, false] {
            db.clear_salsa_events();
            assert_eq!(
                execute_type_inference_request(
                    &db,
                    TypeInferenceCaller::new("test", "guardedPromise"),
                    PromiseClassificationRequest::new(module, range),
                ),
                expected,
                "{marker}"
            );
            let events = db.take_salsa_events();
            if cold {
                assert_function_query_was_run(&db, infer_expression_is_promise, input, &events);
            } else {
                assert_function_query_was_not_run(&db, infer_expression_is_promise, input, &events);
            }
            assert_no_whole_module_inference(&db, &events);
        }
    }
    for (marker, expected) in [("nullArray", NoMatch), ("array", Match)] {
        assert_eq!(
            execute_type_inference_request(
                &db,
                TypeInferenceCaller::new("test", "guardedPromiseArray"),
                ArrayOfPromisesClassificationRequest::new(
                    module,
                    marked_range(SOURCE, marker, "value"),
                ),
            ),
            expected,
            "{marker}"
        );
    }
    for (marker, expected) in [("nullCallback", NoMatch), ("callback", Match)] {
        assert_eq!(
            execute_type_inference_request(
                &db,
                TypeInferenceCaller::new("test", "guardedPromiseCallback"),
                PromiseReturningFunctionClassificationRequest::new(
                    module,
                    marked_range(SOURCE, marker, "value"),
                ),
            ),
            expected,
            "{marker}"
        );
    }
    assert_no_whole_module_inference(&db, &db.take_salsa_events());
}

#[test]
fn heavily_referenced_guards_narrow() {
    for count in [1, 2048] {
        let source = format!(
            r#"
            declare function guard(input: unknown): input is string;
            function unrelated() {{ {} }}
            function inspect(value: string | number) {{
                if (guard(value)) {{ /*read*/value; }}
            }}
        "#,
            "guard(null);".repeat(count - 1)
        );
        let (db, module) = guard_db(&source);
        let ty = normalized_type_at(&db, module, &source, "read", "value");
        assert_variants(&db, ty, &[InferredTypeData::String], &count.to_string());
        assert_no_whole_module_inference(&db, &db.take_salsa_events());
    }
}

#[test]
fn guard_projection_does_not_resolve_unrelated_signature_types() {
    const SOURCE: &str = r#"
        import type { Unrelated } from "./unrelated.ts";
        declare function guard(ignored: Unrelated | undefined, input: unknown): input is string;
        function inspect(value: string | number) {
            if (guard(undefined, value)) { /*read*/value; }
        }
    "#;
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);
    fs.insert(
        "/src/unrelated.ts".into(),
        "export interface Unrelated { member: string }",
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts", "/src/unrelated.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    db.clear_salsa_events();
    assert_eq!(
        normalized_type_at(&db, module, SOURCE, "read", "value"),
        InferredTypeData::String
    );
    let events = db.take_salsa_events();
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "infer_export_type", &events),
        0
    );
    assert_no_whole_module_inference(&db, &events);
}

#[test]
fn guard_requests_reuse_warm_and_unrelated_modules_but_recompute_consumed_predicates() {
    use InferredTypeData::{Number, String};
    const SOURCE: &str = r#"
        declare function guard(input: unknown): input is string;
        function inspect(value: string | number) {
            if (guard(value)) { /*read*/value; }
        }
    "#;
    let changed_source = SOURCE.replace("input is string", "input is number");
    let expression = marked_range(SOURCE, "read", "value");
    assert_eq!(expression, marked_range(&changed_source, "read", "value"));
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);
    fs.insert("/src/unrelated.ts".into(), "export const unused = 1;");
    let mut db = build_js_test_module_db(&fs, &["/src/index.ts", "/src/unrelated.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    let unrelated = db
        .module_for_path(Utf8Path::new("/src/unrelated.ts"))
        .unwrap();

    for cold in [true, false] {
        db.clear_salsa_events();
        assert_eq!(
            normalized_type_at(&db, module, SOURCE, "read", "value"),
            String
        );
        let events = db.take_salsa_events();
        let input = ExpressionTypeInput::new(&db, module, expression);
        if cold {
            assert_function_query_was_run(&db, infer_expression_type, input, &events);
        } else {
            assert_function_query_was_not_run(&db, infer_expression_type, input, &events);
        }
        assert_no_whole_module_inference(&db, &events);
    }

    fs.insert("/src/unrelated.ts".into(), "export const unused = false;");
    let kind = resolve_js_module_kind_for_test(&fs, "/src/unrelated.ts", true);
    salsa::Setter::to(unrelated.set_kind(&mut db), kind);
    db.clear_salsa_events();
    assert_eq!(
        normalized_type_at(&db, module, SOURCE, "read", "value"),
        String
    );
    let events = db.take_salsa_events();
    let input = ExpressionTypeInput::new(&db, module, expression);
    assert_function_query_was_not_run(&db, infer_expression_type, input, &events);
    assert_no_whole_module_inference(&db, &events);

    fs.insert("/src/index.ts".into(), changed_source.as_str());
    let kind = resolve_js_module_kind_for_test(&fs, "/src/index.ts", true);
    salsa::Setter::to(module.set_kind(&mut db), kind);
    db.clear_salsa_events();
    assert_eq!(
        normalized_type_at(&db, module, &changed_source, "read", "value"),
        Number
    );
    let events = db.take_salsa_events();
    let input = ExpressionTypeInput::new(&db, module, expression);
    assert_function_query_was_run(&db, infer_expression_type, input, &events);
    assert_no_whole_module_inference(&db, &events);
}
