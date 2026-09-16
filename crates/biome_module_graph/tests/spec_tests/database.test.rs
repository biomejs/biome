use super::*;

#[test]
fn test_module_keys_reject_stale_handles() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), "export const value = 1;");

    let mut db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let path = Utf8PathBuf::from("/src/index.ts");
    let original = db.module_for_path(&path).expect("module must exist");
    let replacement = ModuleInfo::new(&db, path.clone(), original.kind(&db).clone());
    db.modules.insert(path, replacement);

    assert!(
        module_for_key(&db, InferredModuleKey::new(original.as_id())).is_none(),
        "stale module handles must be rejected"
    );
    assert_eq!(
        module_for_key(&db, InferredModuleKey::new(replacement.as_id())),
        Some(replacement)
    );
}
#[test]
fn test_infer_module_types_is_memoized() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export const value: string = "value";
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    let _ = infer_module_types(&db, index_module);
    db.clear_salsa_events();
    let _ = infer_module_types(&db, index_module);
    let events = db.take_salsa_events();

    assert_function_query_was_not_run(&db, infer_module_types, index_module, &events);
}

#[test]
fn test_infer_module_types_backdates_equal_output() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export const value: string = "value";
        "#,
    );

    let mut db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let expression_count = inferred_expression_count(&db, index_module);
    assert!(expression_count > 0);

    fs.insert(
        "/src/index.ts".into(),
        r#"
            export const value: string = "value";
            // This changes the module input, but not the inferred types.
        "#,
    );
    let module_kind = resolve_js_module_kind_for_test(&fs, "/src/index.ts", true);
    salsa::Setter::to(index_module.set_kind(&mut db), module_kind);

    db.clear_salsa_events();
    assert_eq!(
        inferred_expression_count(&db, index_module),
        expression_count
    );
    let events = db.take_salsa_events();

    assert_function_query_was_run(&db, infer_module_types, index_module, &events);
    assert_function_query_was_not_run(&db, inferred_expression_count, index_module, &events);
}
