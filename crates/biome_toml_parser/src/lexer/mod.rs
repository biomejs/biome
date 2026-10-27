//! A byte-oriented TOML 1.1 lexer.
//!
//! TOML tokens depend on their position: `1.5` is a float on the right-hand
//! side of `=` but the dotted key `1`.`5` on the left-hand side, and `[[` opens
//! an array-of-tables header at the start of a line but two nested arrays in a
//! value. The parser selects a [`TomlLexContext`] for every token it requests.
//!
//! The lexer validates the lexical rules of the specification (escape
//! sequences, control characters, number formats, date-time ranges) and reports
//! violations as diagnostics while still producing a token, so the tree stays
//! lossless and the parser can continue.

#[rustfmt::skip]
mod tests;

use biome_diagnostics::Diagnostic;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{
    LexContext, Lexer, LexerCheckpoint, LexerWithCheckpoint, ReLexer, TokenFlags,
};
use biome_rowan::SyntaxKind;
use biome_toml_syntax::{T, TextRange, TextSize, TomlSyntaxKind, TomlSyntaxKind::*};
use biome_unicode_table::{Dispatch::*, lookup_byte};

/// Selects how the lexer interprets the next token.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub(crate) enum TomlLexContext {
    /// Keys, table headers, and the punctuation around them.
    ///
    /// Words made of `A-Za-z0-9_-` are bare keys, and `[[`/`]]` are lexed as
    /// single tokens.
    #[default]
    Key,
    /// Values: the right-hand side of `=` and the elements of an array.
    ///
    /// Words are numbers, booleans, or date-times, and every bracket is a
    /// single token.
    Value,
}

impl LexContext for TomlLexContext {
    fn is_regular(&self) -> bool {
        matches!(self, Self::Key)
    }
}

pub(crate) struct TomlLexer<'src> {
    source: &'src str,

    /// The start byte position in the source text of the next token.
    position: usize,

    /// Byte offset of the current token from the start of the source.
    current_start: TextSize,

    current_kind: TomlSyntaxKind,

    current_flags: TokenFlags,

    /// `true` if a newline was lexed since the last non-trivia token.
    after_newline: bool,

    /// If the source starts with a Unicode BOM, this is the number of bytes for that token.
    unicode_bom_length: usize,

    diagnostics: Vec<ParseDiagnostic>,
}

impl<'src> Lexer<'src> for TomlLexer<'src> {
    const NEWLINE: Self::Kind = NEWLINE;
    const WHITESPACE: Self::Kind = WHITESPACE;

    type Kind = TomlSyntaxKind;
    type LexContext = TomlLexContext;
    type ReLexContext = TomlLexContext;

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

        let kind = match self.current_byte() {
            Some(current) => self.consume_token(current, context),
            None => EOF,
        };

        self.current_flags
            .set(TokenFlags::PRECEDING_LINE_BREAK, self.after_newline);
        self.current_kind = kind;

        if kind == NEWLINE {
            self.after_newline = true;
        } else if !kind.is_trivia() {
            self.after_newline = false;
        }

        kind
    }

    fn has_preceding_line_break(&self) -> bool {
        self.current_flags.has_preceding_line_break()
    }

    fn has_unicode_escape(&self) -> bool {
        false
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
            ..
        } = checkpoint;

        self.position = usize::from(position);
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

    fn position(&self) -> usize {
        self.position
    }

    fn push_diagnostic(&mut self, diagnostic: ParseDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    #[inline]
    fn advance_char_unchecked(&mut self) {
        let c = self.current_char_unchecked();
        self.position += c.len_utf8();
    }

    #[inline]
    fn advance(&mut self, n: usize) {
        self.position += n;
    }
}

impl<'src> LexerWithCheckpoint<'src> for TomlLexer<'src> {
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

impl<'src> ReLexer<'src> for TomlLexer<'src> {
    /// Lexes the current token again in `context`. The token keeps its flags,
    /// including whether it follows a line break.
    fn re_lex(&mut self, context: Self::ReLexContext) -> Self::Kind {
        self.position = usize::from(self.current_start);
        // Drop the diagnostics of the previous interpretation. The lexer
        // reports diagnostics in source order, inside the token's range.
        while self.diagnostics.last().is_some_and(|diagnostic| {
            diagnostic
                .location()
                .span
                .is_some_and(|range| range.start() >= self.current_start)
        }) {
            self.diagnostics.pop();
        }

        let kind = match self.current_byte() {
            Some(current) => self.consume_token(current, context),
            None => EOF,
        };

        self.current_kind = kind;
        kind
    }
}

