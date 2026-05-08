mod base_name_store;
mod tests;

use crate::lexer::base_name_store::{BASENAME_STORE, is_delimiter};
use crate::token_source::TailwindLexContext;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{Lexer, LexerCheckpoint, LexerWithCheckpoint, ReLexer, TokenFlags};
use biome_rowan::{SyntaxKind, TextLen};
use biome_tailwind_syntax::T;
use biome_tailwind_syntax::TailwindSyntaxKind::*;
use biome_tailwind_syntax::{TailwindSyntaxKind, TextSize};
use biome_unicode_table::{Dispatch::*, lookup_byte};

pub(crate) struct TailwindLexer<'src> {
    /// Source text
    source: &'src str,
    /// The start byte position in the source text of the next token.
    position: usize,
    current_kind: TailwindSyntaxKind,
    current_start: TextSize,
    diagnostics: Vec<ParseDiagnostic>,
    current_flags: TokenFlags,
    preceding_line_break: bool,
    after_newline: bool,
    unicode_bom_length: usize,
}

impl<'src> TailwindLexer<'src> {
    pub fn from_str(source: &'src str) -> Self {
        Self {
            source,
            position: 0,
            current_kind: TailwindSyntaxKind::EOF,
            current_start: TextSize::default(),
            diagnostics: Vec::new(),
            current_flags: TokenFlags::empty(),
            preceding_line_break: false,
            after_newline: false,
            unicode_bom_length: 0,
        }
    }

    fn consume_token(&mut self, current: u8) -> TailwindSyntaxKind {
        let dispatched = lookup_byte(current);
        match dispatched {
            WHS => self.consume_newline_or_whitespaces(),
            PNO => self.consume_byte(T!['(']),
            PNC => self.consume_byte(T![')']),
            BTO => self.consume_byte(T!['[']),
            BTC => self.consume_byte(T![']']),
            _ if self.current_kind == T!['['] => self.consume_bracketed_thing(TW_SELECTOR, BTC),
            _ if self.current_kind == T!['('] => self.consume_bracketed_thing(TW_VALUE, PNC),
            _ if self.current_kind == T![-] => self.consume_after_dash(),
            _ if self.current_kind == T![/] => self.consume_modifier(),
            COL => self.consume_byte(T![:]),
            MIN => self.consume_byte(T![-]),
            EXL => self.consume_byte(T![!]),
            SLH => self.consume_byte(T![/]),
            IDT | ZER | DIG => self.consume_base(),
            _ => {
                if self.position == 0
                    && let Some((bom, bom_size)) = self.consume_potential_bom(UNICODE_BOM)
                {
                    self.unicode_bom_length = bom_size;
                    return bom;
                }
                self.consume_unexpected_character()
            }
        }
    }

    fn consume_token_saw_negative(&mut self, current: u8) -> TailwindSyntaxKind {
        let dispatched = lookup_byte(current);
        match dispatched {
            WHS => self.consume_newline_or_whitespaces(),
            PNO => self.consume_byte(T!['(']),
            PNC => self.consume_byte(T![')']),
            BTO => self.consume_byte(T!['[']),
            BTC => self.consume_byte(T![']']),
            _ if self.current_kind == T!['['] => self.consume_bracketed_thing(TW_SELECTOR, BTC),
            _ if self.current_kind == T!['('] => self.consume_bracketed_thing(TW_VALUE, PNC),
            COL => self.consume_byte(T![:]),
            MIN => self.consume_byte(T![-]),
            EXL => self.consume_byte(T![!]),
            SLH => self.consume_byte(T![/]),
            IDT | ZER | DIG => self.consume_base(),
            _ => self.consume_unexpected_character(),
        }
    }

    /// Consume a token in the arbitrary variant context
    fn consume_token_arbitrary_variant(&mut self, current: u8) -> TailwindSyntaxKind {
        let dispatched = lookup_byte(current);
        match dispatched {
            PNO => self.consume_byte(T!['(']),
            PNC => self.consume_byte(T![')']),
            BTO => self.consume_byte(T!['[']),
            BTC => self.consume_byte(T![']']),
            WHS => self.consume_newline_or_whitespaces(),
            _ if self.current_kind == T!['['] => self.consume_bracketed_thing(TW_SELECTOR, BTC),
            _ => self.consume_named_value(),
        }
    }

