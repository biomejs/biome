mod tests;

use crate::token_source::TailwindLexContext;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{Lexer, LexerCheckpoint, LexerWithCheckpoint, ReLexer, TokenFlags};
use biome_rowan::{SyntaxKind, TextLen};
use biome_tailwind_syntax::T;
use biome_tailwind_syntax::TailwindSyntaxKind::*;
use biome_tailwind_syntax::{TailwindSyntaxKind, TextSize};

pub(crate) struct TailwindLexer<'src> {
    /// Source text
    source: &'src str,

    /// The start byte position in the source text of the next token.
    position: usize,

    current_kind: TailwindSyntaxKind,

    current_start: TextSize,

    diagnostics: Vec<ParseDiagnostic>,

    current_flags: TokenFlags,

    preceding_line_break: bool,

    after_newline: bool,

    unicode_bom_length: usize,
}

impl<'src> TailwindLexer<'src> {
    pub fn from_str(source: &'src str) -> Self {
        Self {
            source,
            position: 0,
            current_kind: TailwindSyntaxKind::EOF,
            current_start: TextSize::default(),
            diagnostics: Vec::new(),
            current_flags: TokenFlags::empty(),
            preceding_line_break: false,
            after_newline: false,
            unicode_bom_length: 0,
        }
    }

    fn consume_token(&mut self, current: u8) -> TailwindSyntaxKind {
        match current {
            b'\n' | b'\r' | b'\t' | b' ' => self.consume_newline_or_whitespaces(),
            bracket @ (b'[' | b']' | b'(' | b')') => self.consume_bracket(bracket),
            _ if self.current_kind == T!['['] => self.consume_bracketed_thing(TW_SELECTOR, b']'),
            _ if self.current_kind == T!['('] => self.consume_bracketed_thing(TW_VALUE, b')'),
            _ if self.current_kind == T![-] => self.consume_named_value(),
            _ if self.current_kind == T![/] => self.consume_modifier(),
            b':' => self.consume_byte(T![:]),
            b'-' => self.consume_byte(T![-]),
            b'!' => self.consume_byte(T![!]),
            b'/' => self.consume_byte(T![/]),
            _ if current.is_ascii_alphabetic() => self.consume_base(),
            _ => {
                if self.position == 0 {
                    if let Some((bom, bom_size)) = self.consume_potential_bom(UNICODE_BOM) {
                        self.unicode_bom_length = bom_size;
                        return bom;
                    }
                }
                self.consume_unexpected_character()
            }
        }
    }

    /// Consume a token in the arbitrary context
    fn consume_token_arbitrary(&mut self, current: u8) -> TailwindSyntaxKind {
        match current {
            bracket @ (b'[' | b']' | b'(' | b')') => self.consume_bracket(bracket),
            b'\n' | b'\r' | b'\t' | b' ' => self.consume_newline_or_whitespaces(),
            _ if self.current_kind == T!['['] => self.consume_bracketed_thing(TW_VALUE, b']'),
            _ => self.consume_named_value(),
        }
    }

    /// Consume a token in the arbitrary variant context
    fn consume_token_arbitrary_variant(&mut self, current: u8) -> TailwindSyntaxKind {
        match current {
            bracket @ (b'[' | b']' | b'(' | b')') => self.consume_bracket(bracket),
            b'\n' | b'\r' | b'\t' | b' ' => self.consume_newline_or_whitespaces(),
            _ if self.current_kind == T!['['] => self.consume_bracketed_thing(TW_SELECTOR, b']'),
            _ => self.consume_named_value(),
        }
    }

    fn consume_bracket(&mut self, byte: u8) -> TailwindSyntaxKind {
        let kind = match byte {
            b'[' => T!['['],
            b']' => T![']'],
            b'(' => T!['('],
            b')' => T![')'],
            _ => unreachable!(),
        };
        self.consume_byte(kind)
    }

