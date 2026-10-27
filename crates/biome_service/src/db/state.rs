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
    ParsedSourceUpdateMode, ProjectUpdateMode, SharedWorkspaceDb, ViewId, WorkspaceDb,
    WorkspaceDbData,
};
use crate::WorkspaceError;
#[cfg(feature = "module_graph")]
use crate::module_graph::{ModuleInfo, ModuleInfoKind};
use crate::projects::ProjectKey;
use crate::settings::Settings;
use crate::workspace::ClientId;
use biome_configuration::Configuration;
use biome_db::Source;
use biome_languages::DocumentFileSource;
use biome_resolver::FsWithResolverProxy;
#[cfg(feature = "module_graph")]
use biome_resolver::ResolverPathChanges;
use camino::{Utf8Path, Utf8PathBuf};
use ignore::gitignore::Gitignore;
use parking_lot::{Mutex, MutexGuard};
#[cfg(feature = "module_graph")]
use salsa::Setter;
use std::cell::Cell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::panic::resume_unwind;
use std::sync::atomic::AtomicBool;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tracing::error;

/// Represents the state of the database in the workspace.
pub struct DbState {
    storage: DbStorage,
    /// Collects the changes that need Salsa setters when this state is the
    /// view of a scan over Owned storage. It's `None` in every other state.
    scanner_changes: Option<Arc<ScannerEpochChanges>>,
    /// See [Self::defers_manifests].
    defers_manifests: bool,
}

/// The changes from the view of a scan that need Salsa setters. The scan
/// publishes them when it commits a phase.
#[derive(Default)]
struct ScannerEpochChanges {
    /// Set when the view adds or removes a project.
    projects_replaced: AtomicBool,
    /// The settings changes, in the order the view made them.
    project_updates: Mutex<Vec<ProjectUpdate>>,
    /// Set when the view changes the module map.
    #[cfg(feature = "module_graph")]
    module_graph_dirty: AtomicBool,
    /// The resolver path info to refresh.
    #[cfg(feature = "module_graph")]
    resolver_changes: Mutex<Vec<(Utf8PathBuf, ResolverPathChanges)>>,
    /// The kinds to set on the modules that existed before the scan.
    #[cfg(feature = "module_graph")]
    module_kinds: Mutex<Vec<(ModuleInfo, ModuleInfoKind)>>,
    /// The views that the view couldn't publish, because the inputs of their
    /// paths need setters.
    deferred_views: Mutex<Vec<DeferredView>>,
}

/// A change to the settings of a project.
enum ProjectUpdate {
    RootSettings {
        project_key: ProjectKey,
        settings: Settings,
    },
    NestedSettings {
        project_key: ProjectKey,
        path: Utf8PathBuf,
        settings: Settings,
    },
    /// The matchers of nested ignore files, from
    /// [WorkspaceDb::nested_git_ignores].
    NestedGitIgnores {
        project_key: ProjectKey,
        git_ignores: Vec<Gitignore>,
    },
}

impl ProjectUpdate {
    fn apply(self, db: &mut WorkspaceDb) {
        match self {
            Self::RootSettings {
                project_key,
                settings,
            } => {
                db.insert_root_settings_with_mode(project_key, settings, ProjectUpdateMode::Setters)
            }
            Self::NestedSettings {
                project_key,
                path,
                settings,
            } => db.insert_nested_setting_with_mode(
                project_key,
                path,
                settings,
                ProjectUpdateMode::Setters,
            ),
            Self::NestedGitIgnores {
                project_key,
                git_ignores,
            } => {
                // The only error is a missing project. During a scan, the
                // setter gate blocks the writes that remove projects.
                let _ = db.insert_nested_git_ignores_with_mode(
                    project_key,
                    &git_ignores,
                    ProjectUpdateMode::Setters,
                );
            }
        }
    }
}

/// A text to publish as a view of a path. See [WorkspaceDb::update_or_insert_view].
struct DeferredView {
    path: Utf8PathBuf,
    view: ViewId,
    text: Arc<str>,
    file_source_index: usize,
    configuration_index: Option<usize>,
}

enum DbStorage {
    Shared(SharedWorkspaceDb),
    Owned(OwnedDb),
}

/// A project scan, split into three phases: [ScanSetup], [ScanFiles] and
/// [ScanDeferredFiles].
///
/// The scanner works through [Self::view] during each phase. The view can't
/// run Salsa setters, so it records the changes that need them. Committing a
/// phase publishes these changes, and the next phase reads them. Within a
/// phase, queries may read outdated resolver path info or module
/// associations.
///
/// With Owned storage, the session holds the setter gate from start to end,
/// so the scan is a single epoch: other writes wait, while reads continue.
/// Dropping the session commits the current phase. With Shared storage, the
/// view publishes each change immediately, so a commit has nothing to publish.
pub(crate) struct ScanSession<'a, Phase> {
    // Fields drop in declaration order. `view` must drop before `writer`,
    // because `writer` runs Salsa setters when it drops.
    view: DbState,
    writer: ScanWriter<'a>,
    phase: PhantomData<Phase>,
}

/// The first phase of a scan: reads the configuration files, manifests and
/// ignore files of the project.
///
/// Its view defers every file whose inputs need Salsa setters, including the
/// manifests. No other file reads them before [ScanFiles] indexes them again.
pub(crate) struct ScanSetup;

/// The second phase of a scan: indexes again the files that [ScanSetup]
/// deferred, then the other files of the project and their dependencies.
///
/// Its view never defers the manifests and the JSON files, because the files
/// that it indexes after them read them.
pub(crate) struct ScanFiles;

