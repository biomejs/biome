use std::{fmt, rc::Rc};

use biome_formatter::{
    CstFormatContext, FormatContext, FormatOptions, IndentStyle, IndentWidth, LineEnding,
    LineWidth, TrailingNewline, TransformSourceMap, comments::Comments, printer::PrinterOptions,
};
use biome_markdown_syntax::MarkdownLanguage;

use crate::comments::{FormatMarkdownLeadingComment, MarkdownCommentStyle};

pub type MarkdownComments = Comments<MarkdownLanguage>;

pub struct MarkdownFormatContext {
    source_map: Option<TransformSourceMap>,
    options: MarkdownFormatOptions,
    comments: Rc<MarkdownComments>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MarkdownFormatOptions {
    indent_style: IndentStyle,
    indent_width: IndentWidth,
    line_ending: LineEnding,
    line_width: LineWidth,
    trailing_newline: TrailingNewline,
}

impl CstFormatContext for MarkdownFormatContext {
    type Language = MarkdownLanguage;
    type Style = MarkdownCommentStyle;
    type CommentRule = FormatMarkdownLeadingComment;

    fn comments(&self) -> &MarkdownComments {
        &self.comments
    }
}

impl FormatOptions for MarkdownFormatOptions {
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

    fn trailing_newline(&self) -> TrailingNewline {
        self.trailing_newline
    }
}

impl MarkdownFormatOptions {
    pub fn new() -> Self {
        Self {
            indent_style: IndentStyle::default(),
            indent_width: IndentWidth::default(),
            line_ending: LineEnding::default(),
            line_width: LineWidth::default(),
            trailing_newline: TrailingNewline::default(),
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

    pub fn with_line_width(mut self, line_width: LineWidth) -> Self {
        self.line_width = line_width;
        self
    }

    pub fn with_line_ending(mut self, line_ending: LineEnding) -> Self {
        self.line_ending = line_ending;
        self
    }

    pub fn with_trailing_newline(mut self, trailing_newline: TrailingNewline) -> Self {
        self.trailing_newline = trailing_newline;
        self
    }
}

impl MarkdownFormatContext {
    pub fn new(options: MarkdownFormatOptions) -> Self {
        Self {
            options,
            comments: Rc::new(MarkdownComments::default()),
            source_map: None,
        }
    }

    pub fn with_source_map(mut self, source_map: Option<TransformSourceMap>) -> Self {
        self.source_map = source_map;
        self
    }

    pub fn comments(&self) -> &MarkdownComments {
        &self.comments
    }
}

impl FormatContext for MarkdownFormatContext {
    type Options = MarkdownFormatOptions;

    fn options(&self) -> &Self::Options {
        &self.options
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        None
    }
}

impl fmt::Display for MarkdownFormatOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
