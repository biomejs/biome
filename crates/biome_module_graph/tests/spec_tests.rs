#![allow(clippy::arc_with_non_send_sync)]

mod snap;

use std::fs::read_link;
use std::ops::Deref;
use std::sync::Arc;

use biome_resolver::ResolveError;

use crate::snap::ModuleGraphSnapshot;
use biome_configuration::{Configuration, HtmlConfiguration};
use biome_css_parser::{CssModulesKind, CssParserOptions};
use biome_css_syntax::{
    CssFileSource, EmbeddingHtmlKind, EmbeddingKind, EmbeddingStyleApplicability,
};
use biome_deserialize::json::deserialize_from_json_str;
use biome_fs::{BiomePath, FileSystem, MemoryFileSystem, OsFileSystem, normalize_path};
use biome_html_parser::HtmlParserOptions;
use biome_html_syntax::HtmlFileSource;
use biome_js_semantic::ScopeId;
use biome_js_type_info::{TypeData, TypeResolver};
use biome_jsdoc_comment::JsdocComment;
use biome_json_parser::{JsonParserOptions, parse_json};
use biome_json_value::{JsonObject, JsonString};
use biome_module_graph::{
    HtmlEmbeddedContent, ImportSymbol, JsExport, JsImport, JsImportPath, JsImportPhase,
    JsModuleInfoDiagnostic, JsOwnExport, JsReexport, ModuleDiagnostic, ModuleGraph, ModuleResolver,
    ResolvedPath,
};
use biome_package::{Dependencies, PackageJson};
use biome_project_layout::ProjectLayout;
use biome_rowan::Text;
use biome_service::Workspace;
use biome_service::file_handlers::DocumentFileSource;
use biome_service::settings::ModuleGraphResolutionKind;
use biome_service::test_utils::setup_workspace_and_open_project;
use biome_service::workspace::UpdateSettingsParams;
use biome_test_utils::{get_added_js_paths, get_css_added_paths};
use camino::{Utf8Path, Utf8PathBuf};
use walkdir::WalkDir;

fn create_test_project_layout() -> (MemoryFileSystem, ProjectLayout) {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import { foo } from "shared";
            import { bar } from "./bar.ts";
            import { Hello } from "@components/Hello";

            foo();
        "#,
    );
    fs.insert(
        "/src/bar.ts".into(),
        r#"
            export function bar() {}
        "#,
    );
    fs.insert(
        "/src/components/Hello.tsx".into(),
        r#"
            export function Hello() {}
        "#,
    );

    fs.insert(
        "/node_modules/shared/dist/index.js".into(),
        r#"
            export function foo() {}
        "#,
    );

    fs.insert(
        "/node_modules/shared/dist/index.d.ts".into(),
        r#"
            declare namespace shared {
                type Result = string;
            }

            declare const shared: {
                foo(): shared.Result;
            }

            export = shared;
        "#,
    );

    let project_layout = ProjectLayout::default();
    project_layout.insert_node_manifest(
        "/".into(),
        PackageJson::new("frontend")
            .with_version("0.0.0")
            .with_dependencies(Dependencies(Box::new([(
                "shared".into(),
                "link:./node_modules/shared".into(),
            )]))),
    );

    let tsconfig_json = parse_json(
        r#"{
        "compilerOptions": {
            "paths": {
                "@components/*": ["./src/components/*"]
            }
        }
    }"#,
        JsonParserOptions::default(),
    );
    project_layout
        .insert_serialized_tsconfig("/".into(), &tsconfig_json.syntax().as_send().unwrap());

    project_layout.insert_node_manifest(
        "/node_modules/shared".into(),
        PackageJson::new("shared")
            .with_exports(JsonObject::from([
                ("types".into(), JsonString::from("./dist/index.d.ts").into()),
                ("default".into(), JsonString::from("./dist/index.js").into()),
            ]))
            .with_version("0.0.1"),
    );

    (fs, project_layout)
}

/// Returns the path to the `fixtures/` directory, regardless of working dir.
fn get_fixtures_path() -> Utf8PathBuf {
    let mut path: Utf8PathBuf = std::env::current_dir().unwrap().try_into().unwrap();
    while !path.join("Cargo.lock").exists() {
        path = path
            .parent()
            .expect("couldn't find Cargo.lock")
            .to_path_buf();
    }
    path.join("crates/biome_module_graph/tests/fixtures")
}

#[test]
fn test_type_flattening_does_not_explode_on_recursive_parent_element_pattern() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/repro.ts".into(),
        r#"
            const root = {} as Element;

            for (let el: Element | null = root; el && el !== root; el = el.parentElement) {
                // noop
            }
        "#,
    );

    let project_layout = ProjectLayout::default();
    let added_paths = [BiomePath::new("/src/repro.ts")];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &project_layout, &added_paths, true);

    let data = module_graph.data();
    let module = data.get(Utf8Path::new("/src/repro.ts")).unwrap();
    let module = module.as_js_module_info().unwrap();

    assert!(
        !module.diagnostics().iter().any(|diagnostic| matches!(
            diagnostic,
            ModuleDiagnostic::JsInfo(JsModuleInfoDiagnostic::ExceededTypesLimit(_))
        )),
        "expected module graph not to hit the types-limit diagnostic",
    );
}

#[test]
fn test_resolve_relative_import() {
    let (fs, project_layout) = create_test_project_layout();
    let added_paths = [
        BiomePath::new("/src/index.ts"),
        BiomePath::new("/src/bar.ts"),
    ];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &project_layout, &added_paths, true);

    let imports = module_graph.data();
    let file_imports = imports.get(Utf8Path::new("/src/index.ts")).unwrap();
    let file_imports = file_imports.as_js_module_info().unwrap();

    assert_eq!(file_imports.static_imports.len(), 3);
    assert_eq!(
        file_imports.static_imports.get("bar"),
        Some(&JsImport {
            specifier: "./bar.ts".into(),
            resolved_path: ResolvedPath::from_path("/src/bar.ts"),
            symbol: "bar".into()
        })
    );
}

#[test]
fn test_resolve_package_import() {
    let (fs, project_layout) = create_test_project_layout();
    let added_paths = [
        BiomePath::new("/src/index.ts"),
        BiomePath::new("/node_modules/shared/dist/index.js"),
    ];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &project_layout, &added_paths, true);

    let imports = module_graph.data();
    let file_imports = imports.get(Utf8Path::new("/src/index.ts")).unwrap();
    let file_imports = file_imports.as_js_module_info().unwrap();

    assert_eq!(file_imports.static_imports.len(), 3);
    assert_eq!(
        file_imports.static_imports.get("foo"),
        Some(&JsImport {
            specifier: "shared".into(),
            resolved_path: ResolvedPath::from_path("/node_modules/shared/dist/index.d.ts"),
            symbol: "foo".into()
        })
    );
}

#[test]
fn test_import_through_path_alias() {
    let (fs, project_layout) = create_test_project_layout();
    let added_paths = [
        BiomePath::new("/src/index.ts"),
        BiomePath::new("/src/components/Hello.tsx"),
    ];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &project_layout, &added_paths, true);

    let imports = module_graph.data();
    let file_imports = imports.get(Utf8Path::new("/src/index.ts")).unwrap();
    let file_imports = file_imports.as_js_module_info().unwrap();

    assert_eq!(file_imports.static_imports.len(), 3);
    assert_eq!(
        file_imports.static_imports.get("Hello"),
        Some(&JsImport {
            specifier: "@components/Hello".into(),
            resolved_path: ResolvedPath::from_path("/src/components/Hello.tsx"),
            symbol: "Hello".into()
        })
    );
}

#[test]
fn test_resolve_package_import_in_monorepo_fixtures() {
    let fixtures_path = get_fixtures_path();

    let fs = OsFileSystem::new(fixtures_path.clone());

    let project_layout = ProjectLayout::default();
    project_layout.insert_node_manifest(format!("{fixtures_path}/frontend").into(), {
        let path = Utf8PathBuf::from(format!("{fixtures_path}/frontend/package.json"));
        deserialize_from_json_str::<PackageJson>(
            &fs.read_file_from_path(&path)
                .expect("package.json must be readable"),
            JsonParserOptions::default(),
            "package.json",
        )
        .into_deserialized()
        .expect("package.json must parse")
    });

    project_layout.insert_node_manifest(format!("{fixtures_path}/shared").into(), {
        let path = Utf8PathBuf::from(format!("{fixtures_path}/shared/package.json"));
        deserialize_from_json_str::<PackageJson>(
            &fs.read_file_from_path(&path)
                .expect("package.json must be readable"),
            JsonParserOptions::default(),
            "package.json",
        )
        .into_deserialized()
        .expect("package.json must parse")
    });

    project_layout.insert_node_manifest(
        format!("{fixtures_path}/frontend/node_modules/shared").into(),
        {
            let path = Utf8PathBuf::from(format!(
                "{fixtures_path}/frontend/node_modules/shared/package.json"
            ));
            deserialize_from_json_str::<PackageJson>(
                &fs.read_file_from_path(&path)
                    .expect("package.json must be readable"),
                JsonParserOptions::default(),
                "package.json",
            )
            .into_deserialized()
            .expect("package.json must parse")
        },
    );

    let added_paths = [
        BiomePath::new(format!("{fixtures_path}/frontend/src/bar.ts")),
        BiomePath::new(format!("{fixtures_path}/frontend/src/index.ts")),
        BiomePath::new(format!(
            "{fixtures_path}/frontend/node_modules/shared/dist/index.js"
        )),
        BiomePath::new(format!("{fixtures_path}/shared/dist/index.js")),
    ];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &project_layout, &added_paths, true);

    let imports = module_graph.data();
    let file_imports = imports
        .get(Utf8Path::new(&format!(
            "{fixtures_path}/frontend/src/index.ts"
        )))
        .unwrap();
    let file_imports = file_imports.as_js_module_info().unwrap();

    assert_eq!(file_imports.static_imports.len(), 3);
    assert_eq!(
        file_imports.static_imports.get("sharedFoo"),
        Some(&JsImport {
            specifier: "shared".into(),
            resolved_path: ResolvedPath::from_path(format!("{fixtures_path}/shared/dist/index.js")),
            symbol: "sharedFoo".into()
        })
    );
    assert_eq!(
        file_imports.static_imports.get("bar"),
        Some(&JsImport {
            specifier: "./bar".into(),
            resolved_path: ResolvedPath::from_path(format!("{fixtures_path}/frontend/src/bar.ts")),
            symbol: "bar".into()
        })
    );
}

#[test]
fn test_export_referenced_function() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            /**
             * @returns {string}
             */
            function foo() {}

            export { foo };
        "#,
    );

    let added_paths = [BiomePath::new("/src/index.ts")];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let snapshot = ModuleGraphSnapshot::new(&module_graph, &fs);

    snapshot.assert_snapshot("test_export_referenced_function");
}

#[test]
fn test_export_default_function_declaration() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            /**
             * @public
             * @returns {JSX.Element}
             */
            export default function Component(): JSX.Element {}
        "#,
    );

    let added_paths = [BiomePath::new("/src/index.ts")];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let snapshot = ModuleGraphSnapshot::new(&module_graph, &fs);
    snapshot.assert_snapshot("test_export_default_function_declaration");
}

#[test]
fn test_export_default_imported_binding() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/foo.ts".into(),
        r#"
            /**
             * @returns {number}
             */
            export function foo(): number {
                return 42;
            }
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import { foo } from "./foo.ts";

            export default foo;
        "#,
    );

    let added_paths = [
        BiomePath::new("/src/foo.ts"),
        BiomePath::new("/src/index.ts"),
    ];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let index_module = module_graph
        .js_module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    // Test that the default export's type is correctly resolved as a function returning number
    let default_export_ty = resolver
        .resolved_type_of_default_export()
        .expect("default export must exist");
    assert!(
        default_export_ty.is_function(),
        "Default export should be a function, got: {default_export_ty:?}"
    );

    let snapshot = ModuleGraphSnapshot::new(module_graph.as_ref(), &fs).with_resolver(&resolver);
    snapshot.assert_snapshot("test_export_default_imported_binding");
}

#[test]
fn test_export_const_type_declaration_with_namespace() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.d.ts".into(),
        r#"
            declare namespace shared {
                type Result = string;
            }

            declare const shared: {
                foo(): shared.Result;
            }

            export = shared;
        "#,
    );

    let added_paths = [BiomePath::new("/src/index.d.ts")];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let snapshot = ModuleGraphSnapshot::new(&module_graph, &fs);
    snapshot.assert_snapshot("test_export_const_type_declaration_with_namespace");
}