/// The last phase of a scan: indexes again the files that [ScanFiles]
/// deferred, because updating their inputs needed Salsa setters.
pub(crate) struct ScanDeferredFiles;

impl<Phase> ScanSession<'_, Phase> {
    /// Returns the database state for the scanner to use during this phase.
    pub(crate) fn view(&self) -> &DbState {
        &self.view
    }
}

impl<'a, Phase> ScanSession<'a, Phase> {
    /// Publishes the changes of this phase, then starts the phase `Next` with
    /// a new view that reads them. Returns the paths of the files that this
    /// phase deferred, sorted and without duplicates.
    fn commit_into<Next>(self) -> (ScanSession<'a, Next>, Vec<Utf8PathBuf>) {
        let Self {
            view, mut writer, ..
        } = self;
        // `publish` runs Salsa setters, which wait until every handle to the
        // storage is dropped, and `view` holds one. Leaving `view` out of the
        // pattern would keep it alive until the function returns.
        drop(view);
        writer.publish();
        let mut deferred_paths = std::mem::take(&mut writer.deferred_paths);
        deferred_paths.sort_unstable();
        deferred_paths.dedup();
        let session = ScanSession {
            view: writer.view(false),
            writer,
            phase: PhantomData,
        };
        (session, deferred_paths)
    }
}

impl<'a> ScanSession<'a, ScanSetup> {
    /// Publishes the changes of the setup phase, such as project settings, and
    /// starts [ScanFiles]. Returns the paths of the deferred files, which
    /// [ScanFiles] indexes again first.
    pub(crate) fn commit(self) -> (ScanSession<'a, ScanFiles>, Vec<Utf8PathBuf>) {
        self.commit_into()
    }
}

impl<'a> ScanSession<'a, ScanFiles> {
    /// Publishes the changes of the files phase and starts [ScanDeferredFiles].
    /// Returns the paths of the deferred files, which [ScanDeferredFiles]
    /// indexes again.
    pub(crate) fn commit(self) -> (ScanSession<'a, ScanDeferredFiles>, Vec<Utf8PathBuf>) {
        self.commit_into()
    }
}

impl ScanSession<'_, ScanDeferredFiles> {
    /// Publishes the changes of the last phase and ends the session.
    pub(crate) fn finish(self) {}
}

/// Publishes the changes that the views of a [ScanSession] record. Dropping
/// it publishes the pending changes.
struct ScanWriter<'a> {
    storage: ScanStorage<'a>,
    changes: Arc<ScannerEpochChanges>,
    /// The paths of the files that the view of the current phase deferred.
    /// Committing the phase returns them.
    deferred_paths: Vec<Utf8PathBuf>,
}

enum ScanStorage<'a> {
    Shared(SharedWorkspaceDb),
    Owned {
        owner: &'a OwnedDb,
        setter_guard: MutexGuard<'a, ()>,
    },
}

impl ScanWriter<'_> {
    /// Returns a new view of the database. The view records the changes that
    /// need Salsa setters. See [DbState::defers_manifests] for
    /// `defers_manifests`.
    fn view(&self, defers_manifests: bool) -> DbState {
        match &self.storage {
            ScanStorage::Shared(shared_db) => DbState {
                storage: DbStorage::Shared(shared_db.clone()),
                scanner_changes: None,
                defers_manifests,
            },
            ScanStorage::Owned { owner, .. } => {
                let shared_db = {
                    let db = owner.db.lock();
                    SharedWorkspaceDb::from_workspace_db(db.clone())
                };
                DbState {
                    storage: DbStorage::Shared(shared_db),
                    scanner_changes: Some(self.changes.clone()),
                    defers_manifests,
                }
            }
        }
    }

    /// Publishes the changes that the views recorded, with Salsa setters.
    ///
    /// Call it only after every view is dropped, because a setter waits until
    /// no view is alive.
    fn publish(&mut self) {
        let ScanStorage::Owned {
            owner,
            setter_guard,
        } = &self.storage
        else {
            return;
        };
        let changes = &self.changes;
        let projects_replaced = changes.projects_replaced.swap(false, Ordering::AcqRel);
        let project_updates = std::mem::take(&mut *changes.project_updates.lock());
        let deferred_views = std::mem::take(&mut *changes.deferred_views.lock());
        self.deferred_paths
            .extend(deferred_views.iter().map(|deferred| deferred.path.clone()));
        #[cfg(feature = "module_graph")]
        let resolver_changes = std::mem::take(&mut *changes.resolver_changes.lock());
        #[cfg(feature = "module_graph")]
        let module_kinds = std::mem::take(&mut *changes.module_kinds.lock());
        #[cfg(feature = "module_graph")]
        let module_graph_dirty = changes.module_graph_dirty.swap(false, Ordering::AcqRel);

        let projects_changed = projects_replaced || !project_updates.is_empty();
        #[cfg(feature = "module_graph")]
        let needs_setter = projects_changed
            || !deferred_views.is_empty()
            || module_graph_dirty
            || !resolver_changes.is_empty()
            || !module_kinds.is_empty();
        #[cfg(not(feature = "module_graph"))]
        let needs_setter = projects_changed || !deferred_views.is_empty();
        if needs_setter {
            owner.with_setter_gate_held(setter_guard, |db| {
                for update in project_updates {
                    update.apply(db);
                }
                if projects_changed {
                    db.invalidate_projects();
                }
                for deferred in deferred_views {
                    db.update_or_insert_view(
                        &deferred.path,
                        deferred.view,
                        deferred.text,
                        deferred.file_source_index,
                        deferred.configuration_index,
                        ParsedSourceUpdateMode::Setters,
                    );
                }
                #[cfg(feature = "module_graph")]
                {
                    db.sync_resolver_paths(resolver_changes);
                    for (module, kind) in module_kinds {
                        module.set_kind(db).to(kind);
                    }
                    if module_graph_dirty {
                        db.invalidate_module_graph();
                    }
                }
            });
        }
    }
}

