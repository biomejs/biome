//! The state machine of a document.
//!
//! A document is in one of the [DocumentState]s while it has a view, and a
//! [Transition] moves it to another state, or removes it. Transitions are
//! pure: [DocumentState::step] computes the next state from the current one,
//! so the lifecycle can apply it atomically to the record of the path, even
//! when a concurrent update makes it compute the step again. The lifecycle
//! updates the database to match, before or after the step depending on the
//! transition: see the [lifecycle documentation](super).

use super::DiskOwner;
use crate::db::ViewId;
use crate::diagnostics::FileTooLarge;
use crate::workspace::views::{ClientId, Views};
use biome_configuration::Configuration;
use std::sync::Arc;

/// The state of a document that has a view. A path without views has no
/// state.
#[derive(Clone, Debug)]
pub(super) enum DocumentState {
    /// Only the services read the document, from the disk view that the
    /// database holds. Clients read no view.
    DatabaseOnly { owner: DiskOwner },
    /// Clients read `views`. `disk` is the owner of the disk view that the
    /// database holds, or `None` when the database holds none, such as after
    /// the folder of the file was removed.
    Visible {
        views: Views<DocumentView>,
        disk: Option<DiskOwner>,
    },
}

/// The metadata of one view of a document, which clients read without a
/// database.
#[derive(Clone, Debug)]
pub(super) struct DocumentView {
    /// The database shares the same allocation as the text of the file.
    pub(super) content: Arc<str>,
    /// The version of the content, as specified by the client of an editor
    /// view. The disk view has none.
    pub(super) version: Option<i32>,
    /// The index of the file source the content is parsed as, in the table of
    /// file sources of the database.
    pub(super) file_source_index: usize,
    /// Request-scoped settings applied on top of the project configuration.
    pub(super) inline_config: Option<Arc<Configuration>>,
    /// `None` when the content isn't parsed, and an error when it's too large
    /// to be parsed.
    pub(super) syntax: Option<Result<(), FileTooLarge>>,
}

/// An event that moves a document from one state to another.
pub(super) enum Transition {
    /// A view is published. `metadata` is `None` for a view that only the
    /// services read, and `owner` is `None` for an editor view.
    Publish {
        view: ViewId,
        metadata: Option<DocumentView>,
        owner: Option<DiskOwner>,
    },
    /// A client changes the content that it reads. Only the documents that
    /// clients read can change.
    Change {
        view: ViewId,
        metadata: DocumentView,
        owner: Option<DiskOwner>,
    },
    /// A client closes its editor view. A disk view that the clients own is
    /// removed with the last editor view.
    Close(ClientId),
    /// The disk view is removed, from the metadata and from the database.
    RemoveDisk,
    /// The scanner stops keeping the disk view up to date, so the clients own
    /// it.
    ReleaseDisk,
    /// The database drops the disk view, and the metadata keeps it.
    DropDatabaseDisk,
    /// Every view of the document is removed.
    Unload,
}

/// What a transition does to the state of a document.
pub(super) enum Step {
    /// The document moves to the state.
    To(DocumentState),
    /// The document loses its last view.
    Remove,
    /// The transition doesn't apply to the state, which doesn't change.
    Stay,
}

impl DocumentState {
    /// Returns the step that `transition` takes from `current`, the state of
    /// the document, or `None` when it has no view.
    pub(super) fn step(current: Option<&Self>, transition: &Transition) -> Step {
        match transition {
            Transition::Publish {
                view,
                metadata,
                owner,
            } => Self::publish(current, *view, metadata.as_ref(), *owner),
            Transition::Change {
                view,
                metadata,
                owner,
            } => match current {
                Some(Self::Visible { .. }) => Self::publish(current, *view, Some(metadata), *owner),
                _ => Step::Stay,
            },
            Transition::Close(client) => current.map_or(Step::Stay, |state| state.close(*client)),
            Transition::RemoveDisk => match current {
                None => Step::Stay,
                Some(Self::DatabaseOnly { .. }) => Step::Remove,
                Some(Self::Visible { views, .. }) => match views.clone().without_disk() {
                    Some(views) => Step::To(Self::Visible { views, disk: None }),
                    None => Step::Remove,
                },
            },
            Transition::ReleaseDisk => match current {
                Some(Self::DatabaseOnly {
                    owner: DiskOwner::Scanner,
                }) => Step::To(Self::DatabaseOnly {
                    owner: DiskOwner::Clients,
                }),
                Some(Self::Visible {
                    views,
                    disk: Some(DiskOwner::Scanner),
                }) => Step::To(Self::Visible {
                    views: views.clone(),
                    disk: Some(DiskOwner::Clients),
                }),
                _ => Step::Stay,
            },
            Transition::DropDatabaseDisk => match current {
                Some(Self::DatabaseOnly { .. }) => Step::Remove,
                Some(Self::Visible {
                    views,
                    disk: Some(_),
                }) => Step::To(Self::Visible {
                    views: views.clone(),
                    disk: None,
                }),
                _ => Step::Stay,
            },
            Transition::Unload => current.map_or(Step::Stay, |_| Step::Remove),
        }
    }