impl<'src> TomlLexer<'src> {
    pub fn from_str(source: &'src str) -> Self {
        Self {
            source,
            position: 0,
            current_start: TextSize::from(0),
            current_kind: TOMBSTONE,
            current_flags: TokenFlags::empty(),
            after_newline: false,
            unicode_bom_length: 0,
            diagnostics: Vec::new(),
        }
    }

    /// Lexes the next token. Must not be called at the end of the file.
    fn consume_token(&mut self, current: u8, context: TomlLexContext) -> TomlSyntaxKind {
        match lookup_byte(current) {
            WHS => match current {
                b' ' | b'\t' => self.consume_whitespace(),
                b'\n' => self.consume_byte(NEWLINE),
                b'\r' => self.consume_carriage_return(),
                _ => self.consume_unexpected_character(),
            },
            HAS => self.consume_comment(),
            QOT if current == b'"' => self.consume_basic_string(),
            QOT => self.consume_literal_string(),
            BTO if context == TomlLexContext::Key && self.peek_byte() == Some(b'[') => {
                self.advance(2);
                T!["[["]
            }
            BTO => self.consume_byte(T!['[']),
            BTC if context == TomlLexContext::Key && self.peek_byte() == Some(b']') => {
                self.advance(2);
                T!["]]"]
            }
            BTC => self.consume_byte(T![']']),
            BEO => self.consume_byte(T!['{']),
            BEC => self.consume_byte(T!['}']),
            EQL => self.consume_byte(T![=]),
            COM => self.consume_byte(T![,]),
            IDT | ZER | DIG | MIN => match context {
                TomlLexContext::Key => self.consume_bare_key(),
                TomlLexContext::Value => self.consume_value_word(),
            },
            PRD => match context {
                TomlLexContext::Key => self.consume_byte(T![.]),
                TomlLexContext::Value => self.consume_value_word(),
            },
            PLS | COL if context == TomlLexContext::Value => self.consume_value_word(),
            UNI if self.position == 0 => {
                if let Some((bom, bom_size)) = self.consume_potential_bom(UNICODE_BOM) {
                    self.unicode_bom_length = bom_size;
                    bom
                } else {
                    self.consume_unexpected_character()
                }
            }
            _ => self.consume_unexpected_character(),
        }
    }

    #[inline]
    fn consume_byte(&mut self, kind: TomlSyntaxKind) -> TomlSyntaxKind {
        self.advance(1);
        kind
    }

    fn consume_whitespace(&mut self) -> TomlSyntaxKind {
        while matches!(self.current_byte(), Some(b' ' | b'\t')) {
            self.advance(1);
        }
        WHITESPACE
    }

    /// Consumes a `\r`. TOML only accepts it as part of a CRLF line ending.
    fn consume_carriage_return(&mut self) -> TomlSyntaxKind {
        if self.peek_byte() == Some(b'\n') {
            self.advance(2);
        } else {
            let start = self.text_position();
            self.advance(1);
            self.push_diagnostic(
                ParseDiagnostic::new(
                    "A carriage return must be followed by a line feed.",
                    start..self.text_position(),
                )
                .with_hint("TOML only allows LF and CRLF line endings."),
            );
        }
        NEWLINE
    }

    fn consume_comment(&mut self) -> TomlSyntaxKind {
        self.assert_byte(b'#');
        self.advance(1);

        while let Some(byte) = self.current_byte() {
            match byte {
                b'\n' => break,
                b'\r' if self.peek_byte() == Some(b'\n') => break,
                b'\t' | 0x20..=0x7E => self.advance(1),
                _ if byte >= 0x80 => self.advance_char_unchecked(),
                _ => self.consume_control_character("comments"),
            }
        }

        COMMENT
    }

    fn consume_bare_key(&mut self) -> TomlSyntaxKind {
        while let Some(byte) = self.current_byte() {
            if is_bare_key_byte(byte) {
                self.advance(1);
            } else {
                break;
            }
        }
        TOML_BARE_KEY_LITERAL
    }

