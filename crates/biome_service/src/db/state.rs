//! Coordinates access to workspace data shared between database forks.
//!
//! [`DbState`] supports two storage modes because operation-local work and the
//! long-lived LSP database require different update strategies.
//!
//! ## Storage modes
//!
//! Both modes share the underlying Salsa storage and workspace collections
//! between database forks. The names describe whether [`DbState`] owns a
//! canonical [`WorkspaceDb`] value:
//!
//! - **Shared mode** ([`DbStorage::Shared`]) retains a [`SharedWorkspaceDb`],
//!   which contains shared storage and collection handles but no Salsa local
//!   state. Each operation constructs a temporary [`WorkspaceDb`] with fresh
//!   local state from those handles. There is no canonical database value to
//!   mutate; an operation publishes its result through the shared collections.
//! - **Owned mode** ([`DbStorage::Owned`]) retains one canonical
//!   [`WorkspaceDb`] inside [`OwnedDb`]. Read operations use clones of that
//!   database. Write operations that need the canonical database run through
//!   [`OwnedDb::with_setter`], which locks it and coordinates the write with
//!   outstanding read clones.
//!
//! This ownership difference determines how Salsa-backed values are updated.
//! In shared mode, a temporary fork may allocate a replacement input and
//! publish its handle through a shared collection, but it must not call a Salsa
//! setter. A setter waits for exclusive access to the Salsa storage, which
//! cannot happen while the retained shared handle remains alive. In owned mode,
//! Salsa field setters run through `with_setter`; Salsa can then cancel outdated
//! queries and wait for read clones to be dropped before changing tracked
//! fields.
//!
//! Operations that intentionally do not participate in Salsa dependency
//! tracking can use [`WorkspaceDbData`] instead of the owned database. This is
//! required when the operation must remain available while a setter is
//! pending: a thread holding a read clone must be able to finish the untracked
//! operation so that the setter can acquire exclusive access.
//!
//! ## Choosing an update strategy
//!
//! First, determine which changes a Salsa query must observe. Salsa tracks
//! fields read from an already known input, but it does not automatically track
//! changes to an external lookup collection such as a Papaya map. Those changes
//! include adding a key, removing a key, or making an existing key point to a
//! replacement Salsa input.
//!
//! If queries use the collection to discover Salsa inputs, they must first read
//! a mutable Salsa-tracked change signal for that collection. A path, map key,
//! or Salsa input ID is not such a signal: it identifies an entry but does not
//! change when the external collection changes.
//!
//! One option is a generation counter for the whole map. Every map change
//! increments the same counter, so all queries that read it are invalidated.
//! [`ModuleGraphGeneration`](biome_module_graph::ModuleGraphGeneration) uses
//! this design because the module path map is stored outside Salsa.
//!
//! Another option is one stable Salsa input per map key. For example, the map
//! can retain the same input for `src/index.js`, with tracked `exists: bool` and
//! `revision: u64` fields. A query looks up that input and reads those fields.
//! Removing the file sets `exists` to `false`; recreating or changing it updates
//! `exists` or `revision`. Salsa then invalidates only queries that read the
//! input for `src/index.js`. The key identifies the input, while changes to its
//! tracked fields provide the signal.
//!
//! The examples below apply directly when collection changes intentionally do
//! not invalidate Salsa queries. Otherwise, mutate the collection's tracked
//! change signal as part of the operation. Owned mode can do this with a Salsa
//! setter inside `with_setter`. Shared mode cannot call that setter, so it
//! requires a collection-specific replacement or invalidation design. Do not
//! mutate the shared collection without providing that design.
//!
//! The snippets are schematic because each collection defines different lookup
//! and publication operations.
//!
//! ### Add
//!
//! Input creation allocates a permanent Salsa entry. Publishing its handle in
//! an external collection does not tell Salsa that the collection gained a key.
//! If queries must observe the new key, the operation must also mutate the
//! collection's tracked change signal.
//!
//! Shared mode creates the input from an operation-local fork and publishes
//! the handle through the shared collection:
//!
//! ```ignore
//! let db = shared.fork();
//! let input = Input::new(&db, fields);
//! shared_index.insert(key, input);
//! ```
//!
//! Owned mode creates and publishes the input inside `with_setter`, which
//! serializes the operation with other writes to the canonical database:
//!
//! ```ignore
//! owned.with_setter(|db| {
//!     let input = Input::new(db, fields);
//!     db.data().shared_index.insert(key, input);
//! });
//! ```
//!
//! ### Modify
//!
//! Shared mode cannot call a Salsa setter. It creates a replacement input and
//! publishes it only if the collection still contains the input from which the
//! replacement was derived. A concurrent change retries from the latest input:
//!
//! ```ignore
//! let current = shared_index.get(&key).copied().unwrap();
//! let replacement = Input::new(&db, update_fields(&db, current, changes));
//! shared_index.compute(key, |entry| match entry {
//!     Some((_, actual)) if *actual == current => Operation::Insert(replacement),
//!     _ => Operation::Abort(()),
//! });
//! ```
//!
//! Owned mode preserves the existing handle and mutates its tracked fields with
//! Salsa setters inside `with_setter`:
//!
//! ```ignore
//! owned.with_setter(|db| {
//!     let input = db.data().shared_index.get(&key).copied().unwrap();
//!     input.set_field(db).to(value);
//! });
//! ```
//!
//! Replacing an input in owned mode loses stable identity and leaves an
//! additional Salsa entry allocated.
//!
//! ### Remove
//!
//! Salsa inputs cannot be deleted. Removal drops the handle from the
//! application index while leaving the Salsa entry allocated. The direct
//! removal examples below are valid only for untracked indexes.
//!
//! Shared mode removes the handle directly from the shared collection:
//!
//! ```ignore
//! shared_index.remove(&key);
//! ```
//!
//! Owned mode also removes an untracked handle directly; no Salsa setter is
//! required:
//!
//! ```ignore
//! owned.data.shared_index.remove(&key);
//! ```
//!
//! Neither removal invalidates tracked queries. When queries must observe the
//! removed key, Owned mode updates the generation with a Salsa setter and
//! removes the handle as one coordinated operation:
//!
//! ```ignore
//! owned.with_setter(|db| {
//!     let next_generation = generation(db).wrapping_add(1);
//!     let pending = generation_input(db).set_value(db);
//!     db.data().shared_index.remove(&key);
//!     pending.to(next_generation);
//! });
//! ```
//!
//! Shared mode must not perform the same setter-based operation. It needs an
//! explicit replacement or invalidation mechanism for that collection before
//! keys can be added, removed, or redirected safely.
//!
//! Setter coordination has two additional constraints. [`DbState::fork`]
//! unwinds with [`salsa::Cancelled`] instead of waiting while a setter is
//! pending, because the calling thread may already hold a clone needed by that
//! setter. For the same reason, [`OwnedDb::with_setter`] must not be called
//! while the current thread holds a [`DbReadGuard`].

