use biome_parser::lexer::BufferedLexer;
use biome_parser::prelude::{BumpWithContext, ParseDiagnostic, TokenSource};
use biome_parser::token_source::{TokenSourceCheckpoint, TokenSourceWithBufferedLexer, Trivia};
use biome_rowan::TriviaPieceKind;
use biome_yaml_syntax::YamlSyntaxKind::{self, EOF};

use crate::lexer::{YamlLexContext, YamlLexer};

pub(crate) struct YamlTokenSource<'source> {
    lexer: BufferedLexer<YamlSyntaxKind, YamlLexer<'source>>,
    trivia_list: Vec<Trivia>,
}

pub(crate) type YamlTokenSourceCheckpoint = TokenSourceCheckpoint<YamlSyntaxKind>;

impl<'source> YamlTokenSource<'source> {
    pub(crate) fn new(lexer: BufferedLexer<YamlSyntaxKind, YamlLexer<'source>>) -> Self {
        Self {
            lexer,
            trivia_list: Vec::new(),
        }
    }

    pub fn from_str(source: &'source str) -> Self {
        let lexer = YamlLexer::from_str(source);
        let lexer = BufferedLexer::new(lexer);

        let mut source = YamlTokenSource::new(lexer);
        source.next_non_trivia_token(YamlLexContext::Regular, true);
        source
    }

    fn next_non_trivia_token(&mut self, context: YamlLexContext, first_token: bool) {
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

    pub fn re_lex(&mut self, mode: YamlLexContext) -> YamlSyntaxKind {
        self.lexer.re_lex(mode)
    }

    /// Creates a checkpoint to which it can later return using [Self::rewind].
    pub fn checkpoint(&self) -> YamlTokenSourceCheckpoint {
        YamlTokenSourceCheckpoint {
            trivia_len: self.trivia_list.len() as u32,
            lexer_checkpoint: self.lexer.checkpoint(),
        }
    }

    /// Restores the token source to a previous state
    pub fn rewind(&mut self, checkpoint: YamlTokenSourceCheckpoint) {
        assert!(self.trivia_list.len() >= checkpoint.trivia_len as usize);
        self.trivia_list.truncate(checkpoint.trivia_len as usize);
        self.lexer.rewind(checkpoint.lexer_checkpoint);
    }
}

impl TokenSource for YamlTokenSource<'_> {
    type Kind = YamlSyntaxKind;

    fn current(&self) -> Self::Kind {
        self.lexer.current()
    }

    fn current_range(&self) -> biome_rowan::TextRange {
        self.lexer.current_range()
    }

    fn text(&self) -> &str {
        self.lexer.source()
    }

    fn has_preceding_line_break(&self) -> bool {
        self.lexer.has_preceding_line_break()
    }

    fn bump(&mut self) {
        self.bump_with_context(YamlLexContext::default());
    }

    fn skip_as_trivia(&mut self) {
        if self.current() != EOF {
            self.trivia_list.push(Trivia::new(
                TriviaPieceKind::Skipped,
                self.current_range(),
                false,
            ));

            self.next_non_trivia_token(YamlLexContext::Regular, false)
        }
    }

    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        (self.trivia_list, self.lexer.finish())
    }
}

impl<'source> TokenSourceWithBufferedLexer<YamlLexer<'source>> for YamlTokenSource<'source> {
    fn lexer(&mut self) -> &mut BufferedLexer<YamlSyntaxKind, YamlLexer<'source>> {
        &mut self.lexer
    }
}

impl BumpWithContext for YamlTokenSource<'_> {
    type Context = YamlLexContext;

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
