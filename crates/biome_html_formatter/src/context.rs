use std::{fmt, rc::Rc};

use biome_formatter::{
    printer::PrinterOptions, AttributePosition, BracketSpacing, CstFormatContext, FormatContext,
    FormatOptions, IndentStyle, IndentWidth, LineEnding, LineWidth, TransformSourceMap,
};
use biome_html_syntax::{HtmlFileSource, HtmlLanguage};

use crate::comments::{FormatHtmlLeadingComment, HtmlCommentStyle, HtmlComments};

#[derive(Debug, Clone, Default)]
pub struct HtmlFormatOptions {
    /// The indent style.
    indent_style: IndentStyle,

    /// The indent width.
    indent_width: IndentWidth,

    /// The type of line ending.
    line_ending: LineEnding,

    /// What's the max width of a line. Defaults to 80.
    line_width: LineWidth,

    /// Attribute position style. By default auto.
    attribute_position: AttributePosition,
}

impl HtmlFormatOptions {
    pub fn new(_file_source: HtmlFileSource) -> Self {
        Self {
            ..Default::default()
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

    pub fn with_attribute_position(mut self, attribute_position: AttributePosition) -> Self {
        self.attribute_position = attribute_position;
        self
    }

    pub fn indent_style(&self) -> IndentStyle {
        self.indent_style
    }

    pub fn indent_width(&self) -> IndentWidth {
        self.indent_width
    }

    pub fn line_ending(&self) -> LineEnding {
        self.line_ending
    }

    pub fn line_width(&self) -> LineWidth {
        self.line_width
    }

    pub fn attribute_position(&self) -> AttributePosition {
        self.attribute_position
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

    pub fn set_attribute_position(&mut self, attribute_position: AttributePosition) {
        self.attribute_position = attribute_position;
    }
}

impl fmt::Display for HtmlFormatOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Indent style: {}", self.indent_style)?;
        writeln!(f, "Indent width: {}", self.indent_width.value())?;
        writeln!(f, "Line ending: {}", self.line_ending)?;
        writeln!(f, "Line width: {}", self.line_width.value())?;
        writeln!(f, "Attribute Position: {}", self.attribute_position)
    }
}

impl FormatOptions for HtmlFormatOptions {
    fn indent_style(&self) -> IndentStyle {
        self.indent_style
    }

    fn indent_width(&self) -> IndentWidth {
        self.indent_width
    }

    fn line_ending(&self) -> LineEnding {
        self.line_ending
    }

    fn line_width(&self) -> LineWidth {
        self.line_width
    }

    fn attribute_position(&self) -> AttributePosition {
        self.attribute_position
    }

    fn bracket_spacing(&self) -> biome_formatter::BracketSpacing {
        BracketSpacing::default()
    }

    fn as_print_options(&self) -> biome_formatter::prelude::PrinterOptions {
        PrinterOptions::from(self)
    }
}

#[derive(Debug, Clone)]
pub struct HtmlFormatContext {
    options: HtmlFormatOptions,

    /// The comments of the nodes and tokens in the document.
    comments: Rc<HtmlComments>,

    source_map: Option<TransformSourceMap>,
}

impl HtmlFormatContext {
    pub fn new(options: HtmlFormatOptions, comments: HtmlComments) -> Self {
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

impl FormatContext for HtmlFormatContext {
    type Options = HtmlFormatOptions;

    fn options(&self) -> &Self::Options {
        &self.options
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        self.source_map.as_ref()
    }
}

impl CstFormatContext for HtmlFormatContext {
    type Language = HtmlLanguage;
    type Style = HtmlCommentStyle;
    type CommentRule = FormatHtmlLeadingComment;

    fn comments(&self) -> &HtmlComments {
        &self.comments
    }
}