    fn consume_unexpected_character(&mut self) -> TomlSyntaxKind {
        let start = self.text_position();
        let char = self.current_char_unchecked();
        self.advance_char_unchecked();

        let diagnostic = if char.is_whitespace() {
            ParseDiagnostic::new(
                format!("Unexpected whitespace character U+{:04X}.", char as u32),
                start..self.text_position(),
            )
            .with_hint("TOML only allows spaces and tabs as whitespace.")
        } else if char.is_control() {
            ParseDiagnostic::new(
                format!("Unexpected control character U+{:04X}.", char as u32),
                start..self.text_position(),
            )
        } else {
            ParseDiagnostic::new(
                format!("Unexpected character `{char}`."),
                start..self.text_position(),
            )
        };

        self.push_diagnostic(diagnostic);
        ERROR_TOKEN
    }

    /// Reports and skips a control character that isn't allowed in `place`.
    fn consume_control_character(&mut self, place: &str) {
        let start = self.text_position();
        let char = self.current_char_unchecked();
        self.advance_char_unchecked();
        self.push_diagnostic(
            ParseDiagnostic::new(
                format!(
                    "Control character U+{:04X} isn't allowed in {place}.",
                    char as u32
                ),
                start..self.text_position(),
            )
            .with_hint("Use an escape sequence in a basic string instead."),
        );
    }

    fn consume_basic_string(&mut self) -> TomlSyntaxKind {
        let start = self.text_position();
        self.assert_byte(b'"');

        if self.peek_byte() == Some(b'"') && self.byte_at(2) == Some(b'"') {
            self.advance(3);
            return self.consume_multiline_basic_string_body(start);
        }

        self.advance(1);

        loop {
            match self.current_byte() {
                Some(b'"') => {
                    self.advance(1);
                    break;
                }
                Some(b'\\') => self.consume_escape_sequence(false),
                None | Some(b'\n') => {
                    self.report_unterminated_string(start, "\"");
                    break;
                }
                Some(b'\r') if self.peek_byte() == Some(b'\n') => {
                    self.report_unterminated_string(start, "\"");
                    break;
                }
                Some(b'\t' | 0x20..=0x7E) => self.advance(1),
                Some(byte) if byte >= 0x80 => self.advance_char_unchecked(),
                Some(_) => self.consume_control_character("basic strings"),
            }
        }

        TOML_BASIC_STRING_LITERAL
    }

    fn consume_multiline_basic_string_body(&mut self, start: TextSize) -> TomlSyntaxKind {
        loop {
            match self.current_byte() {
                Some(b'"') => {
                    if self.consume_multiline_closing_quotes(b'"') {
                        break;
                    }
                }
                Some(b'\\') => self.consume_escape_sequence(true),
                Some(b'\n') => self.advance(1),
                Some(b'\r') if self.peek_byte() == Some(b'\n') => self.advance(2),
                Some(b'\t' | 0x20..=0x7E) => self.advance(1),
                Some(byte) if byte >= 0x80 => self.advance_char_unchecked(),
                Some(_) => self.consume_control_character("multi-line basic strings"),
                None => {
                    self.report_unterminated_string(start, "\"\"\"");
                    break;
                }
            }
        }

        TOML_ML_BASIC_STRING_LITERAL
    }

    fn consume_literal_string(&mut self) -> TomlSyntaxKind {
        let start = self.text_position();
        self.assert_byte(b'\'');

        if self.peek_byte() == Some(b'\'') && self.byte_at(2) == Some(b'\'') {
            self.advance(3);
            return self.consume_multiline_literal_string_body(start);
        }

        self.advance(1);

        loop {
            match self.current_byte() {
                Some(b'\'') => {
                    self.advance(1);
                    break;
                }
                None | Some(b'\n') => {
                    self.report_unterminated_string(start, "'");
                    break;
                }
                Some(b'\r') if self.peek_byte() == Some(b'\n') => {
                    self.report_unterminated_string(start, "'");
                    break;
                }
                Some(b'\t' | 0x20..=0x7E) => self.advance(1),
                Some(byte) if byte >= 0x80 => self.advance_char_unchecked(),
                Some(_) => self.consume_control_character("literal strings"),
            }
        }

        TOML_LITERAL_STRING_LITERAL
    }

