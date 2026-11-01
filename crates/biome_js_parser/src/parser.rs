//! The physical parser structure.
//! This may not hold your expectations of a traditional parser,
//! the parser yields events like `Start node`, `Error`, etc.
//! These events are then applied to a `TreeSink`.

use biome_parser::*;
pub(crate) mod rewrite_parser;
pub(crate) mod single_token_parse_recovery;

pub(crate) use crate::bulk_lexer::{BulkTokenSource, BulkTokenSourceCheckpoint};
use crate::lexer::JsReLexContext;
pub(crate) use crate::parser::parse_recovery::{
    ParseRecoveryTokenSet, RecoveryError, RecoveryResult,
};
use crate::prelude::*;
use crate::state::JsParserStateCheckpoint;
use crate::state::{ChangeParserState, ParserStateGuard};
use crate::*;
use biome_js_syntax::JsSyntaxKind::{self, JS_REGEX_LITERAL};
use biome_js_syntax::T;
use biome_languages::JsFileSource;
use biome_parser::diagnostic::merge_diagnostics;
use biome_parser::event::Event;
use biome_parser::token_source::Trivia;
use biome_parser::{ParserContext, ParserContextCheckpoint};
use biome_rowan::TextRange;
pub(crate) use parsed_syntax::ParsedSyntax;

/// An extremely fast, error tolerant, completely lossless JavaScript parser
///
/// The Parser yields lower level events instead of nodes.
/// These events are then processed into a syntax tree through a [`TreeSink`] implementation.
pub struct JsParser<'source> {
    pub(super) state: JsParserState,
    pub source_type: JsFileSource,
    context: ParserContext<JsSyntaxKind>,
    source: BulkTokenSource<'source>,
    options: JsParserOptions,
}

impl<'source> JsParser<'source> {
    /// Creates a new parser that parses the `source`.
    pub fn new(source: &'source str, source_type: JsFileSource, options: JsParserOptions) -> Self {
        Self::new_with_metavariables(source, source_type, options, &[])
    }

    /// Creates a new parser that parses the `source`, lexing the given source
    /// ranges as Grit metavariables.
    pub fn new_with_metavariables(
        source: &'source str,
        source_type: JsFileSource,
        options: JsParserOptions,
        metavariables: &'source [TextRange],
    ) -> Self {
        let source = BulkTokenSource::new(source, &source_type, metavariables);

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

    /// Re-lexes the current token where a binary operator goes: a `>` as the operator that
    /// starts with it, such as `>>` or `>=`, and a regex as the `/` or `/=` that starts it.
    /// Returns the current token's kind.
    pub(crate) fn re_lex_binary_operator(&mut self) -> JsSyntaxKind {
        match self.cur() {
            T![>] | JS_REGEX_LITERAL => self.re_lex(JsReLexContext::BinaryOperator),
            kind => kind,
        }
    }

    /// Re-lexes a regex that the bulk lexer decided on as the `/` or `/=` that starts it, where the
    /// grammar doesn't allow a regex. Returns the current token's kind.
    pub(crate) fn re_lex_regex_as_slash(&mut self) -> JsSyntaxKind {
        if self.at(JS_REGEX_LITERAL) {
            self.re_lex(JsReLexContext::BinaryOperator)
        } else {
            self.cur()
        }
    }

    /// Re-lexes a `<<` as the `<` that opens type arguments, and returns the current token's
    /// kind. No other token re-lexes differently there, so this re-lexes nothing else.
    pub(crate) fn re_lex_type_argument_less_than(&mut self) -> JsSyntaxKind {
        if self.at(T![<<]) {
            self.re_lex(JsReLexContext::TypeArgumentLessThan)
        } else {
            self.cur()
        }
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
    type Source = BulkTokenSource<'source>;

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

    /// Consumes the current token if it's `kind`. A regex that the bulk lexer decided on is the
    /// `/` or `/=` that the grammar asks for here.
    #[inline]
    fn eat(&mut self, kind: JsSyntaxKind) -> bool {
        if matches!(kind, T![/] | T![/=]) {
            self.re_lex_regex_as_slash();
        }
        if !self.at(kind) {
            return false;
        }
        self.do_bump(kind);
        true
    }

    /// Bumps the current token regardless of its kind, as error recovery does.
    ///
    /// The literal expression is the only syntax that takes a regex that the bulk lexer decided
    /// on, so anywhere else the regex is the `/` or `/=` that starts it.
    fn bump_any(&mut self) {
        let kind = self.re_lex_regex_as_slash();
        assert_ne!(kind, JsSyntaxKind::EOF);
        self.do_bump(kind);
    }

    /// Bumps the current token regardless of its kind using the new context, re-lexing a regex as
    /// [Self::bump_any] does.
    fn bump_any_with_context(&mut self, context: <Self::Source as BumpWithContext>::Context) {
        let kind = self.re_lex_regex_as_slash();
        assert_ne!(kind, JsSyntaxKind::EOF);
        self.do_bump_with_context(kind, context);
    }
}

pub struct JsParserCheckpoint {
    pub(super) context: ParserContextCheckpoint,
    pub(super) source: BulkTokenSourceCheckpoint,
    state: JsParserStateCheckpoint,
}

#[cfg(test)]
mod tests {
    use crate::JsParserOptions;
    use crate::prelude::*;
    use biome_js_syntax::JsSyntaxKind;
    use biome_languages::JsFileSource;

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
