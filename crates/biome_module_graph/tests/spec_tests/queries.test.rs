use super::*;
use biome_module_graph::{
    BindingTypeInput, ExpressionTypeInput, LocalTypeInput, SymbolFromModuleInfo, find_member_type,
    function_returns_promise, infer_binding_type, infer_export_type,
    infer_expression_function_returns_promise, infer_expression_is_array_of_promises,
    infer_expression_is_promise, infer_expression_type, infer_local_type, is_array_of_promise_type,
    is_promise_type, resolve_callable_type, type_inference::TypeInferenceClassification,
};

fn local_type_id_by_name(db: &dyn ModuleDb, module: ModuleInfo, name: &str) -> InferredLocalTypeId {
    let ModuleInfoKind::Js(js_info) = module.kind(db) else {
        panic!("module must contain JavaScript information");
    };
    (0..js_info.raw_types.len())
        .map(InferredLocalTypeId::new)
        .find(|type_id| {
            js_info
                .local_type_name(*type_id)
                .is_some_and(|type_name| type_name.text() == name)
        })
        .unwrap_or_else(|| panic!("{name} local type must exist"))
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

const BUDGETED_BINDING_QUERY: &str = "infer_binding_type_with_import_budget";
const IMPORT_DEPTH_PREPARATION_QUERY: &str = "prepare_module_types_bottom_up_for_import_depth";

#[test]
fn test_binding_query_does_not_infer_complete_module_tables() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), "export const value = 1;");

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let range = binding_range_by_name(&db, module, "value");
    let input = BindingTypeInput::new(&db, module, range);

    db.clear_salsa_events();
    let ty = infer_binding_type(&db, input).expect("value type must be inferred");
    assert!(is_inferred_number(&db, ty));
    let events = db.take_salsa_events();

    assert_function_query_was_run(&db, infer_binding_type, input, &events);
    assert_function_query_was_not_run(&db, infer_module_types, module, &events);
}

#[test]
fn test_binding_query_resolves_imports_without_complete_module_inference() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/source.ts".into(), "export const value = 1;");
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import { value } from "./source.ts";
            export const result = value;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/source.ts", "/src/index.ts"], true);
    let source_module = db
        .module_for_path(Utf8Path::new("/src/source.ts"))
        .expect("source module must exist");
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("index module must exist");
    let range = binding_range_by_name(&db, index_module, "result");
    let input = BindingTypeInput::new(&db, index_module, range);

    db.clear_salsa_events();
    let ty = infer_binding_type(&db, input).expect("result type must be inferred");
    assert!(is_inferred_number(&db, ty));
    let events = db.take_salsa_events();

    assert_function_query_was_not_run(&db, infer_module_types, index_module, &events);
    assert_function_query_was_not_run(&db, infer_module_types, source_module, &events);
}

#[test]
fn test_binding_query_keeps_local_export_granular_beside_deep_import_branch() {
    const IMPORT_COUNT: usize = 129;

    let fs = MemoryFileSystem::default();
    let mut paths = (0..=IMPORT_COUNT)
        .map(|index| format!("/src/branch{index}.ts"))
        .collect::<Vec<_>>();
    for (index, path) in paths.iter().enumerate().take(IMPORT_COUNT) {
        fs.insert(
            path.clone().into(),
            format!("import './branch{}.ts';", index + 1),
        );
    }
    fs.insert(paths[IMPORT_COUNT].clone().into(), "export {};");
    fs.insert(
        "/src/source.ts".into(),
        "import './branch0.ts'; export const value = 1;",
    );
    fs.insert(
        "/src/index.ts".into(),
        "import { value } from './source.ts'; export const result = value;",
    );
    paths.extend(["/src/source.ts".to_string(), "/src/index.ts".to_string()]);
    let path_refs = paths.iter().map(String::as_str).collect::<Vec<_>>();
    let db = build_js_test_module_db(&fs, &path_refs, true);
    let source = db
        .module_for_path(Utf8Path::new("/src/source.ts"))
        .expect("source module must exist");
    let index = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("index module must exist");
    let input = BindingTypeInput::new(&db, index, binding_range_by_name(&db, index, "result"));

    db.clear_salsa_events();
    let ty = infer_binding_type(&db, input).expect("result type must be inferred");
    assert!(is_inferred_number(&db, ty));
    let events = db.take_salsa_events();
    assert_function_query_was_not_run(&db, infer_module_types, source, &events);
    assert_function_query_was_not_run(&db, infer_module_types, index, &events);
    assert_eq!(
        function_query_will_execute_count_by_name(&db, IMPORT_DEPTH_PREPARATION_QUERY, &events,),
        0
    );
}

