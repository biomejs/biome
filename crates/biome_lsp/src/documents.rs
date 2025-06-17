use biome_lsp_converters::line_index::LineIndex;
use biome_service::projects::ProjectKey;

/// Represents an open [`textDocument`]. Can be cheaply cloned.
///
/// [`textDocument`]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentItem
#[derive(Clone)]
pub(crate) struct Document {
    pub(crate) project_key: ProjectKey,
    pub(crate) version: i32,
    pub(crate) line_index: LineIndex,
}

impl Document {
    pub(crate) fn new(project_key: ProjectKey, version: i32, text: &str) -> Self {
        Self {
            project_key,
            version,
            line_index: LineIndex::new(text),
        }
    }
}
