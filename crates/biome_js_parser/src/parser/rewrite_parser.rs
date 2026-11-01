use crate::parser::{BulkTokenSourceCheckpoint, JsParser};

use crate::prelude::*;
use biome_console::fmt::Display;
use biome_diagnostics::location::AsSpan;
use biome_js_syntax::{JsSyntaxKind, TextRange};
use biome_parser::{
    CompletedMarker, Marker,
    diagnostic::{ParseDiagnostic, ToDiagnostic},
    event::Event,
};
use biome_rowan::TextSize;

/// Simplified parser API for when rewriting the AST structure with `rewrite_events`.
///
/// The difference from the regular [Parser] is that the [TokenSource] must be detached during
/// rewriting to avoid lexing previously lexed tokens in a different context. For example for `a[`test`] = "b"`.
/// Template literal elements get lexed in the [TemplateElement] context. However, if the rewriter
/// rewinds the token source then all tokens are lexed in the [LexContext::Regular] which yields
/// complete different results.
///
/// This is why the [RewriteParser] tracks the source offset without relying on the `TokenSource`
/// and explicitly passes the positions to [Marker] and [CompletedMarker]. This further has the
/// benefit that rewriting the events doesn't require re-lexing all tokens as well.
pub(crate) struct RewriteParser<'parser, 'source> {
    /// The byte offset of the current token from the start of the source
    offset: TextSize,

    inner: &'parser mut JsParser<'source>,

    /// Index of the first token of the token source that the rewriter hasn't passed yet.
    token: usize,
}

impl<'parser, 'source> RewriteParser<'parser, 'source> {
    pub fn new(p: &'parser mut JsParser<'source>, checkpoint: BulkTokenSourceCheckpoint) -> Self {
        Self {
            inner: p,
            offset: checkpoint.current_start(),
            token: checkpoint.token_index(),
        }
    }

    /// Starts a marker for a new node.
    pub fn start(&mut self) -> RewriteMarker {
        let pos = self.inner.context().events().len() as u32;
        self.skip_trivia();
        self.inner.context_mut().push_event(Event::tombstone());
        RewriteMarker(Marker::new(pos, self.offset))
    }

    /// Bumps the passed in token
    pub fn bump(&mut self, token: RewriteToken) {
        self.skip_trivia();
        debug_assert!(self.offset < token.end);
        self.inner.context_mut().push_token(token.kind, token.end);

        // test ts ts_decorator_assignment
        // @test(--a)
        // class Test {}

        self.offset = token.end;
    }

    /// Moves `offset` past the trivia, and the tokens skipped as trivia, to the start of the next
    /// token.
    fn skip_trivia(&mut self) {
        let source = self.inner.source();
        while self.token < source.token_count() {
            let range = source.token_range(self.token);
            if range.end() <= self.offset || source.is_skipped(self.token) {
                self.token += 1;
                continue;
            }
            self.offset = self.offset.max(range.start());
            break;
        }
    }

    /// Finishes the rewriter
    ///
    /// ## Panics
    /// If not all tokens have been consumed or if they have been consumed out of order
    pub fn finish(mut self) {
        self.skip_trivia();
        assert_eq!(
            self.offset,
            self.inner.source().position(),
            "Rewrite didn't consume all tokens"
        );
    }

    /// Returns true if the parser is in strict mode
    pub fn is_strict_mode(&self) -> bool {
        self.inner.state().strict().is_some()
    }

    pub fn err_builder(&self, message: impl Display, span: impl AsSpan) -> ParseDiagnostic {
        self.inner.err_builder(message, span)
    }

    pub fn error(&mut self, diagnostic: impl ToDiagnostic<JsParser<'source>>) {
        self.inner.error(diagnostic)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct RewriteToken {
    pub(crate) kind: JsSyntaxKind,
    end: TextSize,
}

impl RewriteToken {
    pub fn new(kind: JsSyntaxKind, end: TextSize) -> Self {
        Self { kind, end }
    }
}

#[derive(Debug)]
pub(crate) struct RewriteMarker(Marker);

impl RewriteMarker {
    /// Completes the node with the specified kind
    pub fn complete(self, p: &mut RewriteParser, kind: JsSyntaxKind) -> RewriteCompletedMarker {
        RewriteCompletedMarker(self.0.complete(p.inner, kind))
    }
}

#[derive(Debug)]
pub(crate) struct RewriteCompletedMarker(CompletedMarker);

impl RewriteCompletedMarker {
    /// Returns the range of the marker
    pub fn range(&self, p: &RewriteParser) -> TextRange {
        self.0.range(p.inner)
    }

    /// Returns the source text of the marker
    pub fn text<'a>(&self, p: &'a RewriteParser) -> &'a str {
        self.0.text(p.inner)
    }

    pub fn change_to_bogus(&mut self, p: &mut RewriteParser) {
        self.0.change_to_bogus(p.inner)
    }
}

impl From<RewriteCompletedMarker> for CompletedMarker {
    fn from(inner: RewriteCompletedMarker) -> Self {
        inner.0
    }
}
