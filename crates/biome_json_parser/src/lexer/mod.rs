//! An extremely fast, lookup table based, JSON lexer which yields SyntaxKind tokens used by the rome-json parser.

#[rustfmt::skip]
mod tests;

use biome_json_syntax::{JsonSyntaxKind, JsonSyntaxKind::*, TextLen, TextRange, TextSize, T};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_unicode_table::{is_js_id_continue, is_js_id_start, lookup_byte, Dispatch::*};
use std::iter::FusedIterator;
use std::ops::Add;
use unicode_bom::Bom;

use crate::JsonParserOptions;

pub struct Token {
    kind: JsonSyntaxKind,
    range: TextRange,
}

impl Token {
    pub fn kind(&self) -> JsonSyntaxKind {
        self.kind
    }

    pub fn range(&self) -> TextRange {
        self.range
    }
}

/// An extremely fast, lookup table based, lossless JSON lexer
#[derive(Debug)]
pub(crate) struct Lexer<'src> {
    /// Source text
    source: &'src str,

    /// The start byte position in the source text of the next token.
    position: usize,

    diagnostics: Vec<ParseDiagnostic>,
    options: JsonParserOptions,
}

impl<'src> Lexer<'src> {
    /// Make a new lexer from a str, this is safe because strs are valid utf8
    pub fn from_str(string: &'src str) -> Self {
        Self {
            source: string,
            position: 0,
            diagnostics: vec![],
            options: JsonParserOptions::default(),
        }
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
    /// Returns its kind and any potential error.
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
    fn eat_byte(&mut self, tok: JsonSyntaxKind) -> JsonSyntaxKind {
        self.advance(1);
        tok
    }

    /// Consume just one newline/line break.
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn consume_newline(&mut self) -> bool {
        self.assert_at_char_boundary();

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
        self.assert_at_char_boundary();

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
                                "The JSON standard only allows tabs, whitespace, carriage return and line feed whitespace.",
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
    fn consume_newline_or_whitespaces(&mut self) -> JsonSyntaxKind {
        if self.consume_newline() {
            NEWLINE
        } else {
            self.consume_whitespaces();
            WHITESPACE
        }
    }

    /// Check if the source starts with a Unicode BOM character. If it does,
    /// consume it and return the UNICODE_BOM token kind.
    ///
    /// Note that JSON explicitly forbids BOM characters from appearing in a
    /// network-transmitted JSON Text: https://datatracker.ietf.org/doc/html/rfc8259#section-8.1.
    /// However, Windows editors in particular will occasionally add a BOM
    /// anyway, and Biome should not remove those characters when present, so
    /// they need to be tracked.
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary (and realistically only at
    /// the start position of the source).
    fn consume_potential_bom(&mut self) -> Option<JsonSyntaxKind> {
        // Bom needs at least the first three bytes of the source to know if it
        // matches the UTF-8 BOM and not an alternative. This can be expanded
        // to more bytes to support other BOM characters if Biome decides to
        // support other encodings like UTF-16.
        if let Some(first) = self.source().get(0..3) {
            let bom = Bom::from(first.as_bytes());
            self.advance(bom.len());

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
        // Precautionary measure for making sure the unsafe code below does not read over memory boundary
        debug_assert!(!self.is_eof());
        self.assert_at_char_boundary();

        // Safety: We know this is safe because we require the input to the lexer to be valid utf8 and we always call this when we are at a char
        let string = unsafe {
            std::str::from_utf8_unchecked(self.source.as_bytes().get_unchecked(self.position..))
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

    /// Asserts that the lexer is at a UTF8 char boundary
    #[inline]
    fn assert_at_char_boundary(&self) {
        debug_assert!(self.source.is_char_boundary(self.position));
    }

    /// Asserts that the lexer is currently positioned at `byte`
    #[inline]
    fn assert_byte(&self, byte: u8) {
        debug_assert_eq!(self.source.as_bytes()[self.position], byte);
    }

    /// Peeks at the next byte
    #[inline]
    fn peek_byte(&self) -> Option<u8> {
        self.byte_at(1)
    }

    /// Returns the byte at position `self.position + offset` or `None` if it is out of bounds.
    #[inline]
    fn byte_at(&self, offset: usize) -> Option<u8> {
        self.source.as_bytes().get(self.position + offset).copied()
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
    fn lex_token(&mut self, current: u8) -> JsonSyntaxKind {
        // The speed difference comes from the difference in table size, a 2kb table is easily fit into cpu cache
        // While a 16kb table will be ejected from cache very often leading to slowdowns, this also allows LLVM
        // to do more aggressive optimizations on the match regarding how to map it to instructions
        let dispatched = lookup_byte(current);

        match dispatched {
            WHS => self.consume_newline_or_whitespaces(),
            QOT => self.lex_string_literal(current),
            IDT | DOL => self.lex_identifier(current),
            COM => self.eat_byte(T![,]),
            MIN | DIG | ZER => self.lex_number(current),
            COL => self.eat_byte(T![:]),
            BTO => self.eat_byte(T!['[']),
            BTC => self.eat_byte(T![']']),
            BEO => self.eat_byte(T!['{']),
            BEC => self.eat_byte(T!['}']),

            SLH => self.lex_slash(),

            UNI => {
                let chr = self.current_char_unchecked();

                if is_js_id_start(chr) {
                    self.lex_identifier(current)
                } else if self.position == 0 && self.consume_potential_bom().is_some() {
                    // A BOM can only appear at the start of a file, so if we haven't advanced at all yet,
                    // perform the check. At any other position, the BOM is just considered plain whitespace.
                    UNICODE_BOM
                } else {
                    self.eat_unexpected_character()
                }
            }

            ERR | EXL | HAS | TLD | PIP | TPL | CRT | BSL | AT_ | QST | MOR | LSS | SEM | MUL
            | PNO | PNC | PRD | PRC | AMP | EQL | PLS => self.eat_unexpected_character(),
        }
    }

    #[inline]
    fn eat_unexpected_character(&mut self) -> JsonSyntaxKind {
        self.assert_at_char_boundary();

        let char = self.current_char_unchecked();
        let err = ParseDiagnostic::new(
            format!("unexpected character `{char}`"),
            self.text_position()..self.text_position() + char.text_len(),
        );
        self.diagnostics.push(err);
        self.advance(char.len_utf8());

        ERROR_TOKEN
    }

    /// Lexes a JSON number literal
    fn lex_number(&mut self, current: u8) -> JsonSyntaxKind {
        self.assert_at_char_boundary();

        let start = self.text_position();

        if current == b'-' {
            self.advance(1);
        }

        let mut state = LexNumberState::FirstDigit;

        loop {
            state = match self.current_byte() {
                Some(b'0') => {
                    let position = self.text_position();

                    self.advance(1);

                    match state {
                        LexNumberState::FirstDigit
                            if matches!(self.current_byte(), Some(b'0'..=b'9')) =>
                        {
                            LexNumberState::Invalid {
                                position,
                                reason: InvalidNumberReason::Octal,
                            }
                        }
                        LexNumberState::FirstDigit => LexNumberState::IntegerPart,
                        state => state,
                    }
                }
                Some(b'0'..=b'9') => {
                    self.advance(1);

                    match state {
                        LexNumberState::FirstDigit => LexNumberState::IntegerPart,
                        state => state,
                    }
                }
                Some(b'.') => {
                    let position = self.text_position();

                    self.advance(1);

                    match state {
                        LexNumberState::IntegerPart
                            if matches!(self.current_byte(), Some(b'0'..=b'9')) =>
                        {
                            LexNumberState::FractionalPart
                        }
                        LexNumberState::IntegerPart => LexNumberState::Invalid {
                            position: self.text_position(),
                            reason: InvalidNumberReason::MissingFraction,
                        },
                        invalid @ LexNumberState::Invalid { .. } => invalid,
                        _ => LexNumberState::Invalid {
                            position,
                            reason: InvalidNumberReason::Fraction,
                        },
                    }
                }
                Some(b'e' | b'E') => {
                    let position = self.text_position();

                    match self.peek_byte() {
                        Some(b'-' | b'+') => self.advance(2),
                        _ => self.advance(1),
                    };

                    match state {
                        LexNumberState::IntegerPart | LexNumberState::FractionalPart
                            if matches!(self.current_byte(), Some(b'0'..=b'9')) =>
                        {
                            LexNumberState::Exponent
                        }
                        LexNumberState::IntegerPart | LexNumberState::FractionalPart => {
                            LexNumberState::Invalid {
                                position: self.text_position(),
                                reason: InvalidNumberReason::MissingExponent,
                            }
                        }
                        invalid @ LexNumberState::Invalid { .. } => invalid,
                        _ => LexNumberState::Invalid {
                            position,
                            reason: InvalidNumberReason::Exponent,
                        },
                    }
                }
                _ => {
                    break;
                }
            }
        }

        match state {
            LexNumberState::IntegerPart
            | LexNumberState::FractionalPart
            | LexNumberState::Exponent => JSON_NUMBER_LITERAL,
            LexNumberState::FirstDigit => {
                let err = ParseDiagnostic::new(
                    "Minus must be followed by a digit",
                    start..self.text_position(),
                );
                self.diagnostics.push(err);
                ERROR_TOKEN
            }
            LexNumberState::Invalid { position, reason } => {
                let diagnostic = match reason {
                    InvalidNumberReason::Fraction => ParseDiagnostic::new(

                        "Invalid fraction part",
                        position..position + TextSize::from(1),
                    ),
                    InvalidNumberReason::Exponent => ParseDiagnostic::new(

                        "Invalid exponent part",
                        position..position + TextSize::from(1),
                    ),
                    InvalidNumberReason::Octal => ParseDiagnostic::new(

                        "The JSON standard doesn't allow octal number notation (numbers starting with zero)",
                        position..position + TextSize::from(1),
                    ),
                    InvalidNumberReason::MissingExponent => {
                        ParseDiagnostic::new( "Missing exponent", start..position)
                            .with_detail(position..position + TextSize::from(1), "Expected a digit as the exponent")
                    }
                    InvalidNumberReason::MissingFraction => {
                        ParseDiagnostic::new( "Missing fraction", position..position + TextSize::from(1))
                            .with_hint("Remove the `.`")
                    }
                };

                self.diagnostics.push(diagnostic);
                ERROR_TOKEN
            }
        }
    }

    fn lex_string_literal(&mut self, quote: u8) -> JsonSyntaxKind {
        // Handle invalid quotes
        self.assert_at_char_boundary();
        let start = self.text_position();

        self.advance(1); // Skip over the quote
        let mut state = match quote {
            b'\'' => LexStringState::InvalidQuote,
            _ => LexStringState::InString,
        };

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
                        Some(b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't') => {
                            self.advance(1)
                        }

                        Some(b'u') => match (self.lex_unicode_escape(), state) {
                            (Ok(_), _) => {}
                            (Err(err), LexStringState::InString) => {
                                self.diagnostics.push(err);
                                state = LexStringState::InvalidEscapeSequence;
                            }
                            (Err(_), _) => {}
                        },

                        // Handle escaped `'` but only if this is a single quote string. The whole string will
                        // be marked as erroneous
                        Some(b'\'') if quote == b'\'' => {
                            self.advance(1);
                        }

                        Some(_) => {
                            if matches!(state, LexStringState::InString) {
                                let c = self.current_char_unchecked();
                                self.diagnostics.push(
                                    ParseDiagnostic::new(

                                        "Invalid escape sequence",
                                        escape_start..self.text_position() + c.text_len(),
                                    )
                                        .with_hint(r#"Valid escape sequences are: `\\`, `\/`, `/"`, `\b\`, `\f`, `\n`, `\r`, `\t` or any unicode escape sequence `\uXXXX` where X is hexedecimal number. "#),
                                );
                                state = LexStringState::InvalidEscapeSequence;
                            }
                        }

                        None => {
                            if matches!(state, LexStringState::InString) {
                                self.diagnostics.push(ParseDiagnostic::new(

                                    "Expected an escape sequence following a backslash, but found none",
                                    escape_start..self.text_position(),
                                )
                                    .with_detail(self.text_position()..self.text_position(), "File ends here")
                                );
                                state = LexStringState::InvalidEscapeSequence;
                            }
                        }
                    }
                }
                WHS if matches!(chr, b'\n' | b'\r') => {
                    let unterminated =
                        ParseDiagnostic::new("Missing closing quote", start..self.text_position())
                            .with_hint("The closing quote must be on the same line.");
                    self.diagnostics.push(unterminated);
                    return ERROR_TOKEN;
                }
                UNI => self.advance_char_unchecked(),

                // From the spec:
                // All code points may be placed within the quotation marks except for the code points that
                //must be escaped:
                // * quotation mark: (U+0022),
                // * reverse solidus (U+005C),
                // * and the **control characters U+0000 to U+001F** <- This
                ERR | WHS if matches!(state, LexStringState::InString) && chr <= 0x1f => {
                    self.diagnostics.push(
                        ParseDiagnostic::new(

                            format!(
                                "Control character '\\u{chr:04x}' is not allowed in string literals."
                            ),
                            self.text_position()..self.text_position() + TextSize::from(1),
                        )
                        .with_hint(format!("Use the escape sequence '\\u{chr:04x}' instead.")),
                    );
                    state = LexStringState::InvalidEscapeSequence;
                }
                _ => self.advance(1),
            }
        }

        match state {
            LexStringState::Terminated => JSON_STRING_LITERAL,
            LexStringState::InvalidQuote => {
                let literal_range = TextRange::new(start, self.text_position());
                self.diagnostics.push(
                    ParseDiagnostic::new(
                        "JSON standard does not allow single quoted strings",
                        literal_range,
                    )
                    .with_hint("Use double quotes to escape the string."),
                );
                ERROR_TOKEN
            }
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

    /// Lexes a `\u0000` escape sequence. Assumes that the lexer is positioned at the `u` token.
    ///
    /// A unicode escape sequence must consist of 4 hex characters.
    fn lex_unicode_escape(&mut self) -> Result<(), ParseDiagnostic> {
        self.assert_byte(b'u');
        self.assert_at_char_boundary();

        let start = self.text_position();

        let start = start
            // Subtract 1 to get position of `\`
            .checked_sub(TextSize::from(1))
            .unwrap_or(start);

        self.advance(1); // Advance over `u'`

        for _ in 0..4 {
            match self.current_byte() {
                Some(byte) if byte.is_ascii_hexdigit() => self.advance(1),
                Some(_) => {
                    let char = self.current_char_unchecked();
                    // Reached a non hex digit which is invalid
                    return Err(ParseDiagnostic::new(

                        "Invalid unicode sequence",
                        start..self.text_position(),
                    )
                    .with_detail(self.text_position()..self.text_position().add(char.text_len()), "Non hexadecimal number")
                    .with_hint("A unicode escape sequence must consist of 4 hexadecimal numbers: `\\uXXXX`, e.g. `\\u002F' for '/'."));
                }
                None => {
                    // Reached the end of the file before processing 4 hex digits
                    return Err(ParseDiagnostic::new(

                        "Unicode escape sequence with two few hexadecimal numbers.",
                        start..self.text_position(),
                    )
                    .with_detail(
                        self.text_position()..self.text_position(),
                        "reached the end of the file",
                    )
                    .with_hint("A unicode escape sequence must consist of 4 hexadecimal numbers: `\\uXXXX`, e.g. `\\u002F' for '/'."));
                }
            }
        }

        Ok(())
    }

    /// Implements basic lexing of identifiers without support for escape sequences.
    /// This is merely for improved error recovery as identifiers are not valid in JSON.
    fn lex_identifier(&mut self, first: u8) -> JsonSyntaxKind {
        self.assert_at_char_boundary();

        let mut keyword = KeywordMatcher::from_byte(first);

        self.advance_byte_or_char(first);

        while let Some(byte) = self.current_byte() {
            self.current_char_unchecked();
            match lookup_byte(byte) {
                IDT | DOL | DIG | ZER => {
                    keyword = keyword.next_character(byte);
                    self.advance(1)
                }
                UNI => {
                    let char = self.current_char_unchecked();
                    keyword = KeywordMatcher::None;
                    if is_js_id_continue(char) {
                        self.advance(char.len_utf8());
                    } else {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }
        }

        match keyword {
            KeywordMatcher::Null => NULL_KW,
            KeywordMatcher::True => TRUE_KW,
            KeywordMatcher::False => FALSE_KW,
            _ => IDENT,
        }
    }

    /// Lexes a comment. Comments are not supported in JSON but it should yield better error recovery.
    fn lex_slash(&mut self) -> JsonSyntaxKind {
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

                            if !self.options.allow_comments {
                                self.diagnostics.push(ParseDiagnostic::new(
                                    "JSON standard does not allow comments.",
                                    start..self.text_position(),
                                ));
                            }

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
            Some(b'/') => {
                self.advance(2);

                while let Some(chr) = self.current_byte() {
                    match chr {
                        b'\n' | b'\r' => return COMMENT,
                        chr => self.advance_byte_or_char(chr),
                    }
                }

                if !self.options.allow_comments {
                    self.diagnostics.push(ParseDiagnostic::new(
                        "JSON standard does not allow comments.",
                        start..self.text_position(),
                    ));
                }

                COMMENT
            }
            _ => self.eat_unexpected_character(),
        }
    }

    pub(crate) fn with_options(mut self, options: JsonParserOptions) -> Self {
        self.options = options;
        self
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

impl FusedIterator for Lexer<'_> {}

#[derive(Debug, Copy, Clone)]
enum LexNumberState {
    /// At the start, after a minus
    FirstDigit,

    /// Parsing the digits before the exponent or fractional (after .`) part
    IntegerPart,

    /// Parsing the digits after a `.`
    FractionalPart,

    /// Parsing the exponent digits (after a `e` or `E`)
    Exponent,

    /// Parsing the rest of an invalid number
    Invalid {
        reason: InvalidNumberReason,
        position: TextSize,
    },
}

#[derive(Copy, Clone, Debug)]
enum InvalidNumberReason {
    /// Fraction in an invalid position
    Fraction,
    /// Exponent in an invalid position
    Exponent,

    /// Missing digit after an `e` or `E`
    MissingExponent,

    /// Missing digit after faction (.)
    MissingFraction,

    /// Number starting with a 0
    Octal,
}

#[derive(Copy, Clone, Debug)]
enum LexStringState {
    /// When using `'` instead of `"`
    InvalidQuote,

    /// String that contains an invalid escape sequence
    InvalidEscapeSequence,

    /// Between the opening `"` and closing `"` quotes.
    InString,

    /// Properly terminated string
    Terminated,
}

enum KeywordMatcher {
    MaybeNull(u32),
    MaybeFalse(u32),
    MaybeTrue(u32),
    Null,
    False,
    True,
    None,
}

impl KeywordMatcher {
    fn from_byte(c: u8) -> KeywordMatcher {
        if c.is_ascii() {
            match c {
                b'n' => KeywordMatcher::MaybeNull(1),
                b't' => KeywordMatcher::MaybeTrue(1),
                b'f' => KeywordMatcher::MaybeFalse(1),
                _ => KeywordMatcher::None,
            }
        } else {
            KeywordMatcher::None
        }
    }

    fn next_character(self, next: u8) -> KeywordMatcher {
        match self {
            KeywordMatcher::MaybeNull(position) => match (next, position) {
                (b'u', 1) => KeywordMatcher::MaybeNull(2),
                (b'l', 2) => KeywordMatcher::MaybeNull(3),
                (b'l', 3) => KeywordMatcher::Null,
                _ => KeywordMatcher::None,
            },
            KeywordMatcher::MaybeFalse(position) => match (next, position) {
                (b'a', 1) => KeywordMatcher::MaybeFalse(2),
                (b'l', 2) => KeywordMatcher::MaybeFalse(3),
                (b's', 3) => KeywordMatcher::MaybeFalse(4),
                (b'e', 4) => KeywordMatcher::False,
                _ => KeywordMatcher::None,
            },
            KeywordMatcher::MaybeTrue(position) => match (next, position) {
                (b'r', 1) => KeywordMatcher::MaybeTrue(2),
                (b'u', 2) => KeywordMatcher::MaybeTrue(3),
                (b'e', 3) => KeywordMatcher::True,
                _ => KeywordMatcher::None,
            },
            KeywordMatcher::None
            | KeywordMatcher::Null
            | KeywordMatcher::False
            | KeywordMatcher::True => KeywordMatcher::None,
        }
    }
}
