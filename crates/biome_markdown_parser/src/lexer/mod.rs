//! An extremely fast, lookup table based, JSON lexer which yields SyntaxKind tokens used by the rome-json parser.

#[rustfmt::skip]
mod tests;

use biome_markdown_syntax::MarkdownSyntaxKind;
use biome_markdown_syntax::MarkdownSyntaxKind::*;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{
    LexContext, Lexer, LexerCheckpoint, LexerWithCheckpoint, ReLexer, TokenFlags,
};
use biome_rowan::{SyntaxKind, TextSize};
use biome_unicode_table::{lookup_byte, Dispatch::*};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum MarkdownLexContext {
    #[default]
    Regular,
}

impl LexContext for MarkdownLexContext {
    /// Returns true if this is [MarkdownLexContext::Regular]
    fn is_regular(&self) -> bool {
        matches!(self, MarkdownLexContext::Regular)
    }
}

/// Context in which the [MarkdownLexContext]'s current should be re-lexed.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MarkdownReLexContext {
    #[expect(dead_code)]
    Regular,
    // UnicodeRange,
}

/// An extremely fast, lookup table based, lossless Markdown lexer
#[derive(Debug)]
pub(crate) struct MarkdownLexer<'src> {
    /// Source text
    source: &'src str,

    /// The start byte position in the source text of the next token.
    position: usize,

    /// `true` if there has been a line break between the last non-trivia token and the next non-trivia token.
    after_newline: bool,

    /// If the source starts with a Unicode BOM, this is the number of bytes for that token.
    unicode_bom_length: usize,

    /// Byte offset of the current token from the start of the source.
    ///
    /// The range of the current token can be computed by `self.position - self.current_start`
    current_start: TextSize,

    /// The kind of the current token
    current_kind: MarkdownSyntaxKind,

    /// Flags for the current token
    current_flags: TokenFlags,

    diagnostics: Vec<ParseDiagnostic>,
}

impl<'src> Lexer<'src> for MarkdownLexer<'src> {
    const NEWLINE: Self::Kind = NEWLINE;

    const WHITESPACE: Self::Kind = WHITESPACE;
    type Kind = MarkdownSyntaxKind;
    type LexContext = MarkdownLexContext;
    type ReLexContext = MarkdownReLexContext;

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

    fn next_token(&mut self, _context: Self::LexContext) -> Self::Kind {
        self.current_start = self.text_position();
        self.current_flags = TokenFlags::empty();

        let kind = match self.current_byte() {
            Some(current) => self.consume_token(current),
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
}

impl<'src> MarkdownLexer<'src> {
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
        }
    }

    pub(crate) fn consume_token(&mut self, current: u8) -> MarkdownSyntaxKind {
        let dispatched = lookup_byte(current);
        match dispatched {
            WHS => self.consume_newline_or_whitespace(),
            MUL | MIN | IDT => self.consume_thematic_break_literal(),
            _ => self.consume_textual(),
        }
    }

    fn text_position(&self) -> TextSize {
        TextSize::try_from(self.position).expect("Input to be smaller than 4 GB")
    }

    /// Bumps the current byte and creates a lexed token of the passed in kind
    #[expect(dead_code)]
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
    fn consume_newline_or_whitespace(&mut self) -> MarkdownSyntaxKind {
        match self.current_byte() {
            Some(b'\n' | b'\r') => self.consume_newline(),
            Some(b' ') => self.consume_whitespace(),
            Some(b'\t') => self.consume_tab(),
            _ => self.consume_textual(),
        }
    }

    /// Consume just one newline/line break.
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn consume_newline(&mut self) -> MarkdownSyntaxKind {
        self.assert_at_char_boundary();

        match self.current_byte() {
            Some(b'\n') => {
                self.advance(1);
            }
            Some(b'\r') => {
                if self.peek_byte() == Some(b'\n') {
                    self.advance(2)
                } else {
                    self.advance(1)
                }
            }
            _ => unreachable!(),
        }
        self.after_newline = true;
        NEWLINE
    }

    /// Consumes all whitespace until a non-whitespace or a newline is found.
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn consume_whitespace(&mut self) -> MarkdownSyntaxKind {
        self.assert_at_char_boundary();
        while let Some(b' ') = self.current_byte() {
            self.advance(1);
        }

        WHITESPACE
    }

    fn consume_tab(&mut self) -> MarkdownSyntaxKind {
        self.assert_at_char_boundary();

        if matches!(self.current_byte(), Some(b'\t')) {
            self.advance(1)
        }
        TAB
    }

    fn consume_thematic_break_literal(&mut self) -> MarkdownSyntaxKind {
        self.assert_at_char_boundary();

        let start_char = match self.current_byte() {
            Some(b'-') => b'-',
            Some(b'*') => b'*',
            Some(b'_') => b'_',
            _ => return self.consume_textual(),
        };

        let mut count = 0;
        loop {
            self.consume_whitespace();
            if matches!(self.current_byte(), Some(ch) if ch == start_char) {
                self.advance(1);
                count += 1;
            } else {
                break;
            }
        }
        // until next newline or eof
        if matches!(self.current_byte(), Some(b'\n' | b'\r') | None) && count >= 3 {
            return MD_THEMATIC_BREAK_LITERAL;
        }
        ERROR_TOKEN
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

    #[inline]
    fn consume_textual(&mut self) -> MarkdownSyntaxKind {
        self.assert_at_char_boundary();

        let char = self.current_char_unchecked();
        self.advance(char.len_utf8());

        MD_TEXTUAL_LITERAL
    }

    /// Bumps the current byte and creates a lexed token of the passed in kind
    #[expect(dead_code)]
    fn consume_byte(&mut self, tok: MarkdownSyntaxKind) -> MarkdownSyntaxKind {
        self.advance(1);
        tok
    }
}

impl<'src> ReLexer<'src> for MarkdownLexer<'src> {
    fn re_lex(&mut self, context: Self::ReLexContext) -> Self::Kind {
        let old_position = self.position;
        self.position = u32::from(self.current_start) as usize;

        let re_lexed_kind = match self.current_byte() {
            Some(current) => match context {
                MarkdownReLexContext::Regular => self.consume_token(current),
                // MarkdownReLexContext::UnicodeRange => self.consume_unicode_range_token(current),
            },
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
}

impl<'src> LexerWithCheckpoint<'src> for MarkdownLexer<'src> {
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
}
