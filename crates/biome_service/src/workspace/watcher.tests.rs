use biome_fs::{BiomePath, MemoryFileSystem};
use camino::Utf8PathBuf;
use crossbeam::channel::unbounded;
use tokio::sync::watch;

use super::*;
use crate::workspace::OpenProjectResult;
use crate::{
    WatcherInstruction,
    workspace::{
        ChangeFileParams, CloseFileParams, FileContent, GetFileContentParams, OpenFileParams,
        OpenProjectParams, ServiceDataNotification,
    },
};

#[test]
fn close_file_through_watcher_before_client() {
    const FILE_CONTENT: &str = "import 'foo';";

    let mut fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/a.js"), FILE_CONTENT);

    let (watcher_tx, _) = unbounded();
    let (service_data_tx, _) = watch::channel(ServiceDataNotification::Updated);
    let workspace = WorkspaceServer::new(Box::new(fs), watcher_tx, service_data_tx, None);
    let OpenProjectResult { project_key, .. } = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new("/project"),
            open_uninitialized: true,
            skip_rules: None,
            only_rules: None,
        })
        .expect("can open project");

    workspace
        .open_file_during_initial_scan(project_key, BiomePath::new("/project/a.js"))
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

    let mut fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/a.js"), FILE_CONTENT);

    let (watcher_tx, _) = unbounded();
    let (service_data_tx, _) = watch::channel(ServiceDataNotification::Updated);
    let workspace = WorkspaceServer::new(Box::new(fs), watcher_tx, service_data_tx, None);
    let OpenProjectResult { project_key, .. } = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new("/project"),
            open_uninitialized: true,
            skip_rules: None,
            only_rules: None,
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

    let mut fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/a.js"), FILE_CONTENT);

    let (watcher_tx, rx) = unbounded();
    let (service_data_tx, _) = watch::channel(ServiceDataNotification::Updated);
    let workspace = WorkspaceServer::new(Box::new(fs), watcher_tx, service_data_tx, None);
    let OpenProjectResult { project_key, .. } = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new("/project"),
            open_uninitialized: true,
            skip_rules: None,
            only_rules: None,
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
        .open_path_through_watcher(Utf8Path::new("/project/a.js"))
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
