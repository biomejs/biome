use super::diagnostic::ParseDiagnostic;
use biome_rowan::{SyntaxKind, TextRange, TextSize};
use bitflags::bitflags;
use std::collections::VecDeque;
use std::iter::FusedIterator;

pub trait Lexer<'src> {
    type Kind: SyntaxKind;
    type LexContext: LexContext + Default;
    type ReLexContext;

    /// Returns the source code
    fn source(&self) -> &'src str;

    /// Returns the kind of the current token
    fn current(&self) -> Self::Kind;

    /// Returns the range of the current token (The token that was lexed by the last `next` call)

    fn current_range(&self) -> TextRange;

    /// Creates a checkpoint storing the current lexer state.
    ///
    /// Use `rewind` to restore the lexer to the state stored in the checkpoint.
    fn checkpoint(&self) -> LexerCheckpoint<Self::Kind>;

    /// Lexes the next token.
    ///
    /// ## Return
    /// Returns its kind and any potential error.
    fn next_token(&mut self, context: Self::LexContext) -> Self::Kind;

    /// Lexes the current token again under the passed [Self::ReLexContext].
    /// Useful in case a token can have different meaning depending on the context.
    ///
    /// For example, a `/` must either be lexed as a `/` token or as a regular expression if it
    /// appears at the start of an expression. Re-lexing allows to always lex the `/` as a `/` token and
    /// call into `re_lex` when the parser is at a valid regular expression position, to see if the
    /// current token can be lexed out as a regular expression literal.
    ///
    /// ## Returns
    /// The new token kind and any associated diagnostic if current token has a different meaning under
    /// the passed [Self::ReLexContext].
    ///
    /// Returns the current kind without any diagnostic if not. Any cached lookahead remains valid in that case.
    fn re_lex(&mut self, context: Self::ReLexContext) -> Self::Kind;

    /// Returns `true` if a line break precedes the current token.
    fn has_preceding_line_break(&self) -> bool;

    /// Returns `true` if the current token is an identifier and it contains a unicode escape sequence (`\u...`).
    fn has_unicode_escape(&self) -> bool;

    /// Rewinds the lexer to the same state as when the passed in `checkpoint` was created.
    fn rewind(&mut self, checkpoint: LexerCheckpoint<Self::Kind>);

    fn finish(self) -> Vec<ParseDiagnostic>;

    fn current_flags(&self) -> TokenFlags;
}

pub trait LexContext {
    fn is_regular(&self) -> bool;
}

/// Wrapper around a [Lexer] that supports lookahead.
///
/// The underlying [Lexer] only supports inspecting the current token and lexing the next token.
/// However, the current token is often not enough for the Parser to decide what the next node is,
/// it often needs information about the next non-trivia tokens.
///
/// The [BufferedLexer] adds support for lookahead by caching the lexed tokens and keeping track
/// of the current position (and what the `nth` token is). This means, that every token
/// only gets lexed once except if the buffer cached some lookahead tokens and:
///
/// * `next_token` is called with a context other than [LexContext::default()].
/// * the lexer gets rewinded to a previous position
/// * re-lexing the current token changes the kind of the token. That means,
///   that any following token may turn out to be different as well, thus, it's necessary to clear the
///   lookahead cache.
#[derive(Debug)]
pub struct BufferedLexer<'l, Lex: Lexer<'l>> {
    /// Cache storing the lookahead tokens. That are, all tokens between the `current` token and
    /// the "current" of the [Lexer]. This is because the [Lexer]'s current token points to the
    /// furthest requested lookahead token.
    ///
    /// For example for the following source `let a = 2;`. The `current` token of the inner [Lexer] and
    /// of the [BufferedLexer] after one call to `next_token` is the `let` token. However, the `current`
    /// token diverges if the [BufferedLexer] performs lookahead. Let's say you do a lookahead of 4 (`=` token).
    /// Now, the [BufferedLexer] calls [Lexer::next_token] four times, moving the [Lexer]'s `current`
    /// token to the `=`. However, the `current` of the [BufferedLexer] still points to the `let` token.
    /// That's why the [BufferedLexer] stores the following information:
    /// * `current`: `let` (information about the `current` token from the consumer perspective)
    /// * `lookahead`: [WHITESPACE, IDENT: 'a', WHITESPACE]. The tokens that have been lexed to
    ///    answer the "lookahead 4" request but haven't been returned yet.
    /// * [Lexer::current]: Points to `=`
    lookahead: VecDeque<LexerCheckpoint<Lex::Kind>>,

