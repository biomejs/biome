//! Glimmer template lexer
//!
//! This lexer tokenizes Glimmer templates, including:
//! - Text content
//! - Mustache expressions {{...}}
//! - HTML tags and attributes
//! - Glimmer-specific syntax (@args, #blocks, etc.)

use biome_glimmer_syntax::{
    GlimmerSyntaxKind, GlimmerSyntaxKind::*, T, TextLen, TextRange, TextSize,
};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{LexContext, Lexer as LexerTrait};

use crate::token_source::GlimmerLexContext;

/// The Glimmer lexer
pub(crate) struct GlimmerLexer<'src> {
    /// Source text
    source: &'src str,

    /// The start byte position in the source text of the next token.
    position: usize,

    /// Accumulated diagnostics
    diagnostics: Vec<ParseDiagnostic>,

    /// Whether we just parsed a line break
    after_newline: bool,
}

impl<'src> GlimmerLexer<'src> {
    /// Make a new lexer from a str
    pub fn from_str(source: &'src str) -> Self {
        Self {
            source,
            position: 0,
            diagnostics: vec![],
            after_newline: false,
        }
    }

    fn current_byte(&self) -> Option<u8> {
        if self.position < self.source.len() {
            Some(self.source.as_bytes()[self.position])
        } else {
            None
        }
    }

    fn peek_byte(&self) -> Option<u8> {
        if self.position + 1 < self.source.len() {
            Some(self.source.as_bytes()[self.position + 1])
        } else {
            None
        }
    }

    fn advance(&mut self, n: usize) {
        self.position = self.position.saturating_add(n).min(self.source.len());
    }

    fn text_position(&self) -> TextSize {
        TextSize::try_from(self.position).expect("Input to be smaller than 4 GB")
    }

    fn consume_byte(&mut self, kind: GlimmerSyntaxKind) -> GlimmerSyntaxKind {
        self.advance(1);
        kind
    }

    /// Lex a token in the given context
    fn lex_token(&mut self, context: GlimmerLexContext) -> GlimmerSyntaxKind {
        match context {
            GlimmerLexContext::Regular => self.lex_regular(),
            GlimmerLexContext::InsideMustache => self.lex_inside_mustache(),
            GlimmerLexContext::InsideTag => self.lex_inside_tag(),
            GlimmerLexContext::AttributeValue => self.lex_attribute_value(),
        }
    }

    /// Lex tokens in regular template context (text content)
    fn lex_regular(&mut self) -> GlimmerSyntaxKind {
        match self.current_byte() {
            Some(b'{') if self.peek_byte() == Some(b'{') => {
                self.advance(2);
                L_CURLY2
            }
            Some(b'<') => self.consume_byte(T![<]),
            Some(b'\n') => {
                self.after_newline = true;
                self.consume_byte(NEWLINE)
            }
            Some(b'\r') => {
                if self.peek_byte() == Some(b'\n') {
                    self.advance(2);
                } else {
                    self.advance(1);
                }
                self.after_newline = true;
                NEWLINE
            }
            Some(b' ') | Some(b'\t') => {
                self.advance(1);
                while matches!(self.current_byte(), Some(b' ' | b'\t')) {
                    self.advance(1);
                }
                WHITESPACE
            }
            Some(_) => {
                // Consume text until we hit a special character
                let start = self.position;
                while let Some(byte) = self.current_byte() {
                    match byte {
                        b'{' | b'<' | b'\n' | b'\r' => break,
                        _ => self.advance(1),
                    }
                }
                TEXT
            }
            None => EOF,
        }
    }

