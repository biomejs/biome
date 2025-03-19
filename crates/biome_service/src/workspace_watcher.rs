use camino::Utf8PathBuf;
use crossbeam::channel::{Receiver, Sender, bounded, unbounded};
use notify::{
    Error as NotifyError, Event as NotifyEvent, EventKind, RecursiveMode, Result as NotifyResult,
    Watcher,
    event::{CreateKind, ModifyKind, RemoveKind, RenameMode},
    recommended_watcher,
};
use tracing::{debug, warn};

use crate::{IGNORE_ENTRIES, WorkspaceError, WorkspaceServer, diagnostics::WatchError};

/// Instructions to let the watcher either watch or unwatch a given folder.
#[derive(Debug, Eq, PartialEq)]
pub enum WatcherInstruction {
    WatchFolder(Utf8PathBuf),
    UnwatchFolder(Utf8PathBuf),

    /// Resyncs a file after a file was closed by a client.
    ///
    /// This is done through an instruction instead of calling
    /// [WorkspaceServer::open_file_by_scanner()] directly to ensure it is only
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
    AddedOrChanged,
    Removed,
}

/// Watcher to keep the [WorkspaceServer] in sync with the filesystem state.
///
/// Conceptually, it helps to think of the watcher as a helper to the scanner.
/// The watcher watches the same directories as those scanned by the scanner, so
/// the watcher is also instructed to watch folders that were scanned through
/// [WorkspaceServer::scan_project_folder()].
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
                    Ok(Ok(event)) => {
                        if event.paths.iter().all(|path| path
                            .components()
                            .any(|component| IGNORE_ENTRIES.contains(&component.as_os_str().as_encoded_bytes())))
                        {
                            continue;
                        }

                        if !matches!(event.kind, EventKind::Access(_)) {
                            debug!(event = debug(&event), "watcher_event");
                        }

                        let result = match event.kind {
                            EventKind::Access(_) => Ok(()),
                            EventKind::Create(create_kind) => match create_kind {
                                CreateKind::Folder => {
                                    workspace.open_folders_through_watcher(event.paths)
                                }
                                _ => workspace.open_paths_through_watcher(event.paths),
                            },
                            EventKind::Modify(modify_kind) => match modify_kind {
                                // `ModifyKind::Any` needs to be included as a catch-all.
                                // Without it, we'll miss events on Windows.
                                ModifyKind::Data(_) | ModifyKind::Any => {
                                    workspace.open_paths_through_watcher(event.paths)
                                },
                                ModifyKind::Name(RenameMode::From) => {
                                    workspace.close_paths_through_watcher(event.paths)
                                }
                                ModifyKind::Name(RenameMode::To) => {
                                    workspace.open_paths_through_watcher(event.paths)
                                },
                                ModifyKind::Name(RenameMode::Both) => {
                                    workspace.rename_path_through_watcher(
                                        &event.paths[0],
                                        &event.paths[1]
                                    )
                                },
                                _ => Ok(()),
                            },
                            EventKind::Remove(remove_kind) => match remove_kind {
                                RemoveKind::File => workspace.close_files_through_watcher(event.paths),
                                _ => workspace.close_paths_through_watcher(event.paths),
                            },
                            EventKind::Any | EventKind::Other => Ok(()),
                        };
                        if let Err(error) = result {
                            // TODO: Improve error propagation.
                            warn!("Error processing watch event: {error}");
                        }
                    },
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
                    Ok(WatcherInstruction::WatchFolder(path)) => {
                        debug!(%path, "watch_folder");
                        if let Err(error) = self.watcher.watch(path.as_std_path(), RecursiveMode::Recursive) {
                            // TODO: Improve error propagation.
                            warn!("Error watching path {path}: {error}");
                        }
                    }
                    Ok(WatcherInstruction::UnwatchFolder(path)) => {
                        debug!(%path, "unwatch_folder");
                        if let Err(error) = self.watcher.unwatch(path.as_std_path()) {
                            // TODO: Improve error propagation.
                            warn!("Error unwatching path {path}: {error}");
                        }
                    }
                    Ok(WatcherInstruction::ResyncFile(path)) => {
                        debug!(%path, "resync_file");
                        if let Err(error) = workspace.open_path_through_watcher(&path) {
                            // TODO: Improve error propagation.
                            warn!("Error resyncing file {path}: {error}");
                        }
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
}

impl From<NotifyError> for WorkspaceError {
    fn from(error: NotifyError) -> Self {
        Self::WatchError(WatchError {
            reason: error.to_string(),
        })
    }
}
