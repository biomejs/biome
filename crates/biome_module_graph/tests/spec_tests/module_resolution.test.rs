use biome_deserialize::json::deserialize_from_json_str;
use biome_fs::{BiomePath, FileSystem, MemoryFileSystem, OsFileSystem};
use biome_module_graph::{
    ImportSymbol, JsExport, JsExportedSymbolLookup, JsImport, JsOwnExport, JsReexport, ModuleDb,
    ResolvedPath, SymbolFromModuleInfo, find_js_exported_symbol,
};
use biome_package::{Dependencies, PackageJson};
use biome_project_layout::ProjectLayout;
use biome_resolver::ResolveError;
use biome_rowan::{Text, TextRange, TextSize};
use camino::{Utf8Path, Utf8PathBuf};

use super::support::build_js_db;
use super::*;

fn project() -> (MemoryFileSystem, ProjectLayout) {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        "import { foo } from 'shared'; import { bar } from './bar.ts';",
    );
    fs.insert("/src/bar.ts".into(), "export function bar() {}");
    fs.insert(
        "/node_modules/shared/index.d.ts".into(),
        "export function foo(): void;",
    );
    let layout = ProjectLayout::default();
    layout.insert_node_manifest(
        "/".into(),
        PackageJson::new("app")
            .with_dependencies(Dependencies(Box::new([("shared".into(), "1.0.0".into())]))),
    );
    let manifest = biome_deserialize::json::deserialize_from_json_str::<PackageJson>(
        r#"{"name":"shared","version":"1.0.0","types":"index.d.ts"}"#,
        Default::default(),
        "package.json",
    )
    .into_deserialized()
    .unwrap();
    layout.insert_node_manifest("/node_modules/shared".into(), manifest);
    (fs, layout)
}

#[test]
fn test_resolve_relative_import() {
    let (fs, layout) = project();
    let db = build_js_db(
        &fs,
        &layout,
        &[
            BiomePath::new("/src/index.ts"),
            BiomePath::new("/src/bar.ts"),
        ],
        true,
    );
    let info = db
        .js_module_info_for_path(Utf8Path::new("/src/index.ts"))
        .unwrap();
    assert_eq!(
        info.static_imports.get("bar"),
        Some(&JsImport {
            specifier: "./bar.ts".into(),
            resolved_path: ResolvedPath::from_path("/src/bar.ts"),
            symbol: "bar".into()
        })
    );
}

#[test]
fn test_resolve_package_import() {
    let (fs, layout) = project();
    let db = build_js_db(
        &fs,
        &layout,
        &[
            BiomePath::new("/src/index.ts"),
            BiomePath::new("/src/components/Hello.tsx"),
        ],
        true,
    );
    assert_eq!(
        db.js_module_info_for_path(Utf8Path::new("/src/index.ts"))
            .unwrap()
            .static_imports
            .get("foo")
            .unwrap()
            .resolved_path
            .as_path(),
        Some(Utf8Path::new("/node_modules/shared/index.d.ts"))
    );
}

#[test]
fn test_import_through_path_alias() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        "import { Hello } from '@components/Hello';",
    );
    fs.insert(
        "/src/components/Hello.tsx".into(),
        "export function Hello() {}",
    );
    let layout = ProjectLayout::default();
    layout.insert_node_manifest("/".into(), PackageJson::new("app").with_version("1.0.0"));
    let json = biome_json_parser::parse_json(
        r#"{"compilerOptions":{"paths":{"@components/*":["./src/components/*"]}}}"#,
        Default::default(),
    );
    layout.insert_serialized_tsconfig("/".into(), &json.syntax().as_send().unwrap());
    let db = build_js_db(&fs, &layout, &[BiomePath::new("/src/index.ts")], true);
    assert_eq!(
        db.js_module_info_for_path(Utf8Path::new("/src/index.ts"))
            .unwrap()
            .static_imports
            .get("Hello")
            .unwrap()
            .resolved_path
            .as_path(),
        Some(Utf8Path::new("/src/components/Hello.tsx"))
    );
}