use super::{
    ParsedSourceUpdateMode, ProjectUpdateMode, SharedWorkspaceDb, WorkspaceDb, WorkspaceDbData,
};
use crate::WorkspaceError;
use crate::embed::EmbedContent;
use crate::module_graph::PathInfoCache;
#[cfg(feature = "module_graph")]
use crate::module_graph::{ModuleInfo, ModuleInfoKind};
use crate::projects::ProjectKey;
use crate::settings::Settings;
use biome_db::{ParsedSnippet, ParsedSource};
use biome_languages::DocumentFileSource;
use biome_parser::AnyParse;
use camino::{Utf8Path, Utf8PathBuf};
use parking_lot::Mutex;
use std::cell::Cell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::panic::resume_unwind;
use std::sync::atomic::{AtomicUsize, Ordering};
use tracing::error;

/// Represents the state of the database in the workspace.
pub struct DbState {
    storage: DbStorage,
    pub(crate) path_info_cache: PathInfoCache,
}

enum DbStorage {
    Shared(SharedWorkspaceDb),
    Owned(OwnedDb),
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

    /// Returns the database without the guard.
    ///
    /// After this call, [`DbState`] cannot detect that the returned database is
    /// still in use. A write on the same thread could then wait forever for that
    /// database to be dropped. Keep the guard for normal reads. Use this method
    /// only when passing the database to code that only reads from [`DbState`].
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

/// Owns the canonical LSP database and coordinates writes with read forks.
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

