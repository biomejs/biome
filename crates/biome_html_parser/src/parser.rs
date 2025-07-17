use crate::token_source::HtmlTokenSource;
use biome_html_factory::HtmlSyntaxFactory;
use biome_html_syntax::{
    HtmlFileSource, HtmlLanguage, HtmlSyntaxKind, HtmlTextExpressions, HtmlVariant,
};
use biome_parser::diagnostic::{ParseDiagnostic, merge_diagnostics};
use biome_parser::event::Event;
use biome_parser::prelude::*;
use biome_parser::tree_sink::LosslessTreeSink;
use biome_parser::{Parser, ParserContext};

pub(crate) type HtmlLosslessTreeSink<'source> =
    LosslessTreeSink<'source, HtmlLanguage, HtmlSyntaxFactory>;

pub(crate) struct HtmlParser<'source> {
    context: ParserContext<HtmlSyntaxKind>,
    source: HtmlTokenSource<'source>,
    options: HtmlParseOptions,
}

impl<'source> HtmlParser<'source> {
    pub fn new(source: &'source str, options: HtmlParseOptions) -> Self {
        Self {
            context: ParserContext::default(),
            source: HtmlTokenSource::from_str(source),
            options,
        }
    }

    pub(crate) fn options(&self) -> &HtmlParseOptions {
        &self.options
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
pub struct HtmlParseOptions {
    pub(crate) frontmatter: bool,
    pub(crate) text_expression: Option<TextExpressionKind>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum TextExpressionKind {
    Single,
    Double,
}

impl HtmlParseOptions {
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
}

impl From<&HtmlFileSource> for HtmlParseOptions {
    fn from(file_source: &HtmlFileSource) -> Self {
        let mut options = Self::default();

        match file_source.variant() {
            HtmlVariant::Standard(text_expressions) => match text_expressions {
                HtmlTextExpressions::None => {}
                HtmlTextExpressions::Single => {
                    options = options.with_single_text_expression();
                }
                HtmlTextExpressions::Double => {
                    options = options.with_double_text_expression();
                }
            },
            HtmlVariant::Astro => {
                options = options.with_single_text_expression().with_frontmatter();
            }
            HtmlVariant::Vue => {
                options = options.with_double_text_expression();
            }
            HtmlVariant::Svelte => {
                options = options.with_single_text_expression();
            }
        }

        options
    }
}