    /// Returns the step that publishing `view` takes. Once the scanner reads a
    /// file, it owns the disk view, even when a client reads the file again:
    /// see [DiskOwner::after_read].
    fn publish(
        current: Option<&Self>,
        view: ViewId,
        metadata: Option<&DocumentView>,
        owner: Option<DiskOwner>,
    ) -> Step {
        let current_disk = current.and_then(Self::disk);
        let disk = match (view, current_disk, owner) {
            (ViewId::Disk, Some(current), Some(reader)) => Some(current.after_read(reader)),
            (ViewId::Disk, current, reader) => current.or(reader),
            (ViewId::Editor(_), current, _) => current,
        };
        match (current, metadata) {
            (Some(Self::Visible { views, .. }), Some(metadata)) => Step::To(Self::Visible {
                views: views.clone().with(view, metadata.clone()),
                disk,
            }),
            (_, Some(metadata)) => Step::To(Self::Visible {
                views: Views::new(view, metadata.clone()),
                disk,
            }),
            // The database holds the editor views that only the services
            // read, so they don't change the state.
            (_, None) if view != ViewId::Disk => Step::Stay,
            (Some(Self::Visible { views, .. }), None) => Step::To(Self::Visible {
                views: views.clone(),
                disk,
            }),
            (_, None) => disk.map_or(Step::Stay, |owner| Step::To(Self::DatabaseOnly { owner })),
        }
    }

    fn close(&self, client: ClientId) -> Step {
        let keeps_disk = self.keeps_disk();
        match self {
            Self::DatabaseOnly { .. } if keeps_disk => Step::To(self.clone()),
            Self::DatabaseOnly { .. } => Step::Remove,
            Self::Visible { views, disk } => {
                let views = views
                    .clone()
                    .without_editor(client)
                    .filter(|views| views.has_editors() || keeps_disk);
                match views {
                    Some(views) => {
                        let disk = disk.filter(|_| keeps_disk || views.has_editors());
                        Step::To(Self::Visible { views, disk })
                    }
                    // The disk view was in the database only.
                    None if keeps_disk => Step::To(Self::DatabaseOnly {
                        owner: DiskOwner::Scanner,
                    }),
                    None => Step::Remove,
                }
            }
        }
    }

    /// Returns the owner of the disk view that the database holds.
    pub(super) fn disk(&self) -> Option<DiskOwner> {
        match self {
            Self::DatabaseOnly { owner } => Some(*owner),
            Self::Visible { disk, .. } => *disk,
        }
    }

    /// Returns whether the disk view remains when no client has the file
    /// open.
    pub(super) fn keeps_disk(&self) -> bool {
        self.disk() == Some(DiskOwner::Scanner)
    }

    /// Returns the views that clients read.
    pub(super) fn views(&self) -> Option<&Views<DocumentView>> {
        match self {
            Self::DatabaseOnly { .. } => None,
            Self::Visible { views, .. } => Some(views),
        }
    }

    /// Returns whether a client has the file open.
    pub(super) fn has_editors(&self) -> bool {
        self.views().is_some_and(Views::has_editors)
    }

    /// Returns the metadata of the view that `client` reads.
    pub(super) fn for_client(&self, client: ClientId) -> Option<&DocumentView> {
        self.views()?.for_client(client)
    }
}
