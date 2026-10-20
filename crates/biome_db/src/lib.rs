//! The Salsa database shared by the Biome crates.
//!
//! A file enters the database as a [ParsedSource] input. The input holds what
//! the file is made of, usually its text, and a tracked query derives the
//! parse from it. The same query creates the snippets embedded in the file as
//! tracked structs, so parsing the file again reuses their identities: a query
//! keyed by a snippet runs again only when the snippet fields it reads change.

pub mod testing;

use biome_diagnostics::{Diagnostic, Severity};
use biome_parser::AnyParse;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_rowan::{AstNode, Language, SendNode, SyntaxNode, TextRange, TextSize};
use camino::{Utf8Path, Utf8PathBuf};
use std::any::Any;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;

#[salsa::db]
pub trait Db: salsa::Database {
    fn parsed_source_for_path(&self, path: &Utf8Path) -> Option<ParsedSource>;

    /// Parses `text`, the content of `source`.
    ///
    /// Databases that only create sources from a [SourceParse] don't parse
    /// text. The default implementation panics.
    fn parse_text(&self, source: ParsedSource, text: &SourceText) -> SourceParse {
        let _ = (source, text);
        unimplemented!("this database can't parse text")
    }
}

/// The primordial type of the biome database. It represents a file.
/// The `path` is the ID.
///
/// The parse is derived from the content, and it is computed the first time it
/// is read.
#[salsa::input]
#[derive(Debug)]
pub struct ParsedSource {
    pub path: Utf8PathBuf,

    #[no_eq]
    #[returns(ref)]
    pub content: SourceContent,
}

/// What a [ParsedSource] is made of.
#[derive(Clone, Debug)]
pub enum SourceContent {
    /// The text of the file, parsed with [Db::parse_text].
    Text(SourceText),
    /// The result of a parse made outside the database.
    ///
    /// It suits databases that don't parse text, such as the ones of tests.
    Parsed(SourceParse),
}

impl From<SourceText> for SourceContent {
    fn from(text: SourceText) -> Self {
        Self::Text(text)
    }
}

impl From<SourceParse> for SourceContent {
    fn from(parse: SourceParse) -> Self {
        Self::Parsed(parse)
    }
}

/// The text of a file, and what the database needs to parse it.
#[derive(Clone, Debug)]
pub struct SourceText {
    pub text: Arc<str>,

    /// The index of the file source to parse the text as.
    ///
    /// The parse can select a more specific file source, which
    /// [ParsedSource::document_source_index] returns.
    pub source_index: usize,

    pub context: ParseContext,
}

/// Data that the database reads to parse a [SourceText], such as the settings
/// the file was opened with.
///
/// Only the database that creates the context reads it, so its type is erased
/// here.
#[derive(Clone)]
pub struct ParseContext(Arc<dyn Any + Send + Sync>);

impl ParseContext {
    pub fn new(value: impl Any + Send + Sync) -> Self {
        Self(Arc::new(value))
    }

    /// Returns the value of the context, or `None` when it has another type.
    pub fn get<T: Any>(&self) -> Option<&T> {
        self.0.downcast_ref()
    }
}

impl Debug for ParseContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ParseContext(..)")
    }
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

/// Parses the content of `source`.
///
/// The snippets are created again in the same order every time the content is
/// parsed, so the snippet at an index keeps its identity as long as the
/// snippets before it keep their file sources.
#[salsa::tracked(returns(copy))]
fn parsed_root(db: &dyn Db, source: ParsedSource) -> ParsedRoot<'_> {
    let SourceParse {
        parse,
        document_source_index,
        snippets,
    } = match source.content(db) {
        SourceContent::Text(text) => db.parse_text(source, text),
        SourceContent::Parsed(parse) => parse.clone(),
    };
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

impl ParsedSource {
    /// Creates a source from the result of a parse made outside the database.
    pub fn from_parse(
        db: &dyn Db,
        path: Utf8PathBuf,
        parse: AnyParse,
        document_source_index: usize,
        snippets: Vec<SnippetParse>,
    ) -> Self {
        Self::new(
            db,
            path,
            SourceParse::new(parse, document_source_index)
                .with_snippets(snippets)
                .into(),
        )
    }

    pub fn parsed(self, db: &dyn Db) -> &AnyParse {
        parsed_root(db, self).parse(db)
    }

    /// Returns the index of the file source of the document.
    pub fn document_source_index(self, db: &dyn Db) -> usize {
        parsed_root(db, self).document_source_index(db)
    }