impl Drop for ScanWriter<'_> {
    fn drop(&mut self) {
        self.publish();
    }
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
    /// Serializes the setters, and keeps them from running during a scan.
    /// Queued setters do not announce pending writes while waiting here.
    setter_lock: Mutex<()>,
    /// The collections shared between the database and all its clones. See
    /// [WorkspaceDbData].
    data: WorkspaceDbData,
    /// Whether a setter currently owns the setter gate and is applying, or
    /// waiting to apply, an update.
    pending_setters: AtomicUsize,
    /// Number of setter attempts observed before acquiring the setter gate.
    #[cfg(test)]
    setter_gate_attempts: AtomicUsize,
}

/// Clears the pending-setter signal when a setter completes or unwinds.
struct PendingSetterGuard<'a>(&'a AtomicUsize);

impl Drop for PendingSetterGuard<'_> {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::Release);
    }
}

impl OwnedDb {
    fn new(db: WorkspaceDb) -> Self {
        let data = db.data();
        Self {
            db: Mutex::new(db),
            setter_lock: Mutex::new(()),
            data,
            pending_setters: AtomicUsize::new(0),
            #[cfg(test)]
            setter_gate_attempts: AtomicUsize::new(0),
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

        #[cfg(test)]
        self.setter_gate_attempts.fetch_add(1, Ordering::Release);
        let setter_guard = self.setter_lock.lock();
        self.with_setter_gate_held(&setter_guard, f)
    }

    /// Runs a setter after the caller has acquired this database's setter gate.
    ///
    /// The guard keeps queued setters from announcing a pending write until the
    /// current operation has completed.
    fn with_setter_gate_held<R>(
        &self,
        _setter_guard: &MutexGuard<'_, ()>,
        f: impl FnOnce(&mut WorkspaceDb) -> R,
    ) -> R {
        self.pending_setters.fetch_add(1, Ordering::Release);
        let _guard = PendingSetterGuard(&self.pending_setters);
        let mut db = self.db.lock();
        f(&mut db)
    }
}

impl DbState {
    /// Creates a Shared-mode database state, used by short-lived operations
    /// such as the CLI.
    ///
    /// The resolver reads path info from `fs`.
    pub fn new(fs: Arc<dyn FsWithResolverProxy>) -> Self {
        Self {
            storage: DbStorage::Shared(SharedWorkspaceDb::new(fs)),
            scanner_changes: None,
            defers_manifests: false,
        }
    }

    /// Creates an Owned-mode database state, used by the long-lived LSP.
    ///
    /// The resolver reads path info from `fs`.
    pub fn lsp(fs: Arc<dyn FsWithResolverProxy>) -> Self {
        Self {
            storage: DbStorage::Owned(OwnedDb::new(WorkspaceDb::new(fs))),
            scanner_changes: None,
            defers_manifests: false,
        }
    }

    /// Starts a project scan. See [ScanSession].
    ///
    /// With Owned storage, this waits for the setter gate. It doesn't announce
    /// a pending Salsa write, so reads continue meanwhile.
    pub(crate) fn scan_session(&self) -> ScanSession<'_, ScanSetup> {
        let storage = match &self.storage {
            DbStorage::Shared(shared_db) => ScanStorage::Shared(shared_db.clone()),
            DbStorage::Owned(owner) => ScanStorage::Owned {
                owner,
                setter_guard: owner.setter_lock.lock(),
            },
        };
        let writer = ScanWriter {
            storage,
            changes: Arc::default(),
            deferred_paths: Vec::new(),
        };
        ScanSession {
            view: writer.view(true),
            writer,
            phase: PhantomData,
        }
    }

    /// Returns whether the scanner can defer the manifests and the JSON files
    /// that it indexes in this state, like any other file.
    ///
    /// Only the view of [ScanSetup] allows it, because no other file reads
    /// them before [ScanFiles] indexes them again. Other states publish them
    /// right away, because the files that the scanner indexes next read them.
    pub(crate) fn defers_manifests(&self) -> bool {
        self.defers_manifests
    }

    pub(crate) fn fork(&self) -> DbReadGuard {
        match &self.storage {
            DbStorage::Shared(shared_db) => DbReadGuard::new(shared_db.fork(), false),
            DbStorage::Owned(db) => DbReadGuard::new(db.fork(), true),
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
            DbStorage::Shared(shared_db) => {
                if let Some(changes) = &self.scanner_changes {
                    changes
                        .project_updates
                        .lock()
                        .push(ProjectUpdate::NestedSettings {
                            project_key,
                            path,
                            settings,
                        });
                } else {
                    shared_db.fork().insert_nested_setting_with_mode(
                        project_key,
                        path,
                        settings,
                        ProjectUpdateMode::Replace,
                    );
                }
            }
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
            DbStorage::Shared(db) => {
                if let Some(changes) = &self.scanner_changes {
                    changes
                        .project_updates
                        .lock()
                        .push(ProjectUpdate::RootSettings {
                            project_key,
                            settings,
                        });
                } else {
                    db.fork().insert_root_settings_with_mode(
                        project_key,
                        settings,
                        ProjectUpdateMode::Replace,
                    );
                }
            }
            DbStorage::Owned(db) => db.with_setter(|db| {
                db.insert_root_settings_with_mode(project_key, settings, ProjectUpdateMode::Setters)
            }),
        }
    }