    fn consume_base(&mut self) -> TailwindSyntaxKind {
        self.assert_current_char_boundary();

        while let Some(byte) = self.current_byte() {
            let char = self.current_char_unchecked();
            if char.is_whitespace() || byte == b'-' || byte == b'!' || byte == b':' {
                break;
            }
            self.advance(char.len_utf8());
        }

        TW_BASE
    }

    fn consume_named_value(&mut self) -> TailwindSyntaxKind {
        self.assert_current_char_boundary();

        while let Some(byte) = self.current_byte() {
            let char = self.current_char_unchecked();
            if char.is_whitespace()
                || byte == b':'
                || byte == b'/'
                || byte == b'!'
                || byte == b']'
                || byte == b')'
            {
                break;
            }
            self.advance(char.len_utf8());
        }

        TW_VALUE
    }

    fn consume_modifier(&mut self) -> TailwindSyntaxKind {
        self.assert_current_char_boundary();

        let mut empty = true;
        while let Some(byte) = self.current_byte() {
            let char = self.current_char_unchecked();
            if char.is_whitespace() || byte == b'!' {
                break;
            }
            self.advance(char.len_utf8());
            empty = false;
        }

        if empty { ERROR_TOKEN } else { TW_VALUE }
    }

    fn consume_bracketed_thing(
        &mut self,
        kind: TailwindSyntaxKind,
        looking_for: u8,
    ) -> TailwindSyntaxKind {
        self.assert_current_char_boundary();

        let mut empty = true;
        while let Some(byte) = self.current_byte() {
            let char = self.current_char_unchecked();
            if byte == looking_for || (byte as char).is_whitespace() {
                break;
            }
            self.advance(char.len_utf8());
            empty = false;
        }

        if empty { ERROR_TOKEN } else { kind }
    }

    /// Bumps the current byte and creates a lexed token of the passed in kind.
    #[inline]
    fn consume_byte(&mut self, tok: TailwindSyntaxKind) -> TailwindSyntaxKind {
        self.advance(1);
        tok
    }

    fn consume_unexpected_character(&mut self) -> TailwindSyntaxKind {
        self.assert_at_char_boundary();

        let char = self.current_char_unchecked();
        let err = ParseDiagnostic::new(
            format!("Unexpected character `{char}`"),
            self.text_position()..self.text_position() + char.text_len(),
        );
        self.diagnostics.push(err);
        self.advance(char.len_utf8());

        ERROR_TOKEN
    }

    /// Asserts that the lexer is at a UTF8 char boundary
    #[inline]
    fn assert_at_char_boundary(&self) {
        debug_assert!(self.source.is_char_boundary(self.position));
    }
}

impl<'src> Lexer<'src> for TailwindLexer<'src> {
    const NEWLINE: Self::Kind = NEWLINE;
    const WHITESPACE: Self::Kind = WHITESPACE;
    type Kind = TailwindSyntaxKind;
    type LexContext = TailwindLexContext;
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

    fn next_token(&mut self, context: Self::LexContext) -> Self::Kind {
        self.current_start = TextSize::from(self.position as u32);
        self.current_flags = TokenFlags::empty();

        let kind = if self.is_eof() {
            EOF
        } else {
            match self.current_byte() {
                Some(current) => match context {
                    TailwindLexContext::Regular => self.consume_token(current),
                    TailwindLexContext::Arbitrary => self.consume_token_arbitrary(current),
                    TailwindLexContext::ArbitraryVariant => {
                        self.consume_token_arbitrary_variant(current)
                    }
                },
                None => EOF,
            }
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
        self.preceding_line_break
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

    fn position(&self) -> usize {
        self.position
    }

    fn push_diagnostic(&mut self, diagnostic: ParseDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    fn advance_char_unchecked(&mut self) {
        let c = self.current_char_unchecked();
        self.position += c.len_utf8();
    }

    fn advance(&mut self, n: usize) {
        self.position += n;
    }
}

impl<'src> LexerWithCheckpoint<'src> for TailwindLexer<'src> {
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

impl<'src> ReLexer<'src> for TailwindLexer<'src> {
    fn re_lex(&mut self, _context: Self::ReLexContext) -> Self::Kind {
        todo!()
    }
}
