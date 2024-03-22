use crate::lexer::HtmlLexer;
use biome_html_syntax::HtmlSyntaxKind::{EOF, TOMBSTONE};
use biome_html_syntax::{HtmlSyntaxKind, TextRange};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{LexContext, Lexer};
use biome_parser::token_source::{TokenSource, Trivia};
use biome_rowan::TriviaPieceKind;

pub(crate) struct HtmlTokenSource<'source> {
    lexer: HtmlLexer<'source>,
    trivia: Vec<Trivia>,
    current: HtmlSyntaxKind,
    current_range: TextRange,
    preceding_line_break: bool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum HtmlLexContext {
    /// Default context: no particular rules are applied to the lexer logic.
    #[default]
    Regular,

}

impl HtmlLexContext {
    const fn is_regular(&self) -> bool {
        matches!(self, Self::Regular)
    }
}

impl LexContext for HtmlLexContext {
    fn is_regular(&self) -> bool {
        self.is_regular()
    }
}

impl<'source> HtmlTokenSource<'source> {
    pub fn from_str(source: &'source str) -> Self {
        let lexer = HtmlLexer::from_str(source);

        let mut source = Self {
            lexer,
            trivia: Vec::new(),
            current: TOMBSTONE,
            current_range: TextRange::default(),
            preceding_line_break: false,
        };

        source.next_non_trivia_token(true);
        source
    }

    fn next_non_trivia_token(&mut self,  first_token: bool) {
        let mut trailing = !first_token;
        self.preceding_line_break = false;

        loop {
            let token = self.lexer.next_token(()) ;
            if token == EOF {
                break
            } else {
                let trivia_kind = TriviaPieceKind::try_from(token);

                match trivia_kind {
                    Err(_) => {
                        self.current = token;
                        self.current_range = self.lexer.current_range();
                        // Not trivia
                        break;
                    }

                    Ok(trivia_kind) => {
                        if trivia_kind.is_newline() {
                            trailing = false;
                            self.preceding_line_break = true;
                        }

                        self.trivia
                            .push(Trivia::new(trivia_kind, self.lexer.current_range(), trailing));
                    }
                }
            }


        }
    }

}

impl<'src> TokenSource for HtmlTokenSource<'src> {
    type Kind = HtmlSyntaxKind;

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
        self.lexer.has_preceding_line_break()
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