    /// Consume a token in the arbitrary candidate context
    fn consume_token_arbitrary_candidate(&mut self, current: u8) -> TailwindSyntaxKind {
        let dispatched = lookup_byte(current);
        match dispatched {
            PNO => self.consume_byte(T!['(']),
            PNC => self.consume_byte(T![')']),
            BTO => self.consume_byte(T!['[']),
            BTC => self.consume_byte(T![']']),
            WHS => self.consume_newline_or_whitespaces(),
            COL => self.consume_byte(T![:]),
            _ if self.current_kind == T!['['] => self.consume_bracketed_thing(TW_PROPERTY, COL),
            _ if self.current_kind == T![:] => self.consume_token_css_value(current),
            _ => self.consume_named_value(),
        }
    }

    fn consume_token_css_value(&mut self, current: u8) -> TailwindSyntaxKind {
        let dispatched = lookup_byte(current);
        match dispatched {
            WHS => self.consume_newline_or_whitespaces(),
            PNO => self.consume_byte(T!['(']),
            PNC => self.consume_byte(T![')']),
            BTO => self.consume_byte(T!['[']),
            BTC => self.consume_byte(T![']']),
            COL => self.consume_byte(T![:]),
            MIN if self.peek_byte().is_some_and(is_css_identifier_start) => {
                self.consume_css_identifier()
            }
            MIN => self.consume_css_number_or_punct(),
            DIG | ZER => self.consume_css_number(),
            PRD if self.peek_byte().is_some_and(|byte| byte.is_ascii_digit()) => {
                self.consume_css_number()
            }
            IDT if current == b'_' => self.consume_css_underscore_whitespace(),
            IDT => self.consume_css_identifier(),
            QOT => self.consume_css_string(current),
            HAS => self.consume_css_color(),
            PRC => self.consume_byte(T![%]),
            PLS => self.consume_byte(T![+]),
            MUL => self.consume_byte(T![*]),
            COM => self.consume_byte(T![,]),
            PRD => self.consume_byte(T![.]),
            EQL => self.consume_byte(T![=]),
            SLH => self.consume_byte(T![/]),
            _ => self.consume_unexpected_character(),
        }
    }

    fn consume_token_css_url_raw_value(&mut self, current: u8) -> TailwindSyntaxKind {
        match lookup_byte(current) {
            PNC => self.consume_byte(T![')']),
            WHS => self.consume_newline_or_whitespaces(),
            QOT => self.consume_css_string(current),
            _ => self.consume_css_url_raw_value(),
        }
    }

    fn consume_css_url_raw_value(&mut self) -> TailwindSyntaxKind {
        let mut empty = true;
        while let Some(byte) = self.current_byte() {
            if matches!(lookup_byte(byte), PNC | WHS) {
                break;
            }
            let char = self.current_char_unchecked();
            self.advance(char.len_utf8());
            empty = false;
        }

        if empty {
            ERROR_TOKEN
        } else {
            CSS_URL_VALUE_RAW_LITERAL
        }
    }

    fn consume_css_number_or_punct(&mut self) -> TailwindSyntaxKind {
        if self
            .peek_byte()
            .is_some_and(|byte| byte.is_ascii_digit() || byte == b'.')
        {
            self.consume_css_number()
        } else {
            self.consume_byte(T![-])
        }
    }

    fn consume_css_number(&mut self) -> TailwindSyntaxKind {
        if matches!(self.current_byte(), Some(b'+' | b'-')) {
            self.advance(1);
        }

        self.consume_ascii_digits();

        if self.current_byte() == Some(b'.')
            && self.peek_byte().is_some_and(|byte| byte.is_ascii_digit())
        {
            self.advance(1);
            self.consume_ascii_digits();
        }

        if matches!(self.current_byte(), Some(b'e' | b'E')) {
            let checkpoint = self.position;
            self.advance(1);
            if matches!(self.current_byte(), Some(b'+' | b'-')) {
                self.advance(1);
            }
            if self
                .current_byte()
                .is_some_and(|byte| byte.is_ascii_digit())
            {
                self.consume_ascii_digits();
            } else {
                self.position = checkpoint;
            }
        }

        if self.current_byte() == Some(b'%') {
            CSS_PERCENTAGE_VALUE
        } else if self.current_byte().is_some_and(is_css_identifier_start) {
            CSS_DIMENSION_VALUE
        } else {
            CSS_NUMBER_LITERAL
        }
    }

