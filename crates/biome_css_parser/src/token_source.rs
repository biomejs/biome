use crate::CssParserOptions;
use crate::lexer::{CssLexContext, CssLexer, CssReLexContext, CssStringQuote};
use biome_css_syntax::CssSyntaxKind::EOF;
use biome_css_syntax::{CssFileSource, CssSyntaxKind, TextRange};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{BufferedLexer, LexerCheckpoint};
use biome_parser::prelude::{BumpWithContext, TokenSource};
use biome_parser::token_source::{TokenSourceWithBufferedLexer, Trivia};
use biome_rowan::TriviaPieceKind;
use smallvec::SmallVec;

pub(crate) struct CssTokenSource<'src> {
    lexer: BufferedLexer<CssSyntaxKind, CssLexer<'src>>,

    /// List of the skipped trivia. Needed to construct the CST and compute the non-trivia token offsets.
    pub(super) trivia_list: Vec<Trivia>,
    /// Stack of outer SCSS string quotes whose `#{...}` bodies are currently
    /// being lexed through `CssLexContext::Regular`.
    ///
    /// While this stack is non-empty, `effective_context(...)` upgrades
    /// regular lexing to `CssLexContext::ScssStringInterpolation(...)` using
    /// the innermost quote. The stack shape is required because nested
    /// interpolated strings may introduce nested string-origin interpolation
    /// scopes before the outer one closes.
    scss_string_interpolation_quotes: SmallVec<[CssStringQuote; 1]>,
}

#[derive(Debug, Clone)]
pub(crate) struct CssTokenSourceCheckpoint {
    pub lexer_checkpoint: LexerCheckpoint<CssSyntaxKind>,
    /// A `u32` should be enough because `TextSize` is also limited to `u32`.
    pub trivia_len: u32,
    /// Checkpointed copy of `CssTokenSource::scss_string_interpolation_quotes`
    /// so rewinds restore the active string-origin interpolation scopes.
    pub scss_string_interpolation_quotes: SmallVec<[CssStringQuote; 1]>,
}

impl<'src> CssTokenSource<'src> {
    /// Creates a new token source.
    pub(crate) fn new(lexer: BufferedLexer<CssSyntaxKind, CssLexer<'src>>) -> Self {
        CssTokenSource {
            lexer,
            trivia_list: vec![],
            scss_string_interpolation_quotes: SmallVec::new(),
        }
    }

    /// Creates a new token source for the given string
    pub fn from_str(
        source: &'src str,
        options: CssParserOptions,
        source_type: CssFileSource,
    ) -> Self {
        let lexer = CssLexer::from_str(source)
            .with_options(options)
            .with_source_type(source_type);

        let buffered = BufferedLexer::new(lexer);
        let mut source = CssTokenSource::new(buffered);

        source.next_non_trivia_token(CssLexContext::default(), true);
        source
    }

    fn next_non_trivia_token(&mut self, context: CssLexContext, first_token: bool) {
        let mut trailing = !first_token;
        let context = self.effective_context(context);

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

    pub fn re_lex(&mut self, mode: CssReLexContext) -> CssSyntaxKind {
        self.lexer.re_lex(mode)
    }

    pub(crate) fn current_has_pending_scss_string_start(&self) -> bool {
        self.lexer.lexer().has_pending_scss_string_start()
    }

    #[inline]
    fn effective_context(&self, context: CssLexContext) -> CssLexContext {
        match (
            context,
            self.scss_string_interpolation_quotes.last().copied(),
        ) {
            (CssLexContext::Regular, Some(quote)) => CssLexContext::ScssStringInterpolation(quote),
            _ => context,
        }
    }

    pub(crate) fn enter_scss_string_interpolation(&mut self, quote: CssStringQuote) {
        self.scss_string_interpolation_quotes.push(quote);
    }

    pub(crate) fn exit_scss_string_interpolation(&mut self) {
        let previous = self.scss_string_interpolation_quotes.pop();
        debug_assert!(
            previous.is_some(),
            "exiting SCSS string interpolation requires a matching entry"
        );
    }

    /// Creates a checkpoint to which it can later return using [Self::rewind].
    pub fn checkpoint(&self) -> CssTokenSourceCheckpoint {
        CssTokenSourceCheckpoint {
            trivia_len: self.trivia_list.len() as u32,
            lexer_checkpoint: self.lexer.checkpoint(),
            scss_string_interpolation_quotes: self.scss_string_interpolation_quotes.clone(),
        }
    }

    /// Restores the token source to a previous state
    pub fn rewind(&mut self, checkpoint: CssTokenSourceCheckpoint) {
        assert!(self.trivia_list.len() >= checkpoint.trivia_len as usize);
        self.trivia_list.truncate(checkpoint.trivia_len as usize);
        self.lexer.rewind(checkpoint.lexer_checkpoint);
        self.scss_string_interpolation_quotes = checkpoint.scss_string_interpolation_quotes;
    }
}

impl TokenSource for CssTokenSource<'_> {
    type Kind = CssSyntaxKind;

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

    fn has_preceding_whitespace(&self) -> bool {
        self.lexer.has_preceding_whitespace()
    }

    fn bump(&mut self) {
        self.bump_with_context(CssLexContext::Regular)
    }

    fn skip_as_trivia(&mut self) {
        self.skip_as_trivia_with_context(CssLexContext::Regular)
    }

    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        (self.trivia_list, self.lexer.finish())
    }
}

impl BumpWithContext for CssTokenSource<'_> {
    type Context = CssLexContext;

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

impl<'source> TokenSourceWithBufferedLexer<CssLexer<'source>> for CssTokenSource<'source> {
    fn lexer(&mut self) -> &mut BufferedLexer<CssSyntaxKind, CssLexer<'source>> {
        &mut self.lexer
    }
}
