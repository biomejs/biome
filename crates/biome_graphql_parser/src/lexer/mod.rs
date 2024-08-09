//! An extremely fast, lookup table based, GraphQL lexer which yields SyntaxKind tokens used by the biome GraphQL parser.
#[rustfmt::skip]
mod tests;

use biome_graphql_syntax::{GraphqlSyntaxKind, GraphqlSyntaxKind::*, TextLen, TextSize, T};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{Lexer, LexerCheckpoint, LexerWithCheckpoint, TokenFlags};
use biome_rowan::SyntaxKind;
use std::ops::Add;

#[derive(Debug)]
pub struct GraphqlLexer<'src> {
    /// Source text
    source: &'src str,

    /// The start byte position in the source text of the next token.
    position: usize,

    /// If the source starts with a Unicode BOM, this is the number of bytes for that token.
    unicode_bom_length: usize,

    /// Byte offset of the current token from the start of the source
    /// The range of the current token can be computed by
    /// `self.position - self.current_start`.
    current_start: TextSize,

    /// The kind of the current token
    current_kind: GraphqlSyntaxKind,

    /// Flags for the current token
    current_flags: TokenFlags,

    diagnostics: Vec<ParseDiagnostic>,
}

impl<'src> Lexer<'src> for GraphqlLexer<'src> {
    const NEWLINE: Self::Kind = NEWLINE;
    const WHITESPACE: Self::Kind = WHITESPACE;

    type Kind = GraphqlSyntaxKind;
    type LexContext = ();
    type ReLexContext = ();

    fn source(&self) -> &'src str {
        self.source
    }

    fn current(&self) -> Self::Kind {
        self.current_kind
    }

    fn position(&self) -> usize {
        self.position
    }

    fn current_start(&self) -> TextSize {
        self.current_start
    }

    fn push_diagnostic(&mut self, diagnostic: ParseDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    fn has_unicode_escape(&self) -> bool {
        self.current_flags().has_unicode_escape()
    }

    fn has_preceding_line_break(&self) -> bool {
        self.current_flags().has_preceding_line_break()
    }

    fn consume_newline_or_whitespaces(&mut self) -> Self::Kind {
        if self.consume_newline() {
            self.current_flags
                .set(TokenFlags::PRECEDING_LINE_BREAK, true);
            NEWLINE
        } else {
            self.consume_whitespaces();
            WHITESPACE
        }
    }

    fn next_token(&mut self, _context: Self::LexContext) -> Self::Kind {
        self.current_start = self.text_position();
        self.current_flags = TokenFlags::empty();

        let kind = match self.current_byte() {
            Some(current) => self.consume_token(current),
            None => EOF,
        };

        self.current_kind = kind;

        if !kind.is_trivia() {
            self.current_flags
                .set(TokenFlags::PRECEDING_LINE_BREAK, false);
        }

        kind
    }

    #[inline]
    fn advance_char_unchecked(&mut self) {
        let c = self.current_char_unchecked();
        self.position += c.len_utf8();
    }

    /// Advances the current position by `n` bytes.
    #[inline]
    fn advance(&mut self, n: usize) {
        self.position += n;
    }

    fn finish(self) -> Vec<ParseDiagnostic> {
        self.diagnostics
    }

    fn rewind(&mut self, _checkpoint: LexerCheckpoint<Self::Kind>) {
        unimplemented!("GraphQL lexer doesn't support rewinding");
    }
}

impl<'src> LexerWithCheckpoint<'src> for GraphqlLexer<'src> {
    fn checkpoint(&self) -> LexerCheckpoint<Self::Kind> {
        LexerCheckpoint {
            position: TextSize::from(self.position as u32),
            current_start: self.current_start,
            current_flags: self.current_flags,
            current_kind: self.current_kind,
            after_line_break: self.has_preceding_line_break(),
            unicode_bom_length: self.unicode_bom_length,
            diagnostics_pos: self.diagnostics.len() as u32,
        }
    }
}

impl<'src> GraphqlLexer<'src> {
    /// Make a new lexer from a str, this is safe because strs are valid utf8
    pub fn from_str(source: &'src str) -> Self {
        Self {
            source,
            current_kind: TOMBSTONE,
            current_start: TextSize::from(0),
            current_flags: TokenFlags::empty(),
            position: 0,
            diagnostics: vec![],
            unicode_bom_length: 0,
        }
    }

    /// Bumps the current byte and creates a lexed token of the passed in kind
    fn consume_byte(&mut self, tok: GraphqlSyntaxKind) -> GraphqlSyntaxKind {
        self.advance(1);
        tok
    }

