use super::{
    BulkLexOptions, BulkTokens, DIAGNOSTICS, JSX_ATTRIBUTE_VALUE, REGULAR, TokenDiagnostic,
    UNICODE_ESCAPE, context_flag, kind,
};
use crate::lexer::{JsLexContext, JsLexer, JsReLexContext, TextRange};
use crate::prelude::*;
use biome_js_syntax::JsSyntaxKind::{
    COMMENT, EOF, IDENT, JS_REGEX_LITERAL, JSX_IDENT, MULTILINE_COMMENT, NEWLINE, WHITESPACE,
};
use biome_js_syntax::{JsSyntaxKind, T};
use biome_languages::JsFileSource;
use biome_parser::lexer::{Lexer, TokenFlags};
use biome_parser::token_source::Trivia;
use biome_rowan::{SyntaxKind, TextSize, TriviaPieceKind};

/// Token source for the parser over [BulkTokens], which skips over any non-trivia token.
///
/// The parser's lexing contexts and re-lexing pick among the tokens that [BulkTokens] decided
/// on: a regex reads as the `/` or `/=` that [JsLexer] returns, until the parser re-lexes it.
/// Where the parser asks for a token that the bulk tokens don't have, at an offset where no bulk
/// token starts or in a context they didn't predict, the token source lexes with [JsLexer] until
/// it reaches a bulk token that fits again.
pub struct BulkTokenSource<'l> {
    lexer: JsLexer<'l>,
    tokens: BulkTokens,

    current: JsSyntaxKind,
    current_start: TextSize,
    current_end: TextSize,
    current_has_line_break: bool,
    current_has_unicode_escape: bool,
    /// The length of `diagnostics` before the diagnostics of the current token.
    current_diagnostics: u32,

    /// Index of the bulk token that starts at `current_end`, or [NO_TOKEN] if none does.
    ///
    /// Not an `Option`: writing an `Option<u32>` takes two stores, which the next read can't
    /// forward from.
    next: u32,

    /// When the current token is the `/` or `/=` of a regex that the bulk tokens decided on,
    /// the index of that regex, else [NO_TOKEN].
    regex: u32,

    /// Whether a line break separates the last non-trivia token from `current_end`.
    after_newline: bool,

    /// The index of a bulk token at or before `current_end`, where [BulkTokens::find_from]
    /// starts searching when the token source lexes with [JsLexer].
    resync_hint: u32,

    /// The last token that [Self::lookahead] returned, so that scanning ahead one token at a
    /// time doesn't start over from the current token for every token.
    lookahead: LookaheadCursor,

    /// The diagnostics of the tokens up to the current one.
    diagnostics: Vec<ParseDiagnostic>,

    /// List of the skipped trivia. Needed to construct the CST and compute the non-trivia token offsets.
    pub(crate) trivia_list: Vec<Trivia>,
}

/// The value of [BulkTokenSource::next] when no bulk token starts at the current token's end.
const NO_TOKEN: u32 = u32::MAX;

/// Where [BulkTokenSource::lookahead] stopped. Lookahead reads the tokens after the current one
/// as [JsLexContext::Regular] does, so it depends only on where the current token ends.
#[derive(Clone, Copy)]
struct LookaheadCursor {
    /// The end of the current token when the lookahead ran.
    from: TextSize,
    /// How many non-trivia tokens after the current one the lookahead read, or zero if none.
    count: usize,
    /// The last of those tokens.
    token: LookaheadToken,
    /// Index of the bulk token after it, or [NO_TOKEN].
    next: u32,
}

/// A token returned by lookahead.
#[derive(Clone, Copy)]
struct LookaheadToken {
    kind: JsSyntaxKind,
    range: TextRange,
    has_preceding_line_break: bool,
}

/// Returns the trivia kind of `kind`, if it's trivia.
#[inline(always)]
fn trivia_kind(kind: JsSyntaxKind) -> Option<TriviaPieceKind> {
    match kind {
        NEWLINE => Some(TriviaPieceKind::Newline),
        WHITESPACE => Some(TriviaPieceKind::Whitespace),
        COMMENT => Some(TriviaPieceKind::SingleLineComment),
        MULTILINE_COMMENT => Some(TriviaPieceKind::MultiLineComment),
        _ => None,
    }
}

