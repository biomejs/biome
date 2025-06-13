#![allow(dead_code)]

use std::collections::VecDeque;

use biome_parser::{
    diagnostic::ParseDiagnostic,
    lexer::{Lexer, LexerCheckpoint},
};
use biome_rowan::{TextLen, TextRange, TextSize};
use biome_unicode_table::{Dispatch::WHS, lookup_byte};
use biome_yaml_syntax::{T, YamlSyntaxKind, YamlSyntaxKind::*};
mod tests;

pub(crate) struct YamlLexer<'src> {
    /// Source text
    source: &'src str,

    /// The start byte position in the source text of the next token.
    position: usize,

    current_start_position: usize,

    // TODO: change to usize
    column: i32,

    current_start_column: i32,

    /// diagnostics emitted during the parsing phase
    diagnostics: Vec<ParseDiagnostic>,

    /// Keeps track of the current indentation level to emit INDENT and DEDENT tokens
    indent_levels: Vec<usize>,

    tokens: VecDeque<LexToken>,

    scopes: Vec<LexScope>,

    token_index: usize,
}

impl<'src> YamlLexer<'src> {
    pub fn from_str(source: &'src str) -> Self {
        let mut tokens = VecDeque::new();
        tokens.push_back(LexToken {
            kind: TOMBSTONE,
            start: 0,
            start_col: 0,
            end: 0,
            end_col: 0,
        });
        Self {
            source,
            position: 0,
            diagnostics: Vec::new(),
            indent_levels: Vec::new(),
            tokens,
            column: 0,
            current_start_position: 0,
            current_start_column: 0,
            scopes: vec![LexScope {
                ty: LexScopeType::Document,
                border: -1,
            }],
            token_index: 0,
        }
    }

    fn push_token(&mut self, kind: YamlSyntaxKind) {
        let (end, end_col) = if matches!(
            kind,
            MAPPING_START | MAPPING_END | SEQUENCE_START | SEQUENCE_END | FLOW_START | FLOW_END
        ) {
            (self.current_start_position, self.current_start_column)
        } else {
            (self.position, self.column)
        };
        self.tokens.push_back(LexToken {
            start: self.current_start_position,
            end,
            start_col: self.current_start_column,
            end_col,
            kind,
        });
        self.current_start_column = end_col;
        self.current_start_position = end;
    }

    fn push_scope(&mut self, scope: LexScope) {
        self.scopes.push(scope);
    }

    fn current_scope(&self) -> &LexScope {
        self.scopes.last().unwrap_or(&LexScope {
            ty: LexScopeType::Document,
            border: -1,
        })
    }

    /// Push a token
    fn lex_token(&mut self, current: u8) {
        let start = self.text_position();

        match current {
            _ if self.is_first_plain_char(current) => self.consume_plain_literal(),
            b':' => self.consume_byte(T![:]),
            b',' => self.consume_byte(T![,]),
            b'[' => self.consume_byte(T!['[']),
            b']' => self.consume_byte(T![']']),
            b'?' => self.consume_question(),
            b'-' => self.consume_dash(),
            b'#' => self.consume_comment(),
            b'\'' => self.consume_single_quoted_literal(),
            b'"' => self.consume_double_quoted_literal(),
            _ if is_space(current) || is_break(current) => self.consume_blank(),
            _ => self.consume_unexpected_token(),
        };

        debug_assert!(self.text_position() > start, "Lexer did not advance");
    }

    /// Bumps the current byte and creates a lexed token of the passed in kind.
    #[inline]
    fn consume_byte(&mut self, tok: YamlSyntaxKind) {
        self.advance(1);
        self.push_token(tok);
    }

    fn consume_dash(&mut self) {
        self.assert_byte(b'-');
        let current_scope = self.current_scope();
        if (!matches!(current_scope.ty, LexScopeType::BlockSequence)
            && self.column >= current_scope.border)
            || (matches!(current_scope.ty, LexScopeType::BlockSequence)
                && self.column > current_scope.border)
        {
            self.push_token(SEQUENCE_START);
            self.push_scope(LexScope {
                ty: LexScopeType::BlockSequence,
                border: self.column,
            });
        }
        self.advance(1);
        self.push_token(T![-]);
    }

    fn consume_question(&mut self) {
        self.assert_byte(b'?');
        let current_scope = self.current_scope();
        if !matches!(current_scope.ty, LexScopeType::Flow) && self.column > current_scope.border {
            self.push_token(MAPPING_START);
            self.push_scope(LexScope {
                ty: LexScopeType::BlockMap,
                border: self.column,
            });
        }
        self.advance(1);
        self.push_token(T![?]);
    }

