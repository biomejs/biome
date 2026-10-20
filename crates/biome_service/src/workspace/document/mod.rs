pub(crate) mod services;

use crate::diagnostics::FileTooLarge;
use biome_languages::DocumentFileSource;
use std::sync::Arc;

/// The views of a file that the workspace knows about.
///
/// The disk view holds the content that the scanner or the watcher last read
/// from the filesystem. The editor view holds the content that a client opened
/// and changed, typically an editor through the LSP. When both exist, the
/// editor view shadows the disk view: a disk update leaves the editor's
/// content untouched, and closing the file falls back to the disk view.
#[derive(Clone, Debug)]
pub(crate) enum Document {
    /// Only the scanner or the watcher read the file.
    Disk(DiskView),
    /// A client opened the file. `disk` is set when the scanner or the watcher
    /// read it too.
    Editor {
        editor: EditorView,
        disk: Option<DiskView>,
    },
}

/// The content of a file as read from the filesystem.
#[derive(Clone, Debug)]
pub(crate) struct DiskView {
    /// Document content.
    ///
    /// The content of the file is only available if it belongs to the project. For example, we don't
    /// want to store the content of files coming from dependencies.
    ///
    /// The database shares the same allocation as the text of the file.
    pub(crate) content: Arc<str>,

    /// The file source the content is parsed as.
    pub(crate) file_source: DocumentFileSource,

    /// The result of the parser (syntax tree + diagnostics).
    /// Types explained:
    /// - `Option`: if the file can be read
    /// - `Result`: if the file is read, but the file is too large
    pub(crate) syntax: Option<Result<(), FileTooLarge>>,
}

/// The content of a file as a client opened and changed it.
#[derive(Clone, Debug)]
pub(crate) struct EditorView {
    /// Document content. See [DiskView::content].
    pub(crate) content: Arc<str>,

    /// The version of the content, as specified by the client.
    pub(crate) version: i32,

    /// The file source the content is parsed as.
    pub(crate) file_source: DocumentFileSource,

    /// The result of the parser. See [DiskView::syntax].
    pub(crate) syntax: Option<Result<(), FileTooLarge>>,
}

impl From<DiskView> for Document {
    fn from(disk: DiskView) -> Self {
        Self::Disk(disk)
    }
}

impl From<EditorView> for Document {
    fn from(editor: EditorView) -> Self {
        Self::Editor { editor, disk: None }
    }
}

impl Document {
    pub(crate) fn editor(&self) -> Option<&EditorView> {
        match self {
            Self::Disk(_) => None,
            Self::Editor { editor, .. } => Some(editor),
        }
    }

    /// Returns the content of the editor view if there is one, otherwise the
    /// content of the disk view.
    pub(crate) fn content(&self) -> &Arc<str> {
        match self {
            Self::Disk(disk) => &disk.content,
            Self::Editor { editor, .. } => &editor.content,
        }
    }

    /// Returns the file source of the view that [Self::content] reads.
    pub(crate) fn file_source(&self) -> DocumentFileSource {
        match self {
            Self::Disk(disk) => disk.file_source,
            Self::Editor { editor, .. } => editor.file_source,
        }
    }

    /// Returns the parse result of the view that [Self::content] reads.
    pub(crate) fn syntax(&self) -> Option<Result<(), FileTooLarge>> {
        match self {
            Self::Disk(disk) => disk.syntax.clone(),
            Self::Editor { editor, .. } => editor.syntax.clone(),
        }
    }

    /// Returns the version of the editor view, or `None` when no client has
    /// the file open.
    pub(crate) fn version(&self) -> Option<i32> {
        self.editor().map(|editor| editor.version)
    }

    /// Returns the document with `disk` as its disk view.
    pub(crate) fn with_disk(self, disk: DiskView) -> Self {
        match self {
            Self::Disk(_) => Self::Disk(disk),
            Self::Editor { editor, .. } => Self::Editor {
                editor,
                disk: Some(disk),
            },
        }
    }

    /// Returns the document with `editor` as its editor view.
    pub(crate) fn with_editor(self, editor: EditorView) -> Self {
        match self {
            Self::Disk(disk) => Self::Editor {
                editor,
                disk: Some(disk),
            },
            Self::Editor { disk, .. } => Self::Editor { editor, disk },
        }
    }

    /// Returns the document without its editor view, or `None` when it has no
    /// disk view.
    pub(crate) fn without_editor(self) -> Option<Self> {
        match self {
            Self::Disk(disk) => Some(Self::Disk(disk)),
            Self::Editor { disk, .. } => disk.map(Self::Disk),
        }
    }

    /// Returns the document without its disk view, or `None` when it has no
    /// editor view.
    #[cfg(feature = "module_graph")]
    pub(crate) fn without_disk(self) -> Option<Self> {
        match self {
            Self::Disk(_) => None,
            Self::Editor { editor, .. } => Some(editor.into()),
        }
    }
}