    /// Stores the information of the current token in case the `lexer` is at least one token ahead.
    current: Option<LexerCheckpoint<Lex::Kind>>,

    /// Underlying lexer. May be ahead if iterated with lookahead
    inner: Lex,
}

impl<'l, Lex: Lexer<'l>> BufferedLexer<'l, Lex> {
    /// Creates a new [BufferedLexer] wrapping the passed in [Lexer].
    pub fn new(lexer: Lex) -> Self {
        Self {
            inner: lexer,
            current: None,
            lookahead: VecDeque::new(),
        }
    }

    /// Returns the kind of the next token and any associated diagnostic.
    ///
    /// [See `Lexer.next_token`](Lexer::next_token)
    #[inline(always)]
    pub fn next_token(&mut self, context: Lex::LexContext) -> Lex::Kind {
        // Reset the lookahead if the context isn't the regular context because it's highly likely
        // that the lexer will return a different token.
        if !context.is_regular() {
            self.reset_lookahead();
        }
        // Retrieve the next token from the lookahead cache if it isn't empty
        else if let Some(next) = self.lookahead.pop_front() {
            let kind = next.current_kind;

            // Store the lookahead as the current token if the lookahead isn't empty (in which case,
            // the lexer is still at least one token ahead).

            if self.lookahead.is_empty() {
                self.current = None;
            } else {
                self.current = Some(next);
            }

            return kind;
        }

        // The [BufferedLexer] and [Lexer] are now both at the same position. Clear the cached
        // current token and lex out the next token.
        self.current = None;
        self.inner.next_token(context)
    }

    /// Returns the kind of the current token
    #[inline(always)]
    pub fn current(&self) -> Lex::Kind {
        if let Some(current) = &self.current {
            current.current_kind
        } else {
            self.inner.current()
        }
    }

    /// Returns the range of the current token
    #[inline(always)]
    pub fn current_range(&self) -> TextRange {
        if let Some(current) = &self.current {
            TextRange::new(current.current_start, current.position)
        } else {
            self.inner.current_range()
        }
    }

    /// Tests if there's a line break before the current token.
    #[inline(always)]
    pub fn has_preceding_line_break(&self) -> bool {
        if let Some(current) = &self.current {
            current.has_preceding_line_break()
        } else {
            self.inner.has_preceding_line_break()
        }
    }

    /// Returns true if the current token is an identifier, and it contains any unicode escape sequences
    #[inline]
    pub fn has_unicode_escape(&self) -> bool {
        if let Some(current) = &self.current {
            current.has_unicode_escape()
        } else {
            self.inner.has_unicode_escape()
        }
    }

    /// Returns the source text
    #[inline]
    pub fn source(&self) -> &'l str {
        self.inner.source()
    }

    /// Creates a checkpoint representing the current lexer state. Allows rewinding
    /// the lexer to this position later on.
    pub fn checkpoint(&self) -> LexerCheckpoint<Lex::Kind> {
        if let Some(current) = &self.current {
            current.clone()
        } else {
            self.inner.checkpoint()
        }
    }

    /// Rewinds the lexer to the state stored in the checkpoint.
    pub fn rewind(&mut self, checkpoint: LexerCheckpoint<Lex::Kind>) {
        self.inner.rewind(checkpoint);
        self.lookahead.clear();
        self.current = None;
    }

    fn reset_lookahead(&mut self) {
        if let Some(current) = self.current.take() {
            self.inner.rewind(current);
            self.lookahead.clear();
        }
    }

    /// Re-lex the current token in the given context
    /// See [Lexer::re_lex]
    pub fn re_lex(&mut self, context: Lex::ReLexContext) -> Lex::Kind {
        let current_kind = self.current();
        let current_checkpoint = self.inner.checkpoint();

        if let Some(current) = self.current.take() {
            self.inner.rewind(current);
        }

        let new_kind = self.inner.re_lex(context);

        if new_kind != current_kind {
            // The token has changed, clear the lookahead
            self.lookahead.clear();
        } else if !self.lookahead.is_empty() {
            // It's still the same kind. So let's move the lexer back to the position it was before re-lexing
            // and keep the lookahead as is.
            self.current = Some(self.inner.checkpoint());
            self.inner.rewind(current_checkpoint);
        }

        new_kind
    }

    /// Returns an iterator over the tokens following the current token to perform lookahead.
    /// For example, what's the 3rd token after the current token?
    #[inline(always)]
    pub fn lookahead<'s>(&'s mut self) -> LookaheadIterator<'s, 'l, Lex> {
        LookaheadIterator::new(self)
    }

    /// Consumes the buffered lexer and returns the lexing diagnostics
    pub fn finish(self) -> Vec<ParseDiagnostic> {
        self.inner.finish()
    }
}

