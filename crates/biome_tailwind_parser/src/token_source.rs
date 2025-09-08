use crate::lexer::TailwindLexer;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{BufferedLexer, LexContext};
use biome_parser::prelude::BumpWithContext;
use biome_parser::token_source::{
    TokenSource, TokenSourceCheckpoint, TokenSourceWithBufferedLexer, Trivia,
};
use biome_rowan::TriviaPieceKind;
use biome_tailwind_syntax::TailwindSyntaxKind::EOF;
use biome_tailwind_syntax::{TailwindSyntaxKind, TextRange};

pub(crate) struct TailwindTokenSource<'source> {
    lexer: BufferedLexer<TailwindSyntaxKind, TailwindLexer<'source>>,

    /// List of the skipped trivia. Needed to construct the CST and compute the non-trivia token offsets.
    pub(super) trivia_list: Vec<Trivia>,
}
pub(crate) type TailwindTokenSourceCheckpoint = TokenSourceCheckpoint<TailwindSyntaxKind>;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) enum TailwindLexContext {
    /// The default state.
    #[default]
    Regular,
    /// The lexer has encountered a `[` and the parser has yet to encounter the matching `]`.
    Arbitrary,
    /// Like Arbitrary, but specifically for arbitrary variants.
    ArbitraryVariant,
    /// Like Arbitrary, but specifically for arbitrary candidates.
    ArbitraryCandidate,
}

impl LexContext for TailwindLexContext {
    fn is_regular(&self) -> bool {
        matches!(self, Self::Regular)
    }
}

impl<'source> TailwindTokenSource<'source> {
    /// Creates a new token source for the given string
    pub fn from_str(source: &'source str) -> Self {
        let lexer = TailwindLexer::from_str(source);

        let buffered = BufferedLexer::new(lexer);
        let mut source = Self::new(buffered);

        source.next_non_trivia_token(TailwindLexContext::Regular, true);
        source
    }

    /// Creates a new token source.
    pub(crate) fn new(lexer: BufferedLexer<TailwindSyntaxKind, TailwindLexer<'source>>) -> Self {
        Self {
            lexer,
            trivia_list: vec![],
        }
    }

    fn next_non_trivia_token(&mut self, context: TailwindLexContext, first_token: bool) {
        let mut trailing = !first_token;

        // Unlike most token sources, we can't skip over trivia tokens blindly.
        // This is because whitespace is invalid inside Tailwind utility classes.
        // Tailwind also has no comments, so we don't need to worry about them.
        loop {
            let kind = self.lexer.next_token(context);

            let trivia_kind = TriviaPieceKind::try_from(kind);

            match trivia_kind {
                Err(_) => {
                    // Not trivia
                    break;
                }
                Ok(trivia_kind) => {
                    self.trivia_list
                        .push(Trivia::new(trivia_kind, self.current_range(), trailing));

                    if trivia_kind.is_newline() {
                        trailing = false;
                        // skipping over newlines is OK
                        continue;
                    }
                    break;
                }
            }
        }
    }

    /// Creates a checkpoint to which it can later return using [Self::rewind].
    pub fn checkpoint(&self) -> TailwindTokenSourceCheckpoint {
        TailwindTokenSourceCheckpoint {
            trivia_len: self.trivia_list.len() as u32,
            lexer_checkpoint: self.lexer.checkpoint(),
        }
    }

    /// Restores the token source to a previous state
    pub fn rewind(&mut self, checkpoint: TailwindTokenSourceCheckpoint) {
        assert!(self.trivia_list.len() >= checkpoint.trivia_len as usize);
        self.trivia_list.truncate(checkpoint.trivia_len as usize);
        self.lexer.rewind(checkpoint.lexer_checkpoint);
    }
}

impl TokenSource for TailwindTokenSource<'_> {
    type Kind = TailwindSyntaxKind;

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
        self.bump_with_context(TailwindLexContext::Regular)
    }

    fn skip_as_trivia(&mut self) {
        self.skip_as_trivia_with_context(TailwindLexContext::Regular)
    }

    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        (self.trivia_list, self.lexer.finish())
    }
}

impl BumpWithContext for TailwindTokenSource<'_> {
    type Context = TailwindLexContext;
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

impl<'source> TokenSourceWithBufferedLexer<TailwindLexer<'source>>
    for TailwindTokenSource<'source>
{
    fn lexer(&mut self) -> &mut BufferedLexer<TailwindSyntaxKind, TailwindLexer<'source>> {
        &mut self.lexer
    }
}
