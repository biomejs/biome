use super::*;
use biome_module_graph::{
    BindingTypeInput, ExpressionTypeInput, LocalTypeInput, find_member_type, infer_binding_type,
    infer_expression_type, infer_local_type,
};

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

fn local_type_id_by_name(db: &dyn ModuleDb, module: ModuleInfo, name: &str) -> InferredLocalTypeId {
    let ModuleInfoKind::Js(js_info) = module.kind(db) else {
        panic!("module must contain JavaScript information");
    };
    (0..js_info.raw_types.len())
        .map(InferredLocalTypeId::new)
        .find(|type_id| {
            js_info
                .local_type_name(*type_id)
                .is_some_and(|type_name| type_name.text() == name)
        })
        .unwrap_or_else(|| panic!("{name} local type must exist"))
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
            import { b, wrapper } from "./b.ts";
            export const stable = 1;
            export const a = b;
            export const outer = wrapper;
        "#,
    );
    fs.insert(
        "/src/b.ts".into(),
        r#"
            import { a, stable } from "./a.ts";
            export const b = { a, stable };
            export const wrapper = { cyclic: a, stable };
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import { a, outer, stable } from "./a.ts";
            import { wrapper } from "./b.ts";
            export const result = { a, outer, stable, wrapper };
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

    let a_module = db
        .module_for_path(Utf8Path::new("/src/a.ts"))
        .expect("a module must exist");
    let cyclic_input =
        BindingTypeInput::new(&db, a_module, binding_range_by_name(&db, a_module, "a"));
    let cyclic = infer_binding_type(&db, cyclic_input).expect("cyclic type must be inferred");
    assert_eq!(unwrap_typeof_values(&db, cyclic), InferredTypeData::Unknown);

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

    let wrapper = find_member_type(&db, result, "wrapper").expect("wrapper must be inferred");
    let wrapper = unwrap_typeof_values(&db, wrapper);
    let wrapper_cyclic =
        find_member_type(&db, wrapper, "cyclic").expect("cyclic member must be present");
    let wrapper_cyclic = unwrap_typeof_values(&db, wrapper_cyclic);
    assert_eq!(wrapper_cyclic, InferredTypeData::Unknown);
    let wrapper_stable =
        find_member_type(&db, wrapper, "stable").expect("stable member must be inferred");
    let wrapper_stable = unwrap_typeof_values(&db, wrapper_stable);
    assert!(
        is_inferred_number(&db, wrapper_stable),
        "wrapper stable member must be numeric, got {wrapper_stable:?}"
    );

    let outer = find_member_type(&db, result, "outer").expect("outer must be inferred");
    let outer = unwrap_typeof_values(&db, outer);
    let outer_stable =
        find_member_type(&db, outer, "stable").expect("outer stable member must be inferred");
    let outer_stable = unwrap_typeof_values(&db, outer_stable);
    assert!(
        is_inferred_number(&db, outer_stable),
        "outer stable member must be numeric, got {outer_stable:?}"
    );

    let db = build_js_test_module_db(&fs, &["/src/a.ts", "/src/b.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("index module must exist");
    let result_input = BindingTypeInput::new(
        &db,
        index_module,
        binding_range_by_name(&db, index_module, "result"),
    );
    let result = infer_binding_type(&db, result_input).expect("result type must be inferred");
    let stable = find_member_type(&db, result, "stable").expect("stable member must be present");
    assert!(is_inferred_number(&db, unwrap_typeof_values(&db, stable)));

    let a_module = db
        .module_for_path(Utf8Path::new("/src/a.ts"))
        .expect("a module must exist");
    let cyclic_input =
        BindingTypeInput::new(&db, a_module, binding_range_by_name(&db, a_module, "a"));
    let cyclic = infer_binding_type(&db, cyclic_input).expect("cyclic type must be inferred");
    assert_eq!(unwrap_typeof_values(&db, cyclic), InferredTypeData::Unknown);
}

#[test]
fn test_binding_query_ignores_unselected_namespace_exports_in_an_import_cycle() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/a.ts".into(),
        r#"
            import * as b from "./b.ts";
            export const value = b.stable;
        "#,
    );
    fs.insert(
        "/src/b.ts".into(),
        r#"
            import { value } from "./a.ts";
            export const stable = 1;
            export const unrelatedCycle = value;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/a.ts", "/src/b.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/a.ts"))
        .expect("a module must exist");
    let input = BindingTypeInput::new(&db, module, binding_range_by_name(&db, module, "value"));

    let ty = infer_binding_type(&db, input).expect("value must be inferred");
    let ty = unwrap_typeof_values(&db, ty);
    assert!(
        is_inferred_number(&db, ty),
        "selected namespace export must be numeric, got {ty:?}"
    );
}

#[test]
fn test_binding_query_projects_a_namespace_member_through_a_local_alias() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/a.ts".into(),
        r#"
            import * as b from "./b.ts";
            const alias = b;
            export const value = alias.stable;
        "#,
    );
    fs.insert(
        "/src/b.ts".into(),
        r#"
            import { value } from "./a.ts";
            export const stable = 1;
            export const unrelatedCycle = value;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/a.ts", "/src/b.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/a.ts"))
        .expect("a module must exist");
    let input = BindingTypeInput::new(&db, module, binding_range_by_name(&db, module, "value"));

    let ty = infer_binding_type(&db, input).expect("value must be inferred");
    let ty = unwrap_typeof_values(&db, ty);
    assert!(
        is_inferred_number(&db, ty),
        "selected namespace export must be numeric through an alias, got {ty:?}"
    );
}

