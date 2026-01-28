use crate::lexer::{MarkdownLexContext, MarkdownLexer, MarkdownReLexContext};
use biome_markdown_syntax::MarkdownSyntaxKind;
use biome_markdown_syntax::MarkdownSyntaxKind::EOF;
use biome_parser::lexer::BufferedLexer;
use biome_parser::prelude::{BumpWithContext, TokenSource};
use biome_parser::token_source::{TokenSourceWithBufferedLexer, Trivia};
use biome_parser::{diagnostic::ParseDiagnostic, token_source::TokenSourceCheckpoint};
use biome_rowan::{TextRange, TriviaPieceKind};

/// Find the start position of the current line in source text.
///
/// Given a slice of text, finds the byte offset where the current line begins
/// (after the last newline, handling CRLF).
fn find_line_start(before: &str) -> usize {
    let last_newline_pos = before.rfind(['\n', '\r']);
    match last_newline_pos {
        Some(pos) => {
            let bytes = before.as_bytes();
            // Handle CRLF: if we found \r and next char is \n, skip both
            if bytes.get(pos) == Some(&b'\r') && bytes.get(pos + 1) == Some(&b'\n') {
                pos + 2
            } else {
                pos + 1
            }
        }
        None => 0,
    }
}

pub(crate) struct MarkdownTokenSource<'source> {
    lexer: BufferedLexer<MarkdownSyntaxKind, MarkdownLexer<'source>>,

    /// List of the skipped trivia. Needed to construct the CST and compute the non-trivia token offsets.
    pub(super) trivia_list: Vec<Trivia>,
}

pub(crate) type MarkdownTokenSourceCheckpoint = TokenSourceCheckpoint<MarkdownSyntaxKind>;

impl<'source> MarkdownTokenSource<'source> {
    /// Creates a new token source.
    pub(crate) fn new(lexer: BufferedLexer<MarkdownSyntaxKind, MarkdownLexer<'source>>) -> Self {
        MarkdownTokenSource {
            lexer,
            trivia_list: vec![],
        }
    }
    /// Creates a new token source for the given string
    pub fn from_str(source: &'source str) -> Self {
        let lexer = MarkdownLexer::from_str(source);

        let buffered = BufferedLexer::new(lexer);
        let mut source = MarkdownTokenSource::new(buffered);

        source.next_non_trivia_token(MarkdownLexContext::default(), true);
        source
    }

    fn next_non_trivia_token(&mut self, context: MarkdownLexContext, first_token: bool) {
        let mut trailing = !first_token;

        loop {
            let kind = self.lexer.next_token(context);

            let trivia_kind = TriviaPieceKind::try_from(kind);

            match trivia_kind {
                Err(_) => {
                    // Not trivia
                    break;
                }
                Ok(trivia_kind) => {
                    if trivia_kind.is_newline() {
                        trailing = false;
                    }

                    self.trivia_list
                        .push(Trivia::new(trivia_kind, self.current_range(), trailing));
                }
            }
        }
    }

    // === Token-based helpers for Stage 1 refactor ===
    // These helpers work with explicit NEWLINE tokens rather than trivia inspection.

    /// Returns true if the current token is at the start of input (position 0).
    ///
    /// This is a position-based check that doesn't rely on trivia_len,
    /// making it work correctly when NEWLINE is an explicit token.
    pub fn at_start_of_input(&self) -> bool {
        self.current_range().start() == 0.into()
    }

    /// Returns the source text starting from the current token position.
    /// This is useful for lookahead when detecting HTML blocks.
    pub fn source_after_current(&self) -> &str {
        let range = self.lexer.current_range();
        let start: usize = range.start().into();
        let source = self.lexer.source();
        &source[start..]
    }

    /// Returns the full source text.
    pub fn source_text(&self) -> &str {
        self.lexer.source()
    }

    /// Count leading indentation on the current line, including whitespace inside the current token.
    ///
    /// This scans from the start of the current line to the first non-whitespace character.
    /// Tab characters are counted as 4 spaces per CommonMark spec.
    pub fn line_start_leading_indent(&self) -> usize {
        let range = self.lexer.current_range();
        let start: usize = range.start().into();

        let source = self.lexer.source();
        let line_start = find_line_start(&source[..start]);

        let line = &source[line_start..];
        let mut count = 0usize;
        for c in line.chars() {
            match c {
                ' ' => count += 1,
                '\t' => count += 4,
                _ => break,
            }
        }
        count
    }