        if LIVE_READS.with(|reads| reads.get()) != 0 {
            debug_assert!(
                false,
                "db setter invoked while this thread holds a db clone; move database reads into a smaller scope, collect owned inputs, then call the DbState write after the read guard is dropped"
            );
            error!(
                "db setter invoked while this thread holds a db clone; cancelling the update to avoid a deadlock"
            );
            resume_unwind(Box::new(salsa::Cancelled::PendingWrite));
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

    pub(crate) fn fork(&self) -> DbReadGuard {
        match &self.storage {
            DbStorage::Shared(shared_db) => DbReadGuard::new(shared_db.fork(), false),
            DbStorage::Owned(db) => DbReadGuard::new(db.fork(), true),
        }
    }

    pub(crate) fn insert_source(&self, document_file_source: DocumentFileSource) -> usize {
        match &self.storage {
            DbStorage::Shared(shared_db) => shared_db.fork().insert_source(document_file_source),
            DbStorage::Owned(db) => db.data.insert_source(document_file_source),
        }
    }

    // #region Project functions

    /// Inserts a nested setting.
    ///
    /// Does nothing if the project doesn't exist.
    pub fn insert_nested_settings(
        &self,
        project_key: ProjectKey,
        path: Utf8PathBuf,
        settings: Settings,
    ) {
        match &self.storage {
            DbStorage::Shared(shared_db) => shared_db.fork().insert_nested_setting_with_mode(
                project_key,
                path,
                settings,
                ProjectUpdateMode::Replace,
            ),
            DbStorage::Owned(db) => db.with_setter(|db| {
                db.insert_nested_setting_with_mode(
                    project_key,
                    path,
                    settings,
                    ProjectUpdateMode::Setters,
                )
            }),
        };
    }

    pub fn insert_root_settings(&self, project_key: ProjectKey, settings: Settings) {
        match &self.storage {
            DbStorage::Shared(db) => db.fork().insert_root_settings_with_mode(
                project_key,
                settings,
                ProjectUpdateMode::Replace,
            ),
            DbStorage::Owned(db) => db.with_setter(|db| {
                db.insert_root_settings_with_mode(project_key, settings, ProjectUpdateMode::Setters)
            }),
        }
    }

    /// Inserts project
    pub fn insert_project(&self, path: Utf8PathBuf) -> ProjectKey {
        match &self.storage {
            DbStorage::Shared(shared_db) => shared_db.fork().insert_project(path),
            DbStorage::Owned(db) => db.with_setter(|db| db.insert_project(path)),
        }
    }

    /// Inserts project
    pub fn remove_project(&self, project_key: ProjectKey) {
        match &self.storage {
            DbStorage::Shared(shared_db) => shared_db.data().remove_project(project_key),
            DbStorage::Owned(db) => db.data.remove_project(project_key),
        }
    }

    pub fn store_nested_ignore_patterns(
        &self,
        project_key: ProjectKey,
        payload: Vec<(Utf8PathBuf, Vec<String>)>,
    ) -> Result<(), WorkspaceError> {
        match &self.storage {
            DbStorage::Shared(shared_db) => {
                shared_db.fork().store_nested_ignore_patterns_with_mode(
                    project_key,
                    payload,
                    ProjectUpdateMode::Replace,
                )
            }
            DbStorage::Owned(db) => db.with_setter(|db| {
                db.store_nested_ignore_patterns_with_mode(
                    project_key,
                    payload,
                    ProjectUpdateMode::Setters,
                )
            }),
        }
    }

    // #endregion

    /// Checks whether the shared module map currently contains `path` without
    /// creating a database snapshot.
    ///
    /// Use this to skip work that has already been done. Use the module
    /// database APIs when reading or analyzing the module itself.
    #[cfg(feature = "module_graph")]
    pub(crate) fn contains_module_untracked(&self, path: &Utf8Path) -> bool {
        match &self.storage {
            DbStorage::Shared(shared_db) => shared_db.data().contains_module_untracked(path),
            DbStorage::Owned(db) => db.data.contains_module_untracked(path),
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
            DbStorage::Shared(shared_db) => shared_db.fork().data().unload_path(path),
            DbStorage::Owned(db) => db.with_setter(|db| db.unload_path(path)),
        }
    }

    #[cfg(feature = "module_graph")]
    pub(crate) fn upsert_module_kind(
        &self,
        path: camino::Utf8PathBuf,
        kind: ModuleInfoKind,
    ) -> ModuleInfo {
        match &self.storage {
            DbStorage::Shared(shared_db) => {
                let db = shared_db.fork();
                let module = ModuleInfo::new(&db, path.clone(), kind);
                db.data().insert_module(path, module);
                module
            }
            DbStorage::Owned(db) => db.with_setter(|db| db.update_or_insert_module(path, kind)),
        }
    }

    #[cfg(feature = "module_graph")]
    pub(crate) fn remove_module(&self, path: &Utf8Path) {
        match &self.storage {
            DbStorage::Shared(shared_db) => shared_db.fork().data().remove_module(path),
            DbStorage::Owned(db) => db.with_setter(|db| db.remove_module(path)),
        }
    }

    /// Returns how many setter-based updates are currently running or
    /// waiting to run. Only used by tests to synchronize with a setter
    /// without relying on sleeps.
    #[cfg(test)]
    pub(crate) fn pending_setters(&self) -> usize {
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
    use crate::projects::ProjectDb;
    use biome_configuration::vcs::VcsClientKind;
    use biome_js_parser::{JsParserOptions, parse};
    use biome_languages::JsFileSource;
    use camino::Utf8PathBuf;
    use salsa::plumbing::AsId;
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

    fn git_settings() -> Settings {
        let mut settings = Settings::default();
        settings.vcs_settings.client_kind = Some(VcsClientKind::Git);
        settings
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

    /// Scanner bookkeeping must remain available while another thread is
    /// publishing database updates. Unlike a tracked database fork, checking
    /// shared module membership must not unwind when a setter is pending.
    #[cfg(feature = "module_graph")]
    #[test]
    fn untracked_module_membership_does_not_wait_for_pending_setters() {
        let state = Arc::new(DbState::lsp());
        let path = Utf8PathBuf::from("test.js");
        state.update_parsed_file(&path, parse_js("let a = 1;"), 0, vec![]);

        let db = state.fork();
        let setter = {
            let state = state.clone();
            let path = path.clone();
            thread::spawn(move || {
                state.update_parsed_file(&path, parse_js("let b = 2;"), 0, vec![]);
            })
        };

        wait_for_pending_setter(&state);
        assert!(!state.contains_module_untracked(&path));

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
        assert!(db.get_parsed_source(&path).is_some());
    }

    #[test]
    fn shared_storage_project_lifecycle_replaces_inputs_without_waiting_for_reads() {
        let state = Arc::new(DbState::default());
        let path = Utf8PathBuf::from("project");
        let project_key = state.insert_project(path.clone());
        let retained_db = state.fork();
        let project = retained_db.get_project(&project_key).unwrap();
        assert_eq!(project.path(&*retained_db), path.as_path());
        assert_eq!(
            project
                .root_settings(&*retained_db)
                .as_ref()
                .vcs_settings
                .client_kind,
            None
        );

        let (done_tx, done_rx) = mpsc::channel();
        let updater = {
            let state = state.clone();
            thread::spawn(move || {
                state.insert_root_settings(project_key, git_settings());
                done_tx.send(()).unwrap();
            })
        };
        assert!(
            done_rx.recv_timeout(Duration::from_secs(5)).is_ok(),
            "Shared project update waited for a retained database fork"
        );
        updater.join().unwrap();

        let updated_project = retained_db.get_project(&project_key).unwrap();
        assert_ne!(updated_project.as_id(), project.as_id());
        assert_eq!(
            updated_project
                .root_settings(&*retained_db)
                .as_ref()
                .vcs_settings
                .client_kind,
            Some(VcsClientKind::Git)
        );

        state.remove_project(project_key);
        assert!(retained_db.get_project(&project_key).is_none());
    }

    #[test]
    fn owned_storage_project_lifecycle_preserves_input_identity() {
        let state = DbState::lsp();
        let path = Utf8PathBuf::from("project");
        let project_key = state.insert_project(path.clone());
        let project = {
            let db = state.fork();
            let project = db.get_project(&project_key).unwrap();
            assert_eq!(project.path(&*db), path.as_path());
            assert_eq!(
                project
                    .root_settings(&*db)
                    .as_ref()
                    .vcs_settings
                    .client_kind,
                None
            );
            project
        };

        state.insert_root_settings(project_key, git_settings());

        {
            let db = state.fork();
            let updated_project = db.get_project(&project_key).unwrap();
            assert_eq!(updated_project.as_id(), project.as_id());
            assert_eq!(
                updated_project
                    .root_settings(&*db)
                    .as_ref()
                    .vcs_settings
                    .client_kind,
                Some(VcsClientKind::Git)
            );
        }

        state.remove_project(project_key);
        assert!(state.fork().get_project(&project_key).is_none());
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
