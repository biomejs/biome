use super::*;
use biome_module_graph::{
    BindingTypeInput, ExpressionTypeInput, infer_binding_type, infer_expression_is_promise,
    infer_expression_type,
    type_inference::{
        NormalizedExpressionTypeRequest, PromiseClassificationRequest,
        PromiseReturningFunctionClassificationRequest, TypeInferenceCaller,
        TypeInferenceClassification, execute_type_inference_request,
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
        TypeInferenceCaller::new("test", "narrowing"),
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

fn assert_no_flow_queries(db: &TestModuleDb, events: &[salsa::Event]) {
    for query in [
        "infer_flow_expression_type",
        "infer_flow_expression_type_impl",
        "infer_flow_binding_type",
        "narrowing_flow_for_root",
        "module_control_flow",
    ] {
        assert_eq!(
            function_query_will_execute_count_by_name(db, query, events),
            0,
            "{query} must not execute without flow candidates"
        );
    }
}

#[test]
fn declaration_only_modules_do_not_build_flow_candidates() {
    let (db, module) = narrowing_db("type Alias = string; interface Shape { value: Alias; }");
    let ModuleInfoKind::Js(info) = module.kind(&db) else {
        panic!("module must contain JavaScript information");
    };
    assert!(info.raw_expressions.is_empty());
    db.clear_salsa_events();
    assert!(infer_module_types(&db, module).is_some());
    let events = db.take_salsa_events();
    assert_no_flow_queries(&db, &events);
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "flow_candidates_for_module", &events),
        0
    );
}

