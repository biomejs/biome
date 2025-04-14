use biome_parser::{
    diagnostic::ParseDiagnostic,
    lexer::{Lexer, TokenFlags},
};
use biome_rowan::{TextRange, TextSize};
use biome_yaml_syntax::{T, YamlSyntaxKind, YamlSyntaxKind::*};

#[rustfmt::skip]
mod tests;

pub(crate) struct YamlLexer<'src> {
    /// Source text
    source: &'src str,

    /// The start byte position in the source text of the next token.
    position: usize,

    /// If the source starts with a Unicode BOM, this is the number of bytes for that token.
    #[expect(unused)]
    unicode_bom_length: usize,

    /// Byte offset of the current token from the start of the source
    /// The range of the current token can be computed by `self.position - self.current_start`
    current_start: TextSize,

    /// The kind of the current token
    current_kind: YamlSyntaxKind,

    /// Flags for the current token
    current_flags: TokenFlags,

    /// diagnostics emitted during the parsing phase
    diagnostics: Vec<ParseDiagnostic>,
}

impl YamlLexer<'_> {
    /// Consume a byte in the given context
    fn consume_token(&mut self, current: u8) -> YamlSyntaxKind {
        let start = self.text_position();

        let kind = match current {
            b'#' => self.consume_comment(),
            b'-' => self.consume_byte(T![-]),
            b':' => self.consume_byte(T![:]),
            b'[' => self.consume_byte(T!['[']),
            b',' => self.consume_byte(T![,]),
            b']' => self.consume_byte(T![']']),
            b'\'' => self.consume_single_quoted_literal(),
            b'"' => self.consume_double_quoted_literal(),
            b' ' | b'\n' => self.consume_newline_or_whitespaces(),
            _ => self.consume_plain_literal(),
        };

        debug_assert!(self.text_position() > start, "Lexer did not advance");
        kind
    }

    /// Bumps the current byte and creates a lexed token of the passed in kind.
    #[inline]
    fn consume_byte(&mut self, tok: YamlSyntaxKind) -> YamlSyntaxKind {
        self.advance(1);
        tok
    }

    fn consume_comment(&mut self) -> YamlSyntaxKind {
        self.assert_byte(b'#');
        while let Some(c) = self.current_byte() {
            if c == b'\n' {
                break;
            }
            self.advance(1);
        }
        COMMENT
    }

    // https://yaml.org/spec/1.2.2/#rule-ns-plain
    // TODO: parse multiline plain scalar at current indentation level
    fn consume_plain_literal(&mut self) -> YamlSyntaxKind {
        while let Some(c) = self.current_byte() {
            if c == b'\n' {
                break;
            }
            self.advance(1);
        }
        PLAIN_LITERAL
    }

    // https://yaml.org/spec/1.2.2/#731-double-quoted-style
    fn consume_double_quoted_literal(&mut self) -> YamlSyntaxKind {
        self.assert_byte(b'"');
        self.advance(1);

        loop {
            match self.current_byte() {
                Some(b'\\') => {
                    if matches!(self.peek_byte(), Some(b'"')) {
                        self.advance(2)
                    } else {
                        self.advance(1)
                    }
                }
                Some(b'"') => {
                    self.advance(1);
                    break DOUBLE_QUOTED_LITERAL;
                }
                Some(_) => self.advance(1),
                None => break ERROR_TOKEN,
            }
        }
    }

    // https://yaml.org/spec/1.2.2/#732-single-quoted-style
    fn consume_single_quoted_literal(&mut self) -> YamlSyntaxKind {
        self.assert_byte(b'\'');
        self.advance(1);

        loop {
            match self.current_byte() {
                Some(b'\'') => {
                    if matches!(self.peek_byte(), Some(b'\'')) {
                        self.advance(2)
                    } else {
                        self.advance(1);
                        break SINGLE_QUOTED_LITERAL;
                    }
                }
                Some(_) => self.advance(1),
                None => break ERROR_TOKEN,
            }
        }
    }
}

impl<'src> Lexer<'src> for YamlLexer<'src> {
    const NEWLINE: Self::Kind = YamlSyntaxKind::NEWLINE;
    const WHITESPACE: Self::Kind = YamlSyntaxKind::WHITESPACE;

    type Kind = YamlSyntaxKind;
    type LexContext = ();
    type ReLexContext = ();

    fn source(&self) -> &'src str {
        self.source
    }

    fn current(&self) -> Self::Kind {
        self.current_kind
    }

    fn current_range(&self) -> TextRange {
        TextRange::new(self.current_start, TextSize::from(self.position as u32))
    }

    #[inline]
    fn advance_char_unchecked(&mut self) {
        let c = self.current_char_unchecked();
        self.position += c.len_utf8();
    }

    #[inline]
    fn current_start(&self) -> TextSize {
        self.current_start
    }

    fn next_token(&mut self, _context: Self::LexContext) -> Self::Kind {
        self.current_start = TextSize::from(self.position as u32);
        self.current_flags = TokenFlags::empty();
        let kind = match self.current_byte() {
            Some(current) => self.consume_token(current),
            None => EOF,
        };
        self.current_kind = kind;
        kind
    }

    fn has_preceding_line_break(&self) -> bool {
        self.current_flags.has_preceding_line_break()
    }

    fn has_unicode_escape(&self) -> bool {
        self.current_flags.has_unicode_escape()
    }

    fn rewind(&mut self, _checkpoint: biome_parser::lexer::LexerCheckpoint<Self::Kind>) {
        todo!()
    }

    fn finish(self) -> Vec<ParseDiagnostic> {
        self.diagnostics
    }

    fn current_flags(&self) -> TokenFlags {
        self.current_flags
    }

    fn push_diagnostic(&mut self, diagnostic: ParseDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    fn position(&self) -> usize {
        self.position
    }

    #[inline]
    fn advance(&mut self, n: usize) {
        self.position += n;
    }

    /// Consume one newline or all whitespace until a non-whitespace or a newline is found.
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn consume_newline_or_whitespaces(&mut self) -> YamlSyntaxKind {
        if self.consume_newline() {
            YamlSyntaxKind::NEWLINE
        } else {
            self.consume_whitespaces();
            YamlSyntaxKind::WHITESPACE
        }
    }
}