#[test]
fn test_resolve_package_import_in_monorepo_fixtures() {
    let mut fixtures: Utf8PathBuf = std::env::current_dir().unwrap().try_into().unwrap();
    while !fixtures.join("Cargo.lock").exists() {
        fixtures = fixtures.parent().unwrap().to_path_buf();
    }
    fixtures.push("crates/biome_module_graph/tests/fixtures");
    let fs = OsFileSystem::new(fixtures.clone());
    let layout = ProjectLayout::default();
    for directory in ["frontend", "shared", "frontend/node_modules/shared"] {
        let root = fixtures.join(directory);
        let manifest = deserialize_from_json_str::<PackageJson>(
            &fs.read_file_from_path(&root.join("package.json")).unwrap(),
            Default::default(),
            "package.json",
        )
        .into_deserialized()
        .unwrap();
        layout.insert_node_manifest(root, manifest);
    }
    let frontend = fixtures.join("frontend/src/index.ts");
    let shared = fixtures.join("shared/dist/index.js");
    let db = build_js_db(
        &fs,
        &layout,
        &[
            BiomePath::new(fixtures.join("frontend/src/bar.ts")),
            BiomePath::new(frontend.clone()),
            BiomePath::new(fixtures.join("frontend/node_modules/shared/dist/index.js")),
            BiomePath::new(shared.clone()),
        ],
        true,
    );
    let info = db.js_module_info_for_path(&frontend).unwrap();
    assert_eq!(
        info.static_imports
            .get("sharedFoo")
            .unwrap()
            .resolved_path
            .as_path(),
        Some(shared.as_path())
    );
    assert_eq!(
        info.static_imports
            .get("bar")
            .unwrap()
            .resolved_path
            .as_path(),
        Some(fixtures.join("frontend/src/bar.ts").as_path())
    );
}

#[test]
fn test_node_builtin_imports_resolve_to_builtin_error() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), "import fs from 'node:fs'; import path from 'node:path'; import { fileURLToPath } from 'node:url';");
    let db = build_js_db(
        &fs,
        &ProjectLayout::default(),
        &[BiomePath::new("/src/index.ts")],
        false,
    );
    let info = db
        .js_module_info_for_path(Utf8Path::new("/src/index.ts"))
        .unwrap();
    for specifier in ["node:fs", "node:path", "node:url"] {
        assert_eq!(
            info.import_paths
                .get(specifier)
                .unwrap()
                .resolved_path
                .error(),
            Some(&ResolveError::NodeBuiltIn)
        );
    }
}

#[test]
fn test_package_typings_field_resolution() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), "import { Icon } from 'my-icons';");
    fs.insert(
        "/node_modules/my-icons/dist/index.d.ts".into(),
        "export declare function Icon(): void;",
    );
    let layout = ProjectLayout::default();
    layout.insert_node_manifest(
        "/".into(),
        PackageJson::new("app").with_dependencies(Dependencies(Box::new([(
            "my-icons".into(),
            "1.0.0".into(),
        )]))),
    );
    let manifest = biome_deserialize::json::deserialize_from_json_str::<PackageJson>(
        r#"{"name":"my-icons","version":"1.0.0","typings":"./dist/index.d.ts"}"#,
        Default::default(),
        "package.json",
    )
    .into_deserialized()
    .unwrap();
    layout.insert_node_manifest("/node_modules/my-icons".into(), manifest);
    let db = build_js_db(&fs, &layout, &[BiomePath::new("/src/index.ts")], false);
    assert_eq!(
        db.js_module_info_for_path(Utf8Path::new("/src/index.ts"))
            .unwrap()
            .static_imports
            .get("Icon")
            .unwrap()
            .resolved_path
            .as_path(),
        Some(Utf8Path::new("/node_modules/my-icons/dist/index.d.ts"))
    );
}

