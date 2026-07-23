use super::*;

#[test]
fn test_normalize_type_preserves_recursive_array_local_edge() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type Tree = number | Tree[];

            export function readTree(value: Tree): Tree {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let tree_ty = inferred_function_return_ty_by_name(&db, index_module, inferred, "readTree")
        .expect("readTree return type must be inferred");
    let tree_index = local_type_id_of_instance(&db, tree_ty)
        .expect("readTree must return an instance of the local Tree type");
    let normalized_ty = normalize_type(&db, index_module, tree_ty);

    let InferredTypeData::Union(union) = normalized_ty else {
        panic!("recursive Tree type must normalize to a union, got {normalized_ty:?}");
    };
    let normalized_tree = format_inferred_type(&db, normalized_ty);
    assert!(
        union
            .types(&db)
            .iter()
            .any(|ty| is_inferred_number(&db, *ty)),
        "recursive Tree union must keep its number branch"
    );
    assert!(
        union.types(&db).iter().any(|ty| {
            matches!(
                ty,
                InferredTypeData::InstanceOf(instance)
                    if instance.ty(&db).is_array_class(&db)
                    && instance.type_parameters(&db).iter().any(|parameter| {
                        matches!(
                            parameter,
                            InferredTypeData::Local(local)
                                if local.type_id(&db).index() == tree_index
                        )
                        || local_type_id_of_instance(&db, *parameter) == Some(tree_index)
                    })
            )
        }),
        "recursive Tree union must keep the recursive Array local edge: {normalized_tree}"
    );

    assert_inferred_type_snapshot(
        "test_normalize_type_preserves_recursive_array_local_edge",
        &db,
        &fs,
    );
}