#[derive(Debug)]
pub struct LookaheadIterator<'l, 't, Lex: Lexer<'t>> {
    buffered: &'l mut BufferedLexer<'t, Lex>,
    nth: usize,
}

impl<'l, 't, Lex: Lexer<'t>> LookaheadIterator<'l, 't, Lex> {
    fn new(lexer: &'l mut BufferedLexer<'t, Lex>) -> Self {
        Self {
            buffered: lexer,
            nth: 0,
        }
    }
}

impl<'l, 't, Lex: Lexer<'t>> Iterator for LookaheadIterator<'l, 't, Lex> {
    type Item = LookaheadToken<Lex::Kind>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let lookbehind = &self.buffered.lookahead;
        self.nth += 1;

        // Is the `nth` token already in the cache, then return it
        if let Some(lookbehind) = lookbehind.get(self.nth - 1) {
            let lookahead = LookaheadToken::from(lookbehind);
            return Some(lookahead);
        }

        let lexer = &mut self.buffered.inner;

        // We're already at the end, calling next now only returns `EOF` again. End the iterator.
        if lexer.current() == SyntaxKind::EOF {
            return None;
        }

        // Store the current token before moving the inner lexer forward if we haven't done so.
        // Necessary to prevent that [BufferedLexer::current] moves forward when performing lookahead.
        if self.buffered.current.is_none() {
            self.buffered.current = Some(lexer.checkpoint());
        }

        let kind = lexer.next_token(Lex::LexContext::default());
        // Lex the next token and cache it in the lookahead cache. Needed to cache it right away
        // because of the diagnostic.
        let checkpoint = lexer.checkpoint();
        self.buffered.lookahead.push_back(checkpoint);

        Some(LookaheadToken {
            kind,
            flags: lexer.current_flags(),
        })
    }
}

impl<'l, 't, Lex: Lexer<'t>> FusedIterator for LookaheadIterator<'l, 't, Lex> {}

#[derive(Debug)]
pub struct LookaheadToken<Kind: SyntaxKind> {
    kind: Kind,
    flags: TokenFlags,
}

impl<Kind: SyntaxKind> LookaheadToken<Kind> {
    pub fn kind(&self) -> Kind {
        self.kind
    }

    pub fn has_preceding_line_break(&self) -> bool {
        self.flags.has_preceding_line_break()
    }
}

impl<Kind: SyntaxKind> From<&LexerCheckpoint<Kind>> for LookaheadToken<Kind> {
    fn from(checkpoint: &LexerCheckpoint<Kind>) -> Self {
        LookaheadToken {
            kind: checkpoint.current_kind,
            flags: checkpoint.current_flags,
        }
    }
}

/// Stores the state of the lexer so that it may later be restored to that position.
#[derive(Debug, Clone)]
pub struct LexerCheckpoint<Kind: SyntaxKind> {
    pub position: TextSize,
    pub current_start: TextSize,
    pub current_kind: Kind,
    pub current_flags: TokenFlags,
    pub after_line_break: bool,
    pub diagnostics_pos: u32,
}

impl<Kind: SyntaxKind> LexerCheckpoint<Kind> {
    /// Returns the byte offset of the current token.
    pub fn current_start(&self) -> TextSize {
        self.current_start
    }

    pub(crate) fn has_preceding_line_break(&self) -> bool {
        self.current_flags.has_preceding_line_break()
    }

    pub(crate) fn has_unicode_escape(&self) -> bool {
        self.current_flags.has_unicode_escape()
    }
}

bitflags! {
    /// Flags for a lexed token.
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct TokenFlags: u8 {
        /// Indicates that there has been a line break between the last non-trivia token
        const PRECEDING_LINE_BREAK = 1 << 0;

        /// Indicates that an identifier contains an unicode escape sequence
        const UNICODE_ESCAPE = 1 << 1;
    }
}

impl TokenFlags {
    pub const fn has_preceding_line_break(&self) -> bool {
        self.contains(TokenFlags::PRECEDING_LINE_BREAK)
    }

    pub const fn has_unicode_escape(&self) -> bool {
        self.contains(TokenFlags::UNICODE_ESCAPE)
    }
}
