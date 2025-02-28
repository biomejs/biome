use biome_parser::{
    diagnostic::ParseDiagnostic,
    lexer::{LexContext, Lexer, TokenFlags},
};
use biome_rowan::{TextRange, TextSize};
use biome_yaml_syntax::{YamlSyntaxKind, T};
use std::iter::FusedIterator;

#[rustfmt::skip]
mod tests;

pub struct Token {
    kind: YamlSyntaxKind,
    range: TextRange,
}

impl Token {
    #[expect(dead_code)]
    pub fn kind(&self) -> YamlSyntaxKind {
        self.kind
    }

    #[expect(dead_code)]
    pub fn range(&self) -> TextRange {
        self.range
    }
}

pub(crate) struct YamlLexer<'src> {
    /// Source text
    source: &'src str,

    /// The start byte position in the source text of the next token.
    position: usize,

    /// `true` if there has been a line break between the last non-trivia token and the next non-trivia token.
    after_newline: bool,

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

    context: YamlLexContext,
}

impl YamlLexer<'_> {
    fn current_char(&self) -> Option<u8> {
        self.source.as_bytes().get(self.position).copied()
    }

    fn peek_next_char(&self) -> Option<u8> {
        self.source.as_bytes().get(self.position + 1).copied()
    }

    fn is_next_char_whitespace(&self) -> bool {
        matches!(self.peek_next_char(), Some(b' ' | b'\n'))
    }

    /// Consumes and returns the next token, if any
    fn consume_token(&mut self) -> Option<Token> {
        let start = self.text_position();
        let char = self.current_char()?;
        let kind = self.consume_token_in_context(char, self.context);
        self.current_kind = kind;
        let end = self.text_position();
        Some(Token {
            kind,
            range: TextRange::new(start, end),
        })
    }

    /// Consume a byte in the given context
    fn consume_token_in_context(&mut self, current: u8, context: YamlLexContext) -> YamlSyntaxKind {
        if self.position >= self.source.len() {
            return YamlSyntaxKind::EOF;
        }

        let start = self.text_position();

        let kind = match current {
            b'#' => self.consume_comment(),
            b'-' => {
                if self.is_next_char_whitespace() {
                    self.consume_byte(T![-])
                } else {
                    self.consume_identifer_or_value()
                }
            }
            b':' => {
                if self.is_next_char_whitespace() {
                    self.context = YamlLexContext::AfterIdent;
                    self.consume_byte(T![:])
                } else {
                    self.consume_identifer_or_value()
                }
            }
            b'\'' | b'"' => self.consume_string_literal(current),
            b' ' => self.consume_newline_or_whitespaces(),
            b'\n' => {
                if self.context == YamlLexContext::AfterIdent {
                    self.context = YamlLexContext::Regular;
                }
                self.consume_newline_or_whitespaces()
            }
            b'[' => self.consume_array_inline_start(),
            b',' => {
                if self.context == YamlLexContext::AfterInlineArray {
                    self.consume_byte(T![,])
                } else {
                    self.consume_identifer_or_value()
                }
            }
            b']' => {
                if self.context == YamlLexContext::AfterInlineArray {
                    self.consume_array_inline_end()
                } else {
                    self.consume_identifer_or_value()
                }
            }
            _ => match context {
                YamlLexContext::Regular => self.consume_identifer_or_value(),
                YamlLexContext::AfterIdent => self.consume_value(),
                YamlLexContext::AfterInlineArray => self.consume_value(),
            },
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
        self.consume_until_newline();
        YamlSyntaxKind::COMMENT
    }

    fn consume_until_newline(&mut self) {
        while let Some(c) = self.current_char() {
            if c == b'\n' {
                break;
            }
            self.advance(1);
        }
        self.context = YamlLexContext::Regular;
    }

    fn consume_string_literal(&mut self, quote: u8) -> YamlSyntaxKind {
        self.assert_current_char_boundary();
        self.assert_byte(quote);
        self.advance(1);

        let mut escape = false;
        loop {
            match self.current_char() {
                Some(b'\\') => {
                    escape = true;
                    self.advance(1);
                }
                Some(c) if c == quote && !escape => {
                    self.advance(1);
                    break;
                }
                Some(_) => {
                    escape = false;
                    self.advance(1);
                }
                None => {
                    break;
                }
            }
        }
        YamlSyntaxKind::YAML_STRING_VALUE
    }

    /// Consume a line up to the colon or the end of the line
    fn consume_identifer_or_value(&mut self) -> YamlSyntaxKind {
        let start = self.position;
        let mut is_ident = false;
        while let Some(c) = self.current_char() {
            if c == b'\n' {
                break;
            }
            if c == b':' && self.is_next_char_whitespace() {
                is_ident = true;
                break;
            }
            self.advance(1);
        }
        if is_ident {
            YamlSyntaxKind::YAML_IDENTIFIER
        } else {
            let value = &self.source[start..self.position];
            interpret_value(value)
        }
    }

    fn consume_value(&mut self) -> YamlSyntaxKind {
        let start = self.position;
        while let Some(c) = self.current_char() {
            if c == b'\n' {
                break;
            }
            if self.context == YamlLexContext::AfterInlineArray && (c == b',' || c == b']') {
                break;
            }
            self.advance(1);
        }
        let value = &self.source[start..self.position];
        interpret_value(value)
    }

    fn consume_array_inline_start(&mut self) -> YamlSyntaxKind {
        self.assert_byte(b'[');
        self.advance(1);
        self.context = YamlLexContext::AfterInlineArray;
        YamlSyntaxKind::L_BRACK
    }

    fn consume_array_inline_end(&mut self) -> YamlSyntaxKind {
        self.assert_byte(b']');
        self.advance(1);
        self.context = YamlLexContext::Regular;
        YamlSyntaxKind::R_BRACK
    }
}

