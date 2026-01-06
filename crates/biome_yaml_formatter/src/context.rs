use std::default::Default;
use std::fmt;
use std::rc::Rc;

use biome_formatter::{
    CstFormatContext, FormatContext, FormatOptions, IndentStyle, LineEnding, LineWidth,
    TrailingNewline, TransformSourceMap,
};
use biome_formatter::{IndentWidth, prelude::*};
use biome_yaml_syntax::{YamlFileSource, YamlLanguage};

use crate::YamlCommentStyle;
use crate::comments::{FormatYamlLeadingComment, YamlComments};

#[derive(Debug)]
pub struct YamlFormatContext {
    options: YamlFormatOptions,
    /// The comments of the nodes and tokens in the program.
    comments: Rc<YamlComments>,
    source_map: Option<TransformSourceMap>,
}

impl YamlFormatContext {
    pub fn new(options: YamlFormatOptions, comments: YamlComments) -> Self {
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

impl FormatContext for YamlFormatContext {
    type Options = YamlFormatOptions;

    fn options(&self) -> &Self::Options {
        &self.options
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        None
    }
}

impl CstFormatContext for YamlFormatContext {
    type Language = YamlLanguage;
    type Style = YamlCommentStyle;
    type CommentRule = FormatYamlLeadingComment;

    fn comments(&self) -> &YamlComments {
        &self.comments
    }
}

#[derive(Debug, Default, Clone)]
pub struct YamlFormatOptions {
    indent_style: IndentStyle,
    indent_width: IndentWidth,
    line_ending: LineEnding,
    line_width: LineWidth,
    /// Whether to add a trailing newline at the end of the file. Defaults to true.
    trailing_newline: TrailingNewline,
    /// The kind of file
    _file_source: YamlFileSource,
}

impl YamlFormatOptions {
    pub fn new(file_source: YamlFileSource) -> Self {
        Self {
            _file_source: file_source,
            trailing_newline: TrailingNewline::default(),
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

    pub fn with_trailing_newline(mut self, trailing_newline: TrailingNewline) -> Self {
        self.trailing_newline = trailing_newline;
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

    pub fn set_trailing_newline(&mut self, trailing_newline: TrailingNewline) {
        self.trailing_newline = trailing_newline;
    }
}

impl FormatOptions for YamlFormatOptions {
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

    fn trailing_newline(&self) -> TrailingNewline {
        self.trailing_newline
    }

    fn as_print_options(&self) -> PrinterOptions {
        PrinterOptions::from(self)
    }
}

impl fmt::Display for YamlFormatOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Indent style: {}", self.indent_style)?;
        writeln!(f, "Indent width: {}", self.indent_width.value())?;
        writeln!(f, "Line ending: {}", self.line_ending)?;
        writeln!(f, "Line width: {}", self.line_width.value())?;
        writeln!(f, "Trailing newline: {}", self.trailing_newline.value())?;

        Ok(())
    }
}
