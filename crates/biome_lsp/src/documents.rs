use biome_languages::DocumentFileSource;
use biome_line_index::LineIndex;
use biome_service::projects::ProjectKey;
use std::sync::Arc;

/// Represents an open [`textDocument`]. Can be cheaply cloned.
///
/// [`textDocument`]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentItem
#[derive(Debug, Clone)]
pub(crate) struct Document {
    pub(crate) project_key: ProjectKey,
    pub(crate) version: i32,
    pub(crate) line_index: LineIndex,
    /// The text last sent by *this* client. The shared workspace may hold
    /// another client's text, so anything mapped onto this client's buffer
    /// must be computed from this one.
    pub(crate) content: Arc<str>,
    /// The language declared at open, to re-open the file with it.
    pub(crate) file_source: DocumentFileSource,
}

impl Document {
    pub(crate) fn new(
        project_key: ProjectKey,
        version: i32,
        text: &str,
        file_source: DocumentFileSource,
    ) -> Self {
        Self {
            project_key,
            version,
            line_index: LineIndex::new(text),
            content: Arc::from(text),
            file_source,
        }
    }
}
