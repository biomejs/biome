//! The parser's token source: the non-trivia tokens that [BulkTokens] decided on, in flat arrays
//! that the parser moves through by index.
//!
//! The parser reads the tokens as the bulk pass decided them, a regex included. Where its grammar
//! needs a token in a lexing context that the stream's token doesn't fit, or re-lexes a token,
//! the token source lexes with [JsLexer] from there and splices the tokens it gets into the
//! stream, up to where they meet the stream's tokens again.
//!
//! The trivia isn't tracked while parsing: [BulkTokenSource::finish] collects it from the gaps
//! between the tokens of the final stream.

use super::stream::{Stream, Token};
use super::{
    BulkLexOptions, BulkTokens, CONTEXTS, DIAGNOSTICS, GAP_DIAGNOSTICS, GAP_SHIFT,
    JSX_ATTRIBUTE_VALUE, LINE_BREAK, REGULAR, TokenDiagnostic, UNICODE_ESCAPE, context_flag, kind,
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
use biome_rowan::{TextSize, TriviaPieceKind};

/// The token source lexed the token with [JsLexer]; its diagnostics, and those of the trivia
/// before it, are in [BulkTokenSource::lexed_diagnostics].
const LEXED: u32 = 1 << 22;
/// The parser skipped the token as trivia.
const SKIPPED: u32 = 1 << 23;

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

/// Returns the bits that a word needs for its token and the trivia before it to fit `context`, or
/// `None` for a context that no stream token is known to fit.
#[inline(always)]
fn needed(context: JsLexContext) -> Option<u32> {
    context_flag(context).map(|flag| {
        let flag = u32::from(flag);
        flag | flag << GAP_SHIFT
    })
}

/// A splice of the stream, which a rewind undoes.
struct Splice {
    pos: usize,
    /// How many tokens the splice put in.
    inserted: usize,
    /// The tokens that it took out.
    removed: Vec<Token>,
}

/// Token source for the parser over the stream of [BulkTokens].
pub struct BulkTokenSource<'l> {
    lexer: JsLexer<'l>,
    /// The diagnostics of the bulk pass.
    bulk: BulkTokens,

    /// The non-trivia tokens, ending with [EOF].
    stream: Stream,
    /// Index of the current token, which is never before the gap of `stream`.
    pos: usize,
    /// The token at `pos`, and its kind.
    current: Token,
    current_kind: JsSyntaxKind,

    /// The length of `diagnostics` before the diagnostics of the token at `current_diagnostics_pos`.
    /// Any other current token arrived without diagnostics.
    current_diagnostics: u32,
    current_diagnostics_pos: usize,
    /// The diagnostics of the tokens up to the current one.
    diagnostics: Vec<ParseDiagnostic>,
    /// The diagnostics of the tokens marked [LEXED] and the trivia before them, by token start,
    /// with the index at which the diagnostics of each lexing of a token start: the latest lexing
    /// of the token at a start is the one in the stream.
    lexed_diagnostics: Vec<(u32, u32, ParseDiagnostic)>,

    /// The splices since the start, for rewinds to undo.
    splices: Vec<Splice>,

    /// The indices of the tokens that the parser skipped as trivia, in order.
    skipped: Vec<u32>,
    /// The trivia between the tokens of the bulk pass.
    trivia: Vec<Trivia>,
    /// Whether the trivia between the tokens of the stream may differ from `trivia`.
    trivia_changed: bool,
    /// The trivia that [JsLexer] lexed, in the contexts that the parser asked for, and the tokens
    /// that re-lexing turned into trivia. Later re-lexing can bring tokens back where such trivia
    /// was, so [Self::trivia] only takes the latest trivia in the gaps of the final stream.
    relexed_trivia: Vec<Trivia>,
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
        let mut source = if metavariables.is_empty() {
            let bulk = BulkTokens::new(&mut lexer, BulkLexOptions::new(source_type));
            Self::from_bulk(lexer, bulk)
        } else {
            // `BulkTokens` doesn't lex metavariables, so the lexer lexes every token.
            Self::lexed(lexer)
        };
        source.arrive(JsLexContext::Regular);
        source
    }

    /// Creates a token source over the tokens of `bulk`.
    fn from_bulk(lexer: JsLexer<'l>, mut bulk: BulkTokens) -> Self {
        let (tokens, trivia) = bulk.take_tokens();
        Self {
            lexer,
            bulk,
            stream: Stream::new(tokens),
            pos: 0,
            current: Token::default(),
            current_kind: EOF,
            current_diagnostics: 0,
            current_diagnostics_pos: 0,
            diagnostics: Vec::new(),
            lexed_diagnostics: Vec::new(),
            splices: Vec::new(),
            skipped: Vec::new(),
            trivia,
            trivia_changed: false,
            relexed_trivia: Vec::new(),
        }
    }

    /// Creates a token source whose tokens [JsLexer] lexes in [JsLexContext::Regular].
    fn lexed(lexer: JsLexer<'l>) -> Self {
        let mut source = Self::from_bulk(lexer, BulkTokens::new_empty());
        source.lex_tokens(None, 0, JsLexContext::Regular);
        source.trivia_changed = true;
        source
    }

    #[inline(always)]
    fn word(&self) -> u32 {
        self.current.word
    }

    /// Reads the token at `pos` into `current`.
    #[inline(always)]
    fn load_current(&mut self) {
        self.current = *self.stream.ahead(self.pos);
        self.current_kind = kind(self.current.word as u16);
    }

    /// The end of the token before the current one, where the trivia before it starts.
    #[inline(always)]
    fn gap_start(&self) -> u32 {
        if self.pos == 0 {
            0
        } else {
            self.stream.get(self.pos - 1).end
        }
    }

    /// The length of `diagnostics` before the diagnostics of the current token.
    fn current_diagnostics(&self) -> usize {
        if self.current_diagnostics_pos == self.pos {
            self.current_diagnostics as usize
        } else {
            self.diagnostics.len()
        }
    }

    fn set_current_diagnostics(&mut self, len: usize) {
        self.current_diagnostics = len as u32;
        self.current_diagnostics_pos = self.pos;
    }

    /// Makes the token at `pos` the current token, as lexed in `context`.
    #[inline(always)]
    fn arrive(&mut self, context: JsLexContext) {
        self.load_current();
        let word = self.word();
        if let Some(needed) = needed(context)
            && word & needed == needed
            && word & (u32::from(DIAGNOSTICS) | GAP_DIAGNOSTICS | LEXED) == 0
        {
            return;
        }
        self.arrive_slow(context);
    }

    #[inline(never)]
    fn arrive_slow(&mut self, context: JsLexContext) {
        if !self.fits(context) {
            self.lex_tokens(None, self.gap_start(), context);
        }
        self.take_diagnostics(context);
    }

    /// Whether the current token and the trivia before it are what [JsLexer] returns in
    /// `context`.
    fn fits(&self, context: JsLexContext) -> bool {
        let word = self.word();
        match context_flag(context) {
            Some(JSX_ATTRIBUTE_VALUE) => {
                // An attribute value that isn't quoted lexes as in the regular context.
                let either = u32::from(JSX_ATTRIBUTE_VALUE | REGULAR);
                let quoted = matches!(
                    self.lexer
                        .source()
                        .as_bytes()
                        .get(self.current.start as usize),
                    Some(b'"' | b'\'')
                );
                (word & u32::from(JSX_ATTRIBUTE_VALUE) != 0
                    || (word & u32::from(REGULAR) != 0 && !quoted))
                    && word & either << GAP_SHIFT != 0
            }
            Some(flag) => {
                let flag = u32::from(flag);
                word & (flag | flag << GAP_SHIFT) == flag | flag << GAP_SHIFT
            }
            None => false,
        }
    }

    /// Adds the diagnostics of the current token and the trivia before it.
    fn take_diagnostics(&mut self, context: JsLexContext) {
        let word = self.word();
        if word & GAP_DIAGNOSTICS != 0 {
            self.diagnostics.extend(
                self.bulk
                    .diagnostics_in(self.gap_start(), self.current.start)
                    .iter()
                    .map(|TokenDiagnostic { diagnostic, .. }| diagnostic.clone()),
            );
        }
        self.set_current_diagnostics(self.diagnostics.len());
        let start = self.current.start;
        if word & LEXED != 0
            && let Some(&(_, lexing, _)) = self
                .lexed_diagnostics
                .iter()
                .rev()
                .find(|(token_start, ..)| *token_start == start)
        {
            self.diagnostics.extend(
                self.lexed_diagnostics[lexing as usize..]
                    .iter()
                    .filter(|(token_start, entry_lexing, _)| {
                        *token_start == start && *entry_lexing == lexing
                    })
                    .map(|(.., diagnostic)| diagnostic.clone()),
            );
        }
        if word & u32::from(DIAGNOSTICS) != 0 {
            let tagged = matches!(context, JsLexContext::TemplateElement { tagged: true });
            self.diagnostics.extend(
                self.bulk
                    .diagnostics_at(TextSize::from(start))
                    .iter()
                    .filter(|diagnostic| !tagged || diagnostic.in_tagged_template)
                    .map(|TokenDiagnostic { diagnostic, .. }| diagnostic.clone()),
            );
        }
    }

    /// Puts tokens that [JsLexer] lexes in place of the stream's tokens from `pos`: `first`, if
    /// any, then the tokens from `position`, the first non-trivia one in `context` and the rest
    /// in [JsLexContext::Regular], until the lexer reaches the start of a later stream token.
    #[inline(never)]
    fn lex_tokens(&mut self, first: Option<Token>, mut position: u32, context: JsLexContext) {
        let mut tokens: Vec<_> = first.into_iter().collect();
        let mut context = if tokens.is_empty() {
            context
        } else {
            JsLexContext::Regular
        };
        let mut gap_flags = 0;
        let mut gap_diagnostics = Vec::new();
        let mut next = self.pos;
        let len = self.stream.len();
        loop {
            while next < len && self.stream.get(next).start < position {
                next += 1;
            }
            if !tokens.is_empty()
                && next > self.pos
                && self.stream.get(next - 1).end == position
                && gap_flags == 0
                && gap_diagnostics.is_empty()
            {
                // A replaced token ended here, so the trivia after it and the stream's tokens
                // from `next` on fit as they are.
                break;
            }
            if !tokens.is_empty() && next < len && self.stream.get(next).start == position {
                // The stream's tokens fit again from `next` on.
                let word = &mut self.stream.get_mut(next).word;
                *word &= !(LINE_BREAK | CONTEXTS << GAP_SHIFT | GAP_DIAGNOSTICS);
                *word |= gap_flags | u32::from(REGULAR) << GAP_SHIFT;
                if !gap_diagnostics.is_empty() {
                    *word |= LEXED;
                    self.set_lexed_diagnostics(position, gap_diagnostics);
                }
                break;
            }
            let kind = self
                .lexer
                .lex_at(position as usize, gap_flags != 0, context);
            let range = self.lexer.current_range();
            gap_diagnostics.extend(self.lexer.take_diagnostics_after(0));
            let (start, end) = (u32::from(range.start()), u32::from(range.end()));
            position = end;
            if let Some(trivia_kind) = trivia_kind(kind) {
                if matches!(kind, NEWLINE | MULTILINE_COMMENT) {
                    gap_flags = LINE_BREAK;
                }
                self.relexed_trivia
                    .push(Trivia::new(trivia_kind, range, false));
                self.trivia_changed = true;
                continue;
            }
            let flags = self.lexer.current_flags();
            let context_bits = u32::from(context_flag(context).unwrap_or(0));
            let mut word = kind as u32 | context_bits | context_bits << GAP_SHIFT | gap_flags;
            if flags.has_unicode_escape() {
                word |= u32::from(UNICODE_ESCAPE);
            }
            if !gap_diagnostics.is_empty() {
                word |= LEXED;
                self.set_lexed_diagnostics(start, std::mem::take(&mut gap_diagnostics));
            }
            tokens.push(Token { word, start, end });
            gap_flags = 0;
            context = JsLexContext::Regular;
            if kind == EOF {
                next = len;
                break;
            }
        }
        let removed = self
            .stream
            .splice(self.pos, next.min(len) - self.pos, &tokens);
        self.load_current();
        // The replaced tokens had trivia between them, or before the first token of a repair.
        if removed.windows(2).any(|pair| pair[1].start > pair[0].end)
            || removed
                .first()
                .zip(tokens.first())
                .is_some_and(|(old, new)| old.start != new.start)
        {
            self.trivia_changed = true;
        }
        self.splices.push(Splice {
            pos: self.pos,
            inserted: tokens.len(),
            removed,
        });
    }

    /// Records the diagnostics of the token lexed at `start`, in place of those of any earlier
    /// lexing there.
    fn set_lexed_diagnostics(&mut self, start: u32, diagnostics: Vec<ParseDiagnostic>) {
        let lexing = self.lexed_diagnostics.len() as u32;
        self.lexed_diagnostics.extend(
            diagnostics
                .into_iter()
                .map(|diagnostic| (start, lexing, diagnostic)),
        );
    }

    /// Replaces the current token with `kind` over `start..end`, followed by the tokens that
    /// [JsLexer] lexes after it.
    fn replace_current(&mut self, kind: JsSyntaxKind, start: u32, end: u32, flags: u32) {
        let word = kind as u32
            | u32::from(REGULAR)
            | (self.word() & (LINE_BREAK | CONTEXTS << GAP_SHIFT))
            | flags;
        self.lex_tokens(Some(Token { word, start, end }), end, JsLexContext::Regular);
    }

    #[inline(always)]
    pub(crate) fn has_unicode_escape(&self) -> bool {
        self.word() & u32::from(UNICODE_ESCAPE) != 0
    }

    /// Returns `true` if the next token has any preceding trivia (either trailing trivia of the current
    /// token or leading trivia of the next token)
    pub(crate) fn has_next_preceding_trivia(&self) -> bool {
        self.stream
            .get_ahead(self.pos + 1)
            .is_some_and(|next| next.start > self.current.end)
    }

    /// Returns the number of tokens in the stream.
    pub(crate) fn token_count(&self) -> usize {
        self.stream.len()
    }

    /// Returns the range of the token at `index`.
    pub(crate) fn token_range(&self, index: usize) -> TextRange {
        let token = self.stream.get(index);
        TextRange::new(TextSize::from(token.start), TextSize::from(token.end))
    }

    /// Whether the parser skipped the token at `index` as trivia.
    pub(crate) fn is_skipped(&self, index: usize) -> bool {
        self.stream.get(index).word & SKIPPED != 0
    }

    pub(crate) fn re_lex(&mut self, mode: JsReLexContext) -> JsSyntaxKind {
        match mode {
            JsReLexContext::Regex => self.re_lex_regex(),
            JsReLexContext::BinaryOperator => self.re_lex_binary_operator(),
            JsReLexContext::TypeArgumentLessThan => self.re_lex_type_argument_less_than(),
            JsReLexContext::JsxIdentifier => self.re_lex_jsx_identifier(),
            JsReLexContext::AstroJsxAttributeName => self.re_lex_astro_jsx_attribute_name(),
            _ => self.re_lex_with_lexer(mode),
        }
    }

    /// Re-lexes a `/` or `/=` as a regex, unless the bulk pass already decided on one.
    fn re_lex_regex(&mut self) -> JsSyntaxKind {
        if matches!(self.current(), T![/] | T![/=]) {
            self.re_lex_with_lexer(JsReLexContext::Regex)
        } else {
            self.current()
        }
    }

    /// Re-lexes `>` as one of `>>`, `>>>`, `>=`, `>>=` or `>>>=`, as [JsLexer] does, and a regex
    /// that the bulk pass decided on as the `/` or `/=` that starts it.
    fn re_lex_binary_operator(&mut self) -> JsSyntaxKind {
        let source = self.lexer.source().as_bytes();
        let start = self.current.start;
        let byte = |offset: u32| source.get((start + offset) as usize).copied();
        let (kind, len) = match self.current() {
            T![>] => match (byte(1), byte(2), byte(3)) {
                (Some(b'>'), Some(b'>'), Some(b'=')) => (T![>>>=], 4),
                (Some(b'>'), Some(b'>'), _) => (T![>>>], 3),
                (Some(b'>'), Some(b'='), _) => (T![>>=], 3),
                (Some(b'>'), _, _) => (T![>>], 2),
                (Some(b'='), _, _) => (T![>=], 2),
                _ => return T![>],
            },
            JS_REGEX_LITERAL => {
                // The regex's diagnostics don't apply to the operator.
                self.diagnostics.truncate(self.current_diagnostics());
                if byte(1) == Some(b'=') {
                    (T![/=], 2)
                } else {
                    (T![/], 1)
                }
            }
            kind => return kind,
        };
        self.replace_current(kind, start, start + len, 0);
        kind
    }

    /// Re-lexes `<<` as `<`, as [JsLexer] does.
    fn re_lex_type_argument_less_than(&mut self) -> JsSyntaxKind {
        if self.current() == T![<<] {
            let start = self.current.start;
            self.replace_current(T![<], start, start + 1, 0);
        }
        self.current()
    }

    /// Re-lexes an identifier or keyword as a JSX identifier, which continues with `-` and more
    /// identifier parts, as [JsLexer] does.
    fn re_lex_jsx_identifier(&mut self) -> JsSyntaxKind {
        let current = self.current();
        if !(current.is_keyword() || current == IDENT) {
            return current;
        }
        let source = self.lexer.source().as_bytes();
        let mut end = self.current.end as usize;
        while let Some(&byte) = source.get(end) {
            match byte {
                b'-' => end += 1,
                byte if byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'$') => end += 1,
                // Escapes and Unicode identifier parts.
                b'\\' | 0x80.. => return self.re_lex_with_lexer(JsReLexContext::JsxIdentifier),
                _ => break,
            }
        }
        let escape = self.word() & u32::from(UNICODE_ESCAPE);
        let start = self.current.start;
        self.replace_current(JSX_IDENT, start, end as u32, escape);
        JSX_IDENT
    }

    /// Re-lexes the current token as an Astro attribute name with [JsLexer].
    ///
    /// When [JsLexer] reads a name, it drops every diagnostic that starts where the name does,
    /// which includes an empty one that the previous token reported at its end.
    fn re_lex_astro_jsx_attribute_name(&mut self) -> JsSyntaxKind {
        let source = self.lexer.source().as_bytes();
        let start = self.current.start;
        if source.get(start as usize).is_some_and(|&byte| {
            !matches!(byte, b'=' | b'>' | b'/' | b'{' | b'}' | b'<') && !byte.is_ascii_whitespace()
        }) {
            let start = TextSize::from(start);
            while self.diagnostics.last().is_some_and(|diagnostic| {
                diagnostic.span().is_some_and(|span| span.start() >= start)
            }) {
                self.diagnostics.pop();
            }
            let len = self.current_diagnostics().min(self.diagnostics.len());
            self.set_current_diagnostics(len);
        }
        self.re_lex_with_lexer(JsReLexContext::AstroJsxAttributeName)
    }

    #[inline(never)]
    fn re_lex_with_lexer(&mut self, mode: JsReLexContext) -> JsSyntaxKind {
        let current = self.current();
        let range = self.current_range();
        let mut flags = TokenFlags::empty();
        flags.set(
            TokenFlags::PRECEDING_LINE_BREAK,
            self.has_preceding_line_break(),
        );
        flags.set(TokenFlags::UNICODE_ESCAPE, self.has_unicode_escape());
        // Some re-lexing drops the diagnostics of the token it replaces, from the lexer's.
        let current_diagnostics = self.current_diagnostics();
        for diagnostic in self.diagnostics.drain(current_diagnostics..) {
            self.lexer.push_diagnostic(diagnostic);
        }
        let kind = self.lexer.re_lex_at(range, current, flags, mode);
        let new_range = self.lexer.current_range();
        let diagnostics = self.lexer.take_diagnostics_after(0);
        self.diagnostics.extend(diagnostics.iter().cloned());
        if kind == current && new_range == range {
            return kind;
        }
        let (start, end) = (u32::from(new_range.start()), u32::from(new_range.end()));
        let mut extra = if self.lexer.current_flags().has_unicode_escape() {
            u32::from(UNICODE_ESCAPE)
        } else {
            0
        };
        if !diagnostics.is_empty() {
            extra |= LEXED;
            self.set_lexed_diagnostics(start, diagnostics);
        }
        if let Some(trivia_kind) = trivia_kind(kind) {
            // An Astro HTML comment, which the parser must never see as current.
            let continuation = match mode {
                JsReLexContext::JsxChild { astro } => JsLexContext::JsxChild { astro },
                _ => JsLexContext::Regular,
            };
            self.relexed_trivia
                .push(Trivia::new(trivia_kind, new_range, false));
            self.trivia_changed = true;
            self.lex_tokens(None, end, continuation);
            self.take_diagnostics(continuation);
            return self.current();
        }
        self.replace_current(kind, start, end, extra);
        kind
    }

    /// Creates a checkpoint to which it can later return using [Self::rewind].
    pub(crate) fn checkpoint(&self) -> BulkTokenSourceCheckpoint {
        BulkTokenSourceCheckpoint {
            pos: self.pos as u32,
            start: self.current.start,
            current_diagnostics: self.current_diagnostics() as u32,
            diagnostics_len: self.diagnostics.len() as u32,
            lexed_diagnostics_len: self.lexed_diagnostics.len() as u32,
            skipped_len: self.skipped.len() as u32,
            splices_len: self.splices.len() as u32,
            relexed_trivia_len: self.relexed_trivia.len() as u32,
        }
    }

    /// Restores the token source to a previous state, undoing the splices since.
    pub(crate) fn rewind(&mut self, checkpoint: BulkTokenSourceCheckpoint) {
        for index in self.skipped.drain(checkpoint.skipped_len as usize..) {
            self.stream.get_mut(index as usize).word &= !SKIPPED;
        }
        while self.splices.len() > checkpoint.splices_len as usize {
            let Some(Splice {
                pos,
                inserted,
                removed,
            }) = self.splices.pop()
            else {
                break;
            };
            self.stream.splice(pos, inserted, &removed);
        }
        self.relexed_trivia
            .truncate(checkpoint.relexed_trivia_len as usize);
        self.lexed_diagnostics
            .truncate(checkpoint.lexed_diagnostics_len as usize);
        self.diagnostics
            .truncate(checkpoint.diagnostics_len as usize);
        self.pos = checkpoint.pos as usize;
        self.set_current_diagnostics(checkpoint.current_diagnostics as usize);
        self.stream.keep_gap_before(self.pos);
        self.load_current();
    }

    /// Collects the trivia between the tokens of the stream, and the tokens that the parser
    /// skipped.
    fn trivia(&mut self) -> Vec<Trivia> {
        let mut trivia = Vec::with_capacity(self.trivia.len() + self.skipped.len());
        // The latest trivia at each offset.
        let mut latest: Vec<_> = self.relexed_trivia.iter().copied().enumerate().collect();
        latest.sort_unstable_by_key(|(order, piece)| (piece.offset(), std::cmp::Reverse(*order)));
        latest.dedup_by_key(|(_, piece)| piece.offset());
        self.relexed_trivia = latest.into_iter().map(|(_, piece)| piece).collect();
        let mut bulk = 0;
        let mut relexed = 0;
        let mut gap_start = 0;
        let mut trailing = false;
        for index in 0..self.stream.len() {
            let Token { word, start, end } = self.stream.get(index);
            let mut position = gap_start;
            while position < start {
                while self
                    .relexed_trivia
                    .get(relexed)
                    .is_some_and(|piece| u32::from(piece.offset()) < position)
                {
                    relexed += 1;
                }
                let piece = if let Some(piece) = self.relexed_trivia.get(relexed)
                    && u32::from(piece.offset()) == position
                {
                    relexed += 1;
                    (piece.kind(), u32::from(piece.end_offset()))
                } else {
                    while self
                        .trivia
                        .get(bulk)
                        .is_some_and(|piece| u32::from(piece.offset()) < position)
                    {
                        bulk += 1;
                    }
                    match self.trivia.get(bulk) {
                        Some(piece)
                            if u32::from(piece.offset()) == position
                                && u32::from(piece.end_offset()) <= start =>
                        {
                            (piece.kind(), u32::from(piece.end_offset()))
                        }
                        _ => {
                            let kind =
                                self.lexer
                                    .lex_at(position as usize, false, JsLexContext::Regular);
                            self.lexer.take_diagnostics_after(0);
                            let end = u32::from(self.lexer.current_range().end()).min(start);
                            (
                                trivia_kind(kind).unwrap_or(TriviaPieceKind::Skipped),
                                end.max(position + 1),
                            )
                        }
                    }
                };
                if piece.0.is_newline() {
                    trailing = false;
                }
                trivia.push(Trivia::new(
                    piece.0,
                    TextRange::new(TextSize::from(position), TextSize::from(piece.1)),
                    trailing,
                ));
                position = piece.1;
            }
            if word & SKIPPED != 0 {
                trivia.push(Trivia::new(
                    TriviaPieceKind::Skipped,
                    TextRange::new(TextSize::from(start), TextSize::from(end)),
                    false,
                ));
                trailing = false;
            } else {
                trailing = true;
            }
            gap_start = end;
        }
        trivia
    }
}

