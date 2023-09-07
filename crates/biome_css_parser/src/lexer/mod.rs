//! An extremely fast, lookup table based, СSS lexer which yields SyntaxKind tokens used by the rome-css parser.
#[rustfmt::skip]
mod tests;

use crate::CssParserOptions;
use biome_css_syntax::{CssSyntaxKind, CssSyntaxKind::*, TextLen, TextRange, TextSize, T};
use rome_js_unicode_table::{is_id_continue, is_id_start, lookup_byte, Dispatch::*};
use rome_parser::diagnostic::ParseDiagnostic;
use std::char::REPLACEMENT_CHARACTER;
use std::iter::FusedIterator;

pub struct Token {
    kind: CssSyntaxKind,
    range: TextRange,
}

impl Token {
    pub fn kind(&self) -> CssSyntaxKind {
        self.kind
    }

    pub fn range(&self) -> TextRange {
        self.range
    }
}

/// An extremely fast, lookup table based, lossless CSS lexer
#[derive(Debug)]
pub(crate) struct Lexer<'src> {
    /// Source text
    source: &'src str,

    /// The start byte position in the source text of the next token.
    position: usize,

    diagnostics: Vec<ParseDiagnostic>,

    config: CssParserOptions,
}

impl<'src> Lexer<'src> {
    /// Make a new lexer from a str, this is safe because strs are valid utf8
    pub fn from_str(source: &'src str) -> Self {
        Self {
            source,
            position: 0,
            diagnostics: vec![],
            config: CssParserOptions::default(),
        }
    }

    pub(crate) fn with_config(mut self, config: CssParserOptions) -> Self {
        self.config = config;
        self
    }

