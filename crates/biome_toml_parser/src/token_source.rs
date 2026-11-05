use crate::lexer::{TomlLexContext, TomlLexer};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::BufferedLexer;
use biome_parser::prelude::{BumpWithContext, TokenSource};
use biome_parser::token_source::{TokenSourceWithBufferedLexer, Trivia};
use biome_rowan::TriviaPieceKind;
use biome_toml_syntax::TomlSyntaxKind::EOF;
use biome_toml_syntax::{TextRange, TomlSyntaxKind};

pub(crate) struct TomlTokenSource<'source> {
    lexer: BufferedLexer<TomlSyntaxKind, TomlLexer<'source>>,
    trivia_list: Vec<Trivia>,
}

impl<'source> TomlTokenSource<'source> {
    pub fn from_str(source: &'source str) -> Self {
        let lexer = BufferedLexer::new(TomlLexer::from_str(source));

        let mut source = Self {
            lexer,
            trivia_list: Vec::new(),
        };

        source.next_non_trivia_token(TomlLexContext::default(), true);
        source
    }

    fn next_non_trivia_token(&mut self, context: TomlLexContext, first_token: bool) {
        let mut trailing = !first_token;

        loop {
            let kind = self.lexer.next_token(context);

            match TriviaPieceKind::try_from(kind) {
                Err(_) => break,
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

    /// Lexes the current token again in `context`, discarding diagnostics
    /// reported for the previous interpretation.
    pub fn re_lex(&mut self, context: TomlLexContext) -> TomlSyntaxKind {
        self.lexer.re_lex(context)
    }
}

impl TokenSource for TomlTokenSource<'_> {
    type Kind = TomlSyntaxKind;

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
        self.bump_with_context(TomlLexContext::default());
    }

    fn skip_as_trivia(&mut self) {
        self.skip_as_trivia_with_context(TomlLexContext::default());
    }

    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        (self.trivia_list, self.lexer.finish())
    }
}

impl BumpWithContext for TomlTokenSource<'_> {
    type Context = TomlLexContext;

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

            self.next_non_trivia_token(context, true);
        }
    }
}

impl<'source> TokenSourceWithBufferedLexer<TomlLexer<'source>> for TomlTokenSource<'source> {
    fn lexer(&mut self) -> &mut BufferedLexer<TomlSyntaxKind, TomlLexer<'source>> {
        &mut self.lexer
    }
}
