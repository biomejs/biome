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
use biome_configuration::{Configuration, FilesConfiguration};
use biome_fs::{BiomePath, MemoryFileSystem, TemporaryFs};
use biome_glob::NormalizedGlob;
use camino::{Utf8Path, Utf8PathBuf};

use crate::scanner::IndexTrigger;
use crate::test_utils::setup_workspace_and_open_project;
use crate::workspace::{
    CloseFileParams, FileContent, GetFileContentParams, OpenFileParams, ScanProjectParams,
    UpdateSettingsParams,
};
use crate::{Workspace, WorkspaceError};

use super::{ScanKind, WorkspaceScannerBridge};

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
        })
        .expect("can also open from client");

    assert!(
        workspace.is_indexed(&file_path),
        "file should still be indexed"
    );

    workspace
        .unload_file(&file_path)
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
        .unload_file(&file_path)
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
