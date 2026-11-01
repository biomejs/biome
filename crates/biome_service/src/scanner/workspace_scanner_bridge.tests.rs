//! Tests for the methods of [`WorkspaceScannerBridge`].
//!
//! These tests mainly verify that the workspace can handle interactions between
//! clients and the scanner when files are simultaneously opened and indexed, as
//! well as making sure that the workspace rejects indexing requests from the
//! scanner for paths that should be ignored by the scanner.
//!
//! No real watcher is instantiated in these tests.

use std::str::FromStr;

use biome_configuration::vcs::{VcsClientKind, VcsConfiguration};
use biome_configuration::{Configuration, FilesConfiguration, TailwindConfiguration};
use biome_fs::{BiomePath, MemoryFileSystem, TemporaryFs};
use biome_glob::NormalizedGlob;
use camino::{Utf8Path, Utf8PathBuf};

use super::{ScanKind, WorkspaceScannerBridge};
use crate::projects::ProjectKey;
use crate::scanner::{IndexRequestKind, IndexTrigger};

use crate::settings::ModuleGraphResolutionKind;
use crate::test_utils::setup_workspace_and_open_project;
use crate::workspace::{
    CloseFileParams, FileContent, GetFileContentParams, OpenFileParams, ScanProjectParams,
    UpdateSettingsParams,
};
use crate::{LocalWorkspace, Workspace, WorkspaceError};

#[test]
fn close_file_through_watcher_before_client() {
    const FILE_CONTENT: &str = "import 'foo';";

    let file_path = Utf8PathBuf::from("/project/a.js");

    let fs = MemoryFileSystem::default();
    fs.insert(file_path.clone(), FILE_CONTENT);

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");

    workspace
        .index_file(project_key, file_path.clone(), IndexTrigger::InitialScan)
        .expect("can index file");

    assert!(workspace.is_indexed(&file_path), "file should be indexed");

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(&file_path),
            content: FileContent::FromClient {
                content: FILE_CONTENT.to_string(),
                version: 1,
            },
            document_file_source: None,
            persist_node_cache: true,
            inline_config: None,
            editor_features: None,
        })
        .expect("can also open from client");

    assert!(
        workspace.is_indexed(&file_path),
        "file should still be indexed"
    );

    workspace
        .unload_file(&file_path, project_key)
        .expect("can unload indexed file");

    assert!(
        !workspace.is_indexed(&file_path),
        "file should no longer be indexed"
    );

    let content = workspace
        .get_file_content(GetFileContentParams {
            project_key,
            path: BiomePath::new(&file_path),
        })
        .expect("can still get file content");

    assert_eq!(content, FILE_CONTENT);

    workspace
        .close_file(CloseFileParams {
            project_key,
            path: BiomePath::new(&file_path),
        })
        .expect("can close file from client");

    let error = workspace
        .get_file_content(GetFileContentParams {
            project_key,
            path: BiomePath::new(&file_path),
        })
        .expect_err("can no longer get file content");

    assert!(matches!(error, WorkspaceError::NotFound(_)));
}

#[test]
fn close_file_from_client_before_watcher() {
    const FILE_CONTENT: &str = "import 'foo';";

    let file_path = Utf8PathBuf::from("/project/a.js");

    let fs = MemoryFileSystem::default();
    fs.insert(file_path.clone(), FILE_CONTENT);

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(&file_path),
            content: FileContent::FromClient {
                content: FILE_CONTENT.to_string(),
                version: 1,
            },
            document_file_source: None,
            persist_node_cache: true,
            inline_config: None,
            editor_features: None,
        })
        .expect("can open from client");

    workspace
        .index_file(project_key, file_path.clone(), IndexTrigger::Update)
        .expect("can also index file");

    let content = workspace
        .get_file_content(GetFileContentParams {
            project_key,
            path: BiomePath::new(&file_path),
        })
        .expect("can still get file content");

    assert_eq!(content, FILE_CONTENT);

    workspace
        .close_file(CloseFileParams {
            project_key,
            path: BiomePath::new(&file_path),
        })
        .expect("can close file from client");

    assert!(
        workspace.is_indexed(&file_path),
        "file should still be indexed"
    );

    workspace
        .unload_file(&file_path, project_key)
        .expect("can unload file from index");

    assert!(
        !workspace.is_indexed(&file_path),
        "file should no longer be indexed"
    );
}

