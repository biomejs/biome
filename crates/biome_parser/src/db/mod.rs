//! Parsing on top of the Salsa database of [biome_db].
//!
//! A [Source] holds the text of a file and the file source it's parsed as,
//! never a parse result. A database that implements [ParseDb] knows how to
//! parse it, and a tracked query derives the parse from the text the first
//! time it's read:
//!
//! ```text
//! Source ──tracked_source───▶ SourceOutput { ParseHandle, Vec<SnippetOutput> }
//!          └ db.parse_source(source)
//! Source ──tracked_snippets─▶ Vec<TrackedSnippet>   (one per SnippetOutput)
//! TrackedSnippet ──tracked_snippet_parse─▶ ParseHandle
//!
//! analyzers: AnalyzedSource = Source | SnippetRef   (SnippetRef::resolve(db) → TrackedSnippet)
//! ```
//!
//! The parse cache has a size limit. It keeps the parses read most recently
//! and evicts the others, so the syntax trees of unused files don't stay in
//! memory. Reading an evicted parse parses the text again. Semantic models and
//! the other results that hold a syntax tree have the same limit.
//!
//! The types of this module are of three kinds, one per file:
//!
//! - `output`: the values that a parser returns. [SourceOutput] holds the
//!   parse of a source and its [SnippetOutput]s. [ParseHandle] compares parses
//!   by allocation instead of walking their trees.
//! - `tracked`: the queries that parse a source and create its
//!   [TrackedSnippet]s. [SourceExt] reads them. A tracked snippet borrows the
//!   database. Parsing a source again creates its snippets again in the same
//!   order, so each snippet keeps its identity. A query keyed by a snippet
//!   runs again only when the snippet fields it reads change.
//! - `handles`: the values that analyzers hold, which outlive the borrow of
//!   the database: [SnippetRef] and [AnalyzedSource].

mod handles;
mod output;
mod tracked;

pub use handles::{AnalyzedSource, SnippetRef};
pub use output::{ParseHandle, SnippetOutput, SourceOutput};
pub use tracked::{SourceExt, TrackedSnippet};

use biome_db::{Db, Source};

/// A database that parses the texts of its [Source] inputs.
#[salsa::db]
pub trait ParseDb: Db {
    /// Parses the text of `source` as its file source, along with the snippets
    /// embedded in it.
    ///
    /// Only the tracked parse of `source` calls it, so Salsa tracks what it
    /// reads from the database. A change to anything it read, such as the
    /// settings of the file, parses the text again.
    ///
    /// Until something it read changes, it must return the same snippets in
    /// the same order: a [TrackedSnippet] finds its output by its position in
    /// the list.
    fn parse_source(&self, source: Source) -> SourceOutput;
}