#[test]
fn test_resolve_exports() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            /**
             * @returns {string}
             */
            function foo() {}

            export { foo, qux };

            /** @package */
            export function bar() {}

            /** @private */
            export const quz = {};

            /* @ignored because of incorrect amount of asterisks */
            export async function baz() {}

            var qux = 1;

            export const { a, b, c: [d, e] } = getObject();

            type GetObjectResult = {
                a: string,
                b: Array<number>,
                c: [first: boolean, last: boolean | undefined],
            };

            function getObject(): GetObjectResult {
                return {}; // We're not a type checker, so this is a-okay.
            }

            /**
             * @public
             * @returns {JSX.Element}
             */
            export default function Component(): JSX.Element {}

            export * from "./reexports";
            export { ohNo as "oh\x0Ano" } from "./renamed-reexports";

            /**
             * Hello, namespace 2.
             */
            export * as renamed2 from "./renamed-reexports";
        "#,
    );
    fs.insert(
        "/src/reexports.ts".into(),
        r#"
            /**
             * Hello, namespace 1.
             */
            export * as renamed from "./renamed-reexports";
        "#,
    );
    fs.insert(
        "/src/renamed-reexports.ts".into(),
        r#"
            export function ohNo() {}
        "#,
    );

    let project_layout = ProjectLayout::default();
    project_layout.insert_node_manifest(
        "/".into(),
        PackageJson::new("frontend").with_version("0.0.0"),
    );

    let added_paths = [
        BiomePath::new("/src/index.ts"),
        BiomePath::new("/src/reexports.ts"),
        BiomePath::new("/src/renamed-reexports.ts"),
    ];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &project_layout, &added_paths, true);

    let dependency_data = module_graph.data();
    let data = dependency_data.get(Utf8Path::new("/src/index.ts")).unwrap();
    let data = data.as_js_module_info().unwrap();
    let mut exports = data.exports.clone();

    // Remove this entry, or the Windows tests fail on the path in the snapshot below:
    assert_eq!(
        exports.swap_remove(&Text::new_static("oh\nno")),
        Some(JsExport::Reexport(JsReexport {
            import: JsImport {
                specifier: "./renamed-reexports".into(),
                resolved_path: ResolvedPath::from_path("/src/renamed-reexports.ts"),
                symbol: "ohNo".into()
            },
            jsdoc_comment: None
        }))
    );
    assert_eq!(
        exports.swap_remove(&Text::new_static("renamed2")),
        Some(JsExport::Own(JsOwnExport::Namespace(JsReexport {
            import: JsImport {
                specifier: "./renamed-reexports".into(),
                resolved_path: ResolvedPath::from_path("/src/renamed-reexports.ts"),
                symbol: ImportSymbol::All,
            },
            jsdoc_comment: Some(JsdocComment::from_comment_text(
                "/**\n* Hello, namespace 2.\n*/"
            )),
        })))
    );

    assert_eq!(
        data.blanket_reexports,
        &[JsReexport {
            import: JsImport {
                specifier: "./reexports".into(),
                resolved_path: ResolvedPath::from_path("/src/reexports.ts"),
                symbol: ImportSymbol::All,
            },
            jsdoc_comment: None
        }]
    );

    let data = dependency_data
        .get(Utf8Path::new("/src/reexports.ts"))
        .unwrap();
    let data = data.as_js_module_info().unwrap();
    assert_eq!(data.exports.len(), 1);
    assert_eq!(
        data.exports.get(&Text::new_static("renamed")),
        Some(&JsExport::Own(JsOwnExport::Namespace(JsReexport {
            import: JsImport {
                specifier: "./renamed-reexports".into(),
                resolved_path: ResolvedPath::from_path("/src/renamed-reexports.ts"),
                symbol: ImportSymbol::All,
            },
            jsdoc_comment: Some(JsdocComment::from_comment_text(
                "/**\n* Hello, namespace 1.\n*/"
            )),
        })))
    );

    let snapshot = ModuleGraphSnapshot::new(&module_graph, &fs);
    snapshot.assert_snapshot("test_resolve_exports");
}

#[test]
fn test_resolve_export_types() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export const theAnswer = 42;

            /**
             * Built by a race of hyper-intelligent pan-dimensional beings to
             * calculate the Ultimate Answer to the Ultimate Question of Life,
             * The Universe, and Everything.
             *
             * This JSDoc comment should not be transferred to the exported
             * instance variable below.
             */
            class DeepThought {
                answerMe(): number {
                    return theAnswer;
                }

                giveMeABiggerAnswer(delta: number) {
                    // Return type should be inferred to `number`.
                    // TODO: At some point.
                    return theAnswer + delta;
                }

                whatWasTheUltimateQuestion(): unknown {
                    // This should not be inferred to `string` due to the
                    // explicit annotation in the signature.
                    return "Life, The Universe, and Everything";
                }
            }

            export const superComputer = new DeepThought();
        "#,
    );

    let added_paths = [BiomePath::new("/src/index.ts")];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let snapshot = ModuleGraphSnapshot::new(&module_graph, &fs);
    snapshot.assert_snapshot("test_resolve_export_types");
}

#[test]
fn test_resolve_generic_return_value() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"function useCallback<T extends Function>(
    callback: T,
    deps: DependencyList,
): T;

export const makePromise = (): Promise => Promise.resolve(1);

export const makePromiseCb = useCallback(makePromise);

export const promise = makePromiseCb();
"#,
    );

    let added_paths = [BiomePath::new("/src/index.ts")];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let index_module = module_graph
        .js_module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let promise_id = resolver
        .resolve_type_of(&Text::new_static("promise"), ScopeId::GLOBAL)
        .expect("promise variable not found");
    let promise_ty = resolver.resolved_type_for_id(promise_id);
    assert!(promise_ty.is_promise_instance());

    let snapshot = ModuleGraphSnapshot::new(module_graph.as_ref(), &fs).with_resolver(&resolver);
    snapshot.assert_snapshot("test_resolve_generic_return_value");
}

#[test]
fn test_resolve_generic_mapped_value() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"const mapped = [1, 2, 3].map(async (x) => x + 1);
"#,
    );

    let added_paths = [BiomePath::new("/src/index.ts")];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let index_module = module_graph
        .js_module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let mapped_id = resolver
        .resolve_type_of(&Text::new_static("mapped"), ScopeId::GLOBAL)
        .expect("mapped variable not found");
    let mapped_ty = resolver.resolved_type_for_id(mapped_id);
    let _mapped_ty_string = format!("{:?}", mapped_ty.deref()); // for debugging
    assert!(mapped_ty.is_array_of(|elem_ty| {
        let _elem_ty_string = format!("{:?}", elem_ty.deref()); // for debugging
        elem_ty.is_promise_instance()
    }));

    let snapshot = ModuleGraphSnapshot::new(module_graph.as_ref(), &fs).with_resolver(&resolver);
    snapshot.assert_snapshot("test_resolve_generic_mapped_value");
}

#[test]
fn test_resolve_generic_return_value_with_multiple_modules() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/bar.ts".into(),
        r#"
        export type Bar = { bar: "bar" };
        "#,
    );
    fs.insert(
        "/src/foo.ts".into(),
        r#"
        import type { Bar } from "./bar.ts";

        export function foo<T>(foo: T, bar: Bar): T;
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"import type { Bar } from "./bar.ts";
        import { foo } from "./foo.ts";

        const bar: Bar = { bar: "bar" };

        const stringyBar = bar.bar;

        const result = foo(bar.bar, 1);
        "#,
    );

    let added_paths = [
        BiomePath::new("/src/foo.ts"),
        BiomePath::new("/src/bar.ts"),
        BiomePath::new("/src/index.ts"),
    ];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let index_module = module_graph
        .js_module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let result_id = resolver
        .resolve_type_of(&Text::new_static("result"), ScopeId::GLOBAL)
        .expect("result variable not found");
    let result_ty = resolver.resolved_type_for_id(result_id);
    assert!(result_ty.is_string_or_string_literal());

    let snapshot =
        ModuleGraphSnapshot::new(module_graph.as_ref(), &fs).with_resolver(resolver.as_ref());
    snapshot.assert_snapshot("test_resolve_generic_return_value_with_multiple_modules");
}

#[test]
fn test_resolve_import_as_namespace() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/foo.ts".into(),
        r#"
        export function foo(): number {
            return 1;
        }
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"import * as fooNs from "./foo.ts";

        const result = fooNs.foo();
        "#,
    );

    let added_paths = [
        BiomePath::new("/src/foo.ts"),
        BiomePath::new("/src/index.ts"),
    ];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let index_module = module_graph
        .js_module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let result_id = resolver
        .resolve_type_of(&Text::new_static("result"), ScopeId::GLOBAL)
        .expect("result variable not found");
    let result_ty = resolver.resolved_type_for_id(result_id);
    assert!(result_ty.is_number_or_number_literal());

    let snapshot = ModuleGraphSnapshot::new(module_graph.as_ref(), &fs).with_resolver(&resolver);
    snapshot.assert_snapshot("test_resolve_import_as_namespace");
}

#[test]
fn test_resolve_nested_function_call_with_namespace_in_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/foo.ts".into(),
        r#"
        export function foo(): Type {}
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"import { foo } from "./foo.ts";

        const result = bar(foo());
        "#,
    );

    let added_paths = [
        BiomePath::new("/src/foo.ts"),
        BiomePath::new("/src/index.ts"),
    ];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let index_module = module_graph
        .js_module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = ModuleResolver::for_module(index_module, module_graph.clone());

    let snapshot = ModuleGraphSnapshot::new(module_graph.as_ref(), &fs).with_resolver(&resolver);
    snapshot.assert_snapshot("test_resolve_nested_function_call_with_namespace_in_return_type");
}

#[test]
fn test_resolve_return_value_of_function() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        export function foo(input: number) {
            switch (input) {
                case 0: return null;
                case 1: return "one";
                case 2: return "two";
                default: return "many";
            }
            return "many"; // Check if this one gets deduplicated.
        }
        "#,
    );

    let added_paths = [BiomePath::new("/src/index.ts")];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let index_module = module_graph
        .js_module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let foo_id = resolver
        .resolve_type_of(&Text::new_static("foo"), ScopeId::GLOBAL)
        .expect("foo variable not found");
    let foo_ty = resolver.resolved_type_for_id(foo_id);
    let _foo_string_ty = format!("{foo_ty:?}");
    let return_ty = foo_ty
        .as_function()
        .expect("foo must be a function")
        .return_type
        .as_type()
        .and_then(|return_ty| foo_ty.resolve(return_ty))
        .expect("expected a resolvable return type");
    assert!(return_ty.has_variant(|ty| ty.is_string_literal("one")));
    assert!(return_ty.has_variant(|ty| ty.is_string_literal("two")));
    assert!(return_ty.has_variant(|ty| ty.is_string_literal("many")));
    assert!(return_ty.has_variant(|ty| ty.is_null()));
    match return_ty.resolved_data().unwrap().as_raw_data() {
        TypeData::Union(union) => assert_eq!(union.types().len(), 4),
        _ => panic!("expected a union type"),
    }
}

#[test]
fn test_resolve_type_of_property_with_getter() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        class Foo {
            get foo() {
                if (!this.initialised) {
                    this.initialise();
                    return "foo";
                }

                return "foo";
            }
        }

        const fooness = new Foo();
        const foo = fooness.foo;
        "#,
    );

    let added_paths = [BiomePath::new("/src/index.ts")];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let index_module = module_graph
        .js_module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let foo_id = resolver
        .resolve_type_of(&Text::new_static("foo"), ScopeId::GLOBAL)
        .expect("foo variable not found");
    let foo_ty = resolver.resolved_type_for_id(foo_id);
    let _foo_string_ty = format!("{foo_ty:?}");
    assert!(foo_ty.is_string_literal("foo"));

    let snapshot = ModuleGraphSnapshot::new(&module_graph, &fs);
    snapshot.assert_snapshot("test_resolve_type_of_property_with_getter");
}

macro_rules! class_tests {
    ($($name:ident: $prefix:expr,)*) => {
    $(
        #[test]
        fn $name() {
            class_this_test_helper(stringify!($name), $prefix);
        }
    )*
    }
}

class_tests! {
    test_resolve_type_of_this_in_class_plain: "class Foo",
    test_resolve_type_of_this_in_class_assign: "const Foo = class",
    test_resolve_type_of_this_in_class_export: "export default class Foo",
}

fn class_this_test_helper(case_name: &str, prefix: &str) {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        format!(
            "{prefix} {}",
            r#"
        {
            x = 'foo';
            y = this.x;

            get fooGetter() {
                return this.x
            }

            arrow = () => this.x

            func = function() {
                return this.x
            }

            meth() {
                return this.x
            }

            nestedArrow() {
                const fn = () => this.x;
                return fn();
            }

            inObject() {
                const inner = {
                    x: this.x
                };
                return inner.x;
            }
        }

        const obj = new Foo();

        const foo1 = obj.y;
        const foo2 = obj.fooGetter;
        const foo3 = obj.arrow();
        const foo4 = obj.func();
        const foo5 = obj.meth();
        const foo6 = obj.nestedArrow();
        const foo7 = obj.inObject();
        "#
        ),
    );

    let added_paths = [BiomePath::new("/src/index.ts")];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let index_module = module_graph
        .js_module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    for i in 1..=7 {
        let name = format!("foo{i}");
        let foo_id = resolver
            .resolve_type_of(&Text::from(name.clone()), ScopeId::GLOBAL)
            .unwrap_or_else(|| panic!("{name} variable not found"));
        let foo_ty = resolver.resolved_type_for_id(foo_id);
        assert!(foo_ty.is_string_literal("foo"), "{name}: {foo_ty:?}");
    }

    let snapshot = ModuleGraphSnapshot::new(&module_graph, &fs);
    snapshot.assert_snapshot(case_name);
}

