//! Watcher that helps to scanner to keep scanned files up-to-date with the
//! file system.
//!
//! This module only contains the methods on the
//! [workspace server](crate::WorkspaceServer) that facilitate the watcher.
//! The heart of the watcher is implemented inside [crate::WorkspaceWatcher].

use std::path::{Path, PathBuf};

use biome_fs::{FileSystemDiagnostic, PathKind};
use camino::Utf8Path;

use crate::WorkspaceError;

use super::{
    server::Document, FileContent, OpenFileParams, ScanProjectFolderParams, Workspace,
    WorkspaceServer,
};

impl WorkspaceServer {
    /// Used by the watcher to open one or more files.
    ///
    /// This method can also be used if we don't know whether the paths are
    /// files or folders, because [Self::open_file_through_watcher()] checks the
    /// kind internally.
    pub fn open_files_through_watcher(&self, paths: Vec<PathBuf>) -> Result<(), WorkspaceError> {
        for path in paths {
            self.open_file_through_watcher(
                path.as_path()
                    .try_into()
                    .map_err(|_| FileSystemDiagnostic::non_utf8_path(&path))?,
            )?;
        }

        Ok(())
    }

    /// Used by the watcher to open one or more folders.
    pub fn open_folders_through_watcher(&self, paths: Vec<PathBuf>) -> Result<(), WorkspaceError> {
        for path in paths {
            self.open_folder_through_watcher(
                path.as_path()
                    .try_into()
                    .map_err(|_| FileSystemDiagnostic::non_utf8_path(&path))?,
            )?;
        }

        Ok(())
    }

    /// Used by the watcher to open an individual file.
    ///
    /// Note that we don't always know whether the path belongs to a file, so we
    /// explicitly need to check the kind here.
    pub fn open_file_through_watcher(&self, path: &Utf8Path) -> Result<(), WorkspaceError> {
        if let PathKind::Directory { .. } = self.fs.path_kind(path)? {
            return self.open_folder_through_watcher(path);
        }

        let Some(project_key) = self.projects.find_project_for_path(path) else {
            return Ok(()); // file events outside our projects can be safely ignored.
        };

        self.open_file_by_scanner(OpenFileParams {
            project_key,
            path: path.into(),
            content: FileContent::FromServer,
            version: None,
            document_file_source: None,
            persist_node_cache: false,
        })
    }

    /// Used by the watcher to open an individual folder.
    pub fn open_folder_through_watcher(&self, path: &Utf8Path) -> Result<(), WorkspaceError> {
        let Some(project_key) = self.projects.find_project_for_path(path) else {
            return Ok(()); // file events outside our projects can be safely ignored.
        };

        self.scan_project_folder(ScanProjectFolderParams {
            project_key,
            path: Some(path.into()),
            watch: false, // It's already being watched.
        })
        .map(|_| ())
    }

    /// Used by the watcher to close one or more files.
    pub fn close_files_through_watcher(&self, paths: Vec<PathBuf>) -> Result<(), WorkspaceError> {
        for path in paths {
            self.close_file_through_watcher(
                path.as_path()
                    .try_into()
                    .map_err(|_| FileSystemDiagnostic::non_utf8_path(&path))?,
            )?;
        }

        Ok(())
    }

    /// Used by the watcher to close one or more folders.
    pub fn close_folders_through_watcher(&self, paths: Vec<PathBuf>) -> Result<(), WorkspaceError> {
        for path in paths {
            self.close_folder_through_watcher(
                path.as_path()
                    .try_into()
                    .map_err(|_| FileSystemDiagnostic::non_utf8_path(&path))?,
            )?;
        }

        Ok(())
    }

    /// Used by the watcher to close an individual file.
    pub fn close_file_through_watcher(&self, path: &Utf8Path) -> Result<(), WorkspaceError> {
        // Assign to a temporary to release the lock ASAP.
        let has_node_cache = self.node_cache.lock().unwrap().contains_key(path);

        // This one is a bit tricky: If the user has the path open in an editor,
        // we don't want to unload the document even though it was removed from
        // the file system. But otherwise we do.
        //
        // Fortunately we have a great hueristic to determine whether a document
        // is opened by an editor: the node cache. Node caches are only kept for
        // editor documents. So if one is present for this path, we only unflag
        // the document as being opened by the scanner.
        let documents = self.documents.pin();
        if has_node_cache {
            documents.update(path.to_path_buf(), |document| Document {
                content: document.content.clone(),
                file_source_index: document.file_source_index,
                syntax: document.syntax.clone(),
                version: document.version,
                opened_by_scanner: false,
            });
        } else {
            documents.remove(path);
        }

        Ok(())
    }

    /// Used by the watcher to close an individual folder.
    ///
    /// Note that we don't really have a concept of open folders in the
    /// workspace, so instead we just iterate the documents to find paths that
    /// would be inside the closed folder.
    ///
    /// This method is also used when closing an individual file or folder if we
    /// don't know which kind it is. This is because the watcher would only
    /// attempt to close a file or folder after it has been removed, so asking
    /// the file system for the kind wouldn't work anymore.
    pub fn close_folder_through_watcher(&self, path: &Utf8Path) -> Result<(), WorkspaceError> {
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
        from: &Path,
        to: &Path,
    ) -> Result<(), WorkspaceError> {
        self.close_folder_through_watcher(
            from.try_into()
                .map_err(|_| FileSystemDiagnostic::non_utf8_path(from))?,
        )?;
        self.open_file_through_watcher(
            to.try_into()
                .map_err(|_| FileSystemDiagnostic::non_utf8_path(to))?,
        )
    }
}
