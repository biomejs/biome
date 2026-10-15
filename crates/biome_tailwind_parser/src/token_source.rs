use crate::TailwindParserOptions;
use crate::lexer::TailwindLexer;
use crate::lexer::bulk::{self, BulkTokens, previous};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{LexContext, Lexer};
use biome_parser::prelude::BumpWithContext;
use biome_parser::token_source::{NthToken, TokenSource, Trivia};
use biome_rowan::TriviaPieceKind;
use biome_tailwind_syntax::TailwindSyntaxKind::EOF;
use biome_tailwind_syntax::{TailwindSyntaxKind, TextRange, TextSize};

/// Token source for the parser.
///
/// The tokens come from [BulkTokens], which lexes the whole source up front. Where the parser
/// asks for a token in a context or after a token that the bulk lexer didn't predict, the token
/// source lexes it with [TailwindLexer] instead.
pub(crate) struct TailwindTokenSource<'source> {
    lexer: TailwindLexer<'source>,
    tokens: BulkTokens,

    /// The kind of the current token as lexed, which the parser may remap.
    current: TailwindSyntaxKind,
    current_start: TextSize,
    current_end: TextSize,
    current_has_line_break: bool,

    /// Index of the regular bulk token that starts at `current_end`, while the current token is
    /// the bulk token before it, so that the prediction holds. Otherwise [UNKNOWN], or
    /// [NO_TOKEN] if no bulk token starts at `current_end`.
    next: u32,

    /// Whether the token before the next one is whitespace with a line break.
    after_newline: bool,

    /// The index of a regular bulk token at or before `current_end`, where
    /// [BulkTokens::find_from] starts searching.
    resync_hint: u32,

    /// The last token that [Self::lookahead] returned.
    lookahead: LookaheadCursor,

    /// Trivia emitted by the lexer and tokens skipped during parser recovery.
    pub(super) trivia_list: Vec<Trivia>,
}

/// The value of [TailwindTokenSource::next] when no regular bulk token starts at the current
/// token's end.
const NO_TOKEN: u32 = u32::MAX;
/// The value of [TailwindTokenSource::next] when the regular bulk token at the current token's
/// end hasn't been looked up.
const UNKNOWN: u32 = u32::MAX - 1;

/// Where [TailwindTokenSource::lookahead] stopped. Lookahead lexes in
/// [TailwindLexContext::Regular] after the current token, so it depends only on where the
/// current token ends and on its kind.
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
struct LookaheadToken {
    kind: TailwindSyntaxKind,
    range: TextRange,
    has_preceding_line_break: bool,
    /// Whether the token is whitespace with a line break.
    line_break: bool,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) enum TailwindLexContext {
    /// The default state.
    #[default]
    Regular,
    /// The parser just encountered a `-` before a basename, e.g. in `-mt-4`. That meant that the next token should be a basename.
    SawNegative,
    /// Like Arbitrary, but specifically for arbitrary variants.
    ArbitraryVariant,
    /// Like Arbitrary, but specifically for arbitrary candidates.
    ArbitraryCandidate,
    /// Lexing one segment inside a Tailwind variant expression.
    ///
    /// Named segments stop at `-` and `:` so `group-peer-hover`
    /// becomes `group` / `peer` / `hover`.
    VariantSegment,
    /// CSS value lexing inside arbitrary values and arbitrary candidate values.
    CssValue,
    /// Raw URL body lexing inside `url(...)`.
    CssUrlRawValue,
}

impl LexContext for TailwindLexContext {
    fn is_regular(&self) -> bool {
        matches!(self, Self::Regular)
    }
}

/// Returns whether the regular bulk token `raw` is the token that the lexer returns in
/// [TailwindLexContext::Regular] after a token of [previous] class `previous`.
#[inline(always)]
fn fits_regular(raw: u16, previous: u16) -> bool {
    let lexed_after = bulk::previous_of(raw);
    raw & bulk::SLOW == 0 && (lexed_after == previous::ANY || lexed_after == previous)
}