#[test]
fn test_resolve_type_of_this_in_object() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        const obj = {
            x: 'foo',
            y: this.x,

            get fooGetter() {
                return this.x
            },

            arrow: () => this.x,

            func: function() {
                return this.x
            },

            meth() {
                return this.x
            },

            nestedArrow() {
                const fn = () => this.x;
                return fn();
            },

            inObject() {
                const inner = {
                    x: this.x
                };
                return inner.x;
            },
        };

        const foo1 = obj.fooGetter;
        const foo2 = obj.func();
        const foo3 = obj.meth();
        const foo4 = obj.nestedArrow();
        const foo5 = obj.inObject();

        const notFoo1 = obj.y;
        const notFoo2 = obj.arrow();
        "#,
    );

    let added_paths = [BiomePath::new("/src/index.ts")];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let index_module = module_graph
        .js_module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    for i in 1..=5 {
        let name = format!("foo{i}");
        let foo_id = resolver
            .resolve_type_of(&Text::from(name.clone()), ScopeId::GLOBAL)
            .unwrap_or_else(|| panic!("{name} variable not found"));
        let foo_ty = resolver.resolved_type_for_id(foo_id);
        assert!(foo_ty.is_string_literal("foo"), "{name}: {foo_ty:?}");
    }
    for i in 1..=2 {
        let name = format!("notFoo{i}");
        let foo_id = resolver
            .resolve_type_of(&Text::from(name.clone()), ScopeId::GLOBAL)
            .unwrap_or_else(|| panic!("{name} variable not found"));
        let foo_ty = resolver.resolved_type_for_id(foo_id);
        assert!(!foo_ty.is_string_literal("foo"), "{name}: {foo_ty:?}");
    }

    let snapshot = ModuleGraphSnapshot::new(&module_graph, &fs);
    snapshot.assert_snapshot("test_resolve_type_of_this_in_object");
}

#[test]
fn test_resolve_type_of_this_in_class_wrong_scope() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        class Foo {
            x = 'foo';

            nested() {
                const fn = function() {
                    return this.x;
                }
                return fn();
            }
            nested2() {
                function fn() {
                    return this.x;
                }
                return fn();
            }

            nestedObject() {
                const inner = {
                    fn: function() {
                        return this.x;
                    }
                };
                return inner.fn();
            }
            nestedObject2() {
                const inner = {
                    fn() {
                        return this.x;
                    }
                };
                return inner.fn();
            }

            nestedInArrow = () => {
                const fn = function() {
                    return this.x;
                }
                return fn();
            }
        }

        const obj = new Foo();

        const notFoo1 = obj.nested();
        const notFoo2 = obj.nested2();
        const notFoo3 = obj.nestedInArrow();
        const notFoo4 = obj.nestedObject();
        const notFoo5 = obj.nestedObject2();
        "#,
    );

    let added_paths = [BiomePath::new("/src/index.ts")];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let index_module = module_graph
        .js_module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    for i in 1..=5 {
        let name = format!("notFoo{i}");
        let foo_id = resolver
            .resolve_type_of(&Text::from(name.clone()), ScopeId::GLOBAL)
            .unwrap_or_else(|| panic!("{name} variable not found"));
        let foo_ty = resolver.resolved_type_for_id(foo_id);
        assert!(!foo_ty.is_string_literal("foo"), "{name}: {foo_ty:?}");
    }

    let snapshot = ModuleGraphSnapshot::new(&module_graph, &fs);
    snapshot.assert_snapshot("test_resolve_type_of_this_in_class_wrong_scope");
}

#[test]
fn test_resolve_promise_export() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            async function returnsPromise() {
                return 'value';
            }

            export const promise = returnsPromise();
        "#,
    );

    let added_paths = [BiomePath::new("/src/index.ts")];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let snapshot = ModuleGraphSnapshot::new(&module_graph, &fs);
    snapshot.assert_snapshot("test_resolve_promise_export");
}

#[test]
fn test_resolve_merged_types() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"type A = 'a';
type B = 'b';
type C = 'c';
export type Union = A | B | C;

const A = 'a';
const B = 1;
const C = true;

export type Union2 = typeof A | typeof B | typeof C;

export { A, B };
"#,
    );

    let added_paths = [BiomePath::new("/src/index.ts")];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let snapshot = ModuleGraphSnapshot::new(&module_graph, &fs);
    snapshot.assert_snapshot("test_resolve_merged_types");
}

#[test]
fn test_resolve_merged_namespace_with_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"export namespace Foo {
    interface Bar {};
}

export type Foo = Foo.Bar;
"#,
    );

    let added_paths = [BiomePath::new("/src/index.ts")];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let snapshot = ModuleGraphSnapshot::new(&module_graph, &fs);

    snapshot.assert_snapshot("test_resolve_merged_namespace_with_type");
}

#[test]
fn test_resolve_recursive_looking_country_info() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/node_modules/@types/iso-3166-2/index.d.ts".into(),
        r#"// Type definitions for iso-3166-2 1.0
// Project: https://github.com/olahol/iso-3166-2.js
// Definitions by: Matt Rollins <https://github.com/sicilica>, Emily Klassen <https://github.com/forivall>
// Definitions: https://github.com/DefinitelyTyped/DefinitelyTyped

export namespace CountryInfo {
    interface Partial {
        name: string;
        sub: SubdivisionInfo.Map;
    }
    interface Full extends Partial {
        code: string;
    }

    interface Map {
        // full data if this country has been retrieved with country() at least once
        [code: string]: Full | Partial;
    }
}
export type CountryInfo = CountryInfo.Full;

export namespace SubdivisionInfo {
    interface Partial {
        type: string;
        name: string;
    }
    interface Full extends Partial {
        countryName: string;
        countryCode: string;
        code: string;
        regionCode: string;
    }

    interface Map {
        // full data if this subdivision has been retrieved with subdivision() at least once
        [code: string]: Full | Partial;
    }
}
export type SubdivisionInfo = SubdivisionInfo.Full;

export function subdivision(countryCodeOrFullSubdivisionCode: string, subdivisionCodeOrName?: string): SubdivisionInfo | null;

export function country(countryCodeOrName: string): CountryInfo | null;

export const data: CountryInfo.Map;

// map of alpha 3 codes to alpha 3 codes
export const codes: {
    [alpha3: string]: string
};
"#,
    );

    let added_paths = [BiomePath::new("/node_modules/@types/iso-3166-2/index.d.ts")];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let snapshot = ModuleGraphSnapshot::new(&module_graph, &fs);
    snapshot.assert_snapshot("test_resolve_recursive_looking_country_info");
}

#[test]
fn test_resolve_recursive_looking_vfile() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/node_modules/vfile/types/index.d.ts".into(),
        r#"// TypeScript Version: 3.0

import * as Unist from 'unist'
import * as vfileMessage from 'vfile-message'

declare namespace vfile {
  /**
   * Encodings supported by the buffer class
   *
   * @remarks
   * This is a copy of the typing from Node, copied to prevent Node globals from being needed.
   * Copied from https://github.com/DefinitelyTyped/DefinitelyTyped/blob/a2bc1d868d81733a8969236655fa600bd3651a7b/types/node/globals.d.ts#L174
   */
  type BufferEncoding =
    | 'ascii'
    | 'utf8'
    | 'utf-8'
    | 'utf16le'
    | 'ucs2'
    | 'ucs-2'
    | 'base64'
    | 'latin1'
    | 'binary'
    | 'hex'

  /**
   * VFileContents can either be text, or a Buffer like structure
   * @remarks
   * This does not directly use type `Buffer, because it can also be used in a browser context.
   * Instead this leverages `Uint8Array` which is the base type for `Buffer`, and a native JavaScript construct.
   */
  type VFileContents = string | Uint8Array
  type VFileCompatible = VFile | VFileOptions | VFileContents
  interface Settings {
    [key: string]: unknown
  }
  type VFileReporter<T = Settings> = (files: VFile[], options: T) => string

  interface VFileOptions {
    contents?: VFileContents
    path?: string
    basename?: string
    stem?: string
    extname?: string
    dirname?: string
    cwd?: string
    data?: any
    [key: string]: any
  }

  interface VFile {
    /**
     * Create a new virtual file. If `options` is `string` or `Buffer`, treats it as `{contents: options}`.
     * If `options` is a `VFile`, returns it. All other options are set on the newly created `vfile`.
     *
     * Path related properties are set in the following order (least specific to most specific): `history`, `path`, `basename`, `stem`, `extname`, `dirname`.
     *
     * It’s not possible to set either `dirname` or `extname` without setting either `history`, `path`, `basename`, or `stem` as well.
     *
     * @param options If `options` is `string` or `Buffer`, treats it as `{contents: options}`. If `options` is a `VFile`, returns it. All other options are set on the newly created `vfile`.
     */
    <F extends VFile>(input?: VFileContents | F | VFileOptions): F
    /**
     * List of file-paths the file moved between.
     */
    history: string[]
    /**
     * Place to store custom information.
     * It's OK to store custom data directly on the `vfile`, moving it to `data` gives a little more privacy.
     */
    data: unknown
    /**
     * List of messages associated with the file.
     */
    messages: vfileMessage.VFileMessage[]
    /**
     * Raw value.
     */
    contents: VFileContents
    /**
     * Path of `vfile`.
     * Cannot be nullified.
     */
    path?: string
    /**
     * Path to parent directory of `vfile`.
     * Cannot be set if there's no `path` yet.
     */
    dirname?: string
    /**
     * Current name (including extension) of `vfile`.
     * Cannot contain path separators.
     * Cannot be nullified either (use `file.path = file.dirname` instead).
     */
    basename?: string
    /**
     * Name (without extension) of `vfile`.
     * Cannot be nullified, and cannot contain path separators.
     */
    stem?: string
    /**
     * Extension (with dot) of `vfile`.
     * Cannot be set if there's no `path` yet and cannot contain path separators.
     */
    extname?: string
    /**
     * Base of `path`.
     * Defaults to `process.cwd()`.
     */
    cwd: string
    /**
     * Convert contents of `vfile` to string.
     * @param encoding If `contents` is a buffer, `encoding` is used to stringify buffers (default: `'utf8'`).
     */
    toString: (encoding?: BufferEncoding) => string
    /**
     * Associates a message with the file for `reason` at `position`.
     * When an error is passed in as `reason`, copies the stack.
     * Each message has a `fatal` property which by default is set to `false` (ie. `warning`).
     * @param reason Reason for message. Uses the stack and message of the error if given.
     * @param position Place at which the message occurred in `vfile`.
     * @param ruleId Category of message.
     */
    message: (
      reason: string,
      position?: Unist.Point | Unist.Position | Unist.Node,
      ruleId?: string
    ) => vfileMessage.VFileMessage
    /**
     * Associates a fatal message with the file, then immediately throws it.
     * Note: fatal errors mean a file is no longer processable.
     * Calls `message()` internally.
     * @param reason Reason for message. Uses the stack and message of the error if given.
     * @param position Place at which the message occurred in `vfile`.
     * @param ruleId Category of message.
     */
    fail: (
      reason: string,
      position?: Unist.Point | Unist.Position | Unist.Node,
      ruleId?: string
    ) => never
    /**
     * Associates an informational message with the file, where `fatal` is set to `null`.
     * Calls `message()` internally.
     * @param reason Reason for message. Uses the stack and message of the error if given.
     * @param position Place at which the message occurred in `vfile`.
     * @param ruleId Category of message.
     */
    info: (
      reason: string,
      position?: Unist.Point | Unist.Position | Unist.Node,
      ruleId?: string
    ) => vfileMessage.VFileMessage

    [key: string]: unknown
  }
}

declare const vfile: vfile.VFile

export = vfile
"#,
    );

    let added_paths = [BiomePath::new("/node_modules/vfile/types/index.d.ts")];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let snapshot = ModuleGraphSnapshot::new(&module_graph, &fs);
    snapshot.assert_snapshot("test_resolve_recursive_looking_vfile");
}

#[test]
fn test_resolve_react_types() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/node_modules/@types/react/index.d.ts".into(),
        include_bytes!("../../biome_resolver/tests/fixtures/resolver_cases_5/node_modules/@types/react/index.d.ts")
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"import { useCallback } from "react";

        const fn = useCallback(async () => {});
        const promise = fn();
        "#,
    );

    let added_paths = [
        BiomePath::new("/node_modules/@types/react/index.d.ts"),
        BiomePath::new("/src/index.ts"),
    ];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let project_layout = ProjectLayout::default();
    project_layout.insert_node_manifest(
        "/".into(),
        PackageJson::new("frontend")
            .with_version("0.0.0")
            .with_dependencies(Dependencies(Box::new([("react".into(), "19.0.0".into())]))),
    );

    let tsconfig_json = parse_json(r#"{}"#, JsonParserOptions::default());
    project_layout
        .insert_serialized_tsconfig("/".into(), &tsconfig_json.syntax().as_send().unwrap());

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &project_layout, &added_paths, true);

    let index_module = module_graph
        .js_module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let use_callback_id = resolver
        .resolve_type_of(&Text::new_static("useCallback"), ScopeId::GLOBAL)
        .expect("useCallback variable not found");
    let use_callback_ty = resolver.resolved_type_for_id(use_callback_id);
    assert!(use_callback_ty.is_function());

    let promise_id = resolver
        .resolve_type_of(&Text::new_static("promise"), ScopeId::GLOBAL)
        .expect("promise variable not found");
    let promise_ty = resolver.resolved_type_for_id(promise_id);
    assert!(promise_ty.is_promise_instance());
}

#[test]
fn test_resolve_redis_commander_types() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/RedisCommander.d.ts".into(),
        include_bytes!("../benches/RedisCommander.d.ts"),
    );
    fs.insert(
        "/index.ts".into(),
        r#"import RedisCommander from "./RedisCommander.d.ts";
        "#,
    );

    let added_paths = [
        BiomePath::new("/RedisCommander.d.ts"),
        BiomePath::new("/index.ts"),
    ];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    // We previously had an issue with `RedisCommander.d.ts` that caused types
    // to be duplicated. We should look out in this snapshot that method
    // signatures are registered only once per signature.
    let redis_commander_module = module_graph
        .js_module_info_for_path(Utf8Path::new("/RedisCommander.d.ts"))
        .expect("module must exist");
    let num_registered_signatures = redis_commander_module
        .types()
        .iter()
        .filter(|ty| {
            matches!(
                ty,
                TypeData::Function(function)
                    if function
                        .name
                        .as_ref()
                        .is_some_and(|name| *name == "zunionstore")
            )
        })
        .count();
    assert_eq!(num_registered_signatures, 24);
}

