use super::diagnostic::ParseDiagnostic;
use biome_rowan::{SyntaxKind, TextRange, TextSize};
use biome_unicode_table::{lookup_byte, Dispatch::WHS};
use enumflags2::{bitflags, make_bitflags, BitFlags};
use std::collections::VecDeque;
use std::iter::FusedIterator;
use std::ops::{BitOr, BitOrAssign};
use unicode_bom::Bom;

/// `Lexer` trait defines the necessary methods a lexer must implement.
/// Lexer is responsible for dividing the source code into meaningful parsing units (kinds).
pub trait Lexer<'src> {
    /// The kind of the newline
    const NEWLINE: Self::Kind;

    /// The kind of the space
    const WHITESPACE: Self::Kind;

    /// A kind of syntax, as identified by the lexer.
    type Kind: SyntaxKind;
    /// The specific context in which the lexer operates.
    type LexContext: LexContext + Default;
    /// The special context for re-lexing the kinds.
    type ReLexContext;

    /// Returns the entire source code that lexer needs to tokenize.
    fn source(&self) -> &'src str;

    /// Returns the syntax kind of the current kind.
    fn current(&self) -> Self::Kind;

    /// Provides the text range of the current kind.
    fn current_range(&self) -> TextRange {
        TextRange::new(self.current_start(), TextSize::from(self.position() as u32))
    }

    /// Byte offset of the current token from the start of the source
    /// The range of the current token can be computed by `self.position - self.current_start`
    fn current_start(&self) -> TextSize;

    /// Tokenizes the next kind into a single coherent token within the given lexing context.
    fn next_token(&mut self, context: Self::LexContext) -> Self::Kind;

    /// Returns `true` if the current kind is preceded by a line break.
    fn has_preceding_line_break(&self) -> bool;

    /// Returns if the current kind is an identifier that includes a unicode escape sequence (`\u...`).
    fn has_unicode_escape(&self) -> bool;

    /// Restores lexer to the state of a previously created checkpoint.
    fn rewind(&mut self, checkpoint: LexerCheckpoint<Self::Kind>);

    /// Returns any parsing diagnostics upon finishing lexing all kinds.
    fn finish(self) -> Vec<ParseDiagnostic>;

    /// Provides flags related to the current kind, containing the meta-information about the kind.
    fn current_flags(&self) -> TokenFlags {
        TokenFlags::empty()
    }

    /// Returns the current position
    fn position(&self) -> usize;

    fn push_diagnostic(&mut self, diagnostic: ParseDiagnostic);

    /// Consume one newline or all whitespace until a non-whitespace or a newline is found.
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn consume_newline_or_whitespaces(&mut self) -> Self::Kind {
        if self.consume_newline() {
            Self::NEWLINE
        } else {
            self.consume_whitespaces();
            Self::WHITESPACE
        }
    }

    /// Consumes all whitespace until a non-whitespace or a newline is found.
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn consume_whitespaces(&mut self) {
        self.assert_current_char_boundary();

        while let Some(byte) = self.current_byte() {
            let dispatch = lookup_byte(byte);

            match dispatch {
                WHS => match byte {
                    b'\t' | b' ' => self.advance(1),
                    b'\r' | b'\n' => {
                        break;
                    }
                    _ => {
                        let start = self.text_position();
                        self.advance(1);

                        self.push_diagnostic(
                            ParseDiagnostic::new(
                                "The CSS standard only allows tabs, whitespace, carriage return and line feed whitespace.",
                                start..self.text_position(),
                            )
                                .with_hint("Use a regular whitespace character instead."),
                        )
                    }
                },

                _ => break,
            }
        }
    }

    /// Asserts that the lexer is currently positioned at `byte`
    fn assert_byte(&self, byte: u8) {
        debug_assert_eq!(self.source().as_bytes()[self.position()], byte);
    }

    /// Advances the position by one and returns the next byte value
    fn next_byte(&mut self) -> Option<u8> {
        self.advance(1);
        self.current_byte()
    }

    /// Returns `true` if the parser is at or passed the end of the file.
    fn is_eof(&self) -> bool {
        self.position() >= self.source().len()
    }

    fn advance_byte_or_char(&mut self, chr: u8) {
        if chr.is_ascii() {
            self.advance(1);
        } else {
            self.advance_char_unchecked();
        }
    }

    /// Asserts that the lexer is at a UTF8 char boundary
    #[inline]
    fn assert_at_char_boundary(&self, offset: usize) {
        debug_assert!(self.source().is_char_boundary(self.position() + offset));
    }

    /// Advances the current position by the current char UTF8 length
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn advance_char_unchecked(&mut self);

    /// Consume just one newline/line break.
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn consume_newline(&mut self) -> bool {
        self.assert_current_char_boundary();

        match self.current_byte() {
            Some(b'\n') => {
                self.advance(1);
                true
            }
            Some(b'\r') => {
                if self.peek_byte() == Some(b'\n') {
                    self.advance(2)
                } else {
                    self.advance(1)
                }
                true
            }

            _ => false,
        }
    }

    /// Advances the current position by `n` bytes.
    fn advance(&mut self, n: usize);

    /// Asserts that the lexer is at current a UTF8 char boundary
    #[inline]
    fn assert_current_char_boundary(&self) {
        debug_assert!(self.source().is_char_boundary(self.position()));
    }

    /// Gets the current byte.
    ///
    /// ## Returns
    /// The current byte if the lexer isn't at the end of the file.
    #[inline]
    fn current_byte(&self) -> Option<u8> {
        if self.is_eof() {
            None
        } else {
            Some(self.source().as_bytes()[self.position()])
        }
    }

    /// Peeks at the next byte
    #[inline]
    fn peek_byte(&self) -> Option<u8> {
        self.byte_at(1)
    }

    /// Returns the byte at position `self.position + offset` or `None` if it is out of bounds.
    #[inline]
    fn byte_at(&self, offset: usize) -> Option<u8> {
        self.source()
            .as_bytes()
            .get(self.position() + offset)
            .copied()
    }

    #[inline]
    fn text_position(&self) -> TextSize {
        TextSize::try_from(self.position()).expect("Input to be smaller than 4 GB")
    }

    /// Check if the source starts with a Unicode BOM character. If it does,
    /// consume it and return the UNICODE_BOM token kind.
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary (and realistically only at
    /// the start position of the source).
    #[inline]
    fn consume_potential_bom<K>(&mut self, kind: K) -> Option<(K, usize)> {
        // Bom needs at least the first three bytes of the source to know if it
        // matches the UTF-8 BOM and not an alternative. This can be expanded
        // to more bytes to support other BOM characters if Biome decides to
        // support other encodings like UTF-16.
        if let Some(first) = self.source().get(0..3) {
            let bom = Bom::from(first.as_bytes());
            self.advance(bom.len());

            match bom {
                Bom::Null => None,
                _ => Some((kind, bom.len())),
            }
        } else {
            None
        }
    }

    /// Get the UTF8 char which starts at the current byte
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    #[inline]
    fn current_char_unchecked(&self) -> char {
        // Precautionary measure for making sure the unsafe code below does not
        // read over memory boundary.
        debug_assert!(!self.is_eof());
        self.assert_current_char_boundary();

        // Safety: We know this is safe because we require the input to the
        // lexer to be valid utf8 and we always call this when we are at a char.
        unsafe {
            let Some(chr) = self
                .source()
                .get_unchecked(self.position()..self.source().len())
                .chars()
                .next()
            else {
                core::hint::unreachable_unchecked();
            };
            chr
        }
    }

    /// Check if the lexer starts a grit metavariable
    fn is_metavariable_start(&mut self) -> bool {
        let current_char = self.current_char_unchecked();
        if current_char == 'µ' {
            let current_char_length = current_char.len_utf8();
            // µ[a-zA-Z_][a-zA-Z0-9_]*
            if matches!(
                self.byte_at(current_char_length),
                Some(b'a'..=b'z' | b'A'..=b'Z' | b'_')
            ) {
                return true;
            }

            // µ...
            if self.byte_at(current_char_length) == Some(b'.')
                && self.byte_at(current_char_length + 1) == Some(b'.')
                && self.byte_at(current_char_length + 2) == Some(b'.')
            {
                return true;
            }
        }
        false
    }

    /// Consume a grit metavariable(µ[a-zA-Z_][a-zA-Z0-9_]*|µ...)
    /// <https://github.com/getgrit/gritql/blob/8f3f077d078ccaf0618510bba904a06309c2435e/resources/language-metavariables/tree-sitter-css/grammar.js#L388>
    fn consume_metavariable<T>(&mut self, kind: T) -> T {
        debug_assert!(self.is_metavariable_start());

        // SAFETY: We know the current character is µ.
        let current_char = self.current_char_unchecked();
        self.advance(current_char.len_utf8());

        if self.current_byte() == Some(b'.') {
            // SAFETY: We know that the current token is µ...
            self.advance(3);
        } else {
            // µ[a-zA-Z_][a-zA-Z0-9_]*
            self.advance(1);
            while let Some(chr) = self.current_byte() {
                match chr {
                    b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' => {
                        self.advance(1);
                    }
                    _ => break,
                }
            }
        }

        kind
    }
}