impl<'l> BulkTokenSource<'l> {
    /// Creates a new token source for the given string, lexing the given source ranges as Grit
    /// metavariables.
    pub(crate) fn new(
        source: &'l str,
        source_type: &JsFileSource,
        metavariables: &'l [TextRange],
    ) -> Self {
        let mut lexer = JsLexer::from_str(source).with_metavariables(metavariables);
        // `BulkTokens` doesn't lex metavariables, so the lexer lexes every token around them.
        let tokens = if metavariables.is_empty() {
            BulkTokens::new(&mut lexer, BulkLexOptions::new(source_type))
        } else {
            BulkTokens::new_empty(TextSize::of(source))
        };

        let mut source = Self {
            lexer,
            next: if tokens.start(0) == 0 { 0 } else { NO_TOKEN },
            tokens,
            current: JsSyntaxKind::TOMBSTONE,
            current_start: TextSize::from(0),
            current_end: TextSize::from(0),
            current_has_line_break: false,
            current_has_unicode_escape: false,
            current_diagnostics: 0,
            regex: NO_TOKEN,
            after_newline: false,
            resync_hint: 0,
            lookahead: LookaheadCursor {
                from: TextSize::from(0),
                count: 0,
                token: LookaheadToken {
                    kind: EOF,
                    range: TextRange::default(),
                    has_preceding_line_break: false,
                },
                next: NO_TOKEN,
            },
            diagnostics: Vec::new(),
            trivia_list: vec![],
        };

        source.next_non_trivia_token(JsLexContext::default(), true);
        source
    }

    /// Whether the parser can take the bulk token `raw` as is when it asks for a token in
    /// `context`.
    #[inline(always)]
    fn accepts(&self, raw: u16, context: JsLexContext) -> bool {
        match context_flag(context) {
            Some(JSX_ATTRIBUTE_VALUE) => {
                // An attribute value that isn't quoted lexes as in the regular context.
                raw & JSX_ATTRIBUTE_VALUE != 0
                    || (raw & REGULAR != 0
                        && !matches!(
                            self.lexer
                                .source()
                                .as_bytes()
                                .get(usize::from(self.current_end)),
                            Some(b'"' | b'\'')
                        ))
            }
            Some(flag) => raw & flag != 0,
            None => false,
        }
    }

    #[inline]
    fn next_non_trivia_token(&mut self, context: JsLexContext, first_token: bool) {
        let mut trailing = !first_token;
        let flag = context_flag(context).unwrap_or(0);

        loop {
            // Take the bulk tokens that fit, without going through `self` for each.
            let mut index = self.next;
            let mut position = self.current_end;
            let mut after_newline = self.after_newline;
            while let Some(raw) = self.tokens.get(index)
                && raw & flag != 0
                && raw & DIAGNOSTICS == 0
                && kind(raw) != JS_REGEX_LITERAL
            {
                let kind = kind(raw);
                let end = TextSize::from(self.tokens.start(index + 1));
                index += 1;
                let Some(trivia_kind) = trivia_kind(kind) else {
                    self.current = kind;
                    self.current_start = position;
                    self.current_end = end;
                    self.current_has_line_break = after_newline;
                    self.current_has_unicode_escape = raw & UNICODE_ESCAPE != 0;
                    self.current_diagnostics = self.diagnostics.len() as u32;
                    self.regex = NO_TOKEN;
                    self.next = index;
                    self.after_newline = false;
                    return;
                };
                // Trivia after and including the newline is considered the leading trivia of the next token
                if trivia_kind.is_newline() {
                    trailing = false;
                }
                after_newline |= matches!(
                    trivia_kind,
                    TriviaPieceKind::Newline | TriviaPieceKind::MultiLineComment
                );
                self.trivia_list.push(Trivia::new(
                    trivia_kind,
                    TextRange::new(position, end),
                    trailing,
                ));
                position = end;
            }
            self.next = index;
            self.current_end = position;
            self.after_newline = after_newline;

            let kind = self.next_token_slow(context);
            let Some(trivia_kind) = trivia_kind(kind) else {
                return;
            };
            if trivia_kind.is_newline() {
                trailing = false;
            }
            self.trivia_list
                .push(Trivia::new(trivia_kind, self.current_range(), trailing));
        }
    }

