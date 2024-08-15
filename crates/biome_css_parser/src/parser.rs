use crate::lexer::CssReLexContext;
use crate::state::CssParserState;
use crate::token_source::{CssTokenSource, CssTokenSourceCheckpoint};
use biome_css_syntax::CssSyntaxKind;
use biome_parser::diagnostic::merge_diagnostics;
use biome_parser::event::Event;
use biome_parser::token_source::Trivia;
use biome_parser::ParserContext;
use biome_parser::{prelude::*, ParserContextCheckpoint};

pub(crate) struct CssParser<'source> {
    context: ParserContext<CssSyntaxKind>,
    source: CssTokenSource<'source>,
    state: CssParserState,
    options: CssParserOptions,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct CssParserOptions {
    /// If this is `true`, **wrong** comments starting with `//` will be treated
    /// as a comment.
    ///
    /// This option exists because there are so many css-in-js tools and people
    /// use `//` as a comment because it's javascript file.
    ///
    /// Defaults to `false`.
    pub allow_wrong_line_comments: bool,

    /// Enables parsing of CSS Modules specific features.
    /// Defaults to `false`.
    pub css_modules: bool,

    /// Enables parsing of Grit metavariables.
    /// Defaults to `false`.
    pub grit_metavariables: bool,
}

impl CssParserOptions {
    /// Allows the parser to parse wrong line comments.
    pub fn allow_wrong_line_comments(mut self) -> Self {
        self.allow_wrong_line_comments = true;
        self
    }

    /// Enables parsing of css modules selectors.
    pub fn allow_css_modules(mut self) -> Self {
        self.css_modules = true;
        self
    }

    /// Enables parsing of Grit metavariables.
    pub fn allow_metavariables(mut self) -> Self {
        self.grit_metavariables = true;
        self
    }

    /// Checks if parsing of CSS Modules features is disabled.
    pub fn is_css_modules_disabled(&self) -> bool {
        !self.css_modules
    }

    /// Checks if parsing of Grit metavariables is enabled.
    pub fn is_metavariable_enabled(&self) -> bool {
        self.grit_metavariables
    }
}

impl<'source> CssParser<'source> {
    pub fn new(source: &'source str, options: CssParserOptions) -> Self {
        Self {
            context: ParserContext::default(),
            source: CssTokenSource::from_str(source, options),
            state: CssParserState::new(),
            options,
        }
    }

    pub fn options(&self) -> &CssParserOptions {
        &self.options
    }

    /// Re-lexes the current token in the specified context. Returns the kind
    /// of the re-lexed token (can be the same as before if the context doesn't make a difference for the current token)
    pub fn re_lex(&mut self, context: CssReLexContext) -> CssSyntaxKind {
        self.source_mut().re_lex(context)
    }

    pub(crate) fn state(&self) -> &CssParserState {
        &self.state
    }

    pub(crate) fn state_mut(&mut self) -> &mut CssParserState {
        &mut self.state
    }

    pub fn checkpoint(&self) -> CssParserCheckpoint {
        CssParserCheckpoint {
            context: self.context.checkpoint(),
            source: self.source.checkpoint(),
            // `state` is not checkpointed because it (currently) only contains
            // scoped properties that aren't only dependent on checkpoints and
            // should be reset manually when the scope of their use is exited.
        }
    }

    pub fn rewind(&mut self, checkpoint: CssParserCheckpoint) {
        let CssParserCheckpoint { context, source } = checkpoint;

        self.context.rewind(context);
        self.source.rewind(source);
        // `state` is not checkpointed because it (currently) only contains
        // scoped properties that aren't only dependent on checkpoints and
        // should be reset manually when the scope of their use is exited.
    }

    pub fn finish(self) -> (Vec<Event<CssSyntaxKind>>, Vec<ParseDiagnostic>, Vec<Trivia>) {
        let (trivia, lexer_diagnostics) = self.source.finish();
        let (events, parse_diagnostics) = self.context.finish();

        let diagnostics = merge_diagnostics(lexer_diagnostics, parse_diagnostics);

        (events, diagnostics, trivia)
    }
}

impl<'source> Parser for CssParser<'source> {
    type Kind = CssSyntaxKind;
    type Source = CssTokenSource<'source>;

    fn context(&self) -> &ParserContext<Self::Kind> {
        &self.context
    }

    fn context_mut(&mut self) -> &mut ParserContext<Self::Kind> {
        &mut self.context
    }

    fn source(&self) -> &Self::Source {
        &self.source
    }

    fn source_mut(&mut self) -> &mut Self::Source {
        &mut self.source
    }

    fn is_speculative_parsing(&self) -> bool {
        self.state.speculative_parsing
    }
}

pub struct CssParserCheckpoint {
    pub(super) context: ParserContextCheckpoint,
    pub(super) source: CssTokenSourceCheckpoint,
    // `state` is not checkpointed because it (currently) only contains
    // scoped properties that aren't only dependent on checkpoints and
    // should be reset manually when the scope of their use is exited.
}