    /// Lex tokens inside mustache expressions
    fn lex_inside_mustache(&mut self) -> GlimmerSyntaxKind {
        match self.current_byte() {
            Some(b'}') if self.peek_byte() == Some(b'}') => {
                self.advance(2);
                R_CURLY2
            }
            Some(b'(') => self.consume_byte(T!['(']),
            Some(b')') => self.consume_byte(T![')']),
            Some(b'.') => self.consume_byte(T![.]),
            Some(b'=') => self.consume_byte(T![=]),
            Some(b'#') => self.consume_byte(T![#]),
            Some(b'@') => self.consume_byte(T![@]),
            Some(b'|') => self.consume_byte(T![|]),
            Some(b'"') => self.lex_string_literal(),
            Some(b'0'..=b'9') => self.lex_number_literal(),
            Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r') => self.lex_whitespace(),
            Some(_) => self.lex_identifier_or_keyword(),
            None => EOF,
        }
    }

    /// Lex tokens inside HTML/component tags
    fn lex_inside_tag(&mut self) -> GlimmerSyntaxKind {
        match self.current_byte() {
            Some(b'>') => self.consume_byte(T![>]),
            Some(b'/') => self.consume_byte(T![/]),
            Some(b'=') => self.consume_byte(T![=]),
            Some(b'"') => self.lex_string_literal(),
            Some(b'{') if self.peek_byte() == Some(b'{') => {
                self.advance(2);
                L_CURLY2
            }
            Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r') => self.lex_whitespace(),
            Some(_) => self.lex_identifier_or_keyword(),
            None => EOF,
        }
    }

    /// Lex attribute values
    fn lex_attribute_value(&mut self) -> GlimmerSyntaxKind {
        // For now, treat similar to inside tags
        self.lex_inside_tag()
    }

    fn lex_whitespace(&mut self) -> GlimmerSyntaxKind {
        match self.current_byte() {
            Some(b'\n') => {
                self.after_newline = true;
                self.consume_byte(NEWLINE)
            }
            Some(b'\r') => {
                if self.peek_byte() == Some(b'\n') {
                    self.advance(2);
                } else {
                    self.advance(1);
                }
                self.after_newline = true;
                NEWLINE
            }
            Some(b' ') | Some(b'\t') => {
                self.advance(1);
                while matches!(self.current_byte(), Some(b' ' | b'\t')) {
                    self.advance(1);
                }
                WHITESPACE
            }
            _ => WHITESPACE,
        }
    }

    fn lex_identifier_or_keyword(&mut self) -> GlimmerSyntaxKind {
        let start = self.position;

        // Simple identifier lexing - consume alphanumeric and underscores
        while let Some(byte) = self.current_byte() {
            match byte {
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'-' => {
                    self.advance(1);
                }
                _ => break,
            }
        }

        let text = &self.source[start..self.position];

        // Check for keywords
        match text {
            "this" => THIS_KW,
            "as" => AS_KW,
            "if" => IF_KW,
            "else" => ELSE_KW,
            "each" => EACH_KW,
            "let" => LET_KW,
            "yield" => YIELD_KW,
            "true" => TRUE_KW,
            "false" => FALSE_KW,
            "null" => NULL_KW,
            "undefined" => UNDEFINED_KW,
            _ => IDENT,
        }
    }

    fn lex_string_literal(&mut self) -> GlimmerSyntaxKind {
        // Consume opening quote
        self.advance(1);

        // Consume string content until closing quote or EOF
        while let Some(byte) = self.current_byte() {
            match byte {
                b'"' => {
                    self.advance(1);
                    return STRING_LITERAL;
                }
                b'\\' => {
                    // Skip escaped character
                    self.advance(2);
                }
                _ => {
                    self.advance(1);
                }
            }
        }

        // Unterminated string
        STRING_LITERAL
    }

    fn lex_number_literal(&mut self) -> GlimmerSyntaxKind {
        while let Some(byte) = self.current_byte() {
            match byte {
                b'0'..=b'9' | b'.' | b'e' | b'E' | b'+' | b'-' => {
                    self.advance(1);
                }
                _ => break,
            }
        }
        NUMBER_LITERAL
    }
}

impl<'src> LexerTrait for GlimmerLexer<'src> {
    type Kind = GlimmerSyntaxKind;
    type LexContext = GlimmerLexContext;
    type ReLexContext = GlimmerLexContext;

    fn source(&self) -> &str {
        self.source
    }

    fn current(&self) -> Self::Kind {
        // This will be called after next_token
        // For now, return a placeholder
        EOF
    }

    fn next_token(&mut self, context: Self::LexContext) -> Self::Kind {
        self.after_newline = false;

        if self.position >= self.source.len() {
            return EOF;
        }

        self.lex_token(context)
    }

    fn has_preceding_line_break(&self) -> bool {
        self.after_newline
    }

    fn has_unicode_escape(&self) -> bool {
        false
    }

    fn rewind(&mut self, _checkpoint: biome_parser::lexer::LexerCheckpoint<Self::Kind>) {
        // TODO: Implement rewinding if needed
    }

    fn finish(self) -> Vec<ParseDiagnostic> {
        self.diagnostics
    }

    fn current_range(&self) -> TextRange {
        TextRange::empty(self.text_position())
    }

    fn checkpoint(&self) -> biome_parser::lexer::LexerCheckpoint<Self::Kind> {
        todo!()
    }

    fn re_lex(&mut self, _context: Self::ReLexContext) -> Self::Kind {
        self.current()
    }

    fn position(&self) -> usize {
        self.position
    }
}
