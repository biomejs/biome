use crate::lexer::{Lexer, Token};
use biome_demo_syntax::MarkdownSyntaxKind;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::prelude::TokenSource;
use biome_parser::token_source::Trivia;
use biome_rowan::{TextRange, TriviaPieceKind};

pub(crate) struct DemoTokenSource<'source> {
    lexer: Lexer<'source>,
    trivia: Vec<Trivia>,
    current: MarkdownSyntaxKind,
    current_range: TextRange,
    preceding_line_break: bool,
}

impl<'source> DemoTokenSource<'source> {
    pub fn from_str(source: &'source str) -> Self {
        let lexer = Lexer::from_str(source);

        let mut source = Self {
            lexer,
            trivia: Vec::new(),
            current: MarkdownSyntaxKind::TOMBSTONE,
            current_range: TextRange::default(),
            preceding_line_break: false,
        };

        source.next_non_trivia_token(true);
        source
    }

    fn next_non_trivia_token(&mut self, first_token: bool) {
        let mut trailing = !first_token;
        self.preceding_line_break = false;

        while let Some(token) = self.lexer.next_token() {
            let trivia_kind = TriviaPieceKind::try_from(token.kind());

            match trivia_kind {
                Err(_) => {
                    self.set_current_token(token);
                    // Not trivia
                    break;
                }
                Ok(trivia_kind) if trivia_kind.is_comment() => {
                    self.set_current_token(token);

                    // Not trivia
                    break;
                }
                Ok(trivia_kind) => {
                    if trivia_kind.is_newline() {
                        trailing = false;
                        self.preceding_line_break = true;
                    }

                    self.trivia
                        .push(Trivia::new(trivia_kind, token.range(), trailing));
                }
            }
        }
    }

    fn set_current_token(&mut self, token: Token) {
        self.current = token.kind();
        self.current_range = token.range()
    }
}

impl<'source> TokenSource for DemoTokenSource<'source> {
    type Kind = MarkdownSyntaxKind;

    fn current(&self) -> Self::Kind {
        self.current
    }

    fn current_range(&self) -> TextRange {
        self.current_range
    }

    fn text(&self) -> &str {
        self.lexer.source()
    }

    fn has_preceding_line_break(&self) -> bool {
        self.preceding_line_break
    }

    fn bump(&mut self) {
        if self.current != MarkdownSyntaxKind::EOF {
            self.next_non_trivia_token(false)
        }
    }

    fn skip_as_trivia(&mut self) {
        if self.current() != MarkdownSyntaxKind::EOF {
            self.trivia.push(Trivia::new(
                TriviaPieceKind::Skipped,
                self.current_range(),
                false,
            ));

            self.next_non_trivia_token(false)
        }
    }

    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        (self.trivia, self.lexer.finish())
    }
}
