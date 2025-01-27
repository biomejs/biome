use crate::lexer::GritLexer;
use biome_grit_syntax::GritSyntaxKind::{EOF, TOMBSTONE};
use biome_grit_syntax::{GritSyntaxKind, TextRange};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::Lexer;
use biome_parser::prelude::TokenSource;
use biome_parser::token_source::Trivia;
use biome_rowan::TriviaPieceKind;

pub(crate) struct GritTokenSource<'source> {
    lexer: GritLexer<'source>,
    trivia: Vec<Trivia>,
    current: NonTriviaToken,
    next: Option<NonTriviaToken>,
}

struct NonTriviaToken {
    kind: GritSyntaxKind,
    range: TextRange,
    preceding_line_break: bool,
}

impl Default for NonTriviaToken {
    fn default() -> Self {
        Self {
            kind: TOMBSTONE,
            range: TextRange::default(),
            preceding_line_break: false,
        }
    }
}

impl<'source> GritTokenSource<'source> {
    pub fn from_str(source: &'source str) -> Self {
        let lexer = GritLexer::from_str(source);

        let mut source = Self {
            lexer,
            trivia: Vec::new(),
            current: NonTriviaToken::default(),
            next: None,
        };

        source.advance_to_next_non_trivia_token(true);
        source
    }

    fn advance_to_next_non_trivia_token(&mut self, first_token: bool) {
        self.current = match self.next.take() {
            Some(next) => next,
            None => self.next_non_trivia_token(first_token),
        }
    }

    pub fn lookahead(&mut self) -> GritSyntaxKind {
        match self.next.as_ref() {
            Some(next) => next.kind,
            None if self.current.kind != EOF => {
                let next_token = self.next_non_trivia_token(false);
                let next_kind = next_token.kind;
                self.next = Some(next_token);
                next_kind
            }
            None => EOF,
        }
    }

    #[must_use]
    fn next_non_trivia_token(&mut self, first_token: bool) -> NonTriviaToken {
        let mut non_trivia_token = NonTriviaToken::default();

        let mut trailing = !first_token;

        loop {
            let kind = self.lexer.next_token(());
            let trivia_kind = TriviaPieceKind::try_from(kind);

            match trivia_kind {
                Err(_) => {
                    // Not trivia
                    non_trivia_token.kind = kind;
                    non_trivia_token.range = self.lexer.current_range();
                    break;
                }
                Ok(trivia_kind) => {
                    if trivia_kind.is_newline() {
                        trailing = false;
                        non_trivia_token.preceding_line_break = true;
                    }

                    self.trivia.push(Trivia::new(
                        trivia_kind,
                        self.lexer.current_range(),
                        trailing,
                    ));
                }
            }
        }

        non_trivia_token
    }
}

impl TokenSource for GritTokenSource<'_> {
    type Kind = GritSyntaxKind;

    fn current(&self) -> Self::Kind {
        self.current.kind
    }

    fn current_range(&self) -> TextRange {
        self.current.range
    }

    fn text(&self) -> &str {
        self.lexer.source()
    }

    fn has_preceding_line_break(&self) -> bool {
        self.current.preceding_line_break
    }

    fn bump(&mut self) {
        if self.current.kind != EOF {
            self.advance_to_next_non_trivia_token(false)
        }
    }

    fn skip_as_trivia(&mut self) {
        if self.current() != EOF {
            self.trivia.push(Trivia::new(
                TriviaPieceKind::Skipped,
                self.current_range(),
                false,
            ));

            self.advance_to_next_non_trivia_token(false)
        }
    }

    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        (self.trivia, self.lexer.finish())
    }
}