    fn consume_multiline_literal_string_body(&mut self, start: TextSize) -> TomlSyntaxKind {
        loop {
            match self.current_byte() {
                Some(b'\'') => {
                    if self.consume_multiline_closing_quotes(b'\'') {
                        break;
                    }
                }
                Some(b'\n') => self.advance(1),
                Some(b'\r') if self.peek_byte() == Some(b'\n') => self.advance(2),
                Some(b'\t' | 0x20..=0x7E) => self.advance(1),
                Some(byte) if byte >= 0x80 => self.advance_char_unchecked(),
                Some(_) => self.consume_control_character("multi-line literal strings"),
                None => {
                    self.report_unterminated_string(start, "'''");
                    break;
                }
            }
        }

        TOML_ML_LITERAL_STRING_LITERAL
    }

    /// Consumes a run of `quote` bytes inside a multi-line string and returns
    /// `true` if the run closes the string.
    ///
    /// The body may end with one or two quotes right before the closing
    /// delimiter, so a run of three to five quotes closes the string. Longer
    /// runs are reported and still close the string.
    fn consume_multiline_closing_quotes(&mut self, quote: u8) -> bool {
        let start = self.text_position();
        let mut count = 0;
        while self.current_byte() == Some(quote) {
            self.advance(1);
            count += 1;
        }

        if count > 5 {
            self.push_diagnostic(
                ParseDiagnostic::new(
                    "Too many quotes at the end of a multi-line string.",
                    start..self.text_position(),
                )
                .with_hint(
                    "A multi-line string can end with at most two quotes before its closing delimiter.",
                ),
            );
        }

        count >= 3
    }

    fn report_unterminated_string(&mut self, start: TextSize, delimiter: &str) {
        self.push_diagnostic(
            ParseDiagnostic::new("Missing closing quote.", start..self.text_position())
                .with_detail(
                    self.text_position()..self.text_position(),
                    format!("The string should be closed with `{delimiter}` here."),
                ),
        );
    }

    /// Consumes an escape sequence that starts at the current `\`.
    ///
    /// In multi-line basic strings, a `\` followed by optional whitespace and a
    /// newline is a line-ending backslash.
    fn consume_escape_sequence(&mut self, multiline: bool) {
        let start = self.text_position();
        self.assert_byte(b'\\');
        self.advance(1);

        match self.current_byte() {
            Some(b'b' | b't' | b'n' | b'f' | b'r' | b'e' | b'"' | b'\\') => self.advance(1),
            Some(b'x') => {
                self.advance(1);
                self.consume_unicode_escape(start, 2);
            }
            Some(b'u') => {
                self.advance(1);
                self.consume_unicode_escape(start, 4);
            }
            Some(b'U') => {
                self.advance(1);
                self.consume_unicode_escape(start, 8);
            }
            Some(b' ' | b'\t' | b'\n' | b'\r') if multiline => {
                let mut offset = 0;
                while matches!(self.byte_at(offset), Some(b' ' | b'\t')) {
                    offset += 1;
                }
                match self.byte_at(offset) {
                    Some(b'\n') => self.advance(offset + 1),
                    Some(b'\r') if self.byte_at(offset + 1) == Some(b'\n') => {
                        self.advance(offset + 2)
                    }
                    _ => {
                        self.advance(offset);
                        self.push_diagnostic(
                            ParseDiagnostic::new(
                                "A line-ending backslash must only be followed by whitespace and a newline.",
                                start..self.text_position(),
                            )
                            .with_hint("Use `\\\\` to write a literal backslash."),
                        );
                    }
                }
            }
            Some(byte) => {
                if byte >= 0x80 {
                    self.advance_char_unchecked();
                } else if byte != b'\n' && byte != b'\r' {
                    self.advance(1);
                }
                self.push_diagnostic(
                    ParseDiagnostic::new("Invalid escape sequence.", start..self.text_position())
                        .with_hint(
                            "Valid escape sequences are `\\b`, `\\t`, `\\n`, `\\f`, `\\r`, `\\e`, `\\\"`, `\\\\`, `\\xHH`, `\\uHHHH`, and `\\UHHHHHHHH`.",
                        ),
                );
            }
            None => self.push_diagnostic(ParseDiagnostic::new(
                "Invalid escape sequence.",
                start..self.text_position(),
            )),
        }
    }

