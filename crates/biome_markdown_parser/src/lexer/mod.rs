//! An extremely fast, lookup table based, JSON lexer which yields SyntaxKind tokens used by the rome-json parser.

#[rustfmt::skip]
mod tests;

use biome_markdown_syntax::MarkdownSyntaxKind;
use biome_markdown_syntax::MarkdownSyntaxKind::*;
use biome_markdown_syntax::T;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_rowan::{TextLen, TextRange, TextSize};
use biome_unicode_table::{is_js_id_continue, is_js_id_start, lookup_byte, Dispatch::*};
use std::iter::FusedIterator;
use std::ops::Add;
use std::process::id;
use unicode_bom::Bom;

pub struct Token {
    kind: MarkdownSyntaxKind,
    range: TextRange,
}

impl Token {
    pub fn kind(&self) -> MarkdownSyntaxKind {
        self.kind
    }

    pub fn range(&self) -> TextRange {
        self.range
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum MarkdownLexContext {
    #[default]
    Regular,
}

/// An extremely fast, lookup table based, lossless Markdown lexer
#[derive(Debug)]
pub(crate) struct Lexer<'src> {
    /// Source text
    source: &'src str,

    /// The start byte position in the source text of the next token.
    position: usize,

    diagnostics: Vec<ParseDiagnostic>,

    context: MarkdownLexContext,
}

impl<'src> Lexer<'src> {
    /// Make a new lexer from a str, this is safe because strs are valid utf8
    pub fn from_str(string: &'src str) -> Self {
        Self {
            source: string,
            position: 0,
            diagnostics: vec![],
            context: MarkdownLexContext::Regular,
        }
    }

    /// Returns the source code
    pub fn source(&self) -> &'src str {
        self.source
    }

    pub fn finish(self) -> Vec<ParseDiagnostic> {
        self.diagnostics
    }

    pub fn position(&self) -> usize {
        self.position
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
    fn eat_byte(&mut self, tok: MarkdownSyntaxKind) -> MarkdownSyntaxKind {
        self.advance(1);
        tok
    }
    /// Returns the byte at position `self.position + offset` or `None` if it is out of bounds.
    #[inline]
    fn byte_at(&self, offset: usize) -> Option<u8> {
        self.source()
            .as_bytes()
            .get(self.position() + offset)
            .copied()
    }

    /// Peeks at the next byte
    #[inline]
    fn peek_byte(&self) -> Option<u8> {
        self.byte_at(1)
    }

    /// Consume one newline or all whitespace until a non-whitespace or a newline is found.
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn consume_newline_or_whitespaces(&mut self) -> MarkdownSyntaxKind {
        if self.consume_newline() {
            NEWLINE
        } else {
            self.consume_whitespaces();
            WHITESPACE
        }
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
                                "The Markdown standard only allows tabs, whitespace, carriage return and line feed whitespace.",
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

    /// Advances the current position by `n` bytes.
    #[inline]
    fn advance(&mut self, n: usize) {
        self.position += n;
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
    fn lex_token(&mut self, current: u8) -> MarkdownSyntaxKind {
        // The speed difference comes from the difference in table size, a 2kb table is easily fit into cpu cache
        // While a 16kb table will be ejected from cache very often leading to slowdowns, this also allows LLVM
        // to do more aggressive optimizations on the match regarding how to map it to instructions
        let dispatched = lookup_byte(current);
        match dispatched {
            WHS => self.consume_newline_or_whitespaces(),
            _ => self.consume_textual(),
        }
    }

    
    #[inline]
    fn consume_textual(&mut self) -> MarkdownSyntaxKind {
        self.assert_at_char_boundary();

        let char = self.current_char_unchecked();
        self.advance(char.len_utf8());

        MarkdownSyntaxKind::MARKDOWN_TEXTUAL_LITERAL
    }
    
    
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

impl FusedIterator for Lexer<'_> {}
