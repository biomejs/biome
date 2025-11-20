use std::sync::Arc;

use biome_fs::{BiomePath, FileSystem};
use crossbeam::channel::{Receiver, unbounded};
use tokio::sync::watch;

use crate::projects::ProjectKey;
use crate::workspace::{OpenProjectParams, OpenProjectResult, ServiceNotification};
use crate::{WatcherInstruction, Workspace, WorkspaceServer};

/// Convenience call for setting up the workspace and opening a project.
///
/// Uses the given `fs` and `project_path`. Does not set up a real watcher.
pub(super) fn setup_workspace_and_open_project(
    fs: impl FileSystem + 'static,
    project_path: &str,
) -> (WorkspaceServer, ProjectKey) {
    let (workspace, project_key, ..) =
        setup_workspace_and_open_project_and_get_watcher_instruction_receiver(fs, project_path);
    (workspace, project_key)
}

/// Convenience call for setting up the workspace, opening a project, and
/// getting a receiver for watcher instructions.
///
/// Watcher instructions are those that would be sent *to* the watcher. No real
/// watcher is set up.
///
/// Uses the given `fs` and `project_path`.
pub(super) fn setup_workspace_and_open_project_and_get_watcher_instruction_receiver(
    fs: impl FileSystem + 'static,
    project_path: &str,
) -> (WorkspaceServer, ProjectKey, Receiver<WatcherInstruction>) {
    let (watcher_tx, watcher_rx) = unbounded();
    let (service_tx, _) = watch::channel(ServiceNotification::IndexUpdated);
    let workspace = WorkspaceServer::new(Arc::new(fs), watcher_tx, service_tx, None);
    let OpenProjectResult { project_key } = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new(project_path),
            open_uninitialized: true,
        })
        .expect("can open project");

    (workspace, project_key, watcher_rx)
}
