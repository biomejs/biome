use super::*;

#[test]
fn test_infer_module_types_merges_class_instance_intersections_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function readPromiseObject(
                value: Promise<string> & { value: number },
            ): Promise<string> & { value: number } {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let promise_object_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readPromiseObject")
            .expect("readPromiseObject return type must be inferred");
    let promise_object_ty = normalize_type(&db, index_module, promise_object_ty);
    let InferredTypeData::Intersection(intersection) = promise_object_ty else {
        panic!("readPromiseObject must preserve the intersection, got {promise_object_ty:?}");
    };
    assert!(
        intersection
            .types(&db)
            .iter()
            .any(|ty| is_inferred_promise_instance(&db, *ty))
    );

    let value_ty = inferred
        .find_member_type(&db, promise_object_ty, "value")
        .expect("merged class instance must expose value");
    assert!(is_inferred_number(&db, value_ty));

    assert_inferred_type_snapshot(
        "test_infer_module_types_merges_class_instance_intersections_on_build",
        &db,
        &fs,
    );
}
