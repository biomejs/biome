use crate::module_graph::PathInfoCache;
use biome_db::{ParsedSnippet, ParsedSource};
use biome_languages::DocumentFileSource;
use biome_parser::AnyParse;
use biome_rowan::SendNode;
use biome_workspace_db::{ParsedSourceUpdateMode, SharedWorkspaceDb, WorkspaceDb, WorkspaceDbData};
use camino::{Utf8Path, Utf8PathBuf};
use parking_lot::Mutex;
use std::panic::resume_unwind;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::embed::EmbedContent;

/// Represents the state of the database in the workspace.
pub struct DbState {
    storage: DbStorage,
    pub(crate) path_info_cache: PathInfoCache,
}

enum DbStorage {
    Shared(SharedWorkspaceDb),
    Owned(OwnedDb),
}

/// Storage for the LSP, where the database is updated through salsa setters
/// so that salsa can cancel outdated queries.
///
/// A salsa setter needs `&mut` access to the database. To get it, salsa waits
/// until every clone of the database has been dropped. Two rules keep this
/// from turning into a deadlock:
///
/// 1. A thread that holds a clone must be able to finish its work without
///    taking the lock on [Self::db]. Updates that don't go through salsa use
///    the shared [Self::data] collections instead, which need no lock.
/// 2. [Self::fork] must not wait for a setter to finish. The setter is itself
///    waiting for existing clones to be dropped, so if the thread asking for
///    a new clone already holds one, both would wait on each other forever.
///    Instead, while a setter is waiting, [Self::fork] gives up by unwinding
///    with [salsa::Cancelled] — the same signal a salsa query receives when
///    it runs during a pending update. Callers catch it and retry.
///
/// For the same reason, a thread must never call [Self::with_setter] while it
/// holds a clone: the setter would wait for the thread's own clone, which is
/// never dropped.
struct OwnedDb {
    /// The database instance itself. The lock is held only briefly to create
    /// a clone, and for the whole duration of a setter-based update.
    db: Mutex<WorkspaceDb>,
    /// The collections shared between the database and all its clones. See
    /// [WorkspaceDbData].
    data: WorkspaceDbData,
    /// How many threads are currently applying, or waiting to apply, a
    /// setter-based update.
    pending_setters: AtomicUsize,
}

impl OwnedDb {
    fn new(db: WorkspaceDb) -> Self {
        let data = db.data();
        Self {
            db: Mutex::new(db),
            data,
            pending_setters: AtomicUsize::new(0),
        }
    }

    fn fork(&self) -> WorkspaceDb {
        loop {
            if self.pending_setters.load(Ordering::Acquire) > 0 {
                // A setter is waiting for all clones to be dropped. If we
                // waited for it here, a thread that already holds a clone
                // could get stuck forever: see the rules on [OwnedDb].
                resume_unwind(Box::new(salsa::Cancelled::PendingWrite));
            }
            // Normally the lock is only held for the time it takes to create
            // a clone. A setter may still grab it right after the check
            // above, so never block on the lock: try it, and if that fails,
            // check again whether a setter is the reason.
            if let Some(db) = self.db.try_lock() {
                return db.clone();
            }
            std::thread::yield_now();
        }
    }

    fn with_setter<R>(&self, f: impl FnOnce(&mut WorkspaceDb) -> R) -> R {
        struct PendingSetterGuard<'a>(&'a AtomicUsize);
        impl Drop for PendingSetterGuard<'_> {
            fn drop(&mut self) {
                self.0.fetch_sub(1, Ordering::Release);
            }
        }

        self.pending_setters.fetch_add(1, Ordering::Release);
        let _guard = PendingSetterGuard(&self.pending_setters);
        let mut db = self.db.lock();
        f(&mut db)
    }
}

impl Default for DbState {
    fn default() -> Self {
        Self {
            storage: DbStorage::Shared(SharedWorkspaceDb::default()),
            path_info_cache: PathInfoCache::default(),
        }
    }
}

impl DbState {
    pub fn lsp() -> Self {
        Self {
            storage: DbStorage::Owned(OwnedDb::new(WorkspaceDb::default())),
            path_info_cache: PathInfoCache::default(),
        }
    }

    pub(crate) fn fork(&self) -> WorkspaceDb {
        match &self.storage {
            DbStorage::Shared(shared_db) => shared_db.fork(),
            DbStorage::Owned(db) => db.fork(),
        }
    }

    pub(crate) fn insert_source(&self, document_file_source: DocumentFileSource) -> usize {
        match &self.storage {
            DbStorage::Shared(shared_db) => shared_db.fork().insert_source(document_file_source),
            DbStorage::Owned(db) => db.data.insert_source(document_file_source),
        }
    }

    pub(crate) fn update_parsed_root(&self, path: &Utf8Path, new_root: SendNode) {
        match &self.storage {
            DbStorage::Shared(shared_db) => shared_db.fork().update_parsed_root_with_mode(
                path,
                new_root,
                ParsedSourceUpdateMode::Replace,
            ),
            DbStorage::Owned(db) => db.with_setter(|db| {
                db.update_parsed_root_with_mode(path, new_root, ParsedSourceUpdateMode::Setters)
            }),
        }
    }

    pub(crate) fn update_parsed_file(
        &self,
        path: &Utf8Path,
        parsed: AnyParse,
        language_index: usize,
        snippets: Vec<(AnyParse, EmbedContent, usize)>,
    ) -> ParsedSource {
        match &self.storage {
            DbStorage::Shared(shared_db) => {
                let mut db = shared_db.fork();
                let parsed_snippets = create_parsed_snippets(&db, snippets);
                db.update_or_insert_file(
                    path,
                    parsed,
                    language_index,
                    parsed_snippets,
                    ParsedSourceUpdateMode::Replace,
                )
            }
            DbStorage::Owned(db) => db.with_setter(|db| {
                let parsed_snippets = create_parsed_snippets(db, snippets);
                db.update_or_insert_file(
                    path,
                    parsed,
                    language_index,
                    parsed_snippets,
                    ParsedSourceUpdateMode::Setters,
                )
            }),
        }
    }

