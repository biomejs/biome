use std::{str::FromStr, sync::Arc};

use biome_configuration::vcs::{VcsClientKind, VcsConfiguration};
use biome_configuration::{Configuration, FilesConfiguration};
use biome_fs::{BiomePath, MemoryFileSystem};
use biome_glob::NormalizedGlob;
use camino::Utf8PathBuf;
use crossbeam::channel::unbounded;
use tokio::sync::watch;

use crate::{
    WatcherInstruction,
    workspace::{
        ChangeFileParams, CloseFileParams, FileContent, FormatFileParams, GetFileContentParams,
        OpenFileParams, OpenProjectParams, OpenProjectResult, ServiceDataNotification,
        UpdateSettingsParams,
    },
};

use super::*;

#[test]
fn should_not_open_an_ignored_file_inside_vcs_ignore_file() {
    const FILE_CONTENT: &str = "import 'foo';";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/a.js"), FILE_CONTENT);
    fs.insert(Utf8PathBuf::from("/project/.gitignore"), "a.js\n");

    let (watcher_tx, _) = unbounded();
    let (service_data_tx, _) = watch::channel(ServiceDataNotification::Updated);
    let workspace = WorkspaceServer::new(Arc::new(fs), watcher_tx, service_data_tx, None);
    let OpenProjectResult { project_key } = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new("/project"),
            open_uninitialized: true,
        })
        .expect("can open project");

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
        .open_path_through_watcher(Utf8Path::new("/project/a.js"), &ScanKind::KnownFiles)
        .expect("can open file");

    let result = workspace.format_file(FormatFileParams {
        path: BiomePath::new("/project/a.js"),
        project_key,
    });

    assert!(result.is_err());
}

#[test]
fn should_not_open_an_ignored_file_inside_file_includes() {
    const FILE_CONTENT: &str = "import 'foo';";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/a.js"), FILE_CONTENT);
    fs.insert(Utf8PathBuf::from("/project/dist/minified.js"), FILE_CONTENT);

    let (watcher_tx, _) = unbounded();
    let (service_data_tx, _) = watch::channel(ServiceDataNotification::Updated);
    let workspace = WorkspaceServer::new(Arc::new(fs), watcher_tx, service_data_tx, None);
    let OpenProjectResult { project_key } = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new("/project"),
            open_uninitialized: true,
        })
        .expect("can open project");

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
        .open_path_through_watcher(
            Utf8Path::new("/project/dist/minified.js"),
            &ScanKind::KnownFiles,
        )
        .expect("can open file");

    let result = workspace.format_file(FormatFileParams {
        path: BiomePath::new("/project/dist/minified.js"),
        project_key,
    });

    assert!(result.is_err());
}

#[test]
fn close_file_through_watcher_before_client() {
    const FILE_CONTENT: &str = "import 'foo';";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/a.js"), FILE_CONTENT);

    let (watcher_tx, _) = unbounded();
    let (service_data_tx, _) = watch::channel(ServiceDataNotification::Updated);
    let workspace = WorkspaceServer::new(Arc::new(fs), watcher_tx, service_data_tx, None);
    let OpenProjectResult { project_key } = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new("/project"),
            open_uninitialized: true,
        })
        .expect("can open project");

    workspace
        .open_path_through_watcher(Utf8Path::new("/project/a.js"), &ScanKind::Project)
        .expect("can open file");

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new("/project/a.js"),
            content: FileContent::FromClient {
                content: FILE_CONTENT.to_string(),
                version: 1,
            },
            document_file_source: None,
            persist_node_cache: true,
        })
        .expect("can also open from client");

    workspace
        .close_file_through_watcher(Utf8Path::new("/project/a.js"))
        .expect("can close file through watcher");

    let content = workspace
        .get_file_content(GetFileContentParams {
            project_key,
            path: BiomePath::new("/project/a.js"),
        })
        .expect("can still get file content");

    assert_eq!(content, FILE_CONTENT);

    workspace
        .close_file(CloseFileParams {
            project_key,
            path: BiomePath::new("/project/a.js"),
        })
        .expect("can close file from client");

    let error = workspace
        .get_file_content(GetFileContentParams {
            project_key,
            path: BiomePath::new("/project/a.js"),
        })
        .expect_err("can no longer get file content");

    assert!(matches!(error, WorkspaceError::NotFound(_)));
}

