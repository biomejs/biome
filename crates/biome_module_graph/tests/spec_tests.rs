#![allow(clippy::arc_with_non_send_sync)]

mod snap;

use std::fs::read_link;
use std::ops::Deref;
use std::sync::Arc;

use crate::snap::ModuleGraphSnapshot;
use biome_deserialize::json::deserialize_from_json_str;
use biome_fs::{BiomePath, FileSystem, MemoryFileSystem, OsFileSystem, normalize_path};
use biome_js_type_info::{ScopeId, TypeData, TypeResolver};
use biome_jsdoc_comment::JsdocComment;
use biome_json_parser::{JsonParserOptions, parse_json};
use biome_json_value::{JsonObject, JsonString};
use biome_module_graph::JsExport;
use biome_module_graph::{
    ImportSymbol, JsImport, JsReexport, ModuleGraph, ModuleResolver, ResolvedPath,
};
use biome_package::{Dependencies, PackageJson};
use biome_project_layout::ProjectLayout;
use biome_rowan::Text;
use biome_test_utils::get_added_paths;
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
    project_layout.insert_serialized_tsconfig("/".into(), tsconfig_json.into());

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
fn test_resolve_relative_import() {
    let (fs, project_layout) = create_test_project_layout();
    let added_paths = [
        BiomePath::new("/src/index.ts"),
        BiomePath::new("/src/bar.ts"),
    ];
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &project_layout, &added_paths, &[]);

    let imports = module_graph.data();
    let file_imports = imports.get(Utf8Path::new("/src/index.ts")).unwrap();

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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &project_layout, &added_paths, &[]);

    let imports = module_graph.data();
    let file_imports = imports.get(Utf8Path::new("/src/index.ts")).unwrap();

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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &project_layout, &added_paths, &[]);

    let imports = module_graph.data();
    let file_imports = imports.get(Utf8Path::new("/src/index.ts")).unwrap();

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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &project_layout, &added_paths, &[]);

    let imports = module_graph.data();
    let file_imports = imports
        .get(Utf8Path::new(&format!(
            "{fixtures_path}/frontend/src/index.ts"
        )))
        .unwrap();

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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, &[]);

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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, &[]);

    let snapshot = ModuleGraphSnapshot::new(&module_graph, &fs);
    snapshot.assert_snapshot("test_export_default_function_declaration");
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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, &[]);

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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &project_layout, &added_paths, &[]);

    let dependency_data = module_graph.data();
    let data = dependency_data
        .get(Utf8Path::new("/src/index.ts"))
        .unwrap()
        .clone();
    let mut exports = data.exports.clone();

    // Remove this entry, or the Windows tests fail on the path in the snapshot below:
    assert_eq!(
        exports.remove(&Text::Static("oh\nno")),
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
        exports.remove(&Text::Static("renamed2")),
        Some(JsExport::Reexport(JsReexport {
            import: JsImport {
                specifier: "./renamed-reexports".into(),
                resolved_path: ResolvedPath::from_path("/src/renamed-reexports.ts"),
                symbol: ImportSymbol::All,
            },
            jsdoc_comment: Some(JsdocComment::from_comment_text(
                "/**\n* Hello, namespace 2.\n*/"
            )),
        }))
    );

    assert_eq!(
        data.blanket_reexports.as_ref(),
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
    assert_eq!(data.exports.len(), 1);
    assert_eq!(
        data.exports.get(&Text::Static("renamed")),
        Some(&JsExport::Reexport(JsReexport {
            import: JsImport {
                specifier: "./renamed-reexports".into(),
                resolved_path: ResolvedPath::from_path("/src/renamed-reexports.ts"),
                symbol: ImportSymbol::All,
            },
            jsdoc_comment: Some(JsdocComment::from_comment_text(
                "/**\n* Hello, namespace 1.\n*/"
            ))
        }))
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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, &[]);

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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, &[]);

    let index_module = module_graph
        .module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let promise_id = resolver
        .resolve_type_of(&Text::Static("promise"), ScopeId::GLOBAL)
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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, &[]);

    let index_module = module_graph
        .module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let mapped_id = resolver
        .resolve_type_of(&Text::Static("mapped"), ScopeId::GLOBAL)
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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, &[]);

    let index_module = module_graph
        .module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let result_id = resolver
        .resolve_type_of(&Text::Static("result"), ScopeId::GLOBAL)
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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, &[]);

    let index_module = module_graph
        .module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let result_id = resolver
        .resolve_type_of(&Text::Static("result"), ScopeId::GLOBAL)
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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, &[]);

    let index_module = module_graph
        .module_info_for_path(Utf8Path::new("/src/index.ts"))
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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, &[]);

    let index_module = module_graph
        .module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let foo_id = resolver
        .resolve_type_of(&Text::Static("foo"), ScopeId::GLOBAL)
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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, &[]);

    let index_module = module_graph
        .module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let foo_id = resolver
        .resolve_type_of(&Text::Static("foo"), ScopeId::GLOBAL)
        .expect("foo variable not found");
    let foo_ty = resolver.resolved_type_for_id(foo_id);
    let _foo_string_ty = format!("{foo_ty:?}");
    assert!(foo_ty.is_string_literal("foo"));

    let snapshot = ModuleGraphSnapshot::new(&module_graph, &fs);
    snapshot.assert_snapshot("test_resolve_type_of_property_with_getter");
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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, &[]);

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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, &[]);

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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, &[]);

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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, &[]);

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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, &[]);

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
    let added_paths = get_added_paths(&fs, &added_paths);

    let project_layout = ProjectLayout::default();
    project_layout.insert_node_manifest(
        "/".into(),
        PackageJson::new("frontend")
            .with_version("0.0.0")
            .with_dependencies(Dependencies(Box::new([("react".into(), "19.0.0".into())]))),
    );

    let tsconfig_json = parse_json(r#"{}"#, JsonParserOptions::default());
    project_layout.insert_serialized_tsconfig("/".into(), tsconfig_json.into());

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &project_layout, &added_paths, &[]);

    let index_module = module_graph
        .module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let use_callback_id = resolver
        .resolve_type_of(&Text::Static("useCallback"), ScopeId::GLOBAL)
        .expect("useCallback variable not found");
    let use_callback_ty = resolver.resolved_type_for_id(use_callback_id);
    assert!(use_callback_ty.is_function());

    let promise_id = resolver
        .resolve_type_of(&Text::Static("promise"), ScopeId::GLOBAL)
        .expect("promise variable not found");
    let promise_ty = resolver.resolved_type_for_id(promise_id);
    assert!(promise_ty.is_promise_instance());
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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, &[]);

    let index_module = module_graph
        .module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let result_id = resolver
        .resolve_type_of(&Text::Static("result"), ScopeId::GLOBAL)
        .expect("result variable not found");
    let ty = resolver.resolved_type_for_id(result_id);
    assert!(ty.is_number_or_number_literal());

    let snapshot = ModuleGraphSnapshot::new(module_graph.as_ref(), &fs).with_resolver(&resolver);
    snapshot.assert_snapshot("test_resolve_single_reexport");
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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, &[]);

    let index_module = module_graph
        .module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let result1_id = resolver
        .resolve_type_of(&Text::Static("result1"), ScopeId::GLOBAL)
        .expect("result1 variable not found");
    let ty = resolver.resolved_type_for_id(result1_id);
    assert!(ty.is_number_or_number_literal());

    let result2_id = resolver
        .resolve_type_of(&Text::Static("result2"), ScopeId::GLOBAL)
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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, &[]);

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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, &[]);

    let index_module = module_graph
        .module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let resolved_id = resolver
        .resolve_type_of(&Text::Static("promise"), ScopeId::GLOBAL)
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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, &[]);

    let index_module = module_graph
        .module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let resolved_id = resolver
        .resolve_type_of(&Text::Static("promise"), ScopeId::GLOBAL)
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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, &[]);

    let index_module = module_graph
        .module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let use_swr_config_id = resolver
        .resolve_type_of(&Text::Static("useSWRConfig"), ScopeId::GLOBAL)
        .expect("mutate variable not found");
    let use_swr_config_ty = resolver.resolved_type_for_id(use_swr_config_id);
    let _use_swr_config_ty_string = format!("{:?}", use_swr_config_ty.deref()); // for debugging
    assert!(use_swr_config_ty.is_function_with_return_type(|return_ty| {
        let _return_ty_string = format!("{:?}", return_ty.deref()); // for debugging
        return_ty.is_interface()
    }));

    let mutate_id = resolver
        .resolve_type_of(&Text::Static("mutate"), ScopeId::GLOBAL)
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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, &[]);

    let index_module = module_graph
        .module_info_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let intersection_id = resolver
        .resolve_type_of(&Text::Static("Intersection"), ScopeId::GLOBAL)
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
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = Arc::new(ModuleGraph::default());
    module_graph.update_graph_for_js_paths(&fs, &project_layout, &added_paths, &[]);

    let index_module = module_graph
        .module_info_for_path(Utf8Path::new(&format!(
            "{fixtures_path}/frontend/src/index.ts"
        )))
        .expect("module must exist");
    assert_eq!(
        index_module.static_import_paths.get("swr"),
        Some(&ResolvedPath::from_path(format!(
            "{swr_path}/dist/index/index.d.mts"
        )))
    );

    let swr_index_module = module_graph
        .module_info_for_path(Utf8Path::new(&format!("{swr_path}/dist/index/index.d.mts")))
        .expect("module must exist");
    assert_eq!(
        swr_index_module
            .static_import_paths
            .get("../_internal/index.mjs"),
        Some(&ResolvedPath::from_path(format!(
            "{swr_path}/dist/_internal/index.d.mts"
        )))
    );

    let resolver = Arc::new(ModuleResolver::for_module(
        index_module,
        module_graph.clone(),
    ));

    let mutate_id = resolver
        .resolve_type_of(&Text::Static("mutate"), ScopeId::GLOBAL)
        .expect("mutate variable not found");

    let mutate_ty = resolver.resolved_type_for_id(mutate_id);
    let _mutate_ty_string = format!("{:?}", mutate_ty.deref()); // for debugging
    assert!(mutate_ty.is_interface_with_member(|member| member.kind().is_call_signature()));

    let mutate_result_id = resolver
        .resolve_type_of(&Text::Static("mutateResult"), ScopeId::GLOBAL)
        .expect("mutateResult variable not found");

    let mutate_result_ty = resolver.resolved_type_for_id(mutate_result_id);
    let _mutate_result_ty_string = format!("{:?}", mutate_result_ty.deref()); // for debugging
    assert!(mutate_result_ty.is_promise_instance());
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
