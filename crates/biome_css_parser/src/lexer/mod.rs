//! An extremely fast, lookup table based, СSS lexer which yields SyntaxKind tokens used by the rome-css parser.
#[rustfmt::skip]
mod tests;

use crate::CssParserOptions;
use biome_css_syntax::{CssSyntaxKind, CssSyntaxKind::*, TextLen, TextRange, TextSize, T};
use biome_js_unicode_table::{is_id_continue, is_id_start, lookup_byte, Dispatch::*};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{LexContext, Lexer, LexerCheckpoint, TokenFlags};
use std::char::REPLACEMENT_CHARACTER;
use unicode_bom::Bom;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum CssLexContext {
    /// Default context: no particular rules are applied to the lexer logic.
    #[default]
    Regular,
    /// Applied when lexing CSS selectors.
    /// Doesn't skip whitespace trivia for a combinator.
    Selector,
    /// Applied when lexing CSS pseudo nth selectors.
    /// Distinct '-' from identifiers and '+' from numbers.
    PseudoNthSelector,
}

impl LexContext for CssLexContext {
    /// Returns true if this is [CssLexContext::Regular]
    fn is_regular(&self) -> bool {
        matches!(self, CssLexContext::Regular)
    }
}

/// Context in which the [CssLexContext]'s current should be re-lexed.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CssReLexContext {}

/// An extremely fast, lookup table based, lossless CSS lexer
#[derive(Debug)]
pub(crate) struct CssLexer<'src> {
    /// Source text
    source: &'src str,

    /// The start byte position in the source text of the next token.
    position: usize,

    /// `true` if there has been a line break between the last non-trivia token and the next non-trivia token.
    after_newline: bool,

    /// If the source starts with a Unicode BOM, this is the number of bytes for that token.
    unicode_bom_length: usize,

    /// Byte offset of the current token from the start of the source
    /// The range of the current token can be computed by `self.position - self.current_start`
    current_start: TextSize,

    /// The kind of the current token
    current_kind: CssSyntaxKind,

    /// Flags for the current token
    current_flags: TokenFlags,

    diagnostics: Vec<ParseDiagnostic>,

    config: CssParserOptions,
}

impl<'src> Lexer<'src> for CssLexer<'src> {
    type Kind = CssSyntaxKind;
    type LexContext = CssLexContext;
    type ReLexContext = CssReLexContext;

    fn source(&self) -> &'src str {
        self.source
    }

    fn current(&self) -> Self::Kind {
        self.current_kind
    }

    fn current_range(&self) -> TextRange {
        TextRange::new(self.current_start, TextSize::from(self.position as u32))
    }

    fn checkpoint(&self) -> LexerCheckpoint<Self::Kind> {
        LexerCheckpoint {
            position: TextSize::from(self.position as u32),
            current_start: self.current_start,
            current_flags: self.current_flags,
            current_kind: self.current_kind,
            after_line_break: self.after_newline,
            unicode_bom_length: self.unicode_bom_length,
            diagnostics_pos: self.diagnostics.len() as u32,
        }
    }

    fn next_token(&mut self, context: Self::LexContext) -> Self::Kind {
        self.current_start = self.text_position();
        self.current_flags = TokenFlags::empty();

        let kind = match self.current_byte() {
            Some(current) => match context {
                CssLexContext::Regular => self.consume_token(current),
                CssLexContext::Selector => self.consume_selector_token(current),
                CssLexContext::PseudoNthSelector => self.consume_pseudo_nth_selector_token(current),
            },
            None => EOF,
        };

        self.current_flags
            .set(TokenFlags::PRECEDING_LINE_BREAK, self.after_newline);
        self.current_kind = kind;

        if !kind.is_trivia() {
            self.after_newline = false;
        }

        kind
    }

    fn re_lex(&mut self, _context: Self::ReLexContext) -> Self::Kind {
        let old_position = self.position;
        self.position = u32::from(self.current_start) as usize;

        let re_lexed_kind = match self.current_byte() {
            Some(current) => self.consume_selector_token(current),
            None => EOF,
        };

        if self.current() == re_lexed_kind {
            // Didn't re-lex anything. Return existing token again
            self.position = old_position;
        } else {
            self.current_kind = re_lexed_kind;
        }

        re_lexed_kind
    }

    fn has_preceding_line_break(&self) -> bool {
        self.current_flags.has_preceding_line_break()
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

    fn current_flags(&self) -> TokenFlags {
        self.current_flags
    }
}

