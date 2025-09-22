use std::sync::mpsc;

use crate::projects::ProjectKey;
use crate::{WatcherInstruction, WorkspaceError};
use biome_diagnostics::serde::Diagnostic;
use biome_fs::{BiomePath, FileSystem};
use camino::{Utf8Path, Utf8PathBuf};
use papaya::HashSet;
use rustc_hash::{FxBuildHasher, FxHashSet};

use super::watcher::WatcherInstructionChannel;
use super::{ScanKind, WorkspaceWatcherBridge};

/// Mock bridge that can be used for testing the watcher.
///
pub(super) struct MockWorkspaceWatcherBridge<'a> {
    fs: &'a dyn FileSystem,
    project_key: ProjectKey,
    scan_kind: ScanKind,
    tx: mpsc::Sender<()>,

    /// Files that are ignored by the workspace.
    ///
    /// Exposed to allow tests to refuse watcher requests.
    pub ignored_paths: FxHashSet<Utf8PathBuf>,

    /// Files that the watcher requested to be indexed.
    ///
    /// Exposed in order to observe watcher side-effects.
    pub indexed_files: HashSet<Utf8PathBuf, FxBuildHasher>,

    /// Folders that the watcher requested to be indexed.
    ///
    /// Exposed in order to observe watcher side-effects.
    pub indexed_folders: HashSet<Utf8PathBuf, FxBuildHasher>,

    /// Folders that the watcher registered as being watched.
    ///
    /// Exposed in order to observe watcher side-effects.
    pub watched_folders: HashSet<Utf8PathBuf, FxBuildHasher>,
}

impl<'a> MockWorkspaceWatcherBridge<'a> {
    /// Creates a new instance of the mock bridge.
    ///
    /// Only a single project is "opened" in the workspace it pretends to
    /// represent, and it pretends to be scanned with the given `scan_kind`.
    ///
    /// Besides the bridge itself, this function also returns a receiver for
    /// notifications from the watcher.
    pub fn new(
        fs: &'a dyn FileSystem,
        project_key: ProjectKey,
        scan_kind: ScanKind,
    ) -> (Self, mpsc::Receiver<()>) {
        let (tx, rx) = mpsc::channel();

        let this = Self {
            fs,
            project_key,
            scan_kind,
            tx,
            ignored_paths: Default::default(),
            indexed_files: Default::default(),
            indexed_folders: Default::default(),
            watched_folders: Default::default(),
        };
        (this, rx)
    }
}

impl WorkspaceWatcherBridge for MockWorkspaceWatcherBridge<'_> {
    fn fs(&self) -> &dyn FileSystem {
        self.fs
    }

    fn find_project_for_path(&self, _path: &Utf8Path) -> Option<ProjectKey> {
        Some(self.project_key)
    }

    fn find_project_with_scan_kind_for_path(
        &self,
        _path: &Utf8Path,
    ) -> Option<(ProjectKey, ScanKind)> {
        Some((self.project_key, self.scan_kind.clone()))
    }

    fn is_ignored(
        &self,
        project_key: ProjectKey,
        scan_kind: &ScanKind,
        path: &Utf8Path,
    ) -> Result<bool, WorkspaceError> {
        assert_eq!(project_key, self.project_key);
        assert_eq!(*scan_kind, self.scan_kind);

        Ok(self.ignored_paths.contains(path))
    }

    fn index_file(
        &self,
        project_key: ProjectKey,
        path: impl Into<BiomePath>,
    ) -> Result<Vec<Diagnostic>, WorkspaceError> {
        assert_eq!(project_key, self.project_key);

        self.indexed_files.pin().insert(path.into().into());
        self.tx.send(()).expect("can send notification");

        Ok(vec![])
    }

    fn index_folder(&self, path: &Utf8Path) -> Result<Vec<Diagnostic>, WorkspaceError> {
        self.indexed_folders.pin().insert(path.to_path_buf());
        self.tx.send(()).expect("can send notification");

        Ok(vec![])
    }

    fn insert_watched_folder(&self, path: Utf8PathBuf) -> bool {
        let newly_watched = self.watched_folders.pin().insert(path);
        self.tx.send(()).expect("can send notification");
        newly_watched
    }

    fn remove_watched_folders(&self, mut callback: impl FnMut(&Utf8Path) -> bool) {
        self.watched_folders.pin().retain(|path| !callback(path));
        self.tx.send(()).expect("can send notification");
    }

    fn unload_file(&self, path: &Utf8Path) -> Result<Vec<Diagnostic>, WorkspaceError> {
        self.indexed_files.pin().remove(path);
        self.tx.send(()).expect("can send notification");

        Ok(vec![])
    }

    fn unload_path(&self, path: &Utf8Path) -> Result<Vec<Diagnostic>, WorkspaceError> {
        self.indexed_files.pin().remove(path);
        self.indexed_folders.pin().remove(path);
        self.tx.send(()).expect("can send notification");

        Ok(vec![])
    }

    fn notify_stopped(&self) {}
}

/// Utility to send [`WatcherInstruction::Stop`] to the watcher on `Drop`.
///
/// This is useful to guarantee tests that instantiate the watcher will
/// terminate when an assertion fails and the test is aborted early.
pub struct StopSender(pub WatcherInstructionChannel);

impl Drop for StopSender {
    fn drop(&mut self) {
        self.0
            .sender
            .send(WatcherInstruction::Stop)
            .expect("can send stop instruction");
    }
}