fn export_db(source: &str, barrel: &str) -> biome_service::db::WorkspaceDb {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/source.ts".into(), source);
    fs.insert("/src/barrel.ts".into(), barrel);
    build_js_db(
        &fs,
        &ProjectLayout::default(),
        &[
            BiomePath::new("/src/source.ts"),
            BiomePath::new("/src/barrel.ts"),
        ],
        false,
    )
}

#[test]
fn test_aliased_named_reexport_is_found_by_alias() {
    let db = export_db(
        "export function originalName() {}",
        "export { originalName as renamedSymbol } from './source.ts';",
    );
    let barrel = db.module_for_path(Utf8Path::new("/src/barrel.ts")).unwrap();
    assert!(matches!(
        find_js_exported_symbol(&db, SymbolFromModuleInfo::new(&db, "renamedSymbol", barrel)),
        JsExportedSymbolLookup::Found(_)
    ));
    assert_eq!(
        find_js_exported_symbol(&db, SymbolFromModuleInfo::new(&db, "originalName", barrel)),
        JsExportedSymbolLookup::Missing
    );
}

#[test]
fn test_namespace_reexport_is_own_export() {
    let db = export_db(
        "export function alpha() {}",
        "export * as MyNs from './source.ts';",
    );
    let barrel = db.module_for_path(Utf8Path::new("/src/barrel.ts")).unwrap();
    let kind = barrel.kind(&db);
    let info = kind.as_js_module_info().unwrap();
    assert_eq!(
        info.exports.get(&Text::new_static("MyNs")),
        Some(&JsExport::Own(JsOwnExport::Namespace(JsReexport {
            export_range: Some(TextRange::new(TextSize::from(0), TextSize::from(36))),
            import: JsImport {
                specifier: "./source.ts".into(),
                resolved_path: ResolvedPath::from_path("/src/source.ts"),
                symbol: ImportSymbol::All
            }
        })))
    );
}

#[test]
fn test_find_symbol_behind_unresolvable_reexport_is_unknown() {
    let db = export_db(
        "",
        "export { missing } from 'not-installed'; export const own = 1;",
    );
    let barrel = db.module_for_path(Utf8Path::new("/src/barrel.ts")).unwrap();
    assert_eq!(
        find_js_exported_symbol(&db, SymbolFromModuleInfo::new(&db, "missing", barrel)),
        JsExportedSymbolLookup::Unknown
    );
    assert_eq!(
        find_js_exported_symbol(&db, SymbolFromModuleInfo::new(&db, "absent", barrel)),
        JsExportedSymbolLookup::Missing
    );
}

#[test]
fn test_find_symbol_reexported_through_package_self_reference() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/node_modules/next/server.d.ts".into(),
        "export { NextRequest } from 'next/dist/request';",
    );
    fs.insert(
        "/node_modules/next/dist/request.d.ts".into(),
        "export declare class NextRequest {}",
    );
    let layout = ProjectLayout::default();
    layout.insert_node_manifest(
        "/node_modules/next".into(),
        PackageJson::new("next").with_version("16.0.0"),
    );
    let db = build_js_db(
        &fs,
        &layout,
        &[
            BiomePath::new("/node_modules/next/server.d.ts"),
            BiomePath::new("/node_modules/next/dist/request.d.ts"),
        ],
        false,
    );
    let server = db
        .module_for_path(Utf8Path::new("/node_modules/next/server.d.ts"))
        .unwrap();
    assert!(matches!(
        find_js_exported_symbol(&db, SymbolFromModuleInfo::new(&db, "NextRequest", server)),
        JsExportedSymbolLookup::Found(_)
    ));
}

