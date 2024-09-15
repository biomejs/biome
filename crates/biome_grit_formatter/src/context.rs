use crate::comments::{FormatGritLeadingComment, GritCommentStyle, GritComments};
use biome_formatter::{
    CstFormatContext, FormatContext, FormatOptions, IndentStyle, IndentWidth, LineEnding,
    LineWidth, QuoteStyle, TransformSourceMap,
};
use biome_grit_syntax::GritLanguage;
use std::rc::Rc;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GritFormatContext {
    comments: Rc<GritComments>,
    source_map: Option<TransformSourceMap>,
}

impl GritFormatContext {
    pub fn new(comments: GritComments) -> Self {
        Self {
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
        todo!()
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        todo!()
    }
}
impl CstFormatContext for GritFormatContext {
    type Language = GritLanguage;

    type Style = GritCommentStyle;

    type CommentRule = FormatGritLeadingComment;

    fn comments(&self) -> &biome_formatter::comments::Comments<Self::Language> {
        todo!()
    }
}

#[derive(Debug, Default, Clone, PartialEq)]

pub struct GritFormatOptions {
    indent_style: IndentStyle,
    indent_width: IndentWidth,
    line_ending: LineEnding,
    line_width: LineWidth,
    quote_style: QuoteStyle,
}

impl GritFormatOptions {
    pub fn new() -> Self {
        Self {
            indent_style: IndentStyle::default(),
            indent_width: IndentWidth::default(),
            line_ending: LineEnding::default(),
            line_width: LineWidth::default(),
            quote_style: QuoteStyle::default(),
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

    pub fn quote_style(&self) -> QuoteStyle {
        self.quote_style
    }
}

impl FormatOptions for GritFormatOptions {
    fn indent_style(&self) -> IndentStyle {
        todo!()
    }

    fn indent_width(&self) -> IndentWidth {
        todo!()
    }

    fn line_width(&self) -> LineWidth {
        todo!()
    }

    fn line_ending(&self) -> LineEnding {
        todo!()
    }

    fn attribute_position(&self) -> biome_formatter::AttributePosition {
        todo!()
    }

    fn bracket_spacing(&self) -> biome_formatter::BracketSpacing {
        todo!()
    }

    fn as_print_options(&self) -> biome_formatter::prelude::PrinterOptions {
        todo!()
    }
}