    /// Moves to the token at `current_end`, lexed in `context`.
    fn next_token(&mut self, context: JsLexContext) -> JsSyntaxKind {
        if let Some(raw) = self.tokens.get(self.next)
            && raw & DIAGNOSTICS == 0
            && kind(raw) != JS_REGEX_LITERAL
            && self.accepts(raw, context)
        {
            return self.take_bulk_token(raw, context);
        }
        self.next_token_slow(context)
    }

    /// Makes the bulk token `raw` at `next` the current token.
    fn take_bulk_token(&mut self, raw: u16, context: JsLexContext) -> JsSyntaxKind {
        let kind = kind(raw);
        let index = self.next;
        self.current_start = self.current_end;
        self.current_has_line_break = self.after_newline;
        self.current_has_unicode_escape = raw & UNICODE_ESCAPE != 0;
        self.current_diagnostics = self.diagnostics.len() as u32;
        if kind == JS_REGEX_LITERAL {
            // The parser reads the `/` or `/=` until it re-lexes it into the regex.
            let source = self.lexer.source().as_bytes();
            let (slash, len) = if source[usize::from(self.current_start) + 1] == b'=' {
                (T![/=], 2)
            } else {
                (T![/], 1)
            };
            self.current = slash;
            self.current_end = self.current_start + TextSize::from(len);
            self.resync(self.current_end);
            self.regex = index;
            self.after_newline = false;
            return slash;
        }
        self.regex = NO_TOKEN;
        self.current = kind;
        self.current_end = TextSize::from(self.tokens.start(index + 1));
        if raw & DIAGNOSTICS != 0 {
            let tagged = matches!(context, JsLexContext::TemplateElement { tagged: true });
            let reported = self.tokens.diagnostics_at(self.current_start);
            self.diagnostics.extend(
                reported
                    .iter()
                    .filter(|diagnostic| !tagged || diagnostic.in_tagged_template)
                    .map(|TokenDiagnostic { diagnostic, .. }| diagnostic.clone()),
            );
        }
        self.next = index + 1;
        if !kind.is_trivia() {
            self.after_newline = false;
        } else if matches!(kind, NEWLINE | MULTILINE_COMMENT) {
            self.after_newline = true;
        }
        kind
    }

    /// Moves to the token at `current_end`, lexed in `context`, when it isn't the next bulk
    /// token without diagnostics.
    #[inline(never)]
    fn next_token_slow(&mut self, context: JsLexContext) -> JsSyntaxKind {
        if let Some(raw) = self.tokens.get(self.next)
            && self.accepts(raw, context)
        {
            return self.take_bulk_token(raw, context);
        }
        let source = self.lexer.source().as_bytes();
        let at = usize::from(self.current_end);
        if context == JsLexContext::Regular
            && source.get(at) == Some(&b'<')
            && !matches!(source.get(at + 1), Some(b'<' | b'='))
        {
            // The second `<` of a `<<` that the parser split for type arguments.
            self.regex = NO_TOKEN;
            self.current = T![<];
            self.current_start = self.current_end;
            self.current_end = self.current_start + TextSize::from(1);
            self.current_has_line_break = self.after_newline;
            self.current_has_unicode_escape = false;
            self.current_diagnostics = self.diagnostics.len() as u32;
            self.after_newline = false;
            self.resync(self.current_end);
            return T![<];
        }
        self.lex_with_lexer(context)
    }

    /// Lexes the token at `current_end` with [JsLexer].
    #[inline(never)]
    fn lex_with_lexer(&mut self, context: JsLexContext) -> JsSyntaxKind {
        let kind = self
            .lexer
            .lex_at(self.current_end.into(), self.after_newline, context);
        self.current_diagnostics = self.diagnostics.len() as u32;
        self.take_lexer_token(kind)
    }