    fn consume_ascii_digits(&mut self) {
        while self
            .current_byte()
            .is_some_and(|byte| byte.is_ascii_digit())
        {
            self.advance(1);
        }
    }

    fn consume_css_identifier(&mut self) -> TailwindSyntaxKind {
        let start = self.position;
        while self.current_byte().is_some_and(is_css_identifier_continue) {
            self.advance(1);
        }

        match &self.source.as_bytes()[start..self.position] {
            b"url" => URL_KW,
            b"var" => VAR_KW,
            b"em" => EM_KW,
            b"rem" => REM_KW,
            b"ex" => EX_KW,
            b"rex" => REX_KW,
            b"cap" => CAP_KW,
            b"rcap" => RCAP_KW,
            b"ch" => CH_KW,
            b"rch" => RCH_KW,
            b"ic" => IC_KW,
            b"ric" => RIC_KW,
            b"lh" => LH_KW,
            b"rlh" => RLH_KW,
            b"vw" => VW_KW,
            b"svw" => SVW_KW,
            b"lvw" => LVW_KW,
            b"dvw" => DVW_KW,
            b"vh" => VH_KW,
            b"svh" => SVH_KW,
            b"lvh" => LVH_KW,
            b"dvh" => DVH_KW,
            b"vi" => VI_KW,
            b"svi" => SVI_KW,
            b"lvi" => LVI_KW,
            b"dvi" => DVI_KW,
            b"vb" => VB_KW,
            b"svb" => SVB_KW,
            b"lvb" => LVB_KW,
            b"dvb" => DVB_KW,
            b"vmin" => VMIN_KW,
            b"svmin" => SVMIN_KW,
            b"lvmin" => LVMIN_KW,
            b"dvmin" => DVMIN_KW,
            b"vmax" => VMAX_KW,
            b"svmax" => SVMAX_KW,
            b"lvmax" => LVMAX_KW,
            b"dvmax" => DVMAX_KW,
            b"cm" => CM_KW,
            b"mm" => MM_KW,
            b"q" => Q_KW,
            b"in" => IN_KW,
            b"pc" => PC_KW,
            b"pt" => PT_KW,
            b"px" => PX_KW,
            b"mozmm" => MOZMM_KW,
            b"rpx" => RPX_KW,
            b"cqw" => CQW_KW,
            b"cqh" => CQH_KW,
            b"cqi" => CQI_KW,
            b"cqb" => CQB_KW,
            b"cqmin" => CQMIN_KW,
            b"cqmax" => CQMAX_KW,
            b"deg" => DEG_KW,
            b"grad" => GRAD_KW,
            b"rad" => RAD_KW,
            b"turn" => TURN_KW,
            b"s" => S_KW,
            b"ms" => MS_KW,
            b"hz" => HZ_KW,
            b"khz" => KHZ_KW,
            b"dpi" => DPI_KW,
            b"dpcm" => DPCM_KW,
            b"dppx" => DPPX_KW,
            b"x" => X_KW,
            b"fr" => FR_KW,
            _ => IDENT,
        }
    }

    fn consume_css_string(&mut self, quote: u8) -> TailwindSyntaxKind {
        self.advance(1);
        while let Some(byte) = self.current_byte() {
            self.advance(1);
            if byte == quote {
                break;
            }
            if byte == b'\\' && !self.is_eof() {
                self.advance(1);
            }
        }
        CSS_STRING_LITERAL
    }

    fn consume_css_color(&mut self) -> TailwindSyntaxKind {
        self.advance(1);
        while self
            .current_byte()
            .is_some_and(|byte| byte.is_ascii_hexdigit())
        {
            self.advance(1);
        }
        CSS_COLOR_LITERAL
    }

    fn consume_css_underscore_whitespace(&mut self) -> TailwindSyntaxKind {
        while self.current_byte() == Some(b'_') {
            self.advance(1);
        }
        WHITESPACE
    }

    fn peek_byte(&self) -> Option<u8> {
        self.source.as_bytes().get(self.position + 1).copied()
    }

