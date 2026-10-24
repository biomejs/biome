pub(crate) mod services;

use crate::diagnostics::FileTooLarge;
use camino::{Utf8Path, Utf8PathBuf};
use parking_lot::lock_api::ArcReentrantMutexGuard;
use parking_lot::{RawMutex, RawThreadId, ReentrantMutex};
use rustc_hash::FxBuildHasher;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub(crate) struct Document {
    /// Identifies one open document, so in-flight edits cannot update a reopened path.
    /// Edits retain this identity; opening the file again creates a new one because versions can restart.
    pub(crate) identity: Arc<()>,

    /// Document content.
    ///
    /// The content of the file is only available if it belongs to the project. For example, we don't
    /// want to store the content of files coming from dependencies.
    pub(crate) content: String,

    /// The version of the document.
    ///
    /// A version is only specified when the document is opened by a client,
    /// typically through the LSP. Documents that are only opened by the scanner
    /// do not have a version.
    pub(crate) version: Option<i32>,

    /// The index of where the original file source is saved.
    /// Use `WorkspaceServer#file_sources` to retrieve the file source that belongs to the document.
    pub(crate) file_source_index: usize,

    /// The result of the parser (syntax tree + diagnostics).
    /// Types explained:
    /// - `Option`: if the file can be read
    /// - `Result`: if the file is read, but the file is too large
    /// - `AnyParse`: the result of the parsed file
    pub(crate) syntax: Option<Result<(), FileTooLarge>>,
}

impl Document {
    pub(crate) fn accepts_change(&self, version: i32, content: &str) -> bool {
        self.version.is_none_or(|current| {
            version > current || version == current && content == self.content
        })
    }
}

/// Holds exclusive access to a document's text and parse updates.
/// Must not be held across an `.await`.
pub(crate) type DocumentLockGuard = ArcReentrantMutexGuard<RawMutex, RawThreadId, ()>;

/// Serializes updates to each document's text and parse, which live in separate stores.
/// Entries remain allocated so closing and reopening a path cannot create competing locks.
#[derive(Default)]
pub(crate) struct DocumentLocks {
    locks: papaya::HashMap<Utf8PathBuf, Arc<ReentrantMutex<()>>, FxBuildHasher>,
}

impl DocumentLocks {
    pub(crate) fn lock(&self, path: &Utf8Path) -> DocumentLockGuard {
        let locks = self.locks.pin();
        let mutex = match locks.get(path) {
            Some(mutex) => Arc::clone(mutex),
            None => Arc::clone(
                locks.get_or_insert_with(path.to_path_buf(), || Arc::new(ReentrantMutex::new(()))),
            ),
        };
        drop(locks);
        mutex.lock_arc()
    }
}
