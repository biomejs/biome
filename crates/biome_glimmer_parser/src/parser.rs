use crate::token_source::{GlimmerTokenSource, GlimmerTokenSourceCheckpoint};
use biome_glimmer_factory::GlimmerSyntaxFactory;
use biome_glimmer_syntax::{GlimmerLanguage, GlimmerSyntaxKind};
use biome_parser::diagnostic::{merge_diagnostics, ParseDiagnostic};
use biome_parser::event::Event;
use biome_parser::prelude::*;
use biome_parser::tree_sink::LosslessTreeSink;
use biome_parser::{Parser, ParserContext, ParserContextCheckpoint};

pub(crate) type GlimmerLosslessTreeSink<'source> =
    LosslessTreeSink<'source, GlimmerLanguage, GlimmerSyntaxFactory>;

pub(crate) struct GlimmerParser<'source> {
    context: ParserContext<GlimmerSyntaxKind>,
    source: GlimmerTokenSource<'source>,
    options: GlimmerParseOptions,
}

impl<'source> GlimmerParser<'source> {
    pub fn new(source: &'source str, options: GlimmerParseOptions) -> Self {
        Self {
            context: ParserContext::default(),
            source: GlimmerTokenSource::from_str(source),
            options,
        }
    }

    pub(crate) fn options(&self) -> &GlimmerParseOptions {
        &self.options
    }

    pub fn finish(
        self,
    ) -> (
        Vec<Event<GlimmerSyntaxKind>>,
        Vec<ParseDiagnostic>,
        Vec<Trivia>,
    ) {
        let (trivia, lexer_diagnostics) = self.source.finish();
        let (events, parse_diagnostics) = self.context.finish();

        let diagnostics = merge_diagnostics(lexer_diagnostics, parse_diagnostics);

        (events, diagnostics, trivia)
    }

    pub(crate) fn checkpoint(&mut self) -> GlimmerParserCheckpoint {
        GlimmerParserCheckpoint {
            context: self.context.checkpoint(),
            source: self.source.checkpoint(),
        }
    }

    pub fn rewind(&mut self, checkpoint: GlimmerParserCheckpoint) {
        let GlimmerParserCheckpoint { context, source } = checkpoint;

        self.context.rewind(context);
        self.source.rewind(source);
    }
}

pub struct GlimmerParserCheckpoint {
    pub(super) context: ParserContextCheckpoint,
    pub(super) source: GlimmerTokenSourceCheckpoint,
}

impl<'src> Parser for GlimmerParser<'src> {
    type Kind = GlimmerSyntaxKind;
    type Source = GlimmerTokenSource<'src>;

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

/// Options for parsing Glimmer templates
#[derive(Default, Debug, Clone)]
pub struct GlimmerParseOptions {
    // Future options can be added here
    // For example: strict mode, different dialects, etc.
}

impl GlimmerParseOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
