use super::*;
use biome_module_graph::{
    BindingTypeInput, ExpressionTypeInput, infer_binding_type, infer_expression_type,
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

fn assertion_db(source: &str) -> (TestModuleDb, ModuleInfo) {
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
        TypeInferenceCaller::new("test", "assertions"),
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
        "targeted assertion requests must not infer complete modules"
    );
}

#[test]
fn standalone_assertions_refine_after_all_arguments_without_changing_declarations() {
    use InferredTypeData::{Number, String};
    for declaration in [
        "function assertString(input: unknown, ignored?: unknown): asserts input is string { if (typeof input !== 'string') throw new Error(); }",
        "declare function assertString(input: unknown, ignored?: unknown): asserts input is string;",
    ] {
        let source = format!(
            r#"
                {declaration}
                declare function identity<T>(input: T): T;
                function inspect(/*binding*/value: string | number) {{
                    /*before*/value;
                    (((assertString)((/*argument*/value), /*laterArgument*/value)));
                    /*after*/value;
                    /*laterCall*/identity(value);
                }}
                function nested(value: string | number, other: string | number) {{
                    assertString(value, assertString(other, /*nestedArgument*/value));
                    /*outerTarget*/value;
                    /*nestedTarget*/other;
                }}
            "#
        );
        let (db, module) = assertion_db(&source);
        for (marker, expression, expected) in [
            ("before", "value", &[String, Number][..]),
            ("argument", "value", &[String, Number]),
            ("laterArgument", "value", &[String, Number]),
            ("after", "value", &[String]),
            ("laterCall", "identity(value)", &[String]),
            ("nestedArgument", "value", &[String, Number]),
            ("outerTarget", "value", &[String]),
            ("nestedTarget", "other", &[String, Number]),
        ] {
            let ty = normalized_type_at(&db, module, &source, marker, expression);
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
fn typed_assertions_preserve_falsy_targets_while_bare_assertions_filter_truthiness() {
    for (target, input, truthy) in [
        ("string", "string | null", "string"),
        ("number", "number | null", "number"),
        ("boolean", "boolean | null", "true"),
        ("bigint", "bigint | null", "bigint"),
        ("symbol", "symbol | null", "symbol"),
        ("null", "null | 'ready'", "'ready'"),
        ("undefined", "undefined | 'ready'", "'ready'"),
        ("false", "false | 'ready'", "'ready'"),
        ("true", "boolean", "true"),
        ("0", "0 | 1", "1"),
        ("''", "'' | 'ready'", "'ready'"),
        ("-1", "-1 | 0", "-1"),
        ("0n", "0n | 1n", "1n"),
        ("-1n", "-1n | 0n", "-1n"),
    ] {
        let source = format!(
            r#"
                declare function assertTyped(input: unknown): asserts input is {target};
                declare function assertTruthy(condition: unknown): asserts condition;
                declare const expectedTyped: {target};
                declare const expectedTruthy: {truthy};
                /*expectedTyped*/expectedTyped;
                /*expectedTruthy*/expectedTruthy;
                function typed(value: {input}) {{
                    assertTyped(value);
                    /*typed*/value;
                }}
                function bare(value: {input}) {{
                    assertTruthy((value));
                    /*bare*/value;
                }}
                function unknownTyped(value: unknown) {{
                    assertTyped(value);
                    /*unknownTyped*/value;
                }}
                function unknownBare(value: unknown) {{
                    assertTruthy(value);
                    /*unknownBare*/value;
                }}
            "#
        );
        let (db, module) = assertion_db(&source);
        let typed = normalized_type_at(&db, module, &source, "expectedTyped", "expectedTyped");
        let truthy = normalized_type_at(&db, module, &source, "expectedTruthy", "expectedTruthy");
        for (marker, expected) in [
            ("typed", typed),
            ("bare", truthy),
            ("unknownTyped", typed),
            ("unknownBare", InferredTypeData::UnknownKeyword),
        ] {
            assert_eq!(
                normalized_type_at(&db, module, &source, marker, "value"),
                expected,
                "{target}: {marker}"
            );
        }
        assert_no_whole_module_inference(&db, &db.take_salsa_events());
    }
}

#[test]
fn bare_assertions_apply_condition_evaluation_and_compose_sequentially() {
    use InferredTypeData::{Null, Number, String, Undefined};
    for (condition, expected) in [
        ("typeof value === 'string'", &[String][..]),
        ("value != null", &[String, Number]),
        ("value === null", &[Null]),
        ("!(typeof value === 'string')", &[Number, Null, Undefined]),
        ("value !== null && typeof value === 'string'", &[String]),
        (
            "typeof value === 'string' || typeof value === 'number'",
            &[String, Number],
        ),
        ("isString((value))", &[String]),
        ("!isString(value)", &[Number, Null, Undefined]),
    ] {
        let source = format!(
            r#"
                declare function assert(condition: unknown): asserts condition;
                declare function assertString(input: unknown): asserts input is string;
                declare function isString(input: unknown): input is string;
                function inspect(value: string | number | null | undefined) {{
                    assert(({condition}));
                    /*read*/value;
                }}
                function sequential(value: string | number | null | undefined) {{
                    assert(value != null);
                    /*first*/value;
                    assertString(/*secondArgument*/value);
                    /*second*/value;
                }}
            "#
        );
        let (db, module) = assertion_db(&source);
        for (marker, expected) in [
            ("read", expected),
            ("first", &[String, Number]),
            ("secondArgument", &[String, Number]),
            ("second", &[String]),
        ] {
            let ty = normalized_type_at(&db, module, &source, marker, "value");
            assert_variants(&db, ty, expected, &format!("{condition}: {marker}"));
        }
        assert_no_whole_module_inference(&db, &db.take_salsa_events());
    }
}

#[test]
fn assertion_parameter_mapping_reuses_this_indexing_and_escaped_names() {
    use InferredTypeData::{Number, String};
    for declaration in [
        "declare function check(ignored: unknown, value: unknown): asserts value is string;",
        "declare function check(this: void, ignored: unknown, value: unknown): asserts value is string;",
        r"declare function check(ignored: unknown, \u0076alue: unknown): asserts value is string;",
        r"declare function check(ignored: unknown, value: unknown): asserts \u0076alue is string;",
        r"declare function check(this: void, ignored: unknown, \u0076alue: unknown): asserts \u{76}alue is string;",
    ] {
        let source = format!(
            r#"
                {declaration}
                declare function assertCondition(this: void, ignored: unknown, condition: unknown): asserts condition;
                function inspect(value: string | number, other: string | number) {{
                    check(/*firstArgument*/value, (/*secondArgument*/other));
                    /*unselected*/value;
                    /*selected*/other;
                }}
                function escapedSubject(\u0076alue: string | number) {{
                    check(undefined, value);
                    /*escapedSubject*/\u0076alue;
                }}
                function condition(value: string | number, other: string | number) {{
                    assertCondition(value, typeof other === 'string');
                    /*unselectedCondition*/value;
                    /*selectedCondition*/other;
                }}
            "#
        );
        let (db, module) = assertion_db(&source);
        for (marker, expression, expected) in [
            ("firstArgument", "value", &[String, Number][..]),
            ("secondArgument", "other", &[String, Number]),
            ("unselected", "value", &[String, Number]),
            ("selected", "other", &[String]),
            ("escapedSubject", r"\u0076alue", &[String]),
            ("unselectedCondition", "value", &[String, Number]),
            ("selectedCondition", "other", &[String]),
        ] {
            let ty = normalized_type_at(&db, module, &source, marker, expression);
            assert_variants(&db, ty, expected, declaration);
        }
        assert_no_whole_module_inference(&db, &db.take_salsa_events());
    }
}

#[test]
fn assertion_effects_join_statement_paths_without_a_negative_complement() {
    use InferredTypeData::{Null, Number, String};
    const SOURCE: &str = r#"
        declare function assertString(input: unknown): asserts input is string;
        function oneBranch(value: string | number | null, flag: boolean) {
            if (flag) {
                assertString(value);
                /*then*/value;
            } else {
                /*else*/value;
            }
            /*join*/value;
        }
        function bothBranches(value: string | number | null, flag: boolean) {
            if (flag) assertString(value); else assertString(value);
            /*both*/value;
        }
        function returningBranch(value: string | number | null, flag: boolean) {
            if (flag) {
                assertString(value);
                return;
            }
            /*surviving*/value;
        }
        function survivingAssertion(value: string | number | null) {
            if (typeof value === 'number') return;
            assertString(value);
            /*string*/value;
        }
    "#;
    let (db, module) = assertion_db(SOURCE);
    for (marker, expected) in [
        ("then", &[String][..]),
        ("else", &[String, Number, Null]),
        ("join", &[String, Number, Null]),
        ("both", &[String]),
        ("surviving", &[String, Number, Null]),
        ("string", &[String]),
    ] {
        let ty = normalized_type_at(&db, module, SOURCE, marker, "value");
        assert_variants(&db, ty, expected, marker);
    }
    assert_no_whole_module_inference(&db, &db.take_salsa_events());
}

#[test]
fn assertion_effects_respect_loop_entries_backedges_breaks_and_continues() {
    use InferredTypeData::{Number, String};
    const SOURCE: &str = r#"
        declare function assertString(input: unknown): asserts input is string;
        function whileLoop(value: string | number, flag: boolean) {
            while (flag) {
                /*whileHead*/value;
                assertString(value);
                /*whileBody*/value;
            }
            /*whileExit*/value;
        }
        function forLoop(value: string | number, flag: boolean) {
            for (; flag; /*update*/value) {
                assertString(value);
                /*forBody*/value;
            }
            /*forExit*/value;
        }
        function doLoop(value: string | number, flag: boolean) {
            do {
                /*doHead*/value;
                assertString(value);
                /*doBody*/value;
            } while (flag);
            /*doExit*/value;
        }
        function breakLoop(value: string | number, flag: boolean, stop: boolean) {
            do {
                if (stop) break;
                assertString(value);
            } while (flag);
            /*breakExit*/value;
        }
        function continueLoop(value: string | number, flag: boolean, skip: boolean) {
            for (; flag; /*continueUpdate*/value) {
                if (skip) continue;
                assertString(value);
            }
        }
    "#;
    let (db, module) = assertion_db(SOURCE);
    for (marker, expected) in [
        ("whileHead", &[String, Number][..]),
        ("whileBody", &[String]),
        ("whileExit", &[String, Number]),
        ("update", &[String]),
        ("forBody", &[String]),
        ("forExit", &[String, Number]),
        ("doHead", &[String, Number]),
        ("doBody", &[String]),
        ("doExit", &[String]),
        ("breakExit", &[String, Number]),
        ("continueUpdate", &[String, Number]),
    ] {
        let ty = normalized_type_at(&db, module, SOURCE, marker, "value");
        assert_variants(&db, ty, expected, marker);
    }
    assert_no_whole_module_inference(&db, &db.take_salsa_events());
}

#[test]
fn assertions_respect_binding_identity_writes_and_existing_subject_restrictions() {
    use InferredTypeData::{Number, String};
    const SOURCE: &str = r#"
        declare function assertString(input: unknown, ignored?: unknown): asserts input is string;
        declare function assertNumber(input: unknown): asserts input is number;
        declare function read(): string | number;
        function shadowedSubject(value: string | number) {
            assertString(value);
            {
                const value = read();
                /*innerBefore*/value;
                assertNumber(value);
                /*innerAfter*/value;
            }
            /*outer*/value;
        }
        function shadowedDeclaration(value: string | number) {
            function assertString(input: unknown): asserts input is number {}
            assertString(value);
            /*localCallee*/value;
        }
        function shadowedParameter(value: string | number, assertString: (input: unknown) => asserts input is string) {
            assertString(value);
            /*parameterCallee*/value;
        }
        function locals() {
            let value: string | number = read();
            assertString(value);
            /*let*/value;
            var other: string | number = read();
            assertString(other);
            /*var*/other;
        }
        function written(value: string | number) {
            assertString(value);
            value = 0;
            /*written*/value;
        }
        function argumentWrite(value: string | number) {
            assertString(value, value = 0);
            /*argumentWrite*/value;
        }
        function captured(value: string | number) {
            assertString(value);
            return () => /*captured*/value;
        }
        function exceptional(value: string | number) {
            try { assertString(value); /*try*/value; } finally {}
        }
        function dynamic(value: string | number) {
            assertString(value);
            \u0065val('value = 0');
            /*eval*/value;
        }
        function writable(input: unknown): asserts input is string {}
        writable = (input): asserts input is string => {};
        function writtenCallee(value: string | number) {
            writable(value);
            /*writtenCallee*/value;
        }
    "#;
    let (db, module) = assertion_db(SOURCE);
    for (marker, expression, expected) in [
        ("innerBefore", "value", &[String, Number][..]),
        ("innerAfter", "value", &[Number]),
        ("outer", "value", &[String]),
        ("localCallee", "value", &[Number]),
        ("parameterCallee", "value", &[String, Number]),
        ("let", "value", &[String]),
        ("var", "other", &[String, Number]),
        ("written", "value", &[String, Number]),
        ("argumentWrite", "value", &[String, Number]),
        ("captured", "value", &[String, Number]),
        ("try", "value", &[String, Number]),
        ("eval", "value", &[String, Number]),
        ("writtenCallee", "value", &[String, Number]),
    ] {
        let ty = normalized_type_at(&db, module, SOURCE, marker, expression);
        assert_variants(&db, ty, expected, marker);
    }
    assert_no_whole_module_inference(&db, &db.take_salsa_events());
}

#[test]
fn optional_and_nested_assertion_calls_preserve_incoming_types() {
    use InferredTypeData::{Number, String};
    for statement in [
        "assertString?.(value); /*read*/value;",
        "consume(assertString(value), /*read*/value);",
        "flag && assertString(value); /*read*/value;",
        "assertString(value) && /*read*/value;",
        "flag || assertString(value); /*read*/value;",
        "assertString(value) ?? /*read*/value;",
        "flag ? assertString(value) : undefined; /*read*/value;",
        "(assertString(value), /*read*/value);",
        "!assertString(value); /*read*/value;",
        "void assertString(value); /*read*/value;",
        "result = assertString(value); /*read*/value;",
        "const assigned = assertString(value); /*read*/value;",
        "return (assertString(value), /*read*/value);",
        "throw (assertString(value), /*read*/value);",
        "assertString(...[value]); /*read*/value;",
    ] {
        let source = format!(
            r#"
                declare function assertString(input: unknown): asserts input is string;
                declare function consume(...values: unknown[]): void;
                function inspect(value: string | number, flag: boolean) {{
                    let result: unknown;
                    {statement}
                }}
                function incoming(value: string | number, flag: boolean) {{
                    let result: unknown;
                    if (typeof value === 'number') {{
                        {incoming}
                    }}
                }}
            "#,
            incoming = statement.replace("/*read*/", "/*incoming*/")
        );
        let (db, module) = assertion_db(&source);
        for (marker, expected) in [("read", &[String, Number][..]), ("incoming", &[Number])] {
            let ty = normalized_type_at(&db, module, &source, marker, "value");
            assert_variants(&db, ty, expected, statement);
        }
        assert_no_whole_module_inference(&db, &db.take_salsa_events());
    }
}

#[test]
fn unsupported_assertion_signatures_and_never_returns_do_not_add_facts() {
    use InferredTypeData::{Null, String};
    for declaration in [
        "declare function check(input: unknown): asserts input is string | number;",
        "type Alias = string; declare function check(input: unknown): asserts input is Alias;",
        "declare function check(input: unknown): asserts input is Promise<void>;",
        "function check(this: unknown, input: unknown): asserts this is string {}",
        "function check(this: unknown, input: unknown): asserts this {}",
        "declare function check<T>(input: unknown): asserts input;",
        "declare function check(input: string): asserts input; declare function check(input: unknown): asserts input is string;",
        "declare function check(input: unknown): never;",
    ] {
        let source = format!(
            r#"
                {declaration}
                function inspect(value: string | null) {{
                    check(value);
                    /*read*/value;
                }}
            "#
        );
        let (db, module) = assertion_db(&source);
        let ty = normalized_type_at(&db, module, &source, "read", "value");
        assert_variants(&db, ty, &[String, Null], declaration);
        assert_no_whole_module_inference(&db, &db.take_salsa_events());
    }
}

#[test]
fn unsupported_assertion_targets_do_not_expand_large_baselines() {
    let variants = (0..1100)
        .map(|index| format!("\"value{index}\""))
        .collect::<Vec<_>>()
        .join(" | ");
    for target in [
        "Alias",
        "string | number",
        "Promise<void>",
        "unknown",
        "any",
        "never",
        "void",
        "object",
        "() => void",
    ] {
        let source = format!(
            "type Alias = string; declare function check(input: unknown): asserts input is {target};\n\
             function f(value: {variants}) {{ /*before*/value; check(value); /*after*/value; }}"
        );
        let (db, module) = assertion_db(&source);
        db.clear_salsa_events();
        let read = |marker| {
            infer_expression_type(
                &db,
                ExpressionTypeInput::new(&db, module, marked_range(&source, marker, "value")),
            )
            .unwrap()
        };
        let ordinary = read("before");
        assert_ne!(ordinary, InferredTypeData::Unknown);
        assert_eq!(read("after"), ordinary, "{target}");
        let events = db.take_salsa_events();
        assert_eq!(
            function_query_will_execute_count_by_name(&db, "infer_flow_binding_type", &events),
            0,
            "{target}"
        );
    }
}

#[test]
fn assertion_consumers_and_complete_expression_tables_agree_with_targeted_reads() {
    use TypeInferenceClassification::{Match, NoMatch};
    const SOURCE: &str = r#"
        declare function assert(condition: unknown): asserts condition;
        declare function assertString(input: unknown): asserts input is string;
        declare function isString(input: unknown): input is string;
        declare function identity<T>(input: T): T;
        function promise(value: Promise<void> | null) {
            /*before*/value;
            assert(/*argument*/value);
            /*promise*/value;
            /*call*/identity(value);
        }
        function predicate(value: Promise<void> | string) {
            assert(!isString(value));
            /*predicate*/value;
        }
        function primitive(value: Promise<void> | string) {
            assertString(value);
            /*string*/value;
            /*stringCall*/identity(value);
            /*addition*/value + 1;
        }
        function array(value: Promise<void>[] | null) {
            assert(value != null);
            /*array*/value;
        }
        function callback(value: (() => Promise<void>) | null) {
            assert(value);
            /*callback*/value;
            /*callbackCall*/value();
        }
        function member(value: { length: number; method(): string } | null) {
            assert(value != null);
            /*member*/value?.length;
            /*method*/value?.method();
        }
    "#;
    let (db, module) = assertion_db(SOURCE);
    for (marker, expression, expected) in [
        ("promise", "value", Match),
        ("call", "identity(value)", Match),
        ("predicate", "value", Match),
        ("string", "value", NoMatch),
        ("stringCall", "identity(value)", NoMatch),
        ("callbackCall", "value()", Match),
    ] {
        assert_eq!(
            execute_type_inference_request(
                &db,
                TypeInferenceCaller::new("test", "assertedPromise"),
                PromiseClassificationRequest::new(module, marked_range(SOURCE, marker, expression)),
            ),
            expected,
            "{marker}"
        );
    }
    assert_eq!(
        execute_type_inference_request(
            &db,
            TypeInferenceCaller::new("test", "assertedPromiseArray"),
            ArrayOfPromisesClassificationRequest::new(
                module,
                marked_range(SOURCE, "array", "value")
            ),
        ),
        Match
    );
    assert_eq!(
        execute_type_inference_request(
            &db,
            TypeInferenceCaller::new("test", "assertedPromiseCallback"),
            PromiseReturningFunctionClassificationRequest::new(
                module,
                marked_range(SOURCE, "callback", "value"),
            ),
        ),
        Match
    );
    for (marker, expression, expected) in [
        ("addition", "value + 1", InferredTypeData::String),
        ("member", "value?.length", InferredTypeData::Number),
        ("method", "value?.method()", InferredTypeData::String),
    ] {
        assert_eq!(
            normalized_type_at(&db, module, SOURCE, marker, expression),
            expected,
            "{marker}"
        );
    }
    let targeted = [
        ("before", "value"),
        ("argument", "value"),
        ("promise", "value"),
        ("call", "identity(value)"),
        ("predicate", "value"),
        ("string", "value"),
        ("stringCall", "identity(value)"),
        ("addition", "value + 1"),
        ("array", "value"),
        ("callback", "value"),
        ("callbackCall", "value()"),
        ("member", "value?.length"),
        ("method", "value?.method()"),
    ]
    .map(|(marker, expression)| {
        let range = marked_range(SOURCE, marker, expression);
        let ty = infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range)).unwrap();
        (range, ty)
    });
    assert_no_whole_module_inference(&db, &db.take_salsa_events());
    let complete = infer_module_types(&db, module).unwrap();
    for (range, ty) in targeted {
        assert_eq!(complete.expressions[&range], ty, "{range:?}");
    }
}

#[test]
fn assertion_requests_reuse_unrelated_edits_and_recompute_signature_and_location_changes() {
    use InferredTypeData::{Number, String};
    const SOURCE: &str = r#"
        import type { Unrelated } from './unrelated.ts';
        declare function assertValue(ignored: Unrelated | undefined, value: unknown): asserts value is string;
        function inspect(value: string | number) {
            assertValue(undefined, value);
            /*read*/value;
        }
    "#;
    let changed_source = SOURCE.replace("value is string", "value is number");
    let expression = marked_range(SOURCE, "read", "value");
    assert_eq!(expression, marked_range(&changed_source, "read", "value"));
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);
    fs.insert(
        "/src/unrelated.ts".into(),
        "export interface Unrelated { member: string }",
    );
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
        assert_eq!(
            function_query_will_execute_count_by_name(&db, "infer_export_type", &events),
            0
        );
        assert_no_whole_module_inference(&db, &events);
    }

    fs.insert(
        "/src/unrelated.ts".into(),
        "export interface Unrelated { member: boolean }",
    );
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

    let shifted_source = format!(
        "\n\n{}",
        changed_source.replace(
            "assertValue(undefined, value);",
            "void assertValue(undefined, value);"
        )
    );
    assert_ne!(expression, marked_range(&shifted_source, "read", "value"));
    for (source, expected) in [
        (changed_source.as_str(), &[Number][..]),
        (shifted_source.as_str(), &[String, Number]),
    ] {
        fs.insert("/src/index.ts".into(), source);
        let kind = resolve_js_module_kind_for_test(&fs, "/src/index.ts", true);
        salsa::Setter::to(module.set_kind(&mut db), kind);
        db.clear_salsa_events();
        let ty = normalized_type_at(&db, module, source, "read", "value");
        assert_variants(&db, ty, expected, "edited assertion");
        let events = db.take_salsa_events();
        let input = ExpressionTypeInput::new(&db, module, marked_range(source, "read", "value"));
        assert_function_query_was_run(&db, infer_expression_type, input, &events);
        assert_no_whole_module_inference(&db, &events);
    }
}
