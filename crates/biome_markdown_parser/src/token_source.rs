use crate::lexer::{MarkdownLexContext, MarkdownLexer, MarkdownReLexContext};
use biome_markdown_syntax::MarkdownSyntaxKind;
use biome_markdown_syntax::MarkdownSyntaxKind::EOF;
use biome_parser::lexer::BufferedLexer;
use biome_parser::prelude::{BumpWithContext, TokenSource};
use biome_parser::token_source::{TokenSourceWithBufferedLexer, Trivia};
use biome_parser::{diagnostic::ParseDiagnostic, token_source::TokenSourceCheckpoint};
use biome_rowan::{TextRange, TriviaPieceKind};

pub(crate) struct MarkdownTokenSource<'source> {
    lexer: BufferedLexer<MarkdownSyntaxKind, MarkdownLexer<'source>>,

    /// List of the skipped trivia. Needed to construct the CST and compute the non-trivia token offsets.
    pub(super) trivia_list: Vec<Trivia>,
}

pub(crate) type MarkdownTokenSourceCheckpoint = TokenSourceCheckpoint<MarkdownSyntaxKind>;

impl<'source> MarkdownTokenSource<'source> {
    /// Creates a new token source.
    pub(crate) fn new(
        lexer: BufferedLexer<MarkdownSyntaxKind, MarkdownLexer<'source>>,
    ) -> MarkdownTokenSource<'source> {
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

    /// Returns the number of whitespace characters before the current token until the first new line.
    /// tab will be counted as 4 spaces https://spec.commonmark.org/0.31.2/#tabs
    /// whitespace will be counted as 1 space
    pub fn before_whitespace_count(&self) -> usize {
        let last_trivia: Vec<&Trivia> = self
            .trivia_list
            .iter()
            .rev()
            .take_while(|item| {
                // get before whitespace and tab collect
                matches!(
                    item.kind(),
                    TriviaPieceKind::Whitespace | TriviaPieceKind::Skipped
                )
            })
            .collect();
        last_trivia.iter().fold(0, |count, b| match b.kind() {
            TriviaPieceKind::Skipped => count + 4,
            TriviaPieceKind::Whitespace => count + u32::from(b.len()) as usize,
            _ => count,
        })
    }

    #[expect(dead_code)]
    pub fn re_lex(&mut self, mode: MarkdownReLexContext) -> MarkdownSyntaxKind {
        self.lexer.re_lex(mode)
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
