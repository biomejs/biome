//! Events emitted by the Parser which are then constructed into a syntax tree

use crate::parser::rewrite_parser::{RewriteParser, RewriteToken};
use crate::parser::JsParserCheckpoint;
use crate::JsParser;
use biome_js_syntax::JsSyntaxKind;
use biome_parser::tree_sink::TreeSink;
use biome_parser::{event::process, prelude::*};
use biome_rowan::TextSize;

struct RewriteParseEventsTreeSink<'r, 'p, T> {
    reparse: &'r mut T,
    parser: RewriteParser<'r, 'p>,
}

impl<T: RewriteParseEvents> TreeSink for RewriteParseEventsTreeSink<'_, '_, T> {
    type Kind = JsSyntaxKind;

    fn token(&mut self, kind: JsSyntaxKind, end: TextSize) {
        self.reparse
            .token(RewriteToken::new(kind, end), &mut self.parser);
    }

    fn start_node(&mut self, kind: JsSyntaxKind) {
        self.reparse.start_node(kind, &mut self.parser);
    }

    fn finish_node(&mut self) {
        self.reparse.finish_node(&mut self.parser);
    }

    fn errors(&mut self, _errors: Vec<ParseDiagnostic>) {}
}

/// Implement this trait if you want to change the tree structure
/// from already parsed events.
pub(crate) trait RewriteParseEvents {
    /// Called for a started node in the original tree
    fn start_node(&mut self, kind: JsSyntaxKind, p: &mut RewriteParser);

    /// Called for a finished node in the original tree
    fn finish_node(&mut self, p: &mut RewriteParser);

    /// Called for every token
    fn token(&mut self, token: RewriteToken, p: &mut RewriteParser) {
        p.bump(token)
    }
}

/// Allows rewriting a super grammar to a sub grammar by visiting each event emitted after the checkpoint.
/// Useful if a node turned out to be of a different kind its subtree must be re-shaped
/// (adding new nodes, dropping sub nodes, etc.).
pub(crate) fn rewrite_events<T: RewriteParseEvents>(
    rewriter: &mut T,
    checkpoint: JsParserCheckpoint,
    p: &mut JsParser,
) {
    // Only rewind the events but do not reset the parser errors nor parser state.
    // The current parsed grammar is a super-set of the grammar that gets re-parsed. Thus, any
    // error that applied to the old grammar also applies to the sub-grammar.
    let events: Vec<_> = unsafe {
        p.context_mut()
            .split_off_events(checkpoint.context.event_position() + 1)
    };

    let mut sink = RewriteParseEventsTreeSink {
        parser: RewriteParser::new(p, checkpoint.source),
        reparse: rewriter,
    };
    process(&mut sink, events, Vec::default());
    sink.parser.finish();
}