#[test]
fn cyclic_bindings_keep_unknown_overrides_for_impossible_branches() {
    const SOURCE: &str = r#"
        import { b as importedB } from "./index.ts";
        export const a = { b: importedB, stable: 1 };
        export const b = a;
        if (a == null) {
            /*first*/a;
            /*second*/a;
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    for marker in ["first", "second"] {
        assert_eq!(
            normalized_type_at(&db, module, SOURCE, marker, "a"),
            InferredTypeData::Unknown
        );
    }
}

#[test]
fn branchless_promise_batches_skip_flow_queries() {
    let mut source = String::from("function batch(value: number) {\n");
    for index in 0..16 {
        source.push_str(&format!("/*call{index}*/Promise.resolve(value);\n"));
    }
    source.push_str("}\n");
    let (db, module) = narrowing_db(&source);
    for cold in [true, false] {
        db.clear_salsa_events();
        for index in 0..16 {
            let marker = format!("call{index}");
            let range = marked_range(&source, &marker, "Promise.resolve(value)");
            assert_eq!(
                execute_type_inference_request(
                    &db,
                    TypeInferenceCaller::new("test", "branchlessPromise"),
                    PromiseClassificationRequest::new(module, range),
                ),
                TypeInferenceClassification::Match
            );
            let ty = normalized_type_at(&db, module, &source, &marker, "Promise.resolve(value)");
            assert!(ty.is_promise_instance(&db));
        }
        let events = db.take_salsa_events();
        assert_no_flow_queries(&db, &events);
        assert_eq!(
            function_query_will_execute_count_by_name(&db, "flow_candidates_for_module", &events),
            0,
            "collection already rules out flow in this module"
        );
        assert_eq!(
            function_query_will_execute_count_by_name(&db, "infer_expression_is_promise", &events),
            if cold { 16 } else { 0 }
        );
        assert_function_query_was_not_run(&db, infer_module_types, module, &events);
    }
}

#[test]
fn reads_before_the_first_condition_do_not_build_flow_graphs() {
    const SOURCE: &str = r#"
        function f(value: string | null) {
            /*before*/value;
            if (/*test*/value !== null) { /*body*/value; }
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    db.clear_salsa_events();
    for marker in ["before", "test"] {
        let ty = normalized_type_at(&db, module, SOURCE, marker, "value");
        assert_variants(
            &db,
            ty,
            &[InferredTypeData::String, InferredTypeData::Null],
            marker,
        );
    }
    assert_no_flow_queries(&db, &db.take_salsa_events());
    assert_eq!(
        normalized_type_at(&db, module, SOURCE, "body", "value"),
        InferredTypeData::String
    );
}

#[test]
fn loop_updates_keep_conditions_that_occur_later_in_the_source() {
    const SOURCE: &str = r#"
        function f(value: string | null) {
            for (;; /*update*/value) {
                if (value === null) break;
            }
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    assert_eq!(
        normalized_type_at(&db, module, SOURCE, "update", "value"),
        InferredTypeData::String
    );
}

#[test]
fn unsupported_condition_subjects_skip_flow_queries() {
    for condition in [
        "value.length",
        "check(value)",
        "value.method()",
        "value === other",
        "typeof value === other",
        "value == 1",
    ] {
        let source = format!(
            "function f(value: string | number, other: unknown) {{ if ({condition}) {{ /*read*/value; }} }}"
        );
        let (db, module) = narrowing_db(&source);
        db.clear_salsa_events();
        let ty = normalized_type_at(&db, module, &source, "read", "value");
        assert_variants(
            &db,
            ty,
            &[InferredTypeData::String, InferredTypeData::Number],
            condition,
        );
        assert_no_flow_queries(&db, &db.take_salsa_events());
    }
}

#[test]
fn nested_supported_tests_still_narrow_inside_an_unsupported_condition() {
    const SOURCE: &str = r#"
        function f(value: string | null) {
            if (check(value !== null && /*argument*/value)) {
                /*body*/value;
            }
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    assert_eq!(
        normalized_type_at(&db, module, SOURCE, "argument", "value"),
        InferredTypeData::String
    );
    let ty = normalized_type_at(&db, module, SOURCE, "body", "value");
    assert_variants(
        &db,
        ty,
        &[InferredTypeData::String, InferredTypeData::Null],
        "body",
    );
}

#[test]
fn unsupported_conditions_do_not_spend_the_baseline_expansion_budget() {
    let variants = (0..1100)
        .map(|index| format!("\"value{index}\""))
        .collect::<Vec<_>>()
        .join(" | ");
    let source = format!(
        "function f(/*binding*/value: {variants}) {{ if (value.length) {{ /*read*/value; }} }}"
    );
    let (db, module) = narrowing_db(&source);
    let binding = BindingTypeInput::new(&db, module, marked_range(&source, "binding", "value"));
    let baseline = infer_binding_type(&db, binding).unwrap();
    let baseline = normalize_type(&db, module, baseline);
    db.clear_salsa_events();
    assert_eq!(
        normalized_type_at(&db, module, &source, "read", "value"),
        baseline
    );
    assert_no_flow_queries(&db, &db.take_salsa_events());
}

#[test]
fn incomplete_conditions_do_not_expand_unrelated_binding_types() {
    let variants = (0..1100)
        .map(|index| format!("\"value{index}\""))
        .collect::<Vec<_>>()
        .join(" | ");
    for subject in ["flag", "value"] {
        let source = format!(
            "function f(value: {variants}, flag: boolean) {{ /*before*/value; if ({}{subject}) {{ /*after*/value; }} if (typeof value === 'string') {{}} }}",
            "!".repeat(32)
        );
        let (db, module) = narrowing_db(&source);
        let read = |marker| {
            infer_expression_type(
                &db,
                ExpressionTypeInput::new(&db, module, marked_range(&source, marker, "value")),
            )
            .unwrap()
        };
        let ordinary = read("before");
        assert_ne!(ordinary, InferredTypeData::Unknown);
        let expected = if subject == "flag" {
            ordinary
        } else {
            InferredTypeData::Unknown
        };
        assert_eq!(read("after"), expected, "{subject}");
    }
}

#[test]
fn shadowed_undefined_does_not_expand_the_compared_binding() {
    let variants = (0..1100)
        .map(|index| format!("\"value{index}\""))
        .collect::<Vec<_>>()
        .join(" | ");
    let source = format!(
        "function f(value: {variants}, undefined: number) {{ /*before*/value; if (value === undefined) {{ /*after*/value; }} }}"
    );
    let (db, module) = narrowing_db(&source);
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
    assert_eq!(read("after"), ordinary);
    assert_no_flow_queries(&db, &db.take_salsa_events());
}

#[test]
fn condition_discovery_preserves_shadowing_and_depth_limits() {
    const SOURCE: &str = r#"
        function f(value: string | number | undefined, undefined: number) {
            if (value === undefined) { /*read*/value; }
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    let ty = normalized_type_at(&db, module, SOURCE, "read", "value");
    assert_variants(
        &db,
        ty,
        &[
            InferredTypeData::String,
            InferredTypeData::Number,
            InferredTypeData::Undefined,
        ],
        "shadowed undefined",
    );
    for depth in [31, 32] {
        let source = format!(
            "function f(value: 'ready' | null) {{ if ({}value) {{ /*read*/value; }} }}",
            "!".repeat(depth)
        );
        let (db, module) = narrowing_db(&source);
        let ty = normalized_type_at(&db, module, &source, "read", "value");
        if depth == 31 {
            assert_eq!(ty, InferredTypeData::Null);
        } else {
            assert!(contains_inferred_null(&db, ty));
            assert_ne!(ty, InferredTypeData::Null);
            assert_ne!(ty, InferredTypeData::Unknown);
        }
    }
}

#[test]
fn unrelated_conditions_writes_and_captures_skip_flow_queries() {
    for source in [
        "function f(value: string | number, other: boolean) { if (other) return; /*read*/value; }",
        "function f(value: string | number) { if (typeof value === 'string') { value = 1; /*read*/value; } }",
        "function f(value: string | number) { if (typeof value === 'string') return () => /*read*/value; }",
    ] {
        let (db, module) = narrowing_db(source);
        db.clear_salsa_events();
        let ty = normalized_type_at(&db, module, source, "read", "value");
        assert_variants(
            &db,
            ty,
            &[InferredTypeData::String, InferredTypeData::Number],
            source,
        );
        let events = db.take_salsa_events();
        assert_no_flow_queries(&db, &events);
        assert_function_query_was_not_run(&db, infer_module_types, module, &events);
    }
}

#[test]
fn typeof_narrows_occurrences_without_changing_the_binding_or_join() {
    use InferredTypeData::{Number, String};
    const SOURCE: &str = r#"
        function inspect(/*binding*/value: string | number) {
            /*before*/value;
            if (typeof /*guard*/value === "string") {
                /*then*/value;
            } else {
                /*else*/value;
            }
            /*join*/value;
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    db.clear_salsa_events();
    for (marker, expected) in [
        ("then", &[String][..]),
        ("else", &[Number]),
        ("before", &[String, Number]),
        ("guard", &[String, Number]),
        ("join", &[String, Number]),
    ] {
        let ty = normalized_type_at(&db, module, SOURCE, marker, "value");
        assert_variants(&db, ty, expected, marker);
    }

    let binding = marked_range(SOURCE, "binding", "value");
    let ty = infer_binding_type(&db, BindingTypeInput::new(&db, module, binding)).unwrap();
    assert_variants(
        &db,
        normalize_type(&db, module, ty),
        &[String, Number],
        "binding",
    );
    let events = db.take_salsa_events();
    assert_function_query_was_not_run(&db, infer_module_types, module, &events);
}

#[test]
fn exhausted_type_filtering_does_not_publish_a_partial_refinement() {
    let variants = (0..1100)
        .map(|index| format!("\"value{index}\""))
        .collect::<Vec<_>>()
        .join(" | ");
    let source = format!(
        "function f(value: {variants}) {{ if (typeof value === \"string\") {{ /*read*/value; }} }}"
    );
    let (db, module) = narrowing_db(&source);
    assert_eq!(
        normalized_type_at(&db, module, &source, "read", "value"),
        InferredTypeData::Unknown
    );
}

#[test]
fn narrowed_addition_keeps_object_coercion_unknown() {
    const SOURCE: &str = r#"
        function calculate(flag: boolean) {
            const value = flag ? { valueOf: () => 1 } : null;
            if (value === null) return;
            /*object*/value + 1;
        }
        function primitive(value: string | number) {
            if (typeof value === "string") {
                /*string*/value + 1;
            } else {
                /*number*/value + 1;
            }
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    for (marker, expected) in [
        ("object", InferredTypeData::Unknown),
        ("string", InferredTypeData::String),
        ("number", InferredTypeData::Number),
    ] {
        assert_eq!(
            normalized_type_at(&db, module, SOURCE, marker, "value + 1"),
            expected,
            "{marker}"
        );
    }
}

#[test]
fn narrowed_callable_shapes_preserve_classification_uncertainty() {
    use TypeInferenceClassification::{Indeterminate, Match, NoMatch};
    const SOURCE: &str = r#"
        type Overloaded = { (): Promise<void>; (value: string): void };
        type Single = { (): Promise<void> };
        function overloaded(cb: Overloaded | null) {
            if (cb === null) return;
            /*overloaded*/cb;
        }
        function single(cb: Single | null) {
            if (cb === null) return;
            /*single*/cb;
        }
        function direct(cb: (() => number) | null) {
            if (cb === null) return;
            /*direct*/cb;
        }
        function promise(cb: (() => Promise<void>) | null) {
            if (cb === null) return;
            /*promise*/cb;
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    for (marker, expected) in [
        ("overloaded", Indeterminate),
        ("single", Indeterminate),
        ("direct", NoMatch),
        ("promise", Match),
    ] {
        assert_eq!(
            execute_type_inference_request(
                &db,
                TypeInferenceCaller::new("test", "narrowedCallable"),
                PromiseReturningFunctionClassificationRequest::new(
                    module,
                    marked_range(SOURCE, marker, "cb")
                ),
            ),
            expected,
            "{marker}"
        );
    }
}

#[test]
fn repeated_narrowed_reads_share_flow_states_and_agree_with_complete_tables() {
    use InferredTypeData::{Number, String};
    const SOURCE: &str = r#"
        function inspect(/*binding*/value: string | number | null) {
            if (value === null) return;
            if (typeof value === "string") {
                /*string0*/value;
                /*string1*/value;
            } else {
                /*number0*/value;
                /*number1*/value;
            }
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    db.clear_salsa_events();
    let targeted = [
        ("string0", String),
        ("string1", String),
        ("number0", Number),
        ("number1", Number),
    ]
    .map(|(marker, expected)| {
        let range = marked_range(SOURCE, marker, "value");
        let ty = infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range)).unwrap();
        assert_eq!(ty, expected, "{marker}");
        (range, ty)
    });
    let events = db.take_salsa_events();
    let binding = BindingTypeInput::new(&db, module, marked_range(SOURCE, "binding", "value"));
    assert_function_query_was_not_run(&db, infer_binding_type, binding, &events);
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "infer_flow_binding_type", &events),
        2,
        "reads in each branch must share its incoming flow state"
    );
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "narrowing_flow_for_root", &events),
        1
    );
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "module_control_flow", &events),
        0,
        "narrowing must construct only the selected root's CFG"
    );
    assert_function_query_was_not_run(&db, infer_module_types, module, &events);

    db.clear_salsa_events();
    for (range, ty) in targeted {
        assert_eq!(
            infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range)),
            Some(ty)
        );
    }
    let events = db.take_salsa_events();
    assert_no_flow_queries(&db, &events);
    assert_function_query_was_not_run(&db, infer_binding_type, binding, &events);

    let complete = infer_module_types(&db, module).unwrap();
    for (range, ty) in targeted {
        assert_eq!(complete.expressions[&range], ty);
    }
}

