//! Parsing on top of the Salsa database of [biome_db].
//!
//! A [Source] holds the text of a file and the file source it's parsed
//! as, never a parse result. A database that implements [ParseDb] knows how to
//! parse it, and a tracked query derives the parse from the text the first
//! time it's read. The same query creates the snippets embedded in the file as
//! tracked structs, so parsing the file again reuses their identities: a query
//! keyed by a snippet runs again only when the snippet fields it reads change.

use crate::AnyParse;
use crate::diagnostic::ParseDiagnostic;
use biome_db::{Db, Source};
use biome_diagnostics::{Diagnostic, Severity};
use biome_rowan::{AstNode, Language, SendNode, SyntaxNode, TextRange, TextSize};
use std::ops::Deref;
use std::sync::Arc;

/// A database that parses the texts of its [Source] inputs.
#[salsa::db]
pub trait ParseDb: Db {
    /// Parses the text of `source` as its file source, along with the snippets
    /// embedded in it.
    ///
    /// Only the tracked parse of `source` calls it, so Salsa tracks what it
    /// reads from the database: a change to it, such as to the settings of the
    /// file, parses the text again.
    fn parse_source(&self, source: Source) -> SourceParse;
}

/// A parse result compared by allocation identity.
///
/// Comparing two syntax trees walks both of them. Two handles are equal only
/// when they share the same allocation, so parsing the same text twice yields
/// handles that aren't equal.
#[derive(Clone, Debug)]
pub struct SharedParse(Arc<AnyParse>);

impl From<AnyParse> for SharedParse {
    fn from(parse: AnyParse) -> Self {
        Self(Arc::new(parse))
    }
}

impl Deref for SharedParse {
    type Target = AnyParse;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for SharedParse {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for SharedParse {}

/// The result of parsing a file: its document and the snippets embedded in it.
#[derive(Clone, Debug)]
pub struct SourceParse {
    pub parse: SharedParse,

    /// The index of the file source of the document.
    pub document_source_index: usize,

    pub snippets: Vec<SnippetParse>,
}

impl SourceParse {
    /// Creates the parse result of a document without snippets.
    pub fn new(parse: AnyParse, document_source_index: usize) -> Self {
        Self {
            parse: parse.into(),
            document_source_index,
            snippets: Vec::new(),
        }
    }

    pub fn with_snippets(mut self, snippets: Vec<SnippetParse>) -> Self {
        self.snippets = snippets;
        self
    }
}

/// The result of parsing content embedded in a document, such as the content
/// of a `<script>` element.
#[derive(Clone, Debug)]
pub struct SnippetParse {
    pub parse: SharedParse,

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

impl SnippetParse {
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

#[salsa::tracked]
#[derive(Debug)]
struct ParsedRoot<'db> {
    #[tracked]
    #[returns(ref)]
    parse: SharedParse,

    #[tracked]
    #[returns(copy)]
    document_source_index: usize,

    #[tracked]
    #[returns(ref)]
    snippets: Vec<ParsedSnippet<'db>>,
}

/// Parses the text of `source`.
///
/// The snippets are created again in the same order every time the text is
/// parsed, so the snippet at an index keeps its identity as long as the
/// snippets before it keep their file sources.
#[salsa::tracked(returns(copy))]
fn parsed_root(db: &dyn ParseDb, source: Source) -> ParsedRoot<'_> {
    let SourceParse {
        parse,
        document_source_index,
        snippets,
    } = db.parse_source(source);
    let snippets = snippets
        .into_iter()
        .enumerate()
        .map(|(index, snippet)| {
            ParsedSnippet::new(
                db,
                source,
                snippet.document_source_index,
                index,
                snippet.parse,
                snippet.element_range,
                snippet.content_range,
                snippet.content_offset,
            )
        })
        .collect();
    ParsedRoot::new(db, parse, document_source_index, snippets)
}

/// The parse of a [Source], which a [ParseDb] computes the first time
/// it's read.
pub trait ParsedSourceExt: Copy {
    fn parsed(self, db: &dyn ParseDb) -> &AnyParse;

    /// Returns the index of the file source of the parsed document. It can
    /// differ from the file source of the source, such as for a Vue file whose
    /// script is parsed as TypeScript.
    fn document_source_index(self, db: &dyn ParseDb) -> usize;

    fn snippets(self, db: &dyn ParseDb) -> &Vec<ParsedSnippet<'_>>;

    fn error_count(self, db: &dyn ParseDb) -> usize {
        self.parsed(db)
            .diagnostics()
            .iter()
            .filter(|d| d.severity() >= Severity::Error)
            .count()
            + self
                .snippets(db)
                .iter()
                .filter(|s| s.has_errors(db))
                .count()
    }

    fn serde_diagnostics(self, db: &dyn ParseDb) -> Vec<biome_diagnostics::serde::Diagnostic> {
        self.parsed(db).clone().into_serde_diagnostics()
    }

    fn parse_diagnostics(self, db: &dyn ParseDb) -> Vec<ParseDiagnostic> {
        self.parsed(db).diagnostics().to_vec()
    }

    fn snippets_parse_diagnostics(self, db: &dyn ParseDb) -> Vec<ParseDiagnostic> {
        let mut diagnostics = vec![];
        for snippet in self.snippets(db) {
            diagnostics.extend(snippet.parsed(db).diagnostics().to_vec())
        }
        diagnostics
    }

    fn has_errors(self, db: &dyn ParseDb) -> bool {
        self.error_count(db) > 0
    }
}

impl ParsedSourceExt for Source {
    fn parsed(self, db: &dyn ParseDb) -> &AnyParse {
        parsed_root(db, self).parse(db)
    }

    fn document_source_index(self, db: &dyn ParseDb) -> usize {
        parsed_root(db, self).document_source_index(db)
    }

    fn snippets(self, db: &dyn ParseDb) -> &Vec<ParsedSnippet<'_>> {
        parsed_root(db, self).snippets(db)
    }
}

/// Represents embedded content extracted from a document, such as HTML.
///
/// This struct stores parsing metadata and provides access to the parsed
/// content with offset-aware positioning to maintain correct source locations.
///
/// A snippet borrows the database. Use [ParsedSnippetRef] to refer to it from
/// data that outlives the borrow, such as the services of an analyzer.
#[salsa::tracked]
#[derive(Debug)]
pub struct ParsedSnippet<'db> {
    /// The document that contains the snippet.
    #[returns(copy)]
    pub source: Source,