    /// Lexes the next token
    ///
    /// Guaranteed to not be at the end of the file
    fn consume_token(&mut self, current: u8) -> GraphqlSyntaxKind {
        // lookup_byte is optimized for
        match current {
            b'!' => self.consume_byte(T![!]),
            b'$' => self.consume_byte(T![$]),
            b'&' => self.consume_byte(T![&]),
            b'(' => self.consume_byte(T!['(']),
            b')' => self.consume_byte(T![')']),
            b'.' => self.consume_ellipsis(),
            b':' => self.consume_byte(T![:]),
            b'\n' | b'\r' | b'\t' | b' ' => self.consume_newline_or_whitespaces(),
            b',' => self.consume_commas(),
            b'"' => self.consume_string(),
            b'=' => self.consume_byte(T![=]),
            b'@' => self.consume_byte(T![@]),
            b'[' => self.consume_byte(T!['[']),
            b']' => self.consume_byte(T![']']),
            b'{' => self.consume_byte(T!['{']),
            b'|' => self.consume_byte(T![|]),
            b'}' => self.consume_byte(T!['}']),
            b'#' => self.consume_comment(),
            _ if is_name_start(current) => self.consume_name(current),
            _ if is_number_start(current) => self.consume_number(current),
            _ if self.position == 0 => {
                if let Some((bom, bom_size)) = self.consume_potential_bom(UNICODE_BOM) {
                    self.unicode_bom_length = bom_size;
                    return bom;
                }
                self.consume_unexpected_character()
            }
            _ => self.consume_unexpected_character(),
        }
    }

    /// Lexes an ellipsis.
    fn consume_ellipsis(&mut self) -> GraphqlSyntaxKind {
        self.assert_byte(b'.');
        let start = self.position;
        self.advance(1);
        if self.current_byte() == Some(b'.') {
            if self.byte_at(1) == Some(b'.') {
                self.advance(2);

                if self.current_byte() == Some(b'.') {
                    while self.current_byte() == Some(b'.') {
                        self.advance(1);
                    }

                    let end = self.position;
                    self.diagnostics.push(
                        ParseDiagnostic::new(
                            format!("'{}' isn't valid here.", ".".repeat(end - start)),
                            start..end,
                        )
                        .with_hint("Did you mean '...'?"),
                    );

                    ERROR_TOKEN
                } else {
                    DOT3
                }
            } else {
                self.advance(1);
                self.diagnostics.push(
                    ParseDiagnostic::new("'..' isn't valid here.", start..self.position)
                        .with_hint("Did you mean '...'?"),
                );

                ERROR_TOKEN
            }
        } else {
            self.diagnostics.push(
                ParseDiagnostic::new("'.' isn't valid here.", start..self.position)
                    .with_hint("Did you mean '...'?"),
            );

            ERROR_TOKEN
        }
    }

    /// Lexes a name, and keywords.
    fn consume_name(&mut self, first: u8) -> GraphqlSyntaxKind {
        self.assert_current_char_boundary();

        // Note to keep the buffer large enough to fit every possible keyword
        // that the lexer can return.
        const BUFFER_SIZE: usize = 32;
        let mut buffer = [0u8; BUFFER_SIZE];
        buffer[0] = first;
        let mut len = 1;

        self.advance_byte_or_char(first);

        while let Some(byte) = self.current_byte() {
            if is_name_continue(byte) {
                if len < BUFFER_SIZE {
                    buffer[len] = byte;
                    len += 1;
                }

                self.advance(1)
            } else {
                break;
            }
        }

        match &buffer[..len] {
            b"true" => TRUE_KW,
            b"false" => FALSE_KW,
            b"query" => QUERY_KW,
            b"mutation" => MUTATION_KW,
            b"subscription" => SUBSCRIPTION_KW,
            b"fragment" => FRAGMENT_KW,
            b"on" => ON_KW,
            b"null" => NULL_KW,
            b"schema" => SCHEMA_KW,
            b"extend" => EXTEND_KW,
            b"scalar" => SCALAR_KW,
            b"type" => TYPE_KW,
            b"implements" => IMPLEMENTS_KW,
            b"interface" => INTERFACE_KW,
            b"union" => UNION_KW,
            b"enum" => ENUM_KW,
            b"input" => INPUT_KW,
            b"directive" => DIRECTIVE_KW,
            b"repeatable" => REPEATABLE_KW,
            b"QUERY" => UPPER_QUERY_KW,
            b"MUTATION" => UPPER_MUTATION_KW,
            b"SUBSCRIPTION" => UPPER_SUBSCRIPTION_KW,
            b"FIELD" => UPPER_FIELD_KW,
            b"FRAGMENT_DEFINITION" => FRAGMENT_DEFINITION_KW,
            b"FRAGMENT_SPREAD" => FRAGMENT_SPREAD_KW,
            b"INLINE_FRAGMENT" => INLINE_FRAGMENT_KW,
            b"VARIABLE_DEFINITION" => VARIABLE_DEFINITION_KW,
            b"SCHEMA" => UPPER_SCHEMA_KW,
            b"SCALAR" => UPPER_SCALAR_KW,
            b"OBJECT" => UPPER_OBJECT_KW,
            b"FIELD_DEFINITION" => FIELD_DEFINITION_KW,
            b"ARGUMENT_DEFINITION" => ARGUMENT_DEFINITION_KW,
            b"INTERFACE" => UPPER_INTERFACE_KW,
            b"UNION" => UPPER_UNION_KW,
            b"ENUM" => UPPER_ENUM_KW,
            b"ENUM_VALUE" => ENUM_VALUE_KW,
            b"INPUT_OBJECT" => INPUT_OBJECT_KW,
            b"INPUT_FIELD_DEFINITION" => INPUT_FIELD_DEFINITION_KW,
            _ => T![ident],
        }
    }