#[test]
fn should_not_index_a_source_file_with_scan_kind_known_files() {
    let file_path = Utf8PathBuf::from("/project/a.js");

    let fs = MemoryFileSystem::default();
    fs.insert(file_path.clone(), "import 'foo';");

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::KnownFiles,
            verbose: false,
        })
        .expect("can scan the project");

    assert!(!workspace.is_indexed(&file_path));
}

#[test]
fn should_index_pnpm_workspace_yaml_as_manifest() {
    let pnpm_workspace_path = Utf8PathBuf::from("/project/pnpm-workspace.yaml");

    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8PathBuf::from("/project/package.json"),
        r#"{ "name": "app" }"#,
    );
    fs.insert(
        pnpm_workspace_path.clone(),
        "catalog:\n  react: 19.0.0\n".as_bytes(),
    );

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: false,
        })
        .expect("can scan the project");

    assert!(workspace.is_indexed(&pnpm_workspace_path));
}

#[test]
fn should_not_index_an_ignored_file_inside_vcs_ignore_file() {
    let file_path = Utf8PathBuf::from("/project/a.js");

    let fs = MemoryFileSystem::default();
    fs.insert(file_path.clone(), "import 'foo';");
    fs.insert(Utf8PathBuf::from("/project/.gitignore"), "a.js\n");

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: Some(BiomePath::new("/project")),
            configuration: Configuration {
                vcs: Some(VcsConfiguration {
                    use_ignore_file: Some(true.into()),
                    enabled: Some(true.into()),
                    client_kind: Some(VcsClientKind::Git),
                    ..Default::default()
                }),
                ..Default::default()
            },
            extended_configurations: Default::default(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::ModulesAndTypes,
        })
        .expect("can update settings");

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: false,
        })
        .expect("can scan the project");

    assert!(!workspace.is_indexed(&file_path));
}

