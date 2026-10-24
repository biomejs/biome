//! Parsing on top of the Salsa database of [biome_db].
//!
//! A [Source] holds the text of a file and the file source it's parsed as,
//! never a parse result. A database that implements [ParseDb] knows how to
//! parse it, and a tracked query derives the parse from the text the first
//! time it's read:
//!
//! ```text
//! Source ──tracked_source──▶ TrackedSource { ParseHandle, Vec<TrackedSnippet> }
//!          └ db.parse_source(source) → SourceOutput { ParseHandle, Vec<SnippetOutput> }
//!
//! analyzers: AnalyzedSource = Source | SnippetRef   (SnippetRef::resolve(db) → TrackedSnippet)
//! ```
//!
//! The types of this module are of three kinds, one per file:
//!
//! - `output`: the values that a parser returns, which Salsa doesn't track:
//!   [SourceOutput], [SnippetOutput], and [ParseHandle], which compares parses
//!   by allocation instead of walking their trees.
//! - `tracked`: the values that Salsa tracks, which borrow the database: the
//!   parse of a source, which [SourceExt] reads, and its [TrackedSnippet]s.
//!   Parsing a source again creates its snippets again in the same order, so
//!   they keep their identities. A query keyed by a snippet runs again only
//!   when the snippet fields it reads change.
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
    fn parse_source(&self, source: Source) -> SourceOutput;
}