fn interpret_value(value: &str) -> YamlSyntaxKind {
    match value {
        "true" | "false" => YamlSyntaxKind::YAML_BOOLEAN_VALUE,
        "null" => YamlSyntaxKind::YAML_NULL_VALUE,
        _ => value
            .parse::<f64>()
            .map_or(YamlSyntaxKind::YAML_STRING_VALUE, |_| {
                YamlSyntaxKind::YAML_NUMBER_VALUE
            }),
    }
}

impl<'src> Lexer<'src> for YamlLexer<'src> {
    const NEWLINE: Self::Kind = YamlSyntaxKind::NEWLINE;
    const WHITESPACE: Self::Kind = YamlSyntaxKind::WHITESPACE;

    type Kind = YamlSyntaxKind;
    type LexContext = YamlLexContext;
    type ReLexContext = YamlReLexContext;

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

    fn next_token(&mut self, context: Self::LexContext) -> Self::Kind {
        self.current_start = TextSize::from(self.position as u32);
        self.current_flags = TokenFlags::empty();
        self.consume_token_in_context(self.current_char().unwrap_or(b'\0'), context)
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
            self.after_newline = true;
            YamlSyntaxKind::NEWLINE
        } else {
            self.consume_whitespaces();
            YamlSyntaxKind::WHITESPACE
        }
    }
}

impl Iterator for YamlLexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume_token()
    }
}

impl FusedIterator for YamlLexer<'_> {}

/// Context in which the lexer should lex the next token
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum YamlLexContext {
    #[default]
    Regular,
    /// The lexer has just lexed an identifier and is expecting a value to come next.
    ///
    /// After an identifier, a `: ` is expected to come next, and then the value. If the next token is another identifier, it must come after a newline. Otherwise, values can remain on the same line.
    /// However, it is possible to have a nested mapping on the same line, but the nested mapping must be enclosed in `{}`.
    ///
    /// Valid:
    /// ```yaml
    /// foo:
    ///   bar: baz
    /// ```
    /// ```yaml
    /// foo: 5
    /// ```
    /// ```yaml
    /// foo: { bar: baz }
    /// ```
    /// Invalid:
    /// ```yaml
    /// foo: bar: baz
    /// #       ^ invalid syntax, mapping values not allowed here
    /// ```
    AfterIdent,
    /// The lexer has lexed an inline array and is expecting a value or the end of the array to come next.
    ///
    /// In this context, commas are allowed to separate values. Normally, commas are simply treated as part of a value.
    /// A newline is not allowed in this context. The newline must come after the array has been closed with `]`
    /// ```yaml
    /// foo: [1, 2, 3]
    /// ```
    AfterInlineArray,
}

impl LexContext for YamlLexContext {
    /// Returns true if this is [YamlLexContext::Regular]
    fn is_regular(&self) -> bool {
        matches!(self, YamlLexContext::Regular)
    }
}

/// Context in which the [YamlLexContext]'s current token should be re-lexed.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum YamlReLexContext {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_interpret_value() {
        assert_eq!(interpret_value("true"), YamlSyntaxKind::YAML_BOOLEAN_VALUE);
        assert_eq!(interpret_value("false"), YamlSyntaxKind::YAML_BOOLEAN_VALUE);
        assert_eq!(interpret_value("null"), YamlSyntaxKind::YAML_NULL_VALUE);
        assert_eq!(interpret_value("foo"), YamlSyntaxKind::YAML_STRING_VALUE);
        assert_eq!(interpret_value("1"), YamlSyntaxKind::YAML_NUMBER_VALUE);
        assert_eq!(interpret_value("1.0"), YamlSyntaxKind::YAML_NUMBER_VALUE);
        assert_eq!(interpret_value("1.0.0"), YamlSyntaxKind::YAML_STRING_VALUE);
    }
}
