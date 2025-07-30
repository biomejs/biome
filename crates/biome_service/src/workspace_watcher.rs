use std::path::{Path, PathBuf};

use camino::Utf8PathBuf;
use crossbeam::channel::{Receiver, Sender, bounded, unbounded};
use notify::{
    Error as NotifyError, Event as NotifyEvent, EventKind, RecursiveMode, Result as NotifyResult,
    Watcher,
    event::{CreateKind, ModifyKind, RemoveKind, RenameMode},
    recommended_watcher,
};
use rustc_hash::FxHashMap;
use tracing::{debug, warn};

use crate::{WorkspaceError, WorkspaceServer, diagnostics::WatchError, workspace::ScanKind};

/// Instructions to let the watcher either watch or unwatch a given folder.
#[derive(Debug, Eq, PartialEq)]
pub enum WatcherInstruction {
    WatchFolder(Utf8PathBuf, ScanKind),
    UnwatchFolder(Utf8PathBuf),

    /// Resyncs a file after a file was closed by a client.
    ///
    /// This is done through an instruction instead of calling
    /// `WorkspaceServer::open_file_by_watcher()` directly to ensure it is only
    /// done if the watcher is active.
    ResyncFile(Utf8PathBuf),

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

/// Kind of change being reported.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WatcherSignalKind {
    AddedOrChanged(OpenFileReason),
    Removed,
}

/// Reports the reason why a file is being opened/indexed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpenFileReason {
    /// A workspace client has explicitly requested the file to be opened.
    ClientRequest,

    /// The file is being opened as part of an initial scanner run.
    InitialScan,

    /// The file is being opened or updated as part of a watcher update.
    WatcherUpdate,
}

impl OpenFileReason {
    pub const fn is_opened_by_scanner(self) -> bool {
        matches!(self, Self::InitialScan | Self::WatcherUpdate)
    }
}

/// Watcher to keep the [WorkspaceServer] in sync with the filesystem state.
///
/// Conceptually, it helps to think of the watcher as a helper to the scanner.
/// The watcher watches the same directories as those scanned by the scanner, so
/// the watcher is also instructed to watch folders that were scanned through
/// `WorkspaceServer::scan_project_folder()`.
///
/// When watch events are received, they are handed back to the workspace. If
/// this results in opening new documents, we say they were opened by the
/// scanner, because the end result should be the same. And if we treat the
/// watcher as part of the scanner, it's not even a contradiction :)
pub struct WorkspaceWatcher {
    /// Internal [`notify::Watcher`] instance.
    // Note: The `Watcher` trait doesn't require its implementations to be
    //       `Send`, but it appears all platform implementations are.
    watcher: Box<dyn Watcher + Send>,

    /// Tracks the folders we are watching, including the scan kind per folder.
    watched_folders: FxHashMap<PathBuf, ScanKind>,

    /// Channel receiver for the events from our
    /// [internal watcher](Self::watcher).
    notify_rx: Receiver<NotifyResult<NotifyEvent>>,

    /// Channel receiver for watch instructions.
    instruction_rx: Receiver<WatcherInstruction>,
}

impl WorkspaceWatcher {
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
            watched_folders: Default::default(),
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
    pub fn run(&mut self, workspace: &WorkspaceServer) {
        loop {
            crossbeam::channel::select! {
                recv(self.notify_rx) -> event => match event {
                    Ok(Ok(event)) => self.handle_notify_event(event, workspace),
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
                        self.watch_folder(path, scan_kind);
                    }
                    Ok(WatcherInstruction::UnwatchFolder(path)) => {
                        self.unwatch_folder(path);
                    }
                    Ok(WatcherInstruction::ResyncFile(path)) => {
                        self.resync_file(path, workspace);
                    }
                    Ok(WatcherInstruction::Stop) | Err(_) => {
                        debug!("stop");
                        break; // Received stop instruction or workspace dropped.
                    }
                }
            }
        }