#[test]
fn should_not_index_an_ignored_file_inside_file_includes() {
    let file_path = Utf8PathBuf::from("/project/dist/minified.js");

    let fs = MemoryFileSystem::default();
    fs.insert(file_path.clone(), "import 'foo';");

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: Some(BiomePath::new("/project")),
            configuration: Configuration {
                files: Some(FilesConfiguration {
                    includes: Some(vec![
                        NormalizedGlob::from_str("**").unwrap(),
                        NormalizedGlob::from_str("!**/dist/**").unwrap(),
                    ]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            extended_configurations: Default::default(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::ModulesAndTypes,
        })
        .expect("can update settings");

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: false,
        })
        .expect("can scan the project");

    assert!(!workspace.is_indexed(&file_path));
}

#[test]
fn should_index_an_ignored_file_if_it_is_a_dependency_of_a_non_ignored_file() {
    let file_path = Utf8PathBuf::from("/project/a.js");

    let fs = MemoryFileSystem::default();
    fs.insert(file_path.clone(), "import 'foo';");
    fs.insert("/project/b.js".into(), "import './a.js';");

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: Some(BiomePath::new("/project")),
            configuration: Configuration {
                files: Some(FilesConfiguration {
                    includes: Some(vec![
                        NormalizedGlob::from_str("**").unwrap(),
                        NormalizedGlob::from_str("!**/a.js").unwrap(),
                    ]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            extended_configurations: Default::default(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::ModulesAndTypes,
        })
        .expect("can update settings");

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: false,
        })
        .expect("can scan the project");

    assert!(workspace.is_indexed(&file_path));
}

#[test]
fn should_not_index_a_force_ignored_file_even_if_it_is_a_dependency() {
    let file_path_a = Utf8PathBuf::from("/project/a.js");
    let file_path_root_b = Utf8PathBuf::from("/project/b.js");
    let file_path_nested_b = Utf8PathBuf::from("/project/nested/b.js");

    let fs = MemoryFileSystem::default();
    fs.insert(
        file_path_a.clone(),
        "import './b.js';\nimport './nested/b.js';",
    );
    fs.insert(file_path_root_b.clone(), "import 'foo';");
    fs.insert(file_path_nested_b.clone(), "import 'foo';");

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: Some(BiomePath::new("/project")),
            configuration: Configuration {
                files: Some(FilesConfiguration {
                    includes: Some(vec![
                        NormalizedGlob::from_str("**").unwrap(),
                        NormalizedGlob::from_str("!!**/b.js").unwrap(),
                        NormalizedGlob::from_str("b.js").unwrap(),
                    ]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            extended_configurations: Default::default(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::ModulesAndTypes,
        })
        .expect("can update settings");

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: false,
        })
        .expect("can scan the project");

    assert!(workspace.is_indexed(&file_path_a));
    assert!(workspace.is_indexed(&file_path_root_b));
    assert!(!workspace.is_indexed(&file_path_nested_b));
}

#[test]
fn should_not_index_dependency_with_scan_kind_known_files() {
    let file_path = Utf8PathBuf::from("/project/a.js");

    let fs = MemoryFileSystem::default();
    fs.insert(file_path.clone(), "import 'foo';");
    fs.insert("/project/b.js".into(), "import './a.js';");

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: Some(BiomePath::new("/project")),
            configuration: Configuration {
                files: Some(FilesConfiguration {
                    includes: Some(vec![
                        NormalizedGlob::from_str("**").unwrap(),
                        NormalizedGlob::from_str("!**/a.js").unwrap(),
                    ]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            extended_configurations: Default::default(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::ModulesAndTypes,
        })
        .expect("can update settings");

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::KnownFiles,
            verbose: false,
        })
        .expect("can scan the project");

    assert!(!workspace.is_indexed(&file_path));
}

#[test]
fn should_not_index_inside_an_ignored_folder_inside_file_includes() {
    let file_path = "dist/minified.js";

    let mut fs = TemporaryFs::new("should_not_index_inside_an_ignored_folder_inside_file_includes");
    fs.create_file(file_path, "import 'foo';");

    let (workspace, project_key) = setup_workspace_and_open_project(fs.create_os(), fs.cli_path());
    let file_path = Utf8Path::new(fs.cli_path()).join(file_path);

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: Some(BiomePath::new("/project")),
            configuration: Configuration {
                files: Some(FilesConfiguration {
                    includes: Some(vec![
                        NormalizedGlob::from_str("**").unwrap(),
                        NormalizedGlob::from_str("!**/dist").unwrap(),
                    ]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            extended_configurations: Default::default(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::ModulesAndTypes,
        })
        .expect("can update settings");

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: false,
        })
        .expect("can scan the project");

    assert!(!workspace.is_indexed(&file_path));
}

#[test]
fn should_not_index_inside_an_ignored_folder_inside_vcs_ignore_file() {
    let file_path = "dist/minified.js";

    let mut fs =
        TemporaryFs::new("should_not_index_inside_an_ignored_folder_inside_vcs_ignore_file");
    fs.create_file(file_path, "import 'foo';");
    fs.create_file(".gitignore", "dist\n");

    let (workspace, project_key) = setup_workspace_and_open_project(fs.create_os(), fs.cli_path());
    let file_path = Utf8Path::new(fs.cli_path()).join(file_path);

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: Some(BiomePath::new(fs.cli_path())),
            configuration: Configuration {
                vcs: Some(VcsConfiguration {
                    use_ignore_file: Some(true.into()),
                    enabled: Some(true.into()),
                    client_kind: Some(VcsClientKind::Git),
                    ..Default::default()
                }),
                ..Default::default()
            },
            extended_configurations: Default::default(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::ModulesAndTypes,
        })
        .expect("can update settings");

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: false,
        })
        .expect("can scan the project");

    assert!(!workspace.is_indexed(&file_path));
}

#[test]
fn should_index_used_type_definition_of_used_dependency() {
    let entry_file_path = "a.js";
    let used_file_path = "node_modules/used/used.d.ts";
    let unused_file_path = "node_modules/used/unused.d.ts";

    let mut fs = TemporaryFs::new("should_index_used_type_definition_of_used_dependency");
    fs.create_file(used_file_path, "import 'foo';");
    fs.create_file(unused_file_path, "import 'foo';");
    fs.create_file(
        "node_modules/used/package.json",
        r#"{ "name": "used", "types": "./used.d.ts" }"#,
    );
    fs.create_file(entry_file_path, "import 'used';");

    let (workspace, project_key) = setup_workspace_and_open_project(fs.create_os(), fs.cli_path());
    let entry_file_path = Utf8Path::new(fs.cli_path()).join(entry_file_path);
    let used_file_path = Utf8Path::new(fs.cli_path()).join(used_file_path);
    let unused_file_path = Utf8Path::new(fs.cli_path()).join(unused_file_path);

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: false,
        })
        .expect("can scan the project");

    assert!(workspace.is_indexed(&entry_file_path));
    assert!(workspace.is_indexed(&used_file_path));
    assert!(!workspace.is_indexed(&unused_file_path));
}

