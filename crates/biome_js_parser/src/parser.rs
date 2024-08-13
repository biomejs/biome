//! The physical parser structure.
//! This may not hold your expectations of a traditional parser,
//! the parser yields events like `Start node`, `Error`, etc.
//! These events are then applied to a `TreeSink`.

use biome_parser::*;
pub(crate) mod rewrite_parser;
pub(crate) mod single_token_parse_recovery;

use crate::lexer::JsReLexContext;
pub(crate) use crate::parser::parse_recovery::{
    ParseRecoveryTokenSet, RecoveryError, RecoveryResult,
};
use crate::prelude::*;
use crate::state::{ChangeParserState, ParserStateGuard};
use crate::token_source::JsTokenSourceCheckpoint;
use crate::*;
use crate::{state::JsParserStateCheckpoint, token_source::JsTokenSource};
use biome_js_syntax::{
    JsFileSource,
    JsSyntaxKind::{self},
};
use biome_parser::diagnostic::merge_diagnostics;
use biome_parser::event::Event;
use biome_parser::token_source::Trivia;
use biome_parser::{ParserContext, ParserContextCheckpoint};
pub(crate) use parsed_syntax::ParsedSyntax;

/// An extremely fast, error tolerant, completely lossless JavaScript parser
///
/// The Parser yields lower level events instead of nodes.
/// These events are then processed into a syntax tree through a [`TreeSink`] implementation.
pub struct JsParser<'source> {
    pub(super) state: JsParserState,
    pub source_type: JsFileSource,
    context: ParserContext<JsSyntaxKind>,
    source: JsTokenSource<'source>,
    options: JsParserOptions,
}

impl<'source> JsParser<'source> {
    /// Creates a new parser that parses the `source`.
    pub fn new(source: &'source str, source_type: JsFileSource, options: JsParserOptions) -> Self {
        let source = JsTokenSource::from_str(source, options.clone());

        JsParser {
            state: JsParserState::new(&source_type),
            source_type,
            context: ParserContext::default(),
            source,
            options,
        }
    }

    pub(crate) fn state(&self) -> &JsParserState {
        &self.state
    }

    pub(crate) fn options(&self) -> &JsParserOptions {
        &self.options
    }

    pub(crate) fn state_mut(&mut self) -> &mut JsParserState {
        &mut self.state
    }

    pub fn source_type(&self) -> JsFileSource {
        self.source_type
    }

    /// Whether the code we are parsing is a module
    pub const fn is_module(&self) -> bool {
        self.source_type.module_kind().is_module()
    }

    /// Re-lexes the current token in the specified context. Returns the kind
    /// of the re-lexed token (can be the same as before if the context doesn't make a difference for the current token)
    pub fn re_lex(&mut self, context: JsReLexContext) -> JsSyntaxKind {
        self.source_mut().re_lex(context)
    }

    /// Stores the parser state and position before calling the function and restores the state
    /// and position before returning.
    ///
    /// Useful in situation where the parser must advance a few tokens to determine whatever a syntax is
    /// of one or the other kind.
    #[inline]
    pub fn lookahead<F, R>(&mut self, op: F) -> R
    where
        F: FnOnce(&mut JsParser) -> R,
    {
        let checkpoint = self.checkpoint();
        let result = op(self);
        self.rewind(checkpoint);

        result
    }

