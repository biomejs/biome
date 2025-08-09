//! Tests for the methods of [`WorkspaceWatcherBridge`].
//!
//! These tests mainly verify that watcher instructions get sent, and that
//! watcher requests for indexing are respected.
//!
//! No real watcher is instantiated in these tests.

use std::collections::BTreeMap;

use biome_fs::{BiomePath, MemoryFileSystem};
use camino::Utf8PathBuf;

use crate::scanner::workspace_bridges::ScannerWatcherBridge;
use crate::test_utils::setup_workspace_and_open_project_and_get_watcher_instruction_receiver;
use crate::workspace::{
    ChangeFileParams, CloseFileParams, FileContent, GetFileContentParams, GetModuleGraphParams,
    OpenFileParams,
};
use crate::{WatcherInstruction, Workspace};

use super::WorkspaceWatcherBridge;

#[test]
fn close_modified_file_from_client_before_watcher() {
    const FILE_CONTENT: &str = "import 'foo';";
    const FILE_CONTENT_MODIFIED: &str = "import 'fooo';";

    let file_path = Utf8PathBuf::from("/project/a.js");

    let fs = MemoryFileSystem::default();
    fs.insert(file_path.clone(), FILE_CONTENT);

    let (workspace, project_key, watcher_rx) =
        setup_workspace_and_open_project_and_get_watcher_instruction_receiver(fs, "/project");

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

    ScannerWatcherBridge::new((&workspace.scanner, &workspace))
        .index_file(project_key, file_path.clone())
        .expect("can also index file");

    workspace
        .change_file(ChangeFileParams {
            project_key,
            path: BiomePath::new(&file_path),
            content: FILE_CONTENT_MODIFIED.to_string(),
            version: 2,
        })
        .expect("can change file");

    let content = workspace
        .get_file_content(GetFileContentParams {
            project_key,
            path: BiomePath::new(&file_path),
        })
        .expect("can get file content");

    assert_eq!(content, FILE_CONTENT_MODIFIED);

    let module_graph = workspace
        .get_module_graph(GetModuleGraphParams {})
        .expect("can get module graph");

    assert_eq!(
        module_graph
            .data
            .get(file_path.as_str())
            .map(|module_info| module_info.static_import_paths.clone()),
        Some(BTreeMap::from([("fooo".to_string(), "fooo".to_string())])),
        "index should've updated to the client state"
    );

    workspace
        .close_file(CloseFileParams {
            project_key,
            path: BiomePath::new(&file_path),
        })
        .expect("can close file from client");

    let instruction = watcher_rx
        .try_recv()
        .expect("instruction should be received");
    assert_eq!(
        instruction,
        WatcherInstruction::ReindexFile(file_path.clone())
    );
    // call the instruction handler manually for the sake of the test:
    ScannerWatcherBridge::new((&workspace.scanner, &workspace))
        .index_file(project_key, file_path.clone())
        .expect("path to be updated by us");

    let module_graph = workspace
        .get_module_graph(GetModuleGraphParams {})
        .expect("can get module graph");

    assert_eq!(
        module_graph
            .data
            .get(file_path.as_str())
            .map(|module_info| module_info.static_import_paths.clone()),
        Some(BTreeMap::from([("foo".to_string(), "foo".to_string())])),
        "index should've reverted to the filesystem state"
    );
}