    fn consume_base(&mut self) -> TailwindSyntaxKind {
        self.assert_current_char_boundary();

        let bytes = self.source.as_bytes();
        let slice = &bytes[self.position..];

        // ASCII fast path: if there is no '-' before a delimiter, the basename ends at the first delimiter.
        // This avoids entering the dashed-basename trie for the common undashed utility names.
        let mut end = 0usize;
        while end < slice.len() {
            let b = slice[end];
            let dispatched = lookup_byte(b);
            if dispatched == MIN || is_delimiter(dispatched) {
                break;
            }
            end += 1;
        }

        if end == 4 && &slice[..end] == b"data" {
            self.advance(end);
            return DATA_KW;
        }

        if end > 0 && (end == slice.len() || is_delimiter(lookup_byte(slice[end]))) {
            self.advance(end);
            return TW_BASE;
        }

        // Fallback to dashed-basename trie matching for cases with '-' inside the basename
        let dashed_end = BASENAME_STORE.matcher(slice).base_end();
        self.advance(dashed_end);

        if dashed_end == 4 && &slice[..dashed_end] == b"data" {
            DATA_KW
        } else {
            TW_BASE
        }
    }

    fn consume_named_value(&mut self) -> TailwindSyntaxKind {
        self.assert_current_char_boundary();

        while let Some(byte) = self.current_byte() {
            let dispatched = lookup_byte(byte);
            if matches!(dispatched, WHS | COL | SLH | EXL | BTC | PNC) {
                break;
            }
            let char = self.current_char_unchecked();
            self.advance(char.len_utf8());
        }

        TW_VALUE
    }

    /// After seeing a '-', we usually lex a value. However, if the next bytes are "data"
    /// and they are followed by another '-', this is a Tailwind data-attribute variant.
    /// In that case, emit DATA_KW so the parser can recognize `TwDataAttribute`.
    fn consume_after_dash(&mut self) -> TailwindSyntaxKind {
        self.assert_current_char_boundary();

        let bytes = self.source.as_bytes();
        let slice = &bytes[self.position..];

        if slice.len() >= 5 && &slice[..4] == b"data" && slice[4] == b'-' {
            // Advance past "data" only. The following '-' will be emitted as its own token.
            self.advance(4);
            return DATA_KW;
        }

        if self.current_byte() == Some(b'[') {
            return self.consume_byte(T!['[']);
        }

        self.consume_named_value()
    }

    fn consume_modifier(&mut self) -> TailwindSyntaxKind {
        self.assert_current_char_boundary();

        let mut empty = true;
        while let Some(byte) = self.current_byte() {
            let dispatched = lookup_byte(byte);
            let char = self.current_char_unchecked();
            if matches!(dispatched, WHS | EXL) {
                break;
            }
            self.advance(char.len_utf8());
            empty = false;
        }

        if empty { ERROR_TOKEN } else { TW_VALUE }
    }

    fn consume_bracketed_thing(
        &mut self,
        kind: TailwindSyntaxKind,
        looking_for: biome_unicode_table::Dispatch,
    ) -> TailwindSyntaxKind {
        self.assert_current_char_boundary();

        let mut empty = true;
        let mut bracket_depth = 0;
        while let Some(byte) = self.current_byte() {
            let dispatched = lookup_byte(byte);
            if dispatched == BTO && looking_for == BTC {
                bracket_depth += 1;
            } else if dispatched == BTC && bracket_depth > 0 {
                bracket_depth -= 1;
            } else if (dispatched == looking_for || dispatched == WHS) && bracket_depth == 0 {
                break;
            }
            let char = self.current_char_unchecked();
            self.advance(char.len_utf8());
            empty = false;
        }

        if empty { ERROR_TOKEN } else { kind }
    }

    /// Bumps the current byte and creates a lexed token of the passed in kind.
    #[inline]
    fn consume_byte(&mut self, tok: TailwindSyntaxKind) -> TailwindSyntaxKind {
        self.advance(1);
        tok
    }

    fn consume_unexpected_character(&mut self) -> TailwindSyntaxKind {
        self.assert_at_char_boundary();

        let char = self.current_char_unchecked();
        let err = ParseDiagnostic::new(
            format!("Unexpected character `{char}`"),
            self.text_position()..self.text_position() + char.text_len(),
        );
        self.diagnostics.push(err);
        self.advance(char.len_utf8());

        ERROR_TOKEN
    }

