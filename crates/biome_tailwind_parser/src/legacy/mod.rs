//! The reference implementation of the parser, which only tests compile.
//!
//! [lexer::TailwindLexer] lexes one token at a time in the context that the parser asks for,
//! [token_source::LegacyTokenSource] buffers its tokens, and `syntax` is the grammar that drives
//! them. The tests check that the parser gives the same trees and diagnostics as this one.

pub(crate) mod lexer;
mod parser;
mod syntax;
pub(crate) mod token_source;

use crate::TailwindParse;
use crate::lexer::BaseNameStore;
use crate::parser::TailwindLosslessTreeSink;
use biome_rowan::NodeCache;
use parser::LegacyParser;
use token_source::LegacyTokenSource;

/// Parses `source` with the reference implementation, matching dashed basenames against
/// `base_names`.
pub(crate) fn parse(source: &str, base_names: &BaseNameStore) -> TailwindParse {
    let mut parser = LegacyParser::new(LegacyTokenSource::from_str(source, base_names));
    syntax::parse_root(&mut parser);

    let (events, diagnostics, trivia) = parser.finish();

    let mut cache = NodeCache::default();
    let mut tree_sink = TailwindLosslessTreeSink::with_cache(source, &trivia, &mut cache);
    biome_parser::event::process(&mut tree_sink, events, diagnostics);
    let (green, diagnostics) = tree_sink.finish();

    TailwindParse::new(green, diagnostics)
}