#[test]
fn shared_flow_points_keep_different_bindings_separate() {
    const SOURCE: &str = r#"
        function inspect(left: string | number, right: string | number) {
            if (typeof left === "string" && typeof right === "number") {
                /*left*/left;
                /*right*/right;
            }
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    db.clear_salsa_events();
    assert_eq!(
        normalized_type_at(&db, module, SOURCE, "left", "left"),
        InferredTypeData::String
    );
    assert_eq!(
        normalized_type_at(&db, module, SOURCE, "right", "right"),
        InferredTypeData::Number
    );
    let events = db.take_salsa_events();
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "infer_flow_binding_type", &events),
        2
    );
}

#[test]
fn cached_unreachable_reads_keep_unknown_overrides() {
    const SOURCE: &str = r#"
        function inspect(value: string | number) {
            if (typeof value === "boolean") {
                /*first*/value;
                /*second*/value;
            }
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    db.clear_salsa_events();
    for marker in ["first", "second"] {
        assert_eq!(
            normalized_type_at(&db, module, SOURCE, marker, "value"),
            InferredTypeData::Unknown
        );
        let input = ExpressionTypeInput::new(&db, module, marked_range(SOURCE, marker, "value"));
        assert_eq!(
            infer_expression_is_promise(&db, input),
            TypeInferenceClassification::Indeterminate
        );
    }
    let events = db.take_salsa_events();
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "infer_flow_binding_type", &events),
        1
    );
}

