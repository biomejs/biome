#[cfg(not(feature = "module_graph"))]
use crate::module_graph::PathInfoCache;
#[cfg(feature = "module_graph")]
use crate::module_graph::{ModuleInfo, ModuleInfoKind, PathInfoCache};
use biome_db::{ParsedSnippet, ParsedSource};
use biome_languages::DocumentFileSource;
use biome_parser::AnyParse;
use biome_rowan::SendNode;
use biome_workspace_db::{ParsedSourceUpdateMode, WorkspaceDb, WorkspaceDbData};
use camino::Utf8Path;
use parking_lot::Mutex;
use std::cell::Cell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::panic::resume_unwind;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::embed::EmbedContent;

/// Represents the state of the database in the workspace.
pub struct DbState {
    storage: OwnedDb,
    pub(crate) path_info_cache: PathInfoCache,
}

// Counts database forks held by the current thread.
thread_local! {
    static LIVE_READS: Cell<usize> = const { Cell::new(0) };
}

/// Read guard returned by [`DbState::fork`].
///
/// It records that the current thread is using a database fork, so [`OwnedDb`]
/// can reject setter-based writes before they wait on that same fork.
///
/// NOTE: This is a runtime safety check, not a complete guarantee. Calling
/// [`Self::into_untracked_db`] or cloning the inner [`WorkspaceDb`] through
/// [`Deref`] can create a database handle that is no longer counted here. Keep
/// those escapes limited to read-only leaf operations.
pub(crate) struct DbReadGuard {
    db: WorkspaceDb,
    _live_read: LiveReadGuard,
    _not_send: PhantomData<std::sync::MutexGuard<'static, ()>>,
}

impl DbReadGuard {
    fn new(db: WorkspaceDb, tracks_live_read: bool) -> Self {
        Self {
            db,
            _live_read: LiveReadGuard::new(tracks_live_read),
            _not_send: PhantomData,
        }
    }

    /// Clones the raw database without extending read tracking to the clone.
    pub(crate) fn clone_untracked_db(&self) -> WorkspaceDb {
        self.db.clone()
    }

    /// Consumes the guard and returns the raw database without read tracking.
    ///
    /// NOTE: After this returns, same-thread writes cannot see that this
    /// database handle is live. Do not call this before code that may perform a
    /// [`DbState`] write on the same thread. Prefer keeping the guard alive for
    /// normal reads, and only use this when a lower-level read-only API needs a
    /// plain [`WorkspaceDb`].
    pub(crate) fn into_untracked_db(self) -> WorkspaceDb {
        self.db
    }
}

/// Increments the current thread's read count while a [`DbReadGuard`] is alive.
struct LiveReadGuard {
    tracks_live_read: bool,
}

impl LiveReadGuard {
    fn new(tracks_live_read: bool) -> Self {
        if tracks_live_read {
            LIVE_READS.with(|reads| reads.set(reads.get() + 1));
        }

        Self { tracks_live_read }
    }
}

impl Drop for LiveReadGuard {
    fn drop(&mut self) {
        if self.tracks_live_read {
            LIVE_READS.with(|reads| {
                let count = reads.get();
                debug_assert!(
                    count > 0,
                    "db read guard counter underflowed; create read guards only through DbState::fork and keep LiveReadGuard ownership paired with DbReadGuard"
                );
                reads.set(count.saturating_sub(1));
            });
        }
    }
}

impl Deref for DbReadGuard {
    type Target = WorkspaceDb;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
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
    update_mode: ParsedSourceUpdateMode,
    #[cfg(test)]
    setter_count: AtomicUsize,
}

impl OwnedDb {
    fn new(db: WorkspaceDb, update_mode: ParsedSourceUpdateMode) -> Self {
        let data = db.data();
        Self {
            db: Mutex::new(db),
            data,
            pending_setters: AtomicUsize::new(0),
            update_mode,
            #[cfg(test)]
            setter_count: AtomicUsize::new(0),
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

        if LIVE_READS.with(|reads| reads.get()) != 0 {
            panic!(
                "db setter invoked while this thread holds a db clone; move database reads into a smaller scope, collect owned inputs, then call the DbState write after the read guard is dropped"
            );
        }

        #[cfg(test)]
        self.setter_count.fetch_add(1, Ordering::Relaxed);
        self.pending_setters.fetch_add(1, Ordering::Release);
        let _guard = PendingSetterGuard(&self.pending_setters);
        let mut db = self.db.lock();
        f(&mut db)
    }

    fn with_parsed_source_update<R>(&self, f: impl FnOnce(&mut WorkspaceDb) -> R) -> R {
        match self.update_mode {
            ParsedSourceUpdateMode::Replace => {
                let mut db = self.db.lock().clone();
                f(&mut db)
            }
            ParsedSourceUpdateMode::Setters => self.with_setter(f),
        }
    }
}

impl Default for DbState {
    fn default() -> Self {
        Self {
            storage: OwnedDb::new(WorkspaceDb::default(), ParsedSourceUpdateMode::Replace),
            path_info_cache: PathInfoCache::default(),
        }
    }
}

impl DbState {
    #[cfg(test)]
    pub(crate) fn setter_count(&self) -> usize {
        self.storage.setter_count.load(Ordering::Relaxed)
    }