/// Returns whether the regular bulk token `raw`, which starts with `first`, is the token that
/// the lexer returns in `context` after a token of [previous] class `previous`.
fn fits_other(context: TailwindLexContext, raw: u16, first: u8, previous: u16) -> bool {
    if raw & bulk::SLOW != 0 {
        return false;
    }
    let lexed_after = bulk::previous_of(raw);
    match context {
        TailwindLexContext::Regular => fits_regular(raw, previous),
        // After `[`, a selector can itself start with `[`.
        TailwindLexContext::ArbitraryVariant => {
            (lexed_after == previous::ANY && !(first == b'[' && previous == previous::L_BRACK))
                || (lexed_after == previous::L_BRACK && previous == previous::L_BRACK)
        }
        // Whitespace, `(`, `)`, `[` and `]` are the same tokens in these contexts.
        TailwindLexContext::SawNegative
        | TailwindLexContext::ArbitraryCandidate
        | TailwindLexContext::CssValue => lexed_after == previous::ANY,
        TailwindLexContext::CssUrlRawValue => {
            lexed_after == previous::ANY && !matches!(first, b'(' | b'[' | b']')
        }
        TailwindLexContext::VariantSegment => false,
    }
}

impl<'source> TailwindTokenSource<'source> {
    /// Creates a new token source for the given string
    pub fn from_str(source: &'source str, options: TailwindParserOptions<'source>) -> Self {
        let lexer = TailwindLexer::from_str(source).with_base_names(options.base_names);
        let tokens = if uses_bulk_tokens(source) {
            BulkTokens::new(source, options.base_names)
        } else {
            BulkTokens::new_empty()
        };

        let mut source = Self {
            lexer,
            tokens,
            // The lexer starts with `EOF` as its current kind.
            current: EOF,
            current_start: TextSize::from(0),
            current_end: TextSize::from(0),
            current_has_line_break: false,
            next: UNKNOWN,
            after_newline: false,
            resync_hint: 0,
            lookahead: LookaheadCursor {
                from: TextSize::from(0),
                after: EOF,
                count: 0,
                token: LookaheadToken {
                    kind: EOF,
                    range: TextRange::default(),
                    has_preceding_line_break: false,
                    line_break: false,
                },
            },
            trivia_list: vec![],
        };

        source.next_non_trivia_token(TailwindLexContext::Regular, true);
        source
    }

    #[inline]
    fn next_non_trivia_token(&mut self, context: TailwindLexContext, first_token: bool) {
        let trailing = !first_token;

        loop {
            let kind = self.next_token(context);
            let Ok(trivia_kind) = TriviaPieceKind::try_from(kind) else {
                break;
            };

            self.trivia_list
                .push(Trivia::new(trivia_kind, self.current_range(), trailing));
        }
        #[cfg(test)]
        self.record();
    }

    /// Logs the current token for [tests] to compare.
    #[cfg(test)]
    fn record(&self) {
        tests::TOKENS.with_borrow_mut(|tokens| {
            tokens.push((
                self.current,
                self.current_range(),
                self.current_has_line_break,
            ));
        });
    }

    /// Moves to the token at `current_end`, lexed in `context`.
    #[inline(always)]
    fn next_token(&mut self, context: TailwindLexContext) -> TailwindSyntaxKind {
        if context == TailwindLexContext::Regular
            && let Some(raw) = self.tokens.get(self.next)
            && raw & bulk::SLOW == 0
        {
            self.take_bulk_token(raw);
            return self.current;
        }
        self.next_token_slow(context)
    }

    /// Moves to the token at `current_end`, lexed in `context`, when it isn't the next regular
    /// bulk token after the current one.
    #[inline(never)]
    fn next_token_slow(&mut self, context: TailwindLexContext) -> TailwindSyntaxKind {
        if self.tokens.is_empty() {
            return self.lex_with_lexer(context);
        }
        let position = u32::from(self.current_end);
        if context == TailwindLexContext::VariantSegment {
            let source = self.lexer.source().as_bytes();
            let token = if (position as usize) < source.len() {
                self.tokens.segment_token(source, position as usize)
            } else {
                Some((EOF as u16, position as usize))
            };
            if let Some((raw, end)) = token {
                self.set_current(raw, end as u32);
                self.next = UNKNOWN;
                return self.current;
            }
            return self.lex_with_lexer(context);
        }

        if self.next == UNKNOWN {
            self.next = self.find(position);
        }
        if let Some(raw) = self.tokens.get(self.next) {
            let previous = bulk::previous_class(self.current);
            let fits = if context == TailwindLexContext::Regular {
                fits_regular(raw, previous)
            } else {
                let source = self.lexer.source().as_bytes();
                let first = source.get(position as usize).copied().unwrap_or(0);
                fits_other(context, raw, first, previous)
            };
            if fits {
                self.take_bulk_token(raw);
                return self.current;
            }
        }
        self.lex_with_lexer(context)
    }

