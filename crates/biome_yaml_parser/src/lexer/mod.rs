#![allow(dead_code)]

use biome_parser::{
    diagnostic::ParseDiagnostic,
    lexer::{LexContext, Lexer, LexerCheckpoint, LexerWithCheckpoint, ReLexer, TokenFlags},
};
use biome_rowan::{TextLen, TextRange, TextSize};
use biome_yaml_syntax::{T, YamlSyntaxKind, YamlSyntaxKind::*};

#[rustfmt::skip]
mod tests;

pub(crate) struct YamlLexer<'src> {
    /// Source text
    source: &'src str,

    /// The start byte position in the source text of the next token.
    position: usize,

    /// If the source starts with a Unicode BOM, this is the number of bytes for that token.
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

impl<'src> YamlLexer<'src> {
    pub fn from_str(source: &'src str) -> Self {
        use biome_parser::lexer::TokenFlags;

        Self {
            source,
            position: 0,
            unicode_bom_length: 0,
            current_kind: TOMBSTONE,
            current_start: TextSize::from(0),
            current_flags: TokenFlags::empty(),
            diagnostics: vec![],
        }
    }

    /// Consume a token in the given context
    fn consume_token(&mut self, current: u8, context: YamlLexContext) -> YamlSyntaxKind {
        let start = self.text_position();

        let kind = match current {
            b':' => self.consume_byte(T![:]),
            b',' => self.consume_byte(T![,]),
            b'[' => self.consume_byte(T!['[']),
            b']' => self.consume_byte(T![']']),
            b'?' => self.consume_byte(T![?]),
            b'-' => self.consume_byte(T![-]),
            b'#' => self.consume_comment(),
            b'\'' => self.consume_single_quoted_literal(),
            b'"' => self.consume_double_quoted_literal(),
            b' ' | b'\n' => self.consume_newline_or_whitespaces(),
            _ if self.is_first_plain_char(current, context) => self.consume_plain_literal(context),
            _ => self.consume_unexpected_token(),
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
    fn consume_plain_literal(&mut self, context: YamlLexContext) -> YamlSyntaxKind {
        self.assert_current_char_boundary();
        debug_assert!(
            self.current_byte()
                .is_some_and(|c| self.is_first_plain_char(c, context))
        );
        self.advance_char_unchecked();
        while let Some(c) = self.current_byte() {
            // https://yaml.org/spec/1.2.2/#rule-ns-plain-char
            if is_plain_safe(c, context) && c != b':' && c != b'#' {
                self.advance_char_unchecked();
            } else if is_non_space_char(c) && self.peek_byte().is_some_and(|c| c == b'#') {
                self.advance_char_unchecked();
                self.advance(1); // '#'
            } else if c == b':' && self.peek_byte().is_some_and(|c| is_plain_safe(c, context)) {
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
        PLAIN_LITERAL
    }

    // https://yaml.org/spec/1.2.2/#rule-ns-plain-first
    fn is_first_plain_char(&self, c: u8, context: YamlLexContext) -> bool {
        (is_non_space_char(c) && !is_indicator(c))
            || ((c == b'?' || c == b':' || c == b'-')
                && self.peek_byte().is_some_and(|c| is_plain_safe(c, context)))
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

    fn consume_unexpected_token(&mut self) -> YamlSyntaxKind {
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
}

impl<'src> Lexer<'src> for YamlLexer<'src> {
    const NEWLINE: Self::Kind = YamlSyntaxKind::NEWLINE;
    const WHITESPACE: Self::Kind = YamlSyntaxKind::WHITESPACE;

    type Kind = YamlSyntaxKind;
    type LexContext = YamlLexContext;
    type ReLexContext = YamlLexContext;

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
        let kind = match self.current_byte() {
            Some(current) => self.consume_token(current, context),
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

    fn rewind(&mut self, checkpoint: LexerCheckpoint<Self::Kind>) {
        let LexerCheckpoint {
            position,
            current_start,
            current_flags,
            current_kind,
            unicode_bom_length,
            diagnostics_pos,
            ..
        } = checkpoint;

        let new_pos = u32::from(position) as usize;

        self.position = new_pos;
        self.current_kind = current_kind;
        self.current_start = current_start;
        self.current_flags = current_flags;
        self.unicode_bom_length = unicode_bom_length;
        self.diagnostics.truncate(diagnostics_pos as usize);
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

impl<'src> ReLexer<'src> for YamlLexer<'src> {
    fn re_lex(&mut self, context: Self::ReLexContext) -> Self::Kind {
        self.position = u32::from(self.current_start) as usize;

        let kind = match self.current_byte() {
            Some(current) => self.consume_token(current, context),
            None => EOF,
        };
        self.current_kind = kind;
        kind
    }
}

impl<'src> LexerWithCheckpoint<'src> for YamlLexer<'src> {
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

/// Lex context as specified in
/// https://yaml.org/spec/1.2.2/#42-production-parameters
#[derive(Default, Clone, Copy)]
pub enum YamlLexContext {
    /// Before getting into the document body, for example:
    /// %YAML 1.2
    /// ---
    /// <document body>
    /// ...
    #[default]
    Regular,
    /// Inside block key context, for example:
    /// abc: xyz
    /// ^^^
    BlockKey,
    /// Inside block value, but outside flow context, for example:
    /// abc: [1, 2, 3]
    ///      ^^^^^^^^^
    FlowOut,
    /// Inside flow context, for example:
    /// abc: [1, 2, [4, 5]]
    ///       ^  ^  ^^^^^^
    FlowIn,
    /// Inside flow key context
    /// abc: {{10: [100]}: 50}
    ///       ^^^^^^^^^^
    FlowKey,
}

impl LexContext for YamlLexContext {
    fn is_regular(&self) -> bool {
        matches!(self, Self::Regular)
    }
}

// https://yaml.org/spec/1.2.2/#rule-ns-plain-safe
fn is_plain_safe(c: u8, context: YamlLexContext) -> bool {
    use YamlLexContext::*;
    match context {
        FlowOut | BlockKey => is_non_space_char(c),
        FlowIn | FlowKey => is_non_space_char(c) && !is_flow_indicator(c),
        // Can happen when a YAML document starts with a plain token. This token will then be
        // used by the parser to determine whether the parser is already inside the document
        // body. This token is not used inside the CST and instead is just used to signal the
        // start of document body
        Regular => is_non_space_char(c) && !is_indicator(c),
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
