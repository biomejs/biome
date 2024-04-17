use biome_parser::{
    diagnostic::ParseDiagnostic,
    lexer::{LexContext, Lexer, TokenFlags},
};
use biome_rowan::{TextRange, TextSize};
use biome_yaml_syntax::YamlSyntaxKind::*;
pub use biome_yaml_syntax::*;

pub(crate) struct YamlLexer<'src> {
    /// Source text
    source: &'src str,

    /// The start byte position in the source text of the next token.
    position: usize,

    /// `true` if there has been a line break between the last non-trivia token and the next non-trivia token.
    after_newline: bool,

    /// If the source starts with a Unicode BOM, this is the number of bytes for that token.
    #[allow(unused)]
    unicode_bom_length: usize,

    /// Byte offset of the current token from the start of the source
    /// The range of the current token can be computed by `self.position - self.current_start`
    current_start: TextSize,

    /// The kind of the current token
    current_kind: YamlSyntaxKind,

    /// Byte offset of the current token from the start of the source
    /// The range of the current token can be computed by
    /// `self.position - self.current_start`.

    /// Flags for the current token
    current_flags: TokenFlags,

    /// diagnostics emitted during the parsing phase
    diagnostics: Vec<ParseDiagnostic>,
}

impl<'src> Lexer<'src> for YamlLexer<'src> {
    const NEWLINE: Self::Kind = NEWLINE;
    const WHITESPACE: Self::Kind = WHITESPACE;

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

    fn next_token(&mut self, _context: Self::LexContext) -> Self::Kind {
        todo!()
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
            NEWLINE
        } else {
            self.consume_whitespaces();
            WHITESPACE
        }
    }
}

/// Context in which the lexer should lex the next token
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum YamlLexContext {
    #[default]
    Regular,
}

impl LexContext for YamlLexContext {
    /// Returns true if this is [YamlLexContext::Regular]
    fn is_regular(&self) -> bool {
        matches!(self, YamlLexContext::Regular)
    }
}

/// Context in which the [YamlLexContext]'s current yoken should be re-lexed.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum YamlReLexContext {}
