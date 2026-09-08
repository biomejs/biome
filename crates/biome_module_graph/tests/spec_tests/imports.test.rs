use super::*;

#[test]
fn test_infer_module_types_bottom_up_skips_side_effect_imports() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/side_effect.ts".into(), "export const unused = 1;");
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import "./side_effect.ts";
            export const value = 1;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/side_effect.ts", "/src/index.ts"], true);
    let side_effect_module = db
        .module_for_path(Utf8Path::new("/src/side_effect.ts"))
        .expect("side-effect module must exist");
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("index module must exist");

    db.clear_salsa_events();
    let inferred = infer_module_types_bottom_up(&db, index_module).expect("types must be inferred");
    let value_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "value")
        .expect("value binding type must be inferred");
    assert!(is_inferred_number(&db, value_ty));

    let events = db.take_salsa_events();
    assert_function_query_was_run(&db, infer_module_types, index_module, &events);
    assert_function_query_was_not_run(&db, infer_module_types, side_effect_module, &events);
}

#[test]
fn test_infer_module_types_bottom_up_prepares_named_imports() {
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

    db.clear_salsa_events();
    let inferred = infer_module_types_bottom_up(&db, index_module).expect("types must be inferred");
    let result_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "result")
        .expect("result binding type must be inferred");
    assert!(is_inferred_number(&db, result_ty));

    let events = db.take_salsa_events();
    assert_function_query_was_run(&db, infer_module_types, source_module, &events);
    assert_function_query_was_run(&db, infer_module_types, index_module, &events);
}

#[test]
fn test_infer_module_types_resolves_react_export_equals_namespace() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/node_modules/@types/react/index.d.ts".into(),
        include_bytes!("../../../biome_resolver/tests/fixtures/resolver_cases_5/node_modules/@types/react/index.d.ts")
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"import { useCallback } from "react";

        const fn = useCallback(async () => {});
        const promise = fn();
        "#,
    );

    let project_layout = ProjectLayout::default();
    project_layout.insert_node_manifest(
        "/".into(),
        PackageJson::new("frontend")
            .with_version("0.0.0")
            .with_dependencies(Dependencies(Box::new([("react".into(), "19.0.0".into())]))),
    );

    let tsconfig_json = parse_json(r#"{}"#, JsonParserOptions::default());
    project_layout
        .insert_serialized_tsconfig("/".into(), &tsconfig_json.syntax().as_send().unwrap());

    let db = build_js_test_module_db_with_layout(
        &fs,
        &project_layout,
        &["/node_modules/@types/react/index.d.ts", "/src/index.ts"],
        true,
    );
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let use_callback_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "useCallback")
        .expect("useCallback binding type must be inferred");
    let use_callback_ty = inferred.resolve_type(&db, use_callback_ty);
    assert!(use_callback_ty.callable_function(&db).is_some());

    let promise_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "promise")
        .expect("promise binding type must be inferred");
    let promise_ty = inferred.resolve_type(&db, promise_ty);
    assert!(promise_ty.is_promise_instance(&db));
}
