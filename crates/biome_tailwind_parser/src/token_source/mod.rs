//! The token source of the parser, over [SimdLexer], which classifies the whole class string
//! before parsing.

#[cfg(test)]
pub(crate) mod tests;

use crate::lexer::simd::{self, SimdLexer, Stop, css_string_end};
use crate::lexer::{BaseNameStore, TailwindLexContext};
use crate::syntax::group_regular;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::prelude::BumpWithContext;
use biome_parser::token_source::{TokenSource, Trivia};
use biome_rowan::TriviaPieceKind;
use biome_tailwind_syntax::TailwindSyntaxKind::{CSS_STRING_LITERAL, EOF, TW_SELECTOR, WHITESPACE};
use biome_tailwind_syntax::{TailwindSyntaxKind, TextRange, TextSize};

/// Groups the atoms after the current token into the token after it: sets its kind and end with
/// the primitives of [TailwindTokenSource], and pushes its diagnostics.
///
/// The same bytes form different tokens depending on what the parser expects, so the parser
/// passes a grouping to each bump.
pub(crate) type Grouping = fn(&mut TailwindTokenSource);

/// Token source for the parser.
///
/// Each token comes from a [Grouping], which merges the atoms after the token before it. A bump
/// without one groups the next token with [group_regular].
pub(crate) struct TailwindTokenSource<'source> {
    lexer: SimdLexer<'source>,
    diagnostics: Vec<ParseDiagnostic>,

    /// The kind of the current token as lexed, which the parser may remap.
    current: TailwindSyntaxKind,
    current_start: TextSize,
    current_end: TextSize,
    current_has_line_break: bool,

    /// Whether the current token is whitespace with a line break.
    after_newline: bool,

    /// The last token that [Self::lookahead] returned.
    lookahead: LookaheadCursor,

    /// Trivia emitted by the lexer and tokens skipped during parser recovery.
    trivia_list: Vec<Trivia>,
}

/// Where [TailwindTokenSource::lookahead] stopped. Lookahead groups with [group_regular] after the
/// current token, so it depends only on where the current token ends and on its kind.
#[derive(Clone, Copy)]
struct LookaheadCursor {
    /// The end of the current token when the lookahead ran.
    from: TextSize,
    /// The kind of the current token when the lookahead ran.
    after: TailwindSyntaxKind,
    /// How many non-trivia tokens after the current one the lookahead lexed, or zero if none.
    count: usize,
    /// The last of those tokens.
    token: LookaheadToken,
}

/// A token returned by lookahead.
#[derive(Clone, Copy)]
pub(crate) struct LookaheadToken {
    pub(crate) kind: TailwindSyntaxKind,
    pub(crate) range: TextRange,
}

/// A position of [TailwindTokenSource] to which it can return.
#[derive(Debug)]
pub(crate) struct TailwindTokenSourceCheckpoint {
    current: TailwindSyntaxKind,
    current_start: TextSize,
    current_end: TextSize,
    current_has_line_break: bool,
    after_newline: bool,
    trivia_len: u32,
    diagnostics_len: u32,
}

impl<'source> TailwindTokenSource<'source> {
    /// Creates a new token source for the given string.
    pub(crate) fn from_str(source: &'source str, base_names: &'source BaseNameStore) -> Self {
        let mut source = Self {
            lexer: SimdLexer::new(source, base_names),
            diagnostics: Vec::new(),
            // The lexer starts with `EOF` as its current kind.
            current: EOF,
            current_start: TextSize::from(0),
            current_end: TextSize::from(0),
            current_has_line_break: false,
            after_newline: false,
            lookahead: LookaheadCursor {
                from: TextSize::from(0),
                after: EOF,
                count: 0,
                token: LookaheadToken {
                    kind: EOF,
                    range: TextRange::default(),
                },
            },
            trivia_list: vec![],
        };
        source.next_non_trivia_token(group_regular, true);
        source
    }

    /// Moves to an empty token of kind `previous` at `offset`, for testing a [Grouping] after
    /// it, and drops the diagnostics and trivia so far.
    #[cfg(test)]
    pub(crate) fn place_after(&mut self, offset: usize, previous: TailwindSyntaxKind) {
        self.current = previous;
        self.current_start = TextSize::from(offset as u32);
        self.current_end = TextSize::from(offset as u32);
        self.after_newline = false;
        self.diagnostics.clear();
        self.trivia_list.clear();
    }

    #[inline]
    fn next_non_trivia_token(&mut self, group: Grouping, first_token: bool) {
        let trailing = !first_token;
        loop {
            group(self);
            let Ok(trivia_kind) = TriviaPieceKind::try_from(self.current) else {
                break;
            };
            self.trivia_list
                .push(Trivia::new(trivia_kind, self.current_range(), trailing));
        }
    }

    /// Lexes the token at `current_end` in `context` with [SimdLexer].
    fn lex(&mut self, context: TailwindLexContext) {
        let start = usize::from(self.current_end);
        let (kind, end) = self
            .lexer
            .lex(start, context, self.current, &mut self.diagnostics);
        let line_break = kind == WHITESPACE && self.lexer.has_line_break(start, end);
        self.set_token(kind, end, line_break);
    }

