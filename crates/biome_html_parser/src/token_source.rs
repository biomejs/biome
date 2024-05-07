use crate::lexer::HtmlLexer;
use biome_html_syntax::HtmlSyntaxKind::EOF;
use biome_html_syntax::{HtmlSyntaxKind, TextRange};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{BufferedLexer, LexContext};
use biome_parser::prelude::BumpWithContext;
use biome_parser::token_source::{TokenSource, TokenSourceWithBufferedLexer, Trivia};
use biome_rowan::TriviaPieceKind;

pub(crate) struct HtmlTokenSource<'source> {
    lexer: BufferedLexer<HtmlSyntaxKind, HtmlLexer<'source>>,

    /// List of the skipped trivia. Needed to construct the CST and compute the non-trivia token offsets.
    pub(super) trivia_list: Vec<Trivia>,
}

#[derive(Copy, Clone, Debug, Default)]
pub(crate) enum HtmlLexContext {
    /// The default state
    #[default]
    Regular,
    #[allow(unused)]
    /// When the lexer is inside a element list, newlines, spaces and quotes are part of the text
    ElementList,
}

impl LexContext for HtmlLexContext {
    fn is_regular(&self) -> bool {
        matches!(self, Self::Regular)
    }
}

impl<'source> HtmlTokenSource<'source> {
    /// Creates a new token source for the given string
    pub fn from_str(source: &'source str) -> Self {
        let lexer = HtmlLexer::from_str(source);

        let buffered = BufferedLexer::new(lexer);
        let mut source = Self::new(buffered);

        source.next_non_trivia_token(HtmlLexContext::Regular, true);
        source
    }

    /// Creates a new token source.
    pub(crate) fn new(lexer: BufferedLexer<HtmlSyntaxKind, HtmlLexer<'source>>) -> Self {
        Self {
            lexer,
            trivia_list: vec![],
        }
    }

    fn next_non_trivia_token(&mut self, context: HtmlLexContext, first_token: bool) {
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
}

impl<'src> TokenSource for HtmlTokenSource<'src> {
    type Kind = HtmlSyntaxKind;

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
        self.bump_with_context(HtmlLexContext::Regular)
    }

    fn skip_as_trivia(&mut self) {
        self.skip_as_trivia_with_context(HtmlLexContext::Regular)
    }

    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        (self.trivia_list, self.lexer.finish())
    }
}

impl<'source> BumpWithContext for HtmlTokenSource<'source> {
    type Context = HtmlLexContext;
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

impl<'source> TokenSourceWithBufferedLexer<HtmlLexer<'source>> for HtmlTokenSource<'source> {
    fn lexer(&mut self) -> &mut BufferedLexer<HtmlSyntaxKind, HtmlLexer<'source>> {
        &mut self.lexer
    }
}