#[test]
fn test_binding_query_bounds_namespace_projection_scope_traversal() {
    std::thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(|| {
            const SCOPE_DEPTH: usize = 1025;

            let fs = MemoryFileSystem::default();
            let mut source = String::from(
                r#"
                    import * as values from "./b.ts";
                    export const selected = (() => {
                "#,
            );
            source.push_str(&"{".repeat(SCOPE_DEPTH));
            source.push_str("return values.stable;");
            source.push_str(&"}".repeat(SCOPE_DEPTH));
            source.push_str("})();");
            fs.insert("/src/a.ts".into(), source);
            fs.insert(
                "/src/b.ts".into(),
                r#"
                    import { selected } from "./a.ts";
                    export const stable = 1;
                    export const root = selected;
                "#,
            );

            let db = build_js_test_module_db(&fs, &["/src/a.ts", "/src/b.ts"], true);
            let module = db
                .module_for_path(Utf8Path::new("/src/b.ts"))
                .expect("b module must exist");
            let input =
                BindingTypeInput::new(&db, module, binding_range_by_name(&db, module, "root"));

            let ty = infer_binding_type(&db, input).expect("root must be inferred");
            assert_eq!(unwrap_typeof_values(&db, ty), InferredTypeData::Unknown);
        })
        .expect("scope traversal test thread must spawn")
        .join()
        .expect("scope traversal test thread must complete");
}

#[test]
fn test_binding_query_resolves_a_selected_namespace_reexport_member_in_an_import_cycle() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/a.ts".into(),
        r#"
            export * as values from "./b.ts";
        "#,
    );
    fs.insert(
        "/src/b.ts".into(),
        r#"
            import { values } from "./a.ts";
            export const stable = 1;
            export const selected = values.stable;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/a.ts", "/src/b.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/b.ts"))
        .expect("b module must exist");
    let input = BindingTypeInput::new(&db, module, binding_range_by_name(&db, module, "selected"));

    let ty = infer_binding_type(&db, input).expect("selected must be inferred");
    let ty = unwrap_typeof_values(&db, ty);
    assert!(
        is_inferred_number(&db, ty),
        "selected namespace member must be numeric, got {ty:?}"
    );
}

#[test]
fn test_binding_query_projects_a_member_through_an_exported_namespace_alias() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/a.ts".into(),
        r#"
            import * as values from "./b.ts";
            export { values };
        "#,
    );
    fs.insert(
        "/src/b.ts".into(),
        r#"
            import { values } from "./a.ts";
            export const stable = 1;
            export const selected = values.stable;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/a.ts", "/src/b.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/b.ts"))
        .expect("b module must exist");
    let input = BindingTypeInput::new(&db, module, binding_range_by_name(&db, module, "selected"));

    let ty = infer_binding_type(&db, input).expect("selected must be inferred");
    let ty = unwrap_typeof_values(&db, ty);
    assert!(
        is_inferred_number(&db, ty),
        "selected exported namespace member must be numeric, got {ty:?}"
    );
}

#[test]
fn test_binding_query_projects_a_destructured_namespace_member() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/a.ts".into(),
        r#"
            import * as values from "./b.ts";
            export const { stable } = values;
        "#,
    );
    fs.insert(
        "/src/b.ts".into(),
        r#"
            import { stable as selected } from "./a.ts";
            export const stable = 1;
            export const unrelatedCycle = selected;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/a.ts", "/src/b.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/a.ts"))
        .expect("a module must exist");
    let input = BindingTypeInput::new(&db, module, binding_range_by_name(&db, module, "stable"));

    let ty = infer_binding_type(&db, input).expect("stable must be inferred");
    let ty = unwrap_typeof_values(&db, ty);
    assert!(
        is_inferred_number(&db, ty),
        "destructured namespace member must be numeric, got {ty:?}"
    );
}