    #[cfg(test)]
    pub(crate) fn reset_setter_count(&self) {
        self.storage.setter_count.store(0, Ordering::Relaxed);
    }

    pub fn lsp() -> Self {
        Self {
            storage: OwnedDb::new(WorkspaceDb::default(), ParsedSourceUpdateMode::Setters),
            path_info_cache: PathInfoCache::default(),
        }
    }

    pub(crate) fn fork(&self) -> DbReadGuard {
        DbReadGuard::new(self.storage.fork(), true)
    }

    pub(crate) fn insert_source(&self, document_file_source: DocumentFileSource) -> usize {
        self.storage.data.insert_source(document_file_source)
    }

    pub(crate) fn update_parsed_root(&self, path: &Utf8Path, new_root: SendNode) {
        self.storage.with_parsed_source_update(|db| {
            db.update_parsed_root_with_mode(path, new_root, self.storage.update_mode)
        })
    }

    pub(crate) fn update_parsed_file(
        &self,
        path: &Utf8Path,
        parsed: AnyParse,
        language_index: usize,
        snippets: Vec<(AnyParse, EmbedContent, usize)>,
    ) -> ParsedSource {
        self.storage.with_parsed_source_update(|db| {
            let parsed_snippets = create_parsed_snippets(db, snippets);
            db.update_or_insert_file(
                path,
                parsed,
                language_index,
                parsed_snippets,
                self.storage.update_mode,
            )
        })
    }

    pub(crate) fn unload_path(&self, path: &Utf8Path) {
        self.storage.with_setter(|db| db.unload_path(path))
    }

    #[cfg(feature = "module_graph")]
    pub(crate) fn upsert_module_kind(
        &self,
        path: camino::Utf8PathBuf,
        kind: ModuleInfoKind,
    ) -> ModuleInfo {
        self.storage
            .with_setter(|db| db.update_or_insert_module(path, kind))
    }

    #[cfg(feature = "module_graph")]
    pub(crate) fn remove_module(&self, path: &Utf8Path) {
        self.storage.with_setter(|db| db.remove_module(path))
    }

    /// Returns how many setter-based updates are currently running or
    /// waiting to run. Only used by tests to synchronize with a setter
    /// without relying on sleeps.
    #[cfg(test)]
    pub(crate) fn pending_setters(&self) -> usize {
        self.storage.pending_setters.load(Ordering::Acquire)
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

    static_assertions::assert_not_impl_any!(DbReadGuard: Send);

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

    #[test]
    #[should_panic(expected = "db setter invoked while this thread holds a db clone")]
    fn owned_storage_setter_panics_when_this_thread_holds_read_guard() {
        let state = DbState::lsp();
        let path = Utf8PathBuf::from("test.js");
        let _db = state.fork();

        state.update_parsed_file(&path, parse_js("let a = 1;"), 0, vec![]);
    }

    #[test]
    fn replacement_update_does_not_cancel_concurrent_reads() {
        let state = DbState::default();
        let path = Utf8PathBuf::from("test.js");
        let db = state.fork();

        state.update_parsed_file(&path, parse_js("let a = 1;"), 0, vec![]);

        assert_eq!(state.pending_setters(), 0);
        assert!(db.get_file(&path).is_some());
    }

    #[test]
    fn owned_storage_setter_from_other_thread_waits_for_read_guard() {
        let state = Arc::new(DbState::lsp());
        let path = Utf8PathBuf::from("test.js");
        state.update_parsed_file(&path, parse_js("let a = 1;"), 0, vec![]);

        let db = state.fork();
        let (done_tx, done_rx) = mpsc::channel();

        let setter = {
            let state = state.clone();
            let path = path.clone();
            thread::spawn(move || {
                state.update_parsed_file(&path, parse_js("let b = 2;"), 0, vec![]);
                done_tx.send(()).unwrap();
            })
        };

        wait_for_pending_setter(&state);
        assert!(
            done_rx.recv_timeout(Duration::from_millis(25)).is_err(),
            "setter should wait while another thread holds a read guard"
        );

        drop(db);
        assert!(
            done_rx.recv_timeout(Duration::from_secs(5)).is_ok(),
            "setter should complete after the other thread drops its read guard"
        );
        setter.join().unwrap();
    }
}
