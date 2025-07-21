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
use biome_fs::{BiomePath, MemoryFileSystem};
use biome_glob::NormalizedGlob;
use camino::Utf8PathBuf;

use crate::test_utils::setup_workspace_and_open_project;
use crate::workspace::{
    CloseFileParams, FileContent, GetFileContentParams, OpenFileParams, UpdateSettingsParams,
};
use crate::{Workspace, WorkspaceError};

use super::{IndexTrigger, ScanKind, WorkspaceScannerBridge};

#[test]
fn close_file_through_watcher_before_client() {
    const FILE_CONTENT: &str = "import 'foo';";

    let file_path = Utf8PathBuf::from("/project/a.js");

    let fs = MemoryFileSystem::default();
    fs.insert(file_path.clone(), FILE_CONTENT);

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");

    workspace
        .index_file(
            project_key,
            &ScanKind::Project,
            &file_path,
            IndexTrigger::WatcherUpdate,
        )
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
        .index_file(
            project_key,
            &ScanKind::Project,
            &file_path,
            IndexTrigger::WatcherUpdate,
        )
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
        .index_file(
            project_key,
            &ScanKind::KnownFiles,
            &file_path,
            IndexTrigger::InitialScan,
        )
        .expect("can request file to be indexed");

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
        .index_file(
            project_key,
            &ScanKind::Project,
            &file_path,
            IndexTrigger::InitialScan,
        )
        .expect("can request file to be indexed");

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
                    includes: Some(vec![NormalizedGlob::from_str("!**/dist/**").unwrap()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
        })
        .expect("can update settings");

    workspace
        .index_file(
            project_key,
            &ScanKind::Project,
            &file_path,
            IndexTrigger::InitialScan,
        )
        .expect("can request file to be indexed");

    assert!(!workspace.is_indexed(&file_path));
}
