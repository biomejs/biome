//! The document lifecycle: the views of a file that the workspace holds, from
//! its first disk or editor view, through publication, updates and view
//! selection, to the removal of its last view.
//!
//! The workspace holds every view in two representations. The metadata holds
//! what clients read without a database: the content, the version, the inline
//! configuration and whether the content is parsed. The database holds the
//! views that queries parse, so the services shared by the clients, such as
//! the module graph, read them too. [DocumentLifecycle] owns both, and every
//! publication and removal updates them together. See
//! [views](crate::workspace::views) for which view each client reads.
//!
//! Requests of a client go through [ClientDocuments]. The scanner, the watcher
//! and the project lifecycle go through [SharedDocuments], because the disk
//! view of a file is shared by all the clients.
//!
//! The policy stays with the callers: which file source a text is parsed as,
//! whether a file is too large or ignored, which project owns a file, and when
//! the services index a file. Callers pass the resulting decisions in their
//! requests, and the lifecycle records them.
//!
//! ## States
//!
//! Each document is a state machine: see [state]. A request applies a
//! transition atomically to the state of its path, without locks, and updates
//! the database to match. Publications and changes update the database first,
//! so a close that finds the new state also finds the new views in the
//! database. Closes and the removals of disk views update the state first.
//!
//! ## Visibility
//!
//! Clients read the views of most documents. The views of a database-only
//! document are in the database only, for the services: the scanner indexes
//! the files of dependencies, for example, without holding their content for
//! the clients.
//!
//! ## Disk owner
//!
//! The scanner owns the disk view of the files that it keeps up to date: when
//! no client has the file open anymore, the disk view is the content of the
//! file again. The clients own the disk view of a file that one of them read
//! for itself, like the CLI reading the files it checks: the last close
//! removes it.
//!
//! ## Closed clients
//!
//! Requests that a client sent before its files were closed, such as when it
//! disconnected, can still be running. The editor views that they publish
//! afterward aren't kept. Two operations race:
//!
//! 1. A request of the client publishes a view, then checks whether the
//!    client is closed.
//! 2. The close records the client as closed, then looks for its views and
//!    removes them.
//!
//! Each operation writes, then reads what the other one writes. Both issue a
//! sequentially consistent fence between the write and the read, so at least
//! one of them observes the other: either the request finds its client closed
//! and removes the view itself, or the close finds the view and removes it.

pub(crate) mod services;
mod state;

use crate::db::{DbState, ViewId, WorkspaceDb};
use crate::diagnostics::FileTooLarge;
use crate::projects::{ProjectDb, ProjectKey};
use crate::workspace::retry_on_pending_write;
use crate::workspace::views::{ClientId, new_views_map};
use biome_configuration::Configuration;
use biome_db::Source;
use biome_fs::BiomePath;
use biome_languages::{DocumentFileSource, LanguageDb};
use camino::{Utf8Path, Utf8PathBuf};
use papaya::{Compute, HashMap, HashSet, Operation};
use rustc_hash::FxBuildHasher;
use state::{DocumentState, DocumentView, Step, Transition};
use std::collections::BTreeSet;
use std::sync::Arc;
#[cfg(test)]
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::atomic::{Ordering, fence};

/// Holds the state of every document of the workspace, and the clients whose
/// files were closed. See the [module documentation](self).
pub(crate) struct DocumentLifecycle {
    documents: HashMap<Utf8PathBuf, DocumentState, FxBuildHasher>,
    closed_clients: HashSet<ClientId, FxBuildHasher>,
    #[cfg(test)]
    checkpoints: Checkpoints,
}

/// Controls that tests use to interrupt lifecycle transitions at fixed points.
#[cfg(test)]
#[derive(Default)]
pub(crate) struct Checkpoints {
    /// Makes the next change close the files of its client after it reads the
    /// document and before it publishes the change.
    pub(crate) close_client_files_before_change_publish: AtomicBool,
    /// Pauses a close after it updates the metadata, before it updates the
    /// database.
    pub(crate) pause_close_after_document_update: AtomicBool,
    /// Counts the closes that reached the pause.
    pub(crate) close_document_updates: AtomicUsize,
}

/// The documents that one client reads and changes.
pub(crate) struct ClientDocuments<'a> {
    lifecycle: &'a DocumentLifecycle,
    db_state: &'a DbState,
    client: ClientId,
}

