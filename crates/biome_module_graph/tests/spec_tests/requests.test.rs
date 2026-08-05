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