    /// Inserts project
    pub fn insert_project(&self, path: Utf8PathBuf) -> ProjectKey {
        match &self.storage {
            DbStorage::Shared(shared_db) => {
                let project_key = shared_db.fork().insert_project(path);
                self.record_projects_replaced();
                project_key
            }
            DbStorage::Owned(db) => db.with_setter(|db| {
                let projects = db.projects.pin().len();
                let project_key = db.insert_project(path);
                if db.projects.pin().len() != projects {
                    db.invalidate_projects();
                }
                project_key
            }),
        }
    }

    /// Removes the project with the given key.
    pub fn remove_project(&self, project_key: ProjectKey) {
        match &self.storage {
            DbStorage::Shared(shared_db) => {
                shared_db.data().remove_project(project_key);
                self.record_projects_replaced();
            }
            DbStorage::Owned(db) => db.with_setter(|db| {
                db.remove_project(project_key);
                db.invalidate_projects();
            }),
        }
    }

    pub fn store_nested_ignore_patterns(
        &self,
        project_key: ProjectKey,
        payload: Vec<(Utf8PathBuf, Vec<String>)>,
    ) -> Result<(), WorkspaceError> {
        match &self.storage {
            DbStorage::Shared(shared_db) => {
                let mut db = shared_db.fork();
                let git_ignores = db.nested_git_ignores(project_key, &payload)?;
                if let Some(changes) = &self.scanner_changes {
                    changes
                        .project_updates
                        .lock()
                        .push(ProjectUpdate::NestedGitIgnores {
                            project_key,
                            git_ignores,
                        });
                    Ok(())
                } else {
                    db.insert_nested_git_ignores_with_mode(
                        project_key,
                        &git_ignores,
                        ProjectUpdateMode::Replace,
                    )
                }
            }
            DbStorage::Owned(db) => db.with_setter(|db| {
                let git_ignores = db.nested_git_ignores(project_key, &payload)?;
                db.insert_nested_git_ignores_with_mode(
                    project_key,
                    &git_ignores,
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

    /// Publishes `text` as `view` of `path`, and returns the view of `path`
    /// that the services shared by the clients read. See
    /// [FileViews](super::FileViews).
    ///
    /// The text is parsed as the file source at `file_source_index`, with the
    /// `inline_configuration` of the client on top of the settings. It isn't
    /// parsed here: the parse runs the first time a query reads it.
    pub(crate) fn update_file_view(
        &self,
        path: &Utf8Path,
        view: ViewId,
        text: Arc<str>,
        file_source_index: usize,
        inline_configuration: Option<Configuration>,
    ) -> Source {
        let configuration_index = inline_configuration.map(|configuration| {
            self.shared_data()
                .insert_inline_configuration(configuration)
        });
        self.publish_file_view(path, view, text, file_source_index, configuration_index)
    }

    /// Publishes `text` as `view` of `path`, like [Self::update_file_view].
    ///
    /// In the view of a scan, it defers the update when the inputs of `path`
    /// hold other text, because updating them needs Salsa setters. The scan
    /// publishes the text when it commits the phase, and this returns `None`.
    /// See [ScanSession].
    pub(crate) fn update_file_view_or_defer(
        &self,
        path: &Utf8Path,
        view: ViewId,
        text: Arc<str>,
        file_source_index: usize,
        inline_configuration: Option<Configuration>,
    ) -> Option<Source> {
        let configuration_index = inline_configuration.map(|configuration| {
            self.shared_data()
                .insert_inline_configuration(configuration)
        });
        if let DbStorage::Shared(shared_db) = &self.storage
            && let Some(changes) = &self.scanner_changes
            && shared_db.fork().view_update_needs_setters(
                path,
                view,
                &text,
                file_source_index,
                configuration_index,
            )
        {
            changes.deferred_views.lock().push(DeferredView {
                path: path.to_path_buf(),
                view,
                text,
                file_source_index,
                configuration_index,
            });
            return None;
        }
        Some(self.publish_file_view(path, view, text, file_source_index, configuration_index))
    }

    /// Publishes `text` as `view` of `path`, with the inline configuration at
    /// `configuration_index`. See [Self::update_file_view].
    fn publish_file_view(
        &self,
        path: &Utf8Path,
        view: ViewId,
        text: Arc<str>,
        file_source_index: usize,
        configuration_index: Option<usize>,
    ) -> Source {
        match &self.storage {
            DbStorage::Shared(shared_db) => {
                let mut db = shared_db.fork();
                // Only the view of a scan over Owned storage reuses inputs,
                // because the database it reads outlives the scan. Other Shared
                // storage lives for one operation, so it allocates new inputs.
                let reused = self.scanner_changes.as_ref().and_then(|_| {
                    db.reuse_view(path, view, &text, file_source_index, configuration_index)
                });
                let file = reused.unwrap_or_else(|| {
                    db.update_or_insert_view(
                        path,
                        view,
                        text,
                        file_source_index,
                        configuration_index,
                        ParsedSourceUpdateMode::Replace,
                    )
                });
                // The update may publish another parsed source for `path`.
                #[cfg(feature = "module_graph")]
                self.record_resolver_changes(shared_db.data().resolver_source_change(path));
                file
            }
            DbStorage::Owned(db) => db.with_setter(|db| {
                db.update_or_insert_view(
                    path,
                    view,
                    text,
                    file_source_index,
                    configuration_index,
                    ParsedSourceUpdateMode::Setters,
                )
            }),
        }
    }

    /// Removes `view` of `path`, and removes the file when it has no view
    /// left.
    #[cfg(feature = "module_graph")]
    pub(crate) fn remove_file_view(&self, path: &Utf8Path, view: ViewId) {
        match &self.storage {
            DbStorage::Shared(shared_db) => {
                shared_db
                    .fork()
                    .remove_view(path, view, ParsedSourceUpdateMode::Replace);
                // The replacement publishes another `Source` input for `path`.
                #[cfg(feature = "module_graph")]
                self.record_resolver_changes(shared_db.data().resolver_source_change(path));
            }
            DbStorage::Owned(db) => db.with_setter(|db| {
                db.remove_view(path, view, ParsedSourceUpdateMode::Setters);
            }),
        }
    }

    /// Removes the editor view that `client` has of `path`. See
    /// [WorkspaceDb::remove_editor_view].
    pub(crate) fn remove_editor_view(
        &self,
        path: &Utf8Path,
        client: ClientId,
        keep_disk_view: bool,
    ) {
        match &self.storage {
            DbStorage::Shared(shared_db) => {
                shared_db.fork().remove_editor_view(
                    path,
                    client,
                    keep_disk_view,
                    ParsedSourceUpdateMode::Replace,
                );
                // The replacement publishes another `Source` input for `path`.
                #[cfg(feature = "module_graph")]
                self.record_resolver_changes(shared_db.data().resolver_source_change(path));
            }
            DbStorage::Owned(db) => db.with_setter(|db| {
                db.remove_editor_view(
                    path,
                    client,
                    keep_disk_view,
                    ParsedSourceUpdateMode::Setters,
                );
            }),
        }
    }

    /// Returns the index of `file_source` in the table of file sources of the
    /// database, and inserts it first if the table doesn't have it.
    pub(crate) fn insert_file_source(&self, file_source: DocumentFileSource) -> usize {
        self.shared_data().insert_source(file_source)
    }

    /// Keeps a node cache for `path`. See [WorkspaceDbData::retain_node_cache].
    pub(crate) fn retain_node_cache(&self, path: &Utf8Path) {
        self.shared_data().retain_node_cache(path);
    }

    /// Drops the node cache of `path`, if there is one.
    pub(crate) fn remove_node_cache(&self, path: &Utf8Path) {
        self.shared_data().remove_node_cache(path);
    }

    /// Returns whether a node cache is kept for `path`.
    #[cfg(test)]
    pub(crate) fn has_node_cache(&self, path: &Utf8Path) -> bool {
        self.shared_data().node_caches.pin().contains_key(path)
    }

    pub(crate) fn unload_path(&self, path: &Utf8Path) {
        match &self.storage {
            DbStorage::Shared(shared_db) => {
                let data = shared_db.data();
                data.unload_path(path);
                #[cfg(feature = "module_graph")]
                {
                    self.mark_module_graph_dirty();
                    self.record_resolver_changes(data.resolver_changes_within(path));
                }
            }
            DbStorage::Owned(db) => db.with_setter(|db| db.unload_path(path)),
        }
    }

    /// Removes the disk views of the files within `path`, and the modules
    /// within it. The files that a client has open keep their editor views.
    ///
    /// Use this when the filesystem reports that `path` was removed.
    pub(crate) fn unload_disk_views(&self, path: &Utf8Path) {
        match &self.storage {
            DbStorage::Shared(shared_db) => {
                shared_db
                    .fork()
                    .remove_disk_views_within(path, ParsedSourceUpdateMode::Replace);
                let data = shared_db.data();
                data.unload_modules(path);
                #[cfg(feature = "module_graph")]
                {
                    self.mark_module_graph_dirty();
                    self.record_resolver_changes(data.resolver_changes_within(path));
                }
            }
            DbStorage::Owned(db) => db.with_setter(|db| {
                db.remove_disk_views_within(path, ParsedSourceUpdateMode::Setters);
                db.unload_modules(path);
            }),
        }
    }

    /// Refreshes the resolver path info of the given paths from the filesystem.
    ///
    /// Call this when the filesystem reports a change, for example from the
    /// watcher. Outside a scan, Owned storage applies the changes immediately,
    /// and skips the write when the path info is already up to date. In the
    /// view of a scan, the scan applies them when it commits the phase. Shared
    /// storage outside a scan never refreshes path info, because it doesn't
    /// observe filesystem changes.
    #[cfg(feature = "module_graph")]
    pub(crate) fn sync_resolver_paths(&self, changes: Vec<(Utf8PathBuf, ResolverPathChanges)>) {
        match &self.storage {
            DbStorage::Shared(_) => self.record_resolver_changes(changes),
            DbStorage::Owned(db) => {
                let needs_sync = db.fork().resolver_paths_need_sync(&changes);
                if needs_sync {
                    db.with_setter(|db| db.sync_resolver_paths(changes));
                }
            }
        }
    }

    /// Returns the resolver changes to report for the known paths equal to or
    /// inside `path`.
    #[cfg(feature = "module_graph")]
    pub(crate) fn resolver_changes_within(
        &self,
        path: &Utf8Path,
    ) -> Vec<(Utf8PathBuf, ResolverPathChanges)> {
        self.data().resolver_changes_within(path)
    }

    /// Returns the change to report when the filesystem reports that `path`
    /// was created or modified.
    #[cfg(feature = "module_graph")]
    pub(crate) fn resolver_changes_for_modified_path(
        &self,
        path: &Utf8Path,
    ) -> ResolverPathChanges {
        self.data().resolver_changes_for_modified_path(path)
    }

    /// Returns the resolver changes to report for the known paths inside the
    /// `node_modules` directories that `directory` can resolve packages from.
    #[cfg(feature = "module_graph")]
    pub(crate) fn resolver_changes_in_visible_node_modules(
        &self,
        directory: &Utf8Path,
    ) -> Vec<(Utf8PathBuf, ResolverPathChanges)> {
        self.data()
            .resolver_changes_in_visible_node_modules(directory)
    }

    /// Returns whether the resolver reads the content of `path`.
    #[cfg(feature = "module_graph")]
    pub(crate) fn is_resolver_manifest(&self, path: &Utf8Path) -> bool {
        self.data().is_resolver_manifest(path)
    }

    /// Returns handles to the collections shared by every database fork.
    #[cfg(feature = "module_graph")]
    fn data(&self) -> WorkspaceDbData {
        self.shared_data()
    }

    fn shared_data(&self) -> WorkspaceDbData {
        match &self.storage {
            DbStorage::Shared(shared_db) => shared_db.data(),
            DbStorage::Owned(db) => db.data.clone(),
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
                // The view of a scan can't run Salsa setters, so the scan sets
                // the kind of an existing module when it commits the phase.
                if let Some(changes) = &self.scanner_changes
                    && let Some(module) = db.get_module(&path)
                {
                    changes.module_kinds.lock().push((module, kind));
                    return module;
                }
                let module = ModuleInfo::new(&db, path.clone(), kind);
                db.data().insert_module(path, module);
                self.mark_module_graph_dirty();
                module
            }
            DbStorage::Owned(db) => db.with_setter(|db| db.update_or_insert_module(path, kind)),
        }
    }

    #[cfg(feature = "module_graph")]
    pub(crate) fn remove_module(&self, path: &Utf8Path) {
        match &self.storage {
            DbStorage::Shared(shared_db) => {
                shared_db.fork().data().remove_module(path);
                self.mark_module_graph_dirty();
            }
            DbStorage::Owned(db) => db.with_setter(|db| db.remove_module(path)),
        }
    }

    /// Records that a project was replaced.
    ///
    /// In the view of a scan, the scan advances the projects generation when
    /// it commits the phase. Outside a scan, Shared storage can't run Salsa
    /// setters, so queries that looked up a project keep their results until
    /// another input changes.
    fn record_projects_replaced(&self) {
        if let Some(changes) = &self.scanner_changes {
            changes.projects_replaced.store(true, Ordering::Release);
        }
    }

    /// Records that a replacement update changed the external module map.
    #[cfg(feature = "module_graph")]
    fn mark_module_graph_dirty(&self) {
        if let Some(changes) = &self.scanner_changes {
            changes.module_graph_dirty.store(true, Ordering::Release);
        }
    }

    /// Records resolver changes made through a replacement update.
    ///
    /// In the view of a scan, the scan applies them when it commits the phase.
    /// Outside a scan, Shared storage discards them: it can't run Salsa
    /// setters, and the operations using it don't observe filesystem changes.
    #[cfg(feature = "module_graph")]
    fn record_resolver_changes(
        &self,
        changes: impl IntoIterator<Item = (Utf8PathBuf, ResolverPathChanges)>,
    ) {
        if let Some(scanner_changes) = &self.scanner_changes {
            scanner_changes.resolver_changes.lock().extend(changes);
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

    /// Returns the number of setters that reached the setter gate.
    #[cfg(test)]
    pub(crate) fn setter_gate_attempts(&self) -> usize {
        match &self.storage {
            DbStorage::Shared(_) => 0,
            DbStorage::Owned(db) => db.setter_gate_attempts.load(Ordering::Acquire),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::projects::ProjectDb;
    use biome_configuration::vcs::VcsClientKind;
    use biome_languages::JsFileSource;
    use camino::Utf8PathBuf;
    use salsa::plumbing::AsId;
    use std::panic::AssertUnwindSafe;
    use std::sync::{Arc, Barrier, mpsc};
    use std::thread;
    use std::time::Duration;

    static_assertions::assert_not_impl_any!(DbReadGuard: Send);

    fn test_fs() -> Arc<dyn FsWithResolverProxy> {
        Arc::new(biome_fs::MemoryFileSystem::default())
    }

    /// Returns the index of the JavaScript module file source in `state`.
    fn js(state: &DbState) -> usize {
        state.insert_file_source(DocumentFileSource::Js(JsFileSource::js_module()))
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
        let state = Arc::new(DbState::lsp(test_fs()));
        let path = Utf8PathBuf::from("test.js");
        // Insert the file first: only updates to files the database already
        // knows about are applied through salsa setters.
        state.update_file_view(&path, ViewId::Disk, "let a = 1;".into(), js(&state), None);

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
                state
                    .shared_data()
                    .insert_source(DocumentFileSource::Js(JsFileSource::js_script()));
                drop(db);
            })
        };

        let setter = {
            let state = state.clone();
            let path = path.clone();
            thread::spawn(move || {
                clone_taken.wait();
                state.update_file_view(&path, ViewId::Disk, "let b = 2;".into(), js(&state), None);
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
        let state = Arc::new(DbState::lsp(test_fs()));
        let path = Utf8PathBuf::from("test.js");
        state.update_file_view(&path, ViewId::Disk, "let a = 1;".into(), js(&state), None);

        // Hold a clone so the setter below has to wait.
        let db = state.fork();

        let setter = {
            let state = state.clone();
            let path = path.clone();
            thread::spawn(move || {
                state.update_file_view(&path, ViewId::Disk, "let b = 2;".into(), js(&state), None);
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

    /// A queued setter must not extend read cancellation while another setter
    /// already owns the database. Only the setter that can make progress should
    /// contribute to `pending_setters`.
    #[test]
    fn owned_storage_queues_only_one_pending_setter() {
        let state = Arc::new(DbState::lsp(test_fs()));
        let path = Utf8PathBuf::from("test.js");
        state.update_file_view(&path, ViewId::Disk, "let a = 1;".into(), js(&state), None);

        let db = state.fork();
        let first_setter = {
            let state = state.clone();
            let path = path.clone();
            thread::spawn(move || {
                state.update_file_view(&path, ViewId::Disk, "let b = 2;".into(), js(&state), None);
            })
        };
        wait_for_pending_setter(&state);

        let second_start = Arc::new(Barrier::new(2));
        let second_setter = {
            let state = state.clone();
            let path = path.clone();
            let second_start = second_start.clone();
            thread::spawn(move || {
                second_start.wait();
                state.update_file_view(&path, ViewId::Disk, "let c = 3;".into(), js(&state), None);
            })
        };
        second_start.wait();

        let deadline = std::time::Instant::now() + Duration::from_secs(1);
        let second_was_pending = loop {
            if state.pending_setters() > 1 {
                break true;
            }
            if std::time::Instant::now() >= deadline {
                break false;
            }
            thread::yield_now();
        };

        drop(db);
        first_setter.join().unwrap();
        second_setter.join().unwrap();

        assert!(
            !second_was_pending,
            "a setter waiting behind another setter prolonged read cancellation"
        );
    }

    /// Scanner bookkeeping must remain available while another thread is
    /// publishing database updates. Unlike a tracked database fork, checking
    /// shared module membership must not unwind when a setter is pending.
    #[cfg(feature = "module_graph")]
    #[test]
    fn untracked_module_membership_does_not_wait_for_pending_setters() {
        let state = Arc::new(DbState::lsp(test_fs()));
        let path = Utf8PathBuf::from("test.js");
        state.update_file_view(&path, ViewId::Disk, "let a = 1;".into(), js(&state), None);

        let db = state.fork();
        let setter = {
            let state = state.clone();
            let path = path.clone();
            thread::spawn(move || {
                state.update_file_view(&path, ViewId::Disk, "let b = 2;".into(), js(&state), None);
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
        let state = DbState::lsp(test_fs());
        let path = Utf8PathBuf::from("test.js");
        let _db = state.fork();

        state.update_file_view(&path, ViewId::Disk, "let a = 1;".into(), js(&state), None);
    }

    #[test]
    fn replacement_update_does_not_cancel_concurrent_reads() {
        let state = DbState::new(test_fs());
        let path = Utf8PathBuf::from("test.js");
        let db = state.fork();

        state.update_file_view(&path, ViewId::Disk, "let a = 1;".into(), js(&state), None);

        assert_eq!(state.pending_setters(), 0);
        assert!(db.get_parsed_source(&path).is_some());
    }

    /// Shared storage lives for one operation, so publishing a file again
    /// allocates a new input, even after the file lost its last view.
    #[test]
    fn shared_storage_allocates_inputs_of_reopened_file() {
        let state = DbState::new(test_fs());
        let path = Utf8PathBuf::from("test.js");
        let client = ClientId::new(1);
        let open = || {
            state.update_file_view(
                &path,
                ViewId::Editor(client),
                "let a = 1;".into(),
                js(&state),
                None,
            )
        };

        let first = open();
        state.remove_editor_view(&path, client, false);
        let second = open();

        assert_ne!(first.as_id(), second.as_id());
    }

    /// Committing the setup phase applies its settings to the parses that the
    /// next phases read, including the parses cached before the scan.
    #[cfg(feature = "lang_json")]
    #[test]
    fn scan_setup_commit_applies_settings_to_cached_parses() {
        use biome_parser::db::SourceExt;

        let state = DbState::lsp(test_fs());
        let project_key = state.insert_project(Utf8PathBuf::from("/project"));
        let source = state.update_file_view(
            Utf8Path::new("/project/data.json"),
            ViewId::Disk,
            "[1,]".into(),
            state.insert_file_source(DocumentFileSource::Json(
                biome_languages::JsonFileSource::json(),
            )),
            None,
        );
        assert!(source.has_errors(&*state.fork()));

        let session = state.scan_session();
        let mut settings = Settings::default();
        settings.languages.json.parser.allow_trailing_commas = Some(true.into());
        session.view().insert_root_settings(project_key, settings);
        assert!(source.has_errors(&*session.view().fork()));
        let (session, _) = session.commit();

        assert!(!source.has_errors(&*session.view().fork()));
    }

    #[test]
    fn shared_storage_project_lifecycle_replaces_inputs_without_waiting_for_reads() {
        let state = Arc::new(DbState::new(test_fs()));
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
        let state = DbState::lsp(test_fs());
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
        let state = Arc::new(DbState::lsp(test_fs()));
        let path = Utf8PathBuf::from("test.js");
        state.update_file_view(&path, ViewId::Disk, "let a = 1;".into(), js(&state), None);

        let db = state.fork();
        let (done_tx, done_rx) = mpsc::channel();

        let setter = {
            let state = state.clone();
            let path = path.clone();
            thread::spawn(move || {
                state.update_file_view(&path, ViewId::Disk, "let b = 2;".into(), js(&state), None);
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

    #[cfg(feature = "module_graph")]
    mod resolver {
        use super::*;
        use biome_fs::MemoryFileSystem;
        use biome_resolver::{
            PathInfo, ResolveError, ResolverDb, ResolverPathChange, ResolverPathData,
        };

        const MANIFEST: &str = "/project/package.json";
        const SOURCE: &str = "/project/index.js";

        fn resolver_fs(fs: &MemoryFileSystem) -> Arc<dyn FsWithResolverProxy> {
            Arc::new(MemoryFileSystem::from_files(fs.files.0.clone()))
        }

        /// Returns the index of the JSON file source in `state`.
        fn json(state: &DbState) -> usize {
            state.insert_file_source(DocumentFileSource::Json(
                biome_languages::JsonFileSource::json(),
            ))
        }

        /// Looks up the resolver path info of `path`, like a resolver query does.
        fn lookup(state: &DbState, path: &str) -> ResolverPathData {
            let db = state.fork();
            db.resolver_paths().get_or_create(&*db, Utf8Path::new(path))
        }

        /// Reads `path` as a package manifest, like a resolver query does.
        fn read_manifest(state: &DbState, path: &str) -> ResolverPathData {
            let data = lookup(state, path);
            let db = state.fork();
            let _ = biome_resolver::package_json_for_path(&*db, data);
            data
        }

        fn revision(state: &DbState, data: ResolverPathData) -> u64 {
            data.revision(&*state.fork())
        }

        fn info(state: &DbState, data: ResolverPathData) -> Result<PathInfo, ResolveError> {
            data.info(&*state.fork()).clone()
        }

        #[test]
        fn owned_updates_notify_the_resolver_only_for_new_manifest_sources() {
            let fs = MemoryFileSystem::default();
            let state = DbState::lsp(resolver_fs(&fs));
            let manifest = read_manifest(&state, MANIFEST);
            let source = lookup(&state, SOURCE);
            let setters = state.setter_gate_attempts();

            state.update_file_view(
                Utf8Path::new(SOURCE),
                ViewId::Disk,
                "".into(),
                js(&state),
                None,
            );
            state.update_file_view(
                Utf8Path::new(SOURCE),
                ViewId::Disk,
                "a".into(),
                js(&state),
                None,
            );
            assert_eq!(
                revision(&state, source),
                0,
                "sources aren't read by the resolver"
            );

            state.update_file_view(
                Utf8Path::new(MANIFEST),
                ViewId::Disk,
                "{}".into(),
                json(&state),
                None,
            );
            assert_eq!(revision(&state, manifest), 1);
            state.update_file_view(
                Utf8Path::new(MANIFEST),
                ViewId::Disk,
                "[]".into(),
                json(&state),
                None,
            );
            assert_eq!(
                revision(&state, manifest),
                1,
                "content changes of an indexed manifest are tracked by its parsed source"
            );

            assert_eq!(
                state.setter_gate_attempts() - setters,
                4,
                "each update is a single write"
            );
        }

        #[test]
        fn owned_sync_skips_the_write_when_path_info_is_current() {
            let fs = MemoryFileSystem::default();
            fs.insert(SOURCE.into(), "");
            let state = DbState::lsp(resolver_fs(&fs));
            let source = lookup(&state, SOURCE);
            let setters = state.setter_gate_attempts();

            state.sync_resolver_paths(vec![(SOURCE.into(), ResolverPathChange::Kind.into())]);
            assert_eq!(state.setter_gate_attempts(), setters);

            fs.remove(Utf8Path::new(SOURCE));
            state.sync_resolver_paths(vec![(SOURCE.into(), ResolverPathChange::Kind.into())]);
            assert_eq!(state.setter_gate_attempts(), setters + 1);
            assert_eq!(info(&state, source), Err(ResolveError::NotFound));
        }

        #[test]
        fn owned_unload_marks_known_paths_as_missing() {
            let fs = MemoryFileSystem::default();
            fs.insert(SOURCE.into(), "");
            let state = DbState::lsp(resolver_fs(&fs));
            let directory = lookup(&state, "/project");
            let source = lookup(&state, SOURCE);

            fs.remove(Utf8Path::new(SOURCE));
            state.unload_path(Utf8Path::new("/project"));

            assert_eq!(info(&state, source), Err(ResolveError::NotFound));
            assert_eq!(
                lookup(&state, SOURCE),
                source,
                "unloading keeps the input, so dependent queries observe the change"
            );
            assert_eq!(info(&state, directory), Err(ResolveError::NotFound));
        }

        #[test]
        fn scan_session_applies_resolver_changes_when_it_ends() {
            let fs = MemoryFileSystem::default();
            let state = DbState::lsp(resolver_fs(&fs));
            let manifest = read_manifest(&state, MANIFEST);
            let source = lookup(&state, SOURCE);

            {
                let session = state.scan_session();
                session.view().update_file_view(
                    Utf8Path::new(MANIFEST),
                    ViewId::Disk,
                    "{}".into(),
                    json(&state),
                    None,
                );
                fs.insert(SOURCE.into(), "");
                session
                    .view()
                    .sync_resolver_paths(vec![(SOURCE.into(), ResolverPathChange::Kind.into())]);

                let db = session.view().fork();
                assert_eq!(manifest.revision(&*db), 0);
                assert_eq!(source.info(&*db), &Err(ResolveError::NotFound));
            }

            assert_eq!(revision(&state, manifest), 1);
            assert_eq!(info(&state, source), Ok(PathInfo::File));
        }

        #[test]
        fn shared_mode_never_refreshes_resolver_path_info() {
            let fs = MemoryFileSystem::default();
            let state = DbState::new(resolver_fs(&fs));
            let manifest = read_manifest(&state, MANIFEST);
            let source = lookup(&state, SOURCE);

            fs.insert(SOURCE.into(), "");
            state.sync_resolver_paths(vec![(SOURCE.into(), ResolverPathChange::Kind.into())]);
            state.update_file_view(
                Utf8Path::new(MANIFEST),
                ViewId::Disk,
                "{}".into(),
                json(&state),
                None,
            );

            assert_eq!(info(&state, source), Err(ResolveError::NotFound));
            assert_eq!(revision(&state, manifest), 0);
        }
    }
}
