use std::path::PathBuf;

use camino::{Utf8Path, Utf8PathBuf};
use crossbeam::channel::{Receiver, Sender, bounded, unbounded};
use notify::event::{CreateKind, ModifyKind, RemoveKind, RenameMode};
use notify::recommended_watcher;
use notify::{
    Error as NotifyError, Event as NotifyEvent, EventKind, RecursiveMode, Result as NotifyResult,
    Watcher as NotifyWatcher,
};
use tracing::{debug, warn};

use crate::WorkspaceError;
use crate::diagnostics::WatchError;
use crate::workspace::ScanKind;

use super::WorkspaceWatcherBridge;

/// Instructions to let the watcher either watch or unwatch a given folder.
#[derive(Debug, Eq, PartialEq)]
pub enum WatcherInstruction {
    WatchFolder(Utf8PathBuf, ScanKind),
    UnwatchFolder(Utf8PathBuf),

    /// Re-indexes a file after it was closed by a client.
    ReindexFile(Utf8PathBuf),

    /// Stops the watcher entirely.
    Stop,
}

/// Channel for sending instructions to the watcher.
///
/// Only exposes the sender of the channel.
///
/// Implements [Drop] so that the watcher is stopped when the channel goes out
/// of scope.
pub struct WatcherInstructionChannel {
    pub sender: Sender<WatcherInstruction>,
}

impl Drop for WatcherInstructionChannel {
    fn drop(&mut self) {
        let _ = self.sender.send(WatcherInstruction::Stop);
    }
}

/// Watcher to keep the [WorkspaceServer] in sync with the filesystem state.
///
/// Conceptually, it helps to think of the watcher as a helper to the scanner.
/// The watcher watches the same directories as those scanned by the scanner, so
/// the watcher is also instructed to watch folders that were scanned through
/// `WorkspaceServer::scan_project()`.
///
/// When watch events are received, they are handed back to the workspace. If
/// this results in opening new documents, we say they were opened by the
/// scanner, because the end result should be the same. And if we treat the
/// watcher as part of the scanner, it's not even a contradiction :)
pub struct Watcher {
    /// Internal [`notify::Watcher`] instance.
    // Note: The `Watcher` trait doesn't require its implementations to be
    //       `Send`, but it appears all platform implementations are.
    watcher: Box<dyn NotifyWatcher + Send>,

    /// Channel receiver for the events from our
    /// [internal watcher](Self::watcher).
    notify_rx: Receiver<NotifyResult<NotifyEvent>>,

    /// Channel receiver for watch instructions.
    instruction_rx: Receiver<WatcherInstruction>,
}

impl Watcher {
    /// Constructor.
    ///
    /// Returns the watcher as well as a channel for sending instructions to the
    /// watcher.
    pub fn new() -> Result<(Self, WatcherInstructionChannel), WorkspaceError> {
        // We use a bounded channel, because watchers are
        // [intrinsically unreliable](https://docs.rs/notify/latest/notify/#watching-large-directories).
        // If we block the sender, some events may get dropped, but that was
        // already a possibility. So there doesn't really seem to be a
        // justification for using an unbounded sender, which could end up
        // consuming an ever-increasing amount of memory.
        // The actual size of the buffer is an arbitrary choice that we can
        // tweak if we find a need for it.
        let (tx, rx) = bounded::<NotifyResult<NotifyEvent>>(128);

        let watcher = recommended_watcher(tx)?;

        let (instruction_tx, instruction_rx) = unbounded();
        let instruction_channel = WatcherInstructionChannel {
            sender: instruction_tx,
        };
        let watcher = Self {
            watcher: Box::new(watcher),
            notify_rx: rx,
            instruction_rx,
        };

        Ok((watcher, instruction_channel))
    }

