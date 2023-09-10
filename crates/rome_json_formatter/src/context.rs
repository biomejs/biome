use crate::JsonCommentStyle;
use rome_formatter::{prelude::*, IndentWidth};
use rome_formatter::{
    CstFormatContext, FormatContext, FormatOptions, IndentStyle, LineWidth, TransformSourceMap,
};

use crate::comments::{FormatJsonLeadingComment, JsonComments};
use biome_json_syntax::JsonLanguage;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct JsonFormatContext {
    options: JsonFormatOptions,
    /// The comments of the nodes and tokens in the program.
    comments: Rc<JsonComments>,
    source_map: Option<TransformSourceMap>,
}

impl JsonFormatContext {
    pub fn new(options: JsonFormatOptions, comments: JsonComments) -> Self {
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

impl FormatContext for JsonFormatContext {
    type Options = JsonFormatOptions;

    fn options(&self) -> &Self::Options {
        &self.options
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        None
    }
}

impl CstFormatContext for JsonFormatContext {
    type Language = JsonLanguage;
    type Style = JsonCommentStyle;
    type CommentRule = FormatJsonLeadingComment;

    fn comments(&self) -> &JsonComments {
        &self.comments
    }
}

#[derive(Debug, Default, Clone)]
pub struct JsonFormatOptions {
    indent_style: IndentStyle,
    indent_width: IndentWidth,
    line_width: LineWidth,
}

impl JsonFormatOptions {
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
}

impl FormatOptions for JsonFormatOptions {
    fn indent_style(&self) -> IndentStyle {
        self.indent_style
    }

    fn indent_width(&self) -> IndentWidth {
        self.indent_width
    }

    fn line_width(&self) -> LineWidth {
        self.line_width
    }

    fn as_print_options(&self) -> PrinterOptions {
        PrinterOptions::from(self)
    }
}

impl fmt::Display for JsonFormatOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Indent style: {}", self.indent_style)?;
        writeln!(f, "Indent width: {}", self.indent_width.value())?;
        writeln!(f, "Line width: {}", self.line_width.value())
    }
}
