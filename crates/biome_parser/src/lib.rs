//! # Authoring Parse Rules
//!
//! This is a short, or not so short, guide to implement parse rules using the Biome parser infrastructure.
//!
//! ## Naming
//! The convention is to prefix your parse rule with `parse_` and then use the name defined in the grammar file.
//!
//! For example, `parse_for_statement` or `parse_expression`.
//!
//! ## Signature
//! Most parse rules take a `&mut` reference to the parser as their only parameter and return a `ParsedSyntax`.
//!
//! ```rust,ignore
//! fn parse_rule_name(&mut: Parser) -> ParsedSyntax {}
//! ```
//!
//! You're free to add additional parameters to your function if needed. There are rare cases where you want to consider returning `ConditionalParsedSyntax` as explained in [conditional syntax](#conditional-syntax)
//!
//!
//! ## Parsing a single node
//!
//! Let's assume you want to parse the JS `if` statement:
//!
//! ```js
//! JsIfStatement =
//!  if
//!  (
//!  test: JsAnyExpression
//!  )
//!  consequent: JsBlockStatement
//!  else_clause: JsElseClause?
//! ```
//!
//! ### Presence Test
//!
//! Now, the parsing function must first test if the parser is positioned at an `if` statement and return `Absent` if that's not the case.
//!
//! ```rust, ignore
//! if !p.at(T![if]) {
//!  return ParsedSyntax::Absent;
//! }
//! ```
//!
//! Why return `ParsedSyntax::Absent`? The function must return `ParsedSyntax::Absent` if the rule can't predict by the next token(s) if they form the expected node or not. Doing so allows the calling rule to decide if this is an error and perform an error recovery if necessary.  The second reason is to ensure that the rule doesn't return a node where all children are missing.
//!
//! Your rule implementation may want to consider more than just the first child to determine if it can parse at least some of the expected children.
//! For example, the if statement rule could test if the parser is located at an `else` clause and then create an `if` statement where all children are missing except the `else` clause:
//!
//! ```rust, ignore
//! if !p.at(T![if]) && !p.at(T![else]){
//!   return Absent
//! }
//! ```
//!
//! Your implementation can also call into another parsing rule if the first child is a node and not a token.
//!
//! ```rust, ignore
//! let assignment_target = parse_assignment_target(p);
//!
//! if assignment_target.is_absent() {
//!   return Absent;
//! }
//!
//! let my_node = assignment_target.precede_or_missing();
//! ```
//!
//! But be careful with calling other rules. Your rule mustn't progress the parser - meaning that it can't
//! advance in the parsing process and consume tokens - if it returns `Absent`.
//!
//!
//! ### Parse children
//! The parse rules will guide you in how to write your implementation and the parser infrastructure provides the following convenience APIs:
//!
//! * Optional token `'ident'?`: Use `p.eat(token)`. It eats the next token if it matches the passed-in token.
//! * Required token `'ident'`: Use`p.expect(token)`. It eats the next token if it matches the passed-in token.
//! It adds an `Expected 'x' but found 'y' instead` error and a missing marker if the token isn't present in the source code.
//! * Optional node `body: JsBlockStatement?`: Use`parse_block_statement(p).or_missing(p)`. It parses the block if it is present in the source code and adds a missing marker if it isn't.
//! * Required node `body: JsBlockStatement`: Use `parse_block_statement(p).or_missing_with_error(p, error_builder)`:
//! it parses the block statement if it is present in the source code and adds a missing marker and an error if not.
//!
//! Using the above-described rules result in the following implementation for the `if` statement rule.
//!
//! ```rust, ignore
//! fn parse_if_statement(p: &mut Parser) -> ParsedSyntax {
//!  if !p.at(T![if]) {
//!   return Absent;
//!  }
//!
//!  let m = p.start();
//!
//!  p.expect(T![if]);
//!  p.expect(T!['(']);
//!  parse_any_expression(p).or_add_diagnostic(p, js_parse_errors::expeced_if_statement);
//!  p.expect(T![')']);
//!  parse_block_statement(p).or_add_diagnostic(p, js_parse_errors::expected_block_statement);
//! // the else block is optional, handle the marker by using `ok`
//!  parse_else_clause(p).ok();
//!
//!  Present(m.complete(p, JS_IF_STATEMENT));
//! }
//! ```
//!
//! Hold on, what are these *missing* markers? Biome's AST facade uses fixed offsets to retrieve a particular child from a node.
//! For example, the 3rd child of the if statement is the condition. However, the condition would become the second element
//! if the opening parentheses `(` isn't present in the source text. That's where missing elements come into play.
//!
//! ## Parsing Lists & Error Recovery
//!
//! Parsing lists is different from parsing single elements with a fixed set of children because it requires looping until
//! the parser reaches a terminal token (or the end of the file).
//!
//! You may remember that `parse_*` methods shouldn't progress parsing if they return `Absent`.
//! Not progressing the parser is problematic inside `while` loops because it inevitably results in an infinite loop.
//!
//! That's why you must do error recovery when parsing lists. Luckily, the parser comes with the infrastructure to make error recovery a piece of cake.
//! The general structure for parsing a list is (yes, that's something the parser infrastructure should provide for you):
//!
//!
//! Let's try to parse an array:
//!
//! ```js
//! [ 1, 3, 6 ]
//! ```
//!
//! We will use  `ParseSeparatedList` in order to achieve that
//!
//! ```rust, ignore
//! struct ArrayElementsList;
//!
//! impl ParseSeparatedList for ArrayElementsList {
//!     type ParsedElement = CompletedMarker;
//!
//!     fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax<Self::ParsedElement> {
//!         parse_array_element(p)
//!     }
//!
//!     fn is_at_list_end(&self, p: &mut Parser) -> bool {
//!         p.at_ts(token_set![T![default], T![case], T!['}']])
//!     }
//!
//!     fn recover(
//!         &mut self,
//!         p: &mut Parser,
//!         parsed_element: ParsedSyntax<Self::ParsedElement>,
//!     ) -> parser::RecoveryResult {
//!         parsed_element.or_recover(
//!             p,
//!             &ParseRecoveryTokenSet::new(JS_BOGUS_STATEMENT, STMT_RECOVERY_SET),
//!             js_parse_error::expected_case,
//!         )
//!     }
//! };
//! ```
//!
//! Let's run through this step by step:
//!
//! ```rust, ignore
//! parsed_element.or_recover(
//!     p,
//!     &ParseRecoveryTokenSet::new(JS_BOGUS_STATEMENT, STMT_RECOVERY_SET),
//!     js_parse_error::expected_case,
//! )
//! ```
//!
//! The `or_recover` performs an error recovery if the `parse_array_element` method returns `Absent`;
//! there's no array element in the source text.
//!
//! The recovery eats all tokens until it finds one of the tokens specified in the `token_set`,
//! a line break (if you called `enable_recovery_on_line_break`) or the end of the file.
//!
//! The recovery doesn't throw the tokens away but instead wraps them inside a `JS_BOGUS_EXPRESSION` node (first parameter).
//! There exist multiple `BOGUS_*` nodes. You must consult the grammar to understand which `BOGUS*` node is supported in your case.
//!
//! > You usually want to include the terminal token ending your list, the element separator token, and the token terminating a statement in your recovery set.
//!
//!
//! Now, the problem with recovery is that it can fail, and there are two reasons:
//!
//! - the parser reached the end of the file;
//! - the next token is one of the tokens specified in the recovery set, meaning there is nothing to recover from;
//!
//! In these cases the `ParseSeparatedList` and `ParseNodeList` will recover the parser for you.
//!
//! ## Conditional Syntax
//!
//! The conditional syntax allows you to express that some syntax may not be valid in all source files. Some use cases are:
//!
//! * syntax that is only supported in strict or sloppy mode: for example, `with` statements is not valid when a JavaScript file uses `"use strict"` or is a module;
//! * syntax that is only supported in certain file types: Typescript, JSX, modules;
//! * syntax that is only available in specific language versions: experimental features, different versions of the language e.g. (ECMA versions for JavaScript);
//!
//! The idea is that the parser always parses the syntax regardless of whatever it is supported in this specific file or context.
//! The main motivation behind doing so is that this gives us perfect error recovery and allows us to use the same code regardless of whether the syntax is supported.
//!
//! However, conditional syntax must be handled because we want to add a diagnostic if the syntax isn't supported for the current file, and the parsed tokens must be attached somewhere.
//!
//! Let's have a look at the `with` statement that is only allowed in loose mode/sloppy mode:
//!
//! ```rust, ignore
//! fn parse_with_statement(p: &mut Parser) -> ParsedSyntax {
//!  if !p.at(T![with]) {
//!   return Absent;
//!  }
//!
//!  let m = p.start();
//!  p.bump(T![with]); // with
//!  parenthesized_expression(p).or_add_diagnostic(p, js_errors::expected_parenthesized_expression);
//!  parse_statement(p).or_add_diagnostic(p, js_error::expected_statement);
//!  let with_stmt = m.complete(p, JS_WITH_STATEMENT);
//!
//!  let conditional = StrictMode.excluding_syntax(p, with_stmt, |p, marker| {
//!   p.err_builder("`with` statements are not allowed in strict mode", marker.range(p))
//!  });
//!
//!
//! }
//! ```
//!
//! The start of the rule is the same as for any other rule. The exciting bits start with
//!
//! ```rust, ignore
//! let conditional = StrictMode.excluding_syntax(p, with_stmt, |p, marker| {
//!  p.err_builder("`with` statements are not allowed in strict mode", marker.range(p))
//! });
//! ```
//!
//! The `StrictMode.excluding_syntax` converts the parsed syntax to a bogus node and uses the diagnostic builder to create a diagnostic if the feature is not supported.
//!
//! You can convert the `ConditionalParsedSyntax` to a regular `ParsedSyntax` by calling `or_invalid_to_bogus`, which wraps the whole parsed `with` statement in an `BOGUS` node if the parser is in strict mode and otherwise returns the unchanged `with` statement.
//!
//! What if there's no `BOGUS` node matching the node of your parse rule? You must then return the `ConditionalParsedSyntax` without making the `or_invalid_to_bogus` recovery. It's then up to the caller to recover the potentially invalid syntax.
//!
//!
//! ## Summary
//!
//! * Parse rules are named `parse_rule_name`
//! * The parse rules should return a `ParsedSyntax`
//! * The rule must return `Present` if it consumes any token and, therefore, can parse the node with at least some of its children.
//! * It returns `Absent` otherwise and must not progress parsing nor add any errors.
//! * Lists must perform error recovery to avoid infinite loops.
//! * Consult the grammar to identify the `BOGUS` node that is valid in the context of your rule.
//!