    /// Consumes the `digits` hex digits of a `\x`, `\u`, or `\U` escape whose
    /// backslash is at `start`.
    fn consume_unicode_escape(&mut self, start: TextSize, digits: usize) {
        let mut value: u32 = 0;
        for _ in 0..digits {
            match self.current_byte() {
                Some(byte) if byte.is_ascii_hexdigit() => {
                    value = (value << 4) | (byte as char).to_digit(16).unwrap_or_default();
                    self.advance(1);
                }
                _ => {
                    self.push_diagnostic(ParseDiagnostic::new(
                        format!("Expected {digits} hexadecimal digits in this escape sequence."),
                        start..self.text_position(),
                    ));
                    return;
                }
            }
        }

        if char::from_u32(value).is_none() {
            self.push_diagnostic(
                ParseDiagnostic::new(
                    format!("U+{value:X} isn't a Unicode scalar value."),
                    start..self.text_position(),
                )
                .with_hint("Surrogate code points and values above U+10FFFF can't be escaped."),
            );
        }
    }

    /// Consumes a number, boolean, special float, or date-time.
    fn consume_value_word(&mut self) -> TomlSyntaxKind {
        let start = self.position;
        self.consume_value_word_bytes();

        // A space may separate the date and time of a date-time. Only treat it
        // as a delimiter when a time follows, so `1979-05-27 # comment` still
        // lexes as a local date.
        if self.position - start == 10
            && self.current_byte() == Some(b' ')
            && is_full_date(&self.source.as_bytes()[start..self.position])
            && matches!(
                (self.byte_at(1), self.byte_at(2), self.byte_at(3)),
                (Some(b'0'..=b'9'), Some(b'0'..=b'9'), Some(b':'))
            )
        {
            self.advance(1);
            self.consume_value_word_bytes();
        }

        let word = &self.source.as_bytes()[start..self.position];
        let range = TextRange::new(
            TextSize::from(start as u32),
            TextSize::from(self.position as u32),
        );

        match classify_value(word) {
            Ok(kind) => kind,
            Err(ValueError {
                kind,
                message,
                hint,
            }) => {
                let mut diagnostic = ParseDiagnostic::new(message, range);
                if let Some(hint) = hint {
                    diagnostic = diagnostic.with_hint(hint);
                }
                self.push_diagnostic(diagnostic);
                kind
            }
        }
    }

    fn consume_value_word_bytes(&mut self) {
        while let Some(byte) = self.current_byte() {
            if is_value_word_byte(byte) {
                self.advance(1);
            } else {
                break;
            }
        }
    }
}

/// Bare keys: `A-Za-z0-9_-`.
#[inline]
fn is_bare_key_byte(byte: u8) -> bool {
    matches!(lookup_byte(byte), IDT | ZER | DIG | MIN)
}

/// Bytes that can appear in a number, boolean, special float, or date-time.
#[inline]
fn is_value_word_byte(byte: u8) -> bool {
    matches!(lookup_byte(byte), IDT | ZER | DIG | MIN | PLS | PRD | COL)
}

struct ValueError {
    /// The kind to use for the invalid token.
    kind: TomlSyntaxKind,
    message: &'static str,
    hint: Option<&'static str>,
}

impl ValueError {
    const fn new(kind: TomlSyntaxKind, message: &'static str) -> Self {
        Self {
            kind,
            message,
            hint: None,
        }
    }

    const fn with_hint(mut self, hint: &'static str) -> Self {
        self.hint = Some(hint);
        self
    }
}

type ValueResult = Result<TomlSyntaxKind, ValueError>;

fn classify_value(word: &[u8]) -> ValueResult {
    match word {
        b"true" => return Ok(T![true]),
        b"false" => return Ok(T![false]),
        b"inf" | b"+inf" | b"-inf" | b"nan" | b"+nan" | b"-nan" => return Ok(TOML_FLOAT_LITERAL),
        _ => {}
    }

    if word.len() >= 10 && word[4] == b'-' && word[7] == b'-' {
        return classify_date_time(word);
    }

    if word.len() >= 5 && word[2] == b':' {
        return classify_local_time(word);
    }

    let first = word[0];
    if first.is_ascii_digit() || first == b'+' || first == b'-' {
        return classify_number(word);
    }

    if word.eq_ignore_ascii_case(b"true") || word.eq_ignore_ascii_case(b"false") {
        return Err(ValueError::new(ERROR_TOKEN, "Booleans must be lowercase.")
            .with_hint("Use `true` or `false`."));
    }

    if word.eq_ignore_ascii_case(b"inf") || word.eq_ignore_ascii_case(b"nan") {
        return Err(ValueError::new(
            ERROR_TOKEN,
            "Special float values must be lowercase.",
        ));
    }

    Err(ValueError::new(ERROR_TOKEN, "Invalid value.")
        .with_hint("Strings must be quoted, for example `\"value\"`."))
}

