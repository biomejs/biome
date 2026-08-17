use super::*;
use biome_module_graph::{
    ExpressionTypeInput, infer_expression_type,
    type_inference::{
        ArrayOfPromisesClassificationRequest, CallableMemberRequest,
        ExpectedCallArgumentTypeRequest, ExpectedConstructorArgumentTypeRequest,
        FunctionReturnTypeRequest, MemberReturnTypeRequest, NormalizedExpressionTypeRequest,
        PromiseClassificationRequest, PromiseReturningFunctionClassificationRequest,
        TypeInferenceArgument, TypeInferenceCaller, TypeInferenceClassification,
        TypeInferenceRequestMetadata, TypeInferenceSource, execute_type_inference_request,
    },
};

const CALL_ARGUMENT_QUERY: &str = "infer_call_argument_type";

#[test]
fn argument_requests_have_distinct_static_contracts() {
    assert_ne!(
        ExpectedCallArgumentTypeRequest::ID,
        ExpectedConstructorArgumentTypeRequest::ID
    );
    assert_ne!(
        ExpectedCallArgumentTypeRequest::LABEL,
        ExpectedConstructorArgumentTypeRequest::LABEL
    );
}

fn expression_range_by_source(
    db: &dyn ModuleDb,
    module: ModuleInfo,
    source: &str,
    expected: &str,
) -> TextRange {
    let ModuleInfoKind::Js(js_info) = module.kind(db) else {
        panic!("module must contain JavaScript information");
    };
    js_info
        .raw_expressions
        .keys()
        .find(|range| source_snippet(source, **range) == expected)
        .copied()
        .unwrap_or_else(|| panic!("{expected} expression must exist"))
}

#[test]
fn normalized_expression_request_keeps_lookup_query_boundary() {
    let source = "const value = 1; value;";
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), source);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let expression = expression_range_by_source(&db, module, source, "value");
    let input = ExpressionTypeInput::new(&db, module, expression);

    db.clear_salsa_events();
    let ty = execute_type_inference_request(
        &db,
        TypeInferenceCaller::new("test", "normalizedExpression"),
        NormalizedExpressionTypeRequest::new(module, expression),
    )
    .expect("expression type must be inferred");
    assert!(is_inferred_number(&db, ty));
    let events = db.take_salsa_events();

    assert_function_query_was_run(&db, infer_expression_type, input, &events);
    assert_function_query_was_not_run(&db, infer_module_types, module, &events);
}

#[test]
fn expected_argument_requests_compose_lookup_queries() {
    let source = "function takes(value: () => void) {} class Job { constructor(value: () => void) {} } takes(() => {}); new Job(() => { let done = true; });";
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), source);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let callee = expression_range_by_source(&db, module, source, "takes");
    let argument = expression_range_by_source(&db, module, source, "() => {}");

    db.clear_salsa_events();
    let ty = execute_type_inference_request(
        &db,
        TypeInferenceCaller::new("test", "expectedArgument"),
        ExpectedCallArgumentTypeRequest::new(
            module,
            argument,
            callee,
            vec![TypeInferenceArgument::new(argument, false)].into_boxed_slice(),
            0,
        ),
    )
    .expect("argument type must be inferred");
    assert!(InferredType::new(&db, ty).function_returns_void());

    let constructor = expression_range_by_source(&db, module, source, "Job");
    let constructor_argument =
        expression_range_by_source(&db, module, source, "() => { let done = true; }");
    let ty = execute_type_inference_request(
        &db,
        TypeInferenceCaller::new("test", "expectedConstructorArgument"),
        ExpectedConstructorArgumentTypeRequest::new(
            module,
            constructor_argument,
            constructor,
            vec![TypeInferenceArgument::new(constructor_argument, false)].into_boxed_slice(),
            0,
        ),
    )
    .expect("constructor argument type must be inferred");
    assert!(InferredType::new(&db, ty).function_returns_void());

    let events = db.take_salsa_events();
    assert_function_query_was_not_run(&db, infer_module_types, module, &events);
}