/// Opens a workspace on `fs` whose root settings point `tailwind.stylesheet`
/// at `stylesheet`, relative to `/project`.
fn setup_workspace_with_tailwind_stylesheet(
    fs: MemoryFileSystem,
    stylesheet: &str,
) -> (LocalWorkspace, ProjectKey) {
    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");
    update_settings_with_tailwind_stylesheet(
        &workspace,
        project_key,
        stylesheet,
        Configuration::default(),
    );
    (workspace, project_key)
}

/// Updates the root settings of `project_key` to `configuration`, with
/// `tailwind.stylesheet` pointing at `stylesheet`, relative to `/project`.
fn update_settings_with_tailwind_stylesheet(
    workspace: &LocalWorkspace,
    project_key: ProjectKey,
    stylesheet: &str,
    configuration: Configuration,
) {
    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: Some(BiomePath::new("/project")),
            configuration: Configuration {
                tailwind: Some(TailwindConfiguration {
                    stylesheet: Some(stylesheet.into()),
                    ..Default::default()
                }),
                ..configuration
            },
            extended_configurations: Default::default(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::None,
        })
        .expect("can update settings");
}

#[test]
fn should_index_tailwind_stylesheet_and_its_imports_with_scan_kind_known_files() {
    let stylesheet_path = Utf8PathBuf::from("/project/src/app.css");
    let theme_path = Utf8PathBuf::from("/project/src/theme.css");
    let source_path = Utf8PathBuf::from("/project/src/App.jsx");

    let fs = MemoryFileSystem::default();
    fs.insert(stylesheet_path.clone(), "@import \"./theme.css\";\n");
    fs.insert(theme_path.clone(), "@theme { --color-brand: #00f; }\n");
    fs.insert(source_path.clone(), "import './theme.css';\n");

    let (workspace, project_key) = setup_workspace_with_tailwind_stylesheet(fs, "./src/app.css");

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::KnownFiles,
            verbose: false,
        })
        .expect("can scan the project");

    assert!(workspace.is_indexed(&stylesheet_path));
    assert!(workspace.is_indexed(&theme_path));
    assert!(!workspace.is_indexed(&source_path));
}

#[test]
fn should_index_tailwind_stylesheet_outside_the_scan_targets() {
    let stylesheet_path = Utf8PathBuf::from("/project/styles/app.css");

    let fs = MemoryFileSystem::default();
    fs.insert(stylesheet_path.clone(), "@theme { --color-brand: #00f; }\n");
    fs.insert(
        Utf8PathBuf::from("/project/src/App.jsx"),
        "export const a = 1;\n",
    );

    let (workspace, project_key) = setup_workspace_with_tailwind_stylesheet(fs, "./styles/app.css");

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::TargetedKnownFiles {
                target_paths: vec![BiomePath::new("/project/src")],
                descend_from_targets: true,
            },
            verbose: false,
        })
        .expect("can scan the project");

    assert!(workspace.is_indexed(&stylesheet_path));
}