    pub fn snippets(self, db: &dyn Db) -> &Vec<ParsedSnippet<'_>> {
        parsed_root(db, self).snippets(db)
    }

    pub fn error_count(&self, db: &dyn Db) -> usize {
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

    pub fn serde_diagnostics(&self, db: &dyn Db) -> Vec<biome_diagnostics::serde::Diagnostic> {
        self.parsed(db).clone().into_serde_diagnostics()
    }

    pub fn parse_diagnostics(&self, db: &dyn Db) -> Vec<ParseDiagnostic> {
        self.parsed(db).diagnostics().to_vec()
    }

    pub fn snippets_parse_diagnostics(&self, db: &dyn Db) -> Vec<ParseDiagnostic> {
        let mut diagnostics = vec![];
        for snippet in self.snippets(db) {
            diagnostics.extend(snippet.parsed(db).diagnostics().to_vec())
        }
        diagnostics
    }

    pub fn has_errors(&self, db: &dyn Db) -> bool {
        self.error_count(db) > 0
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
    pub source: ParsedSource,

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
    pub fn parsed(self, db: &'db dyn Db) -> &'db AnyParse {
        self.parse(db)
    }

    pub fn to_ref(self, db: &dyn Db) -> ParsedSnippetRef {
        ParsedSnippetRef::new(self.source(db), self.index(db))
    }

    pub fn serde_diagnostics(&self, db: &dyn Db) -> Vec<biome_diagnostics::serde::Diagnostic> {
        self.parsed(db).clone().into_serde_diagnostics()
    }

    pub fn has_errors(&self, db: &dyn Db) -> bool {
        self.parsed(db).has_errors()
    }
}

/// Refers to a [ParsedSnippet] by its document and its position, without
/// borrowing the database.
///
/// The reference resolves to the snippet at that position in the current parse
/// of the document. It resolves to the snippet it was created from only in the
/// revision that created it. A database handle keeps that revision current
/// while it's alive, so resolve a reference while the handle it was created
/// from, or one created before it, is still alive.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ParsedSnippetRef {
    source: ParsedSource,
    index: usize,
}

impl ParsedSnippetRef {
    /// Refers to the snippet at `index` among the snippets of `source`.
    pub fn new(source: ParsedSource, index: usize) -> Self {
        Self { source, index }
    }

    /// Returns the snippet.
    ///
    /// ## Panics
    ///
    /// Panics if the current parse of the document has fewer snippets than
    /// the one the reference was created from.
    pub fn resolve(self, db: &dyn Db) -> ParsedSnippet<'_> {
        self.source.snippets(db)[self.index]
    }

    pub fn parsed(self, db: &dyn Db) -> &AnyParse {
        self.resolve(db).parsed(db)
    }

    pub fn document_source_index(self, db: &dyn Db) -> usize {
        self.resolve(db).document_source_index(db)
    }

    pub fn element_range(self, db: &dyn Db) -> TextRange {
        self.resolve(db).element_range(db)
    }

    pub fn content_range(self, db: &dyn Db) -> TextRange {
        self.resolve(db).content_range(db)
    }

    pub fn content_offset(self, db: &dyn Db) -> TextSize {
        self.resolve(db).content_offset(db)
    }

    pub fn serde_diagnostics(self, db: &dyn Db) -> Vec<biome_diagnostics::serde::Diagnostic> {
        self.parsed(db).clone().into_serde_diagnostics()
    }

    pub fn has_errors(self, db: &dyn Db) -> bool {
        self.parsed(db).has_errors()
    }
}

/// Convenient type for source
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AnyParsedSource {
    ParsedSource(ParsedSource),
    ParsedSnippet(ParsedSnippetRef),
}

impl AnyParsedSource {
    pub fn tree<N>(&self, db: &dyn Db) -> N
    where
        N: AstNode,
        N::Language: 'static,
    {
        self.any_parse(db).tree::<N>()
    }

    pub fn into_language_root<N>(self, db: &dyn Db) -> Option<N>
    where
        N: AstNode,
        N::Language: 'static,
    {
        match self {
            Self::ParsedSource(parsed) => parsed.parsed(db).clone().into_language_root(),
            Self::ParsedSnippet(_) => None,
        }
    }

    pub fn syntax<L>(&self, db: &dyn Db) -> SyntaxNode<L>
    where
        L: Language + 'static,
    {
        self.any_parse(db).syntax()
    }

    pub fn serde_diagnostics(&self, db: &dyn Db) -> Vec<biome_diagnostics::serde::Diagnostic> {
        match self {
            Self::ParsedSource(parsed) => parsed.serde_diagnostics(db),
            Self::ParsedSnippet(parsed) => parsed.serde_diagnostics(db),
        }
    }

    pub fn diagnostics<'db>(&self, db: &'db dyn Db) -> &'db [ParseDiagnostic] {
        self.any_parse(db).diagnostics()
    }

    pub fn diagnostic_offset(&self, db: &dyn Db) -> Option<TextSize> {
        match self {
            Self::ParsedSource(_) => None,
            Self::ParsedSnippet(snippet) => Some(snippet.content_offset(db)),
        }
    }

    pub fn document_file_index(&self, db: &dyn Db) -> usize {
        match self {
            Self::ParsedSource(source) => source.document_source_index(db),
            Self::ParsedSnippet(snippet) => snippet.document_source_index(db),
        }
    }

    pub fn unwrap_as_send_node(&self, db: &dyn Db) -> SendNode {
        match self {
            Self::ParsedSource(source) => source.parsed(db).unwrap_as_send_node(),
            Self::ParsedSnippet(_) => panic!("Cannot unwrap ParsedSnippet into SendNode"),
        }
    }

    pub fn any_parse<'db>(&self, db: &'db dyn Db) -> &'db AnyParse {
        match self {
            Self::ParsedSource(source) => source.parsed(db),
            Self::ParsedSnippet(snippet) => snippet.parsed(db),
        }
    }

    pub fn has_errors(&self, db: &dyn Db) -> bool {
        match self {
            Self::ParsedSource(source) => source.has_errors(db),
            Self::ParsedSnippet(snippet) => snippet.has_errors(db),
        }
    }
}

impl From<ParsedSource> for AnyParsedSource {
    fn from(source: ParsedSource) -> Self {
        Self::ParsedSource(source)
    }
}

impl From<ParsedSnippetRef> for AnyParsedSource {
    fn from(snippet: ParsedSnippetRef) -> Self {
        Self::ParsedSnippet(snippet)
    }
}

impl From<&ParsedSource> for AnyParsedSource {
    fn from(source: &ParsedSource) -> Self {
        Self::ParsedSource(*source)
    }
}