pub trait ReLexer<'src>: Lexer<'src> + LexerWithCheckpoint<'src> {
    /// Re-lexes the current kind under a different lexing context.
    /// Useful when a token can have different interpretations based on context.
    fn re_lex(&mut self, context: Self::ReLexContext) -> Self::Kind;
}

pub trait LexerWithCheckpoint<'src>: Lexer<'src> {
    /// Creating a checkpoint of the lexer's current state.
    /// `rewind` can be used later to restore the lexer to the checkpoint's state.
    fn checkpoint(&self) -> LexerCheckpoint<Self::Kind>;
}

/// `LexContext` is a trait that represents the context in
/// which a parser is currently operating. It is used to specify
/// and handle the variations in parsing requirements depending
/// on the context (i.e., the grammatical structure being parsed).
pub trait LexContext {
    /// The `is_regular` method should return `true` if the context is
    /// 'regular', indicating normal parsing rules apply.
    /// Alternatively, `false` signals that the parser is within a special context,
    /// suggesting that a different set of parsing rules or procedures may need to be used.
    fn is_regular(&self) -> bool;
}

impl LexContext for () {
    fn is_regular(&self) -> bool {
        true
    }
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
/// * `next_token` is called with a context other than `LexContext::default.
/// * the lexer gets rewinded to a previous position
/// * re-lexing the current token changes the kind of the token. That means,
///   that any following token may turn out to be different as well, thus, it's necessary to clear the
///   lookahead cache.
#[derive(Debug)]
pub struct BufferedLexer<Kind, Lex> {
    /// Cache storing the lookahead tokens. That are, all tokens between the `current` token and
    /// the "current" of the [Lexer]. This is because the [Lexer]'s current token points to the
    /// furthest requested lookahead token.
    ///
    /// For example for the following source `let a = 2;`. The `current` token of the inner [Lexer] and
    /// of the [BufferedLexer] after one call to `next_token` is the `let` token. However, the `current`
    /// token diverges if the [BufferedLexer] performs lookahead. Let's say you do a non trivia lookahead of 2 (`=` token).
    /// Now, the [BufferedLexer] calls [Lexer::next_token] four times, moving the [Lexer]'s `current`
    /// token to the `=`. However, the `current` of the [BufferedLexer] still points to the `let` token.
    /// That's why the [BufferedLexer] stores the following information:
    /// * `current`: `let` (information about the `current` token from the consumer perspective)
    /// * `all_checkpoints`: [WHITESPACE, IDENT: 'a', WHITESPACE, EQ]. The checkpoints that have been lexed to
    ///    answer the "non trivia lookahead 2" request but haven't been returned yet.
    /// * `non_trivia_checkpoints`: [IDENT: 'a', EQ]. The checkpoints that have been lexed to
    ///    answer the "non trivia lookahead 2" request but haven't been returned yet.
    /// * [Lexer::current]: Points to `=`
    lookahead: Lookahead<Kind>,