    /// Applies the passed in change to the parser's state and reverts the
    /// changes when the returned [ParserStateGuard] goes out of scope.
    pub(crate) fn with_scoped_state<'p, C: ChangeParserState>(
        &'p mut self,
        change: C,
    ) -> ParserStateGuard<'p, 'source, C> {
        let snapshot = change.apply(self.state_mut());
        ParserStateGuard::new(self, snapshot)
    }

    /// Applies the passed in change to the parser state before applying the passed `func` and
    /// restores the state to before the change before returning the result.
    #[inline]
    pub(crate) fn with_state<C, F, R>(&mut self, change: C, func: F) -> R
    where
        C: ChangeParserState,
        F: FnOnce(&mut JsParser) -> R,
    {
        let snapshot = change.apply(self.state_mut());
        let result = func(self);
        C::restore(self.state_mut(), snapshot);
        result
    }

    pub fn checkpoint(&self) -> JsParserCheckpoint {
        JsParserCheckpoint {
            context: self.context.checkpoint(),
            source: self.source.checkpoint(),
            state: self.state.checkpoint(),
        }
    }

    pub fn rewind(&mut self, checkpoint: JsParserCheckpoint) {
        let JsParserCheckpoint {
            context,
            source,
            state,
        } = checkpoint;

        self.context.rewind(context);
        self.source.rewind(source);
        self.state.restore(state);
    }

    pub fn finish(self) -> (Vec<Event<JsSyntaxKind>>, Vec<Trivia>, Vec<ParseDiagnostic>) {
        let (trivia, source_diagnostics) = self.source.finish();
        let (events, parse_diagnostics) = self.context.finish();

        let diagnostics = merge_diagnostics(source_diagnostics, parse_diagnostics);

        (events, trivia, diagnostics)
    }
}

impl<'source> Parser for JsParser<'source> {
    type Kind = JsSyntaxKind;
    type Source = JsTokenSource<'source>;

    fn context(&self) -> &ParserContext<Self::Kind> {
        &self.context
    }

    fn context_mut(&mut self) -> &mut ParserContext<Self::Kind> {
        &mut self.context
    }

    fn source(&self) -> &Self::Source {
        &self.source
    }

    fn source_mut(&mut self) -> &mut Self::Source {
        &mut self.source
    }

    fn is_speculative_parsing(&self) -> bool {
        self.state.speculative_parsing
    }

    fn do_bump_with_context(
        &mut self,
        kind: Self::Kind,
        context: <Self::Source as BumpWithContext>::Context,
    ) where
        Self::Source: BumpWithContext,
    {
        let kind = if kind.is_keyword() && self.source().has_unicode_escape() {
            self.error(self.err_builder(
                format!(
                    "'{}' keyword cannot contain escape character.",
                    kind.to_string().expect("to return a value for a keyword")
                ),
                self.cur_range(),
            ));
            JsSyntaxKind::ERROR_TOKEN
        } else {
            kind
        };

        let end = self.cur_range().end();
        self.context_mut().push_token(kind, end);

        if self.context().is_skipping() {
            self.source_mut().skip_as_trivia_with_context(context);
        } else {
            self.source_mut().bump_with_context(context);
        }
    }

    fn do_bump(&mut self, kind: Self::Kind) {
        self.do_bump_with_context(kind, JsLexContext::Regular)
    }
}

pub struct JsParserCheckpoint {
    pub(super) context: ParserContextCheckpoint,
    pub(super) source: JsTokenSourceCheckpoint,
    state: JsParserStateCheckpoint,
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::JsParserOptions;
    use biome_js_syntax::{JsFileSource, JsSyntaxKind};

    #[test]
    #[should_panic(
        expected = "Marker must either be `completed` or `abandoned` to avoid that children are implicitly attached to a marker's parent."
    )]
    fn uncompleted_markers_panic() {
        let mut parser = JsParser::new(
            "'use strict'",
            JsFileSource::default(),
            JsParserOptions::default(),
        );

        let _ = parser.start();
        // drop the marker without calling complete or abandon
    }

    #[test]
    fn completed_marker_doesnt_panic() {
        let mut p = JsParser::new(
            "'use strict'",
            JsFileSource::default(),
            JsParserOptions::default(),
        );

        let m = p.start();
        p.expect(JsSyntaxKind::JS_STRING_LITERAL);
        m.complete(&mut p, JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION);
    }

    #[test]
    fn abandoned_marker_doesnt_panic() {
        let mut p = JsParser::new(
            "'use strict'",
            JsFileSource::default(),
            JsParserOptions::default(),
        );

        let m = p.start();
        m.abandon(&mut p);
    }
}
