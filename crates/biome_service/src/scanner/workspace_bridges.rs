use std::panic::RefUnwindSafe;

use crate::module_graph::ModuleDependencies;
use biome_diagnostics::Error;
use biome_fs::FileSystem;
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

    /// Returns the VCS ignore files that must be watched for the given project.
    fn get_vcs_watch_paths(&self, project_key: ProjectKey) -> Vec<BiomePath>;

    /// Returns the VCS root whose ignore files apply to the given project.
    fn get_vcs_root(&self, project_key: ProjectKey) -> Option<Utf8PathBuf>;

    /// Reloads the root and ancestor VCS ignore matchers for the given project.
    fn refresh_project_vcs_ignore_settings(
        &self,
        project_key: ProjectKey,
    ) -> Result<(), WorkspaceError>;

    /// Asks the workspace whether the given `path` that falls under the project
    /// with the given `project_key` is ignored, assuming the given `scan_kind`
    /// and `request_kind`.
    ///
    /// Then `path_kind` is `None`, the underling code fallbacks to a sys call.
    ///
    /// `ancestors_checked` is `true` when a traversal reached `path` after
    /// accepting every directory that contains it, so an initial scan only
    /// needs to check `path` itself.
    fn is_ignored(
        &self,
        project_key: ProjectKey,
        scan_kind: &ScanKind,
        path: &Utf8Path,
        request_kind: IndexRequestKind,
        path_kind: Option<PathKind>,
        ancestors_checked: bool,
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
        path: BiomePath,
        trigger: IndexTrigger,
    ) -> Result<(ModuleDependencies, Vec<Error>), WorkspaceError>;

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
        scanned_directory: &Utf8Path,
        files: &[BiomePath],
    ) -> Result<(), WorkspaceError>;

    /// Sends the given `notification` to any service notification listeners.
    fn notify(&self, notification: ServiceNotification);

    /// Unloads the index of the file with the given `path` within the
    /// workspace.
    fn unload_file(
        &self,
        path: &Utf8Path,
        project_key: ProjectKey,
    ) -> Result<Vec<Diagnostic>, WorkspaceError>;

    /// Unloads the given `path` from the workspace index.
    ///
    /// Note that we don't really have a concept of open folders in the
    /// workspace, so we send a [`WatcherInstruction::UnwatchFolder`] just in
    /// case and iterate the documents to find paths that would be inside the
    /// unloaded folder.
    ///
    /// If you already know the path is a file, you should use
    /// [`WorkspaceWatcherBridge::unload_file()`] directly instead.
    fn unload_path(
        &self,
        path: &Utf8Path,
        project_key: ProjectKey,
    ) -> Result<Vec<Diagnostic>, WorkspaceError>;

    /// Refreshes the path info of `path` and of the known paths inside it,
    /// after the watcher reported a change.
    ///
    /// Returns whether the change affects a manifest: `path` itself, or a
    /// manifest inside it. Once the change is indexed, the dependencies that
    /// became reachable must then be indexed using
    /// [`WorkspaceScannerBridge::index_new_module_dependencies()`].
    fn sync_path_info(&self, path: &Utf8Path) -> bool;

    /// Indexes the dependencies that became reachable after a manifest
    /// changed.
    fn index_new_module_dependencies(
        &self,
        project_key: ProjectKey,
    ) -> Result<Vec<Diagnostic>, WorkspaceError>;
}