#[test]
fn test_resolve_single_reexport() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/foo.ts".into(),
        r#"
        export function foo(): number {
            return 1;
        }
        "#,
    );
    fs.insert(
        "/src/reexport.ts".into(),
        r#"
        export * from "./foo.ts";
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"import { foo } from "./reexport.ts";

        const result = foo();
        "#,
    );

    let added_paths = [
        BiomePath::new("/src/foo.ts"),
        BiomePath::new("/src/index.ts"),
        BiomePath::new("/src/reexport.ts"),
    ];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let index_module = module_graph
        .js_module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let result_id = resolver
        .resolve_type_of(&Text::new_static("result"), ScopeId::GLOBAL)
        .expect("result variable not found");
    let ty = resolver.resolved_type_for_id(result_id);
    assert!(ty.is_number_or_number_literal());

    let snapshot = ModuleGraphSnapshot::new(module_graph.as_ref(), &fs).with_resolver(&resolver);
    snapshot.assert_snapshot("test_resolve_single_reexport");
}

#[test]
fn test_resolve_type_of_union_from_imported_module() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/node_modules/react.d.ts".into(),
        r#"
        type BogusType = false;

        export type ReactPortal = BogusType;

        export type ReactElement = BogusType;

        export type ReactNode =
            | ReactElement
            | string
            | number
            | Iterable<ReactNode>
            | ReactPortal
            | boolean
            | null
            | undefined;
        "#,
    );
    fs.insert(
        "/src/reexport.ts".into(),
        r#"export { type ReactElement, type ReactNode } from "../node_modules/react.d.ts";"#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"import { type ReactNode } from "./reexport.ts";

        const foo: ReactNode = undefined;
        const bar = foo && 1;
        "#,
    );

    let added_paths = [
        BiomePath::new("/src/index.ts"),
        BiomePath::new("/src/reexport.ts"),
        BiomePath::new("/node_modules/react.d.ts"),
    ];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let index_module = module_graph
        .js_module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let result_id = resolver
        .resolve_type_of(&Text::new_static("bar"), ScopeId::GLOBAL)
        .expect("bar variable not found");
    let ty = resolver.resolved_type_for_id(result_id);
    assert!(ty.has_variant(|ty| ty.is_null()));
    assert!(ty.has_variant(|ty| ty.is_undefined()));
    assert!(ty.has_variant(|ty| ty.is_boolean_literal(false)));
    assert!(ty.has_variant(|ty| ty.is_number_literal(0.)));
    assert!(ty.has_variant(|ty| ty.is_number_literal(1.)));
}

#[test]
fn test_resolve_multiple_reexports() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/foo.ts".into(),
        r#"
        export function foo(): number {
            return 1;
        }
        "#,
    );
    fs.insert(
        "/src/bar.ts".into(),
        r#"
        export function bar(): string {
            return "bar";
        }
        "#,
    );
    fs.insert(
        "/src/reexports.ts".into(),
        r#"
        export * from "./foo.ts";
        export * from "./bar.ts";
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"import { foo } from "./reexports.ts";
        import * as reexports from "./reexports.ts";

        const result1 = foo();
        const result2 = reexports.bar();
        "#,
    );

    let added_paths = [
        BiomePath::new("/src/foo.ts"),
        BiomePath::new("/src/bar.ts"),
        BiomePath::new("/src/index.ts"),
        BiomePath::new("/src/reexports.ts"),
    ];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let index_module = module_graph
        .js_module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let result1_id = resolver
        .resolve_type_of(&Text::new_static("result1"), ScopeId::GLOBAL)
        .expect("result1 variable not found");
    let ty = resolver.resolved_type_for_id(result1_id);
    assert!(ty.is_number_or_number_literal());

    let result2_id = resolver
        .resolve_type_of(&Text::new_static("result2"), ScopeId::GLOBAL)
        .expect("result2 variable not found");
    let ty = resolver.resolved_type_for_id(result2_id);
    assert!(ty.is_string_or_string_literal());

    let snapshot = ModuleGraphSnapshot::new(module_graph.as_ref(), &fs).with_resolver(&resolver);
    snapshot.assert_snapshot("test_resolve_multiple_reexports");
}

#[test]
fn test_resolve_export_type_referencing_imported_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/promisedResult.ts".into(),
        "export type PromisedResult = Promise<{ result: true | false }>;\n",
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"import type { PromisedResult } from "./promisedResult.ts";

        function returnPromiseResult(): PromisedResult {
            return new Promise(resolve => resolve({ result: true }));
        }

        export { returnPromiseResult };
        "#,
    );

    let added_paths = [
        BiomePath::new("/src/index.ts"),
        BiomePath::new("/src/promisedResult.ts"),
    ];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let snapshot = ModuleGraphSnapshot::new(&module_graph, &fs);
    snapshot.assert_snapshot("test_resolve_export_type_referencing_imported_type");
}

#[test]
fn test_resolve_promise_from_imported_function_returning_imported_promise_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/promisedResult.ts".into(),
        "export type PromisedResult = Promise<{ result: true | false }>;\n",
    );
    fs.insert(
        "/src/returnPromiseResult.ts".into(),
        r#"import type { PromisedResult } from "./promisedResult.ts";

        function returnPromiseResult(): PromisedResult {
            return new Promise(resolve => resolve({ result: true }));
        }

        export { returnPromiseResult };
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"import { returnPromiseResult } from "./returnPromiseResult.ts";

        const promise = returnPromiseResult();
        "#,
    );

    let added_paths = [
        BiomePath::new("/src/index.ts"),
        BiomePath::new("/src/promisedResult.ts"),
        BiomePath::new("/src/returnPromiseResult.ts"),
    ];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let index_module = module_graph
        .js_module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let resolved_id = resolver
        .resolve_type_of(&Text::new_static("promise"), ScopeId::GLOBAL)
        .expect("promise variable not found");

    let ty = resolver.resolved_type_for_id(resolved_id);
    let _ty_string = format!("{:?}", ty.deref()); // for debugging
    assert!(ty.is_promise_instance());

    let snapshot =
        ModuleGraphSnapshot::new(module_graph.as_ref(), &fs).with_resolver(resolver.as_ref());
    snapshot.assert_snapshot(
        "test_resolve_promise_from_imported_function_returning_imported_promise_type",
    );
}

#[test]
fn test_resolve_promise_from_imported_function_returning_reexported_promise_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/promisedResult.ts".into(),
        "export type PromisedResult = Promise<{ result: true | false }>;\n",
    );
    fs.insert(
        "/src/reexport.ts".into(),
        "export { PromisedResult } from \"./promisedResult.ts\";\n",
    );
    fs.insert(
        "/src/returnPromiseResult.ts".into(),
        r#"import type { PromisedResult } from "./reexport.ts";

        function returnPromiseResult(): PromisedResult {
            return new Promise(resolve => resolve({ result: true }));
        }

        export { returnPromiseResult };
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"import { returnPromiseResult } from "./returnPromiseResult.ts";

        const promise = returnPromiseResult();
        "#,
    );

    let added_paths = [
        BiomePath::new("/src/index.ts"),
        BiomePath::new("/src/promisedResult.ts"),
        BiomePath::new("/src/reexport.ts"),
        BiomePath::new("/src/returnPromiseResult.ts"),
    ];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let index_module = module_graph
        .js_module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let resolved_id = resolver
        .resolve_type_of(&Text::new_static("promise"), ScopeId::GLOBAL)
        .expect("promise variable not found");

    let ty = resolver.resolved_type_for_id(resolved_id);
    let _ty_string = format!("{:?}", ty.deref()); // for debugging
    assert!(ty.is_promise_instance());

    let snapshot =
        ModuleGraphSnapshot::new(module_graph.as_ref(), &fs).with_resolver(resolver.as_ref());
    snapshot.assert_snapshot(
        "test_resolve_promise_from_imported_function_returning_reexported_promise_type",
    );
}

#[test]
fn test_resolve_type_of_destructured_field_of_intersection_of_interfaces() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"

type FullConfiguration = InternalConfiguration & PublicConfiguration;

type ScopedMutator<Data = any, T = Data> = (key: Arguments, data?: T | Promise<T> | MutatorCallback<T>, opts?: boolean | MutatorOptions<Data, T>) => Promise<T | undefined>;

interface InternalConfiguration {
    cache: Cache;
    mutate: ScopedMutator;
}

interface PublicConfiguration {
    errorRetryInterval: number;
}

declare const useSWRConfig: () => FullConfiguration;

const { mutate } = useSWRConfig();
"#,
    );

    let added_paths = [BiomePath::new("/src/index.ts")];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let index_module = module_graph
        .js_module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let use_swr_config_id = resolver
        .resolve_type_of(&Text::new_static("useSWRConfig"), ScopeId::GLOBAL)
        .expect("mutate variable not found");
    let use_swr_config_ty = resolver.resolved_type_for_id(use_swr_config_id);
    let _use_swr_config_ty_string = format!("{:?}", use_swr_config_ty.deref()); // for debugging
    assert!(use_swr_config_ty.is_function_with_return_type(|return_ty| {
        let _return_ty_string = format!("{:?}", return_ty.deref()); // for debugging
        return_ty.is_interface()
    }));

    let mutate_id = resolver
        .resolve_type_of(&Text::new_static("mutate"), ScopeId::GLOBAL)
        .expect("mutate variable not found");
    let mutate_ty = resolver.resolved_type_for_id(mutate_id);
    let _mutate_ty_string = format!("{:?}", mutate_ty.deref()); // for debugging
    assert!(
        mutate_ty.is_instance_of(|instance_ty| instance_ty.is_function_with_return_type(
            |return_ty| {
                let _return_ty_string = format!("{:?}", return_ty.deref()); // for debugging
                return_ty.is_promise_instance()
            }
        ))
    );

    let snapshot =
        ModuleGraphSnapshot::new(module_graph.as_ref(), &fs).with_resolver(resolver.as_ref());
    snapshot
        .assert_snapshot("test_resolve_type_of_destructured_field_of_intersection_of_interfaces");
}

#[test]
fn test_resolve_type_of_intersection_of_interfaces() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"interface Foo {
    foo(): string;
}

interface Bar {
    foo(): number;
    bar(): boolean;
}

type Intersection = Foo & Bar;"#,
    );

    let added_paths = [BiomePath::new("/src/index.ts")];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let index_module = module_graph
        .js_module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let intersection_id = resolver
        .resolve_type_of(&Text::new_static("Intersection"), ScopeId::GLOBAL)
        .expect("Intersection type not found");
    let intersection_ty = resolver.resolved_type_for_id(intersection_id);
    let _intersection_ty = format!("{:?}", intersection_ty.deref()); // for debugging
    assert!(intersection_ty.is_interface());

    let snapshot =
        ModuleGraphSnapshot::new(module_graph.as_ref(), &fs).with_resolver(resolver.as_ref());
    snapshot.assert_snapshot("test_resolve_type_of_intersection_of_interfaces");
}

#[test]
fn test_resolve_swr_types() {
    let fixtures_path = get_fixtures_path();

    let fs = OsFileSystem::new(fixtures_path.clone());
    let project_layout = ProjectLayout::default();
    project_layout.insert_node_manifest(format!("{fixtures_path}/frontend").into(), {
        let path = Utf8PathBuf::from(format!("{fixtures_path}/frontend/package.json"));
        deserialize_from_json_str::<PackageJson>(
            &fs.read_file_from_path(&path)
                .expect("package.json must be readable"),
            JsonParserOptions::default(),
            "package.json",
        )
        .into_deserialized()
        .expect("package.json must parse")
    });
    // Bloody symlinks...
    let swr_path = {
        let swr_path = format!("{fixtures_path}/node_modules/swr");
        let symlink = read_link(swr_path).expect("cannot read symlink");
        let symlink = Utf8PathBuf::from_path_buf(symlink).expect("non-UTF8 path");
        normalize_path(Utf8Path::new(&format!(
            "{fixtures_path}/node_modules/{symlink}"
        )))
    };
    project_layout.insert_node_manifest(swr_path.clone(), {
        let path = Utf8PathBuf::from(format!("{swr_path}/package.json"));
        deserialize_from_json_str::<PackageJson>(
            &fs.read_file_from_path(&path)
                .expect("package.json must be readable"),
            JsonParserOptions::default(),
            "package.json",
        )
        .into_deserialized()
        .expect("package.json must parse")
    });

    let mut added_paths = vec![BiomePath::new(format!(
        "{fixtures_path}/frontend/src/index.ts"
    ))];
    for path in find_files_recursively_in_directory(&swr_path, |path| {
        path.extension().is_some_and(|ext| ext != "json")
    }) {
        added_paths.push(BiomePath::new(path));
    }
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &project_layout, &added_paths, true);

    let index_module = module_graph
        .js_module_info_for_path(Utf8Path::new(&format!(
            "{fixtures_path}/frontend/src/index.ts"
        )))
        .expect("module must exist");
    assert_eq!(
        index_module.static_import_paths.get("swr"),
        Some(&JsImportPath {
            resolved_path: ResolvedPath::from_path(format!("{swr_path}/dist/index/index.d.mts")),
            phase: JsImportPhase::Default,
        })
    );

    let swr_index_module = module_graph
        .js_module_info_for_path(Utf8Path::new(&format!("{swr_path}/dist/index/index.d.mts")))
        .expect("module must exist");
    assert_eq!(
        swr_index_module
            .static_import_paths
            .get("../_internal/index.mjs"),
        Some(&JsImportPath {
            resolved_path: ResolvedPath::from_path(format!(
                "{swr_path}/dist/_internal/index.d.mts"
            )),
            phase: JsImportPhase::Default,
        })
    );

    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let mutate_id = resolver
        .resolve_type_of(&Text::new_static("mutate"), ScopeId::GLOBAL)
        .expect("mutate variable not found");

    let mutate_ty = resolver.resolved_type_for_id(mutate_id);
    let _mutate_ty_string = format!("{:?}", mutate_ty.deref()); // for debugging
    assert!(mutate_ty.is_interface_with_member(|member| member.kind().is_call_signature()));

    let mutate_result_id = resolver
        .resolve_type_of(&Text::new_static("mutateResult"), ScopeId::GLOBAL)
        .expect("mutateResult variable not found");

    let mutate_result_ty = resolver.resolved_type_for_id(mutate_result_id);
    let _mutate_result_ty_string = format!("{:?}", mutate_result_ty.deref()); // for debugging
    assert!(mutate_result_ty.is_promise_instance());
}