    /// Makes the lexer's current token the current token, with its diagnostics.
    fn take_lexer_token(&mut self, kind: JsSyntaxKind) -> JsSyntaxKind {
        self.regex = NO_TOKEN;
        let range = self.lexer.current_range();
        self.current = kind;
        self.current_start = range.start();
        self.current_end = range.end();
        let flags = self.lexer.current_flags();
        self.current_has_line_break = flags.has_preceding_line_break();
        self.current_has_unicode_escape = flags.has_unicode_escape();
        self.after_newline = self.lexer.after_newline();
        self.diagnostics
            .extend(self.lexer.take_diagnostics_after(0));
        self.resync(range.end());
        kind
    }

    /// Points `next` at the bulk token starting at `offset`, if there's one.
    fn resync(&mut self, offset: TextSize) {
        if self.next != NO_TOKEN {
            self.resync_hint = self.next;
        }
        self.next = self
            .tokens
            .find_from(offset, self.resync_hint)
            .unwrap_or(NO_TOKEN);
        if self.next != NO_TOKEN {
            self.resync_hint = self.next;
        }
    }

    #[inline(always)]
    pub(crate) fn has_unicode_escape(&self) -> bool {
        self.current_has_unicode_escape
    }

    /// Returns `true` if the next token has any preceding trivia (either trailing trivia of the current
    /// token or leading trivia of the next token)
    pub(crate) fn has_next_preceding_trivia(&mut self) -> bool {
        if self.current == EOF {
            return false;
        }
        if let Some(raw) = self.tokens.get(self.next)
            && raw & REGULAR != 0
        {
            return kind(raw).is_trivia();
        }
        self.lex_lookahead_token(self.current_end, self.after_newline)
            .0
            .is_trivia()
    }

    /// Returns the `n`th non-trivia token after the current one, as [JsLexContext::Regular]
    /// lexes them: a regex that the bulk tokens decided on reads as its `/` or `/=`.
    fn lookahead(&mut self, n: usize) -> Option<LookaheadToken> {
        debug_assert_ne!(n, 0);
        if self.current == EOF {
            return None;
        }
        let cursor = self.lookahead;
        let (mut count, mut position, mut index) =
            if cursor.from == self.current_end && cursor.count != 0 && cursor.count <= n {
                if cursor.count == n {
                    return Some(cursor.token);
                }
                if cursor.token.kind == EOF {
                    return None;
                }
                (cursor.count, cursor.token.range.end(), cursor.next)
            } else {
                (0, self.current_end, self.next)
            };
        // The current token isn't trivia, so no line break follows it yet.
        let mut after_newline = false;

        loop {
            let (kind, end) = match self.tokens.get(index) {
                Some(raw) if raw & REGULAR != 0 && kind(raw) != JS_REGEX_LITERAL => {
                    index += 1;
                    (kind(raw), TextSize::from(self.tokens.start(index)))
                }
                _ => {
                    let token = self.lex_lookahead_token(position, after_newline);
                    index = self
                        .tokens
                        .find_from(token.1, self.resync_hint)
                        .unwrap_or(NO_TOKEN);
                    token
                }
            };
            if let Some(trivia_kind) = trivia_kind(kind) {
                after_newline |= matches!(
                    trivia_kind,
                    TriviaPieceKind::Newline | TriviaPieceKind::MultiLineComment
                );
                position = end;
                continue;
            }
            count += 1;
            let token = LookaheadToken {
                kind,
                range: TextRange::new(position, end),
                has_preceding_line_break: after_newline,
            };
            if count == n || kind == EOF {
                self.lookahead = LookaheadCursor {
                    from: self.current_end,
                    count,
                    token,
                    next: index,
                };
                return (count == n).then_some(token);
            }
            after_newline = false;
            position = end;
        }
    }

    /// Lexes the token at `position` in [JsLexContext::Regular] with [JsLexer], dropping its
    /// diagnostics. Returns its kind and end.
    #[inline(never)]
    fn lex_lookahead_token(
        &mut self,
        position: TextSize,
        after_newline: bool,
    ) -> (JsSyntaxKind, TextSize) {
        let kind = self
            .lexer
            .lex_at(position.into(), after_newline, JsLexContext::Regular);
        self.lexer.take_diagnostics_after(0);
        (kind, self.lexer.current_range().end())
    }