impl<'src> CssLexer<'src> {
    /// Make a new lexer from a str, this is safe because strs are valid utf8
    pub fn from_str(source: &'src str) -> Self {
        Self {
            source,
            after_newline: false,
            unicode_bom_length: 0,
            current_kind: TOMBSTONE,
            current_start: TextSize::from(0),
            current_flags: TokenFlags::empty(),
            position: 0,
            diagnostics: vec![],
            config: CssParserOptions::default(),
        }
    }

    pub(crate) fn with_config(self, config: CssParserOptions) -> Self {
        Self { config, ..self }
    }

    fn text_position(&self) -> TextSize {
        TextSize::try_from(self.position).expect("Input to be smaller than 4 GB")
    }

    /// Bumps the current byte and creates a lexed token of the passed in kind
    fn consume_byte(&mut self, tok: CssSyntaxKind) -> CssSyntaxKind {
        self.advance(1);
        tok
    }

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

                        self.diagnostics.push(
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

    /// Consume one newline or all whitespace until a non-whitespace or a newline is found.
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn consume_newline_or_whitespaces(&mut self) -> CssSyntaxKind {
        if self.consume_newline() {
            self.after_newline = true;
            NEWLINE
        } else {
            self.consume_whitespaces();
            WHITESPACE
        }
    }

    /// Check if the source starts with a Unicode BOM character. If it does,
    /// consume it and return the UNICODE_BOM token kind.
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary (and realistically only at
    /// the start position of the source).
    fn consume_potential_bom(&mut self) -> Option<CssSyntaxKind> {
        // Bom needs at least the first three bytes of the source to know if it
        // matches the UTF-8 BOM and not an alternative. This can be expanded
        // to more bytes to support other BOM characters if Biome decides to
        // support other encodings like UTF-16.
        if let Some(first) = self.source().get(0..3) {
            let bom = Bom::from(first.as_bytes());
            self.unicode_bom_length = bom.len();
            self.advance(self.unicode_bom_length);

            match bom {
                Bom::Null => None,
                _ => Some(UNICODE_BOM),
            }
        } else {
            None
        }
    }

    /// Get the UTF8 char which starts at the current byte
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn current_char_unchecked(&self) -> char {
        self.char_unchecked_at(0)
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
            Some(self.source.as_bytes()[self.position])
        }
    }

    /// Asserts that the lexer is at current a UTF8 char boundary
    #[inline]
    fn assert_current_char_boundary(&self) {
        debug_assert!(self.source.is_char_boundary(self.position));
    }

    /// Peeks at the next byte
    #[inline]
    fn peek_byte(&self) -> Option<u8> {
        self.byte_at(1)
    }

    /// Peek the UTF8 char which starts at the current byte
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn peek_char_unchecked(&self) -> char {
        self.char_unchecked_at(1)
    }

    /// Returns the byte at position `self.position + offset` or `None` if it is out of bounds.
    #[inline]
    fn byte_at(&self, offset: usize) -> Option<u8> {
        self.source.as_bytes().get(self.position + offset).copied()
    }

    /// Advances the position by one and returns the next byte value
    #[inline]
    fn next_byte(&mut self) -> Option<u8> {
        self.advance(1);
        self.current_byte()
    }

    /// Asserts that the lexer is at a UTF8 char boundary
    #[inline]
    fn assert_at_char_boundary(&self, offset: usize) {
        debug_assert!(self.source.is_char_boundary(self.position + offset));
    }

    /// Asserts that the lexer is currently positioned at `byte`
    #[inline]
    fn assert_byte(&self, byte: u8) {
        debug_assert_eq!(self.source.as_bytes()[self.position], byte);
    }

    /// Get the UTF8 char which starts at the current byte
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn char_unchecked_at(&self, offset: usize) -> char {
        // Precautionary measure for making sure the unsafe code below does not read over memory boundary
        debug_assert!(!self.is_eof());
        self.assert_at_char_boundary(offset);

        // Safety: We know this is safe because we require the input to the lexer to be valid utf8 and we always call this when we are at a char
        let string = unsafe {
            std::str::from_utf8_unchecked(
                self.source
                    .as_bytes()
                    .get_unchecked((self.position + offset)..),
            )
        };
        let chr = if let Some(chr) = string.chars().next() {
            chr
        } else {
            // Safety: we always call this when we are at a valid char, so this branch is completely unreachable
            unsafe {
                core::hint::unreachable_unchecked();
            }
        };

        chr
    }

    /// Check if the lexer is at a valid escape. U+005C REVERSE SOLIDUS (\)
    fn is_valid_escape_at(&self, offset: usize) -> bool {
        match self.byte_at(offset) {
            Some(b'\n' | b'\r') | None => false,
            Some(_) => true,
        }
    }

    /// Advances the current position by `n` bytes.
    #[inline]
    fn advance(&mut self, n: usize) {
        self.position += n;
    }

    #[inline]
    fn advance_byte_or_char(&mut self, chr: u8) {
        if chr.is_ascii() {
            self.advance(1);
        } else {
            self.advance_char_unchecked();
        }
    }

    /// Advances the current position by the current char UTF8 length
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    #[inline]
    fn advance_char_unchecked(&mut self) {
        let c = self.current_char_unchecked();
        self.position += c.len_utf8();
    }

    /// Returns `true` if the parser is at or passed the end of the file.
    #[inline]
    fn is_eof(&self) -> bool {
        self.position >= self.source.len()
    }

    /// Lexes the next token
    ///
    /// Guaranteed to not be at the end of the file
    // A lookup table of `byte -> fn(l: &mut Lexer) -> Token` is exponentially slower than this approach
    fn consume_token(&mut self, current: u8) -> CssSyntaxKind {
        // The speed difference comes from the difference in table size, a 2kb table is easily fit into cpu cache
        // While a 16kb table will be ejected from cache very often leading to slowdowns, this also allows LLVM
        // to do more aggressive optimizations on the match regarding how to map it to instructions
        let dispatched = lookup_byte(current);

        match dispatched {
            WHS => self.consume_newline_or_whitespaces(),
            QOT => self.consume_string_literal(current),
            SLH => self.consume_slash(),

            DIG | ZER => self.consume_number(current),

            MIN => self.consume_min(current),

            PLS => {
                if self.is_number_start() {
                    self.consume_number(current)
                } else {
                    self.consume_byte(T![+])
                }
            }

            PRD => {
                if self.is_number_start() {
                    self.consume_number(current)
                } else {
                    self.consume_byte(T![.])
                }
            }

            LSS => self.consume_lss(),

            IDT | UNI | BSL if self.is_ident_start() => self.consume_identifier(),

            IDT if self.next_byte() == Some(b'=') => self.consume_byte(T!["$="]),

            MUL => self.consume_mul(),
            CRT => self.consume_ctr(),
            COL => self.consume_col(),
            AT_ => self.consume_byte(T![@]),
            SEM => self.consume_byte(T![;]),
            HAS => self.consume_byte(T![#]),
            PNO => self.consume_byte(T!['(']),
            PNC => self.consume_byte(T![')']),
            BEO => self.consume_byte(T!['{']),
            BEC => self.consume_byte(T!['}']),
            BTO => self.consume_byte(T!('[')),
            BTC => self.consume_byte(T![']']),
            COM => self.consume_byte(T![,]),
            MOR => self.consume_mor(),
            TLD => self.consume_tilde(),
            PIP => self.consume_pipe(),
            EQL => self.consume_byte(T![=]),
            EXL => self.consume_byte(T![!]),
            PRC => self.consume_byte(T![%]),

            UNI => {
                // A BOM can only appear at the start of a file, so if we haven't advanced at all yet,
                // perform the check. At any other position, the BOM is just considered plain whitespace.
                if self.position == 0 && self.consume_potential_bom().is_some() {
                    UNICODE_BOM
                } else {
                    self.consume_unexpected_character()
                }
            }

            _ => self.consume_unexpected_character(),
        }
    }

    fn consume_selector_token(&mut self, current: u8) -> CssSyntaxKind {
        match current {
            b' ' => self.consume_byte(CSS_SPACE_LITERAL),
            _ => self.consume_token(current),
        }
    }

    fn consume_pseudo_nth_selector_token(&mut self, current: u8) -> CssSyntaxKind {
        match current {
            b'-' => self.consume_byte(T![-]),
            b'+' => self.consume_byte(T![+]),
            b'n' | b'N' => self.consume_byte(T![n]),
            _ => self.consume_token(current),
        }
    }

    fn consume_string_literal(&mut self, quote: u8) -> CssSyntaxKind {
        self.assert_current_char_boundary();
        let start = self.text_position();

        self.advance(1); // Skip over the quote
        let mut state = LexStringState::InString;

        while let Some(chr) = self.current_byte() {
            let dispatch = lookup_byte(chr);

            match dispatch {
                QOT if quote == chr => {
                    self.advance(1);
                    state = match state {
                        LexStringState::InString => LexStringState::Terminated,
                        state => state,
                    };
                    break;
                }
                // '\t' etc
                BSL => {
                    let escape_start = self.text_position();
                    self.advance(1);

                    match self.current_byte() {
                        Some(b'\n' | b'\r') => self.advance(1),

                        // Handle escaped `'` but only if this is a end quote string.
                        Some(b'\'') if quote == b'\'' => {
                            self.advance(1);
                        }

                        // Handle escaped `'` but only if this is a end quote string.
                        Some(b'"') if quote == b'"' => {
                            self.advance(1);
                        }

                        Some(c) if c.is_ascii_hexdigit() => {
                            let hex = self.consume_escape_sequence(c);

                            if hex == REPLACEMENT_CHARACTER {
                                state = LexStringState::InvalidEscapeSequence;

                                let diagnostic = ParseDiagnostic::new(
                                    "Invalid escape sequence",
                                    escape_start..self.text_position(),
                                );
                                self.diagnostics.push(diagnostic);
                            }
                        }

                        Some(chr) => {
                            self.advance_byte_or_char(chr);
                        }

                        None => {}
                    }
                }
                WHS if matches!(chr, b'\n' | b'\r') => {
                    let unterminated =
                        ParseDiagnostic::new("Missing closing quote", start..self.text_position())
                            .with_detail(self.position..self.position + 1, "line breaks here");

                    self.diagnostics.push(unterminated);

                    return ERROR_TOKEN;
                }
                // we don't need to handle IDT because it's always len 1.
                UNI => self.advance_char_unchecked(),

                _ => self.advance(1),
            }
        }

        match state {
            LexStringState::Terminated => CSS_STRING_LITERAL,
            LexStringState::InString => {
                let unterminated =
                    ParseDiagnostic::new("Missing closing quote", start..self.text_position())
                        .with_detail(
                            self.source.text_len()..self.source.text_len(),
                            "file ends here",
                        );
                self.diagnostics.push(unterminated);

                ERROR_TOKEN
            }
            LexStringState::InvalidEscapeSequence => ERROR_TOKEN,
        }
    }

    fn consume_escape_sequence(&mut self, current: u8) -> char {
        debug_assert!(current.is_ascii_hexdigit());

        // SAFETY: The current byte is a hex digit.
        let mut hex = (current as char).to_digit(16).unwrap();
        self.advance(1);
        // Consume as many hex digits as possible, but no more than 6.
        // Note that this means 1-6 hex digits have been consumed in total.
        for _ in 0..5 {
            let Some(digit) = self.current_byte().and_then(|c| {
                if c.is_ascii_hexdigit() {
                    (c as char).to_digit(16)
                } else {
                    None
                }
            }) else {
                break;
            };
            self.advance(1);

            hex = hex * 16 + digit;
        }

        // If the next input code point is whitespace, consume it as well.
        if matches!(self.current_byte(), Some(b'\t' | b' ')) {
            self.advance(1);
        }

        // Interpret the hex digits as a hexadecimal number. If this number is zero, or
        // is for a surrogate, or is greater than the maximum allowed code point, return
        // U+FFFD REPLACEMENT CHARACTER (�).
        match hex {
            // If this number is zero
            0 => REPLACEMENT_CHARACTER,
            // or is for a surrogate
            55_296..=57_343 => REPLACEMENT_CHARACTER,
            // or is greater than the maximum allowed code point
            1_114_112.. => REPLACEMENT_CHARACTER,
            _ => char::from_u32(hex).unwrap_or(REPLACEMENT_CHARACTER),
        }
    }

    /// Lexes a CSS number literal
    fn consume_number(&mut self, current: u8) -> CssSyntaxKind {
        debug_assert!(self.is_number_start());

        if matches!(current, b'+' | b'-') {
            self.advance(1);
        }

        // While the next input code point is a digit, consume it.
        self.consume_number_sequence();

        // If the next 2 input code points are U+002E FULL STOP (.) followed by a digit...
        if matches!(self.current_byte(), Some(b'.'))
            && self.peek_byte().map_or(false, |byte| byte.is_ascii_digit())
        {
            // Consume them.
            self.advance(2);

            // While the next input code point is a digit, consume it.
            self.consume_number_sequence()
        }

        // If the next 2 or 3 input code points are U+0045 LATIN CAPITAL LETTER E (E) or
        // U+0065 LATIN SMALL LETTER E (e), optionally followed by U+002D HYPHEN-MINUS
        // (-) or U+002B PLUS SIGN (+), followed by a digit, then:
        if matches!(self.current_byte(), Some(b'e' | b'E')) {
            match (self.peek_byte(), self.byte_at(2)) {
                (Some(b'-' | b'+'), Some(byte)) if byte.is_ascii_digit() => {
                    // Consume them.
                    self.advance(3);

                    // While the next input code point is a digit, consume it.
                    self.consume_number_sequence()
                }
                (Some(byte), _) if byte.is_ascii_digit() => {
                    // Consume them.
                    self.advance(2);

                    // While the next input code point is a digit, consume it.
                    self.consume_number_sequence()
                }
                _ => {}
            }
        }

        CSS_NUMBER_LITERAL
    }

    fn consume_number_sequence(&mut self) {
        // While the next input code point is a digit, consume it.
        while let Some(b'0'..=b'9') = self.current_byte() {
            self.advance(1);
        }
    }

    fn consume_identifier(&mut self) -> CssSyntaxKind {
        debug_assert!(self.is_ident_start());

        // Note to keep the buffer large enough to fit every possible keyword that
        // the lexer can return
        let mut buf = [0u8; 20];
        let count = self.consume_ident_sequence(&mut buf);

        match buf[..count].to_ascii_lowercase().as_slice() {
            b"media" => MEDIA_KW,
            b"keyframes" => KEYFRAMES_KW,
            b"and" => AND_KW,
            b"only" => ONLY_KW,
            b"or" => OR_KW,
            b"i" => I_KW,
            b"s" => S_KW,
            b"important" => IMPORTANT_KW,
            b"from" => FROM_KW,
            b"to" => TO_KW,
            b"var" => VAR_KW,
            b"highlight" => HIGHLIGHT_KW,
            b"part" => PART_KW,
            b"has" => HAS_KW,
            b"dir" => DIR_KW,
            b"global" => GLOBAL_KW,
            b"local" => LOCAL_KW,
            b"-moz-any" => _MOZ_ANY_KW,
            b"-webkit-any" => _WEBKIT_ANY_KW,
            b"past" => PAST_KW,
            b"current" => CURRENT_KW,
            b"future" => FUTURE_KW,
            b"host" => HOST_KW,
            b"host-context" => HOST_CONTEXT_KW,
            b"not" => NOT_KW,
            b"matches" => MATCHES_KW,
            b"is" => IS_KW,
            b"where" => WHERE_KW,
            b"lang" => LANG_KW,
            b"of" => OF_KW,
            b"n" => N_KW,
            b"even" => EVEN_KW,
            b"odd" => ODD_KW,
            b"nth-child" => NTH_CHILD_KW,
            b"nth-last-child" => NTH_LAST_CHILD_KW,
            b"nth-of-type" => NTH_OF_TYPE_KW,
            b"nth-last-of-type" => NTH_LAST_OF_TYPE_KW,
            b"nth-col" => NTH_COL_KW,
            b"nth-last-col" => NTH_LAST_COL_KW,
            b"ltr" => LTR_KW,
            b"rtl" => RTL_KW,
            b"charset" => CHARSET_KW,
            b"color-profile" => COLOR_PROFILE_KW,
            b"counter-style" => COUNTER_STYLE_KW,
            b"container" => CONTAINER_KW,
            b"style" => STYLE_KW,
            b"font-face" => FONT_FACE_KW,
            _ => IDENT,
        }
    }

    /// Consume a ident sequence.
    fn consume_ident_sequence(&mut self, buf: &mut [u8]) -> usize {
        debug_assert!(self.is_ident_start());

        let mut idx = 0;
        let mut is_first = true;
        // Repeatedly consume the next input code point from the stream.
        while let Some(current) = self.current_byte() {
            if let Some(part) = self.consume_ident_part(current, is_first) {
                is_first = false;

                // In this context, "+ 4" represents a safety measure for the buffer size.
                // It ensures that there are at least 4 slots available in the buffer.
                // This is necessary because the maximum UTF-8 sequence length is 4 bytes.
                if let Some(buf) = buf.get_mut(idx..idx + 4) {
                    let res = part.encode_utf8(buf);
                    idx += res.len();
                }
            } else {
                break;
            }
        }

        idx
    }

    /// Tries to consume a character that forms part of a CSS identifier.
    ///
    /// This function checks if `current` character conforms to the rules for forming
    /// CSS identifiers, taking into account if it's the first character (`is_first`)
    /// in the identifier as the first character has some specific rules (like it cannot start with a digit).
    ///
    /// Also handles CSS escape sequences in identifiers and attach appropriate diagnostics for invalid cases.
    ///
    /// Returns the consumed character wrapped in `Some` if it is part of an identifier,
    /// and `None` if it is not.
    fn consume_ident_part(&mut self, current: u8, is_first: bool) -> Option<char> {
        let dispatched = lookup_byte(current);

        let chr = match dispatched {
            MIN => {
                self.advance(1);
                '-'
            }
            // name code point
            UNI | IDT => {
                // SAFETY: We know that the current byte is a valid unicode code point
                let chr = self.current_char_unchecked();
                let is_id = if is_first {
                    is_id_start(chr)
                } else {
                    is_id_continue(chr)
                };

                if is_id {
                    self.advance(chr.len_utf8());
                    chr
                } else {
                    return None;
                }
            }
            // SAFETY: We know that the current byte is a number and we can use cast.
            DIG | ZER if !is_first => {
                self.advance(1);
                current as char
            }
            // U+005C REVERSE SOLIDUS (\)
            // If the first and second code points are a valid escape, continue consume.
            // Otherwise, break.
            // BSL if self.is_valid_escape_at(1) => '\\',
            BSL if self.is_valid_escape_at(1) => {
                let escape_start = self.text_position();
                self.advance(1);

                match self.current_byte() {
                    Some(c) if c.is_ascii_hexdigit() => {
                        let hex = self.consume_escape_sequence(c);

                        if hex == REPLACEMENT_CHARACTER {
                            let diagnostic = ParseDiagnostic::new(
                                "Invalid escape sequence",
                                escape_start..self.text_position(),
                            );
                            self.diagnostics.push(diagnostic);
                        }

                        hex
                    }

                    Some(_) => {
                        let chr = self.current_char_unchecked();
                        self.advance(chr.len_utf8());
                        chr
                    }

                    None => {
                        let diagnostic = ParseDiagnostic::new(
                            "Invalid escape sequence",
                            escape_start..self.text_position(),
                        );
                        self.diagnostics.push(diagnostic);

                        return None;
                    }
                }
            }
            _ => return None,
        };

        Some(chr)
    }

    /// Lexes a comment.
    fn consume_slash(&mut self) -> CssSyntaxKind {
        self.assert_byte(b'/');

        match self.peek_byte() {
            Some(b'*') => {
                let start = self.text_position();

                // eat `/*`
                self.advance(2);

                let mut has_newline = false;

                while let Some(chr) = self.current_byte() {
                    match chr {
                        b'*' if self.peek_byte() == Some(b'/') => {
                            self.advance(2);

                            if has_newline {
                                self.after_newline = true;
                                return MULTILINE_COMMENT;
                            } else {
                                return COMMENT;
                            }
                        }
                        b'\n' | b'\r' => {
                            has_newline = true;
                            self.advance(1)
                        }
                        chr => self.advance_byte_or_char(chr),
                    }
                }

                let err =
                    ParseDiagnostic::new("Unterminated block comment", start..self.text_position())
                        .with_detail(
                            self.position..self.position + 1,
                            "... but the file ends here",
                        );

                self.diagnostics.push(err);

                if has_newline {
                    MULTILINE_COMMENT
                } else {
                    COMMENT
                }
            }
            Some(b'/') if self.config.allow_wrong_line_comments => {
                self.advance(2);

                while let Some(chr) = self.current_byte() {
                    match chr {
                        b'\n' | b'\r' => return COMMENT,
                        chr => self.advance_byte_or_char(chr),
                    }
                }

                COMMENT
            }
            _ => self.consume_byte(T![/]),
        }
    }

    #[inline]
    fn consume_pipe(&mut self) -> CssSyntaxKind {
        self.assert_byte(b'|');

        match self.next_byte() {
            Some(b'|') => self.consume_byte(T![||]),
            Some(b'=') => self.consume_byte(T![|=]),
            _ => T![|],
        }
    }

    #[inline]
    fn consume_mor(&mut self) -> CssSyntaxKind {
        self.assert_byte(b'>');

        match self.next_byte() {
            Some(b'=') => self.consume_byte(T![>=]),
            _ => T![>],
        }
    }

    #[inline]
    fn consume_tilde(&mut self) -> CssSyntaxKind {
        self.assert_byte(b'~');

        match self.next_byte() {
            Some(b'=') => self.consume_byte(T![~=]),
            _ => T![~],
        }
    }

    #[inline]
    fn consume_col(&mut self) -> CssSyntaxKind {
        self.assert_byte(b':');

        match self.next_byte() {
            Some(b':') => self.consume_byte(T![::]),
            _ => T![:],
        }
    }

    #[inline]
    fn consume_mul(&mut self) -> CssSyntaxKind {
        self.assert_byte(b'*');

        match self.next_byte() {
            Some(b'=') => self.consume_byte(T![*=]),
            _ => T![*],
        }
    }

    #[inline]
    fn consume_ctr(&mut self) -> CssSyntaxKind {
        self.assert_byte(b'^');

        match self.next_byte() {
            Some(b'=') => self.consume_byte(T![^=]),
            _ => T![^],
        }
    }

    #[inline]
    fn consume_lss(&mut self) -> CssSyntaxKind {
        self.assert_byte(b'<');

        // If the next 3 input code points are U+0021 EXCLAMATION MARK U+002D
        // HYPHEN-MINUS U+002D HYPHEN-MINUS (!--), consume them and return a CDO.
        if self.peek_byte() == Some(b'!')
            && self.byte_at(2) == Some(b'-')
            && self.byte_at(3) == Some(b'-')
        {
            self.advance(4);
            return CDO;
        }

        match self.next_byte() {
            Some(b'=') => self.consume_byte(T![<=]),
            _ => T![<],
        }
    }

    #[inline]
    fn consume_min(&mut self, current: u8) -> CssSyntaxKind {
        self.assert_byte(b'-');

        if self.is_number_start() {
            return self.consume_number(current);
        }

        // GREATER-THAN SIGN (->), consume them and return a CDC.
        if self.peek_byte() == Some(b'-') {
            if self.byte_at(2) == Some(b'>') {
                self.advance(3);
                return CDC;
            }

            // --custom-property
            if self.is_ident_start() {
                return self.consume_identifier();
            }
        }

        // -identifier
        if self.is_ident_start() {
            return self.consume_identifier();
        }

        self.consume_byte(T![-])
    }

    #[inline]
    fn consume_unexpected_character(&mut self) -> CssSyntaxKind {
        self.assert_current_char_boundary();

        let char = self.current_char_unchecked();
        let err = ParseDiagnostic::new(
            format!("unexpected character `{}`", char),
            self.text_position()..self.text_position() + char.text_len(),
        );
        self.diagnostics.push(err);
        self.advance(char.len_utf8());

        ERROR_TOKEN
    }

    /// Check if the lexer starts a number.
    fn is_number_start(&self) -> bool {
        match self.current_byte() {
            Some(b'+' | b'-') => match self.peek_byte() {
                // If the second code point is a digit, return true.
                Some(byte) if byte.is_ascii_digit() => true,
                // Otherwise, if the second code point is a U+002E FULL STOP (.) and the
                // third code point is a digit, return true.
                Some(b'.') if self.byte_at(2).map_or(false, |byte| byte.is_ascii_digit()) => true,
                _ => false,
            },
            Some(b'.') => match self.peek_byte() {
                // If the second code point is a digit, return true.
                Some(byte) if byte.is_ascii_digit() => true,
                _ => false,
            },
            Some(byte) => byte.is_ascii_digit(),
            _ => false,
        }
    }

    /// Check if the lexer starts an identifier.
    fn is_ident_start(&self) -> bool {
        let Some(current) = self.current_byte() else {
            return false;
        };

        // Look at the first code point:
        match lookup_byte(current) {
            // U+002D HYPHEN-MINUS
            MIN => {
                let Some(next) = self.peek_byte() else {
                    return false;
                };

                match lookup_byte(next) {
                    MIN => {
                        let Some(next) = self.byte_at(2) else {
                            return false;
                        };

                        match lookup_byte(next) {
                            // If the third code point is a name-start code point
                            // return true.
                            UNI | IDT if is_id_start(self.char_unchecked_at(2)) => true,
                            // or the third and fourth code points are a valid escape
                            // return true.
                            BSL => self.is_valid_escape_at(3),
                            _ => false,
                        }
                    }
                    // If the second code point is a name-start code point
                    // return true.
                    UNI | IDT if is_id_start(self.peek_char_unchecked()) => true,
                    // or the second and third code points are a valid escape
                    // return true.
                    BSL => self.is_valid_escape_at(2),
                    _ => false,
                }
            }
            UNI | IDT if is_id_start(self.current_char_unchecked()) => true,
            // U+005C REVERSE SOLIDUS (\)
            // If the first and second code points are a valid escape, return true. Otherwise,
            // return false.
            BSL => self.is_valid_escape_at(1),

            _ => false,
        }
    }
}
#[derive(Copy, Clone, Debug)]
enum LexStringState {
    /// String that contains an invalid escape sequence
    InvalidEscapeSequence,

    /// Between the opening `"` and closing `"` quotes.
    InString,

    /// Properly terminated string
    Terminated,
}