    fn consume_comment(&mut self) {
        self.assert_byte(b'#');
        while let Some(c) = self.current_byte() {
            if c == b'\n' {
                break;
            }
            self.advance(1);
        }
        self.push_token(COMMENT);
    }

    // https://yaml.org/spec/1.2.2/#rule-ns-plain
    // TODO: parse multiline plain scalar at current indentation level
    fn consume_plain_literal(&mut self) {
        self.assert_current_char_boundary();
        debug_assert!(
            self.current_byte()
                .is_some_and(|c| self.is_first_plain_char(c))
        );
        let border = self.column;
        self.advance_char_unchecked();
        while let Some(c) = self.current_byte() {
            // https://yaml.org/spec/1.2.2/#rule-ns-plain-char
            if is_plain_safe(c, self.current_scope()) && c != b':' && c != b'#' {
                self.advance_char_unchecked();
            } else if is_non_space_char(c) && self.peek_byte().is_some_and(|c| c == b'#') {
                self.advance_char_unchecked();
                self.advance(1); // '#'
            } else if c == b':'
                && self
                    .peek_byte()
                    .is_some_and(|c| is_plain_safe(c, self.current_scope()))
            {
                self.advance(1); // ':'
                self.advance_char_unchecked();
            }
            // Yes plain token can contain spaces
            // For example:
            // a bc: x yz
            // Is a valid mapping
            else if is_space(c) {
                self.advance(1);
            } else {
                break;
            }
        }

        debug_assert!(
            self.current_scope().border <= border,
            "Current token's starting position must not be smaller than its scope's border"
        );
        if border <= self.current_scope().border {
            self.push_token(PLAIN_LITERAL);
        } else if self.current_byte().is_some_and(|c| c == b':')
            && self.peek_byte().is_none_or(|c| is_break(c) || is_space(c))
        {
            self.push_scope(LexScope {
                ty: LexScopeType::BlockMap,
                border: self.column,
            });

            self.push_token(MAPPING_START);
            self.push_token(PLAIN_LITERAL)
        } else {
            self.push_token(FLOW_START);
            self.push_token(PLAIN_LITERAL);
            self.push_token(FLOW_END);
        }
    }

    // https://yaml.org/spec/1.2.2/#rule-ns-plain-first
    fn is_first_plain_char(&self, c: u8) -> bool {
        (is_non_space_char(c) && !is_indicator(c))
            || ((c == b'?' || c == b':' || c == b'-')
                && self
                    .peek_byte()
                    .is_some_and(|c| is_plain_safe(c, self.current_scope())))
    }

    // https://yaml.org/spec/1.2.2/#731-double-quoted-style
    fn consume_double_quoted_literal(&mut self) {
        self.assert_byte(b'"');
        self.advance(1);

        let kind = loop {
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
        };
        self.push_token(kind);
    }

    // https://yaml.org/spec/1.2.2/#732-single-quoted-style
    fn consume_single_quoted_literal(&mut self) {
        self.assert_byte(b'\'');
        self.advance(1);

        let kind = loop {
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
        };
        self.push_token(kind);
    }

    fn consume_blank(&mut self) {
        let mut is_newline = false;
        while let Some(current) = self.current_byte() {
            if is_space(current) {
                self.consume_whitespaces();
            } else if is_break(current) {
                self.consume_newline();
                self.column = 0;
                is_newline = true;
            } else {
                break;
            }
        }
        if is_newline {
            self.push_token(NEWLINE);
            while self.column < self.current_scope().border {
                match self.current_scope().ty {
                    LexScopeType::Document => break,
                    LexScopeType::BlockSequence => self.push_token(SEQUENCE_END),
                    LexScopeType::BlockMap => self.push_token(MAPPING_END),
                    LexScopeType::Flow => self.push_token(FLOW_END),
                }
                self.scopes.pop();
            }
        } else {
            self.push_token(WHITESPACE);
        }
    }

    fn consume_unexpected_token(&mut self) {
        self.assert_current_char_boundary();

        let char = self.current_char_unchecked();
        let err = ParseDiagnostic::new(
            format!("unexpected character `{char}`"),
            self.text_position()..self.text_position() + char.text_len(),
        );
        self.diagnostics.push(err);
        self.advance(char.len_utf8());
        self.push_token(ERROR_TOKEN);
    }