    #[inline]
    fn consume_unexpected_character(&mut self) -> GraphqlSyntaxKind {
        self.assert_current_char_boundary();

        let char = self.current_char_unchecked();
        let err = ParseDiagnostic::new(
            format!("unexpected character `{char}`"),
            self.text_position()..self.text_position() + char.text_len(),
        );
        self.diagnostics.push(err);
        self.advance(char.len_utf8());

        ERROR_TOKEN
    }

    /// consume an entire number, be it a float, int, or scientific notion
    fn consume_number(&mut self, first: u8) -> GraphqlSyntaxKind {
        self.assert_current_char_boundary();

        let start = self.text_position();

        let mut state = match first {
            b'-' => LexNumberState::Minus,
            b'0' => LexNumberState::LeadingZero,
            _ => LexNumberState::IntegerPart,
        };
        self.advance(1);

        while let Some(chr) = self.current_byte() {
            let new_state = match chr {
                b'0'..=b'9' => self.consume_digit(chr, state),
                b'.' => self.consume_fraction(state),
                b'e' | b'E' => self.consume_exponent(chr, start, state),
                _ => break,
            };
            state = new_state;
        }

        match state {
            LexNumberState::LeadingZero | LexNumberState::IntegerPart => GRAPHQL_INT_LITERAL,
            LexNumberState::FractionalPart | LexNumberState::Exponent => GRAPHQL_FLOAT_LITERAL,
            LexNumberState::Minus => {
                self.diagnostics.push(ParseDiagnostic::new(
                    "Unexpected token `-`",
                    start..self.text_position(),
                ));

                ERROR_TOKEN
            }
            LexNumberState::Invalid(diagnostic) => {
                self.diagnostics.push(diagnostic);
                ERROR_TOKEN
            }
        }
    }

    /// consume a single digit in a number
    fn consume_digit(&mut self, chr: u8, state: LexNumberState) -> LexNumberState {
        debug_assert!(chr.is_ascii_digit());
        match chr {
            b'0' => {
                let position = self.text_position();
                self.advance(1);

                match state {
                    LexNumberState::LeadingZero => {
                        let diagnostic = ParseDiagnostic::new(
                            "GraphQL doesn't allow numbers starting with zero",
                            position..position + TextSize::from(1),
                        );
                        LexNumberState::Invalid(diagnostic)
                    }
                    LexNumberState::Minus => LexNumberState::LeadingZero,
                    _ => state,
                }
            }
            b'1'..=b'9' => {
                let position = self.text_position();
                self.advance(1);

                match state {
                    LexNumberState::LeadingZero => {
                        let diagnostic = ParseDiagnostic::new(
                            "GraphQL doesn't allow numbers starting with zero",
                            position..position + TextSize::from(1),
                        );
                        LexNumberState::Invalid(diagnostic)
                    }
                    LexNumberState::Minus => LexNumberState::IntegerPart,
                    _ => state,
                }
            }
            // should never happen
            _ => {
                let position = self.text_position();
                LexNumberState::Invalid(ParseDiagnostic::new(
                    "Invalid character",
                    position..position + TextSize::from(1),
                ))
            }
        }
    }
    fn consume_fraction(&mut self, state: LexNumberState) -> LexNumberState {
        self.assert_byte(b'.');
        let position = self.text_position();
        self.advance(1);

        if !self.current_byte().is_some_and(|b| b.is_ascii_digit()) {
            LexNumberState::Invalid(
                ParseDiagnostic::new("Missing fraction", position..position + TextSize::from(1))
                    .with_hint("Remove the `.`"),
            )
        } else {
            match state {
                LexNumberState::IntegerPart | LexNumberState::LeadingZero => {
                    LexNumberState::FractionalPart
                }
                invalid @ LexNumberState::Invalid(_) => invalid,
                _ => LexNumberState::Invalid(ParseDiagnostic::new(
                    "Invalid fraction part",
                    position..position + TextSize::from(1),
                )),
            }
        }
    }

