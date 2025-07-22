use biome_parser::lexer::Lexer;
use biome_parser::prelude::{ParseDiagnostic, TokenSource};
use biome_parser::token_source::Trivia;
use biome_rowan::TriviaPieceKind;
use biome_yaml_syntax::YamlSyntaxKind::{self, EOF};

use crate::lexer::YamlLexer;

pub(crate) struct YamlTokenSource<'source> {
    lexer: YamlLexer<'source>,
    trivia_list: Vec<Trivia>,
}

impl<'source> YamlTokenSource<'source> {
    pub(crate) fn new(lexer: YamlLexer<'source>) -> Self {
        Self {
            lexer,
            trivia_list: Vec::new(),
        }
    }

    pub fn from_str(source: &'source str) -> Self {
        let lexer = YamlLexer::from_str(source);

        let mut source = YamlTokenSource::new(lexer);
        source.next_non_trivia_token(true);
        source
    }

    fn next_non_trivia_token(&mut self, first_token: bool) {
        let mut trailing = !first_token;

        loop {
            let kind = self.lexer.next_token(());

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
        if self.current() != EOF {
            self.next_non_trivia_token(false);
        }
    }

    fn skip_as_trivia(&mut self) {
        if self.current() != EOF {
            self.trivia_list.push(Trivia::new(
                TriviaPieceKind::Skipped,
                self.current_range(),
                false,
            ));

            self.next_non_trivia_token(false)
        }
    }

    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        (self.trivia_list, self.lexer.finish())
    }
}
