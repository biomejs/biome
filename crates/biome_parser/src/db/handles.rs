//! What analyzers hold: handles to parses that outlive the borrow of the
//! database.

use super::{ParseDb, SnippetOutput, SourceExt, TrackedSnippet};
use crate::AnyParse;
use crate::diagnostic::ParseDiagnostic;
use biome_db::Source;
use biome_rowan::{AstNode, Language, SendNode, SyntaxNode, TextRange, TextSize};

/// A snippet of a [Source] that doesn't borrow the database.
///
/// It holds the parse result and the ranges of the snippet, so reading them
/// never fails. Queries keyed by a snippet need the [TrackedSnippet] itself,
/// which [Self::resolve] returns.
#[derive(Clone, Debug)]
pub struct SnippetRef {
    source: Source,
    index: usize,
    snippet: SnippetOutput,
}

impl SnippetRef {
    /// Creates the reference to the snippet at `index` among the snippets of
    /// `source`, whose output is `snippet`.
    pub(super) fn new(source: Source, index: usize, snippet: SnippetOutput) -> Self {
        Self {
            source,
            index,
            snippet,
        }
    }

    /// Returns the document that contains the snippet.
    pub fn source(&self) -> Source {
        self.source
    }

    /// Returns the [TrackedSnippet] this reference was created from, or
    /// `None` when the current parse of its document doesn't contain it
    /// anymore.
    pub fn resolve<'db>(&self, db: &'db dyn ParseDb) -> Option<TrackedSnippet<'db>> {
        self.source
            .snippets(db)
            .get(self.index)
            .copied()
            .filter(|snippet| *snippet.parse(db) == self.snippet.parse)
    }

    pub fn parsed(&self) -> &AnyParse {
        &self.snippet.parse
    }

    pub fn document_source_index(&self) -> usize {
        self.snippet.document_source_index
    }

    pub fn element_range(&self) -> TextRange {
        self.snippet.element_range
    }

    pub fn content_range(&self) -> TextRange {
        self.snippet.content_range
    }

    pub fn content_offset(&self) -> TextSize {
        self.snippet.content_offset
    }

    pub fn serde_diagnostics(&self) -> Vec<biome_diagnostics::serde::Diagnostic> {
        self.parsed().clone().into_serde_diagnostics()
    }

    pub fn has_errors(&self) -> bool {
        self.parsed().has_errors()
    }
}

/// What an analyzer analyzes: a whole document, or a snippet embedded in one.
#[derive(Clone, Debug)]
pub enum AnalyzedSource {
    Source(Source),
    Snippet(SnippetRef),
}

impl AnalyzedSource {
    /// Returns the document: the source itself, or the document that contains
    /// the snippet.
    pub fn document(&self) -> Source {
        match self {
            Self::Source(source) => *source,
            Self::Snippet(snippet) => snippet.source(),
        }
    }

    pub fn tree<N>(&self, db: &dyn ParseDb) -> N
    where
        N: AstNode,
        N::Language: 'static,
    {
        self.any_parse(db).tree::<N>()
    }

    pub fn into_language_root<N>(self, db: &dyn ParseDb) -> Option<N>
    where
        N: AstNode,
        N::Language: 'static,
    {
        match self {
            Self::Source(parsed) => parsed.parsed(db).clone().into_language_root(),
            Self::Snippet(_) => None,
        }
    }

    pub fn syntax<L>(&self, db: &dyn ParseDb) -> SyntaxNode<L>
    where
        L: Language + 'static,
    {
        self.any_parse(db).syntax()
    }

    pub fn serde_diagnostics(&self, db: &dyn ParseDb) -> Vec<biome_diagnostics::serde::Diagnostic> {
        match self {
            Self::Source(parsed) => parsed.serde_diagnostics(db),
            Self::Snippet(parsed) => parsed.serde_diagnostics(),
        }
    }

    pub fn diagnostics<'a>(&'a self, db: &'a dyn ParseDb) -> &'a [ParseDiagnostic] {
        self.any_parse(db).diagnostics()
    }

    pub fn diagnostic_offset(&self) -> Option<TextSize> {
        match self {
            Self::Source(_) => None,
            Self::Snippet(snippet) => Some(snippet.content_offset()),
        }
    }

    pub fn document_file_index(&self, db: &dyn ParseDb) -> usize {
        match self {
            Self::Source(source) => source.document_source_index(db),
            Self::Snippet(snippet) => snippet.document_source_index(),
        }
    }

    pub fn unwrap_as_send_node(&self, db: &dyn ParseDb) -> SendNode {
        match self {
            Self::Source(source) => source.parsed(db).unwrap_as_send_node(),
            Self::Snippet(_) => panic!("Cannot unwrap a snippet into a SendNode"),
        }
    }

    pub fn any_parse<'a>(&'a self, db: &'a dyn ParseDb) -> &'a AnyParse {
        match self {
            Self::Source(source) => source.parsed(db),
            Self::Snippet(snippet) => snippet.parsed(),
        }
    }

    pub fn has_errors(&self, db: &dyn ParseDb) -> bool {
        match self {
            Self::Source(source) => source.has_errors(db),
            Self::Snippet(snippet) => snippet.has_errors(),
        }
    }
}

impl From<Source> for AnalyzedSource {
    fn from(source: Source) -> Self {
        Self::Source(source)
    }
}

impl From<SnippetRef> for AnalyzedSource {
    fn from(snippet: SnippetRef) -> Self {
        Self::Snippet(snippet)
    }
}

impl From<&Source> for AnalyzedSource {
    fn from(source: &Source) -> Self {
        Self::Source(*source)
    }
}
