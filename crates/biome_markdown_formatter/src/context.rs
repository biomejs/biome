use std::rc::Rc;

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
    pub fn with_line_width(mut self, line_width: LineWidth) -> Self {
        self.line_width = line_width;
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
