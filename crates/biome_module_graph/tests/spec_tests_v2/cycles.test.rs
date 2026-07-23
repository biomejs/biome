use super::*;

#[test]
fn test_infer_module_types_bottom_up_handles_import_cycles() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/a.ts".into(),
        r#"
            import { b } from "./b.ts";
            import { value } from "./value.ts";
            export const a = b;
            export const acyclic = value;
        "#,
    );
    fs.insert(
        "/src/b.ts".into(),
        r#"
            import { a } from "./a.ts";
            export const b = a;
        "#,
    );
    fs.insert("/src/value.ts".into(), "export const value = 1;");

    let db = build_js_test_module_db(&fs, &["/src/a.ts", "/src/b.ts", "/src/value.ts"], true);
    let a_module = db
        .module_for_path(Utf8Path::new("/src/a.ts"))
        .expect("a module must exist");
    let b_module = db
        .module_for_path(Utf8Path::new("/src/b.ts"))
        .expect("b module must exist");

    let a_types = infer_module_types_bottom_up(&db, a_module).expect("types must be inferred");
    let a_ty = inferred_binding_ty_by_name(&db, a_module, a_types, "a")
        .expect("a binding type must be inferred");
    assert_eq!(a_ty, InferredTypeData::Unknown);
    let acyclic_ty = inferred_binding_ty_by_name(&db, a_module, a_types, "acyclic")
        .expect("acyclic binding type must be inferred");
    assert!(is_inferred_number(&db, acyclic_ty));

    let b_types = infer_module_types_bottom_up(&db, b_module).expect("types must be inferred");
    let b_ty = inferred_binding_ty_by_name(&db, b_module, b_types, "b")
        .expect("b binding type must be inferred");
    assert_eq!(b_ty, InferredTypeData::Unknown);
}