    /// Runs the watcher.
    ///
    /// This function is expected to run continuously until either the workspace
    /// is dropped (because the workspace server is the one holding the sending
    /// end of the instructions channel) or the watcher (unexpectedly) stops.
    /// Under normal operation, neither should happen before the daemon
    /// terminates.
    #[tracing::instrument(level = "debug", skip(self, workspace))]
    pub fn run(&mut self, workspace: &impl WorkspaceWatcherBridge) {
        loop {
            crossbeam::channel::select! {
                recv(self.notify_rx) -> event => match event {
                    Ok(Ok(event)) => Self::handle_notify_event(workspace, event),
                    Ok(Err(error)) => {
                        // TODO: Improve error propagation.
                        warn!("Watcher error: {error}");
                        break;
                    },
                    Err(_) => {
                        // TODO: Improve error propagation.
                        warn!("Watcher stopped unexpectedly");
                        break;
                    }
                },
                recv(self.instruction_rx) -> instruction => match instruction {
                    Ok(WatcherInstruction::WatchFolder(path, scan_kind)) => {
                        self.watch_folder(workspace, path, scan_kind);
                    }
                    Ok(WatcherInstruction::UnwatchFolder(path)) => {
                        self.unwatch_folder(workspace, path);
                    }
                    Ok(WatcherInstruction::ReindexFile(path)) => {
                        Self::reindex_file(workspace, &path);
                    }
                    Ok(WatcherInstruction::Stop) | Err(_) => {
                        debug!("stop");
                        break; // Received stop instruction or workspace dropped.
                    }
                }
            }
        }

        workspace.notify_stopped();
    }

    #[tracing::instrument(level = "trace", skip(workspace))]
    fn handle_notify_event(workspace: &impl WorkspaceWatcherBridge, event: NotifyEvent) {
        let paths = Self::watched_paths(workspace, event.paths);
        if paths.is_empty() {
            return;
        };

        let result = match event.kind {
            EventKind::Access(_) => Ok(()),
            EventKind::Create(create_kind) => match create_kind {
                CreateKind::Folder => Self::index_folders(workspace, paths),
                _ => Self::index_paths(workspace, paths),
            },
            EventKind::Modify(modify_kind) => match modify_kind {
                ModifyKind::Name(RenameMode::From) => Self::unload_paths(workspace, paths),
                ModifyKind::Name(RenameMode::To) => Self::index_paths(workspace, paths),
                ModifyKind::Name(RenameMode::Both) => match paths.len() {
                    2 => {
                        // Good, 2 paths are expected.
                        Self::rename_path(workspace, &paths[0], &paths[1])
                    }
                    1 => {
                        // Probably either the `to` or the `from` path was
                        // filtered out, but we don't know which one, so we need
                        // to check:
                        if paths[0].exists() {
                            Self::index_paths(workspace, paths)
                        } else {
                            Self::unload_paths(workspace, paths)
                        }
                    }
                    _ => Ok(()),
                },
                // `RenameMode::Any` and `ModifyKind::Any` need to be included as a catch-all.
                // Without it, we'll miss events on Windows or macOS.
                ModifyKind::Data(_) | ModifyKind::Name(RenameMode::Any) | ModifyKind::Any => {
                    Self::index_paths(workspace, paths)
                }
                _ => Ok(()),
            },
            EventKind::Remove(remove_kind) => match remove_kind {
                RemoveKind::File => Self::unload_files(workspace, paths),
                _ => Self::unload_paths(workspace, paths),
            },
            EventKind::Any | EventKind::Other => Ok(()),
        };
        if let Err(error) = result {
            // TODO: Improve error propagation.
            warn!("Error processing watch event: {error}");
        }
    }

    /// Filters the paths to make sure only paths within watched folders remain.
    fn watched_paths(
        workspace: &impl WorkspaceWatcherBridge,
        paths: Vec<PathBuf>,
    ) -> Vec<Utf8PathBuf> {
        paths
            .into_iter()
            .filter_map(|path| {
                let path = Utf8PathBuf::from_path_buf(path).ok()?;
                workspace
                    .find_project_with_scan_kind_for_path(&path)
                    .and_then(|(project_key, scan_kind)| {
                        match workspace.is_ignored(project_key, &scan_kind, &path) {
                            Ok(is_ignored) => (!is_ignored).then_some(path),
                            Err(_) => None,
                        }
                    })
            })
            .collect()
    }

    fn index_folders(
        workspace: &impl WorkspaceWatcherBridge,
        paths: Vec<Utf8PathBuf>,
    ) -> Result<(), WorkspaceError> {
        for path in paths {
            workspace.index_folder(&path)?;
        }

        Ok(())
    }