    /// Stores the information of the current token in case the `lexer` is at least one token ahead.
    current: Option<LexerCheckpoint<Kind>>,

    /// Underlying lexer. May be ahead if iterated with lookahead
    inner: Lex,
}

/// A structure that maintains two collections of lexer checkpoints:
/// one for all checkpoints and another for non-trivia checkpoints only.
#[derive(Debug)]
struct Lookahead<Kind> {
    /// A buffer of all checkpoints including trivia and non-trivia types.
    all_checkpoints: VecDeque<LexerCheckpoint<Kind>>,
    /// A buffer of only non-trivia checkpoints.
    non_trivia_checkpoints: VecDeque<LexerCheckpoint<Kind>>,
}

impl<K: SyntaxKind> Lookahead<K> {
    /// Creates a new instance of `BufferedLookahead` with specified capacity for both buffers.
    fn new() -> Self {
        Self {
            all_checkpoints: VecDeque::new(),
            non_trivia_checkpoints: VecDeque::new(),
        }
    }

    /// Adds a checkpoint to the end of both buffers, if applicable.
    ///
    /// Non-trivia checkpoints are cloned and added to the `non_trivia_checkpoints` buffer
    /// only if they are not considered trivia.
    fn push_back(&mut self, checkpoint: LexerCheckpoint<K>) {
        if !checkpoint.current_kind.is_trivia() {
            self.non_trivia_checkpoints.push_back(checkpoint.clone());
        }

        self.all_checkpoints.push_back(checkpoint);
    }