#[test]
fn test_binding_query_bounds_an_actual_namespace_declaration_cycle() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/a.ts".into(),
        r#"
            export * as values from "./b.ts";
        "#,
    );
    fs.insert(
        "/src/b.ts".into(),
        r#"
            import { values } from "./a.ts";
            export const recursive = values;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/a.ts", "/src/b.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/b.ts"))
        .expect("b module must exist");
    let input = BindingTypeInput::new(&db, module, binding_range_by_name(&db, module, "recursive"));

    let ty = infer_binding_type(&db, input).expect("recursive must be inferred");
    assert_eq!(unwrap_typeof_values(&db, ty), InferredTypeData::Unknown);
}

#[test]
fn test_local_type_query_preserves_an_acyclic_type_in_an_import_cycle() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/a.ts".into(),
        r#"
            import type { Back } from "./b.ts";
            export interface Stable { value: number }
            export type Link = Back;
        "#,
    );
    fs.insert(
        "/src/b.ts".into(),
        r#"
            import type { Stable } from "./a.ts";
            export type Back = Stable;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/a.ts", "/src/b.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/a.ts"))
        .expect("a module must exist");
    let input = LocalTypeInput::new(&db, module, local_type_id_by_name(&db, module, "Stable"));

    let ty = infer_local_type(&db, input).expect("Stable must be inferred");
    let value = find_member_type(&db, ty, "value").expect("value member must be inferred");
    assert!(
        is_inferred_number(&db, value),
        "value member must be numeric, got {value:?}"
    );
}

#[test]
fn test_binding_query_marks_a_structural_root_declaration_cycle_unknown() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/a.ts".into(),
        r#"
            import { b } from "./b.ts";
            export const a = { b, stable: 1 };
        "#,
    );
    fs.insert(
        "/src/b.ts".into(),
        r#"
            import { a } from "./a.ts";
            export const b = c;
            export const c = { a, stable: 2 };
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/a.ts", "/src/b.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/a.ts"))
        .expect("a module must exist");
    let input = BindingTypeInput::new(&db, module, binding_range_by_name(&db, module, "a"));

    let ty = infer_binding_type(&db, input).expect("a type must be inferred");
    assert_eq!(unwrap_typeof_values(&db, ty), InferredTypeData::Unknown);

    let module = db
        .module_for_path(Utf8Path::new("/src/b.ts"))
        .expect("b module must exist");
    let input = BindingTypeInput::new(&db, module, binding_range_by_name(&db, module, "c"));
    let ty = infer_binding_type(&db, input).expect("c type must be inferred");
    assert_eq!(unwrap_typeof_values(&db, ty), InferredTypeData::Unknown);
}

#[test]
fn test_binding_query_marks_a_structural_self_import_cycle_unknown() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/self.ts".into(),
        r#"
            import { b as importedB } from "./self.ts";
            export const a = { b: importedB, stable: 1 };
            export const b = a;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/self.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/self.ts"))
        .expect("self module must exist");
    let input = BindingTypeInput::new(&db, module, binding_range_by_name(&db, module, "a"));

    let ty = infer_binding_type(&db, input).expect("a type must be inferred");
    assert_eq!(unwrap_typeof_values(&db, ty), InferredTypeData::Unknown);
}

#[test]
fn test_binding_query_evaluates_a_deep_declaration_chain_in_an_import_cycle() {
    const MODULE_COUNT: usize = 132;

    let fs = MemoryFileSystem::default();
    let paths = (0..MODULE_COUNT)
        .map(|index| format!("/src/branch{index}.ts"))
        .collect::<Vec<_>>();

    for (index, path) in paths.iter().enumerate() {
        if index == 0 {
            fs.insert(
                path.into(),
                format!(
                    "import {{ value{} as unused }} from \"./branch{}.ts\"; export const value0 = {{ leaf: 1 }}; export {{ unused }};",
                    MODULE_COUNT - 1,
                    MODULE_COUNT - 1
                ),
            );
            continue;
        }

        let dependencies = [index - 1];
        let imports = dependencies
            .iter()
            .map(|dependency| {
                format!("import {{ value{dependency} }} from \"./branch{dependency}.ts\";")
            })
            .collect::<String>();
        let members = dependencies
            .iter()
            .map(|dependency| format!("value{dependency}"))
            .collect::<Vec<_>>()
            .join(", ");
        fs.insert(
            path.into(),
            format!("{imports} export const value{index} = {{ {members} }};"),
        );
    }

    let path_refs = paths.iter().map(String::as_str).collect::<Vec<_>>();
    let db = build_js_test_module_db(&fs, &path_refs, true);
    let root = db
        .module_for_path(Utf8Path::new(
            paths.last().expect("a module path must exist"),
        ))
        .expect("root module must exist");
    let input = BindingTypeInput::new(
        &db,
        root,
        binding_range_by_name(&db, root, &format!("value{}", MODULE_COUNT - 1)),
    );

    db.clear_salsa_events();
    let mut ty = infer_binding_type(&db, input).expect("type must be inferred");

    for index in (1..MODULE_COUNT).rev() {
        ty = find_member_type(&db, ty, &format!("value{}", index - 1))
            .expect("the preceding branch must be inferred");
        ty = unwrap_typeof_values(&db, ty);
    }
    let leaf = find_member_type(&db, ty, "leaf").expect("the leaf must be inferred");
    let leaf = unwrap_typeof_values(&db, leaf);
    assert!(
        is_inferred_number(&db, leaf),
        "leaf must be numeric, got {leaf:?}"
    );

    let events = db.take_salsa_events();
    let query_count = function_query_will_execute_count_by_name(
        &db,
        "infer_binding_type_with_import_budget",
        &events,
    );
    assert_eq!(query_count, 0);
}