    /// Returns true if the current token starts on a line with only whitespace before it.
    ///
    /// This is a more robust line-start check when NEWLINE is an explicit token.
    pub fn at_line_start_with_whitespace(&self) -> bool {
        let range = self.lexer.current_range();
        let start: usize = range.start().into();

        let source = self.lexer.source();
        let before_token = &source[..start];
        let line_start = find_line_start(before_token);

        source[line_start..start]
            .chars()
            .all(|c| c == ' ' || c == '\t')
    }

    /// Re-lexes the current token in a different context.
    /// Used for context-sensitive parsing like link definitions where whitespace
    /// needs to produce separate tokens to distinguish destination from title.
    pub fn re_lex(&mut self, mode: MarkdownReLexContext) -> MarkdownSyntaxKind {
        self.lexer.re_lex(mode)
    }

    /// Force re-lex the current token in a new lex context.
    ///
    /// Use this after lookahead operations (like `lookahead_reference_link`) when
    /// switching to LinkDefinition context. This ensures that tokens cached during
    /// lookahead in Regular context are discarded and re-lexed correctly.
    pub fn force_relex_in_context(&mut self, context: MarkdownLexContext) -> MarkdownSyntaxKind {
        self.lexer.force_relex_in_context(context)
    }

    pub fn set_force_ordered_list_marker(&mut self, value: bool) {
        self.lexer.lexer_mut().set_force_ordered_list_marker(value);
    }

    /// Bump the current token using the LinkDefinition context.
    /// In this context, whitespace produces separate tokens.
    pub fn bump_link_definition(&mut self) {
        self.bump_with_context(MarkdownLexContext::LinkDefinition);
    }

    /// Creates a checkpoint to which it can later return using [Self::rewind].
    pub fn checkpoint(&self) -> MarkdownTokenSourceCheckpoint {
        MarkdownTokenSourceCheckpoint {
            trivia_len: self.trivia_list.len() as u32,
            lexer_checkpoint: self.lexer.checkpoint(),
        }
    }

    /// Restores the token source to a previous state
    pub fn rewind(&mut self, checkpoint: MarkdownTokenSourceCheckpoint) {
        assert!(self.trivia_list.len() >= checkpoint.trivia_len as usize);
        self.trivia_list.truncate(checkpoint.trivia_len as usize);
        self.lexer.rewind(checkpoint.lexer_checkpoint);
    }
}

impl TokenSource for MarkdownTokenSource<'_> {
    type Kind = MarkdownSyntaxKind;

    fn current(&self) -> Self::Kind {
        self.lexer.current()
    }

    fn current_range(&self) -> TextRange {
        self.lexer.current_range()
    }

    fn text(&self) -> &str {
        self.lexer.source()
    }

    fn has_preceding_line_break(&self) -> bool {
        self.lexer.has_preceding_line_break()
    }

    fn bump(&mut self) {
        self.bump_with_context(MarkdownLexContext::Regular)
    }

    fn skip_as_trivia(&mut self) {
        self.skip_as_trivia_with_context(MarkdownLexContext::Regular)
    }

    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        (self.trivia_list, self.lexer.finish())
    }
}

impl BumpWithContext for MarkdownTokenSource<'_> {
    type Context = MarkdownLexContext;

    fn bump_with_context(&mut self, context: Self::Context) {
        if self.current() != EOF {
            self.next_non_trivia_token(context, false);
        }
    }

    fn skip_as_trivia_with_context(&mut self, context: Self::Context) {
        if self.current() != EOF {
            self.trivia_list.push(Trivia::new(
                TriviaPieceKind::Skipped,
                self.current_range(),
                false,
            ));

            self.next_non_trivia_token(context, true)
        }
    }
}

impl<'source> TokenSourceWithBufferedLexer<MarkdownLexer<'source>>
    for MarkdownTokenSource<'source>
{
    fn lexer(&mut self) -> &mut BufferedLexer<MarkdownSyntaxKind, MarkdownLexer<'source>> {
        &mut self.lexer
    }
}