    fn consume_exponent(
        &mut self,
        chr: u8,
        start: TextSize,
        state: LexNumberState,
    ) -> LexNumberState {
        debug_assert!(matches!(chr, b'e' | b'E'));
        let position = self.text_position();
        self.advance(1);

        if let Some(b'-' | b'+') = self.current_byte() {
            self.advance(1);
        }

        if !self.current_byte().is_some_and(|b| b.is_ascii_digit()) {
            LexNumberState::Invalid(
                ParseDiagnostic::new("Missing exponent", start..position).with_detail(
                    position..position + TextSize::from(1),
                    "Expected a digit as the exponent",
                ),
            )
        } else {
            match state {
                LexNumberState::LeadingZero
                | LexNumberState::IntegerPart
                | LexNumberState::FractionalPart => LexNumberState::Exponent,
                invalid @ LexNumberState::Invalid(_) => invalid,
                _ => LexNumberState::Invalid(ParseDiagnostic::new(
                    "Invalid exponent part",
                    position..position + TextSize::from(1),
                )),
            }
        }
    }

    fn consume_string(&mut self) -> GraphqlSyntaxKind {
        self.assert_byte(b'"');
        let start = self.text_position();

        self.advance(1); // Skip over the quote

        let mut state = LexStringState::Uninitialized;

        let mut has_error = false;

        while let Some(chr) = self.current_byte() {
            let (new_state, diagnostic) = self.consume_string_character(chr, start, state);
            state = new_state;
            if let Some(diagnostic) = diagnostic {
                self.push_diagnostic(diagnostic);
                has_error = true;
            }
            if matches!(state, LexStringState::Terminated) {
                break;
            }
        }
        match state {
            LexStringState::Terminated => {
                if has_error {
                    ERROR_TOKEN
                } else {
                    GRAPHQL_STRING_LITERAL
                }
            }
            LexStringState::InString
            | LexStringState::InBlockString
            | LexStringState::Uninitialized => {
                let unterminated =
                    ParseDiagnostic::new("Missing closing quote", start..self.text_position())
                        .with_detail(
                            self.source.text_len()..self.source.text_len(),
                            "file ends here",
                        );
                self.diagnostics.push(unterminated);

                ERROR_TOKEN
            }
        }
    }

    /// Lexes a character inside a string
    fn consume_string_character(
        &mut self,
        chr: u8,
        start: TextSize,
        state: LexStringState,
    ) -> (LexStringState, Option<ParseDiagnostic>) {
        self.assert_current_char_boundary();
        match state {
            LexStringState::Uninitialized => match chr {
                b'"' => self.consume_quote_in_string(state),
                _ => (LexStringState::InString, None),
            },
            LexStringState::InString => match chr {
                b'"' => self.consume_quote_in_string(state),
                b'\\' => self.consume_escape_sequence_in_string(state),
                b'\n' | b'\r' => (
                    LexStringState::Terminated,
                    Some(ParseDiagnostic::new(
                        "Missing closing quote",
                        start..self.text_position(),
                    )),
                ),
                _ => {
                    self.advance_char_unchecked();
                    (state, None)
                }
            },
            LexStringState::InBlockString => match chr {
                b'"' => self.consume_quote_in_string(state),
                b'\\' => self.consume_escape_sequence_in_string(state),
                _ => {
                    self.advance_char_unchecked();
                    (state, None)
                }
            },
            // should never happen
            _ => (
                state,
                Some(ParseDiagnostic::new(
                    "String terminated",
                    self.position..self.position + 1,
                )),
            ),
        }
    }

