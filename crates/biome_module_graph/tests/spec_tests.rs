#![allow(clippy::arc_with_non_send_sync)]

mod snap;

use std::borrow::Cow;
use std::sync::Arc;

use crate::snap::ModuleGraphSnapshot;
use biome_deserialize::json::deserialize_from_json_str;
use biome_fs::{BiomePath, FileSystem, MemoryFileSystem, OsFileSystem};
use biome_js_type_info::{ResolvedTypeId, ScopeId, Type, TypeData, TypeResolver};
use biome_jsdoc_comment::JsdocComment;
use biome_json_parser::{JsonParserOptions, parse_json};
use biome_json_value::{JsonObject, JsonString};
use biome_module_graph::JsExport;
use biome_module_graph::{
    ImportSymbol, JsImport, JsReexport, ModuleGraph, ResolvedPath, ScopedResolver,
};
use biome_package::{Dependencies, PackageJson};
use biome_project_layout::ProjectLayout;
use biome_rowan::Text;
use biome_test_utils::get_added_paths;
use camino::{Utf8Path, Utf8PathBuf};

fn create_test_project_layout() -> (MemoryFileSystem, ProjectLayout) {
    let mut fs = MemoryFileSystem::default();
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

    assert_eq!(file_imports.static_imports.len(), 2);
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
    let mut fs = MemoryFileSystem::default();
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
    let mut fs = MemoryFileSystem::default();
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
    let mut fs = MemoryFileSystem::default();
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
    let mut fs = MemoryFileSystem::default();
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

    let snapshot = ModuleGraphSnapshot::new(&module_graph, &fs);

    snapshot.assert_snapshot("test_resolve_exports");

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
}

#[test]
fn test_resolve_export_types() {
    let mut fs = MemoryFileSystem::default();
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
    let mut fs = MemoryFileSystem::default();
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
    let mut resolver = ScopedResolver::from_global_scope(index_module, module_graph.clone());
    resolver.run_inference();

    let resolved_id = resolver
        .resolve_type_of(&Text::Static("promise"), ScopeId::GLOBAL)
        .expect("promise variable not found");
    let ty = resolver
        .get_by_resolved_id(resolved_id)
        .expect("cannot find type data")
        .to_data();
    let _ty_string = format!("{ty:?}"); // for debugging
    let ty = ty.inferred(&mut resolver);
    let _ty_string = format!("{ty:?}"); // for debugging

    let id = resolver.register_type(Cow::Owned(ty));
    resolver.run_inference();

    let resolved_id = ResolvedTypeId::new(resolver.level(), id);
    let resolver = Arc::new(resolver);
    let ty = Type::from_id(resolver.clone(), resolved_id);
    assert!(ty.is_promise_instance());

    let snapshot = ModuleGraphSnapshot::new(module_graph.as_ref(), &fs).with_resolver(&resolver);
    snapshot.assert_snapshot("test_resolve_generic_return_value");
}

#[test]
fn test_resolve_generic_return_value_with_multiple_modules() {
    let mut fs = MemoryFileSystem::default();
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
    let mut resolver = ScopedResolver::from_global_scope(index_module, module_graph.clone());
    resolver.run_inference();

    let result_id = resolver
        .resolve_type_of(&Text::Static("result"), ScopeId::GLOBAL)
        .expect("result variable not found");
    let ty = resolver
        .get_by_resolved_id(result_id)
        .expect("cannot find type data")
        .to_data();
    let _ty_string = format!("{ty:?}"); // for debugging
    let ty = ty.inferred(&mut resolver);
    let _ty_string = format!("{ty:?}"); // for debugging

    let id = resolver.register_type(Cow::Owned(ty));
    resolver.run_inference();

    let resolved_id = ResolvedTypeId::new(resolver.level(), id);
    let resolver = Arc::new(resolver);
    let ty = Type::from_id(resolver.clone(), resolved_id);
    assert!(ty.is_string());

    let snapshot =
        ModuleGraphSnapshot::new(module_graph.as_ref(), &fs).with_resolver(resolver.as_ref());
    snapshot.assert_snapshot("test_resolve_generic_return_value_with_multiple_modules");
}

#[test]
fn test_resolve_nested_function_call_with_namespace_in_return_type() {
    let mut fs = MemoryFileSystem::default();
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
    let mut resolver = ScopedResolver::from_global_scope(index_module, module_graph.clone());
    resolver.run_inference();

    let result_id = resolver
        .resolve_type_of(&Text::Static("result"), ScopeId::GLOBAL)
        .expect("result variable not found");
    let ty = resolver
        .get_by_resolved_id(result_id)
        .expect("cannot find type data")
        .to_data();

    let ty = ty.flattened(&mut resolver);
    resolver.register_type(Cow::Owned(ty));
    resolver.run_inference();

    let snapshot = ModuleGraphSnapshot::new(module_graph.as_ref(), &fs).with_resolver(&resolver);
    snapshot.assert_snapshot("test_resolve_nested_function_call_with_namespace_in_return_type");
}