#[test]
fn expected_call_argument_request_skips_requested_non_spread_argument() {
    const SOURCE: &str = r#"
        import { load } from "./source.ts";
        declare function schedule(kind: "sync", task: (value: number) => void): void;
        declare function schedule(kind: "async", task: (value: number) => Promise<void>): void;
        schedule("sync", (value) => load(value));
    "#;
    const CHANGED_CALLEE_SOURCE: &str = r#"
        import { load } from "./source.ts";
        declare function schedule(kind: "noop", task: (value: number) => void): void;
        declare function schedule(kind: "async", task: (value: number) => Promise<void>): void;
        schedule("sync", (value) => load(value));
    "#;
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/source.ts".into(),
        "export async function load(value: number) { return value; }",
    );
    fs.insert("/src/index.ts".into(), SOURCE);

    let mut db = build_js_test_module_db(&fs, &["/src/source.ts", "/src/index.ts"], true);
    let source_module = db
        .module_for_path(Utf8Path::new("/src/source.ts"))
        .expect("source module must exist");
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let callee = expression_range_by_source(&db, module, SOURCE, "schedule");
    let kind = expression_range_by_source(&db, module, SOURCE, r#""sync""#);
    let callback = expression_range_by_source(&db, module, SOURCE, "(value) => load(value)");
    let request = || {
        ExpectedCallArgumentTypeRequest::new(
            module,
            callback,
            callee,
            vec![
                TypeInferenceArgument::new(kind, false),
                TypeInferenceArgument::new(callback, false),
            ]
            .into_boxed_slice(),
            1,
        )
    };

    {
        let callee_input = ExpressionTypeInput::new(&db, module, callee);
        let kind_input = ExpressionTypeInput::new(&db, module, kind);
        let callback_input = ExpressionTypeInput::new(&db, module, callback);

        db.clear_salsa_events();
        let ty = execute_type_inference_request(
            &db,
            TypeInferenceCaller::new("test", "skipRequestedArgument"),
            request(),
        )
        .expect("argument type must be inferred");
        assert!(InferredType::new(&db, ty).function_returns_void());
        let events = db.take_salsa_events();

        assert_function_query_was_run(&db, infer_expression_type, callee_input, &events);
        assert_function_query_was_run(&db, infer_expression_type, kind_input, &events);
        assert_function_query_was_not_run(&db, infer_expression_type, callback_input, &events);
        assert_eq!(
            function_query_will_execute_count_by_name(&db, CALL_ARGUMENT_QUERY, &events),
            1
        );

        db.clear_salsa_events();
        let ty = execute_type_inference_request(
            &db,
            TypeInferenceCaller::new("test", "skipRequestedArgument"),
            request(),
        )
        .expect("cached argument type must be inferred");
        assert!(InferredType::new(&db, ty).function_returns_void());
        let events = db.take_salsa_events();
        assert_function_query_was_not_run(&db, infer_expression_type, callee_input, &events);
        assert_function_query_was_not_run(&db, infer_expression_type, kind_input, &events);
        assert_function_query_was_not_run(&db, infer_expression_type, callback_input, &events);
        assert_eq!(
            function_query_will_execute_count_by_name(&db, CALL_ARGUMENT_QUERY, &events),
            0
        );
    }

    fs.insert(
        "/src/source.ts".into(),
        "export async function load(value: string) { return value; }",
    );
    let source_kind = resolve_js_module_kind_for_test(&fs, "/src/source.ts", true);
    salsa::Setter::to(source_module.set_kind(&mut db), source_kind);
    db.clear_salsa_events();
    let ty = execute_type_inference_request(
        &db,
        TypeInferenceCaller::new("test", "skipRequestedArgument"),
        request(),
    )
    .expect("argument type must remain inferred");
    assert!(InferredType::new(&db, ty).function_returns_void());
    let events = db.take_salsa_events();
    let callee_input = ExpressionTypeInput::new(&db, module, callee);
    let kind_input = ExpressionTypeInput::new(&db, module, kind);
    let callback_input = ExpressionTypeInput::new(&db, module, callback);
    assert_function_query_was_not_run(&db, infer_expression_type, callee_input, &events);
    assert_function_query_was_not_run(&db, infer_expression_type, kind_input, &events);
    assert_function_query_was_not_run(&db, infer_expression_type, callback_input, &events);
    assert_eq!(
        function_query_will_execute_count_by_name(&db, CALL_ARGUMENT_QUERY, &events),
        0
    );

    fs.insert("/src/index.ts".into(), CHANGED_CALLEE_SOURCE);
    let module_kind = resolve_js_module_kind_for_test(&fs, "/src/index.ts", true);
    salsa::Setter::to(module.set_kind(&mut db), module_kind);
    db.clear_salsa_events();
    assert!(
        execute_type_inference_request(
            &db,
            TypeInferenceCaller::new("test", "skipRequestedArgument"),
            request(),
        )
        .is_none()
    );
    let events = db.take_salsa_events();
    let callee_input = ExpressionTypeInput::new(&db, module, callee);
    let kind_input = ExpressionTypeInput::new(&db, module, kind);
    let callback_input = ExpressionTypeInput::new(&db, module, callback);
    assert_function_query_was_run(&db, infer_expression_type, callee_input, &events);
    assert_function_query_was_run(&db, infer_expression_type, kind_input, &events);
    assert_function_query_was_not_run(&db, infer_expression_type, callback_input, &events);
    assert_eq!(
        function_query_will_execute_count_by_name(&db, CALL_ARGUMENT_QUERY, &events),
        1
    );
}