#[test]
fn test_widening_via_assignment() {
    let fs = MemoryFileSystem::default();

    fs.insert(
        "index.ts".into(),
        r#"
let hey = false;
function f() {
    hey = true;
}"#,
    );

    let added_paths = [BiomePath::new("index.ts")];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());

    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let index_module = module_graph
        .js_module_info_for_path(Utf8Path::new("index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let snapshot = ModuleGraphSnapshot::new(&module_graph, &fs).with_resolver(resolver.as_ref());

    snapshot.assert_snapshot("test_widening_via_assignment");
}

#[test]
fn test_widening_via_assignment_multiple_values() {
    let fs = MemoryFileSystem::default();

    fs.insert(
        "index.ts".into(),
        r#"
let hey = undefined;
function f() {
    hey = "some string";
}
function g() {
    hey = 123;
}
"#,
    );

    let added_paths = [BiomePath::new("index.ts")];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());

    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let index_module = module_graph
        .js_module_info_for_path(Utf8Path::new("index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let snapshot = ModuleGraphSnapshot::new(&module_graph, &fs).with_resolver(resolver.as_ref());

    snapshot.assert_snapshot("test_widening_via_assignment_multiple_values");
}

// ============================================================================
// Regression tests for false positives fixed in biome#9143
// ============================================================================

/// `node:fs`, `node:path`, etc. must resolve to `ResolveError::NodeBuiltIn`,
/// not to any file path error. The lint rule silently accepts this error kind.
#[test]
fn test_node_builtin_imports_resolve_to_builtin_error() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import fs from "node:fs";
            import path from "node:path";
            import { fileURLToPath } from "node:url";
        "#,
    );

    let project_layout = ProjectLayout::default();
    project_layout.insert_node_manifest(
        "/".into(),
        PackageJson::new("test-pkg").with_version("0.0.0"),
    );

    let added_paths = [BiomePath::new("/src/index.ts")];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &project_layout, &added_paths, true);

    let data = module_graph.data();
    let module = data
        .get(Utf8Path::new("/src/index.ts"))
        .unwrap()
        .as_js_module_info()
        .unwrap();

    // All three `node:*` specifiers must be present in static_import_paths.
    for specifier in ["node:fs", "node:path", "node:url"] {
        let import_path = module
            .static_import_paths
            .get(specifier)
            .unwrap_or_else(|| panic!("specifier `{specifier}` not found in static_import_paths"));

        assert_eq!(
            import_path.resolved_path.error(),
            Some(&ResolveError::NodeBuiltIn),
            "`{specifier}` should resolve to ResolveError::NodeBuiltIn, got: {:?}",
            import_path.resolved_path.as_deref()
        );
    }
}

/// A package that uses `"typings"` (instead of `"types"`) to declare its type
/// entry point must still be resolved correctly. This regression covers the
/// `lucide-react` false-positive reported in biome#9143.
#[test]
fn test_package_typings_field_resolution() {
    let fs = MemoryFileSystem::default();

    // Package entry point — a .d.ts reached via "typings".
    fs.insert(
        "/node_modules/my-icons/dist/index.d.ts".into(),
        r#"export declare function Icon(): void;"#,
    );

    // The package's package.json uses "typings", not "types".
    fs.insert(
        "/src/index.ts".into(),
        r#"import { Icon } from "my-icons";"#,
    );

    let project_layout = ProjectLayout::default();
    project_layout.insert_node_manifest(
        "/".into(),
        PackageJson::new("test-pkg")
            .with_version("0.0.0")
            .with_dependencies(Dependencies(Box::new([(
                "my-icons".into(),
                "1.0.0".into(),
            )]))),
    );

    // Insert the package manifest using `with_typings` once that API exists,
    // or manually build a PackageJson that has `typings` set.
    // For now we build it from raw JSON — this also exercises the
    // `"typings"` → `"types"` alias in PackageJson deserialization.
    let pkg_json_str = r#"{"name":"my-icons","version":"1.0.0","typings":"./dist/index.d.ts"}"#;
    let pkg_json = biome_deserialize::json::deserialize_from_json_str::<PackageJson>(
        pkg_json_str,
        biome_json_parser::JsonParserOptions::default(),
        "package.json",
    )
    .into_deserialized()
    .expect("package.json must parse");

    project_layout.insert_node_manifest("/node_modules/my-icons".into(), pkg_json);

    let added_paths = [
        BiomePath::new("/src/index.ts"),
        BiomePath::new("/node_modules/my-icons/dist/index.d.ts"),
    ];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &project_layout, &added_paths, true);

    let data = module_graph.data();
    let module = data
        .get(Utf8Path::new("/src/index.ts"))
        .unwrap()
        .as_js_module_info()
        .unwrap();

    // The import must resolve to the .d.ts file, not an error.
    let import_path = module
        .static_imports
        .get("Icon")
        .expect("`Icon` must be in static_imports");
    assert_eq!(
        import_path.resolved_path.as_path(),
        Some(Utf8Path::new("/node_modules/my-icons/dist/index.d.ts")),
        "Package with `typings` field must resolve to its declared entry point"
    );
}

/// `export {{ x as y }} from "./mod"` re-exports `x` under the name `y`.
/// When a consumer imports `y`, the resolver must look up `x` (the source-side
/// name) in the target module — not `y` (the alias). This regression covers
/// the `vitest` / `@tanstack/react-query` false-positives in biome#9143.
#[test]
fn test_aliased_named_reexport_is_found_by_alias() {
    let fs = MemoryFileSystem::default();

    // Source module: exports the function under its original name.
    fs.insert(
        "/src/source.ts".into(),
        r#"export function originalName() {}"#,
    );

    // Barrel: re-exports `originalName` under a different name `renamedSymbol`.
    fs.insert(
        "/src/barrel.ts".into(),
        r#"export { originalName as renamedSymbol } from "./source.ts";"#,
    );

    // Consumer: imports using the public alias name.
    fs.insert(
        "/src/index.ts".into(),
        r#"import { renamedSymbol } from "./barrel.ts";"#,
    );

    let project_layout = ProjectLayout::default();
    project_layout.insert_node_manifest(
        "/".into(),
        PackageJson::new("test-pkg").with_version("0.0.0"),
    );

    let added_paths = [
        BiomePath::new("/src/source.ts"),
        BiomePath::new("/src/barrel.ts"),
        BiomePath::new("/src/index.ts"),
    ];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &project_layout, &added_paths, true);

    // `barrel.ts` must expose `renamedSymbol` as an own export (resolved from
    // the re-export chain to a binding in `source.ts`).
    let data = module_graph.data();
    let barrel = data
        .get(Utf8Path::new("/src/barrel.ts"))
        .unwrap()
        .as_js_module_info()
        .unwrap();

    let found = barrel.find_js_exported_symbol(module_graph.as_ref(), "renamedSymbol");
    assert!(
        found.is_some(),
        "`renamedSymbol` must be found via the aliased re-export chain; got None"
    );

    // `originalName` must NOT be visible under the barrel's public API.
    let not_found = barrel.find_js_exported_symbol(module_graph.as_ref(), "originalName");
    assert!(
        not_found.is_none(),
        "`originalName` must not be directly exported from the barrel"
    );
}

/// `export * as Ns from "./mod"` creates a namespace object and exports it
/// under the name `Ns`. `Ns` must be resolved as `JsOwnExport::Namespace`
/// (an own export of the barrel module, not a forwarding re-export).
/// This regression covers the `@base-ui/react` false-positive in biome#9143.
#[test]
fn test_namespace_reexport_is_own_export() {
    let fs = MemoryFileSystem::default();

    // Source module with some exports.
    fs.insert(
        "/src/source.ts".into(),
        r#"
            export function alpha() {}
            export function beta() {}
        "#,
    );

    // Barrel: re-exports entire namespace of `source.ts` under `MyNs`.
    fs.insert(
        "/src/barrel.ts".into(),
        r#"export * as MyNs from "./source.ts";"#,
    );

    let project_layout = ProjectLayout::default();
    project_layout.insert_node_manifest(
        "/".into(),
        PackageJson::new("test-pkg").with_version("0.0.0"),
    );

    let added_paths = [
        BiomePath::new("/src/source.ts"),
        BiomePath::new("/src/barrel.ts"),
    ];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &project_layout, &added_paths, true);

    let data = module_graph.data();
    let barrel = data
        .get(Utf8Path::new("/src/barrel.ts"))
        .unwrap()
        .as_js_module_info()
        .unwrap();

    // `MyNs` must be stored as an own export (namespace), not as a forwarding
    // re-export. Only then will `find_js_exported_symbol` return `Some`.
    assert_eq!(
        barrel.exports.get(&Text::new_static("MyNs")),
        Some(&JsExport::Own(JsOwnExport::Namespace(JsReexport {
            import: JsImport {
                specifier: "./source.ts".into(),
                resolved_path: ResolvedPath::from_path("/src/source.ts"),
                symbol: ImportSymbol::All,
            },
            jsdoc_comment: None,
        }))),
        "`export * as MyNs` must produce JsExport::Own(JsOwnExport::Namespace(JsReexport {{ .. }}))"
    );

    // Confirm `find_js_exported_symbol` returns `Some` as the lint rule sees it.
    let found = barrel.find_js_exported_symbol(module_graph.as_ref(), "MyNs");
    assert!(
        found.is_some(),
        "`MyNs` must be found by find_js_exported_symbol"
    );
}

/// `export * as Ns from "./mod"` should also support type inference: when
/// `index.ts` imports `{ MyNs }` from a barrel that uses `export * as MyNs`,
/// calling `MyNs.alpha()` must resolve to the return type of `alpha` in
/// the source module. This verifies the `JsOwnExport::Namespace(JsReexport)`
/// variant drives the correct `TypeData::ImportNamespace` path.
#[test]
fn test_namespace_reexport_type_inference() {
    let fs = MemoryFileSystem::default();

    // Source module with a typed export.
    fs.insert(
        "/src/source.ts".into(),
        r#"
            /**
            * @description
            * I am a jsdoc
            */
            export function alpha(): number {
                return 1;
            }
        "#,
    );

    // Barrel: re-exports entire source namespace under `MyNs`.
    fs.insert(
        "/src/barrel.ts".into(),
        r#"export * as MyNs from "./source.ts";"#,
    );

    // Consumer: imports the namespace and calls a member.
    fs.insert(
        "/src/index.ts".into(),
        r#"import { MyNs } from "./barrel.ts";

        const result = MyNs.alpha();
        "#,
    );

    let added_paths = [
        BiomePath::new("/src/source.ts"),
        BiomePath::new("/src/barrel.ts"),
        BiomePath::new("/src/index.ts"),
    ];
    let added_paths = get_added_js_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);

    let index_module = module_graph
        .js_module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let result_id = resolver
        .resolve_type_of(&Text::new_static("result"), ScopeId::GLOBAL)
        .expect("result variable not found");
    let result_ty = resolver.resolved_type_for_id(result_id);
    assert!(
        result_ty.is_number_or_number_literal(),
        "expected `MyNs.alpha()` to resolve to number, got: {result_ty:?}"
    );

    let snapshot = ModuleGraphSnapshot::new(module_graph.as_ref(), &fs).with_resolver(&resolver);
    snapshot.assert_snapshot("test_namespace_reexport_type_inference");
}

/// Verifies that a JSX file that imports a CSS file shows:
/// - the CSS import edge in `static_import_paths`
/// - `referenced_classes` populated from `className="..."` attributes
/// - the CSS module info showing the defined classes
#[test]
fn test_jsx_imports_css_file() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/styles.css".into(),
        r#"
.button {
    color: red;
}
.header {
    font-size: 24px;
}
"#,
    );
    fs.insert(
        "/src/App.jsx".into(),
        r#"
import "./styles.css";

export function App() {
    return <div className="button header">Hello</div>;
}
"#,
    );

    let css_paths = [BiomePath::new("/src/styles.css")];
    let css_roots = get_css_added_paths(&fs, &css_paths);

    let js_paths = [BiomePath::new("/src/App.jsx")];
    let js_roots = get_added_js_paths(&fs, &js_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_css_paths(&fs, &ProjectLayout::default(), &css_roots, None);
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &js_roots, false);

    // Verify the JS module info has the CSS import edge resolved
    let app_info = module_graph
        .js_module_info_for_path(Utf8Path::new("/src/App.jsx"))
        .expect("App.jsx must be in module graph");
    assert!(
        app_info
            .static_import_paths
            .values()
            .any(|p| p.as_path() == Some(Utf8Path::new("/src/styles.css"))),
        "App.jsx must have a resolved import edge to styles.css"
    );

    // Verify referenced_classes contains the JSX className values
    assert!(
        app_info
            .referenced_classes
            .iter()
            .any(|r| r.matches("button")),
        "App.jsx must reference class 'button'"
    );
    assert!(
        app_info
            .referenced_classes
            .iter()
            .any(|r| r.matches("header")),
        "App.jsx must reference class 'header'"
    );

    // Verify the CSS module info has the defined classes
    let css_info = module_graph
        .css_module_info_for_path(Utf8Path::new("/src/styles.css"))
        .expect("styles.css must be in module graph");
    assert!(
        css_info.classes.contains("button"),
        "styles.css must define class 'button'"
    );
    assert!(
        css_info.classes.contains("header"),
        "styles.css must define class 'header'"
    );

    // Snapshot both files to capture the full module info.
    let snapshot = ModuleGraphSnapshot::new(&module_graph, &fs);
    snapshot.assert_snapshot("test_jsx_imports_css_file");
}