    /// Makes the regular bulk token `next`, of kind and flags `raw`, the current token.
    #[inline(always)]
    fn take_bulk_token(&mut self, raw: u16) {
        self.set_current(raw, self.tokens.start(self.next + 1));
        self.next += 1;
        self.resync_hint = self.next;
    }

    /// Returns the index of the regular bulk token at `position`, or [NO_TOKEN].
    #[inline(never)]
    fn find(&mut self, position: u32) -> u32 {
        match self.tokens.find_from(position, self.resync_hint) {
            Some(index) => {
                self.resync_hint = index;
                index
            }
            None => NO_TOKEN,
        }
    }

    /// Makes the bulk token `raw` that ends at `end` the token after the current one.
    #[inline(always)]
    fn set_current(&mut self, raw: u16, end: u32) {
        let kind = bulk::kind(raw);
        self.current = kind;
        self.current_start = self.current_end;
        self.current_end = TextSize::from(end);
        self.current_has_line_break = self.after_newline;
        self.after_newline = raw & bulk::LINE_BREAK != 0;
    }

    /// Lexes the token at `current_end` with [TailwindLexer].
    #[inline(never)]
    fn lex_with_lexer(&mut self, context: TailwindLexContext) -> TailwindSyntaxKind {
        #[cfg(test)]
        tests::LEXED.set(tests::LEXED.get() + 1);
        let kind = self.lexer.lex_at(
            self.current_end.into(),
            self.current,
            self.after_newline,
            context,
        );
        self.take_lexer_token(kind);
        kind
    }

    /// Makes the lexer's current token the current token.
    fn take_lexer_token(&mut self, kind: TailwindSyntaxKind) {
        let range = self.lexer.current_range();
        self.current = kind;
        self.current_start = range.start();
        self.current_end = range.end();
        self.current_has_line_break = self.lexer.has_preceding_line_break();
        self.after_newline = self.lexer.after_newline();
        self.next = UNKNOWN;
    }

    /// Returns the `n`th non-trivia token after the current one, lexed in
    /// [TailwindLexContext::Regular].
    fn lookahead(&mut self, n: usize) -> Option<LookaheadToken> {
        let token = self.scan_lookahead(n);
        #[cfg(test)]
        if let Some(token) = token {
            tests::TOKENS.with_borrow_mut(|tokens| {
                tokens.push((token.kind, token.range, token.has_preceding_line_break));
            });
        }
        token
    }