#[test]
fn should_index_tailwind_stylesheet_of_nested_configuration() {
    let stylesheet_path = Utf8PathBuf::from("/project/admin/admin.css");

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/biome.json"), "{}");
    fs.insert(
        Utf8PathBuf::from("/project/admin/biome.json"),
        r#"{ "root": false, "tailwind": { "stylesheet": "./admin.css" } }"#,
    );
    fs.insert(stylesheet_path.clone(), "@theme { --color-admin: #f00; }\n");

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::KnownFiles,
            verbose: false,
        })
        .expect("can scan the project");

    assert!(workspace.is_indexed(&stylesheet_path));
}

#[test]
fn should_index_tailwind_stylesheet_in_an_ignored_folder() {
    let stylesheet_path = Utf8PathBuf::from("/project/styles/app.css");

    for scan_kind in [ScanKind::KnownFiles, ScanKind::Project] {
        let fs = MemoryFileSystem::default();
        fs.insert(stylesheet_path.clone(), "@theme { --color-brand: #00f; }\n");

        let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");
        update_settings_with_tailwind_stylesheet(
            &workspace,
            project_key,
            "./styles/app.css",
            Configuration {
                files: Some(FilesConfiguration {
                    includes: Some(vec![
                        NormalizedGlob::from_str("**").unwrap(),
                        NormalizedGlob::from_str("!**/styles").unwrap(),
                    ]),
                    ..Default::default()
                }),
                ..Default::default()
            },
        );

        workspace
            .scan_project(ScanProjectParams {
                project_key,
                watch: false,
                force: false,
                scan_kind: scan_kind.clone(),
                verbose: false,
            })
            .expect("can scan the project");

        assert!(
            workspace.is_indexed(&stylesheet_path),
            "the stylesheet should be indexed with {scan_kind:?}"
        );
    }
}

#[test]
fn should_only_accept_known_files_and_css_files_on_update_with_tailwind_stylesheet() {
    let stylesheet_path = Utf8PathBuf::from("/project/src/app.css");
    let theme_path = Utf8PathBuf::from("/project/src/theme.css");
    let source_path = Utf8PathBuf::from("/project/src/App.jsx");
    let ignored_css_path = Utf8PathBuf::from("/project/src/ignored.css");

    let fs = MemoryFileSystem::default();
    fs.insert(stylesheet_path.clone(), "@import \"./theme.css\";\n");
    fs.insert(theme_path.clone(), "@theme { --color-brand: #00f; }\n");
    fs.insert(source_path.clone(), "export const a = 1;\n");
    fs.insert(ignored_css_path.clone(), "a {}\n");

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");
    update_settings_with_tailwind_stylesheet(
        &workspace,
        project_key,
        "./src/app.css",
        Configuration {
            files: Some(FilesConfiguration {
                includes: Some(vec![
                    NormalizedGlob::from_str("**").unwrap(),
                    NormalizedGlob::from_str("!**/ignored.css").unwrap(),
                ]),
                ..Default::default()
            }),
            ..Default::default()
        },
    );

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::KnownFiles,
            verbose: false,
        })
        .expect("can scan the project");

    // The scan watches the folder of the stylesheet, which also holds the
    // source files.
    let is_ignored_on_update = |path: &Utf8Path| {
        workspace
            .is_ignored(
                project_key,
                &ScanKind::KnownFiles,
                path,
                IndexRequestKind::Explicit(IndexTrigger::Update),
                None,
            )
            .expect("can check whether the path is ignored")
    };

    assert!(is_ignored_on_update(&source_path));
    assert!(is_ignored_on_update(&ignored_css_path));

    // Removing a file from disk unloads it, so its next creation must be
    // accepted.
    for path in [&stylesheet_path, &theme_path] {
        workspace
            .unload_file(path, project_key)
            .expect("can unload the file");
        assert!(!workspace.is_indexed(path));
        assert!(!is_ignored_on_update(path), "{path} should be accepted");
    }
}
