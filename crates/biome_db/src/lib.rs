pub mod testing;

use biome_diagnostics::{Diagnostic, Severity};
use biome_parser::AnyParse;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_rowan::{AstNode, Language, SendNode, SyntaxNode, TextRange, TextSize};
use camino::{Utf8Path, Utf8PathBuf};

#[salsa::db]
pub trait Db: salsa::Database {
    fn parsed_source_for_path(&self, path: &Utf8Path) -> Option<ParsedSource>;

    fn parsed_snippets_for_path(&self, path: &Utf8Path) -> Vec<ParsedSnippet> {
        self.parsed_source_for_path(path)
            .map(|source| source.snippets(self).iter().copied().collect::<Vec<_>>())
            .unwrap_or_default()
    }
}

/// The primordial type of the biome database. It represents a parsed file.
/// The `path` is the ID.
#[salsa::input]
#[derive(Debug)]
pub struct ParsedSource {
    #[returns(ref)]
    pub path: Utf8PathBuf,

    #[returns(ref)]
    #[no_eq]
    pub parsed: AnyParse,

    pub document_source_index: usize,

    #[returns(ref)]
    pub snippets: Vec<ParsedSnippet>,
}

impl ParsedSource {
    pub fn error_count(&self, db: &dyn Db) -> usize {
        self.parsed(db)
            .diagnostics()
            .iter()
            .filter(|d| d.severity() >= Severity::Error)
            .count()
    }

    pub fn serde_diagnostics(&self, db: &dyn Db) -> Vec<biome_diagnostics::serde::Diagnostic> {
        self.parsed(db).clone().into_serde_diagnostics()
    }

    pub fn parse_diagnostics<'db>(&self, db: &'db dyn Db) -> Vec<ParseDiagnostic> {
        self.parsed(db)
            .diagnostics()
            .into_iter()
            .cloned()
            .collect::<Vec<_>>()
    }

    pub fn snippets_parse_diagnostics<'db>(&self, db: &'db dyn Db) -> Vec<ParseDiagnostic> {
        let mut diagnostics = vec![];
        for snippet in self.snippets(db) {
            diagnostics.extend(
                snippet
                    .clone()
                    .parsed(db)
                    .diagnostics()
                    .into_iter()
                    .cloned()
                    .collect::<Vec<ParseDiagnostic>>(),
            )
        }
        diagnostics
    }

    pub fn has_errors(&self, db: &dyn Db) -> bool {
        self.error_count(db) > 0
    }
}

/// Represents embedded content extracted from HTML documents.
///
/// This struct stores parsing metadata and provides access to the parsed
/// content with offset-aware positioning to maintain correct source locations.
#[salsa::input]
#[derive(Debug)]
pub struct ParsedSnippet {
    #[returns(ref)]
    #[no_eq]
    pub parsed: AnyParse,

    /// The range of the entire script element in the HTML document,
    /// including the opening and closing tags.
    #[returns(clone)]
    pub element_range: TextRange,

    /// The range of just the JavaScript content within the script element,
    /// excluding the script tags themselves.
    #[returns(clone)]
    pub content_range: TextRange,

    /// The offset where the JavaScript content starts in the parent document.
    /// This is used for offset-aware parsing.
    #[returns(clone)]
    pub content_offset: TextSize,

    /// The file source of the document
    pub document_source_index: usize,
}

impl ParsedSnippet {
    pub fn serde_diagnostics(&self, db: &dyn Db) -> Vec<biome_diagnostics::serde::Diagnostic> {
        self.parsed(db).clone().into_serde_diagnostics()
    }

    pub fn has_errors(&self, db: &dyn Db) -> bool {
        self.parsed(db).has_errors()
    }
}

/// Convenient type for source
#[derive(Debug, Clone)]
pub enum AnyParsedSource {
    ParsedSource(ParsedSource),
    ParsedSnippet(ParsedSnippet),
}

impl AnyParsedSource {
    pub fn tree<N>(&self, db: &dyn Db) -> N
    where
        N: AstNode,
        N::Language: 'static,
    {
        match self {
            Self::ParsedSource(parsed) => parsed.parsed(db).tree::<N>(),
            Self::ParsedSnippet(parsed) => parsed.parsed(db).tree::<N>(),
        }
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

    pub fn syntax<L: Language>(&self, db: &dyn Db) -> SyntaxNode<L>
    where
        L: Language + 'static,
    {
        match self {
            Self::ParsedSource(parsed) => parsed.parsed(db).syntax(),
            Self::ParsedSnippet(parsed) => parsed.parsed(db).syntax(),
        }
    }

    pub fn serde_diagnostics(&self, db: &dyn Db) -> Vec<biome_diagnostics::serde::Diagnostic> {
        match self {
            Self::ParsedSource(parsed) => parsed.serde_diagnostics(db),
            Self::ParsedSnippet(parsed) => parsed.serde_diagnostics(db),
        }
    }

    pub fn diagnostics<'db>(&self, db: &'db dyn Db) -> &'db [ParseDiagnostic] {
        match self {
            Self::ParsedSource(parsed) => parsed.parsed(db).diagnostics(),
            Self::ParsedSnippet(parsed) => parsed.parsed(db).diagnostics(),
        }
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

    pub fn any_parse<'a>(&self, db: &'a dyn Db) -> &'a AnyParse {
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
        AnyParsedSource::ParsedSource(source)
    }
}

impl From<ParsedSnippet> for AnyParsedSource {
    fn from(snippet: ParsedSnippet) -> Self {
        AnyParsedSource::ParsedSnippet(snippet)
    }
}

impl From<&ParsedSource> for AnyParsedSource {
    fn from(source: &ParsedSource) -> Self {
        AnyParsedSource::ParsedSource(source.clone())
    }
}

impl From<&ParsedSnippet> for AnyParsedSource {
    fn from(snippet: &ParsedSnippet) -> Self {
        AnyParsedSource::ParsedSnippet(snippet.clone())
    }
}