    fn scan_lookahead(&mut self, n: usize) -> Option<LookaheadToken> {
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
                    has_preceding_line_break: self.current_has_line_break,
                    line_break: self.after_newline,
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

    /// Returns the token after `previous`, lexed in [TailwindLexContext::Regular].
    fn lookahead_token(&mut self, previous: LookaheadToken) -> LookaheadToken {
        let position = u32::from(previous.range.end());
        if !self.tokens.is_empty()
            && let Some(index) = self.tokens.find_from(position, self.resync_hint)
            && let Some(raw) = self.tokens.get(index)
            && fits_regular(raw, bulk::previous_class(previous.kind))
        {
            return LookaheadToken {
                kind: bulk::kind(raw),
                range: TextRange::new(previous.range.end(), self.tokens.start(index + 1).into()),
                has_preceding_line_break: previous.line_break,
                line_break: raw & bulk::LINE_BREAK != 0,
            };
        }
        #[cfg(test)]
        tests::LEXED.set(tests::LEXED.get() + 1);
        let diagnostics = self.lexer.diagnostics_len();
        let kind = self.lexer.lex_at(
            position as usize,
            previous.kind,
            previous.line_break,
            TailwindLexContext::Regular,
        );
        self.lexer.truncate_diagnostics(diagnostics);
        LookaheadToken {
            kind,
            range: self.lexer.current_range(),
            has_preceding_line_break: self.lexer.has_preceding_line_break(),
            line_break: self.lexer.after_newline(),
        }
    }

    /// Creates a checkpoint to which it can later return using [Self::rewind].
    pub fn checkpoint(&self) -> TailwindTokenSourceCheckpoint {
        TailwindTokenSourceCheckpoint {
            current: self.current,
            current_start: self.current_start,
            current_end: self.current_end,
            current_has_line_break: self.current_has_line_break,
            next: self.next,
            after_newline: self.after_newline,
            trivia_len: self.trivia_list.len() as u32,
            diagnostics_len: self.lexer.diagnostics_len() as u32,
        }
    }

    /// Restores the token source to a previous state
    pub fn rewind(&mut self, checkpoint: TailwindTokenSourceCheckpoint) {
        assert!(self.trivia_list.len() >= checkpoint.trivia_len as usize);
        self.trivia_list.truncate(checkpoint.trivia_len as usize);
        self.lexer
            .truncate_diagnostics(checkpoint.diagnostics_len as usize);
        self.current = checkpoint.current;
        self.current_start = checkpoint.current_start;
        self.current_end = checkpoint.current_end;
        self.current_has_line_break = checkpoint.current_has_line_break;
        self.next = checkpoint.next;
        self.after_newline = checkpoint.after_newline;
    }

    /// Lexes the current token again in `context`, as if no token came before it.
    ///
    /// Like [TailwindLexer], this keeps the diagnostics of the current token, and takes the
    /// current token's line break state as the one before it.
    pub fn re_lex_current_in_context(&mut self, context: TailwindLexContext) -> TailwindSyntaxKind {
        let kind = self.re_lex_current(context);
        #[cfg(test)]
        self.record();
        kind
    }

    fn re_lex_current(&mut self, context: TailwindLexContext) -> TailwindSyntaxKind {
        self.current_end = self.current_start;
        self.current = EOF;
        if context == TailwindLexContext::VariantSegment && !self.tokens.is_empty() {
            let source = self.lexer.source().as_bytes();
            let position = usize::from(self.current_start);
            if position < source.len()
                && let Some((raw, end)) = self.tokens.segment_token(source, position)
            {
                self.set_current(raw, end as u32);
                self.next = UNKNOWN;
                return self.current;
            }
        }
        self.lex_with_lexer(context)
    }
}

/// A position of [TailwindTokenSource] to which it can return.
#[derive(Debug)]
pub(crate) struct TailwindTokenSourceCheckpoint {
    current: TailwindSyntaxKind,
    current_start: TextSize,
    current_end: TextSize,
    current_has_line_break: bool,
    next: u32,
    after_newline: bool,
    trivia_len: u32,
    diagnostics_len: u32,
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
        self.bump_with_context(TailwindLexContext::Regular)
    }

    fn skip_as_trivia(&mut self) {
        self.skip_as_trivia_with_context(TailwindLexContext::Regular)
    }

    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        (self.trivia_list, self.lexer.finish())
    }
}

impl BumpWithContext for TailwindTokenSource<'_> {
    type Context = TailwindLexContext;

    #[inline(always)]
    fn bump_with_context(&mut self, context: Self::Context) {
        if self.current() != EOF {
            self.next_non_trivia_token(context, false);
        }
    }

    fn skip_as_trivia_with_context(&mut self, context: Self::Context) {
        if self.current() != EOF {
            self.trivia_list.push(Trivia::new(
                TriviaPieceKind::Skipped,
                self.current_range(),
                false,
            ));

            self.next_non_trivia_token(context, true)
        }
    }
}

impl<'source> NthToken<TailwindLexer<'source>> for TailwindTokenSource<'source> {
    fn nth(&mut self, n: usize) -> TailwindSyntaxKind {
        if n == 0 {
            self.current()
        } else {
            self.lookahead(n).map_or(EOF, |token| token.kind)
        }
    }

    fn nth_range(&mut self, n: usize) -> Option<TextRange> {
        if n == 0 {
            Some(self.current_range())
        } else {
            self.lookahead(n).map(|token| token.range)
        }
    }

    fn has_nth_preceding_line_break(&mut self, n: usize) -> bool {
        if n == 0 {
            self.has_preceding_line_break()
        } else {
            self.lookahead(n)
                .is_some_and(|token| token.has_preceding_line_break)
        }
    }

    /// The Tailwind lexer never reports whitespace before a token.
    fn has_nth_preceding_whitespace(&mut self, _n: usize) -> bool {
        false
    }
}

/// The length from which [TailwindTokenSource] lexes a class string with [BulkTokens]. Lexing
/// a shorter one with [TailwindLexer] alone is as fast, as the bulk lexer costs as much to
/// start as lexing a few tokens.
const MIN_BULK_TOKENS_LEN: usize = 32;

