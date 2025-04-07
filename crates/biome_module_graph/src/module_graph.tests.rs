use biome_deserialize::json::deserialize_from_json_str;
use biome_fs::{MemoryFileSystem, OsFileSystem};
use biome_json_parser::JsonParserOptions;
use biome_json_value::JsonString;
use biome_package::{Dependencies, PackageJson, Version};
use biome_rowan::Text;
use biome_test_utils::get_added_paths;
use insta::assert_debug_snapshot;

use crate::{Import, jsdoc_comment::JsdocComment, module_info::ReexportAll};

use super::*;

fn create_test_project_layout() -> (MemoryFileSystem, ProjectLayout) {
    let mut fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import { foo } from "shared";
            import { bar } from "./bar.ts";

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
        "/tsconfig.json".into(),
        r#"{
            "include": [
                "./src"
            ]
        }"#,
    );

    fs.insert(
        "/node_modules/shared/dist/index.js".into(),
        r#"
            export function foo() {}
        "#,
    );

    let project_layout = ProjectLayout::default();
    project_layout.insert_node_manifest(
        "/".into(),
        PackageJson::new("frontend")
            .with_path("/package.json")
            .with_version(Version::Literal("0.0.0".into()))
            .with_dependencies(Dependencies::from([(
                "shared".into(),
                Version::Literal("link:./node_modules/shared".into()),
            )])),
    );

    project_layout.insert_node_manifest(
        "/node_modules/shared".into(),
        PackageJson::new("shared")
            .with_path("/node_modules/shared/package.json")
            .with_exports(JsonString::from("./dist/index.js"))
            .with_version(Version::Literal("0.0.1".into())),
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
    path.join("crates/biome_module_graph/fixtures")
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

    let imports = module_graph.data.pin();
    let file_imports = imports.get(Utf8Path::new("/src/index.ts")).unwrap();

    assert_eq!(file_imports.static_imports.len(), 2);
    assert_eq!(
        file_imports.static_imports.get("./bar.ts"),
        Some(&Import {
            resolved_path: Ok(Utf8PathBuf::from("/src/bar.ts"))
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

    let imports = module_graph.data.pin();
    let file_imports = imports.get(Utf8Path::new("/src/index.ts")).unwrap();

    assert_eq!(file_imports.static_imports.len(), 2);
    assert_eq!(
        file_imports.static_imports.get("shared"),
        Some(&Import {
            resolved_path: Ok(Utf8PathBuf::from("/node_modules/shared/dist/index.js"))
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
        .with_path(path)
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
        .with_path(path)
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
            .with_path_and_canonicalized_path(path, format!("{fixtures_path}/shared/package.json"))
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

    let imports = module_graph.data.pin();
    let file_imports = imports
        .get(Utf8Path::new(&format!(
            "{fixtures_path}/frontend/src/index.ts"
        )))
        .unwrap();

    assert_eq!(file_imports.static_imports.len(), 2);
    assert_eq!(
        file_imports.static_imports.get("shared"),
        Some(&Import {
            resolved_path: Ok(Utf8PathBuf::from(format!(
                "{fixtures_path}/shared/dist/index.js"
            )))
        })
    );
    assert_eq!(
        file_imports.static_imports.get("./bar"),
        Some(&Import {
            resolved_path: Ok(Utf8PathBuf::from(format!(
                "{fixtures_path}/frontend/src/bar.ts"
            )))
        })
    );
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

            /**
             * TODO: No types can be detected on these yet.
             */
            export const { a, b, c: [d, e] } = getObject();

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
        PackageJson::new("frontend")
            .with_path("/package.json")
            .with_version(Version::Literal("0.0.0".into())),
    );

    let added_paths = [
        BiomePath::new("/src/index.ts"),
        BiomePath::new("/src/reexports.ts"),
        BiomePath::new("/src/renamed-reexports.ts"),
    ];
    let added_paths = get_added_paths(&fs, &added_paths);

    let module_graph = ModuleGraph::default();
    module_graph.update_graph_for_js_paths(&fs, &project_layout, &added_paths, &[]);

    let dependency_data = module_graph.data.pin();
    let mut data = dependency_data
        .get(Utf8Path::new("/src/index.ts"))
        .unwrap()
        .clone();

    // Remove this entry, or the Windows tests fail on the path in the snapshot below:
    assert_eq!(
        data.exports.remove(&Text::Static("oh\nno")),
        Some(Export::Reexport(Import {
            resolved_path: Ok(Utf8PathBuf::from("/src/renamed-reexports.ts")),
        }))
    );
    assert_eq!(
        data.exports.remove(&Text::Static("renamed2")),
        Some(Export::ReexportAll(ReexportAll {
            import: Import {
                resolved_path: Ok(Utf8PathBuf::from("/src/renamed-reexports.ts")),
            },
            jsdoc_comment: Some(JsdocComment::from_comment_text(
                "/**\n* Hello, namespace 2.\n*/"
            )),
        }))
    );

    assert_debug_snapshot!(data.exports);

    assert_eq!(
        data.blanket_reexports,
        vec![ReexportAll {
            import: Import {
                resolved_path: Ok(Utf8PathBuf::from("/src/reexports.ts")),
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
        Some(&Export::ReexportAll(ReexportAll {
            import: Import {
                resolved_path: Ok(Utf8PathBuf::from("/src/renamed-reexports.ts"))
            },
            jsdoc_comment: Some(JsdocComment::from_comment_text(
                "/**\n* Hello, namespace 1.\n*/"
            ))
        }))
    );
}