#[test]
fn test_binding_queries_reuse_a_deep_import_chain_across_roots() {
    const IMPORT_COUNT: usize = 129;
    const ROOT_COUNT: usize = 64;

    let fs = MemoryFileSystem::default();
    let mut paths = (0..=IMPORT_COUNT)
        .map(|index| format!("/src/chain{index}.ts"))
        .collect::<Vec<_>>();
    for (index, path) in paths.iter().enumerate().take(IMPORT_COUNT) {
        fs.insert(
            path.clone().into(),
            format!(
                "import {{ value as next }} from './chain{}.ts'; export const value = next;",
                index + 1
            ),
        );
    }
    fs.insert(
        paths[IMPORT_COUNT].clone().into(),
        "export const value = 1;",
    );
    for root_index in 0..ROOT_COUNT {
        let path = format!("/src/root{root_index}.ts");
        fs.insert(
            path.clone().into(),
            "import { value } from './chain0.ts'; export const result = value;",
        );
        paths.push(path);
    }

    let path_refs = paths.iter().map(String::as_str).collect::<Vec<_>>();
    let db = build_js_test_module_db(&fs, &path_refs, true);
    let roots = (0..ROOT_COUNT)
        .map(|root_index| {
            db.module_for_path(Utf8Path::new(&format!("/src/root{root_index}.ts")))
                .expect("root module must exist")
        })
        .collect::<Vec<_>>();

    let first_root = roots[0];
    let first_input = BindingTypeInput::new(
        &db,
        first_root,
        binding_range_by_name(&db, first_root, "result"),
    );
    db.clear_salsa_events();
    let ty = infer_binding_type(&db, first_input).expect("result type must be inferred");
    assert!(is_inferred_number(&db, ty));
    let events = db.take_salsa_events();
    assert_eq!(
        function_query_will_execute_count_by_name(&db, BUDGETED_BINDING_QUERY, &events),
        IMPORT_COUNT - 1
    );
    assert_eq!(
        function_query_will_execute_count_by_name(&db, IMPORT_DEPTH_PREPARATION_QUERY, &events,),
        1
    );

    db.clear_salsa_events();
    for root in roots.into_iter().skip(1) {
        let input = BindingTypeInput::new(&db, root, binding_range_by_name(&db, root, "result"));
        let ty = infer_binding_type(&db, input).expect("result type must be inferred");
        assert!(is_inferred_number(&db, ty));
    }
    let events = db.take_salsa_events();
    assert_eq!(
        function_query_will_execute_count_by_name(&db, BUDGETED_BINDING_QUERY, &events),
        0
    );
    assert_eq!(
        function_query_will_execute_count_by_name(&db, IMPORT_DEPTH_PREPARATION_QUERY, &events,),
        0
    );
}

#[test]
fn test_binding_query_bounds_fallbacks_in_a_dense_import_graph() {
    const MODULE_COUNT: usize = 160;
    const IMPORT_FANOUT: usize = 8;
    const MAX_DISTINCT_CUTOFF_MODULES: usize = MODULE_COUNT - 128;

    let fs = MemoryFileSystem::default();
    let paths = (0..MODULE_COUNT)
        .map(|index| format!("/src/dense{index}.ts"))
        .collect::<Vec<_>>();
    fs.insert(
        paths[0].clone().into(),
        "export const value0 = { leaf: 1 };",
    );
    for (index, path) in paths.iter().enumerate().skip(1) {
        let dependencies = (index.saturating_sub(IMPORT_FANOUT)..index).collect::<Vec<_>>();
        let imports = dependencies
            .iter()
            .map(|dependency| {
                format!("import {{ value{dependency} }} from './dense{dependency}.ts';")
            })
            .collect::<Vec<_>>()
            .join("\n");
        let members = dependencies
            .iter()
            .map(|dependency| format!("value{dependency}"))
            .collect::<Vec<_>>()
            .join(", ");
        fs.insert(
            path.clone().into(),
            format!("{imports}\nexport const value{index} = {{ {members} }};"),
        );
    }

    let path_refs = paths.iter().map(String::as_str).collect::<Vec<_>>();
    let db = build_js_test_module_db(&fs, &path_refs, true);
    let root = db
        .module_for_path(Utf8Path::new(&paths[MODULE_COUNT - 1]))
        .expect("root module must exist");
    let input = BindingTypeInput::new(
        &db,
        root,
        binding_range_by_name(&db, root, &format!("value{}", MODULE_COUNT - 1)),
    );

    db.clear_salsa_events();
    let ty = infer_binding_type(&db, input).expect("root type must be inferred");
    assert!(InferredType::new(&db, ty).is_inferred());
    let events = db.take_salsa_events();
    let preparation_count =
        function_query_will_execute_count_by_name(&db, IMPORT_DEPTH_PREPARATION_QUERY, &events);
    assert!(
        (1..=MAX_DISTINCT_CUTOFF_MODULES).contains(&preparation_count),
        "fallback preparation must be bounded by cutoff modules, got {preparation_count}"
    );

    db.clear_salsa_events();
    let ty = infer_binding_type(&db, input).expect("cached root type must be inferred");
    assert!(InferredType::new(&db, ty).is_inferred());
    let events = db.take_salsa_events();
    assert_eq!(
        function_query_will_execute_count_by_name(&db, IMPORT_DEPTH_PREPARATION_QUERY, &events,),
        0
    );
}