/// Whether [TailwindTokenSource] lexes `source` with [BulkTokens] rather than with
/// [TailwindLexer] alone.
#[inline(always)]
fn uses_bulk_tokens(source: &str) -> bool {
    #[cfg(test)]
    if let Some(bulk_tokens) = tests::BULK_TOKENS.get() {
        return bulk_tokens;
    }
    source.len() >= MIN_BULK_TOKENS_LEN
}

#[cfg(test)]
mod tests {
    use super::{TailwindLexContext, TailwindTokenSource, fits_other, fits_regular};
    use crate::lexer::bulk::tests::{fixtures, mutate, random_fragments, xorshift};
    use crate::lexer::bulk::{self, BulkTokens, previous_class};
    use crate::lexer::{BASENAME_STORE, BaseNameStore, TailwindLexer};
    use crate::{TailwindParserOptions, parse_tailwind_with_options};
    use biome_parser::diagnostic::ParseDiagnostic;
    use biome_parser::lexer::{BufferedLexer, Lexer, LexerCheckpoint};
    use biome_parser::prelude::BumpWithContext;
    use biome_parser::token_source::{NthToken, TokenSource, Trivia};
    use biome_rowan::NodeCache;
    use biome_rowan::TriviaPieceKind;
    use biome_tailwind_syntax::T;
    use biome_tailwind_syntax::TailwindSyntaxKind::*;
    use biome_tailwind_syntax::{TailwindSyntaxKind, TextRange};
    use std::cell::{Cell, RefCell};

    thread_local! {
        /// Whether [super::TailwindTokenSource] uses the bulk tokens whatever the length of the
        /// source, so that tests can compare the bulk tokens with the lexer.
        pub(super) static BULK_TOKENS: Cell<Option<bool>> = const { Cell::new(None) };

        /// How many tokens [super::TailwindTokenSource] lexed with the lexer.
        pub(super) static LEXED: Cell<usize> = const { Cell::new(0) };

        /// Every token that [super::TailwindTokenSource] made current or returned from lookahead,
        /// with whether a line break precedes it.
        pub(super) static TOKENS: RefCell<Vec<(TailwindSyntaxKind, TextRange, bool)>> =
            const { RefCell::new(Vec::new()) };
    }

    /// Asserts that parsing `source` gives the same tree and diagnostics with the bulk tokens as
    /// with the lexer alone.
    fn check_with(source: &str, options: TailwindParserOptions) {
        let parse_with = |without_bulk_tokens: bool| {
            BULK_TOKENS.set(Some(!without_bulk_tokens));
            TOKENS.take();
            let parse = parse_tailwind_with_options(source, &mut NodeCache::default(), options);
            BULK_TOKENS.set(None);
            (
                format!("{:#?}", parse.syntax()),
                format!("{:?}", parse.diagnostics()),
                TOKENS.take(),
            )
        };
        let expected = parse_with(true);
        let actual = parse_with(false);
        assert!(expected.0 == actual.0, "the trees differ for {source:?}");
        assert!(
            expected.1 == actual.1,
            "the diagnostics differ for {source:?}"
        );
        assert!(expected.2 == actual.2, "the tokens differ for {source:?}");
    }

    fn check(source: &str) {
        check_with(source, TailwindParserOptions::default());
    }

    /// Parses every fixture, and copies of each with a few edits that break them, with and
    /// without the bulk tokens.
    #[test]
    fn bulk_tokens_parse_like_the_lexer() {
        let mut next = xorshift(0x2545_F491_4F6C_DD1D);
        for source in fixtures() {
            check(&source);
            for _ in 0..3 {
                check(&mutate(&source, &mut next));
            }
        }
    }

    /// Parses sequences of fragments that the bulk tokens and the parser treat differently.
    #[test]
    fn random_fragments_parse_like_the_lexer() {
        let mut next = xorshift(0x9E37_79B9_7F4A_7C15);
        for _ in 0..3000 {
            check(&random_fragments(&mut next));
        }
    }

