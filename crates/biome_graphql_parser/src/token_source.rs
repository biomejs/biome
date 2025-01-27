use crate::lexer::GraphqlLexer;
use biome_graphql_syntax::GraphqlSyntaxKind::EOF;
use biome_graphql_syntax::{GraphqlSyntaxKind, TextRange};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::BufferedLexer;
use biome_parser::prelude::TokenSource;
use biome_parser::token_source::{TokenSourceWithBufferedLexer, Trivia};
use biome_rowan::TriviaPieceKind;

pub(crate) struct GraphqlTokenSource<'source> {
    lexer: BufferedLexer<GraphqlSyntaxKind, GraphqlLexer<'source>>,
    trivia_list: Vec<Trivia>,
}

impl<'source> GraphqlTokenSource<'source> {
    pub(crate) fn new(lexer: BufferedLexer<GraphqlSyntaxKind, GraphqlLexer<'source>>) -> Self {
        Self {
            lexer,
            trivia_list: Vec::new(),
        }
    }
    pub fn from_str(source: &'source str) -> Self {
        let lexer = GraphqlLexer::from_str(source);
        let lexer = BufferedLexer::new(lexer);

        let mut source = GraphqlTokenSource::new(lexer);

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

impl TokenSource for GraphqlTokenSource<'_> {
    type Kind = GraphqlSyntaxKind;

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
        if self.current() != EOF {
            self.next_non_trivia_token(false)
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

impl<'source> TokenSourceWithBufferedLexer<GraphqlLexer<'source>> for GraphqlTokenSource<'source> {
    fn lexer(&mut self) -> &mut BufferedLexer<GraphqlSyntaxKind, GraphqlLexer<'source>> {
        &mut self.lexer
    }
}