    /// Removes and returns the first element from the `all_checkpoints` buffer.
    /// Also removes the corresponding element from `non_trivia_checkpoints` if it's non-trivia.
    fn pop_front(&mut self) -> Option<LexerCheckpoint<K>> {
        if let Some(lookahead) = self.all_checkpoints.pop_front() {
            if !lookahead.current_kind.is_trivia() {
                self.non_trivia_checkpoints.pop_front();
            }
            Some(lookahead)
        } else {
            None
        }
    }

    /// Retrieves a reference to a checkpoint by index from the `all_checkpoints` buffer.
    fn get_checkpoint(&self, index: usize) -> Option<&LexerCheckpoint<K>> {
        self.all_checkpoints.get(index)
    }

    /// Retrieves a reference to a non-trivia checkpoint by index from the `non_trivia_checkpoints` buffer.
    fn get_non_trivia_checkpoint(&self, index: usize) -> Option<&LexerCheckpoint<K>> {
        self.non_trivia_checkpoints.get(index)
    }

    /// Checks if there are no checkpoints.
    fn is_empty(&self) -> bool {
        self.all_checkpoints.is_empty()
    }

    /// Clears all checkpoints from both buffers.
    fn clear(&mut self) {
        self.all_checkpoints.clear();
        self.non_trivia_checkpoints.clear();
    }

    /// Returns the number of checkpoints in the `all_checkpoints` buffer.
    fn all_len(&self) -> usize {
        self.all_checkpoints.len()
    }

