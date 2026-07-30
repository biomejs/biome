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