/// The documents as the scanner, the watcher and the project lifecycle see
/// them: their disk views, which all the clients share.
pub(crate) struct SharedDocuments<'a> {
    lifecycle: &'a DocumentLifecycle,
    db_state: &'a DbState,
}

/// The content of a view, and how queries read it.
pub(crate) struct DocumentContent {
    pub(crate) text: Arc<str>,
    pub(crate) file_source: DocumentFileSource,
    /// Request-scoped settings applied on top of the project configuration.
    pub(crate) inline_config: Option<Configuration>,
    pub(crate) syntax: DocumentSyntax,
    /// Whether the parses of the path share a node cache, which makes
    /// parsing a changed text faster. The cache lives until no client has
    /// the file open.
    pub(crate) persist_node_cache: bool,
}

/// Whether queries parse the text of a view.
pub(crate) enum DocumentSyntax {
    /// Queries parse the text, the first time one reads it. The caller checked
    /// that the file source can be parsed.
    Parsed,
    /// The text isn't parsed, because its file source has no parser.
    Unparsed,
    /// The text isn't parsed, because it exceeds the maximum file size.
    TooLarge(FileTooLarge),
}

/// Whether clients read a view. See the [module documentation](self).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Visibility {
    Visible,
    DatabaseOnly,
}

/// Who keeps the disk view of a file, which decides whether it outlives the
/// last close. See the [module documentation](self).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DiskOwner {
    /// The scanner keeps the disk view up to date, so it outlives the last
    /// close.
    Scanner,
    /// Clients read the file for themselves, so the last close removes the
    /// disk view.
    Clients,
}

impl DiskOwner {
    /// Returns the owner of a disk view after `reader` reads the file from
    /// disk again.
    ///
    /// When the scanner reads the file, it owns the disk view from then on,
    /// because it keeps the content up to date for the services and for the
    /// clients that read it after the last close. A client that reads the file
    /// for itself doesn't take the disk view away from the scanner, because
    /// that need remains. Only [SharedDocuments::release_disk] gives the disk
    /// view back to the clients.
    fn after_read(self, reader: Self) -> Self {
        match (self, reader) {
            (Self::Scanner, _) | (_, Self::Scanner) => Self::Scanner,
            (Self::Clients, Self::Clients) => Self::Clients,
        }
    }
}

/// The editor view that a client opens.
pub(crate) struct OpenDocument<'a> {
    pub(crate) path: &'a BiomePath,
    /// The project whose settings parse a virtual document. Ignored for the
    /// documents that have a path on disk, because their path selects the
    /// project.
    pub(crate) project_key: ProjectKey,
    pub(crate) version: i32,
    pub(crate) content: DocumentContent,
    pub(crate) visibility: Visibility,
}

/// The disk view that the scanner, the watcher or a client reads from the
/// filesystem.
pub(crate) struct PublishDiskDocument<'a> {
    pub(crate) path: &'a BiomePath,
    pub(crate) content: DocumentContent,
    pub(crate) visibility: Visibility,
    pub(crate) owner: DiskOwner,
}

/// A new version of the content of a document, sent by a client.
pub(crate) struct ChangeDocument<'a> {
    pub(crate) path: &'a BiomePath,
    /// The project whose settings parse a virtual document. See
    /// [OpenDocument::project_key].
    pub(crate) project_key: ProjectKey,
    pub(crate) text: String,
    pub(crate) version: i32,
    pub(crate) inline_config: Option<Configuration>,
}

/// The result of publishing a view.
#[derive(Debug)]
pub(crate) enum PublishOutcome {
    /// The view is published. `source` is the view that the services shared
    /// by the clients read, or `None` when queries don't parse the text.
    Applied { source: Option<Source> },
    /// The view of a scan deferred the publication of the text to the commit
    /// of its phase, because updating the inputs of the path needs Salsa
    /// setters. The metadata already holds the view.
    Deferred,
    /// The files of the client were closed after the editor view was
    /// published, so the view was removed again.
    ClientClosed,
}

/// The result of changing a document.
#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ChangeOutcome {
    /// The change is published. `source` is the view that the services
    /// shared by the clients read.
    Applied { source: Source },
    /// The client already published this version with this content, such as
    /// when a change is sent again after it was interrupted. `source` is the
    /// view that the client reads.
    Current { source: Source },
    /// The client published a later version, or this version with another
    /// content, so the change is ignored.
    Outdated,
    /// The files of the client were closed before the change, so nothing was
    /// published.
    ClientClosed,
    /// The change published the editor view of the client, then removed it
    /// again: the files of the client were closed meanwhile, or the document
    /// was, when `document_closed` is set.
    Withdrawn { document_closed: bool },
}

