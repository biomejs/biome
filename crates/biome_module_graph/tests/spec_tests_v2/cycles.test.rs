use super::*;
use biome_module_graph::{BindingTypeInput, find_member_type, infer_binding_type};

fn unwrap_typeof_values<'db>(
    db: &'db dyn ModuleDb,
    mut ty: InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    for _ in 0..16 {
        let InferredTypeData::TypeofValue(value) = ty else {
            return ty;
        };
        ty = value.ty(db);
    }
    ty
}

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

#[test]
fn test_infer_module_types_preserves_acyclic_exports_next_to_reexport_cycles() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/a.ts".into(),
        r#"
            export { value } from "./b.ts";
            export { stable } from "./stable.ts";
        "#,
    );
    fs.insert(
        "/src/b.ts".into(),
        r#"
            export { value } from "./a.ts";
        "#,
    );
    fs.insert("/src/stable.ts".into(), "export const stable = 1;");
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import { stable, value } from "./a.ts";
            export const cyclic = value;
            export const acyclic = stable;
        "#,
    );

    let db = build_js_test_module_db(
        &fs,
        &["/src/a.ts", "/src/b.ts", "/src/stable.ts", "/src/index.ts"],
        true,
    );
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("index module must exist");

    let inferred = infer_module_types_bottom_up(&db, index_module).expect("types must be inferred");
    let cyclic_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "cyclic")
        .expect("cyclic binding type must be inferred");
    assert_eq!(cyclic_ty, InferredTypeData::Unknown);
    let acyclic_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "acyclic")
        .expect("acyclic binding type must be inferred");
    assert!(is_inferred_number(&db, acyclic_ty));
}

#[test]
fn test_infer_module_types_bounds_mutual_namespace_reexports() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/a.ts".into(),
        r#"
            export * as b from "./b.ts";
        "#,
    );
    fs.insert(
        "/src/b.ts".into(),
        r#"
            export * as a from "./a.ts";
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import { b } from "./a.ts";
            export const cyclic = b;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/a.ts", "/src/b.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("index module must exist");

    let inferred = infer_module_types_bottom_up(&db, index_module).expect("types must be inferred");
    let mut ty = inferred_binding_ty_by_name(&db, index_module, inferred, "cyclic")
        .expect("cyclic binding type must be inferred");
    let mut reached_unknown = false;
    for name in ["a", "b"].into_iter().cycle().take(128) {
        let Some(member_ty) = inferred.find_member_type(&db, ty, name) else {
            break;
        };
        if member_ty == InferredTypeData::Unknown {
            reached_unknown = true;
            break;
        }
        ty = member_ty;
    }
    assert!(reached_unknown, "cyclic namespaces must resolve to Unknown");
}

#[test]
fn test_binding_query_preserves_acyclic_data_next_to_an_import_cycle() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/a.ts".into(),
        r#"
            import { b } from "./b.ts";
            export const stable = 1;
            export const a = b;
        "#,
    );
    fs.insert(
        "/src/b.ts".into(),
        r#"
            import { a, stable } from "./a.ts";
            export const b = { a, stable };
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import { a, stable } from "./a.ts";
            export const result = { a, stable };
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/a.ts", "/src/b.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("index module must exist");
    let input = BindingTypeInput::new(
        &db,
        index_module,
        binding_range_by_name(&db, index_module, "result"),
    );

    let result = infer_binding_type(&db, input).expect("result type must be inferred");
    let cyclic = find_member_type(&db, result, "a").expect("cyclic member must be present");
    let cyclic = unwrap_typeof_values(&db, cyclic);
    assert_eq!(cyclic, InferredTypeData::Unknown);

    let stable = find_member_type(&db, result, "stable").expect("stable member must be inferred");
    let stable = unwrap_typeof_values(&db, stable);
    assert!(
        is_inferred_number(&db, stable),
        "stable member must be numeric, got {stable:?}"
    );
}