    fn current_token(&self) -> &LexToken {
        // Gracefully handle out of index
        &self.tokens[self.token_index]
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
        self.current_token().kind
    }

    fn current_range(&self) -> TextRange {
        TextRange::new(
            TextSize::from(self.current_token().start as u32),
            TextSize::from(self.current_token().end as u32),
        )
    }

    #[inline]
    fn advance(&mut self, n: usize) {
        self.position += n;
        self.column += n as i32;
    }

    #[inline]
    fn advance_char_unchecked(&mut self) {
        let c = self.current_char_unchecked();
        self.position += c.len_utf8();
    }

    #[inline]
    fn current_start(&self) -> TextSize {
        TextSize::from(self.current_token().start as u32)
    }

    fn next_token(&mut self, _context: Self::LexContext) -> Self::Kind {
        if self.tokens.len() <= self.token_index + 1 {
            match self.current_byte() {
                Some(current) => self.lex_token(current),
                None => {
                    while let Some(scope) = self.scopes.pop() {
                        match scope.ty {
                            LexScopeType::Document => break,
                            LexScopeType::BlockSequence => self.push_token(SEQUENCE_END),
                            LexScopeType::BlockMap => self.push_token(MAPPING_END),
                            LexScopeType::Flow => self.push_token(FLOW_END),
                        }
                    }
                    self.push_token(EOF)
                }
            }
        }
        self.token_index += 1;
        self.current_token().kind
    }

    fn has_preceding_line_break(&self) -> bool {
        false
    }

    fn has_unicode_escape(&self) -> bool {
        false
    }

    fn rewind(&mut self, _: LexerCheckpoint<Self::Kind>) {
        unimplemented!()
    }

    fn finish(self) -> Vec<ParseDiagnostic> {
        self.diagnostics
    }

    fn push_diagnostic(&mut self, diagnostic: ParseDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    fn position(&self) -> usize {
        self.position
    }

    /// Consumes all whitespace until a non-whitespace or a newline is found.
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn consume_whitespaces(&mut self) {
        self.assert_current_char_boundary();

        while let Some(c) = self.current_byte() {
            let dispatch = lookup_byte(c);
            if !matches!(dispatch, WHS) {
                break;
            }

            if is_space(c) {
                self.advance(1);
            } else if is_break(c) {
                break;
            } else {
                let start = self.text_position();
                self.advance(1);

                self.push_diagnostic(
                    ParseDiagnostic::new(
                        "The YAML standard allows only two types of whitespace characters: tabs and spaces",
                        start..self.text_position(),
                    )
                        .with_hint("Use a regular whitespace character instead. For more detail, please check https://yaml.org/spec/1.2.2/#55-white-space-characters"),
                )
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct LexToken {
    start: usize,
    end: usize,
    start_col: i32,
    end_col: i32,
    kind: YamlSyntaxKind,
}

struct LexScope {
    ty: LexScopeType,
    /// Scope starting column
    border: i32,
}

enum LexScopeType {
    Document,
    BlockSequence,
    BlockMap,
    Flow,
}

// https://yaml.org/spec/1.2.2/#rule-ns-plain-safe
fn is_plain_safe(c: u8, scope: &LexScope) -> bool {
    match scope.ty {
        LexScopeType::Flow => is_non_space_char(c) && !is_flow_indicator(c),
        _ => is_non_space_char(c),
    }
}

// https://yaml.org/spec/1.2.2/#rule-ns-char
fn is_non_space_char(c: u8) -> bool {
    !is_space(c) && !is_break(c)
}

// https://yaml.org/spec/1.2.2/#rule-s-white
fn is_space(c: u8) -> bool {
    c == b' ' || c == b'\t'
}

// https://yaml.org/spec/1.2.2/#rule-b-char
fn is_break(c: u8) -> bool {
    c == b'\n' || c == b'\r'
}

// https://yaml.org/spec/1.2.2/#rule-c-indicator
fn is_indicator(c: u8) -> bool {
    c == b'-'
        || c == b'?'
        || c == b':'
        || c == b'#'
        || c == b'&'
        || c == b'*'
        || c == b'!'
        || c == b'|'
        || c == b'>'
        || c == b'\''
        || c == b'"'
        || c == b'%'
        || c == b'@'
        || c == b'`'
        || is_flow_indicator(c)
}

// https://yaml.org/spec/1.2.2/#rule-c-flow-indicator
fn is_flow_indicator(c: u8) -> bool {
    c == b',' || c == b'[' || c == b']' || c == b'{' || c == b'}'
}