/// What remains of a document after a client closes it.
#[derive(Debug, Eq, PartialEq)]
pub(crate) enum CloseOutcome {
    /// Other clients still have the document open.
    StillOpen,
    /// No client has the document open, and the disk view that the scanner
    /// owns remains.
    Indexed,
    /// No view of the document remains, or the client had none.
    Removed,
}

/// Why a document can't be read or changed.
#[derive(Debug, Eq, PartialEq)]
pub(crate) enum DocumentError {
    /// The client reads no view of the document, or its view isn't parsed.
    NotFound,
    /// The content of the view exceeds the maximum file size, so it isn't
    /// parsed.
    TooLarge,
    /// The project of the request doesn't exist.
    NoProject,
    /// The file source of the document has no parser.
    Unsupported,
}

/// A copy of the metadata of the view that a client reads.
#[derive(Clone, Debug)]
pub(crate) struct DocumentSnapshot {
    /// The content of the view. The database shares the same allocation.
    pub(crate) content: Arc<str>,
    /// Request-scoped settings applied on top of the project configuration.
    pub(crate) inline_config: Option<Arc<Configuration>>,
}

/// A view to publish. Opens and disk publications both publish through it.
struct Publication<'a> {
    path: &'a BiomePath,
    view: ViewId,
    version: Option<i32>,
    content: DocumentContent,
    visibility: Visibility,
    /// The owner of a disk view. `None` for an editor view.
    owner: Option<DiskOwner>,
    /// The project to associate the parsed source of a virtual document with.
    project_key: Option<ProjectKey>,
}

impl DocumentLifecycle {
    pub(crate) fn new() -> Self {
        Self {
            documents: new_views_map(FxBuildHasher),
            closed_clients: HashSet::with_hasher(FxBuildHasher),
            #[cfg(test)]
            checkpoints: Checkpoints::default(),
        }
    }