    /// Covers the contexts that take some of the bulk tokens, the re-lexing and the lookahead.
    #[test]
    fn contexts_parse_like_the_lexer() {
        for source in [
            "[[data-state=open]_&]:flex [&>div]:p-4 has-[>svg]:p-0 [(x)]:a [[a]b]:c",
            "[color:red] [mask:circle_at_center] [--x:calc(1px+2%)] [a:url(x)] [:x]",
            "-mt-4 -translate-x-1/2 !-m-4 -[x] -(--y) -@x -*",
            "bg-[url(a.png)] bg-[url(\'a\')] bg-[url( a )] bg-[url()] bg-[url(var(--x))]",
            "bg-(color:--brand) bg-(--a:b) w-(--x) text-(length:var(--y))/50",
            "grid-cols-[repeat(3,minmax(0,1fr))] w-[calc(100%-1rem)] aspect-[16/9] w-[100svh-var(--h)]",
            "group-hover/menu:flex @sm/main:p-4 @[400px]:flex *:p-1 **:m-0 data-[a=b]:x",
            "hover:\nfocus:\r\nflex\n\x0Bp-4",
            "bg-[url(\nx)] bg-[url(\r\n)] bg-[url( \n x )] bg-(url(\nx))",
            "bg-red-500/[0.5] text-[#555]/[.5] shadow-[0_1px_2px_black] content-['a_b']",
        ] {
            check(source);
        }
    }

    /// Asserts that wherever [fits_regular] and [fits_other] take a regular bulk token, the
    /// lexer returns the same token, for every context, first byte and token before it.
    #[test]
    fn bulk_tokens_fit_where_the_lexer_agrees() {
        // Sources that end with a token of each kind that leads to a different token after it.
        let prefixes = ["", "[", "(", "a-", "a/", "a:", " ", "]", "!", "a/b-"];
        let previous_kinds = [
            EOF,
            T!['['],
            T!['('],
            T![-],
            T![/],
            T![:],
            WHITESPACE,
            TW_BASE,
            T![']'],
            IDENT,
            CSS_DIMENSION_VALUE,
            TW_SELECTOR,
        ];
        let mut texts: Vec<String> = (0x20u8..0x7F)
            .map(|byte| format!("{}ab-c]x/y:z", byte as char))
            .collect();
        texts.extend(
            [
                "\tx",
                "\u{e9}x",
                "[[a]]",
                "((a))",
                "1.5px",
                "-1",
                "--a",
                "url(x)",
                "\x0Bx",
                "\nx",
                "\u{FEFF}a",
                "",
                "a",
                "1",
                ".5",
            ]
            .map(String::from),
        );
        let contexts = [
            TailwindLexContext::Regular,
            TailwindLexContext::SawNegative,
            TailwindLexContext::ArbitraryVariant,
            TailwindLexContext::ArbitraryCandidate,
            TailwindLexContext::VariantSegment,
            TailwindLexContext::CssValue,
            TailwindLexContext::CssUrlRawValue,
        ];
        for prefix in prefixes {
            for text in &texts {
                let source = format!("{prefix}{text}");
                let tokens = BulkTokens::new(&source, &BASENAME_STORE);
                let Some((_, end, raw)) = tokens
                    .iter()
                    .find(|&(start, _, _)| start as usize == prefix.len())
                else {
                    continue;
                };
                let first = source.as_bytes().get(prefix.len()).copied().unwrap_or(0);
                for context in contexts {
                    for previous in previous_kinds {
                        let class = previous_class(previous);
                        let fits = if context == TailwindLexContext::Regular {
                            fits_regular(raw, class)
                        } else {
                            fits_other(context, raw, first, class)
                        };
                        if !fits {
                            continue;
                        }
                        let mut lexer = TailwindLexer::from_str(&source);
                        let kind = lexer.lex_at(prefix.len(), previous, false, context);
                        assert_eq!(
                            (kind, u32::from(lexer.current_range().end())),
                            (bulk::kind(raw), end),
                            "{context:?} after {previous:?} in {source:?}"
                        );
                    }
                }
            }
        }
    }

