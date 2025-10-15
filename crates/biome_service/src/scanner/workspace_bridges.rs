use std::panic::RefUnwindSafe;

use biome_fs::FileSystem;
use biome_module_graph::{ModuleDependencies, ModuleDiagnostic};
use camino::Utf8Path;

use crate::{WorkspaceError, projects::ProjectKey, workspace::ServiceNotification};

use super::*;

/// Trait used to give access to workspace functionality required by the
/// scanner.
pub(crate) trait WorkspaceScannerBridge: Send + Sync + RefUnwindSafe {
    /// Returns a reference to the [`FileSystem`].
    fn fs(&self) -> &dyn FileSystem;

    /// Finds the [`ProjectKey`] of the project to which the given `path`
    /// belongs.
    fn find_project_for_path(&self, path: &Utf8Path) -> Option<ProjectKey>;

    /// Returns the project path for the project with the given `project_key`.
    fn get_project_path(&self, project_key: ProjectKey) -> Option<Utf8PathBuf>;

    /// Asks the workspace whether the given `path` that falls under the project
    /// with the given `project_key` is ignored, assuming the given `scan_kind`
    /// and `request_kind`.
    fn is_ignored(
        &self,
        project_key: ProjectKey,
        scan_kind: &ScanKind,
        path: &Utf8Path,
        request_kind: IndexRequestKind,
    ) -> Result<bool, WorkspaceError>;

    /// Returns whether the given `path` has been indexed.
    fn is_indexed(&self, path: &Utf8Path) -> bool;

    /// Indexes the file with the given `path` within the project with the given
    /// `project_key` in the workspace.
    ///
    /// Returns a set of dependencies that can be scanned using
    /// [`WorkspaceScannerBridge::index_dependencies()`], if desired.
    fn index_file(
        &self,
        project_key: ProjectKey,
        path: impl Into<BiomePath>,
        trigger: IndexTrigger,
    ) -> Result<(ModuleDependencies, Vec<ModuleDiagnostic>), WorkspaceError>;

    /// Informs the workspace of the list of nested config files.
    ///
    /// If a configuration file contains errors, it's not processed and the
    /// project isn't updated.
    ///
    /// It's the responsibility of the caller to process the diagnostics and
    /// handle the errors emitted by the configuration.
    ///
    /// ## Errors
    ///
    /// - A nested configuration file is a root.
    /// - Biome can't read the file.
    fn update_project_config_files(
        &self,
        project_key: ProjectKey,
        files: &[BiomePath],
    ) -> Result<Vec<Diagnostic>, WorkspaceError>;

    /// Informs the workspace of the list of ignore files.
    ///
    /// If the VCS integration is enabled, the files are read and the `Settings`
    /// are updated.
    ///
    /// ## Errors
    ///
    /// - If the project doesn't exist.
    /// - If it's not possible to read the ignore file.
    /// - If the ignore file contains lines that contain incorrect globs.
    fn update_project_ignore_files(
        &self,
        project_key: ProjectKey,
        files: &[BiomePath],
    ) -> Result<(), WorkspaceError>;

    /// Sends the given `notification` to any service notification listeners.
    fn notify(&self, notification: ServiceNotification);

    /// Unloads the index of the file with the given `path` within the
    /// workspace.
    fn unload_file(&self, path: &Utf8Path) -> Result<Vec<Diagnostic>, WorkspaceError>;

    /// Unloads the given `path` from the workspace index.
    ///
    /// Note that we don't really have a concept of open folders in the
    /// workspace, so we send a [`WatcherInstruction::UnwatchFolder`] just in
    /// case and iterate the documents to find paths that would be inside the
    /// unloaded folder.
    ///
    /// If you already know the path is a file, you should use
    /// [`WorkspaceWatcherBridge::unload_file()`] directly instead.
    fn unload_path(&self, path: &Utf8Path) -> Result<Vec<Diagnostic>, WorkspaceError>;
}

/// Trait used to give access to workspace functionality required by the
/// watcher.
pub(crate) trait WorkspaceWatcherBridge {
    /// Returns a reference to the [`FileSystem`].
    fn fs(&self) -> &dyn FileSystem;

    /// Finds the [`ProjectKey`] to use for the given `path`.
    fn find_project_for_path(&self, path: &Utf8Path) -> Option<ProjectKey>;

    /// Finds the [`ProjectKey`] and [`ScanKind`] to use for the given `path`.
    fn find_project_with_scan_kind_for_path(
        &self,
        path: &Utf8Path,
    ) -> Option<(ProjectKey, ScanKind)>;

    /// Asks the workspace whether the given `path` that falls under the project
    /// with the given `project_key` is ignored, assuming the given `scan_kind`.
    fn is_ignored(
        &self,
        project_key: ProjectKey,
        scan_kind: &ScanKind,
        path: &Utf8Path,
    ) -> Result<bool, WorkspaceError>;

    /// Indexes the file with the given `path` within the project with the given
    /// `project_key` in the workspace.
    ///
    /// If relevant to the given `scan_kind`, dependencies of the file are
    /// indexed as well.
    fn index_file(
        &self,
        project_key: ProjectKey,
        path: impl Into<BiomePath>,
    ) -> Result<Vec<Diagnostic>, WorkspaceError>;