/// Verifies that `is_class_referenced_by_importers` returns `true` for a class
/// that is referenced in the importing JSX file, and `false` for a class that
/// is defined in the CSS but never used.
#[test]
fn test_css_classes_referenced_by_jsx() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/styles.css".into(),
        r#"
.used { color: blue; }
.unused { color: green; }
"#,
    );
    fs.insert(
        "/src/Component.jsx".into(),
        r#"
import "./styles.css";

export function Component() {
    return <div className="used" />;
}
"#,
    );

    let css_paths = [BiomePath::new("/src/styles.css")];
    let css_roots = get_css_added_paths(&fs, &css_paths);
    let js_paths = [BiomePath::new("/src/Component.jsx")];
    let js_roots = get_added_js_paths(&fs, &js_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_css_paths(&fs, &ProjectLayout::default(), &css_roots, None);
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &js_roots, false);

    assert!(
        module_graph.is_class_referenced_by_importers(Utf8Path::new("/src/styles.css"), "used"),
        "'used' class should be referenced by Component.jsx"
    );
    assert!(
        !module_graph.is_class_referenced_by_importers(Utf8Path::new("/src/styles.css"), "unused"),
        "'unused' class should not be referenced by any importer"
    );
}

/// Verifies transitive CSS import chain:
/// `App.jsx` → `theme.css` → `base.css`
///
/// `transitive_importers_of(base.css)` must include `App.jsx`, and
/// `is_class_referenced_by_importers(base.css, "base")` must return `true`.
#[test]
fn test_transitive_css_import_chain() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/base.css".into(),
        r#"
.base { margin: 0; }
.orphan { padding: 0; }
"#,
    );
    fs.insert(
        "/src/theme.css".into(),
        r#"
@import "./base.css";
.theme { color: purple; }
"#,
    );
    fs.insert(
        "/src/App.jsx".into(),
        r#"
import "./theme.css";

export function App() {
    return <div className="base theme" />;
}
"#,
    );

    let css_paths = [
        BiomePath::new("/src/base.css"),
        BiomePath::new("/src/theme.css"),
    ];
    let css_roots = get_css_added_paths(&fs, &css_paths);
    let js_paths = [BiomePath::new("/src/App.jsx")];
    let js_roots = get_added_js_paths(&fs, &js_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_css_paths(&fs, &ProjectLayout::default(), &css_roots, None);
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &js_roots, false);

    // CSS class consumers of base.css must include App.jsx (via theme.css).
    let importers = module_graph.transitive_importers_of(Utf8Path::new("/src/base.css"));
    assert!(
        importers
            .iter()
            .any(|p| p.as_path() == Utf8Path::new("/src/App.jsx")),
        "App.jsx must be a transitive importer of base.css; got: {importers:?}"
    );

    // The 'base' class is used by App.jsx, so it should be considered referenced.
    assert!(
        module_graph.is_class_referenced_by_importers(Utf8Path::new("/src/base.css"), "base"),
        "'base' class must be referenced transitively"
    );

    // The 'orphan' class is never used anywhere.
    assert!(
        !module_graph.is_class_referenced_by_importers(Utf8Path::new("/src/base.css"), "orphan"),
        "'orphan' class must not be referenced"
    );
}

/// Verifies the real-world use case: single entry point importing main CSS,
/// which imports multiple component CSS files.
///
/// App.tsx → app.css → components.css (defines .button)
/// App.tsx uses className="button"
///
/// This test ensures that classes defined in deeply nested CSS files are
/// correctly detected as used when referenced by the entry point.
#[test]
fn test_single_entry_point_with_nested_css_imports() {
    let fs = MemoryFileSystem::default();

    // Deeply nested CSS file defining component classes
    fs.insert(
        "/src/styles/components.css".into(),
        r#"
.button { background: blue; }
.card { border: 1px solid; }
.unused-component-class { color: red; }
"#,
    );

    // Utility CSS file
    fs.insert(
        "/src/styles/utils.css".into(),
        r#"
.flex { display: flex; }
.grid { display: grid; }
"#,
    );

    // Main app CSS that imports other CSS files
    fs.insert(
        "/src/app.css".into(),
        r#"
@import "./styles/components.css";
@import "./styles/utils.css";

.app-container { width: 100%; }
"#,
    );

    // Entry point that only imports app.css
    fs.insert(
        "/src/App.tsx".into(),
        r#"
import "./app.css";

export function App() {
    return (
        <div className="app-container flex">
            <button className="button">Click</button>
            <div className="card">Content</div>
        </div>
    );
}
"#,
    );

    let css_paths = [
        BiomePath::new("/src/styles/components.css"),
        BiomePath::new("/src/styles/utils.css"),
        BiomePath::new("/src/app.css"),
    ];
    let css_roots = get_css_added_paths(&fs, &css_paths);
    let js_paths = [BiomePath::new("/src/App.tsx")];
    let js_roots = get_added_js_paths(&fs, &js_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_css_paths(&fs, &ProjectLayout::default(), &css_roots, None);
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &js_roots, false);

    // App.tsx should be found as consumer of components.css (via app.css)
    let consumers =
        module_graph.transitive_importers_of(Utf8Path::new("/src/styles/components.css"));
    assert!(
        consumers
            .iter()
            .any(|p| p.as_path() == Utf8Path::new("/src/App.tsx")),
        "App.tsx must be a consumer of components.css; got: {consumers:?}"
    );

    // Classes used in App.tsx should be detected even from nested CSS
    assert!(
        module_graph.is_class_referenced_by_importers(
            Utf8Path::new("/src/styles/components.css"),
            "button"
        ),
        "'button' class from components.css should be detected as used"
    );

    assert!(
        module_graph
            .is_class_referenced_by_importers(Utf8Path::new("/src/styles/components.css"), "card"),
        "'card' class from components.css should be detected as used"
    );

    // Unused class should not be detected
    assert!(
        !module_graph.is_class_referenced_by_importers(
            Utf8Path::new("/src/styles/components.css"),
            "unused-component-class"
        ),
        "'unused-component-class' should be detected as unused"
    );

    // Utils classes should also work
    assert!(
        module_graph
            .is_class_referenced_by_importers(Utf8Path::new("/src/styles/utils.css"), "flex"),
        "'flex' class from utils.css should be detected as used"
    );

    assert!(
        !module_graph
            .is_class_referenced_by_importers(Utf8Path::new("/src/styles/utils.css"), "grid"),
        "'grid' class from utils.css should be detected as unused"
    );
}

/// Verifies that multiple entry points can all access nested CSS classes.
///
/// components.css → app.css → App.tsx (uses .button)
/// components.css → app.css → Dashboard.tsx (uses .card)
///
/// Both entry points import the same main CSS file.
#[test]
fn test_multiple_entry_points_sharing_css() {
    let fs = MemoryFileSystem::default();

    fs.insert(
        "/src/components.css".into(),
        r#"
.button { background: blue; }
.card { border: 1px solid; }
.modal { position: fixed; }
"#,
    );

    fs.insert(
        "/src/app.css".into(),
        r#"
@import "./components.css";
"#,
    );

    fs.insert(
        "/src/App.tsx".into(),
        r#"
import "./app.css";

export function App() {
    return <button className="button">Click</button>;
}
"#,
    );

    fs.insert(
        "/src/Dashboard.tsx".into(),
        r#"
import "./app.css";

export function Dashboard() {
    return <div className="card">Dashboard</div>;
}
"#,
    );

    let css_paths = [
        BiomePath::new("/src/components.css"),
        BiomePath::new("/src/app.css"),
    ];
    let css_roots = get_css_added_paths(&fs, &css_paths);
    let js_paths = [
        BiomePath::new("/src/App.tsx"),
        BiomePath::new("/src/Dashboard.tsx"),
    ];
    let js_roots = get_added_js_paths(&fs, &js_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_css_paths(&fs, &ProjectLayout::default(), &css_roots, None);
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &js_roots, false);

    // Both entry points should be found as consumers
    let consumers = module_graph.transitive_importers_of(Utf8Path::new("/src/components.css"));
    assert_eq!(
        consumers.len(),
        2,
        "Expected 2 consumers of components.css; got: {consumers:?}"
    );
    assert!(
        consumers
            .iter()
            .any(|p| p.as_path() == Utf8Path::new("/src/App.tsx")),
        "App.tsx should be a consumer"
    );
    assert!(
        consumers
            .iter()
            .any(|p| p.as_path() == Utf8Path::new("/src/Dashboard.tsx")),
        "Dashboard.tsx should be a consumer"
    );

    // button used in App.tsx
    assert!(
        module_graph
            .is_class_referenced_by_importers(Utf8Path::new("/src/components.css"), "button"),
        "'button' should be detected as used"
    );

    // card used in Dashboard.tsx
    assert!(
        module_graph.is_class_referenced_by_importers(Utf8Path::new("/src/components.css"), "card"),
        "'card' should be detected as used"
    );

    // modal not used anywhere
    assert!(
        !module_graph
            .is_class_referenced_by_importers(Utf8Path::new("/src/components.css"), "modal"),
        "'modal' should be detected as unused"
    );
}

/// Verifies deep nesting: 3 levels of CSS imports.
///
/// App.tsx → main.css → theme.css → base.css (defines .primary)
#[test]
fn test_deeply_nested_css_import_chain() {
    let fs = MemoryFileSystem::default();

    fs.insert(
        "/src/base.css".into(),
        r#"
.primary { color: blue; }
.secondary { color: gray; }
"#,
    );

    fs.insert(
        "/src/theme.css".into(),
        r#"
@import "./base.css";
"#,
    );

    fs.insert(
        "/src/main.css".into(),
        r#"
@import "./theme.css";
"#,
    );

    fs.insert(
        "/src/App.tsx".into(),
        r#"
import "./main.css";

export function App() {
    return <button className="primary">Primary Button</button>;
}
"#,
    );

    let css_paths = [
        BiomePath::new("/src/base.css"),
        BiomePath::new("/src/theme.css"),
        BiomePath::new("/src/main.css"),
    ];
    let css_roots = get_css_added_paths(&fs, &css_paths);
    let js_paths = [BiomePath::new("/src/App.tsx")];
    let js_roots = get_added_js_paths(&fs, &js_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_css_paths(&fs, &ProjectLayout::default(), &css_roots, None);
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &js_roots, false);

    // App.tsx should be found even for deeply nested base.css
    let consumers = module_graph.transitive_importers_of(Utf8Path::new("/src/base.css"));
    assert!(
        consumers
            .iter()
            .any(|p| p.as_path() == Utf8Path::new("/src/App.tsx")),
        "App.tsx should be a consumer of base.css through 3-level import chain; got: {consumers:?}"
    );

    // primary used in App.tsx
    assert!(
        module_graph.is_class_referenced_by_importers(Utf8Path::new("/src/base.css"), "primary"),
        "'primary' from deeply nested base.css should be detected as used"
    );

    // secondary not used
    assert!(
        !module_graph.is_class_referenced_by_importers(Utf8Path::new("/src/base.css"), "secondary"),
        "'secondary' should be detected as unused"
    );
}

/// Tests `collect_available_classes_for_js_file` for a JSX file that directly
/// imports CSS.
#[test]
fn test_collect_available_classes_for_js_file() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/styles.css".into(),
        r#"
.button { color: blue; }
.card { border: 1px solid; }
"#,
    );
    fs.insert(
        "/src/App.jsx".into(),
        r#"
import "./styles.css";

export function App() {
    return <div className="button">Hello</div>;
}
"#,
    );

    let css_paths = [BiomePath::new("/src/styles.css")];
    let css_roots = get_css_added_paths(&fs, &css_paths);
    let js_paths = [BiomePath::new("/src/App.jsx")];
    let js_roots = get_added_js_paths(&fs, &js_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_css_paths(&fs, &ProjectLayout::default(), &css_roots, None);
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &js_roots, false);

    let (classes, traversal) =
        module_graph.collect_available_classes_for_js_file(Utf8Path::new("/src/App.jsx"));

    // Should find both CSS classes
    assert!(classes.contains("button"), "Should find .button class");
    assert!(classes.contains("card"), "Should find .card class");

    // Should have one traversal step (direct import)
    assert_eq!(traversal.len(), 1, "Should have 1 CSS file in traversal");
    assert_eq!(
        traversal[0].css_path.as_str().replace('\\', "/"),
        "/src/styles.css",
        "Should show styles.css in traversal"
    );
}

/// Tests `collect_available_classes_for_js_file` with multiple CSS imports.
#[test]
fn test_collect_available_classes_for_js_file_multiple_css() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/buttons.css".into(),
        r#"