/// A position of [BulkTokenSource] to which it can return.
#[derive(Debug)]
pub struct BulkTokenSourceCheckpoint {
    pos: u32,
    /// The start of the current token.
    start: u32,
    current_diagnostics: u32,
    diagnostics_len: u32,
    lexed_diagnostics_len: u32,
    skipped_len: u32,
    splices_len: u32,
    relexed_trivia_len: u32,
}

impl BulkTokenSourceCheckpoint {
    /// Returns the start of the current token.
    pub(crate) fn current_start(&self) -> TextSize {
        TextSize::from(self.start)
    }

    /// Returns the index of the current token.
    pub(crate) fn token_index(&self) -> usize {
        self.pos as usize
    }
}

impl<'source> TokenSource for BulkTokenSource<'source> {
    type Kind = JsSyntaxKind;

    /// Returns the kind of the current non-trivia token
    #[inline(always)]
    fn current(&self) -> JsSyntaxKind {
        self.current_kind
    }

    /// Returns the range of the current non-trivia token
    #[inline(always)]
    fn current_range(&self) -> TextRange {
        TextRange::new(
            TextSize::from(self.current.start),
            TextSize::from(self.current.end),
        )
    }

    #[inline(always)]
    fn text(&self) -> &'source str {
        self.lexer.source()
    }

    #[inline(always)]
    fn position(&self) -> TextSize {
        TextSize::from(self.current.start)
    }

    /// Returns true if the current token is preceded by a line break
    #[inline(always)]
    fn has_preceding_line_break(&self) -> bool {
        self.word() & LINE_BREAK != 0
    }

    #[inline(always)]
    fn bump(&mut self) {
        self.bump_with_context(JsLexContext::Regular)
    }

    fn skip_as_trivia(&mut self) {
        self.skip_as_trivia_with_context(JsLexContext::Regular)
    }

    fn finish(mut self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        let trivia = if self.trivia_changed || !self.skipped.is_empty() {
            self.trivia()
        } else {
            std::mem::take(&mut self.trivia)
        };
        (trivia, self.diagnostics)
    }
}

