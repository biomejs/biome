pub(crate) mod services;

use crate::diagnostics::FileTooLarge;
use crate::workspace::views::{ClientId, Views};
use biome_languages::DocumentFileSource;
use std::sync::Arc;

/// The views of a file that the workspace knows about. See
/// [views](crate::workspace::views).
pub(crate) type Document = Views<DocumentView>;

/// The content of a file in one of its views.
#[derive(Clone, Debug)]
pub(crate) struct DocumentView {
    /// Document content.
    ///
    /// The content of the file is only available if it belongs to the project. For example, we don't
    /// want to store the content of files coming from dependencies.
    ///
    /// The database shares the same allocation as the text of the file.
    pub(crate) content: Arc<str>,

    /// The version of the content, as specified by the client of an editor
    /// view. The disk view has none.
    pub(crate) version: Option<i32>,

    /// The file source the content is parsed as.
    pub(crate) file_source: DocumentFileSource,

    /// The result of the parser (syntax tree + diagnostics).
    /// Types explained:
    /// - `Option`: if the file can be read
    /// - `Result`: if the file is read, but the file is too large
    pub(crate) syntax: Option<Result<(), FileTooLarge>>,
}

impl Document {
    /// Returns the content that `client` reads. See [Views::for_client].
    pub(crate) fn content(&self, client: ClientId) -> Option<&Arc<str>> {
        self.for_client(client).map(|view| &view.content)
    }

    /// Returns the parse result of the view that `client` reads.
    pub(crate) fn syntax(&self, client: ClientId) -> Option<Result<(), FileTooLarge>> {
        self.for_client(client).and_then(|view| view.syntax.clone())
    }

    /// Returns the file source of the view that `client` reads, or the one of
    /// the primary editor view when only other clients have the file open.
    pub(crate) fn file_source(&self, client: ClientId) -> DocumentFileSource {
        self.for_client(client)
            .unwrap_or_else(|| self.current())
            .file_source
    }

    /// Returns the version of the editor view of `client`, or `None` when it
    /// doesn't have the file open.
    pub(crate) fn version(&self, client: ClientId) -> Option<i32> {
        self.editor(client).and_then(|view| view.version)
    }
}