.btn { padding: 0.5rem; }
.btn-primary { background: blue; }
"#,
    );
    fs.insert(
        "/src/layout.css".into(),
        r#"
.container { width: 100%; }
.flex { display: flex; }
"#,
    );
    fs.insert(
        "/src/App.jsx".into(),
        r#"
import "./buttons.css";
import "./layout.css";

export function App() {
    return <div className="container"><button className="btn">Click</button></div>;
}
"#,
    );

    let css_paths = [
        BiomePath::new("/src/buttons.css"),
        BiomePath::new("/src/layout.css"),
    ];
    let css_roots = get_css_added_paths(&fs, &css_paths);
    let js_paths = [BiomePath::new("/src/App.jsx")];
    let js_roots = get_added_js_paths(&fs, &js_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_css_paths(&fs, &ProjectLayout::default(), &css_roots, None);
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &js_roots, false);

    let (classes, traversal) =
        module_graph.collect_available_classes_for_js_file(Utf8Path::new("/src/App.jsx"));

    // Should find all CSS classes from both imports
    assert!(classes.contains("btn"), "Should find .btn class");
    assert!(
        classes.contains("btn-primary"),
        "Should find .btn-primary class"
    );
    assert!(
        classes.contains("container"),
        "Should find .container class"
    );
    assert!(classes.contains("flex"), "Should find .flex class");

    // Should have two CSS files in traversal
    assert_eq!(
        traversal.len(),
        2,
        "Should have 2 CSS files in traversal path"
    );
}

fn find_files_recursively_in_directory(
    directory: &Utf8Path,
    predicate: impl Fn(&Utf8Path) -> bool,
) -> Vec<Utf8PathBuf> {
    WalkDir::new(directory.as_std_path())
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(|entry| Utf8Path::from_path(entry.path()).map(Utf8Path::to_path_buf))
        .filter(|path| predicate(path))
        .collect()
}

// #region HTML module graph + style applicability tests

/// Parses a CSS snippet with the given `CssFileSource` and wraps it as an
/// [`HtmlEmbeddedContent::Css`] ready for [`ModuleGraph::update_graph_for_html_paths`].
fn parse_embedded_css(src: &str, file_source: CssFileSource) -> HtmlEmbeddedContent {
    // Mirror the workspace server: enable CSS modules parsing for embedded CSS
    // in framework files (Vue → Vue dialect; Svelte/Astro → Classic).
    let css_modules_kind = match file_source.as_embedding_kind() {
        EmbeddingKind::Html(EmbeddingHtmlKind::Vue { .. }) => CssModulesKind::Vue,
        EmbeddingKind::Html(EmbeddingHtmlKind::Astro { .. } | EmbeddingHtmlKind::Svelte { .. }) => {
            CssModulesKind::Classic
        }
        _ => CssModulesKind::None,
    };
    let options = CssParserOptions {
        css_modules: css_modules_kind,
        ..Default::default()
    };
    let parsed = biome_css_parser::parse_css(src, file_source, options);
    HtmlEmbeddedContent::Css(parsed.tree(), file_source)
}

/// Parses an HTML snippet and returns a `HtmlRoot`.
fn parse_html_src(src: &str, file_source: HtmlFileSource) -> biome_html_syntax::HtmlRoot {
    let parsed = biome_html_parser::parse_html(src, HtmlParserOptions::from(&file_source));
    parsed.tree()
}

/// Returns a `CssFileSource` for a plain HTML `<style>` block (always Global).
fn html_css_source() -> CssFileSource {
    CssFileSource::css().with_embedding_kind(EmbeddingKind::Html(EmbeddingHtmlKind::Html))
}

/// Returns a `CssFileSource` for a Vue `<style>` (unscoped → Global).
fn vue_global_css_source() -> CssFileSource {
    CssFileSource::css().with_embedding_kind(EmbeddingKind::Html(EmbeddingHtmlKind::Vue {
        applicability: EmbeddingStyleApplicability::Global,
    }))
}

/// Returns a `CssFileSource` for a Vue `<style scoped>` (Local).
fn vue_scoped_css_source() -> CssFileSource {
    CssFileSource::css().with_embedding_kind(EmbeddingKind::Html(EmbeddingHtmlKind::Vue {
        applicability: EmbeddingStyleApplicability::Local,
    }))
}

/// Returns a `CssFileSource` for an Astro `<style>` (default → Local).
fn astro_local_css_source() -> CssFileSource {
    CssFileSource::css().with_embedding_kind(EmbeddingKind::Html(EmbeddingHtmlKind::Astro {
        applicability: EmbeddingStyleApplicability::Local,
    }))
}

/// Returns a `CssFileSource` for an Astro `<style is:global>` (Global).
fn astro_global_css_source() -> CssFileSource {
    CssFileSource::css().with_embedding_kind(EmbeddingKind::Html(EmbeddingHtmlKind::Astro {
        applicability: EmbeddingStyleApplicability::Global,
    }))
}

/// Returns a `CssFileSource` for a Svelte `<style>` (default → Local).
fn svelte_local_css_source() -> CssFileSource {
    CssFileSource::css().with_embedding_kind(EmbeddingKind::Html(EmbeddingHtmlKind::Svelte {
        applicability: EmbeddingStyleApplicability::Local,
    }))
}

/// Plain HTML `<style>` → all classes are Global and visible in the traversal.
#[test]
fn test_html_inline_style_classes_are_global() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.html".into(), r#"<div class="card">Hello</div>"#);

    let html_path = BiomePath::new("/src/index.html");
    let html_root = parse_html_src(r#"<div class="card">Hello</div>"#, HtmlFileSource::html());
    let css = parse_embedded_css(".card { color: red; }", html_css_source());

    let module_graph = ModuleGraph::default();
    let layout = ProjectLayout::default();
    module_graph.update_graph_for_html_paths(&fs, &layout, &[(&html_path, html_root, vec![css])]);

    let html_info = module_graph
        .html_module_info_for_path(Utf8Path::new("/src/index.html"))
        .expect("HTML module must exist");

    assert_eq!(html_info.style_classes.len(), 1, "should have one class");
    let def = html_info.style_classes.iter().next().unwrap();
    assert_eq!(def.name.text(), "card");
    assert_eq!(
        def.applicability,
        EmbeddingStyleApplicability::Unknown,
        "HTML inline styles are always Unknown"
    );

    // The traversal must yield the class.
    let found = module_graph
        .traverse_import_tree_for_html_classes(Utf8Path::new("/src/index.html"))
        .any(|step| step.css_classes.iter().any(|c| c.text() == "card"));
    assert!(found, "Global class must appear in traversal");
}

/// `class="..."` on void/self-closing HTML elements must be collected as
/// referenced classes, not silently dropped.
///
/// Before the fix, `visit_self_closing_element` returned early for non-`<link>`
/// tags, so `<img class="hero" />` and `<input class="field" />` never reached
/// `referenced_classes`, which produced false `noUndeclaredClasses` diagnostics.
#[test]
fn test_html_self_closing_element_class_references_are_collected() {
    let src = r#"<img class="hero" /><input class="field" /><br class="spacer" />"#;
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.html".into(), src);

    let html_path = BiomePath::new("/src/index.html");
    let html_root = parse_html_src(src, HtmlFileSource::html());

    let module_graph = ModuleGraph::default();
    let layout = ProjectLayout::default();
    module_graph.update_graph_for_html_paths(&fs, &layout, &[(&html_path, html_root, vec![])]);

    let html_info = module_graph
        .html_module_info_for_path(Utf8Path::new("/src/index.html"))
        .expect("HTML module must exist");

    let has = |name: &str| html_info.referenced_classes.iter().any(|r| r.matches(name));

    assert!(has("hero"), "expected 'hero' in referenced_classes");
    assert!(has("field"), "expected 'field' in referenced_classes");
    assert!(has("spacer"), "expected 'spacer' in referenced_classes");
}

/// Vue `<style>` (unscoped) → classes are Global.
#[test]
fn test_vue_unscoped_style_classes_are_global() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/Comp.vue".into(),
        r#"<template><div class="card"></div></template>"#,
    );

    let html_path = BiomePath::new("/src/Comp.vue");
    let html_root = parse_html_src(
        r#"<template><div class="card"></div></template>"#,
        HtmlFileSource::vue(),
    );
    let css = parse_embedded_css(".card { color: red; }", vue_global_css_source());

    let module_graph = ModuleGraph::default();
    let layout = ProjectLayout::default();
    module_graph.update_graph_for_html_paths(&fs, &layout, &[(&html_path, html_root, vec![css])]);

    let html_info = module_graph
        .html_module_info_for_path(Utf8Path::new("/src/Comp.vue"))
        .expect("Vue module must exist");

    let def = html_info.style_classes.iter().next().unwrap();
    assert_eq!(def.name.text(), "card");
    assert_eq!(
        def.applicability,
        EmbeddingStyleApplicability::Global,
        "Vue unscoped <style> is Global"
    );

    let found = module_graph
        .traverse_import_tree_for_html_classes(Utf8Path::new("/src/Comp.vue"))
        .any(|step| step.css_classes.iter().any(|c| c.text() == "card"));
    assert!(found, "Global class must appear in traversal");
}

/// Vue `<style scoped>` → classes are Local and hidden from the traversal iterator.
#[test]
fn test_vue_scoped_style_classes_are_local_and_hidden() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/Scoped.vue".into(),
        r#"<template><div class="alpha"></div></template>"#,
    );

    let html_path = BiomePath::new("/src/Scoped.vue");
    let html_root = parse_html_src(
        r#"<template><div class="alpha"></div></template>"#,
        HtmlFileSource::vue(),
    );
    let css = parse_embedded_css(".alpha { margin: 0; }", vue_scoped_css_source());

    let module_graph = ModuleGraph::default();
    let layout = ProjectLayout::default();
    module_graph.update_graph_for_html_paths(&fs, &layout, &[(&html_path, html_root, vec![css])]);

    let html_info = module_graph
        .html_module_info_for_path(Utf8Path::new("/src/Scoped.vue"))
        .expect("Vue module must exist");

    // Class IS stored with Local applicability.
    let def = html_info.style_classes.iter().next().unwrap();
    assert_eq!(def.name.text(), "alpha");
    assert_eq!(
        def.applicability,
        EmbeddingStyleApplicability::Local,
        "Vue <style scoped> is Local"
    );

    // The traversal DOES yield local inline classes for same-file checks,
    // because scoped styles still apply to the component's own elements.
    let found = module_graph
        .traverse_import_tree_for_html_classes(Utf8Path::new("/src/Scoped.vue"))
        .any(|step| step.css_classes.iter().any(|c| c.text() == "alpha"));
    assert!(
        found,
        "Local inline class MUST appear in same-file traversal"
    );
}

/// Vue with both scoped and unscoped blocks → only Global class is visible.
#[test]
fn test_vue_mixed_scoped_and_unscoped() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/Mixed.vue".into(), r#"<template></template>"#);

    let html_path = BiomePath::new("/src/Mixed.vue");
    let html_root = parse_html_src(r#"<template></template>"#, HtmlFileSource::vue());

    let global_css = parse_embedded_css(".global-btn { color: red; }", vue_global_css_source());
    let scoped_css = parse_embedded_css(".scoped-card { border: 1px; }", vue_scoped_css_source());

    let module_graph = ModuleGraph::default();
    let layout = ProjectLayout::default();
    module_graph.update_graph_for_html_paths(
        &fs,
        &layout,
        &[(&html_path, html_root, vec![global_css, scoped_css])],
    );

    let html_info = module_graph
        .html_module_info_for_path(Utf8Path::new("/src/Mixed.vue"))
        .expect("Vue module must exist");

    // Both classes are stored.
    assert_eq!(html_info.style_classes.len(), 2);

    // Only Global class appears in the traversal.
    let traversal_classes: Vec<_> = module_graph
        .traverse_import_tree_for_html_classes(Utf8Path::new("/src/Mixed.vue"))
        .flat_map(|step| {
            step.css_classes
                .iter()
                .map(|c| c.text().to_string())
                .collect::<Vec<_>>()
        })
        .collect();

    assert!(
        traversal_classes.contains(&"global-btn".to_string()),
        "global-btn must appear in traversal"
    );
    // Local inline classes also appear in same-file traversal: scoped styles
    // apply to the component's own elements (scoping only restricts leaking,
    // not same-file applicability).
    assert!(
        traversal_classes.contains(&"scoped-card".to_string()),
        "scoped-card must appear in same-file traversal"
    );
}

/// Astro `<style>` (default → Local) → classes are Local and hidden.
#[test]
fn test_astro_local_style_classes_are_hidden() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/Page.astro".into(), r#"<div class="hero"></div>"#);

    let html_path = BiomePath::new("/src/Page.astro");
    let html_root = parse_html_src(r#"<div class="hero"></div>"#, HtmlFileSource::astro());
    let css = parse_embedded_css(".hero { font-size: 2rem; }", astro_local_css_source());

    let module_graph = ModuleGraph::default();
    let layout = ProjectLayout::default();
    module_graph.update_graph_for_html_paths(&fs, &layout, &[(&html_path, html_root, vec![css])]);

    let html_info = module_graph
        .html_module_info_for_path(Utf8Path::new("/src/Page.astro"))
        .expect("Astro module must exist");

    let def = html_info.style_classes.iter().next().unwrap();
    assert_eq!(def.applicability, EmbeddingStyleApplicability::Local);

    // Local inline classes appear in same-file traversal: scoped styles still
    // apply to the component's own elements. Scoping only restricts leaking to
    // parent/consumer files.
    let found = module_graph
        .traverse_import_tree_for_html_classes(Utf8Path::new("/src/Page.astro"))
        .any(|step| step.css_classes.iter().any(|c| c.text() == "hero"));
    assert!(
        found,
        "Astro local class MUST appear in same-file traversal"
    );
}

