//! The views of the content of a file.
//!
//! The workspace reads a file from the filesystem, and its clients, such as
//! editors through the LSP or the CLI attached to a daemon, open it with their
//! own content. Each source of content is a view of the file: the disk view
//! holds the content read from the filesystem, and every client that opens the
//! file has its own editor view.
//!
//! A client reads its own editor view, or the disk view when it didn't open
//! the file. The services that the clients share, such as the module graph,
//! read the editor view of the client that opened the file first, which is the
//! primary editor view, or the disk view when no client has the file open.

/// Identifies a client of the workspace, such as one connection to the daemon.
///
/// The default client is the one of the in-process uses of the workspace, such
/// as the CLI without a daemon, or the scanner and the watcher of the daemon.
/// It is distinct from every client that [Self::new] creates.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ClientId(Option<u64>);

impl ClientId {
    /// Creates the identifier of the client `id`, such as the number of a
    /// connection to the daemon.
    pub fn new(id: u64) -> Self {
        Self(Some(id))
    }
}

/// Selects one view of a file.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FileView {
    Disk,
    /// The editor view of a client.
    Editor(ClientId),
}

/// The views of a file: its disk view of type `D`, and the editor views, of
/// type `E`, of the clients that have it open. A file has at least one view.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Views<D, E> {
    /// No client has the file open.
    Disk(D),
    /// At least one client has the file open. `disk` is set when the scanner
    /// or the watcher read the file too.
    Editor {
        editors: Editors<E>,
        disk: Option<D>,
    },
}

/// One view of a file.
#[derive(Clone, Copy, Debug)]
pub(crate) enum View<'a, D, E> {
    Disk(&'a D),
    Editor(&'a E),
}

/// The editor views of the clients that have a file open, in the order they
/// opened it. There is at least one.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Editors<E> {
    /// The view of the client that opened the file first.
    primary: (ClientId, E),
    others: Vec<(ClientId, E)>,
}

impl<E> Editors<E> {
    fn new(client: ClientId, editor: E) -> Self {
        Self {
            primary: (client, editor),
            others: Vec::new(),
        }
    }

    fn get(&self, client: ClientId) -> Option<&E> {
        std::iter::once(&self.primary)
            .chain(&self.others)
            .find(|(owner, _)| *owner == client)
            .map(|(_, editor)| editor)
    }

    fn with(mut self, client: ClientId, editor: E) -> Self {
        if self.primary.0 == client {
            self.primary.1 = editor;
        } else if let Some(entry) = self.others.iter_mut().find(|(owner, _)| *owner == client) {
            entry.1 = editor;
        } else {
            self.others.push((client, editor));
        }
        self
    }

    /// Returns the editor views without the one of `client`, or `None` when it
    /// was the only one. When `client` opened the file first, the client that
    /// opened it next becomes the primary one.
    fn without(self, client: ClientId) -> Option<Self> {
        if self.primary.0 != client {
            let others = self
                .others
                .into_iter()
                .filter(|(owner, _)| *owner != client)
                .collect();
            return Some(Self {
                primary: self.primary,
                others,
            });
        }
        let mut others = self.others.into_iter();
        let primary = others.next()?;
        Some(Self {
            primary,
            others: others.collect(),
        })
    }
}

impl<D, E> Views<D, E> {
    pub(crate) fn from_disk(disk: D) -> Self {
        Self::Disk(disk)
    }

    pub(crate) fn from_editor(client: ClientId, editor: E) -> Self {
        Self::Editor {
            editors: Editors::new(client, editor),
            disk: None,
        }
    }

    pub(crate) fn disk(&self) -> Option<&D> {
        match self {
            Self::Disk(disk) => Some(disk),
            Self::Editor { disk, .. } => disk.as_ref(),
        }
    }

    pub(crate) fn editor(&self, client: ClientId) -> Option<&E> {
        match self {
            Self::Disk(_) => None,
            Self::Editor { editors, .. } => editors.get(client),
        }
    }

    /// Returns whether a client has the file open.
    pub(crate) fn has_editors(&self) -> bool {
        matches!(self, Self::Editor { .. })
    }

    /// Returns the primary editor view if a client has the file open,
    /// otherwise the disk view.
    pub(crate) fn current(&self) -> View<'_, D, E> {
        match self {
            Self::Disk(disk) => View::Disk(disk),
            Self::Editor { editors, .. } => View::Editor(&editors.primary.1),
        }
    }

    /// Returns the editor view of `client` if it has the file open, otherwise
    /// the disk view. Returns `None` when only other clients have the file
    /// open.
    pub(crate) fn for_client(&self, client: ClientId) -> Option<View<'_, D, E>> {
        self.editor(client)
            .map(View::Editor)
            .or_else(|| self.disk().map(View::Disk))
    }

    pub(crate) fn with_disk(self, disk: D) -> Self {
        match self {
            Self::Disk(_) => Self::Disk(disk),
            Self::Editor { editors, .. } => Self::Editor {
                editors,
                disk: Some(disk),
            },
        }
    }

    pub(crate) fn with_editor(self, client: ClientId, editor: E) -> Self {
        match self {
            Self::Disk(disk) => Self::Editor {
                editors: Editors::new(client, editor),
                disk: Some(disk),
            },
            Self::Editor { editors, disk } => Self::Editor {
                editors: editors.with(client, editor),
                disk,
            },
        }
    }

    /// Returns the views without the disk view, or `None` when no client has
    /// the file open.
    pub(crate) fn without_disk(self) -> Option<Self> {
        match self {
            Self::Disk(_) => None,
            Self::Editor { editors, .. } => Some(Self::Editor {
                editors,
                disk: None,
            }),
        }
    }

    /// Returns the views without the editor view of `client`, or `None` when
    /// no view is left.
    pub(crate) fn without_editor(self, client: ClientId) -> Option<Self> {
        match self {
            Self::Disk(disk) => Some(Self::Disk(disk)),
            Self::Editor { editors, disk } => match editors.without(client) {
                Some(editors) => Some(Self::Editor { editors, disk }),
                None => disk.map(Self::Disk),
            },
        }
    }
}

impl<T> Views<T, T> {
    pub(crate) fn from_view(view: FileView, value: T) -> Self {
        match view {
            FileView::Disk => Self::from_disk(value),
            FileView::Editor(client) => Self::from_editor(client, value),
        }
    }

    pub(crate) fn get(&self, view: FileView) -> Option<&T> {
        match view {
            FileView::Disk => self.disk(),
            FileView::Editor(client) => self.editor(client),
        }
    }

    pub(crate) fn with(self, view: FileView, value: T) -> Self {
        match view {
            FileView::Disk => self.with_disk(value),
            FileView::Editor(client) => self.with_editor(client, value),
        }
    }

    /// Returns the views without `view`, or `None` when no view is left.
    pub(crate) fn without(self, view: FileView) -> Option<Self> {
        match view {
            FileView::Disk => self.without_disk(),
            FileView::Editor(client) => self.without_editor(client),
        }
    }
}

impl<'a, T> View<'a, T, T> {
    pub(crate) fn value(self) -> &'a T {
        match self {
            Self::Disk(value) | Self::Editor(value) => value,
        }
    }
}

#[cfg(test)]
#[path = "views.tests.rs"]
mod tests;
