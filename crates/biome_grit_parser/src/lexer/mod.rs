//! An extremely fast, lookup table based, GritQL lexer which yields SyntaxKind tokens used by the biome Grit parser.

#[rustfmt::skip]
mod tests;

use crate::constants::SUPPORTED_LANGUAGE_SET_STR;
use biome_grit_syntax::{GritSyntaxKind, GritSyntaxKind::*, TextLen, TextRange, TextSize, T};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{Lexer, LexerCheckpoint};
use biome_rowan::SyntaxKind;
use std::ops::Add;

/// An extremely fast, lookup table based, lossless Grit lexer
#[derive(Debug)]
pub(crate) struct GritLexer<'src> {
    /// Source text
    source: &'src str,

    /// The start byte position in the source text of the next token.
    position: usize,

    /// Byte offset of the current token from the start of the source
    /// The range of the current token can be computed by
    /// `self.position - self.current_start`.
    current_start: TextSize,

    /// The kind of the current token
    current_kind: GritSyntaxKind,

    /// `true` if there has been a line break between the last non-trivia token
    /// and the next non-trivia token.
    after_newline: bool,

    /// `true` if the last thing we parsed was a `js` keyword
    /// This is used to flip into the js lexer when we see a `js` keyword
    in_js: bool,

    diagnostics: Vec<ParseDiagnostic>,
}