#[test]
fn test_namespace_query_keeps_named_exports_symbolic() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/source.ts".into(), "export class Value { field = 1; }");
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import * as source from "./source.ts";
            export { source };
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/source.ts", "/src/index.ts"], true);
    let source_module = db
        .module_for_path(Utf8Path::new("/src/source.ts"))
        .expect("source module must exist");
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("index module must exist");
    let source_binding = BindingTypeInput::new(
        &db,
        index_module,
        binding_range_by_name(&db, index_module, "source"),
    );
    let value_binding = BindingTypeInput::new(
        &db,
        source_module,
        binding_range_by_name(&db, source_module, "Value"),
    );

    db.clear_salsa_events();
    let namespace = infer_binding_type(&db, source_binding).expect("namespace must be inferred");
    let InferredTypeData::Namespace(namespace) = namespace else {
        panic!("namespace import must infer a namespace");
    };
    let value = namespace
        .members(&db)
        .iter()
        .find(|member| member.kind.has_name("Value"))
        .expect("namespace must contain Value");
    assert!(matches!(value.ty, InferredTypeData::Local(_)));
    let events = db.take_salsa_events();

    assert_function_query_was_not_run(&db, infer_binding_type, value_binding, &events);
    assert_function_query_was_not_run(&db, infer_module_types, source_module, &events);
}

#[test]
fn test_member_lookup_resolves_local_types_without_complete_module_inference() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            interface Value {
                field: string;
            }
            export const value: Value = { field: "value" };
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let range = binding_range_by_name(&db, module, "value");
    let input = BindingTypeInput::new(&db, module, range);
    let value_ty = infer_binding_type(&db, input).expect("value type must be inferred");

    db.clear_salsa_events();
    let field_ty = find_member_type(&db, value_ty, "field").expect("field type must be inferred");
    assert!(is_inferred_string(&db, field_ty));
    let events = db.take_salsa_events();

    assert_function_query_was_not_run(&db, infer_module_types, module, &events);
}

#[test]
fn test_callable_type_resolution_skips_interface_parameters_and_siblings() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            interface Noise {
                nested: string;
            }
            interface Handler {
                (value: Noise): void;
                unrelated: Noise;
            }
            declare const handler: Handler;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let handler_input =
        BindingTypeInput::new(&db, module, binding_range_by_name(&db, module, "handler"));
    let handler = infer_binding_type(&db, handler_input).expect("handler type must be inferred");
    let noise = LocalTypeInput::new(&db, module, local_type_id_by_name(&db, module, "Noise"));

    db.clear_salsa_events();
    let callable = resolve_callable_type(&db, handler).expect("Handler must be callable");
    assert!(InferredType::new(&db, callable).function_returns_void());
    let events = db.take_salsa_events();

    assert_function_query_was_not_run(&db, infer_local_type, noise, &events);
    assert_function_query_was_not_run(&db, infer_module_types, module, &events);
}

#[test]
fn test_member_lookup_rejects_stale_local_handles_after_module_replacement() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        "export interface Value { field: string; }",
    );

    let mut db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let path = Utf8PathBuf::from("/src/index.ts");
    let original = db.module_for_path(&path).expect("module must exist");
    let original_key = InferredModuleKey::new(original.as_id());
    let type_id = local_type_id_by_name(&db, original, "Value");
    let replacement = ModuleInfo::new(&db, path.clone(), original.kind(&db).clone());
    db.modules.insert(path, replacement);
    let stale = InferredTypeData::Local(biome_js_type_info::interned_types::LocalTypeHandle::new(
        &db,
        original_key,
        type_id,
    ));

    assert!(find_member_type(&db, stale, "field").is_none());
}

#[test]
fn test_promise_classification_query_skips_instance_type_arguments() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            interface Noise {
                nested: string;
            }
            declare const values: Array<Noise>;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let binding = BindingTypeInput::new(&db, module, binding_range_by_name(&db, module, "values"));
    let values = infer_binding_type(&db, binding).expect("values type must be inferred");
    let noise = LocalTypeInput::new(&db, module, local_type_id_by_name(&db, module, "Noise"));

    db.clear_salsa_events();
    assert_eq!(is_promise_type(&db, values), Some(false));
    let events = db.take_salsa_events();

    assert_function_query_was_not_run(&db, infer_local_type, noise, &events);
}

