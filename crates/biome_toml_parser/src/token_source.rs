use crate::lexer::{TomlLexContext, TomlLexer};
use biome_parser::{
    diagnostic::ParseDiagnostic,
    lexer::Lexer,
    prelude::BumpWithContext,
    token_source::{TokenSource, Trivia},
};
use biome_rowan::TriviaPieceKind;
use biome_toml_syntax::{TextRange, TomlSyntaxKind, TomlSyntaxKind::EOF};

pub(crate) struct TomlTokenSource<'source> {
    lexer: TomlLexer<'source>,
    trivia: Vec<Trivia>,
    preceding_line_break: bool,
}

impl<'source> TomlTokenSource<'source> {
    pub fn from_str(source: &'source str) -> Self {
        let mut source = Self {
            lexer: TomlLexer::from_str(source),
            trivia: Vec::new(),
            preceding_line_break: false,
        };
        source.next_non_trivia_token(TomlLexContext::Key, true);
        source
    }

    fn next_non_trivia_token(&mut self, mut context: TomlLexContext, first_token: bool) {
        let mut trailing = !first_token;
        self.preceding_line_break = false;

        loop {
            let kind = self.lexer.next_token(context);
            match TriviaPieceKind::try_from(kind) {
                Err(_) => break,
                Ok(trivia_kind) => {
                    if trivia_kind.is_newline() {
                        trailing = false;
                        self.preceding_line_break = true;
                        if context == TomlLexContext::Value {
                            context = TomlLexContext::Key;
                        }
                    }
                    self.trivia
                        .push(Trivia::new(trivia_kind, self.current_range(), trailing));
                }
            }
        }
    }

    pub(crate) fn current_starts_key_value(&self) -> bool {
        self.lexer.current_starts_key_value()
    }

    pub(crate) fn current_starts_unambiguous_table_header(&self) -> bool {
        self.lexer.current_starts_unambiguous_table_header()
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
        self.preceding_line_break
    }

    fn bump(&mut self) {
        self.bump_with_context(TomlLexContext::Key);
    }

    fn skip_as_trivia(&mut self) {
        self.skip_as_trivia_with_context(TomlLexContext::Key);
    }

    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        (self.trivia, self.lexer.finish())
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
            self.trivia.push(Trivia::new(
                TriviaPieceKind::Skipped,
                self.current_range(),
                false,
            ));
            self.next_non_trivia_token(context, false);
        }
    }
}
