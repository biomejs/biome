use biome_deserialize::json::deserialize_from_json_str;
use biome_fs::{MemoryFileSystem, OsFileSystem};
use biome_js_parser::JsParserOptions;
use biome_js_syntax::JsFileSource;
use biome_json_parser::JsonParserOptions;
use biome_json_value::JsonString;
use biome_package::{Dependencies, PackageJson, Version};

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
    path.join("crates/biome_dependency_graph/fixtures")
}

#[test]
fn test_resolve_relative_import() {
    let (fs, project_layout) = create_test_project_layout();
    let added_paths = vec![
        BiomePath::new("/src/index.ts"),
        BiomePath::new("/src/bar.ts"),
    ];

    let dependency_graph = DependencyGraph::default();
    dependency_graph.update_graph_for_js_paths(&fs, &project_layout, &added_paths, &[], |path| {
        fs.read_file_from_path(path).ok().and_then(|content| {
            let parsed =
                biome_js_parser::parse(&content, JsFileSource::tsx(), JsParserOptions::default());
            assert!(parsed.diagnostics().is_empty());
            parsed.try_tree()
        })
    });

    let imports = dependency_graph.data.pin();
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
    let added_paths = vec![
        BiomePath::new("/src/index.ts"),
        BiomePath::new("/node_modules/shared/dist/index.js"),
    ];

    let dependency_graph = DependencyGraph::default();
    dependency_graph.update_graph_for_js_paths(&fs, &project_layout, &added_paths, &[], |path| {
        fs.read_file_from_path(path).ok().and_then(|content| {
            let parsed =
                biome_js_parser::parse(&content, JsFileSource::tsx(), JsParserOptions::default());
            assert!(parsed.diagnostics().is_empty());
            parsed.try_tree()
        })
    });

    let imports = dependency_graph.data.pin();
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

    let added_paths = vec![
        BiomePath::new(format!("{fixtures_path}/frontend/src/bar.ts")),
        BiomePath::new(format!("{fixtures_path}/frontend/src/index.ts")),
        BiomePath::new(format!(
            "{fixtures_path}/frontend/node_modules/shared/dist/index.js"
        )),
        BiomePath::new(format!("{fixtures_path}/shared/dist/index.js")),
    ];

    let dependency_graph = DependencyGraph::default();
    dependency_graph.update_graph_for_js_paths(&fs, &project_layout, &added_paths, &[], |path| {
        fs.read_file_from_path(path).ok().and_then(|content| {
            let parsed =
                biome_js_parser::parse(&content, JsFileSource::tsx(), JsParserOptions::default());
            assert!(parsed.diagnostics().is_empty());
            parsed.try_tree()
        })
    });

    let imports = dependency_graph.data.pin();
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
             * FIXME: This does not yet get detected.
             */
            function foo() {}

            export { foo };
            
            /**
             * @package
             */
            export function bar() {}

            export const { a, b, c: [d, e] } = getObject();

            /**
             * @public
             * @returns {JSX.Element}
             */
            export default function Component() {}

            export * from "./reexports";
            export { ohNo as "oh\x0Ano" } from "./renamed-reexports";
        "#,
    );
    fs.insert(
        "/src/reexports.ts".into(),
        r#"
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

    let added_paths = vec![
        BiomePath::new("/src/index.ts"),
        BiomePath::new("/src/reexports.ts"),
        BiomePath::new("/src/renamed-reexports.ts"),
    ];

    let dependency_graph = DependencyGraph::default();
    dependency_graph.update_graph_for_js_paths(&fs, &project_layout, &added_paths, &[], |path| {
        fs.read_file_from_path(path).ok().and_then(|content| {
            let parsed =
                biome_js_parser::parse(&content, JsFileSource::tsx(), JsParserOptions::default());
            assert!(parsed.diagnostics().is_empty());
            parsed.try_tree()
        })
    });

    let dependency_data = dependency_graph.data.pin();
    let data = dependency_data.get(Utf8Path::new("/src/index.ts")).unwrap();

    assert_eq!(data.exports.len(), 8);
    assert_eq!(
        data.exports.get(&Text::Static("foo")),
        Some(&Export::Own(OwnExport {
            jsdoc_comment: None // FIXME: See above.
        }))
    );
    assert_eq!(
        data.exports.get(&Text::Static("bar")),
        Some(&Export::Own(OwnExport {
            jsdoc_comment: Some("@package".to_string())
        }))
    );

    let destructured_exports = ["a", "b", "d", "e"];
    for export in destructured_exports {
        assert_eq!(
            data.exports.get(&Text::Static(export)),
            Some(&Export::Own(OwnExport {
                jsdoc_comment: None
            }))
        );
    }

    assert_eq!(
        data.exports.get(&Text::Static("default")),
        Some(&Export::Own(OwnExport {
            jsdoc_comment: Some("@public\n@returns {JSX.Element}".to_string())
        }))
    );

    assert_eq!(
        data.exports.get(&Text::Static("oh\nno")),
        Some(&Export::Reexport(Import {
            resolved_path: Ok(Utf8PathBuf::from("/src/renamed-reexports.ts"))
        }))
    );

    assert_eq!(
        data.blanket_reexports,
        vec![ReexportAll {
            import: Import {
                resolved_path: Ok(Utf8PathBuf::from("/src/reexports.ts"))
            }
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
            }
        }))
    );
}