    /// Scans and watches the folder with the given `path` within the project
    /// with the given `project_key` in the workspace, while indexing all the
    /// files relevant for the given `scan_kind`.
    ///
    /// If relevant to the given `scan_kind`, dependencies of the files in the
    /// folder are indexed as well.
    fn index_folder(&self, path: &Utf8Path) -> Result<Vec<Diagnostic>, WorkspaceError>;

    /// Informs the workspace the given folder is watched.
    ///
    /// Returns `true` if the folder was not yet watched, `false` otherwise.
    fn insert_watched_folder(&self, path: Utf8PathBuf) -> bool;

    /// Removes watched folders from the workspace.
    ///
    /// `callback` is invoked for every currently watched folder with the path
    /// of said folder. Folders for which the `callback` returns `true` are
    /// removed, others are retained.
    fn remove_watched_folders(&self, callback: impl FnMut(&Utf8Path) -> bool);

    /// Unloads the index of the file with the given `path` within the
    /// workspace.
    fn unload_file(&self, path: &Utf8Path) -> Result<Vec<Diagnostic>, WorkspaceError>;

    /// Unloads the given `path` from the workspace index.
    ///
    /// Note that we don't really have a concept of open folders in the
    /// workspace, so we send a [`WatcherInstruction::UnwatchFolder`] just in
    /// case and iterate the documents to find paths that would be inside the
    /// unloaded folder.
    ///
    /// If you already know the path is a file, you should use
    /// [`WorkspaceWatcherBridge::unload_file()`] directly instead.
    fn unload_path(&self, path: &Utf8Path) -> Result<Vec<Diagnostic>, WorkspaceError>;

    /// Notifies service notification listeners that the watcher has stopped.
    fn notify_stopped(&self);
}

/// Bridge used to implement `WorkspaceWatcherBridge` on top of the scanner.
///
/// This creates a bit of duplication and indirection, compared to implementing
/// it on the workspace directly, but it forces us to consider how we export
/// workspace functionality a bit more carefully. We want the functionality
/// exposed to the watcher to align with what's exposed to the scanner in
/// general, so this is very much intentional.
pub(crate) struct ScannerWatcherBridge<'a, W: WorkspaceScannerBridge> {
    scanner: &'a Scanner,
    workspace: &'a W,
}

impl<'a, W: WorkspaceScannerBridge> ScannerWatcherBridge<'a, W> {
    pub fn new((scanner, workspace): (&'a Scanner, &'a W)) -> Self {
        Self { scanner, workspace }
    }
}

impl<W> WorkspaceWatcherBridge for ScannerWatcherBridge<'_, W>
where
    W: WorkspaceScannerBridge,
{
    #[inline]
    fn fs(&self) -> &dyn FileSystem {
        self.workspace.fs()
    }

    #[inline]
    fn find_project_for_path(&self, path: &Utf8Path) -> Option<ProjectKey> {
        self.workspace.find_project_for_path(path)
    }

    #[inline]
    fn find_project_with_scan_kind_for_path(
        &self,
        path: &Utf8Path,
    ) -> Option<(ProjectKey, ScanKind)> {
        self.workspace
            .find_project_for_path(path)
            .and_then(|project_key| {
                self.scanner
                    .get_scan_kind_for_project(project_key)
                    .map(|scan_kind| (project_key, scan_kind))
            })
    }

    #[inline]
    fn is_ignored(
        &self,
        project_key: ProjectKey,
        scan_kind: &ScanKind,
        path: &Utf8Path,
    ) -> Result<bool, WorkspaceError> {
        self.workspace.is_ignored(
            project_key,
            scan_kind,
            path,
            IndexRequestKind::Explicit(IndexTrigger::Update),
        )
    }

    fn index_file(
        &self,
        project_key: ProjectKey,
        path: impl Into<BiomePath>,
    ) -> Result<Vec<Diagnostic>, WorkspaceError> {
        self.workspace
            .index_file(project_key, path, IndexTrigger::Update)
            .map(|(_, diagnostics)| {
                diagnostics
                    .into_iter()
                    .map(biome_diagnostics::serde::Diagnostic::new)
                    .collect::<Vec<_>>()
            })
    }

    #[inline]
    fn index_folder(&self, path: &Utf8Path) -> Result<Vec<Diagnostic>, WorkspaceError> {
        let Some(project_key) = self.find_project_for_path(path) else {
            return Ok(vec![]); // file events outside our projects can be safely ignored.
        };

        self.scanner.index_folder(self.workspace, project_key, path)
    }

    #[inline]
    fn insert_watched_folder(&self, path: Utf8PathBuf) -> bool {
        self.scanner.insert_watched_folder(path)
    }

    #[inline]
    fn remove_watched_folders(&self, callback: impl FnMut(&Utf8Path) -> bool) {
        self.scanner.remove_watched_folders(callback)
    }

    #[inline]
    fn unload_file(&self, path: &Utf8Path) -> Result<Vec<Diagnostic>, WorkspaceError> {
        self.workspace.unload_file(path)
    }

    #[inline]
    fn unload_path(&self, path: &Utf8Path) -> Result<Vec<Diagnostic>, WorkspaceError> {
        self.workspace.unload_path(path)
    }

    #[inline]
    fn notify_stopped(&self) {
        self.workspace.notify(ServiceNotification::WatcherStopped);
    }
}

#[cfg(test)]
#[path = "workspace_scanner_bridge.tests.rs"]
mod workspace_scanner_bridge_tests;

#[cfg(test)]
#[path = "workspace_watcher_bridge.tests.rs"]
mod workspace_watcher_bridge_tests;