    /// Makes the token of `kind` that ends at `end` the token after the current one.
    #[inline(always)]
    fn set_token(&mut self, kind: TailwindSyntaxKind, end: usize, line_break: bool) {
        self.set_current(kind, TextSize::from(end as u32), line_break);
    }

    /// Returns the first byte after the current token, or `None` at the end of the source.
    #[inline(always)]
    pub(crate) fn next_byte(&self) -> Option<u8> {
        self.text()
            .as_bytes()
            .get(usize::from(self.current_end))
            .copied()
    }

    /// Makes the `len` bytes after the current token a token of `kind`.
    #[inline(always)]
    pub(crate) fn take(&mut self, kind: TailwindSyntaxKind, len: usize) {
        self.set_token(kind, usize::from(self.current_end) + len, false);
    }

    /// Makes the punctuation byte after the current token, one of `()[]:-!/%+*,.=`, the next
    /// token.
    #[inline(always)]
    pub(crate) fn take_punctuation(&mut self, byte: u8) {
        self.take(simd::punctuation(byte), 1);
    }

    /// Returns whether the current token ends at the start of the source, which starts with a
    /// byte order mark.
    pub(crate) fn at_byte_order_mark(&self) -> bool {
        self.current_end == TextSize::from(0) && self.text().starts_with('\u{FEFF}')
    }

    /// Makes the char after the current token an error token, and reports it as unexpected.
    #[cold]
    pub(crate) fn take_unexpected(&mut self) {
        let (kind, end) = self
            .lexer
            .unexpected(usize::from(self.current_end), &mut self.diagnostics);
        self.set_token(kind, end, false);
    }

    /// Makes the basename after the current token the next token: the atoms up to the next `-`,
    /// whitespace, `!`, `:` or `/`, continued over each `-` that the dashed basenames continue
    /// over. A basename that is exactly `data` is `DATA_KW`.
    #[inline(always)]
    pub(crate) fn merge_base(&mut self) {
        let (kind, end) = self.lexer.base(usize::from(self.current_end));
        self.set_token(kind, end, false);
    }

    /// Makes the atoms after the current token, up to the first one that `stop` matches, a
    /// `TW_NUMBER` if they hold only digits and `.`, and a `TW_VALUE` otherwise. The value is an
    /// empty `TW_NUMBER` if `stop` matches the first atom.
    #[inline(always)]
    pub(crate) fn merge_value(&mut self, stop: Stop) {
        let start = usize::from(self.current_end);
        let end = self.lexer.find_stop(start, stop);
        let kind = self.lexer.number_or_value(start, end);
        self.set_token(kind, end, false);
    }

    /// Makes the selector after the current token the next token: the atoms up to the next
    /// whitespace or the `]` that closes the brackets the selector opens.
    #[inline(always)]
    pub(crate) fn merge_selector(&mut self) {
        let end = self.lexer.selector_end(usize::from(self.current_end));
        self.set_token(TW_SELECTOR, end, false);
    }

    /// Makes the whitespace after the current token the next token, and reports the bytes in it
    /// that CSS doesn't allow.
    #[inline(always)]
    pub(crate) fn take_whitespace(&mut self) {
        let start = usize::from(self.current_end);
        let end = self.lexer.whitespace_end(start);
        let line_break = self.lexer.has_line_break_or_bad_whitespace(start, end)
            && self.whitespace_slow(start, end);
        self.set_token(WHITESPACE, end, line_break);
    }

    /// Reports the bytes in the whitespace `start..end` that CSS doesn't allow, and returns
    /// whether it holds a line break.
    #[cold]
    fn whitespace_slow(&mut self, start: usize, end: usize) -> bool {
        self.lexer
            .push_whitespace_diagnostics(start, end, &mut self.diagnostics);
        self.lexer.has_line_break(start, end)
    }

    /// Makes the CSS string after the current token the next token. It runs to its closing quote
    /// or to the next whitespace.
    pub(crate) fn take_string(&mut self) {
        let end = css_string_end(self.text().as_bytes(), usize::from(self.current_end));
        self.set_token(CSS_STRING_LITERAL, end, false);
    }

    /// Makes the atoms after the current token, up to the first one that `stop` matches, a token
    /// of kind `kind`.
    #[inline(always)]
    pub(crate) fn merge_until(&mut self, stop: Stop, kind: TailwindSyntaxKind) {
        let end = self.lexer.find_stop(usize::from(self.current_end), stop);
        self.set_token(kind, end, false);
    }

    /// Makes the token of `kind` that ends at `end` the token after the current one.
    #[inline(always)]
    fn set_current(&mut self, kind: TailwindSyntaxKind, end: TextSize, line_break: bool) {
        self.current = kind;
        self.current_start = self.current_end;
        self.current_end = end;
        self.current_has_line_break = self.after_newline;
        self.after_newline = line_break;
    }