fn classify_number(word: &[u8]) -> ValueResult {
    let (negative, unsigned) = match word[0] {
        b'+' => (false, &word[1..]),
        b'-' => (true, &word[1..]),
        _ => (false, word),
    };
    let signed = unsigned.len() != word.len();

    match unsigned {
        b"inf" | b"nan" => return Ok(TOML_FLOAT_LITERAL),
        [] => return Err(ValueError::new(ERROR_TOKEN, "Invalid number.")),
        _ => {}
    }

    if let [
        b'0',
        prefix @ (b'x' | b'o' | b'b' | b'X' | b'O' | b'B'),
        digits @ ..,
    ] = unsigned
    {
        let radix = match prefix.to_ascii_lowercase() {
            b'x' => 16,
            b'o' => 8,
            _ => 2,
        };

        if prefix.is_ascii_uppercase() {
            return Err(ValueError::new(
                TOML_INTEGER_LITERAL,
                "Integer prefixes must be lowercase.",
            )
            .with_hint("Use `0x`, `0o`, or `0b`."));
        }

        if signed {
            return Err(ValueError::new(
                TOML_INTEGER_LITERAL,
                "Hexadecimal, octal, and binary integers can't have a sign.",
            ));
        }

        let value = parse_digits(digits, radix).map_err(|error| error.into_value_error())?;
        if value > i64::MAX as u64 {
            return Err(integer_out_of_range());
        }

        return Ok(TOML_INTEGER_LITERAL);
    }

    let int_end = unsigned
        .iter()
        .position(|byte| matches!(byte, b'.' | b'e' | b'E'))
        .unwrap_or(unsigned.len());
    let (int_part, rest) = unsigned.split_at(int_end);

    let int_value = parse_digits(int_part, 10).map_err(|error| error.into_value_error())?;
    if int_part.len() > 1 && int_part[0] == b'0' {
        return Err(ValueError::new(
            if rest.is_empty() {
                TOML_INTEGER_LITERAL
            } else {
                TOML_FLOAT_LITERAL
            },
            "Leading zeros aren't allowed.",
        ));
    }

    if rest.is_empty() {
        let max = if negative {
            i64::MIN.unsigned_abs()
        } else {
            i64::MAX as u64
        };
        if int_value > max {
            return Err(integer_out_of_range());
        }
        return Ok(TOML_INTEGER_LITERAL);
    }

    let mut rest = rest;
    if let [b'.', fraction @ ..] = rest {
        let fraction_end = fraction
            .iter()
            .position(|byte| matches!(byte, b'e' | b'E'))
            .unwrap_or(fraction.len());
        let (digits, after) = fraction.split_at(fraction_end);
        parse_digits(digits, 10).map_err(|error| error.into_float_error())?;
        rest = after;
    }

    if let [b'e' | b'E', exponent @ ..] = rest {
        let digits = match exponent {
            [b'+' | b'-', digits @ ..] => digits,
            digits => digits,
        };
        parse_digits(digits, 10).map_err(|error| error.into_float_error())?;
        rest = &[];
    }

    if rest.is_empty() {
        Ok(TOML_FLOAT_LITERAL)
    } else {
        Err(ValueError::new(TOML_FLOAT_LITERAL, "Invalid float."))
    }
}

fn integer_out_of_range() -> ValueError {
    ValueError::new(
        TOML_INTEGER_LITERAL,
        "This integer doesn't fit in a 64-bit signed integer.",
    )
}

enum DigitsError {
    Empty,
    InvalidDigit,
    Underscore,
}

impl DigitsError {
    fn into_value_error(self) -> ValueError {
        match self {
            Self::Empty => ValueError::new(TOML_INTEGER_LITERAL, "Expected digits."),
            Self::InvalidDigit => ValueError::new(ERROR_TOKEN, "Invalid number."),
            Self::Underscore => underscore_error(TOML_INTEGER_LITERAL),
        }
    }

    fn into_float_error(self) -> ValueError {
        match self {
            Self::Empty => ValueError::new(TOML_FLOAT_LITERAL, "Expected digits.")
                .with_hint("A decimal point and an exponent must be followed by digits."),
            Self::InvalidDigit => ValueError::new(TOML_FLOAT_LITERAL, "Invalid float."),
            Self::Underscore => underscore_error(TOML_FLOAT_LITERAL),
        }
    }
}

