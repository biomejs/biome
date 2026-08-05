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

fn insert_import_chain(fs: &MemoryFileSystem, import_count: usize) -> Vec<String> {
    let paths = (0..=import_count)
        .map(|index| format!("/src/chain{index}.ts"))
        .collect::<Vec<_>>();

    for (index, path) in paths.iter().enumerate() {
        let source = if index == import_count {
            "export const value = 1;".to_string()
        } else {
            format!(
                "import {{ value as next }} from \"./chain{}.ts\"; export const value = next;",
                index + 1
            )
        };
        fs.insert(path.into(), source);
    }

    paths
}

#[test]
fn test_import_chains_preserve_terminal_export_across_on_demand_depth_boundary() {
    for import_count in [127, 128, 129] {
        let fs = MemoryFileSystem::default();
        let paths = insert_import_chain(&fs, import_count);
        let path_refs = paths.iter().map(String::as_str).collect::<Vec<_>>();
        let db = build_js_test_module_db(&fs, &path_refs, true);
        let root = db
            .module_for_path(Utf8Path::new(&paths[0]))
            .expect("root module must exist");

        let inferred = infer_module_types(&db, root).expect("types must be inferred");
        let value = inferred_binding_ty_by_name(&db, root, inferred, "value")
            .expect("terminal export type must be inferred");
        assert!(
            is_inferred_number(&db, unwrap_typeof_values(&db, value)),
            "terminal export must remain numeric across {import_count} imports, got {value:?}"
        );
    }
}

#[test]
fn test_deep_import_cycle_preserves_neighboring_acyclic_export() {
    const CYCLE_LENGTH: usize = 1024;

    let fs = MemoryFileSystem::default();
    let mut paths = (0..CYCLE_LENGTH)
        .map(|index| format!("/src/cycle{index}.ts"))
        .collect::<Vec<_>>();
    for (index, path) in paths.iter().enumerate() {
        let next = (index + 1) % CYCLE_LENGTH;
        let (stable_import, stable_export) = if index == 0 {
            (
                "import { stable } from \"./stable.ts\";",
                "export const acyclic = stable;",
            )
        } else {
            ("", "")
        };
        fs.insert(
            path.into(),
            format!(
                "{stable_import} import {{ cyclic as next }} from \"./cycle{next}.ts\"; export const cyclic = next; {stable_export}"
            ),
        );
    }
    fs.insert("/src/stable.ts".into(), "export const stable = 1;");
    paths.push("/src/stable.ts".to_string());

    let path_refs = paths.iter().map(String::as_str).collect::<Vec<_>>();
    let db = build_js_test_module_db(&fs, &path_refs, true);
    let root = db
        .module_for_path(Utf8Path::new(&paths[0]))
        .expect("root module must exist");

    let inferred = infer_module_types(&db, root).expect("types must be inferred");
    let cyclic = inferred_binding_ty_by_name(&db, root, inferred, "cyclic")
        .expect("cyclic binding type must be inferred");
    assert_eq!(unwrap_typeof_values(&db, cyclic), InferredTypeData::Unknown);
    let acyclic = inferred_binding_ty_by_name(&db, root, inferred, "acyclic")
        .expect("acyclic binding type must be inferred");
    assert!(
        is_inferred_number(&db, unwrap_typeof_values(&db, acyclic)),
        "neighboring acyclic export must remain numeric, got {acyclic:?}"
    );
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
    let mut reached_unknown = ty == InferredTypeData::Unknown;
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

#[test]
fn test_binding_query_is_invalidated_when_import_cycle_is_broken() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/a.ts".into(),
        "import { b } from './b.ts'; export const value = b;",
    );
    fs.insert(
        "/src/b.ts".into(),
        "import { value } from './a.ts'; export const b = value;",
    );
    fs.insert(
        "/src/index.ts".into(),
        "import { value } from './a.ts'; export const result = value;",
    );
    let mut db = build_js_test_module_db(&fs, &["/src/a.ts", "/src/b.ts", "/src/index.ts"], true);
    let index = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("index module must exist");
    let range = binding_range_by_name(&db, index, "result");
    {
        let input = BindingTypeInput::new(&db, index, range);
        assert_eq!(
            infer_binding_type(&db, input),
            Some(InferredTypeData::Unknown)
        );
    }

    let b = db
        .module_for_path(Utf8Path::new("/src/b.ts"))
        .expect("b module must exist");
    fs.insert("/src/b.ts".into(), "export const b = 1;");
    let b_kind = resolve_js_module_kind_for_test(&fs, "/src/b.ts", true);
    salsa::Setter::to(b.set_kind(&mut db), b_kind);

    db.clear_salsa_events();
    let input = BindingTypeInput::new(&db, index, range);
    let ty = infer_binding_type(&db, input).expect("result type must be inferred");
    assert!(is_inferred_number(&db, unwrap_typeof_values(&db, ty)));
    let events = db.take_salsa_events();
    assert_function_query_was_run(&db, infer_binding_type, input, &events);
}