/// Trait used to give access to workspace functionality required by the
/// watcher.
pub trait WorkspaceWatcherBridge {
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
        path_kind: Option<PathKind>,
    ) -> Result<bool, WorkspaceError>;

    /// Indexes the file with the given `path` within the project with the given
    /// `project_key` in the workspace.
    ///
    /// If relevant to the given `scan_kind`, dependencies of the file are
    /// indexed as well.
    fn index_file(
        &self,
        project_key: ProjectKey,
        path: BiomePath,
    ) -> Result<Vec<Diagnostic>, WorkspaceError>;

    /// Scans and watches the folder with the given `path` within the project
    /// with the given `project_key` in the workspace, while indexing all the
    /// files relevant for the given `scan_kind`.
    ///
    /// If relevant to the given `scan_kind`, dependencies of the files in the
    /// folder are indexed as well.
    fn index_folder(&self, path: &Utf8Path) -> Result<Vec<Diagnostic>, WorkspaceError>;

    /// Reloads ignore matchers, unloads disk-backed entries under `path`, and rescans it.
    fn refresh_ignore_files(&self, path: &Utf8Path) -> Result<Vec<Diagnostic>, WorkspaceError>;

    /// Returns whether `path` is an exact VCS ignore file watched by a project.
    fn is_vcs_watch_path(&self, path: &Utf8Path) -> bool;

    /// Reloads projects affected by a change to an exact VCS ignore file.
    fn refresh_vcs_watch_path(&self, path: &Utf8Path) -> Result<Vec<Diagnostic>, WorkspaceError>;

    /// Informs the workspace the given folder is watched.
    ///
    /// Returns `true` if the folder was not yet watched, `false` otherwise.
    fn insert_watched_folder(&self, path: Utf8PathBuf) -> bool;

    /// Returns every logical alias for a physical watcher event path.
    fn logical_paths_for_watcher_event(&self, path: &Utf8Path) -> Vec<Utf8PathBuf>;

    /// Removes logical watch aliases under `path`.
    ///
    /// `callback` is invoked for every physical folder that no longer has a
    /// logical alias and must be unwatched.
    fn remove_watch_aliases(&self, path: &Utf8Path, callback: impl FnMut(&Utf8Path));

    /// Stops tracking the physical folder `path` as watched when no logical alias maps to it.
    ///
    /// Returns `true` if the folder was watched and must be unwatched, `false` otherwise.
    fn remove_unused_watched_folder(&self, path: &Utf8Path) -> bool;

    /// Unloads the index of the file with the given `path` within the
    /// workspace.
    fn unload_file(
        &self,
        path: &Utf8Path,
        project_key: ProjectKey,
    ) -> Result<Vec<Diagnostic>, WorkspaceError>;

    /// Unloads the given `path` from the workspace index.
    ///
    /// Note that we don't really have a concept of open folders in the
    /// workspace, so we send a [`WatcherInstruction::UnwatchFolder`] just in
    /// case and iterate the documents to find paths that would be inside the
    /// unloaded folder.
    ///
    /// If you already know the path is a file, you should use
    /// [`WorkspaceWatcherBridge::unload_file()`] directly instead.
    fn unload_path(
        &self,
        path: &Utf8Path,
        project_key: ProjectKey,
    ) -> Result<Vec<Diagnostic>, WorkspaceError>;

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

    fn vcs_watch_projects(&self, path: &Utf8Path) -> Vec<ProjectKey> {
        let path = self.workspace.fs().biome_path(path);
        // VCS watch paths are ignore files or the Git `info/exclude` file. Checking the file name
        // first avoids resolving the watch paths of every project, which reads the filesystem,
        // for ordinary file events.
        if !path.is_ignore() && path.as_path().file_name() != Some("exclude") {
            return Vec::new();
        }
        self.scanner
            .projects
            .pin()
            .keys()
            .copied()
            .filter(|project_key| {
                self.workspace
                    .get_vcs_watch_paths(*project_key)
                    .contains(&path)
            })
            .collect()
    }

    fn refresh_project_ignore_files(
        &self,
        project_key: ProjectKey,
        path: &Utf8Path,
    ) -> Result<Vec<Diagnostic>, WorkspaceError> {
        // Ignore files don't change the index of a project that doesn't use VCS ignores.
        if self.workspace.get_vcs_root(project_key).is_none() {
            return Ok(Vec::new());
        }
        let mut diagnostics =
            self.scanner
                .refresh_ignore_files(self.workspace, project_key, path)?;
        diagnostics.extend(self.unload_path(path, project_key)?);
        let manifest_changed = self.workspace.sync_path_info(path);
        diagnostics.extend(
            self.scanner
                .index_folder(self.workspace, project_key, path)?,
        );
        if manifest_changed {
            diagnostics.extend(self.workspace.index_new_module_dependencies(project_key)?);
        }
        Ok(diagnostics)
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
        path_kind: Option<PathKind>,
    ) -> Result<bool, WorkspaceError> {
        self.workspace.is_ignored(
            project_key,
            scan_kind,
            path,
            IndexRequestKind::Explicit(IndexTrigger::Update),
            path_kind,
            false,
        )
    }

    fn index_file(
        &self,
        project_key: ProjectKey,
        path: BiomePath,
    ) -> Result<Vec<Diagnostic>, WorkspaceError> {
        // The file is indexed with the refreshed path info, so its imports resolve
        // against the current filesystem.
        let manifest_changed = self.workspace.sync_path_info(path.as_path());
        let (dependencies, diagnostics) =
            self.workspace
                .index_file(project_key, path, IndexTrigger::Update)?;
        let mut diagnostics = diagnostics
            .into_iter()
            .map(biome_diagnostics::serde::Diagnostic::new)
            .collect::<Vec<_>>();
        if !dependencies.is_empty()
            && let Some(project_path) = self.workspace.get_project_path(project_key)
        {
            diagnostics.extend(self.scanner.index_dependencies(
                self.workspace,
                project_key,
                &project_path,
                dependencies,
                IndexTrigger::Update,
            )?);
        }
        if manifest_changed {
            diagnostics.extend(self.workspace.index_new_module_dependencies(project_key)?);
        }
        Ok(diagnostics)
    }

    fn index_folder(&self, path: &Utf8Path) -> Result<Vec<Diagnostic>, WorkspaceError> {
        let Some(project_key) = self.find_project_for_path(path) else {
            return Ok(vec![]); // file events outside our projects can be safely ignored.
        };

        let manifest_changed = self.workspace.sync_path_info(path);
        let mut diagnostics =
            self.scanner
                .refresh_ignore_files(self.workspace, project_key, path)?;
        diagnostics.extend(
            self.scanner
                .index_folder(self.workspace, project_key, path)?,
        );
        if manifest_changed {
            diagnostics.extend(self.workspace.index_new_module_dependencies(project_key)?);
        }
        Ok(diagnostics)
    }

    fn refresh_ignore_files(&self, path: &Utf8Path) -> Result<Vec<Diagnostic>, WorkspaceError> {
        let Some(project_key) = self.find_project_for_path(path) else {
            return Ok(vec![]);
        };
        self.refresh_project_ignore_files(project_key, path)
    }

    fn is_vcs_watch_path(&self, path: &Utf8Path) -> bool {
        !self.vcs_watch_projects(path).is_empty()
    }

    fn refresh_vcs_watch_path(&self, path: &Utf8Path) -> Result<Vec<Diagnostic>, WorkspaceError> {
        let mut diagnostics = Vec::new();
        for project_key in self.vcs_watch_projects(path) {
            // Projects can share the ignore file, so the failure of one project is reported
            // without stopping the refresh of the others.
            let result = self
                .workspace
                .refresh_project_vcs_ignore_settings(project_key)
                .and_then(|()| {
                    self.workspace.get_project_path(project_key).map_or_else(
                        || Ok(Vec::new()),
                        |project_path| {
                            self.refresh_project_ignore_files(project_key, project_path.as_path())
                        },
                    )
                });
            match result {
                Ok(project_diagnostics) => diagnostics.extend(project_diagnostics),
                Err(error) => diagnostics.push(Diagnostic::new(error)),
            }
        }
        Ok(diagnostics)
    }

    #[inline]
    fn insert_watched_folder(&self, path: Utf8PathBuf) -> bool {
        self.scanner.insert_watched_folder(path)
    }

    #[inline]
    fn logical_paths_for_watcher_event(&self, path: &Utf8Path) -> Vec<Utf8PathBuf> {
        self.scanner.logical_paths_for_watcher_event(path)
    }

    #[inline]
    fn remove_watch_aliases(&self, path: &Utf8Path, callback: impl FnMut(&Utf8Path)) {
        self.scanner.remove_watch_aliases(path, callback)
    }

    #[inline]
    fn remove_unused_watched_folder(&self, path: &Utf8Path) -> bool {
        self.scanner.remove_unused_watched_folder(path)
    }

    fn unload_file(
        &self,
        path: &Utf8Path,
        project_key: ProjectKey,
    ) -> Result<Vec<Diagnostic>, WorkspaceError> {
        let mut diagnostics = self.workspace.unload_file(path, project_key)?;
        if self.workspace.sync_path_info(path) {
            diagnostics.extend(self.workspace.index_new_module_dependencies(project_key)?);
        }
        Ok(diagnostics)
    }

    fn unload_path(
        &self,
        path: &Utf8Path,
        project_key: ProjectKey,
    ) -> Result<Vec<Diagnostic>, WorkspaceError> {
        let mut diagnostics = self.workspace.unload_path(path, project_key)?;
        // Renames and directory removals also unload manifests, which may make
        // other dependencies reachable.
        if self.workspace.sync_path_info(path) {
            diagnostics.extend(self.workspace.index_new_module_dependencies(project_key)?);
        }
        Ok(diagnostics)
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