fn underscore_error(kind: TomlSyntaxKind) -> ValueError {
    ValueError::new(kind, "An underscore must be surrounded by digits.")
}

/// Parses `digits` in `radix`, allowing single underscores between digits.
///
/// Returns the value saturated at `u64::MAX`.
fn parse_digits(digits: &[u8], radix: u32) -> Result<u64, DigitsError> {
    if digits.is_empty() {
        return Err(DigitsError::Empty);
    }

    let mut value: u64 = 0;
    let mut previous_is_digit = false;
    for &byte in digits {
        if byte == b'_' {
            if !previous_is_digit {
                return Err(DigitsError::Underscore);
            }
            previous_is_digit = false;
            continue;
        }

        let Some(digit) = (byte as char).to_digit(radix) else {
            return Err(DigitsError::InvalidDigit);
        };
        value = value
            .saturating_mul(u64::from(radix))
            .saturating_add(u64::from(digit));
        previous_is_digit = true;
    }

    if previous_is_digit {
        Ok(value)
    } else {
        Err(DigitsError::Underscore)
    }
}

fn is_full_date(word: &[u8]) -> bool {
    word.len() == 10
        && word[4] == b'-'
        && word[7] == b'-'
        && word[..4].iter().all(u8::is_ascii_digit)
        && word[5..7].iter().all(u8::is_ascii_digit)
        && word[8..10].iter().all(u8::is_ascii_digit)
}

fn classify_date_time(word: &[u8]) -> ValueResult {
    let invalid = || ValueError::new(ERROR_TOKEN, "Invalid date-time.");

    if !is_full_date(&word[..10]) {
        return Err(invalid().with_hint("Dates use the `YYYY-MM-DD` format."));
    }

    let year = two_digits(&word[0..2]) * 100 + two_digits(&word[2..4]);
    let month = two_digits(&word[5..7]);
    let day = two_digits(&word[8..10]);

    if !(1..=12).contains(&month) {
        return Err(ValueError::new(
            TOML_LOCAL_DATE_LITERAL,
            "The month must be between 01 and 12.",
        ));
    }

    if day < 1 || day > days_in_month(year, month) {
        return Err(ValueError::new(
            TOML_LOCAL_DATE_LITERAL,
            "The day doesn't exist in this month.",
        ));
    }

    let Some((&delimiter, time)) = word[10..].split_first() else {
        return Ok(TOML_LOCAL_DATE_LITERAL);
    };

    if !matches!(delimiter, b'T' | b't' | b' ') {
        return Err(invalid().with_hint("Separate the date and the time with `T` or a space."));
    }

    let offset_start = time
        .iter()
        .position(|byte| matches!(byte, b'Z' | b'z' | b'+' | b'-'))
        .unwrap_or(time.len());
    let (time, offset) = time.split_at(offset_start);

    let time = validate_partial_time(time)?;

    // Offset from UTC, in minutes.
    let offset = match offset {
        [] => return Ok(TOML_LOCAL_DATE_TIME_LITERAL),
        [b'Z' | b'z'] => 0,
        [sign @ (b'+' | b'-'), h1, h2, b':', m1, m2]
            if [h1, h2, m1, m2].iter().all(|b| b.is_ascii_digit()) =>
        {
            let hours = two_digits(&[*h1, *h2]);
            let minutes = two_digits(&[*m1, *m2]);
            if hours > 23 || minutes > 59 {
                return Err(ValueError::new(
                    TOML_OFFSET_DATE_TIME_LITERAL,
                    "Invalid time offset.",
                ));
            }
            let offset = (hours * 60 + minutes) as i32;
            if *sign == b'-' { -offset } else { offset }
        }
        _ => return Err(invalid().with_hint("Time offsets use the `Z` or `+HH:MM` format.")),
    };

    if time.second == 60 && !is_leap_second(year, month, day, time, offset) {
        return Err(ValueError::new(
            TOML_OFFSET_DATE_TIME_LITERAL,
            "This time isn't a leap second.",
        )
        .with_hint("A second of 60 is only valid at 23:59:60 UTC on a date with a leap second."));
    }

    Ok(TOML_OFFSET_DATE_TIME_LITERAL)
}