    /// Returns the number of non-trivia checkpoints in the `non_trivia_checkpoints` buffer.
    fn non_trivia_len(&self) -> usize {
        self.non_trivia_checkpoints.len()
    }
}

impl<'l, Lex: Lexer<'l>> BufferedLexer<Lex::Kind, Lex> {
    /// Creates a new [BufferedLexer] wrapping the passed in [Lexer].
    pub fn new(lexer: Lex) -> Self {
        Self {
            inner: lexer,
            current: None,
            lookahead: Lookahead::new(),
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

    /// Returns an iterator over the tokens following the current token to perform lookahead.
    /// For example, what's the 3rd token after the current token?
    #[inline(always)]
    pub fn lookahead_iter<'s>(&'s mut self) -> LookaheadIterator<'s, 'l, Lex> {
        LookaheadIterator::new(self)
    }

    /// Consumes the buffered lexer and returns the lexing diagnostics
    pub fn finish(self) -> Vec<ParseDiagnostic> {
        self.inner.finish()
    }
}
impl<'l, Lex> BufferedLexer<Lex::Kind, Lex>
where
    Lex: ReLexer<'l>,
{
    /// Re-lex the current token in the given context
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
}

impl<'l, Lex> BufferedLexer<Lex::Kind, Lex>
where
    Lex: LexerWithCheckpoint<'l>,
{
    #[inline(always)]
    pub fn nth_non_trivia(&mut self, n: usize) -> Option<LookaheadToken<Lex::Kind>> {
        assert_ne!(n, 0);

        if let Some(lookahead_token) = self
            .lookahead
            .get_non_trivia_checkpoint(n - 1)
            .map(LookaheadToken::from)
        {
            return Some(lookahead_token);
        }

        // Jump right to where we've left of last time rather than going through all tokens again.
        let mut remaining = n - self.lookahead.non_trivia_len();
        let current_length = self.lookahead.all_len();
        let iter = self.lookahead_iter().skip(current_length);

        for item in iter {
            if !item.kind().is_trivia() {
                remaining -= 1;

                if remaining == 0 {
                    return Some(item);
                }
            }
        }

        None
    }

    pub fn checkpoint(&self) -> LexerCheckpoint<Lex::Kind> {
        if let Some(current) = &self.current {
            current.clone()
        } else {
            self.inner.checkpoint()
        }
    }
}

#[derive(Debug)]
pub struct LookaheadIterator<'l, 't, Lex: Lexer<'t>> {
    buffered: &'l mut BufferedLexer<Lex::Kind, Lex>,
    nth: usize,
}

impl<'l, 't, Lex: Lexer<'t>> LookaheadIterator<'l, 't, Lex> {
    fn new(lexer: &'l mut BufferedLexer<Lex::Kind, Lex>) -> Self {
        Self {
            buffered: lexer,
            nth: 0,
        }
    }
}

impl<'t, Lex: LexerWithCheckpoint<'t>> Iterator for LookaheadIterator<'_, 't, Lex> {
    type Item = LookaheadToken<Lex::Kind>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let lookbehind = &self.buffered.lookahead;
        self.nth += 1;

        // Is the `nth` token already in the cache, then return it
        if let Some(lookbehind) = lookbehind.get_checkpoint(self.nth - 1) {
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
        self.buffered.lookahead.push_back(lexer.checkpoint());

        Some(LookaheadToken {
            kind,
            flags: lexer.current_flags(),
        })
    }
}

impl<'t, Lex: LexerWithCheckpoint<'t>> FusedIterator for LookaheadIterator<'_, 't, Lex> {}

#[derive(Debug)]
pub struct LookaheadToken<Kind> {
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
pub struct LexerCheckpoint<Kind> {
    pub position: TextSize,
    pub current_start: TextSize,
    pub current_kind: Kind,
    pub current_flags: TokenFlags,
    pub after_line_break: bool,
    pub unicode_bom_length: usize,
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[bitflags]
#[repr(u8)]
enum TokenFlag {
    PrecedingLineBreak = 1 << 0,
    UnicodeEscape = 1 << 1,
}

/// Flags for a lexed token.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TokenFlags(BitFlags<TokenFlag>);

impl TokenFlags {
    /// Indicates that there has been a line break between the last non-trivia token
    pub const PRECEDING_LINE_BREAK: Self = Self(make_bitflags!(TokenFlag::{PrecedingLineBreak}));

    /// Indicates that an identifier contains an unicode escape sequence
    pub const UNICODE_ESCAPE: Self = Self(make_bitflags!(TokenFlag::{UnicodeEscape}));

    pub const fn empty() -> Self {
        Self(BitFlags::EMPTY)
    }

    pub fn contains(&self, other: impl Into<TokenFlags>) -> bool {
        self.0.contains(other.into().0)
    }

    pub fn set(&mut self, other: impl Into<TokenFlags>, cond: bool) {
        self.0.set(other.into().0, cond)
    }

    pub fn has_preceding_line_break(&self) -> bool {
        self.contains(TokenFlags::PRECEDING_LINE_BREAK)
    }

    pub fn has_unicode_escape(&self) -> bool {
        self.contains(TokenFlags::UNICODE_ESCAPE)
    }
}

impl BitOr for TokenFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        TokenFlags(self.0 | rhs.0)
    }
}

impl BitOrAssign for TokenFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