    /// Returns the source code
    pub fn source(&self) -> &'src str {
        self.source
    }

    pub fn finish(self) -> Vec<ParseDiagnostic> {
        self.diagnostics
    }

    /// Lexes the next token.
    ///
    /// ## Return
    /// Returns its kind.
    pub(crate) fn next_token(&mut self) -> Option<Token> {
        let start = self.text_position();

        match self.current_byte() {
            Some(current) => {
                let kind = self.lex_token(current);

                debug_assert!(start < self.text_position(), "Lexer did not progress");
                Some(Token {
                    kind,
                    range: TextRange::new(start, self.text_position()),
                })
            }
            None if self.position == self.source.len() => {
                self.advance(1);
                Some(Token {
                    kind: EOF,
                    range: TextRange::new(start, start),
                })
            }
            None => None,
        }
    }

    fn text_position(&self) -> TextSize {
        TextSize::try_from(self.position).expect("Input to be smaller than 4 GB")
    }

    /// Bumps the current byte and creates a lexed token of the passed in kind
    fn eat_byte(&mut self, tok: CssSyntaxKind) -> CssSyntaxKind {
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
                            .hint("Use a regular whitespace character instead."),
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
            NEWLINE
        } else {
            self.consume_whitespaces();
            WHITESPACE
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

    /// Asserts that the lexer is at a UTF8 char boundary
    #[inline]
    fn assert_at_char_boundary(&self, offset: usize) {
        debug_assert!(self.source.is_char_boundary(self.position + offset));
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
    fn lex_token(&mut self, current: u8) -> CssSyntaxKind {
        // The speed difference comes from the difference in table size, a 2kb table is easily fit into cpu cache
        // While a 16kb table will be ejected from cache very often leading to slowdowns, this also allows LLVM
        // to do more aggressive optimizations on the match regarding how to map it to instructions
        let dispatched = lookup_byte(current);

        match dispatched {
            WHS => self.consume_newline_or_whitespaces(),
            QOT => self.lex_string_literal(current),
            SLH => self.lex_slash(),

            DIG => self.lex_number(current),

            MIN => {
                if self.is_number_start() {
                    return self.lex_number(current);
                }

                // GREATER-THAN SIGN (->), consume them and return a CDC.
                if self.peek_byte() == Some(b'-') {
                    if self.byte_at(2) == Some(b'>') {
                        self.advance(3);
                        return CDC;
                    }

                    // --custom-property
                    if self.is_ident_start() {
                        self.advance(2);
                        self.lex_identifier();

                        return CSS_CUSTOM_PROPERTY;
                    }
                }

                // -identifier
                if self.is_ident_start() {
                    self.advance(1);
                    return self.lex_identifier();
                }

                self.eat_byte(T![-])
            }

            PLS => {
                if self.is_number_start() {
                    self.lex_number(current)
                } else {
                    self.eat_byte(T![+])
                }
            }

            PRD => {
                if self.is_number_start() {
                    self.lex_number(current)
                } else {
                    self.eat_byte(T![.])
                }
            }

            LSS => {
                // If the next 3 input code points are U+0021 EXCLAMATION MARK U+002D
                // HYPHEN-MINUS U+002D HYPHEN-MINUS (!--), consume them and return a CDO.
                if self.peek_byte() == Some(b'!')
                    && self.byte_at(2) == Some(b'-')
                    && self.byte_at(3) == Some(b'-')
                {
                    self.advance(4);
                    return CDO;
                }

                self.eat_byte(T![<])
            }

            IDT | UNI | BSL if self.is_ident_start() => self.lex_identifier(),

            MUL => self.eat_byte(T![*]),
            COL => self.eat_byte(T![:]),
            AT_ => self.eat_byte(T![@]),
            HAS => self.eat_byte(T![#]),
            PNO => self.eat_byte(T!['(']),
            PNC => self.eat_byte(T![')']),
            BEO => self.eat_byte(T!['{']),
            BEC => self.eat_byte(T!['}']),
            BTO => self.eat_byte(T!('[')),
            BTC => self.eat_byte(T![']']),
            COM => self.eat_byte(T![,]),

            _ => self.eat_unexpected_character(),
        }
    }

    fn lex_string_literal(&mut self, quote: u8) -> CssSyntaxKind {
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
                            // SAFETY: We know that the current byte is a hex digit.
                            let mut hex = (c as char).to_digit(16).unwrap();
                            self.advance(1);

                            // Consume as many hex digits as possible, but no more than 5.
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

                            // Interpret the hex digits as a hexadecimal number. If this number is zero, or
                            // is for a surrogate, or is greater than the maximum allowed code point, return
                            // U+FFFD REPLACEMENT CHARACTER (�).
                            let hex = match hex {
                                // If this number is zero
                                0 => REPLACEMENT_CHARACTER,
                                // or is for a surrogate
                                55296..=57343 => REPLACEMENT_CHARACTER,
                                // or is greater than the maximum allowed code point
                                1114112.. => REPLACEMENT_CHARACTER,
                                _ => char::from_u32(hex).unwrap_or(REPLACEMENT_CHARACTER),
                            };

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
                            .detail(self.position..self.position + 1, "line breaks here");

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
                        .detail(
                            self.source.text_len()..self.source.text_len(),
                            "file ends here",
                        );
                self.diagnostics.push(unterminated);

                ERROR_TOKEN
            }
            LexStringState::InvalidEscapeSequence => ERROR_TOKEN,
        }
    }

    /// Lexes a CSS number literal
    fn lex_number(&mut self, current: u8) -> CssSyntaxKind {
        debug_assert!(self.is_number_start());

        if matches!(current, b'+' | b'-') {
            self.advance(1);
        }

        // While the next input code point is a digit, consume it.
        self.consume_number();

        // If the next 2 input code points are U+002E FULL STOP (.) followed by a digit...
        if matches!(self.current_byte(), Some(b'.'))
            && self.peek_byte().map_or(false, |byte| byte.is_ascii_digit())
        {
            // Consume them.
            self.advance(2);

            // While the next input code point is a digit, consume it.
            self.consume_number()
        }

        // If the next 2 or 3 input code points are U+0045 LATIN CAPITAL LETTER E (E) or
        // U+0065 LATIN SMALL LETTER E (e), optionally followed by U+002D HYPHEN-MINUS
        // (-) or U+002B PLUS SIGN (+), followed by a digit, then:
        if matches!(self.current_byte(), Some(b'e') | Some(b'E')) {
            match (self.peek_byte(), self.byte_at(2)) {
                (Some(b'-') | Some(b'+'), Some(byte)) if byte.is_ascii_digit() => {
                    // Consume them.
                    self.advance(3);

                    // While the next input code point is a digit, consume it.
                    self.consume_number()
                }
                (Some(byte), _) if byte.is_ascii_digit() => {
                    // Consume them.
                    self.advance(2);

                    // While the next input code point is a digit, consume it.
                    self.consume_number()
                }
                _ => {}
            }
        }

        CSS_NUMBER_LITERAL
    }

    fn consume_number(&mut self) {
        // While the next input code point is a digit, consume it.
        while let Some(b'0'..=b'9') = self.current_byte() {
            self.advance(1);
        }
    }

    fn lex_identifier(&mut self) -> CssSyntaxKind {
        // Note to keep the buffer large enough to fit every possible keyword that
        // the lexer can return
        let mut buf = [0u8; 20];
        let count = self.consume_ident_sequence(&mut buf);

        match &buf[..count] {
            b"media" => MEDIA_KW,
            b"keyframes" => KEYFRAMES_KW,
            b"not" => NOT_KW,
            b"and" => AND_KW,
            b"only" => ONLY_KW,
            b"or" => OR_KW,
            b"i" => I_KW,
            b"important" => IMPORTANT_KW,
            b"from" => FROM_KW,
            b"to" => TO_KW,
            b"var" => VAR_KW,
            _ => IDENT,
        }
    }

    /// Consume a ident sequence.
    fn consume_ident_sequence(&mut self, buf: &mut [u8]) -> usize {
        debug_assert!(self.is_ident_start());

        let mut idx = 0;
        let mut is_first = true;
        // Repeatedly consume the next input code point from the stream.
        loop {
            let Some(current) = self.current_byte() else {
                break;
            };

            let dispatched = lookup_byte(current);

            let chr = match dispatched {
                // name code point
                UNI | IDT => {
                    // SAFETY: We know that the current byte is a valid unicode code point
                    let chr = self.current_char_unchecked();
                    let is_id = if is_first {
                        is_first = false;
                        is_id_start(chr)
                    } else {
                        is_id_continue(chr)
                    };

                    if is_id {
                        chr
                    } else {
                        break;
                    }
                }
                // SAFETY: We know that the current byte is a number and we can use cast.
                DIG | ZER if !is_first => current as char,
                // U+005C REVERSE SOLIDUS (\)
                // If the first and second code points are a valid escape, continue consume.
                // Otherwise, break.
                BSL if self.is_valid_escape_at(1) => '\\',
                _ => break,
            };

            let len = chr.len_utf8();
            self.advance(len);

            if let Some(buf) = buf.get_mut(idx..idx + 4) {
                let res = chr.encode_utf8(buf);
                idx += res.len();
            }
        }

        idx
    }

    /// Lexes a comment.
    fn lex_slash(&mut self) -> CssSyntaxKind {
        let start = self.text_position();
        match self.peek_byte() {
            Some(b'*') => {
                // eat `/*`
                self.advance(2);

                let mut has_newline = false;

                while let Some(chr) = self.current_byte() {
                    match chr {
                        b'*' if self.peek_byte() == Some(b'/') => {
                            self.advance(2);

                            if has_newline {
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
                        .detail(
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
            Some(b'/') => {
                self.advance(2);

                while let Some(chr) = self.current_byte() {
                    match chr {
                        b'\n' | b'\r' => return COMMENT,
                        chr => self.advance_byte_or_char(chr),
                    }
                }

                COMMENT
            }
            _ => self.eat_unexpected_character(),
        }
    }

    #[inline]
    fn eat_unexpected_character(&mut self) -> CssSyntaxKind {
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
            Some(b'+') | Some(b'-') => match self.peek_byte() {
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

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

impl FusedIterator for Lexer<'_> {}

#[derive(Copy, Clone, Debug)]
enum LexStringState {
    /// String that contains an invalid escape sequence
    InvalidEscapeSequence,

    /// Between the opening `"` and closing `"` quotes.
    InString,

    /// Properly terminated string
    Terminated,
}