#[test]
fn test_array_promise_classification_query_skips_promise_type_arguments() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            interface Noise {
                nested: string;
            }
            declare const values: Array<Promise<Noise>>;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let binding = BindingTypeInput::new(&db, module, binding_range_by_name(&db, module, "values"));
    let values = infer_binding_type(&db, binding).expect("values type must be inferred");
    let noise = LocalTypeInput::new(&db, module, local_type_id_by_name(&db, module, "Noise"));

    db.clear_salsa_events();
    assert_eq!(is_array_of_promise_type(&db, values), Some(true));
    let events = db.take_salsa_events();

    assert_function_query_was_not_run(&db, infer_local_type, noise, &events);
}

#[test]
fn test_promise_return_classification_query_resolves_return_but_skips_parameters() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            interface Noise {
                nested: string;
            }
            type Result = Promise<void>;
            type Callback = (value: Noise) => Result;
            declare const callback: Callback;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let binding =
        BindingTypeInput::new(&db, module, binding_range_by_name(&db, module, "callback"));
    let callback = infer_binding_type(&db, binding).expect("callback type must be inferred");
    let callback_input =
        LocalTypeInput::new(&db, module, local_type_id_by_name(&db, module, "Callback"));
    let result = LocalTypeInput::new(&db, module, local_type_id_by_name(&db, module, "Result"));
    let noise = LocalTypeInput::new(&db, module, local_type_id_by_name(&db, module, "Noise"));

    db.clear_salsa_events();
    assert_eq!(function_returns_promise(&db, callback), Some(true));
    let events = db.take_salsa_events();

    assert_function_query_was_run(&db, infer_local_type, callback_input, &events);
    assert_function_query_was_run(&db, infer_local_type, result, &events);
    assert_function_query_was_not_run(&db, infer_local_type, noise, &events);
}

#[test]
fn test_expression_function_return_query_skips_expression_and_sibling_type_queries() {
    const SOURCE: &str = r#"
        interface Noise {
            nested: string;
        }
        declare function consume(value: unknown): void;
        class Runner {
            #settings = {
                config: { cacheDir: "/tmp", unrelated: null as unknown as Noise }
            };

            run() {
                consume({ image: "", syntaxHighlight: async () => {}, unrelated: null as unknown as Noise });
                consume(this.#settings.config.cacheDir);
            }
        }
    "#;
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let object = ExpressionTypeInput::new(
        &db,
        module,
        expression_range_by_source(
            &db,
            module,
            SOURCE,
            r#"{ image: "", syntaxHighlight: async () => {}, unrelated: null as unknown as Noise }"#,
        ),
    );
    let cache_dir = ExpressionTypeInput::new(
        &db,
        module,
        expression_range_by_source(&db, module, SOURCE, "this.#settings.config.cacheDir"),
    );
    let noise = LocalTypeInput::new(&db, module, local_type_id_by_name(&db, module, "Noise"));

    db.clear_salsa_events();
    assert_eq!(
        infer_expression_function_returns_promise(&db, object),
        TypeInferenceClassification::NoMatch
    );
    assert_eq!(
        infer_expression_function_returns_promise(&db, cache_dir),
        TypeInferenceClassification::NoMatch
    );
    let events = db.take_salsa_events();

    assert_function_query_was_not_run(&db, infer_expression_type, object, &events);
    assert_function_query_was_not_run(&db, infer_expression_type, cache_dir, &events);
    assert_function_query_was_not_run(&db, infer_local_type, noise, &events);
}

#[test]
fn test_expression_function_return_query_skips_returned_call_arguments() {
    const SOURCE: &str = r#"
        import type { Noise } from "./noise.ts";
        declare function invoke(callback: (value: Noise) => void): Promise<void>;
        declare function consume(value: unknown): void;
        const callback = () => invoke((value: Noise) => { void value; });
        consume(callback);
    "#;
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/noise.ts".into(),
        "export interface Noise { nested: string; }",
    );
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/noise.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("index module must exist");
    let noise_module = db
        .module_for_path(Utf8Path::new("/src/noise.ts"))
        .expect("noise module must exist");
    let expression = ExpressionTypeInput::new(
        &db,
        index_module,
        expression_range_by_source(&db, index_module, SOURCE, "callback"),
    );
    let noise = LocalTypeInput::new(
        &db,
        noise_module,
        local_type_id_by_name(&db, noise_module, "Noise"),
    );

    db.clear_salsa_events();
    assert_eq!(
        infer_expression_function_returns_promise(&db, expression),
        TypeInferenceClassification::Match
    );
    let events = db.take_salsa_events();

    assert_function_query_was_not_run(&db, infer_expression_type, expression, &events);
    assert_function_query_was_not_run(&db, infer_local_type, noise, &events);
    assert_function_query_was_not_run(&db, infer_module_types, noise_module, &events);
}