    /// Returns the documents that `client` reads and changes, in `db_state`.
    pub(crate) fn for_client<'a>(
        &'a self,
        db_state: &'a DbState,
        client: ClientId,
    ) -> ClientDocuments<'a> {
        ClientDocuments {
            lifecycle: self,
            db_state,
            client,
        }
    }

    /// Returns the disk views of the documents, in `db_state`.
    pub(crate) fn shared<'a>(&'a self, db_state: &'a DbState) -> SharedDocuments<'a> {
        SharedDocuments {
            lifecycle: self,
            db_state,
        }
    }

    /// Forgets that the files of `client` were closed, so the views that a
    /// client with the same identifier opens are kept again. See
    /// [ClientDocuments::close_all].
    pub(crate) fn forget_client(&self, client: ClientId) {
        self.closed_clients.pin().remove(&client);
    }

    /// Returns the number of documents that clients read.
    pub(crate) fn len(&self) -> usize {
        self.documents
            .pin()
            .values()
            .filter(|state| state.views().is_some())
            .count()
    }

    #[cfg(test)]
    pub(crate) fn checkpoints(&self) -> &Checkpoints {
        &self.checkpoints
    }

    /// Records that the files of `client` are closed, before the caller looks
    /// for its views. See the [module documentation](self).
    fn mark_closed(&self, client: ClientId) {
        self.closed_clients.pin().insert(client);
        fence(Ordering::SeqCst);
    }

    /// Returns whether the files of `client` are closed, after the caller
    /// published a view of the client. See the [module documentation](self).
    fn is_closed(&self, client: ClientId) -> bool {
        fence(Ordering::SeqCst);
        self.closed_clients.pin().contains(&client)
    }

    /// Applies `transition` to the state of `path`.
    ///
    /// Returns `None` when the transition doesn't apply to the state, and
    /// otherwise what `observe` returns for the states before and after the
    /// transition. `None` stands for a path without views.
    fn apply<R>(
        &self,
        path: &Utf8Path,
        transition: &Transition,
        observe: impl FnOnce(Option<&DocumentState>, Option<&DocumentState>) -> R,
    ) -> Option<R> {
        let documents = self.documents.pin();
        let result = documents.compute(path.to_path_buf(), |entry| {
            match DocumentState::step(entry.map(|(_, state)| state), transition) {
                Step::To(state) => Operation::Insert(state),
                Step::Remove => Operation::Remove,
                Step::Stay => Operation::Abort(()),
            }
        });
        match result {
            Compute::Inserted(_, to) => Some(observe(None, Some(to))),
            Compute::Updated {
                old: (_, from),
                new: (_, to),
            } => Some(observe(Some(from), Some(to))),
            Compute::Removed(_, from) => Some(observe(Some(from), None)),
            Compute::Aborted(()) => None,
        }
    }

    /// Publishes a view of `client` in the database, then in the metadata.
    fn publish(
        &self,
        db_state: &DbState,
        client: ClientId,
        publication: Publication,
    ) -> PublishOutcome {
        let Publication {
            path,
            view,
            version,
            content,
            visibility,
            owner,
            project_key,
        } = publication;
        let DocumentContent {
            text,
            file_source,
            inline_config,
            syntax,
            persist_node_cache,
        } = content;
        let file_path = path.as_path();
        let is_parsed = matches!(syntax, DocumentSyntax::Parsed);
        if is_parsed && persist_node_cache {
            db_state.retain_node_cache(file_path);
        }

        let file_source_index = db_state.insert_file_source(file_source);
        let source = if is_parsed {
            // The project layout and module resolution depend on the
            // manifests and on the JSON files that the resolver reads, such as
            // extended TypeScript configurations. The files that a scan reads
            // later need them, so a scan publishes them right away instead of
            // deferring them, except in its setup phase.
            let can_defer = db_state.defers_manifests()
                || (!path.is_manifest() && !matches!(file_source, DocumentFileSource::Json(_)));
            if can_defer {
                db_state.update_file_view_or_defer(
                    file_path,
                    view,
                    text.clone(),
                    file_source_index,
                    inline_config.clone(),
                )
            } else {
                Some(db_state.update_file_view(
                    file_path,
                    view,
                    text.clone(),
                    file_source_index,
                    inline_config.clone(),
                ))
            }
        } else {
            None
        };
        if let Some(project_key) = project_key
            && path.is_virtual()
            && source.is_some()
        {
            Self::associate_client_source(db_state, client, file_path, project_key);
        }

        let metadata = (visibility == Visibility::Visible).then(|| DocumentView {
            content: text,
            version,
            file_source_index,
            inline_config: inline_config.map(Arc::new),
            syntax: match syntax {
                DocumentSyntax::Parsed => Some(Ok(())),
                DocumentSyntax::Unparsed => None,
                DocumentSyntax::TooLarge(error) => Some(Err(error)),
            },
        });
        self.apply(
            file_path,
            &Transition::Publish {
                view,
                metadata,
                owner,
            },
            |_, _| (),
        );

        if view == ViewId::Editor(client) && self.is_closed(client) {
            self.close(db_state, client, file_path);
            return PublishOutcome::ClientClosed;
        }

        match source {
            Some(source) => PublishOutcome::Applied {
                source: Some(source),
            },
            None if is_parsed => PublishOutcome::Deferred,
            None => PublishOutcome::Applied { source: None },
        }
    }

    /// Removes the editor view of `client` from the metadata, then from the
    /// database.
    fn close(&self, db_state: &DbState, client: ClientId, path: &Utf8Path) -> CloseOutcome {
        let (keeps_disk, outcome) = self
            .apply(path, &Transition::Close(client), |from, to| {
                let outcome = match to {
                    Some(state) if state.has_editors() => CloseOutcome::StillOpen,
                    Some(_) => CloseOutcome::Indexed,
                    None => CloseOutcome::Removed,
                };
                (from.is_some_and(DocumentState::keeps_disk), outcome)
            })
            .unwrap_or((false, CloseOutcome::Removed));

        #[cfg(test)]
        {
            self.checkpoints
                .close_document_updates
                .fetch_add(1, Ordering::AcqRel);
            while self
                .checkpoints
                .pause_close_after_document_update
                .load(Ordering::Acquire)
            {
                std::thread::yield_now();
            }
        }

        // Removing only the view of this client preserves the views that
        // concurrent opens publish.
        db_state.remove_editor_view(path, client, keeps_disk);
        if outcome != CloseOutcome::StillOpen {
            db_state.remove_node_cache(path);
        }
        outcome
    }

    /// Associates the parsed source that `client` reads of the virtual
    /// document at `path` with `project_key`, whose settings then parse it.
    fn associate_client_source(
        db_state: &DbState,
        client: ClientId,
        path: &Utf8Path,
        project_key: ProjectKey,
    ) {
        let source = db_state.fork().get_client_parsed_source(path, client);
        if let Some(source) = source {
            db_state.associate_source_with_project(path, client, source, project_key);
        }
    }
}

