use crate::GraphqlCommentStyle;
use biome_formatter::{prelude::*, AttributePosition, BracketSpacing, IndentWidth, QuoteStyle};
use biome_formatter::{
    CstFormatContext, FormatContext, FormatOptions, IndentStyle, LineEnding, LineWidth,
    TransformSourceMap,
};

use crate::comments::{FormatGraphqlLeadingComment, GraphqlComments};
use biome_graphql_syntax::{GraphqlFileSource, GraphqlLanguage};
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct GraphqlFormatContext {
    options: GraphqlFormatOptions,
    /// The comments of the nodes and tokens in the program.
    comments: Rc<GraphqlComments>,
    source_map: Option<TransformSourceMap>,
}

impl GraphqlFormatContext {
    pub fn new(options: GraphqlFormatOptions, comments: GraphqlComments) -> Self {
        Self {
            options,
            comments: Rc::new(comments),
            source_map: None,
        }
    }

    pub fn with_source_map(mut self, source_map: Option<TransformSourceMap>) -> Self {
        self.source_map = source_map;
        self
    }
}

impl FormatContext for GraphqlFormatContext {
    type Options = GraphqlFormatOptions;

    fn options(&self) -> &Self::Options {
        &self.options
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        None
    }
}

impl CstFormatContext for GraphqlFormatContext {
    type Language = GraphqlLanguage;
    type Style = GraphqlCommentStyle;
    type CommentRule = FormatGraphqlLeadingComment;

    fn comments(&self) -> &GraphqlComments {
        &self.comments
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct GraphqlFormatOptions {
    indent_style: IndentStyle,
    indent_width: IndentWidth,
    line_ending: LineEnding,
    line_width: LineWidth,
    quote_style: QuoteStyle,
    attribute_position: AttributePosition,
    bracket_spacing: BracketSpacing,
    _file_source: GraphqlFileSource,
}

impl GraphqlFormatOptions {
    pub fn new(file_source: GraphqlFileSource) -> Self {
        Self {
            _file_source: file_source,
            indent_style: IndentStyle::default(),
            indent_width: IndentWidth::default(),
            line_ending: LineEnding::default(),
            line_width: LineWidth::default(),
            quote_style: QuoteStyle::default(),
            attribute_position: AttributePosition::default(),
            bracket_spacing: BracketSpacing::default(),
        }
    }

    pub fn with_indent_style(mut self, indent_style: IndentStyle) -> Self {
        self.indent_style = indent_style;
        self
    }

    pub fn with_indent_width(mut self, indent_width: IndentWidth) -> Self {
        self.indent_width = indent_width;
        self
    }

    pub fn with_line_ending(mut self, line_ending: LineEnding) -> Self {
        self.line_ending = line_ending;
        self
    }

    pub fn with_line_width(mut self, line_width: LineWidth) -> Self {
        self.line_width = line_width;
        self
    }

    pub fn with_bracket_spacing(mut self, bracket_spacing: BracketSpacing) -> Self {
        self.bracket_spacing = bracket_spacing;
        self
    }

    pub fn with_quote_style(mut self, quote_style: QuoteStyle) -> Self {
        self.quote_style = quote_style;
        self
    }

    pub fn set_indent_style(&mut self, indent_style: IndentStyle) {
        self.indent_style = indent_style;
    }

    pub fn set_indent_width(&mut self, indent_width: IndentWidth) {
        self.indent_width = indent_width;
    }

    pub fn set_line_ending(&mut self, line_ending: LineEnding) {
        self.line_ending = line_ending;
    }

    pub fn set_line_width(&mut self, line_width: LineWidth) {
        self.line_width = line_width;
    }

    pub fn set_quote_style(&mut self, quote_style: QuoteStyle) {
        self.quote_style = quote_style;
    }

    pub fn set_bracket_spacing(&mut self, bracket_spacing: BracketSpacing) {
        self.bracket_spacing = bracket_spacing;
    }

    pub fn bracket_spacing(&self) -> BracketSpacing {
        self.bracket_spacing
    }

    pub fn quote_style(&self) -> QuoteStyle {
        self.quote_style
    }
}

impl FormatOptions for GraphqlFormatOptions {
    fn indent_style(&self) -> IndentStyle {
        self.indent_style
    }

    fn indent_width(&self) -> IndentWidth {
        self.indent_width
    }

    fn line_width(&self) -> LineWidth {
        self.line_width
    }

    fn line_ending(&self) -> LineEnding {
        self.line_ending
    }

    fn as_print_options(&self) -> PrinterOptions {
        PrinterOptions::from(self)
    }
}

impl fmt::Display for GraphqlFormatOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Indent style: {}", self.indent_style)?;
        writeln!(f, "Indent width: {}", self.indent_width.value())?;
        writeln!(f, "Line ending: {}", self.line_ending)?;
        writeln!(f, "Line width: {}", self.line_width.value())?;
        writeln!(f, "Bracket spacing: {}", self.bracket_spacing.value())?;
        writeln!(f, "Quote style: {}", self.quote_style)
    }
}