#[test]
fn close_file_from_client_before_watcher() {
    const FILE_CONTENT: &str = "import 'foo';";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/a.js"), FILE_CONTENT);

    let (watcher_tx, _) = unbounded();
    let (service_data_tx, _) = watch::channel(ServiceDataNotification::Updated);
    let workspace = WorkspaceServer::new(Arc::new(fs), watcher_tx, service_data_tx, None);
    let OpenProjectResult { project_key } = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new("/project"),
            open_uninitialized: true,
        })
        .expect("can open project");

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new("/project/a.js"),
            content: FileContent::FromClient {
                content: FILE_CONTENT.to_string(),
                version: 1,
            },
            document_file_source: None,
            persist_node_cache: true,
        })
        .expect("can also open from client");

    workspace
        .open_file_during_initial_scan(project_key, BiomePath::new("/project/a.js"))
        .expect("can open file");

    workspace
        .close_file(CloseFileParams {
            project_key,
            path: BiomePath::new("/project/a.js"),
        })
        .expect("can close file from client");

    let content = workspace
        .get_file_content(GetFileContentParams {
            project_key,
            path: BiomePath::new("/project/a.js"),
        })
        .expect("can still get file content");

    assert_eq!(content, FILE_CONTENT);

    workspace
        .close_file_through_watcher(Utf8Path::new("/project/a.js"))
        .expect("can close file through watcher");

    let error = workspace
        .get_file_content(GetFileContentParams {
            project_key,
            path: BiomePath::new("/project/a.js"),
        })
        .expect_err("can no longer get file content");

    assert!(matches!(error, WorkspaceError::NotFound(_)));
}

#[test]
fn close_modified_file_from_client_before_watcher() {
    const FILE_CONTENT: &str = "import 'foo';";
    const FILE_CONTENT_MODIFIED: &str = "import 'fooo';";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/a.js"), FILE_CONTENT);

    let (watcher_tx, rx) = unbounded();
    let (service_data_tx, _) = watch::channel(ServiceDataNotification::Updated);
    let workspace = WorkspaceServer::new(Arc::new(fs), watcher_tx, service_data_tx, None);
    let OpenProjectResult { project_key } = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new("/project"),
            open_uninitialized: true,
        })
        .expect("can open project");

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new("/project/a.js"),
            content: FileContent::FromClient {
                content: FILE_CONTENT.to_string(),
                version: 1,
            },
            document_file_source: None,
            persist_node_cache: true,
        })
        .expect("can also open from client");

    workspace
        .open_file_during_initial_scan(project_key, BiomePath::new("/project/a.js"))
        .expect("can open file");

    workspace
        .change_file(ChangeFileParams {
            project_key,
            path: BiomePath::new("/project/a.js"),
            content: FILE_CONTENT_MODIFIED.to_string(),
            version: 2,
        })
        .expect("can change file");

    let content = workspace
        .get_file_content(GetFileContentParams {
            project_key,
            path: BiomePath::new("/project/a.js"),
        })
        .expect("can get file content");

    assert_eq!(content, FILE_CONTENT_MODIFIED);

    workspace
        .close_file(CloseFileParams {
            project_key,
            path: BiomePath::new("/project/a.js"),
        })
        .expect("can close file from client");

    let instruction = rx.try_recv().expect("instruction should be received");
    assert_eq!(
        instruction,
        WatcherInstruction::ResyncFile(Utf8PathBuf::from("/project/a.js"))
    );
    // call the instruction handler manually for the sake of the test:
    workspace
        .open_path_through_watcher(Utf8Path::new("/project/a.js"), &ScanKind::Project)
        .expect("path to be updated by us");

    let content = workspace
        .get_file_content(GetFileContentParams {
            project_key,
            path: BiomePath::new("/project/a.js"),
        })
        .expect("can still get file content");

    // Content should've reverted to the filesystem state.
    assert_eq!(content, FILE_CONTENT);
}

#[test]
fn should_not_open_a_source_file_with_scan_kind_known_files() {
    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/a.js"), "import 'foo';");

    let (watcher_tx, watcher_rx) = unbounded();
    let (service_data_tx, _) = watch::channel(ServiceDataNotification::Updated);
    let workspace = WorkspaceServer::new(Arc::new(fs), watcher_tx, service_data_tx, None);
    let OpenProjectResult { project_key } = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new("/project"),
            open_uninitialized: true,
        })
        .expect("can open project");

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: Some(BiomePath::new("/project")),
            configuration: Configuration::default(),
        })
        .expect("can update settings");

    workspace
        .open_path_through_watcher(Utf8Path::new("/project/a.js"), &ScanKind::KnownFiles)
        .expect("can open file");

    let result = workspace.get_file_content(GetFileContentParams {
        project_key,
        path: BiomePath::new("/project/a.js"),
    });
    assert!(result.is_err());

    let result = watcher_rx.try_recv();
    assert!(result.is_err(), "no watcher notification expected");
}
