use crate::lexer::{CssLexContext, CssLexer, CssReLexContext};
use crate::CssParserOptions;
use biome_css_syntax::CssSyntaxKind::EOF;
use biome_css_syntax::{CssSyntaxKind, TextRange};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::BufferedLexer;
use biome_parser::prelude::{BumpWithContext, TokenSource};
use biome_parser::token_source::{TokenSourceCheckpoint, TokenSourceWithBufferedLexer, Trivia};
use biome_rowan::TriviaPieceKind;

pub(crate) struct CssTokenSource<'src> {
    lexer: BufferedLexer<CssSyntaxKind, CssLexer<'src>>,

    /// List of the skipped trivia. Needed to construct the CST and compute the non-trivia token offsets.
    pub(super) trivia_list: Vec<Trivia>,
}

pub(crate) type CssTokenSourceCheckpoint = TokenSourceCheckpoint<CssSyntaxKind>;

impl<'src> CssTokenSource<'src> {
    /// Creates a new token source.
    pub(crate) fn new(lexer: BufferedLexer<CssSyntaxKind, CssLexer<'src>>) -> CssTokenSource<'src> {
        CssTokenSource {
            lexer,
            trivia_list: vec![],
        }
    }

    /// Creates a new token source for the given string
    pub fn from_str(source: &'src str, options: CssParserOptions) -> Self {
        let lexer = CssLexer::from_str(source).with_options(options);

        let buffered = BufferedLexer::new(lexer);
        let mut source = CssTokenSource::new(buffered);

        source.next_non_trivia_token(CssLexContext::default(), true);
        source
    }

    fn next_non_trivia_token(&mut self, context: CssLexContext, first_token: bool) {
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

    pub fn re_lex(&mut self, mode: CssReLexContext) -> CssSyntaxKind {
        self.lexer.re_lex(mode)
    }

    /// Creates a checkpoint to which it can later return using [Self::rewind].
    pub fn checkpoint(&self) -> CssTokenSourceCheckpoint {
        CssTokenSourceCheckpoint {
            trivia_len: self.trivia_list.len() as u32,
            lexer_checkpoint: self.lexer.checkpoint(),
        }
    }

    /// Restores the token source to a previous state
    pub fn rewind(&mut self, checkpoint: CssTokenSourceCheckpoint) {
        assert!(self.trivia_list.len() >= checkpoint.trivia_len as usize);
        self.trivia_list.truncate(checkpoint.trivia_len as usize);
        self.lexer.rewind(checkpoint.lexer_checkpoint);
    }
}

impl TokenSource for CssTokenSource<'_> {
    type Kind = CssSyntaxKind;

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
        self.bump_with_context(CssLexContext::Regular)
    }

    fn skip_as_trivia(&mut self) {
        self.skip_as_trivia_with_context(CssLexContext::Regular)
    }

    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        (self.trivia_list, self.lexer.finish())
    }
}

impl BumpWithContext for CssTokenSource<'_> {
    type Context = CssLexContext;

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

impl<'source> TokenSourceWithBufferedLexer<CssLexer<'source>> for CssTokenSource<'source> {
    fn lexer(&mut self) -> &mut BufferedLexer<CssSyntaxKind, CssLexer<'source>> {
        &mut self.lexer
    }
}