#[test]
fn test_export_equals_namespace_without_type_inference() {
    let fs = MemoryFileSystem::default();
    fs.insert("/node_modules/react/index.d.ts".into(), "declare namespace React { function useState(): void; function useCallback(): void; } export = React;");
    let db = build_js_db(
        &fs,
        &ProjectLayout::default(),
        &[BiomePath::new("/node_modules/react/index.d.ts")],
        false,
    );
    let module = db
        .module_for_path(Utf8Path::new("/node_modules/react/index.d.ts"))
        .unwrap();
    for name in ["useState", "useCallback"] {
        assert!(matches!(
            find_js_exported_symbol(&db, SymbolFromModuleInfo::new(&db, name, module)),
            JsExportedSymbolLookup::Found(_)
        ));
    }
}

#[test]
fn test_namespace_import_preserves_members_above_export_step_limit() {
    let fs = MemoryFileSystem::default();
    let exports = (0..1025)
        .map(|index| format!("export const member{index} = {index};\n"))
        .collect::<String>();
    fs.insert("/src/source.ts".into(), exports);
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import * as source from "./source";
            export const first = source.member0;
            export const last = source.member1024;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/source.ts", "/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    for name in ["first", "last"] {
        let ty = inferred_binding_ty_by_name(&db, module, inferred, name)
            .expect("namespace member type must be inferred");
        assert!(is_inferred_number(&db, normalize_type(&db, module, ty)));
    }
}
#[test]
fn test_infer_module_types_resolves_namespace_import_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/source.ts".into(),
        r#"
            export function alpha(): number {
                return 1;
            }
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import * as source from "./source.ts";

            export { source };
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/source.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let source_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "source")
        .expect("source import type must be inferred");
    let InferredTypeData::Namespace(_) = source_ty else {
        panic!("namespace import must infer a namespace, got {source_ty:?}");
    };

    let alpha_ty = inferred
        .find_member_type(&db, source_ty, "alpha")
        .expect("namespace import must expose source.alpha");
    assert_inferred_function_returns_number(&db, alpha_ty);

    assert_inferred_type_snapshot(
        "test_infer_module_types_resolves_namespace_import_members",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_resolves_namespace_reexport_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/source.ts".into(),
        r#"
            export function alpha(): number {
                return 1;
            }
        "#,
    );
    fs.insert(
        "/src/barrel.ts".into(),
        r#"export * as MyNs from "./source.ts";"#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import { MyNs } from "./barrel.ts";

            export { MyNs };
        "#,
    );

    let db = build_js_test_module_db(
        &fs,
        &["/src/source.ts", "/src/barrel.ts", "/src/index.ts"],
        true,
    );
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let namespace_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "MyNs")
        .expect("MyNs import type must be inferred");
    let InferredTypeData::Namespace(_) = namespace_ty else {
        panic!("namespace reexport import must infer a namespace, got {namespace_ty:?}");
    };

    let alpha_ty = inferred
        .find_member_type(&db, namespace_ty, "alpha")
        .expect("namespace reexport must expose source.alpha");
    assert_inferred_function_returns_number(&db, alpha_ty);

    assert_inferred_type_snapshot(
        "test_infer_module_types_resolves_namespace_reexport_members",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_bottom_up_warms_blanket_reexports() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/leaf.ts".into(),
        r#"
            export type Source = {
                name: string;
            };
        "#,
    );
    fs.insert(
        "/src/mid.ts".into(),
        r#"
            export * from "./leaf.ts";
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import type { Source } from "./mid.ts";

            export function read(value: Source): Source {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/leaf.ts", "/src/mid.ts", "/src/index.ts"], true);
    let leaf_module = db
        .module_for_path(Utf8Path::new("/src/leaf.ts"))
        .expect("leaf module must exist");
    let mid_module = db
        .module_for_path(Utf8Path::new("/src/mid.ts"))
        .expect("mid module must exist");
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("index module must exist");

    db.clear_salsa_events();
    let inferred = infer_module_types_bottom_up(&db, index_module).expect("types must be inferred");
    let return_ty = inferred_function_return_ty_by_name(&db, index_module, inferred, "read")
        .expect("read return type must be inferred");
    let name_ty = inferred
        .find_member_type(&db, return_ty, "name")
        .expect("re-exported Source must expose name");
    assert!(is_inferred_string(&db, name_ty));

    let events = db.take_salsa_events();
    let leaf_position =
        function_query_will_execute_position(&db, infer_module_types, leaf_module, &events)
            .expect("leaf inference must run");
    let mid_position =
        function_query_will_execute_position(&db, infer_module_types, mid_module, &events)
            .expect("mid inference must run");
    let index_position =
        function_query_will_execute_position(&db, infer_module_types, index_module, &events)
            .expect("index inference must run");
    assert!(
        leaf_position < mid_position && mid_position < index_position,
        "bottom-up inference must warm blanket re-export dependencies before their importers"
    );
}

#[test]
fn test_infer_module_types_resolves_imported_exported_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/types.ts".into(),
        r#"
            export type Foo = string;
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import type { Foo } from "./types.ts";

            export const value: Foo = "value";
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/types.ts", "/src/index.ts"], true);
    let types_module = db
        .module_for_path(Utf8Path::new("/src/types.ts"))
        .expect("module must exist");
    let inferred_types = infer_module_types(&db, types_module).expect("types must be inferred");
    let ModuleInfoKind::Js(types_info) = types_module.kind(&db) else {
        panic!("module must be JavaScript");
    };
    assert_eq!(inferred_types.types.len(), types_info.raw_types.len());

    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    assert!(
        inferred
            .binding_type_data
            .values()
            .any(|data| data.ty == InferredTypeData::String)
    );
}

