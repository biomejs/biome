use super::*;
use biome_module_graph::{ResolutionMode, ResolutionRequest, resolve_module_request};
use biome_resolver::ResolveError;

fn request<'db>(
    db: &'db TestModuleDb,
    base_directory: &str,
    specifier: &str,
    mode: ResolutionMode,
) -> ResolutionRequest<'db> {
    let base_directory = db
        .resolver_paths
        .get_or_create(db, Utf8Path::new(base_directory));
    ResolutionRequest::new(db, base_directory, specifier.to_string(), mode)
}

#[test]
fn resolver_path_info_is_scoped_to_its_database() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/dependency.css".into(), "");
    let first = TestModuleDb::with_fs(&fs);
    let second = TestModuleDb::with_fs(&fs);

    let resolve = |db: &TestModuleDb| {
        resolve_module_request(db, request(db, "/src", "./dependency", ResolutionMode::Css))
            .path()
            .as_path()
            .map(Utf8Path::to_path_buf)
    };
    assert_eq!(resolve(&first), Some("/src/dependency.css".into()));

    fs.remove(Utf8Path::new("/src/dependency.css"));
    assert_eq!(
        resolve(&first),
        Some("/src/dependency.css".into()),
        "path info is only refreshed when the owner of the database reports a change"
    );
    assert_eq!(resolve(&second), None);
}

#[test]
fn resolution_requests_intern_equal_queries() {
    let db = TestModuleDb::new();
    let javascript = request(&db, "/src", "./dependency", ResolutionMode::JavaScript);
    let same_javascript = request(&db, "/src", "./dependency", ResolutionMode::JavaScript);
    let css = request(&db, "/src", "./dependency", ResolutionMode::Css);

    assert_eq!(javascript.as_id(), same_javascript.as_id());
    assert_ne!(javascript.as_id(), css.as_id());
}

#[test]
fn resolution_query_is_memoized_and_tracks_consumed_paths() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/dependency.css".into(), "");
    let mut db = TestModuleDb::with_fs(&fs);

    {
        let request = request(&db, "/src", "./dependency", ResolutionMode::Css);
        assert_eq!(
            resolve_module_request(&db, request).path().as_path(),
            Some(Utf8Path::new("/src/dependency.css"))
        );
        db.clear_salsa_events();

        let _ = resolve_module_request(&db, request);
        let events = db.take_salsa_events();
        assert_function_query_was_not_run(&db, resolve_module_request, request, &events);
    }

    // An unrelated path doesn't invalidate the resolution.
    fs.insert("/other/file.css".into(), "");
    let _ = db
        .resolver_paths
        .get_or_create(&db, Utf8Path::new("/other/file.css"));
    sync_resolver_paths(
        &mut db,
        [("/other/file.css".into(), ResolverPathChange::Kind.into())],
    );
    db.clear_salsa_events();
    {
        let request = request(&db, "/src", "./dependency", ResolutionMode::Css);
        let _ = resolve_module_request(&db, request);
        let events = db.take_salsa_events();
        assert_function_query_was_not_run(&db, resolve_module_request, request, &events);
    }

    fs.remove(Utf8Path::new("/src/dependency.css"));
    sync_resolver_paths(
        &mut db,
        [(
            "/src/dependency.css".into(),
            ResolverPathChange::Kind.into(),
        )],
    );
    db.clear_salsa_events();

    let request = request(&db, "/src", "./dependency", ResolutionMode::Css);
    assert_eq!(
        resolve_module_request(&db, request).path().error(),
        Some(&ResolveError::NotFound)
    );
    let events = db.take_salsa_events();
    assert_function_query_was_run(&db, resolve_module_request, request, &events);
}

#[test]
fn resolution_query_tracks_tsconfig_source_changes() {
    let fs = MemoryFileSystem::default();
    fs.insert("/project/src/first.ts".into(), "");
    fs.insert("/project/src/second.ts".into(), "");
    let mut db = TestModuleDb::with_fs(&fs);
    db.insert_json_source(
        Utf8PathBuf::from("/project/package.json"),
        r#"{"name":"project"}"#,
    );
    db.insert_json_source(
        Utf8PathBuf::from("/project/tsconfig.json"),
        r#"{"compilerOptions":{"paths":{"@dep":["./src/first.ts"]}}}"#,
    );

    {
        let request = request(&db, "/project/src", "@dep", ResolutionMode::JavaScript);
        assert_eq!(
            resolve_module_request(&db, request).path().as_path(),
            Some(Utf8Path::new("/project/src/first.ts"))
        );
    }

    let source = db.files[Utf8Path::new("/project/tsconfig.json")];
    salsa::Setter::to(
        source.set_parsed(&mut db),
        parse_json(
            r#"{"compilerOptions":{"paths":{"@dep":["./src/second.ts"]}}}"#,
            JsonParserOptions::default(),
        )
        .into(),
    );
    db.clear_salsa_events();

    let request = request(&db, "/project/src", "@dep", ResolutionMode::JavaScript);
    assert_eq!(
        resolve_module_request(&db, request).path().as_path(),
        Some(Utf8Path::new("/project/src/second.ts"))
    );
    let events = db.take_salsa_events();
    assert_function_query_was_run(&db, resolve_module_request, request, &events);
}

#[test]
fn resolution_query_observes_a_manifest_indexed_later() {
    let fs = MemoryFileSystem::default();
    fs.insert("/project/src/first.ts".into(), "");
    let mut db = TestModuleDb::with_fs(&fs);

    {
        let request = request(&db, "/project/src", "@dep", ResolutionMode::JavaScript);
        assert!(
            resolve_module_request(&db, request)
                .path()
                .as_path()
                .is_none()
        );
    }

    db.insert_json_source(
        Utf8PathBuf::from("/project/package.json"),
        r#"{"name":"project"}"#,
    );
    db.insert_json_source(
        Utf8PathBuf::from("/project/tsconfig.json"),
        r#"{"compilerOptions":{"paths":{"@dep":["./src/first.ts"]}}}"#,
    );

    let request = request(&db, "/project/src", "@dep", ResolutionMode::JavaScript);
    assert_eq!(
        resolve_module_request(&db, request).path().as_path(),
        Some(Utf8Path::new("/project/src/first.ts"))
    );
}

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