#[test]
fn loops_retain_only_facts_shared_by_reachable_predecessors() {
    use InferredTypeData::{Null, String};
    const SOURCE: &str = r#"
        function inspect(value: string | null) {
            while (value !== null) {
                /*body*/value;
            }
            /*exit*/value;
        }
        function withBreak(value: string | null, stop: boolean) {
            while (value !== null) {
                if (stop) break;
            }
            /*breakExit*/value;
        }
        function unknownValue(value: unknown) {
            if (typeof value === "string") {
                /*unknown*/value;
            }
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    for (marker, expected) in [
        ("body", &[String][..]),
        ("exit", &[Null]),
        ("breakExit", &[String, Null]),
        ("unknown", &[String]),
    ] {
        let ty = normalized_type_at(&db, module, SOURCE, marker, "value");
        assert_variants(&db, ty, expected, marker);
    }
}

#[test]
fn shifted_source_locations_do_not_reuse_stale_flow() {
    const SOURCE: &str =
        "function f(value: string | null) { if (value == null) return; /*read*/value; }";
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);
    let mut db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    assert_eq!(
        normalized_type_at(&db, module, SOURCE, "read", "value"),
        InferredTypeData::String
    );
    let changed = format!("\n\n{}", SOURCE.replace("==", "!="));
    fs.insert("/src/index.ts".into(), changed.as_str());
    let kind = resolve_js_module_kind_for_test(&fs, "/src/index.ts", true);
    salsa::Setter::to(module.set_kind(&mut db), kind);
    assert_eq!(
        normalized_type_at(&db, module, &changed, "read", "value"),
        InferredTypeData::Null
    );
}