use crate::diagnostic::{expected_token, ParseDiagnostic, ToDiagnostic};
use crate::event::Event;
use crate::event::Event::Token;
use crate::token_source::{BumpWithContext, NthToken, TokenSource};
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

    /// Returns a reference to the [`ParserContext`](ParserContext)
    fn context(&self) -> &ParserContext<Self::Kind>;

    /// Returns a mutable reference to the [`ParserContext`](ParserContext).
    fn context_mut(&mut self) -> &mut ParserContext<Self::Kind>;

    /// Returns a reference to the [`TokenSource``](TokenSource]
    fn source(&self) -> &Self::Source;

    /// Returns a mutable reference to the [`TokenSource`](TokenSource)
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
    fn nth(&mut self, n: usize) -> Self::Kind
    where
        Self::Source: NthToken,
    {
        self.source_mut().nth(n)
    }

    /// Checks if a token lookahead is something
    fn nth_at(&mut self, n: usize, kind: Self::Kind) -> bool
    where
        Self::Source: NthToken,
    {
        self.nth(n) == kind
    }

    /// Checks if a token set lookahead is something
    fn nth_at_ts(&mut self, n: usize, kinds: TokenSet<Self::Kind>) -> bool
    where
        Self::Source: NthToken,
    {
        kinds.contains(self.nth(n))
    }

    /// Tests if there's a line break before the nth token.
    #[inline]
    fn has_nth_preceding_line_break(&mut self, n: usize) -> bool
    where
        Self::Source: NthToken,
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
#[derive(Clone)]
pub struct AnyParse {
    pub(crate) root: SendNode,
    pub(crate) diagnostics: Vec<ParseDiagnostic>,
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