impl ClientDocuments<'_> {
    /// Publishes the editor view of this client.
    ///
    /// The view becomes the content that the client reads, and the content
    /// that the services read when the client is the first one to open the
    /// file.
    pub(crate) fn open(&self, request: OpenDocument) -> PublishOutcome {
        let OpenDocument {
            path,
            project_key,
            version,
            content,
            visibility,
        } = request;
        self.lifecycle.publish(
            self.db_state,
            self.client,
            Publication {
                path,
                view: ViewId::Editor(self.client),
                version: Some(version),
                content,
                visibility,
                owner: None,
                project_key: Some(project_key),
            },
        )
    }

    /// Publishes a new version of the content that this client reads.
    ///
    /// When the client has the file open, the change applies to its editor
    /// view. Otherwise, it records the content on disk, as when the CLI writes
    /// the fixes of a file. `can_parse` reports whether the file source of the
    /// document can be parsed, because queries parse the text later.
    ///
    /// A change of a virtual document to the version it already has is
    /// published again when it's for another project, so the document is
    /// parsed with the settings of that project.
    pub(crate) fn change(
        &self,
        request: ChangeDocument,
        can_parse: impl FnOnce(DocumentFileSource) -> bool,
    ) -> Result<ChangeOutcome, DocumentError> {
        let ChangeDocument {
            path,
            project_key,
            text,
            version,
            inline_config,
        } = request;
        let file_path = path.as_path();
        let (current_version, same_content, file_source_index, is_open) = {
            let documents = self.lifecycle.documents.pin();
            let views = documents
                .get(file_path)
                .and_then(DocumentState::views)
                .ok_or(DocumentError::NotFound)?;
            let view = views.for_client(self.client);
            (
                views.editor(self.client).and_then(|view| view.version),
                view.is_some_and(|view| *view.content == *text),
                view.unwrap_or_else(|| views.current()).file_source_index,
                views.editor(self.client).is_some(),
            )
        };

        // Once the views of a client are closed, a change it sent before must
        // not be recorded as the content on disk.
        if self.lifecycle.is_closed(self.client) {
            return Ok(ChangeOutcome::ClientClosed);
        }
        if current_version.is_some_and(|current| current > version)
            || current_version == Some(version) && !same_content
        {
            return Ok(ChangeOutcome::Outdated);
        }
        if current_version == Some(version) {
            let db = self.db_state.fork();
            let source = db
                .get_client_parsed_source(file_path, self.client)
                .ok_or(DocumentError::NotFound)?;
            if !path.is_virtual() || db.project_key_for_source(source) == Some(project_key) {
                return Ok(ChangeOutcome::Current { source });
            }
        }
        {
            let db = self.db_state.fork();
            if db.get_project(&project_key).is_none() {
                return Err(DocumentError::NoProject);
            }
            // The text is parsed the first time a query reads it, so a file
            // source that can't be parsed must be rejected here.
            let file_source = db
                .source_from_index(file_source_index)
                .ok_or(DocumentError::Unsupported)?;
            if !can_parse(file_source) {
                return Err(DocumentError::Unsupported);
            }
        }

        let view = if is_open {
            ViewId::Editor(self.client)
        } else {
            ViewId::Disk
        };
        let text = Arc::<str>::from(text);
        #[cfg(test)]
        if self
            .lifecycle
            .checkpoints
            .close_client_files_before_change_publish
            .swap(false, Ordering::AcqRel)
        {
            self.close_all();
        }
        let source = self.db_state.update_file_view(
            file_path,
            view,
            text.clone(),
            file_source_index,
            inline_config.clone(),
        );
        if path.is_virtual() {
            DocumentLifecycle::associate_client_source(
                self.db_state,
                self.client,
                file_path,
                project_key,
            );
        }

        let transition = Transition::Change {
            view,
            metadata: DocumentView {
                content: text,
                version: is_open.then_some(version),
                file_source_index,
                inline_config: inline_config.map(Arc::new),
                syntax: Some(Ok(())),
            },
            owner: (view == ViewId::Disk).then_some(DiskOwner::Clients),
        };
        let changed = self
            .lifecycle
            .apply(file_path, &transition, |_, _| ())
            .is_some();
        // The document was closed after it was read, such as when the client
        // disconnected meanwhile, so the editor view just published must not
        // outlive it.
        if !changed {
            if view == ViewId::Editor(self.client) {
                self.lifecycle.close(self.db_state, self.client, file_path);
                return Ok(ChangeOutcome::Withdrawn {
                    document_closed: true,
                });
            }
            return Err(DocumentError::NotFound);
        }
        if view == ViewId::Editor(self.client) && self.lifecycle.is_closed(self.client) {
            self.lifecycle.close(self.db_state, self.client, file_path);
            return Ok(ChangeOutcome::Withdrawn {
                document_closed: false,
            });
        }
        Ok(ChangeOutcome::Applied { source })
    }

    /// Returns the metadata of the view that this client reads: its editor
    /// view, or the disk view when it didn't open the file.
    pub(crate) fn read(&self, path: &Utf8Path) -> Result<DocumentSnapshot, DocumentError> {
        let documents = self.lifecycle.documents.pin();
        let view = documents
            .get(path)
            .and_then(|state| state.for_client(self.client))
            .ok_or(DocumentError::NotFound)?;
        Ok(DocumentSnapshot {
            content: view.content.clone(),
            inline_config: view.inline_config.clone(),
        })
    }

    /// Returns the parsed source of the view that this client reads, from
    /// `db`.
    ///
    /// Fails when the client reads no view of the document, or when its view
    /// isn't parsed.
    pub(crate) fn source(
        &self,
        db: &WorkspaceDb,
        path: &Utf8Path,
    ) -> Result<Source, DocumentError> {
        self.check_parsed(path)?;
        db.get_client_parsed_source(path, self.client)
            .ok_or(DocumentError::NotFound)
    }

    /// Returns the parsed source of the view that the services shared by the
    /// clients read, from `db`: the editor view of the client that opened the
    /// file first, or the disk view.
    ///
    /// Fails like [Self::source], for the view that this client reads.
    pub(crate) fn shared_source(
        &self,
        db: &WorkspaceDb,
        path: &Utf8Path,
    ) -> Result<Source, DocumentError> {
        self.check_parsed(path)?;
        db.get_parsed_source(path).ok_or(DocumentError::NotFound)
    }

    /// Returns the parsed source of the view that this client reads in `db`,
    /// including the views of database-only documents, which clients don't
    /// read otherwise.
    pub(crate) fn database_source(&self, db: &WorkspaceDb, path: &Utf8Path) -> Option<Source> {
        db.get_client_parsed_source(path, self.client)
    }

    /// Associates the parsed source that this client reads of the virtual
    /// document at `path` with `project_key`, so it's parsed with the
    /// settings of that project. Does nothing for other documents.
    ///
    /// Call it before taking the database read that reads the source, because
    /// associating the source with another project updates the database.
    pub(crate) fn bind_project(
        &self,
        path: &BiomePath,
        project_key: ProjectKey,
    ) -> Result<(), DocumentError> {
        if !path.is_virtual() {
            return Ok(());
        }
        let source = {
            let db = self.db_state.fork();
            if db.get_project(&project_key).is_none() {
                return Err(DocumentError::NoProject);
            }
            db.get_client_parsed_source(path.as_path(), self.client)
                .ok_or(DocumentError::NotFound)?
        };
        self.db_state.associate_source_with_project(
            path.as_path(),
            self.client,
            source,
            project_key,
        );
        Ok(())
    }

    /// Removes the editor view of this client.
    ///
    /// Other clients that have the file open keep their views. When no client
    /// has the file open anymore, a disk view that the scanner owns becomes the
    /// content of the document again, and one that the clients own is removed.
    pub(crate) fn close(&self, path: &Utf8Path) -> CloseOutcome {
        self.lifecycle.close(self.db_state, self.client, path)
    }

    /// Closes the files that this client has open, as when it disconnects,
    /// including the editor views of database-only documents. Returns the
    /// paths of the closed files.
    ///
    /// The editor views that the client publishes afterward aren't kept,
    /// until [DocumentLifecycle::forget_client] forgets the client.
    pub(crate) fn close_all(&self) -> Vec<Utf8PathBuf> {
        self.lifecycle.mark_closed(self.client);
        let mut paths: BTreeSet<_> =
            retry_on_pending_write(|| self.db_state.fork().paths_open_in(self.client))
                .into_iter()
                .collect();
        paths.extend(
            self.lifecycle
                .documents
                .pin()
                .iter()
                .filter(|(_, state)| {
                    state
                        .views()
                        .is_some_and(|views| views.editor(self.client).is_some())
                })
                .map(|(path, _)| path.clone()),
        );
        for path in &paths {
            self.lifecycle.close(self.db_state, self.client, path);
        }
        paths.into_iter().collect()
    }

    /// Fails like [Self::source] when the metadata of the view that this
    /// client reads says that the view isn't parsed.
    fn check_parsed(&self, path: &Utf8Path) -> Result<(), DocumentError> {
        let syntax = self
            .lifecycle
            .documents
            .pin()
            .get(path)
            .and_then(|state| state.for_client(self.client))
            .and_then(|view| view.syntax.clone());
        match syntax {
            None => Err(DocumentError::NotFound),
            Some(Ok(())) => Ok(()),
            Some(Err(FileTooLarge { .. })) => Err(DocumentError::TooLarge),
        }
    }
}