#[test]
fn unwritten_const_and_let_bindings_narrow_at_their_reads() {
    use InferredTypeData::{Number, String};
    const SOURCE: &str = r#"
        declare function read(): string | number;
        function inspect() {
            const value = read();
            if (typeof value === "string") {
                /*const*/value;
            } else {
                /*constElse*/value;
            }
            let other: string | number = read();
            if (typeof other !== "number") {
                /*let*/other;
            }
            /*join*/other;
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    for (marker, expression, expected) in [
        ("const", "value", &[String][..]),
        ("constElse", "value", &[Number]),
        ("let", "other", &[String]),
        ("join", "other", &[String, Number]),
    ] {
        let ty = normalized_type_at(&db, module, SOURCE, marker, expression);
        assert_variants(&db, ty, expected, marker);
    }
}

#[test]
fn nullish_guards_distinguish_loose_and_strict_equality() {
    use InferredTypeData::{Null, String, Undefined};
    const SOURCE: &str = r#"
        function loose(value: string | null | undefined) {
            if (value == null) {
                /*looseThen*/value;
            } else {
                /*looseElse*/value;
            }
        }
        function strict(value: string | null | undefined) {
            if (value === null) {
                /*strictThen*/value;
            } else {
                /*strictElse*/value;
            }
            if (value !== undefined) {
                /*defined*/value;
            } else {
                /*undefined*/value;
            }
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    for (marker, expected) in [
        ("looseThen", &[Null, Undefined][..]),
        ("looseElse", &[String]),
        ("strictThen", &[Null]),
        ("strictElse", &[String, Undefined]),
        ("defined", &[String, Null]),
        ("undefined", &[Undefined]),
    ] {
        let ty = normalized_type_at(&db, module, SOURCE, marker, "value");
        assert_variants(&db, ty, expected, marker);
    }
}

#[test]
fn early_returns_preserve_facts_on_the_surviving_path() {
    use InferredTypeData::{Number, String};
    const SOURCE: &str = r#"
        function inspect(value: string | number | null | undefined) {
            if (value == null) return;
            /*nonNullish*/value;
            if (typeof value !== "string") return;
            /*string*/value;
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    for (marker, expected) in [("nonNullish", &[String, Number][..]), ("string", &[String])] {
        let ty = normalized_type_at(&db, module, SOURCE, marker, "value");
        assert_variants(&db, ty, expected, marker);
    }
}

#[test]
fn logical_and_conditional_expressions_narrow_only_the_selected_operand() {
    use InferredTypeData::{Null, String, Undefined};
    const SOURCE: &str = r#"
        function inspect(value: string | null | undefined) {
            /*andLeft*/value != null && /*andRight*/value;
            value == null || /*orRight*/value;
            /*coalesceLeft*/value ?? /*coalesceRight*/value;
            value != null ? /*consequent*/value : /*alternate*/value;
            /*join*/value;
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    for (marker, expected) in [
        ("andRight", &[String][..]),
        ("orRight", &[String]),
        ("coalesceRight", &[Null, Undefined]),
        ("consequent", &[String]),
        ("alternate", &[Null, Undefined]),
        ("andLeft", &[String, Null, Undefined]),
        ("coalesceLeft", &[String, Null, Undefined]),
        ("join", &[String, Null, Undefined]),
    ] {
        let ty = normalized_type_at(&db, module, SOURCE, marker, "value");
        assert_variants(&db, ty, expected, marker);
    }
}

#[test]
fn strict_literal_and_truthiness_guards_filter_literal_unions() {
    const SOURCE: &str = r#"
        function strings(value: "ready" | "") {
            if (value === "ready") {
                /*equal*/value;
            } else {
                /*notEqual*/value;
            }
            if (!value) {
                /*falsy*/value;
            } else {
                /*truthy*/value;
            }
            value && /*and*/value;
            value || /*or*/value;
        }
        function numbers(value: 0 | 1) {
            if (value !== 0) {
                /*one*/value;
            } else {
                /*zero*/value;
            }
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    for (marker, expected) in [
        ("equal", "ready"),
        ("notEqual", ""),
        ("falsy", ""),
        ("truthy", "ready"),
        ("and", "ready"),
        ("or", ""),
    ] {
        let ty = normalized_type_at(&db, module, SOURCE, marker, "value");
        assert!(
            is_inferred_string_literal(&db, ty, expected),
            "{marker}: got {}",
            format_inferred_type(&db, ty)
        );
    }
    for (marker, expected) in [("one", "1"), ("zero", "0")] {
        let ty = normalized_type_at(&db, module, SOURCE, marker, "value");
        assert!(
            is_inferred_number_literal(&db, ty, expected),
            "{marker}: got {}",
            format_inferred_type(&db, ty)
        );
    }
}

#[test]
fn enclosing_member_and_call_queries_use_the_narrowed_read() {
    use InferredTypeData::{Number, String};
    const SOURCE: &str = r#"
        declare function identity<T>(value: T): T;
        function inspect(value: string | number) {
            if (typeof value === "string") {
                /*argument*/identity(value);
            } else {
                /*otherArgument*/identity(value);
            }
        }
        function members(value: { length: number; method(): string } | null) {
            if (value !== null) {
                /*member*/value?.length;
                /*method*/value?.method();
                /*nestedArgument*/identity(value?.length);
            }
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    for (marker, expression, expected) in [
        ("member", "value?.length", Number),
        ("method", "value?.method()", String),
        ("argument", "identity(value)", String),
        ("otherArgument", "identity(value)", Number),
        ("nestedArgument", "identity(value?.length)", Number),
    ] {
        let ty = normalized_type_at(&db, module, SOURCE, marker, expression);
        assert_variants(&db, ty, &[expected], marker);
    }
}

#[test]
fn narrowing_is_isolated_by_binding_identity() {
    use InferredTypeData::{Null, Number, String};
    const SOURCE: &str = r#"
        declare function read(): number | null;
        function inspect(value: string | number) {
            if (typeof value === "string") {
                {
                    const value = read();
                    if (value !== null) {
                        /*inner*/value;
                    }
                    /*innerJoin*/value;
                }
                /*outer*/value;
            }
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    for (marker, expected) in [
        ("inner", &[Number][..]),
        ("innerJoin", &[Number, Null]),
        ("outer", &[String]),
    ] {
        let ty = normalized_type_at(&db, module, SOURCE, marker, "value");
        assert_variants(&db, ty, expected, marker);
    }
}

#[test]
fn writes_captures_and_try_finally_roots_remain_conservative() {
    use InferredTypeData::{Number, String};
    const SOURCE: &str = r#"
        function reassigned(value: string | number) {
            if (typeof value === "string") {
                value = 0;
                /*written*/value;
            }
        }
        function captured(value: string | number) {
            if (typeof value === "string") {
                return () => /*captured*/value;
            }
        }
        function unsupported(value: string | number) {
            try {
                if (typeof value === "string") {
                    /*try*/value;
                }
            } finally {
                /*finally*/value;
            }
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    for marker in ["written", "captured", "try", "finally"] {
        let ty = normalized_type_at(&db, module, SOURCE, marker, "value");
        assert_variants(&db, ty, &[String, Number], marker);
    }
}

#[test]
fn structural_types_keep_falsy_primitive_possibilities() {
    const SOURCE: &str = r#"
        interface Value {}
        function inspect(value: Value | null) {
            if (!value) {
                /*interface*/value;
            }
        }
        function anonymous(value: {} | null) {
            if (!value) {
                /*object*/value;
            }
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    for marker in ["interface", "object"] {
        let ty = normalized_type_at(&db, module, SOURCE, marker, "value");
        assert!(contains_inferred_null(&db, ty));
        assert_ne!(
            ty,
            InferredTypeData::Null,
            "{marker}: primitives must remain possible"
        );
    }
}

#[test]
fn consumers_keep_structural_truthiness_and_void_results_uncertain() {
    const SOURCE: &str = r#"
        interface Value {}
        class ValueClass {}
        function anonymous(value: {} | null) {
            if (value == null) return;
            /*object*/value;
        }
        function named(value: Value | null) {
            if (value == null) return;
            /*interface*/value;
        }
        function classShape(value: ValueClass | null) {
            if (value == null) return;
            /*class*/value;
        }
        function ignoredResult(value: string | void) {
            if (typeof value !== "string") {
                /*void*/value;
            }
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    for marker in ["object", "interface", "class"] {
        let ty = normalized_type_at(&db, module, SOURCE, marker, "value");
        assert!(
            !biome_js_type_info::InferredType::new(&db, ty).is_always_truthy(),
            "{marker}"
        );
    }
    assert_eq!(
        normalized_type_at(&db, module, SOURCE, "void", "value"),
        InferredTypeData::Unknown
    );
}

#[test]
fn hoisted_initialization_and_escaped_dynamic_names_prevent_stale_facts() {
    const SOURCE: &str = r#"
        function hoisted(flag: boolean) {
            if (typeof value !== "object") {
                var value: string | null = flag ? "ready" : null;
                /*hoisted*/value;
            }
        }
        function dynamic(value: string | null) {
            if (value !== null) {
                \u0065val("value = null");
                /*eval*/value;
            }
        }
        function mapped(value: string | null) {
            if (value !== null) {
                \u0061rguments[0] = null;
                /*arguments*/value;
            }
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    for marker in ["hoisted", "eval", "arguments"] {
        let ty = normalized_type_at(&db, module, SOURCE, marker, "value");
        assert_variants(
            &db,
            ty,
            &[InferredTypeData::String, InferredTypeData::Null],
            marker,
        );
    }
}

#[test]
fn promise_classification_uses_narrowed_identifiers_without_whole_module_inference() {
    use TypeInferenceClassification::{Match, NoMatch};
    const SOURCE: &str = r#"
        function nullable(value: Promise<void> | null) {
            if (value === null) {
                /*null*/value;
            } else {
                /*promise*/value;
            }
        }
        function primitive(value: Promise<void> | string) {
            if (typeof value === "string") {
                /*string*/value;
            } else {
                /*otherPromise*/value;
            }
        }
    "#;
    let (db, module) = narrowing_db(SOURCE);
    for (marker, expected) in [
        ("null", NoMatch),
        ("promise", Match),
        ("string", NoMatch),
        ("otherPromise", Match),
    ] {
        let expression = marked_range(SOURCE, marker, "value");
        let input = ExpressionTypeInput::new(&db, module, expression);
        for cold in [true, false] {
            db.clear_salsa_events();
            assert_eq!(
                execute_type_inference_request(
                    &db,
                    TypeInferenceCaller::new("test", "narrowedPromise"),
                    PromiseClassificationRequest::new(module, expression),
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
            assert_function_query_was_not_run(&db, infer_module_types, module, &events);
        }
    }
}

#[test]
fn narrowing_requests_reuse_warm_and_unrelated_inputs_and_invalidate_changed_guards() {
    use InferredTypeData::{Number, String};
    const SOURCE: &str = r#"
        function inspect(value: string | number) {
            if (typeof value === "string") {
                /*read*/value;
            }
        }
    "#;
    let changed_source = SOURCE.replace("===", "!==");
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
        let ty = normalized_type_at(&db, module, SOURCE, "read", "value");
        assert_variants(&db, ty, &[String], "read");
        let events = db.take_salsa_events();
        let input = ExpressionTypeInput::new(&db, module, expression);
        if cold {
            assert_function_query_was_run(&db, infer_expression_type, input, &events);
        } else {
            assert_function_query_was_not_run(&db, infer_expression_type, input, &events);
        }
        assert_eq!(
            function_query_will_execute_count_by_name(&db, "infer_flow_binding_type", &events),
            usize::from(cold)
        );
        assert_function_query_was_not_run(&db, infer_module_types, module, &events);
    }

    fs.insert("/src/unrelated.ts".into(), "export const unused = false;");
    let kind = resolve_js_module_kind_for_test(&fs, "/src/unrelated.ts", true);
    salsa::Setter::to(unrelated.set_kind(&mut db), kind);
    db.clear_salsa_events();
    let ty = normalized_type_at(&db, module, SOURCE, "read", "value");
    assert_variants(&db, ty, &[String], "unrelated edit");
    let events = db.take_salsa_events();
    let input = ExpressionTypeInput::new(&db, module, expression);
    assert_function_query_was_not_run(&db, infer_expression_type, input, &events);
    assert_no_flow_queries(&db, &events);
    for module in [module, unrelated] {
        assert_function_query_was_not_run(&db, infer_module_types, module, &events);
    }

    fs.insert("/src/index.ts".into(), changed_source.as_str());
    let kind = resolve_js_module_kind_for_test(&fs, "/src/index.ts", true);
    salsa::Setter::to(module.set_kind(&mut db), kind);
    db.clear_salsa_events();
    let ty = normalized_type_at(&db, module, &changed_source, "read", "value");
    assert_variants(&db, ty, &[Number], "changed guard");
    let events = db.take_salsa_events();
    let input = ExpressionTypeInput::new(&db, module, expression);
    assert_function_query_was_run(&db, infer_expression_type, input, &events);
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "infer_flow_binding_type", &events),
        1
    );
    for module in [module, unrelated] {
        assert_function_query_was_not_run(&db, infer_module_types, module, &events);
    }
}

#[test]
fn flow_candidates_include_nested_expression_and_loop_conditions() {
    use InferredTypeData::{Null, Number, String};
    for (body, expected) in [
        (
            r#"return true && (value !== null && (typeof value === "string" && /*read*/value));"#,
            String,
        ),
        (
            r#"return true ? (typeof value === "number" ? /*read*/value : null) : null;"#,
            Number,
        ),
        (
            r#"while (value !== null && typeof value === "string") { /*read*/value; }"#,
            String,
        ),
        (
            r#"for (; value !== null && typeof value === "string";) { /*read*/value; }"#,
            String,
        ),
        ("do {} while (value !== null); /*read*/value;", Null),
    ] {
        let source = format!("function inspect(value: string | number | null) {{ {body} }}");
        let (db, module) = narrowing_db(&source);
        db.clear_salsa_events();
        assert_eq!(
            normalized_type_at(&db, module, &source, "read", "value"),
            expected,
            "{body}"
        );
        let events = db.take_salsa_events();
        assert_function_query_was_not_run(&db, infer_module_types, module, &events);
    }
}

#[test]
fn adding_and_removing_conditions_invalidates_flow_candidates_with_equal_semantics() {
    use InferredTypeData::{Null, String};
    const SOURCE: &str =
        "function inspect(value: string | null) {    (value !== null);{ /*read*/value; } }";
    let guarded_source = SOURCE.replace("   (value !== null);", "if (value !== null) ");
    let expression = marked_range(SOURCE, "read", "value");
    assert_eq!(expression, marked_range(&guarded_source, "read", "value"));
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);
    let mut db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    let ModuleInfoKind::Js(info) = module.kind(&db) else {
        panic!("module must contain JavaScript information");
    };
    let semantic_model = info.semantic_model.clone();

    for (source, guarded) in [
        (SOURCE, false),
        (guarded_source.as_str(), true),
        (SOURCE, false),
    ] {
        fs.insert("/src/index.ts".into(), source);
        let kind = resolve_js_module_kind_for_test(&fs, "/src/index.ts", true);
        let ModuleInfoKind::Js(info) = &kind else {
            panic!("module must contain JavaScript information");
        };
        assert_eq!(info.semantic_model, semantic_model);
        salsa::Setter::to(module.set_kind(&mut db), kind);
        db.clear_salsa_events();
        let ty = normalized_type_at(&db, module, source, "read", "value");
        let expected = if guarded {
            &[String][..]
        } else {
            &[String, Null]
        };
        assert_variants(&db, ty, expected, source);
        let events = db.take_salsa_events();
        assert_eq!(
            function_query_will_execute_count_by_name(&db, "flow_candidates_for_module", &events),
            1,
            "condition edits must invalidate the syntax-derived candidate index"
        );
        if guarded {
            assert_eq!(
                function_query_will_execute_count_by_name(&db, "narrowing_flow_for_root", &events),
                1
            );
        } else {
            assert_no_flow_queries(&db, &events);
        }
        let input = ExpressionTypeInput::new(&db, module, expression);
        assert_function_query_was_run(&db, infer_expression_type, input, &events);
        assert_function_query_was_not_run(&db, infer_module_types, module, &events);
    }
}