#[test]
fn expected_call_argument_request_resolves_requested_spread_argument() {
    const SOURCE: &str = r#"
        declare function consume(callback: () => void): void;
        const callbacks: [() => Promise<void>] = [async () => {}];
        consume(...callbacks);
    "#;
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let callee = expression_range_by_source(&db, module, SOURCE, "consume");
    let callbacks = expression_range_by_source(&db, module, SOURCE, "callbacks");
    let callbacks_input = ExpressionTypeInput::new(&db, module, callbacks);

    db.clear_salsa_events();
    let ty = execute_type_inference_request(
        &db,
        TypeInferenceCaller::new("test", "resolveRequestedSpread"),
        ExpectedCallArgumentTypeRequest::new(
            module,
            callbacks,
            callee,
            vec![TypeInferenceArgument::new(callbacks, true)].into_boxed_slice(),
            0,
        ),
    )
    .expect("argument type must be inferred");
    assert!(InferredType::new(&db, ty).function_returns_void());
    let events = db.take_salsa_events();

    assert_function_query_was_run(&db, infer_expression_type, callbacks_input, &events);
}

#[test]
fn classification_requests_preserve_conclusive_and_indeterminate_results() {
    let source = r#"
        declare function consume(value: unknown): void;
        const value = 1;
        declare const uncertain: unknown;
        const object = {};
        consume(value);
        consume(uncertain);
        consume(object);
    "#;
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), source);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let caller = TypeInferenceCaller::new("test", "classificationResults");
    let value = expression_range_by_source(&db, module, source, "value");
    let uncertain = expression_range_by_source(&db, module, source, "uncertain");
    let object = expression_range_by_source(&db, module, source, "object");

    assert_eq!(
        execute_type_inference_request(
            &db,
            caller,
            PromiseClassificationRequest::new(module, value),
        ),
        TypeInferenceClassification::NoMatch
    );
    assert_eq!(
        execute_type_inference_request(
            &db,
            caller,
            PromiseClassificationRequest::new(module, uncertain),
        ),
        TypeInferenceClassification::Indeterminate
    );
    assert_eq!(
        execute_type_inference_request(
            &db,
            caller,
            CallableMemberRequest::new(module, object, "missing"),
        ),
        TypeInferenceClassification::NoMatch
    );
    assert_eq!(
        execute_type_inference_request(
            &db,
            caller,
            CallableMemberRequest::new(module, uncertain, "then"),
        ),
        TypeInferenceClassification::Indeterminate
    );
}

#[test]
fn classification_and_return_type_requests_stay_selective() {
    let source = r#"
        const promise = Promise.resolve(1);
        const callbacks = { promises: (): Array<Promise<void>> => [] };
        async function returnsPromise() { return 1; }
        const object = { method() { return 1; } };
        promise;
        callbacks.promises();
        returnsPromise;
        object;
    "#;
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), source);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let caller = TypeInferenceCaller::new("test", "selectiveRequests");
    let promise = expression_range_by_source(&db, module, source, "promise");
    let promises = expression_range_by_source(&db, module, source, "callbacks.promises()");
    let function = expression_range_by_source(&db, module, source, "returnsPromise");
    let object = expression_range_by_source(&db, module, source, "object");

    db.clear_salsa_events();
    assert_eq!(
        execute_type_inference_request(
            &db,
            caller,
            PromiseClassificationRequest::new(module, promise),
        ),
        TypeInferenceClassification::Match
    );
    assert_eq!(
        execute_type_inference_request(
            &db,
            caller,
            ArrayOfPromisesClassificationRequest::new(module, promises),
        ),
        TypeInferenceClassification::Match
    );
    assert_eq!(
        execute_type_inference_request(
            &db,
            caller,
            PromiseReturningFunctionClassificationRequest::new(module, function),
        ),
        TypeInferenceClassification::Match
    );
    let return_ty = execute_type_inference_request(
        &db,
        caller,
        FunctionReturnTypeRequest::new(module, function, TypeInferenceSource::Expression(function)),
    )
    .expect("function return type must be inferred");
    assert!(InferredType::new(&db, return_ty).is_inferred());
    assert_eq!(
        execute_type_inference_request(
            &db,
            caller,
            CallableMemberRequest::new(module, object, "method"),
        ),
        TypeInferenceClassification::Match
    );
    let member_return_ty = execute_type_inference_request(
        &db,
        caller,
        MemberReturnTypeRequest::new(
            module,
            object,
            TypeInferenceSource::Expression(object),
            "method",
        ),
    )
    .expect("member return type must be inferred");
    assert!(is_inferred_number(&db, member_return_ty));

    let events = db.take_salsa_events();
    assert_function_query_was_not_run(&db, infer_module_types, module, &events);
}