impl SharedDocuments<'_> {
    /// Publishes the disk view of a document.
    ///
    /// The clients that didn't open the file read the disk view. Once the
    /// scanner reads the file, it owns the disk view, even when a client reads
    /// the file again: see [DiskOwner::after_read].
    pub(crate) fn publish_disk(&self, request: PublishDiskDocument) -> PublishOutcome {
        let PublishDiskDocument {
            path,
            content,
            visibility,
            owner,
        } = request;
        self.lifecycle.publish(
            self.db_state,
            ClientId::default(),
            Publication {
                path,
                view: ViewId::Disk,
                version: None,
                content,
                visibility,
                owner: Some(owner),
                project_key: None,
            },
        )
    }

    /// Removes the disk view of `path` from the metadata and from the
    /// database. The editor views of the clients remain.
    #[cfg(feature = "module_graph")]
    pub(crate) fn remove_disk(&self, path: &Utf8Path) {
        self.lifecycle
            .apply(path, &Transition::RemoveDisk, |_, _| ());
        self.db_state.remove_file_view(path, ViewId::Disk);
    }

    /// Gives the disk view of `path` back to the clients, because the scanner
    /// stops keeping it up to date, such as when the file is removed. The
    /// disk view remains until no client has the file open.
    pub(crate) fn release_disk(&self, path: &Utf8Path) {
        self.lifecycle
            .apply(path, &Transition::ReleaseDisk, |_, _| ());
    }

    /// Removes the disk views of the files within `path` from the database,
    /// such as when the folder is removed. The editor views remain.
    ///
    /// The metadata keeps the disk views, which the clients that didn't open
    /// these files still read, until a close removes them like the disk views
    /// that the clients own.
    pub(crate) fn unload_subtree(&self, path: &Utf8Path) {
        self.db_state.remove_disk_views_within(path);
        let paths: Vec<_> = self
            .lifecycle
            .documents
            .pin()
            .iter()
            .filter(|(file_path, state)| file_path.starts_with(path) && state.disk().is_some())
            .map(|(file_path, _)| file_path.clone())
            .collect();
        for file_path in paths {
            self.lifecycle
                .apply(&file_path, &Transition::DropDatabaseDisk, |_, _| ());
        }
    }

    /// Unloads the documents of the project at `project_path`, with their node
    /// caches, and removes the files within `project_path` from the database.
    ///
    /// `owns` selects the documents that the project owns. The others keep
    /// their metadata, such as the virtual documents and the documents of a
    /// nested project.
    pub(crate) fn unload_project(&self, project_path: &Utf8Path, owns: impl Fn(&Utf8Path) -> bool) {
        let paths: Vec<_> = self
            .lifecycle
            .documents
            .pin()
            .keys()
            .filter(|path| owns(path))
            .cloned()
            .collect();
        for path in paths {
            self.lifecycle.apply(&path, &Transition::Unload, |_, _| ());
            self.db_state.remove_node_cache(&path);
        }
        self.db_state.unload_files(project_path);
    }

    /// Returns whether a client reads a view of `path`.
    pub(crate) fn contains(&self, path: &Utf8Path) -> bool {
        self.lifecycle
            .documents
            .pin()
            .get(path)
            .is_some_and(|state| state.views().is_some())
    }
}

#[cfg(all(test, feature = "lang_js"))]
#[path = "lifecycle.tests.rs"]
mod tests;