    pub(crate) fn unload_path(&self, path: &Utf8Path) {
        match &self.storage {
            DbStorage::Shared(shared_db) => shared_db.fork().unload_path(path),
            DbStorage::Owned(db) => db.data.unload_path(path),
        }
    }

    #[cfg(feature = "module_graph")]
    pub(crate) fn insert_module(&self, path: Utf8PathBuf, module: biome_module_graph::ModuleInfo) {
        match &self.storage {
            DbStorage::Shared(shared_db) => shared_db.fork().insert_module(path, module),
            DbStorage::Owned(db) => db.data.insert_module(path, module),
        }
    }

    #[cfg(feature = "module_graph")]
    pub(crate) fn remove_module(&self, path: &Utf8Path) {
        match &self.storage {
            DbStorage::Shared(shared_db) => shared_db.fork().remove_module(path),
            DbStorage::Owned(db) => db.data.remove_module(path),
        }
    }

    /// Returns how many setter-based updates are currently running or
    /// waiting to run. Only used by tests to synchronize with a setter
    /// without relying on sleeps.
    #[cfg(test)]
    fn pending_setters(&self) -> usize {
        match &self.storage {
            DbStorage::Shared(_) => 0,
            DbStorage::Owned(db) => db.pending_setters.load(Ordering::Acquire),
        }
    }
}

fn create_parsed_snippets(
    db: &WorkspaceDb,
    snippets: Vec<(AnyParse, EmbedContent, usize)>,
) -> Vec<ParsedSnippet> {
    snippets
        .into_iter()
        .map(|(parse, content, index)| {
            ParsedSnippet::new(
                db,
                parse,
                content.element_range,
                content.content_range,
                content.content_offset,
                index,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_js_parser::{JsParserOptions, parse};
    use biome_languages::JsFileSource;
    use camino::Utf8PathBuf;
    use std::panic::AssertUnwindSafe;
    use std::sync::{Arc, Barrier, mpsc};
    use std::thread;
    use std::time::Duration;

    fn parse_js(source: &str) -> AnyParse {
        parse(
            source,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        )
        .into()
    }

    /// Waits until a setter-based update is running or waiting to run.
    fn wait_for_pending_setter(state: &DbState) {
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        while state.pending_setters() == 0 {
            assert!(
                std::time::Instant::now() < deadline,
                "no setter became pending within 5 seconds"
            );
            thread::yield_now();
        }
    }

    /// A setter-based update waits until all clones of the database are
    /// dropped. While it waits, a thread holding a clone must still be able
    /// to update the shared collections: if that update needed the lock held
    /// by the setter, the two threads would wait on each other forever. This
    /// is a regression test for exactly that deadlock.
    #[test]
    fn owned_storage_shared_data_does_not_wait_for_pending_setters() {
        let state = Arc::new(DbState::lsp());
        let path = Utf8PathBuf::from("test.js");
        // Insert the file first: only updates to files the database already
        // knows about are applied through salsa setters.
        state.update_parsed_file(&path, parse_js("let a = 1;"), 0, vec![]);

        let clone_taken = Arc::new(Barrier::new(2));
        let (done_tx, done_rx) = mpsc::channel();

        let fork_holder = {
            let state = state.clone();
            let clone_taken = clone_taken.clone();
            thread::spawn(move || {
                let db = state.fork();
                clone_taken.wait();
                // Wait for the setter, which in turn waits for our clone to
                // be dropped.
                wait_for_pending_setter(&state);
                // This must complete on its own, without waiting for the
                // lock held by the setter.
                state.insert_source(DocumentFileSource::Js(JsFileSource::js_script()));
                drop(db);
            })
        };

        let setter = {
            let state = state.clone();
            let path = path.clone();
            thread::spawn(move || {
                clone_taken.wait();
                state.update_parsed_file(&path, parse_js("let b = 2;"), 0, vec![]);
                done_tx.send(()).unwrap();
            })
        };

        assert!(
            done_rx.recv_timeout(Duration::from_secs(5)).is_ok(),
            "the setter deadlocked while a thread holding a clone updated the shared collections"
        );
        fork_holder.join().unwrap();
        setter.join().unwrap();
    }

    /// Asking for a new clone while a setter is waiting must fail with
    /// [salsa::Cancelled] instead of blocking: the thread asking might
    /// already hold a clone that the setter is waiting for, and the two
    /// would wait on each other forever.
    #[test]
    fn owned_storage_fork_unwinds_while_setter_is_pending() {
        let state = Arc::new(DbState::lsp());
        let path = Utf8PathBuf::from("test.js");
        state.update_parsed_file(&path, parse_js("let a = 1;"), 0, vec![]);

        // Hold a clone so the setter below has to wait.
        let db = state.fork();

        let setter = {
            let state = state.clone();
            let path = path.clone();
            thread::spawn(move || {
                state.update_parsed_file(&path, parse_js("let b = 2;"), 0, vec![]);
            })
        };

        // Once the setter is waiting, asking for a clone must fail instead
        // of blocking.
        wait_for_pending_setter(&state);
        let result = salsa::Cancelled::catch(AssertUnwindSafe(|| state.fork()));
        assert!(
            matches!(result, Err(salsa::Cancelled::PendingWrite)),
            "fork should fail with a cancellation instead of waiting for the setter"
        );

        drop(db);
        setter.join().unwrap();
    }
}
