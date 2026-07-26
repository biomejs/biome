use crate::token_source::{
    HtmlLexContext, HtmlReLexContext, HtmlTokenSource, HtmlTokenSourceCheckpoint,
    TextExpressionKind,
};
use biome_html_factory::HtmlSyntaxFactory;
use biome_html_syntax::{HtmlLanguage, HtmlSyntaxKind, TextRange, unquote};
use biome_languages::HtmlFileSource;
use biome_languages::html::{HtmlTextExpressions, HtmlVariant};
use biome_parser::diagnostic::{ParseDiagnostic, merge_diagnostics};
use biome_parser::event::Event;
use biome_parser::prelude::*;
use biome_parser::tree_sink::LosslessTreeSink;
use biome_parser::{Parser, ParserContext, ParserContextCheckpoint};

pub(crate) type HtmlLosslessTreeSink<'source> =
    LosslessTreeSink<'source, HtmlLanguage, HtmlSyntaxFactory>;

pub(crate) struct HtmlParser<'source> {
    context: ParserContext<HtmlSyntaxKind>,
    source: HtmlTokenSource<'source>,
    source_text: &'source str,
    options: HtmlParserOptions,
    /// Whether the parser is about to read a top-level block of a Vue
    /// single-file component. Only true for the outermost element list, so
    /// that a `<docs>` nested inside a `<template>` stays ordinary markup.
    at_vue_sfc_top_level: bool,
    /// The value of the attribute currently being parsed. Recorded because a
    /// value is only reachable as a token while it is being consumed, and the
    /// attribute list has to read one back out to answer whether a single-file
    /// component block names a preprocessor.
    attribute_value: Option<TextRange>,
}

impl<'source> HtmlParser<'source> {
    pub fn new(source: &'source str, options: HtmlParserOptions) -> Self {
        Self {
            context: ParserContext::default(),
            source: HtmlTokenSource::from_str(source),
            source_text: source,
            options,
            at_vue_sfc_top_level: false,
            attribute_value: None,
        }
    }

    pub(crate) fn options(&self) -> &HtmlParserOptions {
        &self.options
    }

    /// The full text being parsed, for the few decisions that need to look at
    /// a span of source rather than at the token stream.
    pub(crate) fn source_text(&self) -> &'source str {
        self.source_text
    }

    pub(crate) fn at_sfc_top_level(&self) -> bool {
        self.at_vue_sfc_top_level
    }

    pub(crate) fn set_at_vue_sfc_top_level(&mut self, value: bool) {
        self.at_vue_sfc_top_level = value;
    }

    /// Records the range of the attribute value being consumed. Called as the
    /// value token is bumped, which is the only point at which it is known.
    pub(crate) fn set_attribute_value(&mut self, range: TextRange) {
        self.attribute_value = Some(range);
    }

    /// The text of the value of the attribute that was just parsed, without the
    /// quotes around it, or `None` if the attribute had no value.
    ///
    /// Clears the recorded range, so that an attribute written without a value
    /// doesn't read back the one before it.
    pub(crate) fn take_attribute_value(&mut self) -> Option<&'source str> {
        let range = self.attribute_value.take()?;
        self.source_text
            .get(usize::from(range.start())..usize::from(range.end()))
            .map(unquote)
    }

    pub fn finish(
        self,
    ) -> (
        Vec<Event<HtmlSyntaxKind>>,
        Vec<ParseDiagnostic>,
        Vec<Trivia>,
    ) {
        let (trivia, lexer_diagnostics) = self.source.finish();
        let (events, parse_diagnostics) = self.context.finish();

        let diagnostics = merge_diagnostics(lexer_diagnostics, parse_diagnostics);

        (events, diagnostics, trivia)
    }

    pub(crate) fn checkpoint(&mut self) -> HtmlParserCheckpoint {
        HtmlParserCheckpoint {
            context: self.context.checkpoint(),
            source: self.source.checkpoint(),
            // `state` is not checkpointed because it (currently) only contains
            // scoped properties that aren't only dependent on checkpoints and
            // should be reset manually when the scope of their use is exited.
        }
    }

    pub fn rewind(&mut self, checkpoint: HtmlParserCheckpoint) {
        let HtmlParserCheckpoint { context, source } = checkpoint;

        self.context.rewind(context);
        self.source.rewind(source);
        // `state` is not checkpointed because it (currently) only contains
        // scoped properties that aren't only dependent on checkpoints and
        // should be reset manually when the scope of their use is exited.
    }

    /// Execute a lookahead operation without consuming tokens.
    ///
    /// Saves a checkpoint, executes the provided closure, then rewinds to
    /// the checkpoint. The closure's return value is passed through.
    pub fn lookahead<F, R>(&mut self, op: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        let checkpoint = self.checkpoint();
        let result = op(self);
        self.rewind(checkpoint);
        result
    }

    /// Re-lexes the current token in the specified context. Returns the kind
    /// of the re-lexed token (can be the same as before if the context doesn't make a difference for the current token)
    pub fn re_lex(&mut self, context: HtmlReLexContext) -> HtmlSyntaxKind {
        self.source_mut().re_lex(context)
    }

    /// Signals to the lexer that the frontmatter decision has been made.
    /// After this call, `---` in the `Regular` context is treated as plain
    /// HTML text rather than a `FENCE` token.
    pub(crate) fn set_after_frontmatter(&mut self, value: bool) {
        self.source.set_after_frontmatter(value);
    }

    /// shorthand for: `self.bump_with_context(kind, HtmlLexContext::VueVForValue);`
    #[inline(always)]
    pub fn bump_v_for(&mut self, kind: HtmlSyntaxKind) {
        self.bump_with_context(kind, HtmlLexContext::VueVForValue);
    }

    /// shorthand for: `self.expect_with_context(kind, HtmlLexContext::VueVForValue);`
    #[inline(always)]
    pub fn expect_v_for(&mut self, kind: HtmlSyntaxKind) -> bool {
        self.expect_with_context(kind, HtmlLexContext::VueVForValue)
    }
}

