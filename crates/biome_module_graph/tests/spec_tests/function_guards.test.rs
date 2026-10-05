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

#[test]
fn literal_guards_do_not_exclude_individual_values_from_broad_primitives() {
    use InferredTypeData::{BigInt, Boolean, Number, String};
    for (target, primitive, expected) in [
        (r#""ready""#, "string", String),
        ("true", "boolean", Boolean),
        ("1", "number", Number),
        ("1n", "bigint", BigInt),
    ] {
        let source = format!(
            r#"
                declare function guard(input: unknown): input is {target};
                declare const expectedTarget: {target};
                /*target*/expectedTarget;
                function inspect(value: {primitive}) {{
                    if (guard(value)) {{ /*then*/value; }} else {{ /*else*/value; }}
                }}
            "#
        );
        let (db, module) = guard_db(&source);
        let expected_target = normalized_type_at(&db, module, &source, "target", "expectedTarget");
        assert_eq!(
            normalized_type_at(&db, module, &source, "then", "value"),
            expected_target,
            "{primitive}: then"
        );
        assert_eq!(
            normalized_type_at(&db, module, &source, "else", "value"),
            expected,
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
fn unwritten_let_and_const_subjects_narrow_but_writes_captures_and_var_do_not() {
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
        function unsupportedRoot(value: string | number) {
            try { if (guard(value)) { /*try*/value; } } finally {}
        }
        function dynamic(value: string | number) {
            if (guard(value)) { eval("value = 0"); /*eval*/value; }
        }
    "#;
    let (db, module) = guard_db(SOURCE);
    for (marker, expression, expected) in [
        ("const", "value", &[String][..]),
        ("let", "other", &[String]),
        ("var", "hoisted", &[String, Number]),
        ("written", "value", &[String, Number]),
        ("captured", "value", &[String, Number]),
        ("try", "value", &[String, Number]),
        ("eval", "value", &[String, Number]),
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
            "spread after target",
            "declare function guard(input: unknown): input is string;",
            "guard(value, ...[])",
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
fn guard_selection_limits_do_not_publish_refinements() {
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
        assert_variants(
            &db,
            ty,
            &[InferredTypeData::String, InferredTypeData::Number],
            "limited",
        );
        assert_no_whole_module_inference(&db, &db.take_salsa_events());
    }
}

#[test]
fn unsupported_predicate_targets_preserve_unknown_and_indeterminate_classifications() {
    for target in [
        "{}",
        "{ name: string }",
        "object",
        "string | number",
        "string & {}",
        "Alias",
        "Promise<void>",
        "string[]",
        "any",
        "unknown",
        "void",
        "never",
    ] {
        let source = format!(
            r#"
                type Alias = string;
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
fn heavily_referenced_guards_have_a_bounded_write_proof() {
    for count in [1024, 1025] {
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
        let expected = if count == 1024 {
            &[InferredTypeData::String][..]
        } else {
            &[InferredTypeData::String, InferredTypeData::Number][..]
        };
        assert_variants(&db, ty, expected, &count.to_string());
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
