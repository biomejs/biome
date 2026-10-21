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
fn test_infer_module_types_bottom_up_removes_undefined_from_imported_destructuring_defaults() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/source.ts".into(),
        r#"
            export const object: { b?: number; d?: string | null } = {};
            export const numbers: (number | undefined)[] = [];
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import { numbers, object } from "./source.ts";
            export const { b = 1, d = "value" } = object;
            export const { b: withoutDefault } = object;
            export const [head = 0] = numbers;
            export const [withoutDefaultHead] = numbers;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/source.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("index module must exist");
    let inferred = infer_module_types_bottom_up(&db, index_module).expect("types must be inferred");

    for name in ["b", "head"] {
        let ty = inferred_binding_ty_by_name(&db, index_module, inferred, name)
            .unwrap_or_else(|| panic!("{name} binding type must be inferred"));
        assert!(
            contains_inferred_number(&db, ty),
            "{name} must stay a number"
        );
        assert!(
            !contains_inferred_undefined(&db, ty),
            "{name} must not include undefined"
        );
    }

    let d_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "d")
        .expect("d binding type must be inferred");
    assert!(contains_inferred_string(&db, d_ty));
    assert!(contains_inferred_null(&db, d_ty));
    assert!(!contains_inferred_undefined(&db, d_ty));

    for name in ["withoutDefault", "withoutDefaultHead"] {
        let ty = inferred_binding_ty_by_name(&db, index_module, inferred, name)
            .unwrap_or_else(|| panic!("{name} binding type must be inferred"));
        assert!(
            contains_inferred_undefined(&db, ty),
            "{name} must keep undefined"
        );
    }
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

    fs.insert(
        "/package.json".into(),
        r#"{"name":"frontend","version":"0.0.0","dependencies":{"react":"19.0.0"}}"#,
    );
    fs.insert("/tsconfig.json".into(), "{}");

    let db = build_js_test_module_db(
        &fs,
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
