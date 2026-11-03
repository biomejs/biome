//! What a parser returns: plain values that Salsa doesn't track.

use crate::AnyParse;
use biome_rowan::{TextRange, TextSize};
use std::ops::Deref;
use std::sync::Arc;

/// A parse result compared by allocation identity.
///
/// Comparing two syntax trees walks both of them. Two handles are equal only
/// when they share the same allocation, so parsing the same text twice yields
/// handles that aren't equal.
#[derive(Clone, Debug)]
pub struct ParseHandle(Arc<AnyParse>);

impl From<AnyParse> for ParseHandle {
    fn from(parse: AnyParse) -> Self {
        Self(Arc::new(parse))
    }
}

impl Deref for ParseHandle {
    type Target = AnyParse;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for ParseHandle {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for ParseHandle {}

/// What parsing a source returns: the parse of its document and the snippets
/// embedded in it.
#[derive(Clone, Debug)]
pub struct SourceOutput {
    pub parse: ParseHandle,

    /// The index of the file source of the document.
    pub document_source_index: usize,

    pub snippets: Vec<SnippetOutput>,
}

impl SourceOutput {
    /// Creates the output of a document without snippets.
    pub fn new(parse: AnyParse, document_source_index: usize) -> Self {
        Self {
            parse: parse.into(),
            document_source_index,
            snippets: Vec::new(),
        }
    }

    pub fn with_snippets(mut self, snippets: Vec<SnippetOutput>) -> Self {
        self.snippets = snippets;
        self
    }
}

/// What parsing content embedded in a document returns, such as the content
/// of a `<script>` element.
#[derive(Clone, Debug)]
pub struct SnippetOutput {
    pub parse: ParseHandle,

    /// The range of the entire element in the document, for example a
    /// `<script>` element including its tags.
    pub element_range: TextRange,

    /// The range of the content of the element in the document, excluding
    /// its tags.
    pub content_range: TextRange,

    /// The offset where the content starts in the document. The content is
    /// parsed with this offset, so its ranges are ranges of the document.
    pub content_offset: TextSize,

    /// The index of the file source of the content.
    pub document_source_index: usize,
}

impl SnippetOutput {
    pub fn new(
        parse: AnyParse,
        element_range: TextRange,
        content_range: TextRange,
        content_offset: TextSize,
        document_source_index: usize,
    ) -> Self {
        Self {
            parse: parse.into(),
            element_range,
            content_range,
            content_offset,
            document_source_index,
        }
    }
}
