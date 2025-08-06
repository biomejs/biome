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
    /// Creates a new `HtmlParser` with the given source string and parsing options.
    ///
    /// Initializes the parser context and tokenizes the input HTML source according to the specified options.
    ///
    /// # Examples
    ///
    /// ```
    /// let options = HtmlParseOptions::default().with_single_text_expression();
    /// let parser = HtmlParser::new("<div>Hello</div>", options);
    /// ```
    pub fn new(source: &'source str, options: HtmlParseOptions) -> Self {
        Self {
            context: ParserContext::default(),
            source: HtmlTokenSource::from_str(source),
            options,
        }
    }

    /// Returns a reference to the parser's HTML parse options.
    ///
    /// The returned options determine how the parser handles features such as frontmatter and text expressions.
    pub(crate) fn options(&self) -> &HtmlParseOptions {
        &self.options
    }

    /// Finalizes parsing and returns parse events, diagnostics, and trivia.
    ///
    /// This method completes the parsing process by finalizing both the token source and parser context,
    /// collecting all parse events, diagnostics from both lexer and parser, and trivia tokens.
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - A vector of parse events.
    /// - A vector of combined lexer and parser diagnostics.
    /// - A vector of trivia tokens.
    ///
    /// # Examples
    ///
    /// ```
    /// let parser = HtmlParser::new("<div></div>", HtmlParseOptions::default());
    /// let (events, diagnostics, trivia) = parser.finish();
    /// assert!(!events.is_empty());
    /// ```
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

    /// Returns a mutable reference to the token source used by the parser.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut parser = HtmlParser::new("<div></div>", HtmlParseOptions::default());
    /// let source = parser.source_mut();
    /// // You can now modify the token source if needed.
    /// ```
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
    /// Sets the text expression kind to single quotes in the parse options.
    ///
    /// Returns a new `HtmlParseOptions` with `text_expression` set to `Single`.
    ///
    /// # Examples
    ///
    /// ```
    /// let options = HtmlParseOptions::default().with_single_text_expression();
    /// assert_eq!(options.text_expression, Some(TextExpressionKind::Single));
    /// ```
    pub fn with_single_text_expression(mut self) -> Self {
        self.text_expression = Some(TextExpressionKind::Single);
        self
    }

    /// Sets the text expression kind to double quotes in the parsing options.
    ///
    /// Returns a new `HtmlParseOptions` with `text_expression` set to `Double`.
    ///
    /// # Examples
    ///
    /// ```
    /// let options = HtmlParseOptions::default().with_double_text_expression();
    /// assert_eq!(options.text_expression, Some(TextExpressionKind::Double));
    /// ```
    pub fn with_double_text_expression(mut self) -> Self {
        self.text_expression = Some(TextExpressionKind::Double);
        self
    }

    /// Enables frontmatter parsing in the options.
    ///
    /// Returns a new `HtmlParseOptions` instance with frontmatter enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// let options = HtmlParseOptions::default().with_frontmatter();
    /// assert!(options.frontmatter);
    /// ```
    pub fn with_frontmatter(mut self) -> Self {
        self.frontmatter = true;
        self
    }
}

impl From<&HtmlFileSource> for HtmlParseOptions {
    /// Creates `HtmlParseOptions` from an `HtmlFileSource`, configuring text expression and frontmatter options based on the file variant.
    ///
    /// For standard HTML, sets the text expression kind according to the file's configuration. For Astro, Vue, and Svelte variants, applies the appropriate text expression and enables frontmatter for Astro.
    ///
    /// # Examples
    ///
    /// ```
    /// let file_source = HtmlFileSource::astro();
    /// let options = HtmlParseOptions::from(&file_source);
    /// assert_eq!(options.frontmatter, true);
    /// assert_eq!(options.text_expression, Some(TextExpressionKind::Single));
    /// ```
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