    /// Returns whether a non-trivia token precedes the current one, walking back
    /// over the trivia adjacent to it. Trivia with no earlier token must be
    /// leading, or the tree sink strands it.
    fn preceding_token_exists(&self) -> bool {
        let mut position = self.current_range().start();
        if position == TextSize::from(0) {
            return false;
        }
        for trivia in self.trivia_list.iter().rev() {
            if trivia.offset() + trivia.len() != position {
                break;
            }
            position = trivia.offset();
            if position == TextSize::from(0) {
                return false;
            }
        }
        true
    }

    pub(crate) fn re_lex(&mut self, mode: JsReLexContext) -> JsSyntaxKind {
        let continuation = match mode {
            JsReLexContext::JsxChild { astro } => JsLexContext::JsxChild { astro },
            _ => JsLexContext::Regular,
        };

        let mut kind = match mode {
            // A regex that the bulk tokens decided on is already one.
            JsReLexContext::Regex if self.regex != NO_TOKEN => self.take_regex(),
            JsReLexContext::BinaryOperator => self.re_lex_binary_operator(),
            JsReLexContext::TypeArgumentLessThan => self.re_lex_type_argument_less_than(),
            JsReLexContext::JsxIdentifier => self.re_lex_jsx_identifier(),
            JsReLexContext::AstroJsxAttributeName => self.re_lex_astro_jsx_attribute_name(),
            _ => self.re_lex_with_lexer(mode),
        };
        // Astro HTML comments re-lex into trivia, which the parser must never see as current.
        if TriviaPieceKind::try_from(kind).is_ok() {
            let mut trailing = self.preceding_token_exists() && !self.current_has_line_break;
            while let Ok(trivia_kind) = TriviaPieceKind::try_from(kind) {
                if trivia_kind.is_newline() {
                    trailing = false;
                }
                self.trivia_list
                    .push(Trivia::new(trivia_kind, self.current_range(), trailing));
                kind = self.next_token(continuation);
            }
        }

        kind
    }

    /// Makes the regex that the bulk tokens decided on the current token, in place of its `/`
    /// or `/=`.
    fn take_regex(&mut self) -> JsSyntaxKind {
        let index = self.regex;
        self.regex = NO_TOKEN;
        self.current = JS_REGEX_LITERAL;
        self.current_end = TextSize::from(self.tokens.start(index + 1));
        let reported = self.tokens.diagnostics_at(self.current_start);
        self.diagnostics.extend(
            reported
                .iter()
                .map(|TokenDiagnostic { diagnostic, .. }| diagnostic.clone()),
        );
        self.next = index + 1;
        JS_REGEX_LITERAL
    }

    /// Re-lexes `>` as one of `>>`, `>>>`, `>=`, `>>=` or `>>>=`, as [JsLexer] does, by joining
    /// the bulk tokens that make it up.
    fn re_lex_binary_operator(&mut self) -> JsSyntaxKind {
        let source = self.lexer.source().as_bytes();
        let start = usize::from(self.current_start);
        let byte = |offset: usize| source.get(start + offset).copied();
        if byte(0) != Some(b'>') {
            return self.current;
        }
        let (kind, len) = match (byte(1), byte(2), byte(3)) {
            (Some(b'>'), Some(b'>'), Some(b'=')) => (T![>>>=], 4),
            (Some(b'>'), Some(b'>'), _) => (T![>>>], 3),
            (Some(b'>'), Some(b'='), _) => (T![>>=], 3),
            (Some(b'>'), _, _) => (T![>>], 2),
            (Some(b'='), _, _) => (T![>=], 2),
            _ => (T![>], 1),
        };
        if kind != self.current {
            self.regex = NO_TOKEN;
            self.current = kind;
            self.current_end = self.current_start + TextSize::from(len);
            self.resync(self.current_end);
        }
        kind
    }

    /// Re-lexes `<<` as `<`, as [JsLexer] does.
    fn re_lex_type_argument_less_than(&mut self) -> JsSyntaxKind {
        if self.current == T![<<] {
            self.regex = NO_TOKEN;
            self.current = T![<];
            self.current_end = self.current_start + TextSize::from(1);
            self.resync(self.current_end);
        }
        self.current
    }