    /// Returns the `n`th non-trivia token after the current one, grouped with [group_regular].
    pub(crate) fn lookahead(&mut self, n: usize) -> Option<LookaheadToken> {
        debug_assert_ne!(n, 0);
        if self.current == EOF {
            return None;
        }
        let cursor = self.lookahead;
        let (mut count, mut previous) = if cursor.from == self.current_end
            && cursor.after == self.current
            && cursor.count != 0
            && cursor.count <= n
        {
            if cursor.count == n {
                return Some(cursor.token);
            }
            (cursor.count, cursor.token)
        } else {
            (
                0,
                LookaheadToken {
                    kind: self.current,
                    range: self.current_range(),
                },
            )
        };

        loop {
            if previous.kind == EOF {
                return None;
            }
            let token = self.lookahead_token(previous);
            previous = token;
            if TriviaPieceKind::try_from(token.kind).is_ok() {
                continue;
            }
            count += 1;
            if count == n {
                self.lookahead = LookaheadCursor {
                    from: self.current_end,
                    after: self.current,
                    count,
                    token,
                };
                return Some(token);
            }
        }
    }

    /// Returns the token after `previous`, grouped with [group_regular], without its diagnostics.
    fn lookahead_token(&mut self, previous: LookaheadToken) -> LookaheadToken {
        let checkpoint = self.checkpoint();
        self.current = previous.kind;
        self.current_start = previous.range.start();
        self.current_end = previous.range.end();
        group_regular(self);
        let token = LookaheadToken {
            kind: self.current,
            range: self.current_range(),
        };
        self.rewind(checkpoint);
        token
    }

    /// Creates a checkpoint to which it can later return using [Self::rewind].
    pub(crate) fn checkpoint(&self) -> TailwindTokenSourceCheckpoint {
        TailwindTokenSourceCheckpoint {
            current: self.current,
            current_start: self.current_start,
            current_end: self.current_end,
            current_has_line_break: self.current_has_line_break,
            after_newline: self.after_newline,
            trivia_len: self.trivia_list.len() as u32,
            diagnostics_len: self.diagnostics.len() as u32,
        }
    }

    /// Restores the token source to a previous state.
    pub(crate) fn rewind(&mut self, checkpoint: TailwindTokenSourceCheckpoint) {
        assert!(self.trivia_list.len() >= checkpoint.trivia_len as usize);
        self.trivia_list.truncate(checkpoint.trivia_len as usize);
        self.diagnostics
            .truncate(checkpoint.diagnostics_len as usize);
        self.current = checkpoint.current;
        self.current_start = checkpoint.current_start;
        self.current_end = checkpoint.current_end;
        self.current_has_line_break = checkpoint.current_has_line_break;
        self.after_newline = checkpoint.after_newline;
    }

    /// Groups the atoms of the current token and the ones after it again with `group`, as if no
    /// token came before them.
    ///
    /// Like the legacy token source, this keeps the diagnostics of the current token, and takes
    /// the current token's line break state as the one before it.
    pub(crate) fn regroup_current(&mut self, group: Grouping) {
        self.current_end = self.current_start;
        self.current = EOF;
        group(self);
    }

    /// Returns the kind of the `n`th non-trivia token after the current one, grouped with
    /// [group_regular], or `EOF` past the end.
    pub(crate) fn nth(&mut self, n: usize) -> TailwindSyntaxKind {
        if n == 0 {
            return self.current;
        }
        self.lookahead(n).map_or(EOF, |token| token.kind)
    }

    /// Returns whether a `:` follows the start of the current token before the next whitespace.
    pub(crate) fn has_colon_before_whitespace(&self) -> bool {
        self.lexer
            .has_colon_before_whitespace(usize::from(self.current_start))
    }
}

impl TokenSource for TailwindTokenSource<'_> {
    type Kind = TailwindSyntaxKind;

    #[inline(always)]
    fn current(&self) -> Self::Kind {
        self.current
    }

    #[inline(always)]
    fn current_range(&self) -> TextRange {
        TextRange::new(self.current_start, self.current_end)
    }

    fn text(&self) -> &str {
        self.lexer.source()
    }

    #[inline(always)]
    fn has_preceding_line_break(&self) -> bool {
        self.current_has_line_break
    }

    fn bump(&mut self) {
        self.bump_with_context(group_regular)
    }

    fn skip_as_trivia(&mut self) {
        self.skip_as_trivia_with_context(group_regular)
    }

    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        (self.trivia_list, self.diagnostics)
    }
}

impl BumpWithContext for TailwindTokenSource<'_> {
    type Context = Grouping;

    #[inline(always)]
    fn bump_with_context(&mut self, group: Self::Context) {
        if self.current() != EOF {
            self.next_non_trivia_token(group, false);
        }
    }

    fn skip_as_trivia_with_context(&mut self, group: Self::Context) {
        if self.current() != EOF {
            self.trivia_list.push(Trivia::new(
                TriviaPieceKind::Skipped,
                self.current_range(),
                false,
            ));
            self.next_non_trivia_token(group, true)
        }
    }
}

/// The [Grouping] of the tokens that [SimdLexer] lexes in [TailwindLexContext::CssValue].
pub(crate) fn lex_css_value(source: &mut TailwindTokenSource) {
    source.lex(TailwindLexContext::CssValue);
}