    /// The index of the file source of the content.
    #[returns(copy)]
    pub document_source_index: usize,

    /// The position of the snippet among the snippets of its document.
    #[tracked]
    #[returns(copy)]
    index: usize,

    #[tracked]
    #[returns(ref)]
    parse: SharedParse,

    /// The range of the entire element in the document, including its tags.
    #[tracked]
    #[returns(copy)]
    pub element_range: TextRange,

    /// The range of the content of the element in the document, excluding
    /// its tags.
    #[tracked]
    #[returns(copy)]
    pub content_range: TextRange,

    /// The offset where the content starts in the document.
    #[tracked]
    #[returns(copy)]
    pub content_offset: TextSize,
}

impl<'db> ParsedSnippet<'db> {
    pub fn parsed(self, db: &'db dyn ParseDb) -> &'db AnyParse {
        self.parse(db)
    }

    pub fn to_ref(self, db: &dyn ParseDb) -> ParsedSnippetRef {
        ParsedSnippetRef {
            source: self.source(db),
            index: self.index(db),
            snippet: SnippetParse {
                parse: self.parse(db).clone(),
                element_range: self.element_range(db),
                content_range: self.content_range(db),
                content_offset: self.content_offset(db),
                document_source_index: self.document_source_index(db),
            },
        }
    }

    pub fn serde_diagnostics(&self, db: &dyn ParseDb) -> Vec<biome_diagnostics::serde::Diagnostic> {
        self.parsed(db).clone().into_serde_diagnostics()
    }

    pub fn has_errors(&self, db: &dyn ParseDb) -> bool {
        self.parsed(db).has_errors()
    }
}

/// A snippet of a [Source] that doesn't borrow the database.
///
/// It holds the parse result and the ranges of the snippet, so reading them
/// never fails. Queries keyed by a snippet need the [ParsedSnippet] itself,
/// which [Self::resolve] returns.
#[derive(Clone, Debug)]
pub struct ParsedSnippetRef {
    source: Source,
    index: usize,
    snippet: SnippetParse,
}

impl ParsedSnippetRef {
    /// Returns the document that contains the snippet.
    pub fn source(&self) -> Source {
        self.source
    }

    /// Returns the [ParsedSnippet] this reference was created from, or `None`
    /// when the current parse of its document doesn't contain it anymore.
    pub fn resolve<'db>(&self, db: &'db dyn ParseDb) -> Option<ParsedSnippet<'db>> {
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

/// Convenient type for source
#[derive(Clone, Debug)]
pub enum AnyParsedSource {
    Source(Source),
    ParsedSnippet(ParsedSnippetRef),
}

impl AnyParsedSource {
    /// Returns the document: the source itself, or the document that contains
    /// the snippet.
    pub fn document(&self) -> Source {
        match self {
            Self::Source(source) => *source,
            Self::ParsedSnippet(snippet) => snippet.source(),
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
            Self::ParsedSnippet(_) => None,
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
            Self::ParsedSnippet(parsed) => parsed.serde_diagnostics(),
        }
    }

    pub fn diagnostics<'a>(&'a self, db: &'a dyn ParseDb) -> &'a [ParseDiagnostic] {
        self.any_parse(db).diagnostics()
    }

    pub fn diagnostic_offset(&self) -> Option<TextSize> {
        match self {
            Self::Source(_) => None,
            Self::ParsedSnippet(snippet) => Some(snippet.content_offset()),
        }
    }

    pub fn document_file_index(&self, db: &dyn ParseDb) -> usize {
        match self {
            Self::Source(source) => source.document_source_index(db),
            Self::ParsedSnippet(snippet) => snippet.document_source_index(),
        }
    }

    pub fn unwrap_as_send_node(&self, db: &dyn ParseDb) -> SendNode {
        match self {
            Self::Source(source) => source.parsed(db).unwrap_as_send_node(),
            Self::ParsedSnippet(_) => panic!("Cannot unwrap ParsedSnippet into SendNode"),
        }
    }

    pub fn any_parse<'a>(&'a self, db: &'a dyn ParseDb) -> &'a AnyParse {
        match self {
            Self::Source(source) => source.parsed(db),
            Self::ParsedSnippet(snippet) => snippet.parsed(),
        }
    }

    pub fn has_errors(&self, db: &dyn ParseDb) -> bool {
        match self {
            Self::Source(source) => source.has_errors(db),
            Self::ParsedSnippet(snippet) => snippet.has_errors(),
        }
    }
}

impl From<Source> for AnyParsedSource {
    fn from(source: Source) -> Self {
        Self::Source(source)
    }
}

impl From<ParsedSnippetRef> for AnyParsedSource {
    fn from(snippet: ParsedSnippetRef) -> Self {
        Self::ParsedSnippet(snippet)
    }
}

impl From<&Source> for AnyParsedSource {
    fn from(source: &Source) -> Self {
        Self::Source(*source)
    }
}
