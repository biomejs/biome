//! What Salsa tracks: the parse of a source and the snippets embedded in it,
//! which borrow the database.

use super::{ParseDb, ParseHandle, SnippetOutput, SnippetRef, SourceOutput};
use crate::AnyParse;
use crate::diagnostic::ParseDiagnostic;
use biome_db::Source;
use biome_diagnostics::{Diagnostic, Severity};
use biome_rowan::{TextRange, TextSize};

/// The parse of a source, as [tracked_source] derives it.
#[salsa::tracked]
#[derive(Debug)]
struct TrackedSource<'db> {
    #[tracked]
    #[returns(ref)]
    parse: ParseHandle,

    #[tracked]
    #[returns(copy)]
    document_source_index: usize,

    #[tracked]
    #[returns(ref)]
    snippets: Vec<TrackedSnippet<'db>>,
}

/// Parses the text of `source`.
///
/// The snippets are created again in the same order every time the text is
/// parsed, so the snippet at an index keeps its identity as long as the
/// snippets before it keep their file sources.
#[salsa::tracked(returns(copy))]
fn tracked_source(db: &dyn ParseDb, source: Source) -> TrackedSource<'_> {
    let SourceOutput {
        parse,
        document_source_index,
        snippets,
    } = db.parse_source(source);
    let snippets = snippets
        .into_iter()
        .enumerate()
        .map(|(index, snippet)| {
            TrackedSnippet::new(
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
    TrackedSource::new(db, parse, document_source_index, snippets)
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
        tracked_source(db, self).parse(db)
    }

    fn document_source_index(self, db: &dyn ParseDb) -> usize {
        tracked_source(db, self).document_source_index(db)
    }

    fn snippets(self, db: &dyn ParseDb) -> &Vec<TrackedSnippet<'_>> {
        tracked_source(db, self).snippets(db)
    }
}

/// A snippet embedded in a document, such as the content of a `<script>`
/// element, as Salsa tracks it.
///
/// Queries can be keyed by a snippet, and the snippet keeps its identity when
/// its document is parsed again. It borrows the database: use [SnippetRef] to
/// refer to it from data that outlives the borrow, such as the services of an
/// analyzer.
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

    #[tracked]
    #[returns(ref)]
    pub(super) parse: ParseHandle,

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

    pub fn to_ref(self, db: &dyn ParseDb) -> SnippetRef {
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
}
