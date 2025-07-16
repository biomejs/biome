//! Watcher that helps the scanner to keep scanned files up-to-date with the
//! file system.
//!
//! This module only contains the methods on the
//! [workspace server](crate::WorkspaceServer) that facilitate the watcher.
//! The heart of the watcher is implemented inside [crate::WorkspaceWatcher].
//!
//! All the *public* methods in this module are intended to be called by the
//! watcher. Apart from updating open documents, they are also responsible for
//! updating service data such as the dependency graph.

use std::path::PathBuf;

use super::{
    ScanKind, ScanProjectFolderParams, ServiceDataNotification, Workspace, WorkspaceServer,
    document::Document,
};
use crate::{
    WatcherInstruction, WorkspaceError, workspace::IgnoreKind, workspace_watcher::WatcherSignalKind,
};
use biome_fs::{BiomePath, PathKind};
use camino::{Utf8Path, Utf8PathBuf};
use papaya::{Compute, Operation};

impl WorkspaceServer {
    /// Filters the given `paths` and returns only those the watcher is
    /// interested in.
    pub fn filter_paths_for_watcher(
        &self,
        paths: Vec<PathBuf>,
        scan_kind: &ScanKind,
    ) -> Vec<Utf8PathBuf> {
        paths
            .into_iter()
            .filter_map(|path| {
                let path = Utf8PathBuf::from_path_buf(path).ok()?;
                self.projects
                    .find_project_for_path(&path)
                    .filter(|project_key| {
                        match self.is_ignored_by_scanner(
                            *project_key,
                            scan_kind,
                            &BiomePath::new(&path),
                            IgnoreKind::Ancestors,
                        ) {
                            Ok(is_ignored) => !is_ignored,
                            Err(_) => true,
                        }
                    })
                    .map(|_| path)
            })
            .collect()
    }

    /// Used by the watcher to open one or more files or folders.
    ///
    /// If you already know the paths are folders, use
    /// [Self::open_folders_through_watcher()] instead.
    pub fn open_paths_through_watcher(
        &self,
        paths: Vec<Utf8PathBuf>,
        scan_kind: &ScanKind,
    ) -> Result<(), WorkspaceError> {
        for path in paths {
            self.open_path_through_watcher(&path, scan_kind)?;
        }

        Ok(())
    }

    /// Used by the watcher to open one or more folders.
    pub fn open_folders_through_watcher(
        &self,
        paths: Vec<Utf8PathBuf>,
        scan_kind: &ScanKind,
    ) -> Result<(), WorkspaceError> {
        for path in paths {
            self.open_folder_through_watcher(&path, scan_kind)?;
        }

        Ok(())
    }

    /// Used indirectly by the watcher to open an individual file or folder.
    ///
    /// If you already know the path is a folder, use
    /// [`Self::open_folder_through_watcher()`] instead. If you know it is a
    /// file, you can directly call [`Self::open_file_by_watcher()`] instead.
    pub fn open_path_through_watcher(
        &self,
        path: &Utf8Path,
        scan_kind: &ScanKind,
    ) -> Result<(), WorkspaceError> {
        if let PathKind::Directory { .. } = self.fs.path_kind(path)? {
            self.open_folder_through_watcher(path, scan_kind)
        } else {
            let Some(project_key) = self.projects.find_project_for_path(path) else {
                return Ok(()); // file events outside our projects can be safely ignored.
            };

            self.open_file_by_watcher(project_key, scan_kind, path)
        }
    }

    /// Used indirectly by the watcher to open an individual folder.
    fn open_folder_through_watcher(
        &self,
        path: &Utf8Path,
        scan_kind: &ScanKind,
    ) -> Result<(), WorkspaceError> {
        let Some(project_key) = self.projects.find_project_for_path(path) else {
            return Ok(()); // file events outside our projects can be safely ignored.
        };

        self.scan_project_folder(ScanProjectFolderParams {
            project_key,
            path: Some(path.into()),
            watch: true,
            force: true,
            scan_kind: scan_kind.clone(),
            verbose: false,
        })
        .map(|_| ())
    }