#[test]
fn test_binding_query_does_not_cache_a_depth_limited_declaration() {
    const ALIAS_COUNT: usize = 80;
    const DIRECT_ALIAS: usize = 20;

    let fs = MemoryFileSystem::default();
    let mut source = String::from("const target = 1;\nconst value0 = target;\n");
    for index in 1..ALIAS_COUNT {
        source.push_str(&format!("const value{index} = value{};\n", index - 1));
    }
    source.push_str(&format!(
        "export const root = {{ deep: value{}, direct: value{DIRECT_ALIAS} }};",
        ALIAS_COUNT - 1,
    ));
    fs.insert("/src/a.ts".into(), source);

    let db = build_js_test_module_db(&fs, &["/src/a.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/a.ts"))
        .expect("a module must exist");
    let input = BindingTypeInput::new(&db, module, binding_range_by_name(&db, module, "root"));

    let ty = infer_binding_type(&db, input).expect("root must be inferred");
    let direct = find_member_type(&db, ty, "direct").expect("direct must be inferred");
    let direct = unwrap_typeof_values(&db, direct);
    assert!(
        is_inferred_number(&db, direct),
        "direct must be numeric, got {direct:?}"
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

fn expression_range_by_source(
    db: &dyn ModuleDb,
    module: ModuleInfo,
    source: &str,
    expected: &str,
) -> TextRange {
    let ModuleInfoKind::Js(js_info) = module.kind(db) else {
        panic!("module must contain JavaScript information");
    };
    js_info
        .raw_expressions
        .keys()
        .find(|range| source_snippet(source, **range) == expected)
        .copied()
        .unwrap_or_else(|| panic!("{expected} expression must exist"))
}

#[test]
fn test_expression_query_resolves_a_namespace_reexport_member_in_its_own_import_cycle() {
    const CONSUMER_SOURCE: &str = r#"
        import { values } from "./a.ts";
        export interface Marker {}
        values.stable;
    "#;

    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/a.ts".into(),
        r#"
            export * as values from "./b.ts";
        "#,
    );
    fs.insert(
        "/src/b.ts".into(),
        r#"
            import type { Marker } from "./consumer.ts";
            export const stable = 1;
            export type BMarker = Marker;
        "#,
    );
    fs.insert("/src/consumer.ts".into(), CONSUMER_SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/a.ts", "/src/b.ts", "/src/consumer.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/consumer.ts"))
        .expect("consumer module must exist");
    let input = ExpressionTypeInput::new(
        &db,
        module,
        expression_range_by_source(&db, module, CONSUMER_SOURCE, "values.stable"),
    );

    let ty = infer_expression_type(&db, input).expect("expression must be inferred");
    let ty = unwrap_typeof_values(&db, ty);
    assert!(
        is_inferred_number(&db, ty),
        "namespace member reached through the consumer's own import cycle must be numeric, got {ty:?}"
    );
}

#[test]
fn test_expression_query_resolves_a_namespace_reexport_member_behind_a_foreign_import_cycle() {
    const INDEX_SOURCE: &str = r#"
        import { values } from "./a.ts";
        values.stable;
    "#;

    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/a.ts".into(),
        r#"
            export * as values from "./b.ts";
        "#,
    );
    fs.insert(
        "/src/b.ts".into(),
        r#"
            import { values } from "./a.ts";
            export const stable = 1;
            export const selected = values.stable;
        "#,
    );
    fs.insert("/src/index.ts".into(), INDEX_SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/a.ts", "/src/b.ts", "/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("index module must exist");
    let input = ExpressionTypeInput::new(
        &db,
        module,
        expression_range_by_source(&db, module, INDEX_SOURCE, "values.stable"),
    );

    let ty = infer_expression_type(&db, input).expect("expression must be inferred");
    let ty = unwrap_typeof_values(&db, ty);
    assert!(
        is_inferred_number(&db, ty),
        "namespace member behind a foreign import cycle must be numeric, got {ty:?}"
    );
}