pub struct HtmlParserCheckpoint {
    pub(super) context: ParserContextCheckpoint,
    pub(super) source: HtmlTokenSourceCheckpoint,
    // `state` is not checkpointed because it (currently) only contains
    // scoped properties that aren't only dependent on checkpoints and
    // should be reset manually when the scope of their use is exited.
}

impl<'src> Parser for HtmlParser<'src> {
    type Kind = HtmlSyntaxKind;
    type Source = HtmlTokenSource<'src>;

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
}

#[derive(Default, Debug)]
pub struct HtmlParserOptions {
    pub(crate) frontmatter: bool,
    pub(crate) text_expression: Option<TextExpressionKind>,
    pub(crate) vue: bool,
    pub(crate) svelte: bool,
    pub(crate) is_html: bool,
    pub(crate) angular: bool,
}

impl HtmlParserOptions {
    pub fn with_single_text_expression(mut self) -> Self {
        self.text_expression = Some(TextExpressionKind::Single);
        self
    }

    pub fn with_double_text_expression(mut self) -> Self {
        self.text_expression = Some(TextExpressionKind::Double);
        self
    }

    pub fn with_frontmatter(mut self) -> Self {
        self.frontmatter = true;
        self
    }

    /// Toggle parsing of double-quoted text expressions.
    ///
    /// When `value` is `true`, enables [`TextExpressionKind::Double`].
    /// When `false`, disables text expressions entirely (`None`).
    /// Use [`HtmlParserOptions::with_single_text_expression`] to enable single-quoted mode.
    pub fn set_double_text_expression(&mut self, value: bool) {
        match value {
            true => self.text_expression = Some(TextExpressionKind::Double),
            false => self.text_expression = None,
        }
    }

    pub fn with_vue(mut self) -> Self {
        self.vue = true;
        self
    }

    pub fn set_vue(&mut self, value: bool) {
        self.vue = value;
    }

    pub fn with_svelte(mut self) -> Self {
        self.svelte = true;
        self
    }

    pub fn set_angular(&mut self, value: bool) {
        self.angular = value;
    }

    pub fn with_angular(mut self) -> Self {
        self.angular = true;
        self
    }

    pub fn is_html(&self) -> bool {
        self.is_html
    }
}

impl From<&HtmlFileSource> for HtmlParserOptions {
    fn from(file_source: &HtmlFileSource) -> Self {
        let mut options = Self::default();

        match file_source.variant() {
            HtmlVariant::Standard(text_expressions) => match text_expressions {
                HtmlTextExpressions::Single => {
                    options = options.with_single_text_expression();
                }
                HtmlTextExpressions::Double => {
                    options = options.with_double_text_expression();
                }
                HtmlTextExpressions::None => options.is_html = true,
            },
            HtmlVariant::Astro => {
                options = options.with_single_text_expression().with_frontmatter();
            }
            HtmlVariant::Vue => {
                options = options.with_double_text_expression().with_vue();
            }
            HtmlVariant::Svelte => {
                options = options.with_single_text_expression().with_svelte();
            }
            HtmlVariant::Angular => {
                options = options.with_double_text_expression().with_angular();
            }
        }

        options
    }
}
