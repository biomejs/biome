#![deny(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../CONTRIBUTING.md")]

use crate::diagnostic::{expected_token, ParseDiagnostic, ToDiagnostic};
use crate::event::Event;
use crate::event::Event::Token;
use crate::token_source::{BumpWithContext, NthToken, TokenSource, TokenSourceWithBufferedLexer};
use biome_console::fmt::Display;
use biome_diagnostics::location::AsSpan;
use biome_rowan::{AstNode, Language, SendNode, SyntaxKind, SyntaxNode, TextRange, TextSize};
use std::any::type_name;

pub mod diagnostic;
pub mod event;
pub mod lexer;
mod marker;
pub mod parse_lists;
pub mod parse_recovery;
pub mod parsed_syntax;
pub mod prelude;
pub mod token_set;
pub mod token_source;
pub mod tree_sink;

use crate::lexer::LexerWithCheckpoint;
use crate::parsed_syntax::ParsedSyntax;
use crate::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_diagnostics::serde::Diagnostic;
pub use marker::{CompletedMarker, Marker};
pub use token_set::TokenSet;

pub struct ParserContext<K: SyntaxKind> {
    events: Vec<Event<K>>,
    skipping: bool,
    diagnostics: Vec<ParseDiagnostic>,
}

impl<K: SyntaxKind> Default for ParserContext<K> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: SyntaxKind> ParserContext<K> {
    pub fn new() -> Self {
        Self {
            skipping: false,
            events: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    /// Returns the slice with the parse events
    pub fn events(&self) -> &[Event<K>] {
        &self.events
    }

    /// Returns a slice with the parse diagnostics
    pub fn diagnostics(&self) -> &[ParseDiagnostic] {
        &self.diagnostics
    }

    /// Drops all diagnostics after `at`.
    pub fn truncate_diagnostics(&mut self, at: usize) {
        self.diagnostics.truncate(at);
    }

    /// Pushes a new token event
    pub fn push_token(&mut self, kind: K, end: TextSize) {
        self.push_event(Token { kind, end });
    }

    /// Pushes a parse event
    pub fn push_event(&mut self, event: Event<K>) {
        self.events.push(event)
    }

    /// Returns `true` if the parser is skipping a token as skipped token trivia.
    pub fn is_skipping(&self) -> bool {
        self.skipping
    }

    /// Splits the events into two at the given `position`. Returns a newly allocated vector containing the
    /// elements from `[position;len]`.
    ///
    /// ## Safety
    /// The method is marked as `unsafe` to discourage its usage. Removing events can lead to
    /// corrupted events if not done carefully.
    pub unsafe fn split_off_events(&mut self, position: usize) -> Vec<Event<K>> {
        self.events.split_off(position)
    }

    /// Get the current index of the last event
    fn cur_event_pos(&self) -> usize {
        self.events.len().saturating_sub(1)
    }

    /// Remove `amount` events from the parser
    fn drain_events(&mut self, amount: usize) {
        self.events.truncate(self.events.len() - amount);
    }

    /// Rewind the parser back to a previous position in time
    pub fn rewind(&mut self, checkpoint: ParserContextCheckpoint) {
        let ParserContextCheckpoint {
            event_pos,
            errors_pos,
        } = checkpoint;
        self.drain_events(self.cur_event_pos() - event_pos);
        self.diagnostics.truncate(errors_pos as usize);
    }

    /// Get a checkpoint representing the progress of the parser at this point of time
    #[must_use]
    pub fn checkpoint(&self) -> ParserContextCheckpoint {
        ParserContextCheckpoint {
            event_pos: self.cur_event_pos(),
            errors_pos: self.diagnostics.len() as u32,
        }
    }

    pub fn finish(self) -> (Vec<Event<K>>, Vec<ParseDiagnostic>) {
        (self.events, self.diagnostics)
    }
}

/// A structure signifying the Parser progress at one point in time
#[derive(Debug)]
pub struct ParserContextCheckpoint {
    event_pos: usize,
    /// The length of the errors list at the time the checkpoint was created.
    /// Safety: The parser only supports files <= 4Gb. Storing a `u32` is sufficient to store one error
    /// for each single character in the file, which should be sufficient for any realistic file.
    errors_pos: u32,
}

impl ParserContextCheckpoint {
    pub fn event_position(&self) -> usize {
        self.event_pos
    }
}

pub trait Parser: Sized {
    type Kind: SyntaxKind;
    type Source: TokenSource<Kind = Self::Kind>;

    /// Returns a reference to the [ParserContext].
    fn context(&self) -> &ParserContext<Self::Kind>;

    /// Returns a mutable reference to the [ParserContext].
    fn context_mut(&mut self) -> &mut ParserContext<Self::Kind>;

    /// Returns a reference to the [`TokenSource``](TokenSource]
    fn source(&self) -> &Self::Source;

    /// Returns a mutable reference to the [TokenSource].
    fn source_mut(&mut self) -> &mut Self::Source;

    /// Returns `true` if the parser is trying to parse some syntax but only if it has no errors.
    ///
    /// Returning `true` disables more involved error recovery.
    fn is_speculative_parsing(&self) -> bool {
        false
    }

    /// Gets the source text of a range
    ///
    /// # Panics
    ///
    /// If the range is out of bounds
    fn text(&self, span: TextRange) -> &str {
        &self.source().text()[span]
    }

    /// Gets the current token kind of the parser
    fn cur(&self) -> Self::Kind {
        self.source().current()
    }

    /// Gets the range of the current token
    fn cur_range(&self) -> TextRange {
        self.source().current_range()
    }

    /// Tests if there's a line break before the current token (between the last and current)
    fn has_preceding_line_break(&self) -> bool {
        self.source().has_preceding_line_break()
    }

    /// Get the source code of the parser's current token.
    fn cur_text(&self) -> &str {
        &self.source().text()[self.cur_range()]
    }

    /// Checks if the parser is currently at a specific token
    fn at(&self, kind: Self::Kind) -> bool {
        self.cur() == kind
    }

    /// Check if the parser's current token is contained in a token set
    fn at_ts(&self, kinds: TokenSet<Self::Kind>) -> bool {
        kinds.contains(self.cur())
    }

    /// Look ahead at a token and get its kind.
    fn nth<'l, Lex>(&mut self, n: usize) -> Self::Kind
    where
        Lex: LexerWithCheckpoint<'l, Kind = Self::Kind>,
        Self::Source: NthToken<Lex> + TokenSourceWithBufferedLexer<Lex>,
    {
        self.source_mut().nth(n)
    }

    /// Checks if a token lookahead is something
    fn nth_at<'l, Lex>(&mut self, n: usize, kind: Self::Kind) -> bool
    where
        Lex: LexerWithCheckpoint<'l, Kind = Self::Kind>,
        Self::Source: NthToken<Lex> + TokenSourceWithBufferedLexer<Lex>,
    {
        self.nth(n) == kind
    }

    /// Checks if a token set lookahead is something
    fn nth_at_ts<'l, Lex>(&mut self, n: usize, kinds: TokenSet<Self::Kind>) -> bool
    where
        Lex: LexerWithCheckpoint<'l, Kind = Self::Kind>,
        Self::Source: NthToken<Lex> + TokenSourceWithBufferedLexer<Lex>,
    {
        kinds.contains(self.nth(n))
    }

    /// Tests if there's a line break before the nth token.
    #[inline]
    fn has_nth_preceding_line_break<'l, Lex>(&mut self, n: usize) -> bool
    where
        Lex: LexerWithCheckpoint<'l, Kind = Self::Kind>,
        Self::Source: NthToken<Lex> + TokenSourceWithBufferedLexer<Lex>,
    {
        self.source_mut().has_nth_preceding_line_break(n)
    }

    /// Consume the current token if `kind` matches.
    fn bump(&mut self, kind: Self::Kind) {
        assert_eq!(
            kind,
            self.cur(),
            "expected {:?} but at {:?}",
            kind,
            self.cur()
        );

        self.do_bump(kind)
    }

    /// Consume the current token if token set matches.
    fn bump_ts(&mut self, kinds: TokenSet<Self::Kind>) {
        assert!(
            kinds.contains(self.cur()),
            "expected {:?} but at {:?}",
            kinds,
            self.cur()
        );

        self.bump_any()
    }

    /// Consume any token but cast it as a different kind using the specified `context.
    fn bump_remap_with_context(
        &mut self,
        kind: Self::Kind,
        context: <Self::Source as BumpWithContext>::Context,
    ) where
        Self::Source: BumpWithContext,
    {
        self.do_bump_with_context(kind, context);
    }

    /// Consume any token but cast it as a different kind
    fn bump_remap(&mut self, kind: Self::Kind) {
        self.do_bump(kind);
    }

    /// Bumps the current token regardless of its kind and advances to the next token.
    fn bump_any(&mut self) {
        let kind = self.cur();
        assert_ne!(kind, Self::Kind::EOF);

        self.do_bump(kind);
    }

    /// Consumes the current token if `kind` matches and lexes the next token using the
    /// specified `context.
    fn bump_with_context(
        &mut self,
        kind: Self::Kind,
        context: <Self::Source as BumpWithContext>::Context,
    ) where
        Self::Source: BumpWithContext,
    {
        assert_eq!(
            kind,
            self.cur(),
            "expected {:?} but at {:?}",
            kind,
            self.cur()
        );

        self.do_bump_with_context(kind, context);
    }

    #[doc(hidden)]
    fn do_bump_with_context(
        &mut self,
        kind: Self::Kind,
        context: <Self::Source as BumpWithContext>::Context,
    ) where
        Self::Source: BumpWithContext,
    {
        let end = self.cur_range().end();
        self.context_mut().push_token(kind, end);

        if self.context().skipping {
            self.source_mut().skip_as_trivia_with_context(context);
        } else {
            self.source_mut().bump_with_context(context);
        }
    }

    #[doc(hidden)]
    fn do_bump(&mut self, kind: Self::Kind) {
        let end = self.cur_range().end();
        self.context_mut().push_token(kind, end);

        if self.context().skipping {
            self.source_mut().skip_as_trivia();
        } else {
            self.source_mut().bump();
        }
    }

    /// Consume the next token if `kind` matches using the specified `context.
    fn eat_with_context(
        &mut self,
        kind: Self::Kind,
        context: <Self::Source as BumpWithContext>::Context,
    ) -> bool
    where
        Self::Source: BumpWithContext,
    {
        if !self.at(kind) {
            return false;
        }

        self.do_bump_with_context(kind, context);

        true
    }

    /// Consume the next token if `kind` matches.
    fn eat(&mut self, kind: Self::Kind) -> bool {
        if !self.at(kind) {
            return false;
        }

        self.do_bump(kind);

        true
    }

    /// Consume the next token if token set matches.
    fn eat_ts(&mut self, kinds: TokenSet<Self::Kind>) -> bool {
        if !self.at_ts(kinds) {
            return false;
        }

        self.do_bump(self.cur());

        true
    }

    /// Consume the next token if token set matches using the specified `context.
    fn eat_ts_with_context(
        &mut self,
        kinds: TokenSet<Self::Kind>,
        context: <Self::Source as BumpWithContext>::Context,
    ) -> bool
    where
        Self::Source: BumpWithContext,
    {
        if !self.at_ts(kinds) {
            return false;
        }

        self.do_bump_with_context(self.cur(), context);

        true
    }

    /// Try to eat a specific token kind, if the kind is not there then adds an error to the events stack
    /// using the specified `context.
    fn expect_with_context(
        &mut self,
        kind: Self::Kind,
        context: <Self::Source as BumpWithContext>::Context,
    ) -> bool
    where
        Self::Source: BumpWithContext,
    {
        if self.eat_with_context(kind, context) {
            true
        } else {
            self.error(expected_token(kind));
            false
        }
    }

    /// Try to eat a specific token kind, if the kind is not there then adds an error to the events stack.
    fn expect(&mut self, kind: Self::Kind) -> bool {
        if self.eat(kind) {
            true
        } else {
            self.error(expected_token(kind));
            false
        }
    }

    /// Allows parsing an unsupported syntax as skipped trivia tokens.
    fn parse_as_skipped_trivia_tokens<P>(&mut self, parse: P)
    where
        P: FnOnce(&mut Self),
    {
        let events_pos = self.context().events.len();
        self.context_mut().skipping = true;
        parse(self);
        self.context_mut().skipping = false;

        // Truncate any start/finish events
        self.context_mut().events.truncate(events_pos);
    }

    /// Add a diagnostic
    fn error(&mut self, err: impl ToDiagnostic<Self>) {
        let err = err.into_diagnostic(self);

        // Don't report another diagnostic if the last diagnostic is at the same position of the current one
        if let Some(previous) = self.context().diagnostics.last() {
            match (&err.diagnostic_range(), &previous.diagnostic_range()) {
                (Some(err_range), Some(previous_range))
                    if err_range.start() == previous_range.start() =>
                {
                    return;
                }
                _ => {}
            }
        }
        self.context_mut().diagnostics.push(err)
    }

    /// Creates a new diagnostic. Pass the message and the range where the error occurred
    #[must_use]
    fn err_builder(&self, message: impl Display, span: impl AsSpan) -> ParseDiagnostic {
        ParseDiagnostic::new(message, span)
    }

    /// Bump and add an error event
    fn err_and_bump(&mut self, err: impl ToDiagnostic<Self>, unknown_syntax_kind: Self::Kind) {
        let m = self.start();
        self.bump_any();
        m.complete(self, unknown_syntax_kind);
        self.error(err);
    }

    /// Returns the kind of the last bumped token.
    fn last(&self) -> Option<Self::Kind> {
        self.context()
            .events
            .iter()
            .rev()
            .find_map(|event| match event {
                Token { kind, .. } => Some(*kind),
                _ => None,
            })
    }

    /// Returns the end offset of the last bumped token.
    fn last_end(&self) -> Option<TextSize> {
        self.context()
            .events
            .iter()
            .rev()
            .find_map(|event| match event {
                Token { end, .. } => Some(*end),
                _ => None,
            })
    }

    /// Starts a new node in the syntax tree. All nodes and tokens
    /// consumed between the `start` and the corresponding `Marker::complete`
    /// belong to the same node.
    fn start(&mut self) -> Marker {
        let pos = self.context().events.len() as u32;
        let start = self.source().position();
        self.context_mut().push_event(Event::tombstone());
        Marker::new(pos, start)
    }
}

/// Captures the progress of the parser and allows to test if the parsing is still making progress
#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, Hash, Default)]
pub struct ParserProgress(Option<TextSize>);

impl ParserProgress {
    /// Returns true if the current parser position is passed this position
    #[inline]
    pub fn has_progressed<P>(&self, p: &P) -> bool
    where
        P: Parser,
    {
        match self.0 {
            None => true,
            Some(pos) => pos < p.source().position(),
        }
    }

    /// Asserts that the parsing is still making progress.
    ///
    /// # Panics
    ///
    /// Panics if the parser is still at this position
    #[inline]
    pub fn assert_progressing<P>(&mut self, p: &P)
    where
        P: Parser,
    {
        assert!(
            self.has_progressed(p),
            "The parser is no longer progressing. Stuck at '{}' {:?}:{:?}",
            p.cur_text(),
            p.cur(),
            p.cur_range(),
        );

        self.0 = Some(p.source().position());
    }
}

/// A syntax feature that may or may not be supported depending on the file type and parser configuration
pub trait SyntaxFeature: Sized {
    type Parser<'source>: Parser;

    /// Returns `true` if the current parsing context supports this syntax feature.
    fn is_supported(&self, p: &Self::Parser<'_>) -> bool;

    /// Returns `true` if the current parsing context doesn't support this syntax feature.
    fn is_unsupported(&self, p: &Self::Parser<'_>) -> bool {
        !self.is_supported(p)
    }

    /// Adds a diagnostic and changes the kind of the node to [SyntaxKind::to_bogus] if this feature isn't
    /// supported.
    ///
    /// Returns the parsed syntax.
    fn exclusive_syntax<'source, S, E, D>(
        &self,
        p: &mut Self::Parser<'source>,
        syntax: S,
        error_builder: E,
    ) -> ParsedSyntax
    where
        S: Into<ParsedSyntax>,
        E: FnOnce(&Self::Parser<'source>, &CompletedMarker) -> D,
        D: ToDiagnostic<Self::Parser<'source>>,
    {
        syntax.into().map(|mut syntax| {
            if self.is_unsupported(p) {
                let error = error_builder(p, &syntax);
                p.error(error);
                syntax.change_to_bogus(p);
                syntax
            } else {
                syntax
            }
        })
    }

    /// Parses a syntax and adds a diagnostic and changes the kind of the node to [SyntaxKind::to_bogus] if this feature isn't
    /// supported.
    ///
    /// Returns the parsed syntax.
    fn parse_exclusive_syntax<'source, P, E>(
        &self,
        p: &mut Self::Parser<'source>,
        parse: P,
        error_builder: E,
    ) -> ParsedSyntax
    where
        P: FnOnce(&mut Self::Parser<'source>) -> ParsedSyntax,
        E: FnOnce(&Self::Parser<'source>, &CompletedMarker) -> ParseDiagnostic,
    {
        if self.is_supported(p) {
            parse(p)
        } else {
            let diagnostics_checkpoint = p.context().diagnostics().len();
            let syntax = parse(p);
            p.context_mut().truncate_diagnostics(diagnostics_checkpoint);

            match syntax {
                Present(mut syntax) => {
                    let diagnostic = error_builder(p, &syntax);
                    p.error(diagnostic);
                    syntax.change_to_bogus(p);
                    Present(syntax)
                }
                _ => Absent,
            }
        }
    }

    /// Adds a diagnostic and changes the kind of the node to [SyntaxKind::to_bogus] if this feature is
    /// supported.
    ///
    /// Returns the parsed syntax.
    fn excluding_syntax<'source, S, E>(
        &self,
        p: &mut Self::Parser<'source>,
        syntax: S,
        error_builder: E,
    ) -> ParsedSyntax
    where
        S: Into<ParsedSyntax>,
        E: FnOnce(&Self::Parser<'source>, &CompletedMarker) -> ParseDiagnostic,
    {
        syntax.into().map(|mut syntax| {
            if self.is_unsupported(p) {
                syntax
            } else {
                let error = error_builder(p, &syntax);
                p.error(error);
                syntax.change_to_bogus(p);
                syntax
            }
        })
    }
}

/// Language-independent cache entry for a parsed file
///
/// This struct holds a handle to the root node of the parsed syntax tree,
/// along with the list of diagnostics emitted by the parser while generating
/// this entry.
///
/// It can be dynamically downcast into a concrete [SyntaxNode] or [AstNode] of
/// the corresponding language, generally through a language-specific capability
#[derive(Clone, Debug)]
pub struct AnyParse {
    pub(crate) root: SendNode,
    pub(crate) diagnostics: Vec<ParseDiagnostic>,
}

impl From<SendNode> for AnyParse {
    fn from(root: SendNode) -> Self {
        Self {
            root,
            diagnostics: Vec::new(),
        }
    }
}

impl AnyParse {
    pub fn new(root: SendNode, diagnostics: Vec<ParseDiagnostic>) -> AnyParse {
        AnyParse { root, diagnostics }
    }

    pub fn syntax<L>(&self) -> SyntaxNode<L>
    where
        L: Language + 'static,
    {
        self.root.clone().into_node().unwrap_or_else(|| {
            panic!(
                "could not downcast root node to language {}",
                type_name::<L>()
            )
        })
    }

    pub fn tree<N>(&self) -> N
    where
        N: AstNode,
        N::Language: 'static,
    {
        N::unwrap_cast(self.syntax::<N::Language>())
    }

    /// This function transforms diagnostics coming from the parser into serializable diagnostics
    pub fn into_diagnostics(self) -> Vec<Diagnostic> {
        self.diagnostics.into_iter().map(Diagnostic::new).collect()
    }

    /// Get the diagnostics which occurred when parsing
    pub fn diagnostics(&self) -> &[ParseDiagnostic] {
        &self.diagnostics
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|diag| diag.is_error())
    }
}