    /// Used by the watcher to close one or more files.
    pub fn close_files_through_watcher(
        &self,
        paths: Vec<Utf8PathBuf>,
    ) -> Result<(), WorkspaceError> {
        for path in paths {
            self.close_file_through_watcher(&path)?;
        }

        Ok(())
    }

    /// Used by the watcher to close one or more files or folders.
    ///
    /// If you know the paths are files, use
    /// [Self::close_files_through_watcher()] instead.
    pub fn close_paths_through_watcher(
        &self,
        paths: Vec<Utf8PathBuf>,
    ) -> Result<(), WorkspaceError> {
        for path in &paths {
            self.close_path_through_watcher(path)?;
        }

        Ok(())
    }

    /// Used indirectly by the watcher to close an individual file.
    fn close_file_through_watcher(&self, path: &Utf8Path) -> Result<(), WorkspaceError> {
        let documents = self.documents.pin();
        let result = documents.compute(path.to_path_buf(), |current| {
            match current {
                Some((_path, document)) if document.version.is_some() => {
                    // If the document has a version, some client is also
                    // working with it, so we only unflag it as being opened by
                    // the scanner.
                    Operation::Insert(Document {
                        opened_by_scanner: false,
                        ..document.clone()
                    })
                }
                Some(_) => Operation::Remove,
                None => Operation::Abort(()),
            }
        });
        match result {
            Compute::Removed(_, _) => {
                self.update_service_data(WatcherSignalKind::Removed, path, None)
            }
            _ => Ok(()),
        }
    }

    /// Used indirectly by the watcher to close an individual file or folder.
    ///
    /// Note that we don't really have a concept of open folders in the
    /// workspace, so we send a [`WatcherInstruction::UnwatchFolder`] just in
    /// case and iterate the documents to find paths that would be inside the
    /// closed folder.
    ///
    /// If you already know the path is a file, use
    /// [Self::close_file_through_watcher()] instead.
    fn close_path_through_watcher(&self, path: &Utf8Path) -> Result<(), WorkspaceError> {
        // Note that we cannot check the kind of the path, because the watcher
        // would only attempt to close a file or folder after it has been
        // removed. So asking the file system wouldn't work anymore. If it turns
        // the path didn't belong to a folder, the watcher will ignore it.
        let _ = self
            .watcher_tx
            .try_send(WatcherInstruction::UnwatchFolder(path.to_path_buf()));

        for document_path in self.documents.pin().keys() {
            if document_path.starts_with(path) {
                self.close_file_through_watcher(document_path)?;
            }
        }

        Ok(())
    }

    /// Used by the watcher to rename a path if it knows both the from and to
    /// paths.
    pub fn rename_path_through_watcher(
        &self,
        from: &Utf8Path,
        to: &Utf8Path,
        scan_kind: &ScanKind,
    ) -> Result<(), WorkspaceError> {
        self.close_path_through_watcher(from)?;
        self.open_path_through_watcher(to, scan_kind)?;

        Ok(())
    }

    /// Reopens an individual file if the watcher (still) has interest in it.
    pub fn resync_file_through_watcher(
        &self,
        path: &Utf8Path,
        scan_kind: &ScanKind,
    ) -> Result<(), WorkspaceError> {
        let Some(project_key) = self.projects.find_project_for_path(path) else {
            return Ok(()); // file events outside our projects can be safely ignored.
        };

        if self
            .is_ignored_by_scanner(
                project_key,
                scan_kind,
                &BiomePath::new(path),
                IgnoreKind::Ancestors,
            )
            .unwrap_or(true)
        {
            return Ok(());
        }

        self.open_path_through_watcher(path, scan_kind)
    }

    /// Used by the watcher to indicate it has stopped.
    pub fn watcher_stopped(&self) {
        let _ = self.notification_tx.send(ServiceDataNotification::Stop);
    }
}

#[cfg(test)]
#[path = "watcher.tests.rs"]
mod tests;