impl<'src> Lexer<'src> for GritLexer<'src> {
    const NEWLINE: Self::Kind = NEWLINE;
    const WHITESPACE: Self::Kind = WHITESPACE;
    type Kind = GritSyntaxKind;

    type LexContext = ();
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

    fn next_token(&mut self, _context: Self::LexContext) -> Self::Kind {
        self.current_start = self.text_position();

        let kind = match self.current_byte() {
            Some(current) => {
                let kind = self.lex_token(current);

                debug_assert!(
                    self.current_start < self.text_position(),
                    "Lexer did not progress"
                );
                kind
            }
            None => EOF,
        };

        self.current_kind = kind;

        if !kind.is_trivia() {
            self.after_newline = false;
            self.in_js = matches!(kind, GritSyntaxKind::JS_KW);
        }

        kind
    }
    fn has_preceding_line_break(&self) -> bool {
        self.after_newline
    }

    fn has_unicode_escape(&self) -> bool {
        false // Grit identifiers do not support Unicode escapes
    }

    fn rewind(&mut self, _checkpoint: LexerCheckpoint<Self::Kind>) {
        unimplemented!("Grit lexer doesn't support rewinding");
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

impl<'src> GritLexer<'src> {
    /// Make a new lexer from a str, this is safe because strs are valid utf8
    pub fn from_str(string: &'src str) -> Self {
        Self {
            source: string,
            position: 0,
            current_kind: TOMBSTONE,
            current_start: TextSize::from(0),
            after_newline: false,
            diagnostics: Vec::new(),
            in_js: false,
        }
    }

    /// Bumps the current byte and creates a lexed token of the passed in kind.
    #[inline]
    fn consume_byte(&mut self, tok: GritSyntaxKind) -> GritSyntaxKind {
        self.advance(1);
        tok
    }

    /// Eats all whitespace until a non-whitespace or a newline is found.
    ///
    /// ## Safety
    /// Must be called at a whitespace character.
    fn consume_whitespaces(&mut self) -> GritSyntaxKind {
        self.assert_current_char_boundary();

        while let Some(byte) = self.current_byte() {
            match byte {
                b'\t' | b' ' => self.advance(1),
                b'\r' | b'\n' => {
                    break;
                }
                _ => match byte {
                    b' ' | b'\t' | b'\n' | b'\r' => {
                        let start = self.text_position();
                        self.advance(1);

                        self.diagnostics.push(
                            ParseDiagnostic::new(
                                "GritQL only allows tabs, whitespace, carriage return and line feed whitespace.",
                                start..self.text_position(),
                            )
                            .with_hint("Use a regular whitespace character instead."),
                        )
                    }
                    _ => break,
                },
            }
        }

        WHITESPACE
    }

    /// Lexes the next token
    ///
    /// Guaranteed to not be at the end of the file
    fn lex_token(&mut self, current: u8) -> GritSyntaxKind {
        match current {
            b'\n' | b'\r' => {
                self.consume_newline();
                NEWLINE
            }
            b'\t' | b' ' => self.consume_whitespaces(),
            b'\'' | b'"' => self.consume_string_literal(current),
            b'`' => self.consume_backtick_snippet(),
            b'/' => self.consume_comment_or_slash(),
            b'=' => self.consume_equals_or_rewrite(),
            b'<' => self.consume_lt_or_match(),
            b'>' => self.consume_gt(),
            b':' => self.consume_byte(T![:]),
            b';' => self.consume_byte(T![;]),
            b'.' => self.consume_dots(),
            b',' => self.consume_byte(T![,]),
            b'+' => self.consume_plus_or_acc(),
            b'*' => self.consume_byte(T![*]),
            b'%' => self.consume_byte(T![%]),
            b'[' => self.consume_byte(T!['[']),
            b']' => self.consume_byte(T![']']),
            b'{' => {
                if self.in_js {
                    self.consume_js_body()
                } else {
                    self.consume_byte(T!['{'])
                }
            }
            b'}' => self.consume_byte(T!['}']),
            b'(' => self.consume_byte(T!['(']),
            b')' => self.consume_byte(T![')']),
            b'$' => self.consume_variable(),
            b'!' => self.consume_byte(T![!]),
            _ if is_leading_identifier_byte(current) => self.consume_name(current),
            _ if current.is_ascii_digit() || current == b'-' => self.consume_number(current),
            _ if self.position == 0 && self.consume_potential_bom(UNICODE_BOM).is_some() => {
                UNICODE_BOM
            }
            _ => self.consume_unexpected_character(),
        }
    }

    fn consume_dots(&mut self) -> GritSyntaxKind {
        assert_eq!(self.current_byte(), Some(b'.'));
        let start = self.position;
        self.advance(1);

        match self.current_byte() {
            Some(b'.') => {
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
            }
            _ => T![.],
        }
    }

    fn consume_equals_or_rewrite(&mut self) -> GritSyntaxKind {
        assert_eq!(self.current_byte(), Some(b'='));
        self.advance(1);

        match self.current_byte() {
            Some(b'=') => {
                self.advance(1);
                T![==]
            }
            Some(b'>') => {
                self.advance(1);
                T![=>]
            }
            _ => T![=],
        }
    }

    fn consume_lt_or_match(&mut self) -> GritSyntaxKind {
        assert_eq!(self.current_byte(), Some(b'<'));
        self.advance(1);

        match self.current_byte() {
            Some(b':') => {
                self.advance(1);
                T![<:]
            }
            Some(b'=') => {
                self.advance(1);
                T![<=]
            }
            _ => T![<],
        }
    }

    fn consume_gt(&mut self) -> GritSyntaxKind {
        assert_eq!(self.current_byte(), Some(b'>'));
        self.advance(1);

        match self.current_byte() {
            Some(b'=') => {
                self.advance(1);
                T![>=]
            }
            _ => T![>],
        }
    }

    fn consume_plus_or_acc(&mut self) -> GritSyntaxKind {
        assert_eq!(self.current_byte(), Some(b'+'));
        self.advance(1);

        match self.current_byte() {
            Some(b'=') => {
                self.advance(1);
                T![+=]
            }
            _ => T![+],
        }
    }

    #[inline]
    fn consume_unexpected_character(&mut self) -> GritSyntaxKind {
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

    /// Lexes a Grit number literal
    fn consume_number(&mut self, first: u8) -> GritSyntaxKind {
        self.assert_current_char_boundary();

        let start = self.text_position();

        let is_negative = first == b'-';
        if is_negative {
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
                            if self.current_byte().is_some_and(|b| b.is_ascii_digit()) =>
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
                Some(b'1'..=b'9') => {
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
                            if self.current_byte().is_some_and(|b| b.is_ascii_digit()) =>
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
                            if self.current_byte().is_some_and(|b| b.is_ascii_digit()) =>
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
            LexNumberState::IntegerPart if is_negative => GRIT_NEGATIVE_INT,
            LexNumberState::IntegerPart => GRIT_INT,
            LexNumberState::FractionalPart | LexNumberState::Exponent => GRIT_DOUBLE,
            LexNumberState::FirstDigit => MINUS,
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
                        "GritQL doesn't allow octal number notation (numbers starting with zero)",
                        position..position + TextSize::from(1),
                    ),
                    InvalidNumberReason::MissingExponent => {
                        ParseDiagnostic::new("Missing exponent", start..position).with_detail(
                            position..position + TextSize::from(1),
                            "Expected a digit as the exponent",
                        )
                    }
                    InvalidNumberReason::MissingFraction => ParseDiagnostic::new(
                        "Missing fraction",
                        position..position + TextSize::from(1),
                    )
                    .with_hint("Remove the `.`"),
                };

                self.diagnostics.push(diagnostic);
                ERROR_TOKEN
            }
        }
    }

    fn consume_string_literal(&mut self, quote: u8) -> GritSyntaxKind {
        self.assert_current_char_boundary();
        let start = self.text_position();

        self.advance(1); // Skip over the quote

        let mut state = match quote {
            // Handle invalid quotes
            b'\'' => LexStringState::InvalidQuote,
            _ => LexStringState::InString,
        };

        while let Some(chr) = self.current_byte() {
            match chr {
                b'\'' | b'"' if quote == chr => {
                    self.advance(1);
                    state = match state {
                        LexStringState::InString => LexStringState::Terminated,
                        state => state,
                    };
                    break;
                }
                // '\t' etc
                b'\\' => {
                    let escape_start = self.text_position();
                    self.advance(1);

                    match self.current_byte() {
                        Some(b'$' | b'\\' | b'"' | b'n') => self.advance(1),

                        Some(b'u') => match (self.consume_unicode_escape(), state) {
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
                                    .with_hint(
                                        r#"Valid escape sequences are: `\$`, `\\`, `\"`, `\n`."#,
                                    ),
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
                b'\n' | b'\r' => {
                    let unterminated =
                        ParseDiagnostic::new("Missing closing quote", start..self.text_position())
                            .with_hint("The closing quote must be on the same line.");

                    self.diagnostics.push(unterminated);

                    return ERROR_TOKEN;
                }
                _ => self.advance_char_unchecked(),
            }
        }

        match state {
            LexStringState::Terminated => GRIT_STRING,
            LexStringState::InvalidQuote => {
                let literal_range = TextRange::new(start, self.text_position());
                self.diagnostics.push(
                    ParseDiagnostic::new(
                        "GritQL does not allow single quoted strings",
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

    fn consume_js_body(&mut self) -> GritSyntaxKind {
        self.assert_current_char_boundary();
        let start = self.text_position();

        self.advance(1); // Skip over the opening brace
        let mut state = LexJavaScriptBody::InBody;
        let mut brace_depth = 0;

        while let Some(chr) = self.current_byte() {
            match chr {
                b'}' => {
                    self.advance(1);
                    if matches!(state, LexJavaScriptBody::InBody) {
                        if brace_depth == 0 {
                            state = LexJavaScriptBody::Terminated;
                            break;
                        } else {
                            brace_depth -= 1;
                        }
                    }
                }
                b'{' => {
                    self.advance(1);
                    if matches!(state, LexJavaScriptBody::InBody) {
                        brace_depth += 1;
                    }
                }
                b'/' => {
                    // Consume the comment without using it.
                    let _comment = self.consume_comment_or_slash();
                }
                _ => self.advance_char_unchecked(),
            }
        }

        match state {
            LexJavaScriptBody::Terminated => GRIT_JAVASCRIPT_BODY,
            LexJavaScriptBody::InBody => {
                let unterminated =
                    ParseDiagnostic::new("Missing closing brace", start..self.text_position())
                        .with_detail(
                            self.source.text_len()..self.source.text_len(),
                            "file ends here",
                        );
                self.diagnostics.push(unterminated);

                ERROR_TOKEN
            }
        }
    }

    fn consume_backtick_snippet(&mut self) -> GritSyntaxKind {
        self.assert_current_char_boundary();
        let start = self.text_position();

        self.advance(1); // Skip over the backtick
        let mut state = LexBacktickSnippet::InSnippet;

        while let Some(chr) = self.current_byte() {
            match chr {
                b'`' => {
                    self.advance(1);
                    state = match state {
                        LexBacktickSnippet::InSnippet => LexBacktickSnippet::Terminated,
                        state => state,
                    };
                    break;
                }
                // '\t' etc
                b'\\' => {
                    let escape_start = self.text_position();
                    self.advance(1);

                    match self.current_byte() {
                        Some(b'$' | b'\\' | b'`' | b'n') => self.advance(1),

                        Some(_) => {
                            if matches!(state, LexBacktickSnippet::InSnippet) {
                                let c = self.current_char_unchecked();
                                self.diagnostics.push(
                                    ParseDiagnostic::new(
                                        "Invalid escape sequence",
                                        escape_start..self.text_position() + c.text_len(),
                                    )
                                    .with_hint(
                                        r#"Valid escape sequences are: `\$`, `\\`, `\``, `\n`."#,
                                    ),
                                );
                                state = LexBacktickSnippet::InvalidEscapeSequence;
                            }
                        }

                        None => {
                            if matches!(state, LexBacktickSnippet::InSnippet) {
                                self.diagnostics.push(ParseDiagnostic::new(

                                    "Expected an escape sequence following a backslash, but found none",
                                    escape_start..self.text_position(),
                                )
                                    .with_detail(self.text_position()..self.text_position(), "File ends here")
                                );
                                state = LexBacktickSnippet::InvalidEscapeSequence;
                            }
                        }
                    }
                }
                _ => self.advance_char_unchecked(),
            }
        }

        match state {
            LexBacktickSnippet::Terminated => GRIT_BACKTICK_SNIPPET,
            LexBacktickSnippet::InSnippet => {
                let unterminated =
                    ParseDiagnostic::new("Missing closing quote", start..self.text_position())
                        .with_detail(
                            self.source.text_len()..self.source.text_len(),
                            "file ends here",
                        );
                self.diagnostics.push(unterminated);

                ERROR_TOKEN
            }
            LexBacktickSnippet::InvalidEscapeSequence => ERROR_TOKEN,
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

    /// Lexes a name for variables (without leading `$`), labels, and keywords.
    ///
    /// Branches into regular expressions and raw backticks, which have named
    /// prefixes.
    fn consume_name(&mut self, first: u8) -> GritSyntaxKind {
        let name_start = self.text_position();
        self.assert_current_char_boundary();

        // Note to keep the buffer large enough to fit every possible keyword
        // that the lexer can return.
        const BUFFER_SIZE: usize = 14;
        let mut buffer = [0u8; BUFFER_SIZE];
        buffer[0] = first;
        let mut len = 1;

        self.advance_byte_or_char(first);

        while let Some(byte) = self.current_byte() {
            if is_identifier_byte(byte) {
                if len < BUFFER_SIZE {
                    buffer[len] = byte;
                    len += 1;
                }

                self.advance(1)
            } else if byte == b'"' {
                return match &buffer[..len] {
                    b"r" => self.consume_regex(),
                    b"js" => T![js],
                    b"json" => T![json],
                    b"css" => T![css],
                    b"grit" => T![grit],
                    b"html" => T![html],
                    _ => {
                        self.diagnostics.push(
                            ParseDiagnostic::new(
                                "Unxpected string prefix",
                                name_start..self.text_position(),
                            )
                            .with_hint("Use a language annotation to create a language-specific snippet or use the `r` prefix to create a regex literal.")
                            .with_alternatives("Supported language annotations are:", SUPPORTED_LANGUAGE_SET_STR),
                        );

                        ERROR_TOKEN
                    }
                };
            } else if byte == b'`' {
                return match &buffer[..len] {
                    b"r" => match self.consume_backtick_snippet() {
                        GRIT_BACKTICK_SNIPPET => GRIT_SNIPPET_REGEX,
                        other => other,
                    },
                    b"raw" => match self.consume_backtick_snippet() {
                        GRIT_BACKTICK_SNIPPET => GRIT_RAW_BACKTICK_SNIPPET,
                        other => other,
                    },
                    _ => {
                        self.diagnostics.push(
                            ParseDiagnostic::new(
                                "Unxpected snippet prefix",
                                name_start..self.text_position(),
                            )
                            .with_hint("Supported snippet prefixes are `r` and `raw`."),
                        );

                        ERROR_TOKEN
                    }
                };
            } else {
                break;
            }
        }

        match &buffer[..len] {
            b"sequential" => SEQUENTIAL_KW,
            b"multifile" => MULTIFILE_KW,
            b"engine" => ENGINE_KW,
            b"biome" => BIOME_KW,
            b"marzano" => MARZANO_KW,
            b"language" => LANGUAGE_KW,
            b"js" => JS_KW,
            b"css" => CSS_KW,
            b"json" => JSON_KW,
            b"grit" => GRIT_KW,
            b"html" => HTML_KW,
            b"typescript" => TYPESCRIPT_KW,
            b"jsx" => JSX_KW,
            b"as" => AS_KW,
            b"limit" => LIMIT_KW,
            b"where" => WHERE_KW,
            b"orelse" => ORELSE_KW,
            b"maybe" => MAYBE_KW,
            b"after" => AFTER_KW,
            b"before" => BEFORE_KW,
            b"contains" => CONTAINS_KW,
            b"until" => UNTIL_KW,
            b"includes" => INCLUDES_KW,
            b"if" => IF_KW,
            b"else" => ELSE_KW,
            b"within" => WITHIN_KW,
            b"bubble" => BUBBLE_KW,
            b"not" => NOT_KW,
            b"or" => OR_KW,
            b"and" => AND_KW,
            b"any" => ANY_KW,
            b"some" => SOME_KW,
            b"every" => EVERY_KW,
            b"private" => PRIVATE_KW,
            b"pattern" => PATTERN_KW,
            b"predicate" => PREDICATE_KW,
            b"function" => {
                // Function is also an AST node name, so we need to check if the next token is a `(`
                if self.current_byte() == Some(b'(') {
                    GRIT_NAME
                } else {
                    FUNCTION_KW
                }
            }
            b"true" => TRUE_KW,
            b"false" => FALSE_KW,
            b"undefined" => UNDEFINED_KW,
            b"like" => LIKE_KW,
            b"return" => RETURN_KW,
            _ => GRIT_NAME,
        }
    }

    /// Lexes a regular expression.
    fn consume_regex(&mut self) -> GritSyntaxKind {
        // Handle invalid quotes
        self.assert_current_char_boundary();
        let start = self.text_position();

        self.advance(1); // Skip over the quote
        let mut state = LexRegexState::InRegex;

        while let Some(chr) = self.current_byte() {
            match chr {
                b'"' => {
                    self.advance(1);
                    state = match state {
                        LexRegexState::InRegex => LexRegexState::Terminated,
                        state => state,
                    };
                    break;
                }
                // '\t' etc
                b'\\' => {
                    let escape_start = self.text_position();
                    self.advance(1);

                    match self.current_byte() {
                        Some(_) => self.advance_char_unchecked(),

                        None => {
                            if matches!(state, LexRegexState::InRegex) {
                                self.diagnostics.push(ParseDiagnostic::new(

                                    "Expected an escape sequence following a backslash, but found none",
                                    escape_start..self.text_position(),
                                )
                                    .with_detail(self.text_position()..self.text_position(), "File ends here")
                                );
                                return ERROR_TOKEN;
                            }
                        }
                    }
                }
                b'\n' | b'\r' => {
                    let unterminated =
                        ParseDiagnostic::new("Missing closing quote", start..self.text_position())
                            .with_hint("The closing quote must be on the same line.");

                    self.diagnostics.push(unterminated);

                    return ERROR_TOKEN;
                }
                _ => self.advance_char_unchecked(),
            }
        }

        match state {
            LexRegexState::Terminated => GRIT_REGEX,
            LexRegexState::InRegex => {
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

    /// Lexes a variable (with leading `$`).
    fn consume_variable(&mut self) -> GritSyntaxKind {
        assert_eq!(self.current_byte(), Some(b'$'));
        self.advance(1); // Skip the leading `$`.

        if self.current_byte() == Some(b'_') {
            self.advance(1);
            return DOLLAR_UNDERSCORE;
        }

        while let Some(byte) = self.current_byte() {
            if is_identifier_byte(byte) {
                self.advance(1)
            } else {
                break;
            }
        }

        GRIT_VARIABLE
    }

    /// Lexes a comment or a plain slash token.
    fn consume_comment_or_slash(&mut self) -> GritSyntaxKind {
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

                COMMENT
            }
            _ => {
                self.advance(1);
                T![/]
            }
        }
    }
}

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
    /// When using `'` instead of `"`.
    InvalidQuote,

    /// String that contains an invalid escape sequence.
    InvalidEscapeSequence,

    /// Between the opening `"` and closing `"` quotes.
    InString,

    /// Properly terminated string.
    Terminated,
}

#[derive(Copy, Clone, Debug)]
enum LexBacktickSnippet {
    /// Between the opening and closing backticks.
    InSnippet,

    /// Snippet that contains an invalid escape sequence.
    InvalidEscapeSequence,

    /// Properly terminated snippet.
    Terminated,
}

#[derive(Copy, Clone, Debug)]
enum LexJavaScriptBody {
    /// Inside a js body
    InBody,

    /// Properly terminated body
    Terminated,
}

#[derive(Copy, Clone, Debug)]
enum LexRegexState {
    /// Between the opening and closing quotes.
    InRegex,

    /// Properly terminated regex.
    Terminated,
}

fn is_identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn is_leading_identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_'
}