    /// Asserts that the lexer is at a UTF8 char boundary
    #[inline]
    fn assert_at_char_boundary(&self) {
        debug_assert!(self.source.is_char_boundary(self.position));
    }
}

impl<'src> Lexer<'src> for TailwindLexer<'src> {
    const NEWLINE: Self::Kind = NEWLINE;
    const WHITESPACE: Self::Kind = WHITESPACE;
    type Kind = TailwindSyntaxKind;
    type LexContext = TailwindLexContext;
    type ReLexContext = ();

    fn source(&self) -> &'src str {
        self.source
    }

    fn current(&self) -> Self::Kind {
        self.current_kind
    }

    fn current_start(&self) -> TextSize {
        self.current_start
    }

    fn next_token(&mut self, context: Self::LexContext) -> Self::Kind {
        self.current_start = TextSize::from(self.position as u32);
        self.current_flags = TokenFlags::empty();

        let kind = if self.is_eof() {
            EOF
        } else {
            match self.current_byte() {
                Some(current) => match context {
                    TailwindLexContext::Regular => self.consume_token(current),
                    TailwindLexContext::SawNegative => self.consume_token_saw_negative(current),
                    TailwindLexContext::ArbitraryVariant => {
                        self.consume_token_arbitrary_variant(current)
                    }
                    TailwindLexContext::ArbitraryCandidate => {
                        self.consume_token_arbitrary_candidate(current)
                    }
                    TailwindLexContext::CssValue => self.consume_token_css_value(current),
                    TailwindLexContext::CssUrlRawValue => {
                        self.consume_token_css_url_raw_value(current)
                    }
                },
                None => EOF,
            }
        };

        self.current_flags
            .set(TokenFlags::PRECEDING_LINE_BREAK, self.after_newline);
        self.current_kind = kind;

        if !kind.is_trivia() {
            self.after_newline = false;
        }

        kind
    }

    fn has_preceding_line_break(&self) -> bool {
        self.preceding_line_break
    }

    fn has_unicode_escape(&self) -> bool {
        self.current_flags.has_unicode_escape()
    }

    fn rewind(&mut self, checkpoint: LexerCheckpoint<Self::Kind>) {
        let LexerCheckpoint {
            position,
            current_start,
            current_flags,
            current_kind,
            after_line_break,
            after_whitespace: _,
            unicode_bom_length,
            diagnostics_pos,
        } = checkpoint;

        let new_pos = u32::from(position) as usize;

        self.position = new_pos;
        self.current_kind = current_kind;
        self.current_start = current_start;
        self.current_flags = current_flags;
        self.after_newline = after_line_break;
        self.unicode_bom_length = unicode_bom_length;
        self.diagnostics.truncate(diagnostics_pos as usize);
    }

    fn finish(self) -> Vec<ParseDiagnostic> {
        self.diagnostics
    }

    fn position(&self) -> usize {
        self.position
    }

    fn push_diagnostic(&mut self, diagnostic: ParseDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    fn advance_char_unchecked(&mut self) {
        let c = self.current_char_unchecked();
        self.position += c.len_utf8();
    }

    fn advance(&mut self, n: usize) {
        self.position += n;
    }
}

impl<'src> LexerWithCheckpoint<'src> for TailwindLexer<'src> {
    fn checkpoint(&self) -> LexerCheckpoint<Self::Kind> {
        LexerCheckpoint {
            position: TextSize::from(self.position as u32),
            current_start: self.current_start,
            current_flags: self.current_flags,
            current_kind: self.current_kind,
            after_line_break: self.after_newline,
            after_whitespace: false,
            unicode_bom_length: self.unicode_bom_length,
            diagnostics_pos: self.diagnostics.len() as u32,
        }
    }
}

impl<'src> ReLexer<'src> for TailwindLexer<'src> {
    fn re_lex(&mut self, _context: Self::ReLexContext) -> Self::Kind {
        todo!()
    }
}

fn is_css_identifier_start(byte: u8) -> bool {
    byte != b'_' && matches!(lookup_byte(byte), IDT | MIN)
}

fn is_css_identifier_continue(byte: u8) -> bool {
    byte != b'_' && matches!(lookup_byte(byte), IDT | MIN | DIG | ZER)
}
