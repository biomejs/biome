//! What Salsa tracks: the parse of a source and the snippets embedded in it,
//! which borrow the database.

use super::{ParseDb, ParseHandle, SnippetOutput, SnippetRef, SourceOutput};
use crate::AnyParse;
use crate::diagnostic::ParseDiagnostic;
use biome_db::Source;
use biome_diagnostics::{Diagnostic, Severity};
use biome_rowan::{TextRange, TextSize};

/// Parses the text of `source` and the snippets embedded in it.
///
/// The cache keeps the 200 parses read most recently. Reading an evicted parse
/// parses the text again.
#[salsa::tracked(returns(ref), no_eq, lru = 200)]
fn tracked_source(db: &dyn ParseDb, source: Source) -> SourceOutput {
    db.parse_source(source)
}

/// Returns the file source index that the parser chose for the document of
/// `source`.
///
/// This query caches the index on its own, so reading the index doesn't parse
/// the text again after the cache evicts the parse.
#[salsa::tracked(returns(copy))]
fn tracked_document_source_index(db: &dyn ParseDb, source: Source) -> usize {
    tracked_source(db, source).document_source_index
}

/// Creates the [TrackedSnippet]s of `source`, in the order the parser returns
/// them.
///
/// Parsing the text again creates the snippets again in the same order. A
/// snippet keeps its identity while the snippets before it keep their file
/// sources. This query caches the snippets on their own, so reading them
/// doesn't parse the text again after the cache evicts the parse.
#[salsa::tracked(returns(ref))]
fn tracked_snippets(db: &dyn ParseDb, source: Source) -> Vec<TrackedSnippet<'_>> {
    tracked_source(db, source)
        .snippets
        .iter()
        .enumerate()
        .map(|(index, snippet)| {
            TrackedSnippet::new(
                db,
                source,
                snippet.document_source_index,
                index,
                snippet.element_range,
                snippet.content_range,
                snippet.content_offset,
            )
        })
        .collect()
}

/// Returns the parse of `snippet`.
///
/// A query that reads this parse runs again only when the snippet gets a new
/// parse, not when another part of the document changes. The cache keeps the
/// 200 snippet parses read most recently.
#[salsa::tracked(returns(ref), lru = 200)]
fn tracked_snippet_parse<'db>(db: &'db dyn ParseDb, snippet: TrackedSnippet<'db>) -> ParseHandle {
    snippet.output(db).parse.clone()
}

/// Reads the parse of a [Source]. A [ParseDb] computes the parse the first
/// time it's read.
pub trait SourceExt: Copy {
    fn parsed(self, db: &dyn ParseDb) -> &AnyParse;

    /// Returns the index of the file source of the parsed document. It can
    /// differ from the file source of the source, such as for a Vue file whose
    /// script is parsed as TypeScript.
    fn document_source_index(self, db: &dyn ParseDb) -> usize;

    fn snippets(self, db: &dyn ParseDb) -> &Vec<TrackedSnippet<'_>>;

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

impl SourceExt for Source {
    fn parsed(self, db: &dyn ParseDb) -> &AnyParse {
        &tracked_source(db, self).parse
    }

    fn document_source_index(self, db: &dyn ParseDb) -> usize {
        tracked_document_source_index(db, self)
    }

    fn snippets(self, db: &dyn ParseDb) -> &Vec<TrackedSnippet<'_>> {
        tracked_snippets(db, self)
    }
}

/// A snippet embedded in a document, such as the content of a `<script>`
/// element, as Salsa tracks it.
///
/// Queries can be keyed by a snippet. The snippet keeps its identity when its
/// document is parsed again, and its parse comes from the parse of the
/// document. A snippet borrows the database. To refer to it from data that
/// outlives the borrow, such as the services of an analyzer, use a
/// [SnippetRef].
#[salsa::tracked]
#[derive(Debug)]
pub struct TrackedSnippet<'db> {
    /// The document that contains the snippet.
    #[returns(copy)]
    pub source: Source,

    /// The index of the file source of the content.
    #[returns(copy)]
    pub document_source_index: usize,

    /// The position of the snippet among the snippets of its document.
    #[tracked]
    #[returns(copy)]
    pub(super) index: usize,

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

impl<'db> TrackedSnippet<'db> {
    pub fn parsed(self, db: &'db dyn ParseDb) -> &'db AnyParse {
        self.parse(db)
    }

    pub(super) fn parse(self, db: &'db dyn ParseDb) -> &'db ParseHandle {
        tracked_snippet_parse(db, self)
    }

    pub fn to_ref(self, db: &'db dyn ParseDb) -> SnippetRef {
        SnippetRef::new(
            self.source(db),
            self.index(db),
            SnippetOutput {
                parse: self.parse(db).clone(),
                element_range: self.element_range(db),
                content_range: self.content_range(db),
                content_offset: self.content_offset(db),
                document_source_index: self.document_source_index(db),
            },
        )
    }

    pub fn serde_diagnostics(&self, db: &dyn ParseDb) -> Vec<biome_diagnostics::serde::Diagnostic> {
        self.parsed(db).clone().into_serde_diagnostics()
    }

    pub fn has_errors(&self, db: &dyn ParseDb) -> bool {
        self.parsed(db).has_errors()
    }

    /// Returns the [SnippetOutput] of this snippet from the parse of its
    /// document.
    fn output(self, db: &'db dyn ParseDb) -> &'db SnippetOutput {
        // `tracked_snippets` creates one snippet per entry of this list, and
        // `parse_source` returns the same list until something it read
        // changes, so the index is in range.
        &tracked_source(db, self.source(db)).snippets[self.index(db)]
    }
}