#[test]
fn test_infer_module_types_resolves_anonymous_default_class_export() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export default class {
                name: string;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let ModuleInfoKind::Js(js_info) = index_module.kind(&db) else {
        panic!("module must be JavaScript");
    };
    let default_ty = match js_info
        .exports
        .get("default")
        .and_then(JsExport::as_own_export)
    {
        Some(JsOwnExport::Type(resolved_id)) => inferred
            .types
            .get(resolved_id.index())
            .copied()
            .expect("default export type must be inferred"),
        _ => panic!("default export must have a type"),
    };

    assert!(!js_info.raw_types.is_empty());
    assert!(inferred.named_type_ids.is_empty());

    let name_ty = inferred
        .find_member_type(&db, default_ty, "name")
        .expect("anonymous default class member must be inferred");
    assert!(is_inferred_string(&db, name_ty));
}

#[test]
fn test_infer_module_types_resolves_anonymous_default_function_export() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export default function(): string {
                return "value";
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let ModuleInfoKind::Js(js_info) = index_module.kind(&db) else {
        panic!("module must be JavaScript");
    };
    let default_ty = match js_info
        .exports
        .get("default")
        .and_then(JsExport::as_own_export)
    {
        Some(JsOwnExport::Type(resolved_id)) => inferred
            .types
            .get(resolved_id.index())
            .copied()
            .expect("default export type must be inferred"),
        _ => panic!("default export must have a type"),
    };

    assert!(!js_info.raw_types.is_empty());
    assert!(inferred.named_type_ids.is_empty());
    assert_inferred_function_returns_string(&db, inferred.resolve_type(&db, default_ty));
}

#[test]
fn test_infer_module_types_resolves_imported_anonymous_default_class_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/base.ts".into(),
        r#"
            export default class {
                name: string;
            }
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import Base from "./base.ts";

            export const value: Base = new Base();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/base.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let value_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "value")
        .expect("value binding type must be inferred");

    let name_ty = inferred
        .find_member_type(&db, value_ty, "name")
        .expect("imported anonymous default class member must be inferred");
    assert!(is_inferred_string(&db, name_ty));
}