#[test]
fn test_expression_function_return_query_follows_only_interface_call_signature() {
    const SOURCE: &str = r#"
        interface Noise {
            nested: string;
        }
        type Result = Promise<void>;
        interface AsyncHandler {
            (value: Noise): Result;
            unrelated: Noise;
        }
        type AsyncObject = {
            (value: Noise): Result;
            unrelated: Noise;
        };
        declare const callback: AsyncHandler;
        declare const objectCallback: AsyncObject;
        declare function consume(value: unknown): void;
        consume(callback);
        consume(objectCallback);
    "#;
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let expression = ExpressionTypeInput::new(
        &db,
        module,
        expression_range_by_source(&db, module, SOURCE, "callback"),
    );
    let object_expression = ExpressionTypeInput::new(
        &db,
        module,
        expression_range_by_source(&db, module, SOURCE, "objectCallback"),
    );
    let noise = LocalTypeInput::new(&db, module, local_type_id_by_name(&db, module, "Noise"));

    db.clear_salsa_events();
    assert_eq!(
        infer_expression_function_returns_promise(&db, expression),
        TypeInferenceClassification::Match
    );
    assert_eq!(
        infer_expression_function_returns_promise(&db, object_expression),
        TypeInferenceClassification::Match
    );
    let events = db.take_salsa_events();

    assert_function_query_was_not_run(&db, infer_expression_type, expression, &events);
    assert_function_query_was_not_run(&db, infer_expression_type, object_expression, &events);
    assert_function_query_was_not_run(&db, infer_local_type, noise, &events);
    assert_function_query_was_not_run(&db, infer_module_types, module, &events);
}

#[test]
fn test_expression_array_promise_query_skips_sibling_type_queries() {
    const SOURCE: &str = r#"
        interface Noise {
            nested: string;
        }
        const callbacks = {
            promises: (): Array<Promise<void>> => [],
            unrelated: null as unknown as Noise,
        };
        callbacks.promises();
    "#;
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let expression = ExpressionTypeInput::new(
        &db,
        module,
        expression_range_by_source(&db, module, SOURCE, "callbacks.promises()"),
    );
    let noise = LocalTypeInput::new(&db, module, local_type_id_by_name(&db, module, "Noise"));

    db.clear_salsa_events();
    assert_eq!(
        infer_expression_is_array_of_promises(&db, expression),
        TypeInferenceClassification::Match
    );
    let events = db.take_salsa_events();

    assert_function_query_was_not_run(&db, infer_expression_type, expression, &events);
    assert_function_query_was_not_run(&db, infer_local_type, noise, &events);
}

#[test]
fn test_expression_array_promise_query_skips_returned_call_arguments() {
    const SOURCE: &str = r#"
        import { argument } from "./argument.ts";
        declare function invoke(value: number): Array<Promise<void>>;
        declare function invokeAwaited(value: number): Promise<Array<Promise<void>>>;
        declare function invokeGeneric<T>(value: T): Array<Promise<void>>;
        declare function invokeArray<T>(value: T): Array<T>;
        declare function identity<T>(value: T): T;
        interface Holder<T> {
            get(): T;
        }
        declare const holder: Holder<Array<Promise<void>>>;
        const callback = () => invoke(argument);
        const awaitedCallback = () => invokeAwaited(argument);
        const asyncCallback = async () => invoke(argument);
        const genericIndependentCallback = () => invokeGeneric(argument);
        const genericArrayCallback = () => invokeArray(Promise.resolve());
        const genericCallback = () => identity(Promise.resolve([Promise.resolve()]));
        const holderCallback = () => holder.get();
        callback();
        await awaitedCallback();
        await asyncCallback();
        genericIndependentCallback();
        genericArrayCallback();
        await genericCallback();
        holderCallback();
    "#;
    let fs = MemoryFileSystem::default();
    fs.insert("/src/argument.ts".into(), "export const argument = 1;");
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/argument.ts", "/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let expressions = [
        "callback()",
        "await awaitedCallback()",
        "await asyncCallback()",
        "genericIndependentCallback()",
    ]
    .map(|source| {
        ExpressionTypeInput::new(
            &db,
            module,
            expression_range_by_source(&db, module, SOURCE, source),
        )
    });

    db.clear_salsa_events();
    for expression in expressions {
        assert_eq!(
            infer_expression_is_array_of_promises(&db, expression),
            TypeInferenceClassification::Match
        );
    }
    let generic_expression = ExpressionTypeInput::new(
        &db,
        module,
        expression_range_by_source(&db, module, SOURCE, "await genericCallback()"),
    );
    assert_eq!(
        infer_expression_is_array_of_promises(&db, generic_expression),
        TypeInferenceClassification::Indeterminate
    );
    let generic_array_expression = ExpressionTypeInput::new(
        &db,
        module,
        expression_range_by_source(&db, module, SOURCE, "genericArrayCallback()"),
    );
    assert_eq!(
        infer_expression_is_array_of_promises(&db, generic_array_expression),
        TypeInferenceClassification::Indeterminate
    );
    let holder_expression = ExpressionTypeInput::new(
        &db,
        module,
        expression_range_by_source(&db, module, SOURCE, "holderCallback()"),
    );
    assert_eq!(
        infer_expression_is_array_of_promises(&db, holder_expression),
        TypeInferenceClassification::Indeterminate
    );
    let events = db.take_salsa_events();

    assert_eq!(
        function_query_will_execute_count_by_name(&db, BUDGETED_BINDING_QUERY, &events),
        0
    );
}