    /// The token source before the bulk tokens, which lexes every token with
    /// [BufferedLexer](biome_parser::lexer::BufferedLexer), as the reference for
    /// [random_walks_match_the_reference].
    struct ReferenceTokenSource<'source> {
        lexer: BufferedLexer<TailwindSyntaxKind, TailwindLexer<'source>>,
        trivia_list: Vec<Trivia>,
    }

    impl<'source> ReferenceTokenSource<'source> {
        fn from_str(source: &'source str) -> Self {
            let lexer = TailwindLexer::from_str(source).with_base_names(&BASENAME_STORE);
            let mut source = Self {
                lexer: BufferedLexer::new(lexer),
                trivia_list: vec![],
            };
            source.next_non_trivia_token(TailwindLexContext::Regular, true);
            source
        }

        fn next_non_trivia_token(&mut self, context: TailwindLexContext, first_token: bool) {
            let trailing = !first_token;
            loop {
                let kind = self.lexer.next_token(context);
                let Ok(trivia_kind) = TriviaPieceKind::try_from(kind) else {
                    break;
                };
                let range = self.lexer.current_range();
                self.trivia_list
                    .push(Trivia::new(trivia_kind, range, trailing));
            }
        }
    }

    /// The operations of a token source that the parser uses.
    trait Walk {
        type Checkpoint;
        fn current(&self) -> TailwindSyntaxKind;
        fn current_range(&self) -> TextRange;
        fn has_preceding_line_break(&self) -> bool;
        fn nth(&mut self, n: usize) -> (TailwindSyntaxKind, Option<TextRange>, bool);
        fn checkpoint(&self) -> Self::Checkpoint;
        fn rewind(&mut self, checkpoint: Self::Checkpoint);
        fn re_lex(&mut self, context: TailwindLexContext);
        fn bump(&mut self, context: TailwindLexContext);
        fn skip(&mut self, context: TailwindLexContext);
        fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>);
    }

    impl Walk for TailwindTokenSource<'_> {
        type Checkpoint = super::TailwindTokenSourceCheckpoint;
        fn current(&self) -> TailwindSyntaxKind {
            TokenSource::current(self)
        }
        fn current_range(&self) -> TextRange {
            TokenSource::current_range(self)
        }
        fn has_preceding_line_break(&self) -> bool {
            TokenSource::has_preceding_line_break(self)
        }
        fn nth(&mut self, n: usize) -> (TailwindSyntaxKind, Option<TextRange>, bool) {
            (
                NthToken::nth(self, n),
                self.nth_range(n),
                self.has_nth_preceding_line_break(n),
            )
        }
        fn checkpoint(&self) -> Self::Checkpoint {
            TailwindTokenSource::checkpoint(self)
        }
        fn rewind(&mut self, checkpoint: Self::Checkpoint) {
            TailwindTokenSource::rewind(self, checkpoint);
        }
        fn re_lex(&mut self, context: TailwindLexContext) {
            self.re_lex_current_in_context(context);
        }
        fn bump(&mut self, context: TailwindLexContext) {
            self.bump_with_context(context);
        }
        fn skip(&mut self, context: TailwindLexContext) {
            self.skip_as_trivia_with_context(context);
        }
        fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
            TokenSource::finish(self)
        }
    }

    impl Walk for ReferenceTokenSource<'_> {
        type Checkpoint = (LexerCheckpoint<TailwindSyntaxKind>, usize);
        fn current(&self) -> TailwindSyntaxKind {
            self.lexer.current()
        }
        fn current_range(&self) -> TextRange {
            self.lexer.current_range()
        }
        fn has_preceding_line_break(&self) -> bool {
            self.lexer.has_preceding_line_break()
        }
        fn nth(&mut self, n: usize) -> (TailwindSyntaxKind, Option<TextRange>, bool) {
            let token = self.lexer.nth_non_trivia(n);
            (
                token.as_ref().map_or(EOF, |token| token.kind()),
                token.as_ref().map(|token| token.text_range()),
                token.is_some_and(|token| token.has_preceding_line_break()),
            )
        }
        fn checkpoint(&self) -> Self::Checkpoint {
            (self.lexer.checkpoint(), self.trivia_list.len())
        }
        fn rewind(&mut self, (checkpoint, trivia_len): Self::Checkpoint) {
            self.trivia_list.truncate(trivia_len);
            self.lexer.rewind(checkpoint);
        }
        fn re_lex(&mut self, context: TailwindLexContext) {
            self.lexer.force_relex_in_context(context);
        }
        fn bump(&mut self, context: TailwindLexContext) {
            if self.current() != EOF {
                self.next_non_trivia_token(context, false);
            }
        }
        fn skip(&mut self, context: TailwindLexContext) {
            if self.current() != EOF {
                let range = self.current_range();
                self.trivia_list
                    .push(Trivia::new(TriviaPieceKind::Skipped, range, false));
                self.next_non_trivia_token(context, true);
            }
        }
        fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
            (self.trivia_list, self.lexer.finish())
        }
    }

    /// The token source's state after each step of a pseudo-random walk over its source, which
    /// ends by bumping to the end of the source.
    fn walk(mut tokens: impl Walk, seed: u64) -> Vec<String> {
        let contexts = [
            TailwindLexContext::Regular,
            TailwindLexContext::Regular,
            TailwindLexContext::Regular,
            TailwindLexContext::VariantSegment,
            TailwindLexContext::VariantSegment,
            TailwindLexContext::SawNegative,
            TailwindLexContext::ArbitraryVariant,
            TailwindLexContext::ArbitraryCandidate,
            TailwindLexContext::CssValue,
            TailwindLexContext::CssUrlRawValue,
        ];
        let mut next = xorshift(seed);
        let mut checkpoints = Vec::new();
        let mut log = Vec::new();
        for _ in 0..300 {
            log.push(format!(
                "{:?} {:?} {}",
                tokens.current(),
                tokens.current_range(),
                tokens.has_preceding_line_break()
            ));
            let context = contexts[(next() % contexts.len() as u64) as usize];
            match next() % 12 {
                0 => {
                    for n in 1..4 {
                        log.push(format!("nth {n}: {:?}", tokens.nth(n)));
                    }
                }
                1 => checkpoints.push(tokens.checkpoint()),
                2 => {
                    if let Some(checkpoint) = checkpoints.pop() {
                        tokens.rewind(checkpoint);
                    }
                }
                3 => {
                    if tokens.current() != EOF {
                        tokens.re_lex(context);
                    }
                }
                4 => tokens.skip(context),
                _ => {
                    if tokens.current() == EOF && checkpoints.is_empty() {
                        break;
                    }
                    tokens.bump(context);
                }
            }
        }
        while tokens.current() != EOF {
            tokens.bump(TailwindLexContext::Regular);
        }
        let (trivia, diagnostics) = tokens.finish();
        log.push(format!("{trivia:?}"));
        log.push(format!("{diagnostics:?}"));
        log
    }

    /// Walks the token source over sources with pseudo-random contexts, lookahead, checkpoints,
    /// rewinds and re-lexing, and compares it with [ReferenceTokenSource].
    #[test]
    fn random_walks_match_the_reference() {
        let mut next = xorshift(0x5851_F42D_4C95_7F2D);
        let mut sources = fixtures();
        sources.extend((0..2000).map(|_| random_fragments(&mut next)));
        for source in sources {
            for _ in 0..3 {
                let seed = next() | 1;
                let expected = walk(ReferenceTokenSource::from_str(&source), seed);
                for bulk_tokens in [true, false] {
                    BULK_TOKENS.set(Some(bulk_tokens));
                    let tokens =
                        TailwindTokenSource::from_str(&source, TailwindParserOptions::default());
                    BULK_TOKENS.set(None);
                    assert_eq!(
                        expected,
                        walk(tokens, seed),
                        "{source:?} with seed {seed:#x}, bulk tokens: {bulk_tokens}"
                    );
                }
            }
        }
    }

    /// Asserts that class strings without arbitrary values, CSS variables or negative values,
    /// whose tokens the bulk lexer doesn't predict, take every token from the bulk tokens.
    #[test]
    fn typical_classes_take_bulk_tokens() {
        for source in [
            "flex items-center justify-between px-4 py-2 hover:bg-red-500 md:w-1/2",
            "group-hover/menu:flex peer-checked:bg-blue-500/50 dark:md:hover:text-white",
            "[&>div]:p-4 data-[state=open]:flex @sm:grid-cols-2 *:p-1 flex! !p-0",
        ] {
            check(source);
            assert!(source.len() >= super::MIN_BULK_TOKENS_LEN);
            LEXED.set(0);
            parse_tailwind_with_options(
                source,
                &mut NodeCache::default(),
                TailwindParserOptions::default(),
            );
            assert_eq!(LEXED.get(), 0, "{source:?}");
        }
    }

    #[test]
    fn custom_base_names_parse_like_the_lexer() {
        let base_names = BaseNameStore::new(&["slide-in-from-top", "bg-red", "data-x"]);
        check_with(
            "slide-in-from-top-4 bg-red-500 data-x-1 hover:slide-in-from-top-2 data-[a]:flex",
            TailwindParserOptions::with_base_names(&base_names),
        );
    }
}
