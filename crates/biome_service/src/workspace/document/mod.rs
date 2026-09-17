pub(crate) mod services;

use crate::diagnostics::FileTooLarge;
use camino::{Utf8Path, Utf8PathBuf};
use parking_lot::lock_api::ArcReentrantMutexGuard;
use parking_lot::{RawMutex, RawThreadId, ReentrantMutex};
use rustc_hash::FxBuildHasher;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub(crate) struct Document {
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

/// Guard returned by [`DocumentLocks::lock`]. Must not be held across an
/// `.await`.
pub type DocumentLockGuard = ArcReentrantMutexGuard<RawMutex, RawThreadId, ()>;

/// One lock per document path, taken by every writer of a document and by
/// readers that need the parsed file and the content of a document to belong
/// to the same version.
///
/// A document lives in two places, its parsed file in the database and its
/// content in the workspace's documents, and the two are written one after
/// the other. The LSP maps results computed from the parsed file (formatting,
/// code actions, diagnostics) back onto its client's buffer using the content,
/// so its two reads must not straddle a write. The lock is re-entrant: an LSP
/// request holds it while it asks the workspace to re-open the file, which
/// takes it again on the same thread.
///
/// Every writer takes the lock, whether the content comes from a client or
/// from the file system (the scanner, the watcher, a CLI running through the
/// daemon), and the lock is created on first use. Taking it only when a
/// client has the document open wouldn't do: a writer that finds no lock and
/// then stores its parse races with the client's first open, which creates
/// the lock and stores both of its own. Entries are never removed; there is
/// one per path ever written, next to the document itself.
#[derive(Default)]
pub(crate) struct DocumentLocks {
    locks: papaya::HashMap<Utf8PathBuf, Arc<ReentrantMutex<()>>, FxBuildHasher>,
}

impl DocumentLocks {
    /// Locks `path`, creating its lock if needed.
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