/// The UTC dates whose last minute had a leap second, as `(year, month, day)`.
///
/// Source: IERS Bulletin C. No leap second has been inserted since 2016.
const LEAP_SECOND_DATES: [(u32, u32, u32); 27] = [
    (1972, 6, 30),
    (1972, 12, 31),
    (1973, 12, 31),
    (1974, 12, 31),
    (1975, 12, 31),
    (1976, 12, 31),
    (1977, 12, 31),
    (1978, 12, 31),
    (1979, 12, 31),
    (1981, 6, 30),
    (1982, 6, 30),
    (1983, 6, 30),
    (1985, 6, 30),
    (1987, 12, 31),
    (1989, 12, 31),
    (1990, 12, 31),
    (1992, 6, 30),
    (1993, 6, 30),
    (1994, 6, 30),
    (1995, 12, 31),
    (1997, 6, 30),
    (1998, 12, 31),
    (2005, 12, 31),
    (2008, 12, 31),
    (2012, 6, 30),
    (2015, 6, 30),
    (2016, 12, 31),
];

/// The first year whose leap seconds aren't known when this table was last
/// updated. From this year on, any June 30 or December 31, the dates IERS
/// prefers for insertions, is accepted.
const FIRST_UNKNOWN_LEAP_SECOND_YEAR: u32 = 2026;

/// Returns `true` if the date-time is 23:59:60 UTC on a date that had, or may
/// still get, a leap second. RFC 3339 only allows a second of 60 at such
/// instants.
fn is_leap_second(year: u32, month: u32, day: u32, time: PartialTime, offset: i32) -> bool {
    let utc_minutes = (time.hour * 60 + time.minute) as i32 - offset;
    if utc_minutes.rem_euclid(24 * 60) != 23 * 60 + 59 {
        return false;
    }

    // The offset is less than a day, so 23:59 UTC is on the same date or on
    // the previous day.
    let (year, month, day) = match utc_minutes.div_euclid(24 * 60) {
        0 => (year, month, day),
        -1 if day > 1 => (year, month, day - 1),
        -1 if month > 1 => (year, month - 1, days_in_month(year, month - 1)),
        -1 => (year.wrapping_sub(1), 12, 31),
        _ => return false,
    };

    if year >= FIRST_UNKNOWN_LEAP_SECOND_YEAR {
        matches!((month, day), (6, 30) | (12, 31))
    } else {
        LEAP_SECOND_DATES.contains(&(year, month, day))
    }
}

fn classify_local_time(word: &[u8]) -> ValueResult {
    validate_partial_time(word)?;
    Ok(TOML_LOCAL_TIME_LITERAL)
}

#[derive(Debug, Copy, Clone)]
struct PartialTime {
    hour: u32,
    minute: u32,
    second: u32,
}

/// Validates `HH:MM[:SS[.fraction]]`.
///
/// A second of 60 is accepted because it can be a leap second. Without an
/// offset, the UTC instant is unknown, so only offset date-times can be
/// checked further.
fn validate_partial_time(time: &[u8]) -> Result<PartialTime, ValueError> {
    let invalid = || {
        ValueError::new(ERROR_TOKEN, "Invalid time.")
            .with_hint("Times use the `HH:MM:SS` format, optionally followed by a fraction.")
    };

    let (hour, minute, rest) = match time {
        [h1, h2, b':', m1, m2, rest @ ..]
            if [h1, h2, m1, m2].iter().all(|byte| byte.is_ascii_digit()) =>
        {
            (two_digits(&[*h1, *h2]), two_digits(&[*m1, *m2]), rest)
        }
        _ => return Err(invalid()),
    };

    let second = match rest {
        [] => 0,
        [b':', s1, s2, fraction @ ..] if s1.is_ascii_digit() && s2.is_ascii_digit() => {
            match fraction {
                [] => {}
                [b'.', digits @ ..]
                    if !digits.is_empty() && digits.iter().all(u8::is_ascii_digit) => {}
                _ => return Err(invalid()),
            }
            two_digits(&[*s1, *s2])
        }
        _ => return Err(invalid()),
    };

    if hour > 23 || minute > 59 || second > 60 {
        return Err(ValueError::new(ERROR_TOKEN, "The time is out of range."));
    }

    Ok(PartialTime {
        hour,
        minute,
        second,
    })
}

#[inline]
fn two_digits(bytes: &[u8]) -> u32 {
    u32::from(bytes[0] - b'0') * 10 + u32::from(bytes[1] - b'0')
}

fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        2 if year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400)) => {
            29
        }
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}