        workspace.watcher_stopped();
    }

    #[tracing::instrument(level = "trace", skip(self, workspace))]
    fn handle_notify_event(&self, event: NotifyEvent, workspace: &WorkspaceServer) {
        let Some((paths, scan_kind)) = self.paths_with_scan_kind(event.paths) else {
            return;
        };

        let paths = workspace.filter_paths_for_watcher(paths, &scan_kind);
        if paths.is_empty() {
            return;
        };

        let result = match event.kind {
            EventKind::Access(_) => Ok(()),
            EventKind::Create(create_kind) => match create_kind {
                CreateKind::Folder => workspace.open_folders_through_watcher(paths, &scan_kind),
                _ => workspace.open_paths_through_watcher(paths, &scan_kind),
            },
            EventKind::Modify(modify_kind) => match modify_kind {
                ModifyKind::Name(RenameMode::From) => workspace.close_paths_through_watcher(paths),
                ModifyKind::Name(RenameMode::To) => {
                    workspace.open_paths_through_watcher(paths, &scan_kind)
                }
                ModifyKind::Name(RenameMode::Both) => match paths.len() {
                    2 => {
                        // Good, 2 paths are expected.
                        workspace.rename_path_through_watcher(&paths[0], &paths[1], &scan_kind)
                    }
                    1 => {
                        // Probably either the `to` or the `from` path was
                        // filtered out, but we don't know which one, so we need
                        // to check:
                        if paths[0].exists() {
                            workspace.open_paths_through_watcher(paths, &scan_kind)
                        } else {
                            workspace.close_paths_through_watcher(paths)
                        }
                    }
                    _ => Ok(()),
                },
                // `RenameMode::Any` and `ModifyKind::Any` need to be included as a catch-all.
                // Without it, we'll miss events on Windows or macOS.
                ModifyKind::Data(_) | ModifyKind::Name(RenameMode::Any) | ModifyKind::Any => {
                    workspace.open_paths_through_watcher(paths, &scan_kind)
                }
                _ => Ok(()),
            },
            EventKind::Remove(remove_kind) => match remove_kind {
                RemoveKind::File => workspace.close_files_through_watcher(paths),
                _ => workspace.close_paths_through_watcher(paths),
            },
            EventKind::Any | EventKind::Other => Ok(()),
        };
        if let Err(error) = result {
            // TODO: Improve error propagation.
            warn!("Error processing watch event: {error}");
        }
    }

    /// Filters the paths to make sure only paths within watched folders remain,
    /// and returns the `ScanKind` applicable.
    ///
    /// In the unlikely event that multiple paths are given that reside in
    /// different folders with different `ScanKind`s, the `ScanKind` that would
    /// scan the _most_ files is used.
    ///
    /// Returns `None` if no paths relevant to the scanner remain.
    fn paths_with_scan_kind(&self, paths: Vec<PathBuf>) -> Option<(Vec<PathBuf>, ScanKind)> {
        let mut scan_kind = ScanKind::NoScanner;
        let paths = paths
            .into_iter()
            .filter(|path| match self.scan_kind_for_path(path) {
                None => false,
                Some(scan_kind) if scan_kind == &ScanKind::NoScanner => false,
                Some(new_scan_kind) => {
                    scan_kind = match (&scan_kind, new_scan_kind) {
                        (_, &ScanKind::Project) | (&ScanKind::Project, _) => ScanKind::Project,
                        _ => ScanKind::KnownFiles,
                    };
                    true
                }
            })
            .collect();
        (scan_kind != ScanKind::NoScanner).then_some((paths, scan_kind))
    }

    #[tracing::instrument(level = "debug", skip(self, workspace))]
    fn resync_file(&self, path: Utf8PathBuf, workspace: &WorkspaceServer) {
        let Some(scan_kind) = self.scan_kind_for_path(path.as_std_path()) else {
            return;
        };

        if let Err(error) = workspace.resync_file_through_watcher(&path, scan_kind) {
            // TODO: Improve error propagation.
            warn!("Error resyncing file {path}: {error}");
        }
    }

    #[inline]
    fn scan_kind_for_path(&self, path: &Path) -> Option<&ScanKind> {
        self.watched_folders.get(path).or_else(|| {
            path.parent()
                .and_then(|dir_path| self.watched_folders.get(dir_path))
        })
    }

    #[tracing::instrument(level = "debug", skip(self))]
    fn watch_folder(&mut self, path: Utf8PathBuf, scan_kind: ScanKind) {
        let std_path = path.as_std_path();
        if self
            .watched_folders
            .insert(std_path.to_path_buf(), scan_kind)
            .is_some()
        {
            return; // Already watching.
        }

        if let Err(error) = self.watcher.watch(std_path, RecursiveMode::NonRecursive) {
            // TODO: Improve error propagation.
            warn!("Error watching path {path}: {error}");
        }
    }

    #[tracing::instrument(level = "debug", skip(self))]
    fn unwatch_folder(&mut self, path: Utf8PathBuf) {
        self.watched_folders.retain(|watched_path, _| {
            if watched_path.starts_with(path.as_std_path()) {
                if let Err(error) = self.watcher.unwatch(watched_path) {
                    // TODO: Improve error propagation.
                    warn!("Error unwatching path {}: {error}", watched_path.display());
                }
                false
            } else {
                true
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