/// Astro `<style is:global>` → classes are Global and visible.
#[test]
fn test_astro_global_style_classes_are_visible() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/Layout.astro".into(), r#"<div class="wrapper"></div>"#);

    let html_path = BiomePath::new("/src/Layout.astro");
    let html_root = parse_html_src(r#"<div class="wrapper"></div>"#, HtmlFileSource::astro());
    let css = parse_embedded_css(".wrapper { max-width: 80ch; }", astro_global_css_source());

    let module_graph = ModuleGraph::default();
    let layout = ProjectLayout::default();
    module_graph.update_graph_for_html_paths(&fs, &layout, &[(&html_path, html_root, vec![css])]);

    let found = module_graph
        .traverse_import_tree_for_html_classes(Utf8Path::new("/src/Layout.astro"))
        .any(|step| step.css_classes.iter().any(|c| c.text() == "wrapper"));
    assert!(found, "Astro is:global class must appear in traversal");
}

/// Svelte `<style>` (default → Local) → classes are Local and hidden.
#[test]
fn test_svelte_local_style_classes_are_hidden() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/Button.svelte".into(),
        r#"<button class="btn">Click</button>"#,
    );

    let html_path = BiomePath::new("/src/Button.svelte");
    let html_root = parse_html_src(
        r#"<button class="btn">Click</button>"#,
        HtmlFileSource::svelte(),
    );
    let css = parse_embedded_css(".btn { padding: 0.5rem; }", svelte_local_css_source());

    let module_graph = ModuleGraph::default();
    let layout = ProjectLayout::default();
    module_graph.update_graph_for_html_paths(&fs, &layout, &[(&html_path, html_root, vec![css])]);

    let html_info = module_graph
        .html_module_info_for_path(Utf8Path::new("/src/Button.svelte"))
        .expect("Svelte module must exist");

    let def = html_info.style_classes.iter().next().unwrap();
    assert_eq!(
        def.applicability,
        EmbeddingStyleApplicability::Local,
        "Svelte default <style> is Local"
    );

    // Local inline classes appear in same-file traversal: scoped styles still
    // apply to the component's own elements. Scoping only restricts leaking to
    // parent/consumer files.
    let found = module_graph
        .traverse_import_tree_for_html_classes(Utf8Path::new("/src/Button.svelte"))
        .any(|step| step.css_classes.iter().any(|c| c.text() == "btn"));
    assert!(
        found,
        "Svelte local class MUST appear in same-file traversal"
    );
}

/// Svelte `<style>` with a `:global(.foo)` selector → `.foo` is Global even
/// though the surrounding block is Local.
#[test]
fn test_svelte_global_pseudo_class_is_visible() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/Global.svelte".into(), r#"<div class="prose"></div>"#);

    let html_path = BiomePath::new("/src/Global.svelte");
    let html_root = parse_html_src(r#"<div class="prose"></div>"#, HtmlFileSource::svelte());
    // The surrounding block is Local, but :global(.prose) makes `.prose` Global.
    let css = parse_embedded_css(
        ":global(.prose) { font-size: 1rem; }",
        svelte_local_css_source(),
    );

    let module_graph = ModuleGraph::default();
    let layout = ProjectLayout::default();
    module_graph.update_graph_for_html_paths(&fs, &layout, &[(&html_path, html_root, vec![css])]);

    let html_info = module_graph
        .html_module_info_for_path(Utf8Path::new("/src/Global.svelte"))
        .expect("Svelte module must exist");

    // The class must be stored with Global applicability.
    let def = html_info
        .style_classes
        .iter()
        .find(|c| c.name.text() == "prose")
        .expect(".prose class must exist");
    assert_eq!(
        def.applicability,
        EmbeddingStyleApplicability::Global,
        ":global(.prose) must be stored as Global even in a Local Svelte block"
    );

    // And it must appear in the traversal.
    let found = module_graph
        .traverse_import_tree_for_html_classes(Utf8Path::new("/src/Global.svelte"))
        .any(|step| step.css_classes.iter().any(|c| c.text() == "prose"));
    assert!(found, ":global class must appear in traversal");
}

// #endregion

/// Verifies that upward traversal through a Vue component chain finds CSS classes.
///
/// Component hierarchy:
///   App.vue → imports app.css AND Page.vue
///   Page.vue → imports Button.vue
///   Button.vue → uses .btn-invalid (not in app.css)
///
/// When looking up available classes for Button.vue, we should find the classes
/// from app.css by traversing up: Button → Page → App → app.css.
#[test]
fn test_vue_upward_traversal() {
    use biome_html_parser::HtmlParserOptions;
    use biome_html_syntax::HtmlFileSource;
    use biome_js_parser::JsParserOptions;

    let fs = MemoryFileSystem::default();

    // Insert all files into the filesystem for path resolution
    fs.insert("/src/app.css".into(), ".app { } .page { } .btn { }");
    fs.insert("/src/App.vue".into(), "");
    fs.insert("/src/Page.vue".into(), "");
    fs.insert("/src/Button.vue".into(), "");

    let layout = ProjectLayout::default();
    let module_graph = ModuleGraph::default();

    // Add CSS
    let css_paths = [BiomePath::new("/src/app.css")];
    let css_roots = get_css_added_paths(&fs, &css_paths);
    module_graph.update_graph_for_css_paths(&fs, &layout, &css_roots, None);

    // Parse HTML files
    let app_root = biome_html_parser::parse_html(
        r#"<template><div class="app"></div></template>"#,
        HtmlParserOptions::from(&HtmlFileSource::vue()),
    )
    .tree();
    // App.vue's embedded <script> imports app.css and Page.vue
    let app_script = biome_js_parser::parse(
        r#"import "./app.css"; import Page from "./Page.vue";"#,
        biome_js_parser::JsFileSource::ts(),
        JsParserOptions::default(),
    );
    let app_embedded = vec![HtmlEmbeddedContent::Js(app_script.tree())];

    let page_root = biome_html_parser::parse_html(
        r#"<template><div class="page"></div></template>"#,
        HtmlParserOptions::from(&HtmlFileSource::vue()),
    )
    .tree();
    // Page.vue's embedded <script> imports Button.vue
    let page_script = biome_js_parser::parse(
        r#"import Button from "./Button.vue";"#,
        biome_js_parser::JsFileSource::ts(),
        JsParserOptions::default(),
    );
    let page_embedded = vec![HtmlEmbeddedContent::Js(page_script.tree())];

    let button_root = biome_html_parser::parse_html(
        r#"<template><button class="btn-invalid">Bad class</button></template>"#,
        HtmlParserOptions::from(&HtmlFileSource::vue()),
    )
    .tree();
    let button_embedded: Vec<HtmlEmbeddedContent> = vec![];

    let app_path = BiomePath::new("/src/App.vue");
    let page_path = BiomePath::new("/src/Page.vue");
    let button_path = BiomePath::new("/src/Button.vue");

    module_graph.update_graph_for_html_paths(
        &fs,
        &layout,
        &[
            (&app_path, app_root, app_embedded),
            (&page_path, page_root, page_embedded),
            (&button_path, button_root, button_embedded),
        ],
    );

    // Verify App.vue has resolved import paths
    let app_info = module_graph
        .html_module_info_for_path(Utf8Path::new("/src/App.vue"))
        .expect("App.vue must be in module graph");

    assert!(
        !app_info.static_import_paths.is_empty(),
        "App.vue should have static import paths (app.css and Page.vue)"
    );

    let page_info = module_graph
        .html_module_info_for_path(Utf8Path::new("/src/Page.vue"))
        .expect("Page.vue must be in module graph");

    assert!(
        !page_info.static_import_paths.is_empty(),
        "Page.vue should have static import paths (Button.vue)"
    );

    // Verify upward traversal finds btn from app.css
    let available_classes: Vec<_> = module_graph
        .traverse_import_tree_for_html_classes(Utf8Path::new("/src/Button.vue"))
        .flat_map(|step| step.css_classes.into_iter())
        .collect();

    assert!(
        available_classes.iter().any(|c| c.text() == "btn"),
        "btn class from app.css should be visible to Button.vue via upward traversal. Available: {:?}",
        available_classes
            .iter()
            .map(|c| c.text())
            .collect::<Vec<_>>()
    );
}

/// Snapshot test: a Vue parent component imports a CSS file and a child Vue
/// component via its `<script>` block. Verifies that the module graph resolves
/// both the stylesheet edge and the component import edge.
///
/// Uses a [`WorkspaceServer`] to parse embedded nodes exactly as production
/// does, avoiding hand-crafted snippet mismatches.
#[test]
fn test_vue_component_imports_snapshot() {
    let files: Vec<(&str, &str)> = vec![
        (
            "/src/app.css",
            ".app { color: red; }\n.btn { padding: 8px; }\n",
        ),
        (
            "/src/Button.vue",
            r#"<template>
  <button class="btn">Click me</button>
</template>

<style scoped>
  .btn { font-weight: bold; }
</style>
"#,
        ),
        (
            "/src/App.vue",
            r#"<template>
  <div class="app"><Button /></div>
</template>

<script>
import "./app.css";
import Button from "./Button.vue";
</script>
"#,
        ),
    ];

    let module_graph = build_module_graph_via_workspace(&files);
    let snapshot_files = files_to_snapshot_vec(&files);
    let snapshot = ModuleGraphSnapshot::from_files(&module_graph, snapshot_files);
    snapshot.assert_snapshot("test_vue_component_imports_snapshot");
}

/// Snapshot test: an Astro parent component imports a CSS file and a child
/// Astro component via its frontmatter. Verifies that the module graph resolves
/// both edges correctly for the Astro embedding model.
///
/// Uses a [`WorkspaceServer`] to parse embedded nodes exactly as production
/// does, avoiding hand-crafted snippet mismatches.
#[test]
fn test_astro_component_imports_snapshot() {
    let files: Vec<(&str, &str)> = vec![
        (
            "/src/global.css",
            ".layout { display: flex; }\n.hero { font-size: 2rem; }\n",
        ),
        (
            "/src/Hero.astro",
            r#"---
---
<section class="hero">Welcome</section>

<style>
  .hero { color: navy; }
</style>
"#,
        ),
        (
            "/src/Layout.astro",
            r#"---
import "./global.css";
import Hero from "./Hero.astro";
---
<div class="layout"><Hero /></div>
"#,
        ),
    ];

    let module_graph = build_module_graph_via_workspace(&files);
    let snapshot_files = files_to_snapshot_vec(&files);
    let snapshot = ModuleGraphSnapshot::from_files(&module_graph, snapshot_files);
    snapshot.assert_snapshot("test_astro_component_imports_snapshot");
}

/// Snapshot test: a Svelte parent component imports a CSS file and a child
/// Svelte component. Verifies that the module graph resolves both edges
/// correctly for the Svelte embedding model.
///
/// Uses a [`WorkspaceServer`] to parse embedded nodes exactly as production
/// does, avoiding hand-crafted snippet mismatches.
#[test]
fn test_svelte_component_imports_snapshot() {
    let files: Vec<(&str, &str)> = vec![
        (
            "/src/theme.css",
            ".wrapper { max-width: 1200px; }\n.title { font-weight: bold; }\n",
        ),
        (
            "/src/Card.svelte",
            r#"<script>
</script>

<div class="card">Content</div>

<style>
  .card { border: 1px solid; }
</style>
"#,
        ),
        (
            "/src/App.svelte",
            r#"<script>
import "./theme.css";
import Card from "./Card.svelte";
</script>

<div class="wrapper title"><Card /></div>
"#,
        ),
    ];

    let module_graph = build_module_graph_via_workspace(&files);
    let snapshot_files = files_to_snapshot_vec(&files);
    let snapshot = ModuleGraphSnapshot::from_files(&module_graph, snapshot_files);
    snapshot.assert_snapshot("test_svelte_component_imports_snapshot");
}

// #endregion

/// Builds a [`ModuleGraph`] by indexing all given files through a real
/// [`WorkspaceServer`] instance.
///
/// This mirrors production behavior: `open_file` triggers
/// `parse_embedded_nodes`, which correctly extracts `<style>`, `<script>`, and
/// Astro frontmatter (`---...---`) blocks with their scoping semantics.
fn build_module_graph_via_workspace(files: &[(&str, &str)]) -> Arc<ModuleGraph> {
    let mem_fs = MemoryFileSystem::default();
    for (path, content) in files {
        mem_fs.insert(Utf8PathBuf::from(*path), *content);
    }

    let (workspace, project_key) = setup_workspace_and_open_project(mem_fs, "/src");

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            configuration: Configuration {
                html: Some(HtmlConfiguration {
                    experimental_full_support_enabled: Some(true.into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            workspace_directory: None,
            extended_configurations: vec![],
            module_graph_resolution_kind: ModuleGraphResolutionKind::Modules,
        })
        .expect("can update settings");

    let files_with_sources = files.iter().map(|(path, _)| {
        let biome_path = BiomePath::new(Utf8PathBuf::from(*path));
        let document_file_source = DocumentFileSource::from_well_known(biome_path.as_path(), true);
        (biome_path, document_file_source)
    });
    workspace.index_files_for_test(project_key, files_with_sources);

    workspace.module_graph()
}

/// Converts a `&[(&str, &str)]` file list into the owned form expected by
/// [`ModuleGraphSnapshot::from_files`].
fn files_to_snapshot_vec(files: &[(&str, &str)]) -> Vec<(String, String)> {
    files
        .iter()
        .map(|(path, content)| ((*path).to_string(), (*content).to_string()))
        .collect()
}