    /// Indexes open one or more files or folders.
    ///
    /// If you already know the paths are folders, use [`Self::index_folders()`]
    /// instead.
    fn index_paths(
        workspace: &impl WorkspaceWatcherBridge,
        paths: Vec<Utf8PathBuf>,
    ) -> Result<(), WorkspaceError> {
        for path in paths {
            Self::index_path(workspace, &path)?;
        }

        Ok(())
    }

    /// Indexes an individual file or folder.
    ///
    /// If you already know the path is a folder, use [`Self::index_folder()`]
    /// instead. If you know it is a file, you should directly call
    /// [`WorkspaceWatcherBridge::index_file()`] instead.
    fn index_path(
        workspace: &impl WorkspaceWatcherBridge,
        path: &Utf8Path,
    ) -> Result<(), WorkspaceError> {
        if workspace.fs().path_is_dir(path) {
            workspace.index_folder(path)
        } else {
            let Some(project_key) = workspace.find_project_for_path(path) else {
                return Ok(()); // file events outside our projects can be safely ignored.
            };

            workspace.index_file(project_key, path)
        }
    }

    /// Unloads the given `paths` from the workspace index.
    fn unload_files(
        workspace: &impl WorkspaceWatcherBridge,
        paths: Vec<Utf8PathBuf>,
    ) -> Result<(), WorkspaceError> {
        for path in paths {
            workspace.unload_file(&path)?;
        }

        Ok(())
    }

    /// Unloads the given `paths` from the workspace index.
    ///
    /// If you already know the paths are files, use [`Self::unload_files()`]
    /// instead.
    fn unload_paths(
        workspace: &impl WorkspaceWatcherBridge,
        paths: Vec<Utf8PathBuf>,
    ) -> Result<(), WorkspaceError> {
        for path in &paths {
            workspace.unload_path(path)?;
        }

        Ok(())
    }

    fn rename_path(
        workspace: &impl WorkspaceWatcherBridge,
        from: &Utf8Path,
        to: &Utf8Path,
    ) -> Result<(), WorkspaceError> {
        workspace.unload_path(from)?;
        Self::index_path(workspace, to)
    }

    /// Reindexes an individual file if the watcher (still) has interest in it.
    #[tracing::instrument(level = "debug", skip(workspace))]
    fn reindex_file(workspace: &impl WorkspaceWatcherBridge, path: &Utf8Path) {
        let Some(project_key) = workspace.find_project_for_path(path) else {
            return; // file events outside our projects can be safely ignored.
        };

        if let Err(error) = workspace.index_file(project_key, path) {
            // TODO: Improve error propagation.
            warn!("Error re-indexing path {path}: {error}");
        }
    }

    #[tracing::instrument(level = "debug", skip(self, workspace))]
    fn watch_folder(
        &mut self,
        workspace: &impl WorkspaceWatcherBridge,
        path: Utf8PathBuf,
        scan_kind: ScanKind,
    ) {
        let std_path = path.as_std_path();
        if !workspace.insert_watched_folder(path.clone()) {
            return; // Already watching.
        }

        if let Err(error) = self.watcher.watch(std_path, RecursiveMode::NonRecursive) {
            // TODO: Improve error propagation.
            warn!("Error watching path {path}: {error}");
        }
    }

    #[tracing::instrument(level = "debug", skip(self, workspace))]
    fn unwatch_folder(&mut self, workspace: &impl WorkspaceWatcherBridge, path: Utf8PathBuf) {
        workspace.remove_watched_folders(|watched_path| {
            if watched_path.starts_with(path.as_std_path()) {
                if let Err(error) = self.watcher.unwatch(watched_path.as_std_path()) {
                    // TODO: Improve error propagation.
                    warn!("Error unwatching path {}: {error}", watched_path);
                }
                true
            } else {
                false
            }
        });
    }
}

impl From<NotifyError> for WorkspaceError {
    fn from(error: NotifyError) -> Self {
        Self::WatchError(WatchError {
            reason: error.to_string(),
        })
    }
}

#[cfg(test)]
#[path = "watcher.tests.rs"]
mod tests;