impl BumpWithContext for BulkTokenSource<'_> {
    type Context = JsLexContext;

    #[inline(always)]
    fn bump_with_context(&mut self, context: Self::Context) {
        if self.current() != EOF {
            self.pos += 1;
            self.arrive(context);
        }
    }

    /// Skips the current token as skipped token trivia
    fn skip_as_trivia_with_context(&mut self, context: JsLexContext) {
        if self.current() != EOF {
            self.stream.ahead_mut(self.pos).word |= SKIPPED;
            self.current.word |= SKIPPED;
            self.skipped.push(self.pos as u32);
            self.pos += 1;
            self.arrive(context);
        }
    }
}

impl<'source> NthToken<JsLexer<'source>> for BulkTokenSource<'source> {
    /// Gets the kind of the nth non-trivia token
    #[inline(always)]
    fn nth(&mut self, n: usize) -> JsSyntaxKind {
        if n == 0 {
            return self.current_kind;
        }
        match self.stream.get_ahead(self.pos + n) {
            Some(token) => kind(token.word as u16),
            None => EOF,
        }
    }

    /// Gets the range of the nth non-trivia token.
    fn nth_range(&mut self, n: usize) -> Option<TextRange> {
        self.stream
            .get_ahead(self.pos + n)
            .map(|token| TextRange::new(TextSize::from(token.start), TextSize::from(token.end)))
    }

    /// Returns true if the nth non-trivia token is preceded by a line break
    fn has_nth_preceding_line_break(&mut self, n: usize) -> bool {
        self.stream
            .get_ahead(self.pos + n)
            .is_some_and(|token| token.word & LINE_BREAK != 0)
    }

    /// Returns true if the nth non-trivia token is preceded by whitespace trivia, which the
    /// JavaScript lexer never reports.
    fn has_nth_preceding_whitespace(&mut self, _n: usize) -> bool {
        false
    }
}