#[test]
fn test_expression_array_promise_query_unwraps_awaited_function_returns_selectively() {
    const SOURCE: &str = r#"
        interface Noise {
            nested: string;
        }
        async function asyncValues(value: Noise): Promise<Array<Promise<void>>> {
            return [];
        }
        function syncValues(value: Noise): Promise<Array<Promise<void>>> {
            return Promise.resolve([]);
        }
        declare const directValues: Promise<Array<Promise<void>>>;
        declare const uncertain: unknown;
        declare function overloaded(): Promise<Array<Promise<void>>>;
        declare function overloaded(value: string): Promise<Array<Promise<void>>>;

        await asyncValues(null as never);
        await syncValues(null as never);
        await directValues;
        await await asyncValues(null as never);
        await uncertain;
        await overloaded();
    "#;
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inputs = [
        (
            "await asyncValues(null as never)",
            TypeInferenceClassification::Match,
        ),
        (
            "await syncValues(null as never)",
            TypeInferenceClassification::Match,
        ),
        ("await directValues", TypeInferenceClassification::Match),
        (
            "await await asyncValues(null as never)",
            TypeInferenceClassification::Match,
        ),
        ("await uncertain", TypeInferenceClassification::NoMatch),
        (
            "await overloaded()",
            TypeInferenceClassification::Indeterminate,
        ),
    ]
    .map(|(source, expected)| {
        (
            source,
            ExpressionTypeInput::new(
                &db,
                module,
                expression_range_by_source(&db, module, SOURCE, source),
            ),
            expected,
        )
    });
    let noise = LocalTypeInput::new(&db, module, local_type_id_by_name(&db, module, "Noise"));

    db.clear_salsa_events();
    for (source, input, expected) in inputs {
        assert_eq!(
            infer_expression_is_array_of_promises(&db, input),
            expected,
            "{source}"
        );
    }
    let events = db.take_salsa_events();

    for (_, input, _) in inputs {
        assert_function_query_was_not_run(&db, infer_expression_type, input, &events);
    }
    assert_function_query_was_not_run(&db, infer_local_type, noise, &events);
    assert_function_query_was_not_run(&db, infer_module_types, module, &events);
}

#[test]
fn test_expression_promise_queries_follow_imported_interface_members() {
    const SOURCE: &str = r#"
        import type { LoaderContext } from "./types";
        declare const context: LoaderContext;
        context.store.set({ id: "entry" });
    "#;
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/types.ts".into(),
        r#"
            interface Noise {
                nested: string;
            }
            interface DataStore {
                set(value: { id: string }): void;
            }
            export interface LoaderContext {
                store: DataStore;
                unrelated: Noise;
            }
        "#,
    );
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/types.ts", "/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let types_module = db
        .module_for_path(Utf8Path::new("/src/types.ts"))
        .expect("types module must exist");
    let expression = ExpressionTypeInput::new(
        &db,
        module,
        expression_range_by_source(&db, module, SOURCE, "context.store.set({ id: \"entry\" })"),
    );
    let noise = LocalTypeInput::new(
        &db,
        types_module,
        local_type_id_by_name(&db, types_module, "Noise"),
    );

    db.clear_salsa_events();
    assert_eq!(
        infer_expression_is_promise(&db, expression),
        TypeInferenceClassification::NoMatch
    );
    assert_eq!(
        infer_expression_is_array_of_promises(&db, expression),
        TypeInferenceClassification::NoMatch
    );
    let events = db.take_salsa_events();

    assert_function_query_was_not_run(&db, infer_expression_type, expression, &events);
    assert_function_query_was_not_run(&db, infer_local_type, noise, &events);
}