    /// Re-lexes an identifier or keyword as a JSX identifier, which continues with `-` and more
    /// identifier parts, as [JsLexer] does, by joining the bulk tokens that make it up.
    fn re_lex_jsx_identifier(&mut self) -> JsSyntaxKind {
        if !(self.current.is_keyword() || self.current == IDENT) {
            return self.current;
        }
        let source = self.lexer.source().as_bytes();
        let mut end = usize::from(self.current_end);
        while let Some(&byte) = source.get(end) {
            match byte {
                b'-' => end += 1,
                byte if byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'$') => end += 1,
                // Escapes and Unicode identifier parts.
                b'\\' | 0x80.. => return self.re_lex_with_lexer(JsReLexContext::JsxIdentifier),
                _ => break,
            }
        }
        self.regex = NO_TOKEN;
        self.current = JSX_IDENT;
        self.current_end = TextSize::from(end as u32);
        self.resync(self.current_end);
        JSX_IDENT
    }

    /// Re-lexes the current token as an Astro attribute name with [JsLexer].
    ///
    /// When [JsLexer] reads a name, it drops every diagnostic that starts where the name does,
    /// which includes an empty one that the previous token reported at its end.
    fn re_lex_astro_jsx_attribute_name(&mut self) -> JsSyntaxKind {
        let source = self.lexer.source().as_bytes();
        if source
            .get(usize::from(self.current_start))
            .is_some_and(|&byte| {
                !matches!(byte, b'=' | b'>' | b'/' | b'{' | b'}' | b'<')
                    && !byte.is_ascii_whitespace()
            })
        {
            let start = self.current_start;
            while self.diagnostics.last().is_some_and(|diagnostic| {
                diagnostic.span().is_some_and(|span| span.start() >= start)
            }) {
                self.diagnostics.pop();
            }
            self.current_diagnostics = self.current_diagnostics.min(self.diagnostics.len() as u32);
        }
        self.re_lex_with_lexer(JsReLexContext::AstroJsxAttributeName)
    }

    #[inline(never)]
    fn re_lex_with_lexer(&mut self, mode: JsReLexContext) -> JsSyntaxKind {
        let mut flags = TokenFlags::empty();
        flags.set(
            TokenFlags::PRECEDING_LINE_BREAK,
            self.current_has_line_break,
        );
        flags.set(TokenFlags::UNICODE_ESCAPE, self.current_has_unicode_escape);
        // Some re-lexing drops the diagnostics of the token it replaces, from the lexer's.
        for diagnostic in self.diagnostics.drain(self.current_diagnostics as usize..) {
            self.lexer.push_diagnostic(diagnostic);
        }
        let kind = self
            .lexer
            .re_lex_at(self.current_range(), self.current, flags, mode);
        if kind == self.current && self.lexer.current_range() == self.current_range() {
            // Unchanged: the lexer may still have reported diagnostics, which it keeps.
            self.diagnostics
                .extend(self.lexer.take_diagnostics_after(0));
            return kind;
        }
        self.take_lexer_token(kind)
    }

    /// Creates a checkpoint to which it can later return using [Self::rewind].
    pub(crate) fn checkpoint(&self) -> BulkTokenSourceCheckpoint {
        BulkTokenSourceCheckpoint {
            current: self.current,
            current_start: self.current_start,
            current_end: self.current_end,
            current_has_line_break: self.current_has_line_break,
            current_has_unicode_escape: self.current_has_unicode_escape,
            current_diagnostics: self.current_diagnostics,
            regex: self.regex,
            next: self.next,
            after_newline: self.after_newline,
            trivia_len: self.trivia_list.len() as u32,
            diagnostics_len: self.diagnostics.len() as u32,
        }
    }

    /// Restores the token source to a previous state
    pub(crate) fn rewind(&mut self, checkpoint: BulkTokenSourceCheckpoint) {
        assert!(self.trivia_list.len() >= checkpoint.trivia_len as usize);
        self.trivia_list.truncate(checkpoint.trivia_len as usize);
        self.diagnostics
            .truncate(checkpoint.diagnostics_len as usize);
        self.current = checkpoint.current;
        self.current_start = checkpoint.current_start;
        self.current_end = checkpoint.current_end;
        self.current_has_line_break = checkpoint.current_has_line_break;
        self.current_has_unicode_escape = checkpoint.current_has_unicode_escape;
        self.current_diagnostics = checkpoint.current_diagnostics;
        self.regex = checkpoint.regex;
        self.next = checkpoint.next;
        self.after_newline = checkpoint.after_newline;
    }
}