#[test]
fn test_resolve_promise_export() {
    let mut fs = MemoryFileSystem::default();
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
    let mut fs = MemoryFileSystem::default();
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
    let mut fs = MemoryFileSystem::default();
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
    let mut fs = MemoryFileSystem::default();
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
    let mut fs = MemoryFileSystem::default();
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
    let mut fs = MemoryFileSystem::default();
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
    let mut resolver = ScopedResolver::from_global_scope(index_module, module_graph.clone());
    resolver.run_inference();

    let use_callback_id = resolver
        .resolve_type_of(&Text::Static("useCallback"), ScopeId::GLOBAL)
        .expect("useCallback variable not found");
    let ty = resolver
        .get_by_resolved_id(use_callback_id)
        .expect("cannot find type data");
    assert!(matches!(ty.as_raw_data(), TypeData::Function(_)));

    let promise_id = resolver
        .resolve_type_of(&Text::Static("promise"), ScopeId::GLOBAL)
        .expect("promise variable not found");
    let ty = resolver
        .get_by_resolved_id(promise_id)
        .expect("cannot find type data")
        .to_data();
    let _ty_string = format!("{ty:?}"); // for debugging
    let ty = ty.inferred(&mut resolver);
    let _ty_string = format!("{ty:?}"); // for debugging

    let id = resolver.register_type(Cow::Owned(ty));
    resolver.run_inference();

    let resolved_id = ResolvedTypeId::new(resolver.level(), id);
    let resolver = Arc::new(resolver);
    let ty = Type::from_id(resolver.clone(), resolved_id);
    assert!(ty.is_promise_instance());

    let snapshot =
        ModuleGraphSnapshot::new(module_graph.as_ref(), &fs).with_resolver(resolver.as_ref());
    snapshot.assert_snapshot("test_resolve_react_types");
}

#[test]
fn test_resolve_export_type_referencing_imported_type() {
    let mut fs = MemoryFileSystem::default();
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
    let mut fs = MemoryFileSystem::default();
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
    let mut resolver = ScopedResolver::from_global_scope(index_module, module_graph.clone());
    resolver.run_inference();

    let resolved_id = resolver
        .resolve_type_of(&Text::Static("promise"), ScopeId::GLOBAL)
        .expect("promise variable not found");
    let ty = resolver
        .get_by_resolved_id(resolved_id)
        .expect("cannot find type data")
        .to_data();
    let _ty_string = format!("{ty:?}"); // for debugging
    let ty = ty.inferred(&mut resolver);
    let _ty_string = format!("{ty:?}"); // for debugging

    let id = resolver.register_type(Cow::Owned(ty));
    resolver.run_inference();

    let resolved_id = ResolvedTypeId::new(resolver.level(), id);
    let resolver = Arc::new(resolver);
    let ty = Type::from_id(resolver.clone(), resolved_id);
    assert!(ty.is_promise_instance());

    let snapshot =
        ModuleGraphSnapshot::new(module_graph.as_ref(), &fs).with_resolver(resolver.as_ref());
    snapshot.assert_snapshot(
        "test_resolve_promise_from_imported_function_returning_imported_promise_type",
    );
}

#[test]
fn test_resolve_promise_from_imported_function_returning_reexported_promise_type() {
    let mut fs = MemoryFileSystem::default();
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
    let mut resolver = ScopedResolver::from_global_scope(index_module, module_graph.clone());
    resolver.run_inference();

    let resolved_id = resolver
        .resolve_type_of(&Text::Static("promise"), ScopeId::GLOBAL)
        .expect("promise variable not found");
    let ty = resolver
        .get_by_resolved_id(resolved_id)
        .expect("cannot find type data")
        .to_data();
    let _ty_string = format!("{ty:?}"); // for debugging
    let ty = ty.inferred(&mut resolver);
    let _ty_string = format!("{ty:?}"); // for debugging

    let id = resolver.register_type(Cow::Owned(ty));
    resolver.run_inference();

    let resolved_id = ResolvedTypeId::new(resolver.level(), id);
    let resolver = Arc::new(resolver);
    let ty = Type::from_id(resolver.clone(), resolved_id);
    assert!(ty.is_promise_instance());

    let snapshot =
        ModuleGraphSnapshot::new(module_graph.as_ref(), &fs).with_resolver(resolver.as_ref());
    snapshot.assert_snapshot(
        "test_resolve_promise_from_imported_function_returning_reexported_promise_type",
    );
}
