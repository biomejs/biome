use super::*;

#[test]
fn test_infer_module_types_selects_call_overloads_by_required_object_members_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            interface Options { initial?: string }
            interface DefinedOptions extends Options { initial: string }

            type DefinedResult = { isPending: false };
            type MaybeResult = { isPending: true } | { isPending: false };

            declare function query(options: DefinedOptions): DefinedResult;
            declare function query(options: Options): MaybeResult;

            declare function select(options: { kind: "text" }): string;
            declare function select(options: { kind: "number" }): number;

            export const { isPending } = query({});
            export const selected = select({ kind: "number" });
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let is_pending_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "isPending")
        .expect("isPending binding type must be inferred");

    assert!(is_inferred_boolean(
        &db,
        inferred.resolve_type(&db, is_pending_ty)
    ));
    let selected_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "selected")
        .expect("selected binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, selected_ty)
    ));
    assert_inferred_type_snapshot(
        "test_infer_module_types_selects_call_overloads_by_required_object_members_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_keeps_overloads_viable_for_partially_modelled_arguments_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            interface Options { initial?: string }
            interface DefinedOptions extends Options { initial: string }

            declare function query(options: DefinedOptions): number;
            declare function query(options: Options): string;

            declare const defaults: DefinedOptions;
            declare function tag<T>(value: T): T & { tagged: true };

            export const fromSpread = query({ ...defaults });
            export const fromEmpty = query({});
            export const fromIntersection = query(tag({ ...defaults }));
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    // Inference cannot expand the spread into members, so `initial` may still
    // be present and the first overload stays viable.
    let from_spread_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "fromSpread")
        .expect("fromSpread binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, from_spread_ty)
    ));

    // An object literal without a spread lists every member it has, so the
    // first overload is rejected.
    let from_empty_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "fromEmpty")
        .expect("fromEmpty binding type must be inferred");
    assert!(is_inferred_string(
        &db,
        inferred.resolve_type(&db, from_empty_ty)
    ));

    // Intersecting the spread carries the missing members into the result, so
    // the first overload stays viable there too.
    let from_intersection_ty =
        inferred_binding_ty_by_name(&db, index_module, inferred, "fromIntersection")
            .expect("fromIntersection binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, from_intersection_ty)
    ));

    assert_inferred_type_snapshot(
        "test_infer_module_types_keeps_overloads_viable_for_partially_modelled_arguments_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_selects_call_overloads_by_interface_call_signatures_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            interface SyncHandler { (): string }
            interface AsyncHandler { (): Promise<string> }
            type VoidHandler = () => void;

            declare function run(handler: SyncHandler): number;
            declare function run(handler: AsyncHandler): string;

            declare function schedule(handler: VoidHandler): number;
            declare function schedule(handler: AsyncHandler): string;

            export const ran = run(async () => "value");
            export const scheduled = schedule(async () => "value");
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    // `SyncHandler` returns a value the async handler cannot produce, so the
    // first overload is rejected.
    let ran_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "ran")
        .expect("ran binding type must be inferred");
    assert!(is_inferred_string(&db, inferred.resolve_type(&db, ran_ty)));

    // `VoidHandler` discards the result, so an async handler satisfies it and
    // the first overload wins.
    let scheduled_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "scheduled")
        .expect("scheduled binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, scheduled_ty)
    ));

    assert_inferred_type_snapshot(
        "test_infer_module_types_selects_call_overloads_by_interface_call_signatures_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_substitutes_callback_returns_through_type_aliases_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type Mapper<T> = () => T;

            declare function mapAliased<T>(mapper: Mapper<T>): T;
            declare function mapInline<T>(mapper: () => T): T;

            export const aliased = mapAliased(() => 42);
            export const inline = mapInline(() => 42);
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    for name in ["aliased", "inline"] {
        let ty = inferred_binding_ty_by_name(&db, index_module, inferred, name)
            .unwrap_or_else(|| panic!("{name} binding type must be inferred"));
        let ty = inferred.resolve_type(&db, ty);
        assert!(
            is_inferred_number_literal(&db, ty, "42"),
            "{name} must be the literal 42, got {}",
            format_inferred_type(&db, ty)
        );
    }

    assert_inferred_type_snapshot(
        "test_infer_module_types_substitutes_callback_returns_through_type_aliases_on_build",
        &db,
        &fs,
    );
}