#[test]
fn test_expression_promise_queries_match_unknown_node_builtin_imports() {
    const SOURCE: &str = r#"
        import fs from "node:fs/promises";
        async function createDirectory() {
            await fs.mkdir("/tmp/example", { recursive: true });
        }
    "#;
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let expression = ExpressionTypeInput::new(
        &db,
        module,
        expression_range_by_source(
            &db,
            module,
            SOURCE,
            "await fs.mkdir(\"/tmp/example\", { recursive: true })",
        ),
    );

    db.clear_salsa_events();
    assert_eq!(
        infer_expression_is_promise(&db, expression),
        TypeInferenceClassification::NoMatch
    );
    assert_eq!(
        infer_expression_is_array_of_promises(&db, expression),
        TypeInferenceClassification::NoMatch
    );
    let events = db.take_salsa_events();

    assert_function_query_was_not_run(&db, infer_expression_type, expression, &events);
}

#[test]
fn test_export_query_does_not_infer_complete_module_tables() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), "export const value = 1;");

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let symbol = SymbolFromModuleInfo::new(&db, "value", module);

    db.clear_salsa_events();
    let ty = infer_export_type(&db, symbol).expect("export type must be inferred");
    assert!(is_inferred_number(&db, ty));
    let events = db.take_salsa_events();

    assert_function_query_was_not_run(&db, infer_module_types, module, &events);
}

#[test]
fn test_granular_type_queries_match_module_inference_and_are_memoized() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            interface Value {
                field: string;
            }

            const value: Value = { field: "value" };
            export const result = value.field;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    let mut expression_input = None;
    for (range, expected) in &inferred.expressions {
        let input = ExpressionTypeInput::new(&db, module, *range);
        assert_eq!(infer_expression_type(&db, input), Some(*expected));
        expression_input.get_or_insert(input);
    }

    let mut binding_input = None;
    for (range, expected) in &inferred.binding_type_data {
        let input = BindingTypeInput::new(&db, module, *range);
        assert_eq!(infer_binding_type(&db, input), Some(expected.ty));
        binding_input.get_or_insert(input);
    }

    let mut local_input = None;
    for (index, expected) in inferred.types.iter().enumerate() {
        let input = LocalTypeInput::new(&db, module, InferredLocalTypeId::new(index));
        assert_eq!(infer_local_type(&db, input), Some(*expected));
        local_input.get_or_insert(input);
    }

    let expression_input = expression_input.expect("module must contain an expression type");
    let binding_input = binding_input.expect("module must contain a binding type");
    let local_input = local_input.expect("module must contain a local type");
    db.clear_salsa_events();
    let _ = infer_expression_type(&db, expression_input);
    let _ = infer_binding_type(&db, binding_input);
    let _ = infer_local_type(&db, local_input);
    let events = db.take_salsa_events();

    assert_function_query_was_not_run(&db, infer_expression_type, expression_input, &events);
    assert_function_query_was_not_run(&db, infer_binding_type, binding_input, &events);
    assert_function_query_was_not_run(&db, infer_local_type, local_input, &events);
}

#[test]
fn test_binding_query_is_invalidated_when_its_module_changes() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), "export const value = 1;");

    let mut db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let range = binding_range_by_name(&db, module, "value");
    {
        let input = BindingTypeInput::new(&db, module, range);
        let ty = infer_binding_type(&db, input).expect("value type must be inferred");
        assert!(is_inferred_number(&db, ty));
    }

    fs.insert("/src/index.ts".into(), "export const value = 'changed';");
    let module_kind = resolve_js_module_kind_for_test(&fs, "/src/index.ts", true);
    salsa::Setter::to(module.set_kind(&mut db), module_kind);

    db.clear_salsa_events();
    let input = BindingTypeInput::new(&db, module, range);
    let ty = infer_binding_type(&db, input).expect("changed value type must be inferred");
    assert!(is_inferred_string(&db, ty));
    let events = db.take_salsa_events();
    assert_function_query_was_run(&db, infer_binding_type, input, &events);
}

#[test]
fn test_binding_query_is_invalidated_when_an_import_changes() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/source.ts".into(), "export const value = 1;");
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import { value } from "./source.ts";
            export const result = value;
        "#,
    );

    let mut db = build_js_test_module_db(&fs, &["/src/source.ts", "/src/index.ts"], true);
    let source_module = db
        .module_for_path(Utf8Path::new("/src/source.ts"))
        .expect("source module must exist");
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("index module must exist");
    let range = binding_range_by_name(&db, index_module, "result");
    {
        let input = BindingTypeInput::new(&db, index_module, range);
        let ty = infer_binding_type(&db, input).expect("result type must be inferred");
        assert!(is_inferred_number(&db, ty));
    }

    fs.insert("/src/source.ts".into(), "export const value = 'changed';");
    let module_kind = resolve_js_module_kind_for_test(&fs, "/src/source.ts", true);
    salsa::Setter::to(source_module.set_kind(&mut db), module_kind);

    db.clear_salsa_events();
    let input = BindingTypeInput::new(&db, index_module, range);
    let ty = infer_binding_type(&db, input).expect("changed result type must be inferred");
    assert!(is_inferred_string(&db, ty));
    let events = db.take_salsa_events();
    assert_function_query_was_run(&db, infer_binding_type, input, &events);
    assert_function_query_was_not_run(&db, infer_module_types, index_module, &events);
    assert_function_query_was_not_run(&db, infer_module_types, source_module, &events);
}