    fn consume_quote_in_string(
        &mut self,
        state: LexStringState,
    ) -> (LexStringState, Option<ParseDiagnostic>) {
        self.assert_byte(b'"');
        self.advance(1);
        match state {
            LexStringState::Uninitialized => {
                if self.current_byte() == Some(b'"') {
                    self.advance(1);
                    (LexStringState::InBlockString, None)
                } else {
                    // an empty string
                    (LexStringState::Terminated, None)
                }
            }
            LexStringState::InString => (LexStringState::Terminated, None),
            LexStringState::InBlockString => {
                if self.current_byte() == Some(b'"') && self.byte_at(1) == Some(b'"') {
                    self.advance(2);
                    (LexStringState::Terminated, None)
                } else {
                    (state, None)
                }
            }
            // should never happen
            _ => (
                state,
                Some(ParseDiagnostic::new(
                    "String terminated",
                    self.position..self.position + 1,
                )),
            ),
        }
    }

    fn consume_escape_sequence_in_string(
        &mut self,
        state: LexStringState,
    ) -> (LexStringState, Option<ParseDiagnostic>) {
        self.assert_byte(b'\\');
        let escape_start = self.text_position();
        self.advance(1);
        match state {
            // '\t' etc
            LexStringState::InString => match self.current_byte() {
                Some(b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't') => {
                    self.advance(1);
                    (state, None)
                }

                Some(b'u') => match self.consume_unicode_escape() {
                    Err(diagnostic) => (state, Some(diagnostic)),
                    Ok(_) => (state, None),
                },

                Some(_) => {
                    let c = self.current_char_unchecked();
                    let diagnostic = ParseDiagnostic::new(
                        "Invalid escape sequence",
                        escape_start..self.text_position() + c.text_len(),
                    )
                    .with_alternatives(
                        "Expected one of the following values",
                        &[
                            r"\\",
                            r"\/",
                            r#"\""#,
                            r"\b",
                            r"\f",
                            r"\n",
                            r"\r",
                            r"\t",
                            r"\uXXXX where X is hexedecimal number",
                        ],
                    );
                    (state, Some(diagnostic))
                }

                None => (
                    state,
                    Some(
                        ParseDiagnostic::new(
                            "Expected an escape sequence following a backslash, but found none",
                            escape_start..self.text_position(),
                        )
                        .with_detail(self.text_position()..self.text_position(), "File ends here"),
                    ),
                ),
            },
            // '\"""'
            LexStringState::InBlockString => {
                if self.current_byte() == Some(b'"')
                    && self.byte_at(1) == Some(b'"')
                    && self.byte_at(2) == Some(b'"')
                {
                    self.advance(3);
                }
                (state, None)
            }
            // should never happen
            _ => (
                state,
                Some(ParseDiagnostic::new(
                    "String terminated",
                    self.position..self.position + 1,
                )),
            ),
        }
    }

    /// Lexes a `\u0000` escape sequence. Assumes that the lexer is positioned at the `u` token.
    ///
    /// A unicode escape sequence must consist of 4 hex characters.
    fn consume_unicode_escape(&mut self) -> Result<(), ParseDiagnostic> {
        self.assert_byte(b'u');
        self.assert_current_char_boundary();

        let start = self.text_position();

        let start = start
            // Subtract 1 to get position of `\`
            .checked_sub(TextSize::from(1))
            .unwrap_or(start);

        self.advance(1); // Advance over `u`

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

    fn consume_comment(&mut self) -> GraphqlSyntaxKind {
        self.assert_byte(b'#');

        self.advance(1);

        while let Some(chr) = self.current_byte() {
            match chr {
                b'\n' | b'\r' => return COMMENT,
                chr => self.advance_byte_or_char(chr),
            }
        }
        COMMENT
    }

    fn consume_commas(&mut self) -> GraphqlSyntaxKind {
        while self.current_byte() == Some(b',') {
            self.assert_byte(b',');
            self.advance(1);
        }

        COMMA
    }
}

fn is_name_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_'
}

fn is_name_continue(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn is_number_start(byte: u8) -> bool {
    byte.is_ascii_digit() || byte == b'-'
}

#[derive(Copy, Clone, Debug)]
enum LexStringState {
    Uninitialized,

    /// Between the opening `"` and closing `"` quotes.
    InString,

    /// Between the opening `"""` and closing `"""` quotes.
    InBlockString,

    /// Properly terminated string
    Terminated,
}

/// Current state of a number being parsed
#[derive(Debug, Clone)]
enum LexNumberState {
    /// After a minus sign
    Minus,

    /// After the first digit which is also Zero
    LeadingZero,

    /// Parsing the digits before the exponent or fractional (after .`) part
    IntegerPart,

    /// Parsing the digits after a `.`
    FractionalPart,

    /// Parsing the exponent digits (after a `e` or `E`)
    Exponent,

    /// Parsing the rest of an invalid number
    Invalid(ParseDiagnostic),
}
