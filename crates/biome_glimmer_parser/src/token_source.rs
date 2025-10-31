use crate::lexer::GlimmerLexer;
use biome_glimmer_syntax::GlimmerSyntaxKind::{self, EOF};
use biome_glimmer_syntax::TextRange;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{BufferedLexer, LexContext};
use biome_parser::prelude::BumpWithContext;
use biome_parser::token_source::{
    TokenSource, TokenSourceCheckpoint, TokenSourceWithBufferedLexer, Trivia,
};

pub(crate) struct GlimmerTokenSource<'source> {
    lexer: BufferedLexer<GlimmerSyntaxKind, GlimmerLexer<'source>>,

    /// List of the skipped trivia. Needed to construct the CST and compute the non-trivia token offsets.
    pub(super) trivia_list: Vec<Trivia>,
}

/// Lex context for the Glimmer lexer
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) enum GlimmerLexContext {
    /// Regular template content (text nodes)
    #[default]
    Regular,
    /// Inside a mustache expression {{ ... }}
    InsideMustache,
    /// Inside an HTML/component tag < ... >
    InsideTag,
    /// Inside an attribute value
    AttributeValue,
}

impl LexContext for GlimmerLexContext {
    fn is_regular(&self) -> bool {
        matches!(self, Self::Regular)
    }
}

pub(crate) type GlimmerTokenSourceCheckpoint = TokenSourceCheckpoint<GlimmerSyntaxKind>;

impl<'source> GlimmerTokenSource<'source> {
    /// Creates a new token source for the given string
    pub fn from_str(source: &'source str) -> Self {
        let lexer = GlimmerLexer::from_str(source);

        let buffered = BufferedLexer::new(lexer);
        let mut source = Self::new(buffered);

        source.next_non_trivia_token(GlimmerLexContext::Regular, true);
        source
    }

    /// Creates a new token source.
    pub(crate) fn new(lexer: BufferedLexer<GlimmerSyntaxKind, GlimmerLexer<'source>>) -> Self {
        Self {
            lexer,
            trivia_list: vec![],
        }
    }

    /// Consumes the token source and returns the resulting trivia list and diagnostics
    pub fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        (self.trivia_list, self.lexer.finish())
    }
}

impl<'source> BumpWithContext for GlimmerTokenSource<'source> {
    type Context = GlimmerLexContext;

    fn bump_with_context(&mut self, context: Self::Context) {
        self.next_non_trivia_token(context, false);
    }
}

impl<'source> TokenSource for GlimmerTokenSource<'source> {
    type Kind = GlimmerSyntaxKind;

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
        self.bump_with_context(GlimmerLexContext::Regular)
    }

    fn skip_as_trivia(&mut self) {
        self.skip_as_trivia_with_context(GlimmerLexContext::Regular)
    }

    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        GlimmerTokenSource::finish(self)
    }
}

impl<'source> TokenSourceWithBufferedLexer<GlimmerLexer<'source>> for GlimmerTokenSource<'source> {
    fn lexer(&mut self) -> &mut BufferedLexer<GlimmerSyntaxKind, GlimmerLexer<'source>> {
        &mut self.lexer
    }

    fn trivia(&mut self) -> &mut Vec<Trivia> {
        &mut self.trivia_list
    }

    fn skip_as_trivia_with_context(&mut self, context: GlimmerLexContext) {
        self.next_non_trivia_token(context, true)
    }

    fn next_non_trivia_token(&mut self, context: GlimmerLexContext, skip_as_trivia: bool) {
        let mut trailing = !skip_as_trivia;

        loop {
            let kind = self.lexer.next_token(context);

            let trivia_kind = kind.to_trivia();

            if let Some(trivia_kind) = trivia_kind {
                if !trailing {
                    trailing = trivia_kind.is_newline();
                }

                self.trivia_list
                    .push(Trivia::new(trivia_kind, self.current_range(), trailing));
            } else {
                // Not trivia
                break;
            }
        }
    }
}