/// A position of [BulkTokenSource] to which it can return.
#[derive(Debug)]
pub struct BulkTokenSourceCheckpoint {
    current: JsSyntaxKind,
    current_start: TextSize,
    current_end: TextSize,
    current_has_line_break: bool,
    current_has_unicode_escape: bool,
    current_diagnostics: u32,
    regex: u32,
    next: u32,
    after_newline: bool,
    trivia_len: u32,
    diagnostics_len: u32,
}

#[cfg(feature = "bulk_lexer")]
impl BulkTokenSourceCheckpoint {
    /// Returns the start of the current token.
    pub(crate) fn current_start(&self) -> TextSize {
        self.current_start
    }

    /// Returns the length of the trivia list.
    pub(crate) fn trivia_position(&self) -> usize {
        self.trivia_len as usize
    }
}

impl<'source> TokenSource for BulkTokenSource<'source> {
    type Kind = JsSyntaxKind;

    /// Returns the kind of the current non-trivia token
    #[inline(always)]
    fn current(&self) -> JsSyntaxKind {
        self.current
    }

    /// Returns the range of the current non-trivia token
    #[inline(always)]
    fn current_range(&self) -> TextRange {
        TextRange::new(self.current_start, self.current_end)
    }

    #[inline(always)]
    fn text(&self) -> &'source str {
        self.lexer.source()
    }

    #[inline(always)]
    fn position(&self) -> TextSize {
        self.current_start
    }

    /// Returns true if the current token is preceded by a line break
    #[inline(always)]
    fn has_preceding_line_break(&self) -> bool {
        self.current_has_line_break
    }

    #[inline(always)]
    fn bump(&mut self) {
        self.bump_with_context(JsLexContext::Regular)
    }

    fn skip_as_trivia(&mut self) {
        self.skip_as_trivia_with_context(JsLexContext::Regular)
    }

    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        (self.trivia_list, self.diagnostics)
    }
}

impl BumpWithContext for BulkTokenSource<'_> {
    type Context = JsLexContext;

    #[inline(always)]
    fn bump_with_context(&mut self, context: Self::Context) {
        if self.current() != EOF {
            self.next_non_trivia_token(context, false);
        }
    }

    /// Skips the current token as skipped token trivia
    fn skip_as_trivia_with_context(&mut self, context: JsLexContext) {
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

impl<'source> NthToken<JsLexer<'source>> for BulkTokenSource<'source> {
    /// Gets the kind of the nth non-trivia token
    fn nth(&mut self, n: usize) -> JsSyntaxKind {
        if n == 0 {
            self.current()
        } else {
            self.lookahead(n).map_or(EOF, |token| token.kind)
        }
    }

    /// Gets the range of the nth non-trivia token.
    fn nth_range(&mut self, n: usize) -> Option<TextRange> {
        if n == 0 {
            Some(self.current_range())
        } else {
            self.lookahead(n).map(|token| token.range)
        }
    }

    /// Returns true if the nth non-trivia token is preceded by a line break
    fn has_nth_preceding_line_break(&mut self, n: usize) -> bool {
        if n == 0 {
            self.has_preceding_line_break()
        } else {
            self.lookahead(n)
                .is_some_and(|token| token.has_preceding_line_break)
        }
    }

    /// Returns true if the nth non-trivia token is preceded by whitespace trivia, which the
    /// JavaScript lexer never reports.
    fn has_nth_preceding_whitespace(&mut self, _n: usize) -> bool {
        false
    }
}

#[cfg(all(test, not(feature = "bulk_lexer")))]
mod tests;
