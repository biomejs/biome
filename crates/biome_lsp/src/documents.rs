use crate::converters::line_index::LineIndex;

/// Represents an open [`textDocument`]. Can be cheaply cloned.
///
/// [`textDocument`]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentItem
#[derive(Clone)]
pub(crate) struct Document {
    pub(crate) version: i32,
    pub(crate) line_index: LineIndex,
}

impl Document {
    pub(crate) fn new(version: i32, text: &str) -> Self {
        Self {
            version,
            line_index: LineIndex::new(text),
        }
    }
}