#[test]
fn test_binding_query_handles_long_import_chains() {
    const MODULE_COUNT: usize = 2048;

    let fs = MemoryFileSystem::default();
    let paths = (0..MODULE_COUNT)
        .map(|index| format!("/src/module{index}.ts"))
        .collect::<Vec<_>>();
    fs.insert(paths[0].clone().into(), "export const value = 1;");
    for (index, path) in paths.iter().enumerate().skip(1) {
        fs.insert(
            path.clone().into(),
            format!(
                "import {{ value as previous }} from './module{}.ts'; export const value = previous;",
                index - 1
            ),
        );
    }
    fs.insert("/src/unrelated.ts".into(), "export const unrelated = 1;");
    let mut path_refs = paths.iter().map(String::as_str).collect::<Vec<_>>();
    path_refs.push("/src/unrelated.ts");

    let mut db = build_js_test_module_db(&fs, &path_refs, true);
    let module = db
        .module_for_path(Utf8Path::new(&paths[MODULE_COUNT - 1]))
        .expect("last module must exist");
    let range = binding_range_by_name(&db, module, "value");

    {
        let input = BindingTypeInput::new(&db, module, range);
        db.clear_salsa_events();
        let ty = infer_binding_type(&db, input).expect("value type must be inferred");
        assert!(is_inferred_number(&db, ty));
        let events = db.take_salsa_events();
        assert_eq!(
            function_query_will_execute_count_by_name(&db, IMPORT_DEPTH_PREPARATION_QUERY, &events,),
            1
        );

        db.clear_salsa_events();
        let ty = infer_binding_type(&db, input).expect("cached value type must be inferred");
        assert!(is_inferred_number(&db, ty));
        let events = db.take_salsa_events();
        assert_function_query_was_not_run(&db, infer_binding_type, input, &events);
    }

    let unrelated = db
        .module_for_path(Utf8Path::new("/src/unrelated.ts"))
        .expect("unrelated module must exist");
    fs.insert(
        "/src/unrelated.ts".into(),
        "export const unrelated = 'changed';",
    );
    let unrelated_kind = resolve_js_module_kind_for_test(&fs, "/src/unrelated.ts", true);
    salsa::Setter::to(unrelated.set_kind(&mut db), unrelated_kind);
    db.clear_salsa_events();
    let input = BindingTypeInput::new(&db, module, range);
    let ty = infer_binding_type(&db, input).expect("value type must remain inferred");
    assert!(is_inferred_number(&db, ty));
    let events = db.take_salsa_events();
    assert_function_query_was_not_run(&db, infer_binding_type, input, &events);
    assert_eq!(
        function_query_will_execute_count_by_name(&db, IMPORT_DEPTH_PREPARATION_QUERY, &events,),
        0
    );

    let terminal = db
        .module_for_path(Utf8Path::new(&paths[0]))
        .expect("terminal module must exist");
    fs.insert(paths[0].clone().into(), "export const value = 'changed';");
    let terminal_kind = resolve_js_module_kind_for_test(&fs, &paths[0], true);
    salsa::Setter::to(terminal.set_kind(&mut db), terminal_kind);
    db.clear_salsa_events();
    let input = BindingTypeInput::new(&db, module, range);
    let ty = infer_binding_type(&db, input).expect("changed value type must be inferred");
    assert!(is_inferred_string(&db, ty));
    let events = db.take_salsa_events();
    assert_function_query_was_run(&db, infer_binding_type, input, &events);
    assert_eq!(
        function_query_will_execute_count_by_name(&db, IMPORT_DEPTH_PREPARATION_QUERY, &events,),
        1
    );
}

#[test]
fn test_member_lookup_exhaustion_returns_unknown() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), "");
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let mut ty = InferredTypeData::Object(InferredObject::new(&db, None, Box::default(), false));

    for _ in 0..1025 {
        ty = InferredTypeData::Object(InferredObject::new(&db, Some(ty), Box::default(), false));
    }

    assert_eq!(
        find_value_member_type(&db, ty, "missing"),
        Some(InferredTypeData::Unknown)
    );
}