#[test]
fn test_infer_module_types_resolves_imported_anonymous_default_function() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/base.ts".into(),
        r#"
            export default function(): string {
                return "value";
            }
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import readValue from "./base.ts";

            export { readValue };
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/base.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let read_value_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "readValue")
        .expect("readValue binding type must be inferred");

    assert_inferred_function_returns_string(&db, inferred.resolve_type(&db, read_value_ty));
}

#[test]
fn test_infer_module_types_resolves_imported_local_handle_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/types.ts".into(),
        r#"
            export class Foo {
                name: string;

                static create(): Foo {
                    return new Foo();
                }
            }
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import { Foo } from "./types.ts";

            export const value: Foo = Foo.create();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/types.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let foo_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "Foo")
        .expect("Foo import type must be inferred");

    let create_ty = inferred
        .find_member_type(&db, foo_ty, "create")
        .expect("Foo.create must be found through the imported local handle");
    let InferredTypeData::Function(create_function) = create_ty else {
        panic!("Foo.create must be a function");
    };
    let InferredReturnType::Type(return_ty) = create_function.return_type(&db) else {
        panic!("Foo.create return type must be a type");
    };

    let name_ty = inferred
        .find_member_type(&db, *return_ty, "name")
        .expect("Foo.create().name must be found through the imported local handle");
    assert!(is_inferred_string(&db, name_ty));
}

#[test]
fn test_infer_module_types_resolves_imported_inherited_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/base.ts".into(),
        r#"
            export class Base {
                name: string;

                static label(): string {
                    return "base";
                }
            }
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import { Base } from "./base.ts";

            export class Derived extends Base {
                value: number;
            }

            export const derived: Derived = new Derived();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/base.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let derived_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "derived")
        .expect("derived binding type must be inferred");
    let name_ty = inferred
        .find_member_type(&db, derived_ty, "name")
        .expect("Derived instance must inherit imported Base.name");
    assert!(is_inferred_string(&db, name_ty));

    let derived_class_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "Derived")
        .expect("Derived class type must be inferred");
    let label_ty = inferred
        .find_member_type(&db, derived_class_ty, "label")
        .expect("Derived class must inherit imported Base.label");
    assert!(matches!(label_ty, InferredTypeData::Function(_)));
}

#[test]
fn test_infer_module_types_resolves_imported_interface_extends_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/base.ts".into(),
        r#"
            export interface Base {
                name: string;
            }
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import type { Base } from "./base.ts";

            export interface Derived extends Base {
                value: number;
            }

            export const derived: Derived = {
                name: "derived",
                value: 1,
            };
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/base.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let derived_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "derived")
        .expect("derived binding type must be inferred");

    let name_ty = inferred
        .find_member_type(&db, derived_ty, "name")
        .expect("Derived interface must inherit imported Base.name");
    assert!(is_inferred_string(&db, name_ty));
}

#[test]
fn test_infer_module_types_resolves_redis_commander_types() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/RedisCommander.d.ts".into(),
        include_bytes!("../../benches/RedisCommander.d.ts"),
    );
    fs.insert(
        "/index.ts".into(),
        r#"import RedisCommander from "./RedisCommander.d.ts";
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/RedisCommander.d.ts", "/index.ts"], true);
    let commander_module = db
        .module_for_path(Utf8Path::new("/RedisCommander.d.ts"))
        .expect("module must exist");
    let commander_inferred =
        infer_module_types_bottom_up(&db, commander_module).expect("types must be inferred");
    assert!(!commander_inferred.types.is_empty());

    let index_module = db
        .module_for_path(Utf8Path::new("/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types_bottom_up(&db, index_module).expect("types must be inferred");
    let commander_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "RedisCommander")
        .expect("RedisCommander binding type must be inferred");
    let commander_ty = inferred.resolve_type(&db, commander_ty);
    assert_ne!(commander_ty, InferredTypeData::Unknown);
}
