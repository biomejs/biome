use crate::lexer::{Lexer, Token};
use crate::JsonParserOptions;
use biome_json_syntax::JsonSyntaxKind::{EOF, TOMBSTONE};
use biome_json_syntax::{JsonSyntaxKind, TextRange};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::prelude::TokenSource;
use biome_parser::token_source::Trivia;
use biome_rowan::TriviaPieceKind;

pub(crate) struct JsonTokenSource<'source> {
    lexer: Lexer<'source>,
    trivia: Vec<Trivia>,
    current: JsonSyntaxKind,
    current_range: TextRange,
    preceding_line_break: bool,
    options: JsonParserOptions,
}

impl<'source> JsonTokenSource<'source> {
    pub fn from_str(source: &'source str, options: JsonParserOptions) -> Self {
        let lexer = Lexer::from_str(source).with_options(options);

        let mut source = Self {
            lexer,
            trivia: Vec::new(),
            current: TOMBSTONE,
            current_range: TextRange::default(),
            preceding_line_break: false,
            options,
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
                Ok(trivia_kind) if trivia_kind.is_comment() && !self.options.allow_comments => {
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

impl TokenSource for JsonTokenSource<'_> {
    type Kind = JsonSyntaxKind;

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
        if self.current != EOF {
            self.next_non_trivia_token(false)
        }
    }

    fn skip_as_trivia(&mut self) {
        if self.current() != EOF {
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
