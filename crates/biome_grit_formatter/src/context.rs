use crate::comments::{FormatGritLeadingComment, GritCommentStyle, GritComments};
use biome_formatter::printer::PrinterOptions;
use biome_formatter::{
    AttributePosition, BracketSpacing, CstFormatContext, FormatContext, FormatOptions, IndentStyle,
    IndentWidth, LineEnding, LineWidth, TransformSourceMap,
};
use biome_grit_syntax::file_source::GritFileSource;
use biome_grit_syntax::GritLanguage;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct GritFormatContext {
    options: GritFormatOptions,
    comments: Rc<GritComments>,
    source_map: Option<TransformSourceMap>,
}

impl GritFormatContext {
    pub fn new(options: GritFormatOptions, comments: GritComments) -> Self {
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

impl FormatContext for GritFormatContext {
    type Options = GritFormatOptions;

    fn options(&self) -> &Self::Options {
        &self.options
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        self.source_map.as_ref()
    }
}
impl CstFormatContext for GritFormatContext {
    type Language = GritLanguage;

    type Style = GritCommentStyle;

    type CommentRule = FormatGritLeadingComment;

    fn comments(&self) -> &biome_formatter::comments::Comments<Self::Language> {
        &self.comments
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct GritFormatOptions {
    indent_style: IndentStyle,
    indent_width: IndentWidth,
    line_ending: LineEnding,
    line_width: LineWidth,
    attribute_position: AttributePosition,
    bracket_spacing: BracketSpacing,
    _file_source: GritFileSource,
}

impl GritFormatOptions {
    pub fn new(file_source: GritFileSource) -> Self {
        Self {
            _file_source: file_source,
            indent_style: IndentStyle::default(),
            indent_width: IndentWidth::default(),
            line_ending: LineEnding::default(),
            line_width: LineWidth::default(),
            bracket_spacing: BracketSpacing::default(),
            attribute_position: AttributePosition::default(),
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

    pub fn attribute_position(&self) -> AttributePosition {
        self.attribute_position
    }

    pub fn bracket_spacing(&self) -> BracketSpacing {
        self.bracket_spacing
    }
}

impl Display for GritFormatOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Indent style: {}", self.indent_style)?;
        writeln!(f, "Indent width: {}", self.indent_width.value())?;
        writeln!(f, "Line ending: {}", self.line_ending)?;
        writeln!(f, "Line width: {}", self.line_width.value())?;
        writeln!(f, "Attribute Position: {}", self.attribute_position)
    }
}

impl FormatOptions for GritFormatOptions {
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

    fn as_print_options(&self) -> biome_formatter::prelude::PrinterOptions {
        PrinterOptions::from(self)
    }
}
