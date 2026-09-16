use crate::comments::{FormatMarkdownLeadingComment, MarkdownCommentStyle};
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{
    CstFormatContext, FormatContext, FormatOptions, IndentStyle, IndentWidth, LineEnding,
    LineWidth, TrailingNewline, TransformSourceMap, comments::Comments, printer::PrinterOptions,
};
use biome_markdown_syntax::MarkdownLanguage;
use std::{fmt, rc::Rc, str::FromStr};

pub type MarkdownComments = Comments<MarkdownLanguage>;

#[derive(Debug, Clone)]
pub struct MarkdownFormatContext {
    source_map: Option<TransformSourceMap>,
    options: MdFormatOptions,
    comments: Rc<MarkdownComments>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MdFormatOptions {
    indent_style: IndentStyle,
    indent_width: IndentWidth,
    line_ending: LineEnding,
    line_width: LineWidth,
    trailing_newline: TrailingNewline,
    prose_wrap: ProseWrap,
}

/// Controls whether Biome keeps, adds, or removes line breaks in Markdown paragraphs.
///
/// Manual line breaks are always kept. In Markdown, a manual line break is created by ending a
/// line with two spaces or a backslash.
#[derive(Debug, Default, Clone, Copy, Deserializable, Eq, Hash, Merge, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum ProseWrap {
    /// Keep line breaks as written in the source file. This is the default.
    #[default]
    Preserve,
    /// Wrap paragraphs to fit the configured `lineWidth`.
    Always,
    /// Remove line breaks from paragraphs so that each paragraph is on one line.
    Never,
}

impl fmt::Display for ProseWrap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProseWrap::Preserve => write!(f, "preserve"),
            ProseWrap::Always => write!(f, "always"),
            ProseWrap::Never => write!(f, "never"),
        }
    }
}

impl FromStr for ProseWrap {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "preserve" => Ok(Self::Preserve),
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            _ => Err("Unsupported value for this option"),
        }
    }
}

impl CstFormatContext for MarkdownFormatContext {
    type Language = MarkdownLanguage;
    type Style = MarkdownCommentStyle;
    type CommentRule = FormatMarkdownLeadingComment;

    fn comments(&self) -> &MarkdownComments {
        &self.comments
    }
}

impl FormatOptions for MdFormatOptions {
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

impl MdFormatOptions {
    pub fn new() -> Self {
        Self {
            indent_style: IndentStyle::default(),
            indent_width: IndentWidth::default(),
            line_ending: LineEnding::default(),
            line_width: LineWidth::default(),
            trailing_newline: TrailingNewline::default(),
            prose_wrap: ProseWrap::default(),
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

    pub fn with_prose_wrap(mut self, prose_wrap: ProseWrap) -> Self {
        self.prose_wrap = prose_wrap;
        self
    }

    pub fn prose_wrap(&self) -> ProseWrap {
        self.prose_wrap
    }
}

impl MarkdownFormatContext {
    pub fn new(options: MdFormatOptions) -> Self {
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
    type Options = MdFormatOptions;

    fn options(&self) -> &Self::Options {
        &self.options
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        None
    }
}

impl fmt::Display for MdFormatOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Indent style: {}", self.indent_style)?;
        writeln!(f, "Indent width: {}", self.indent_width.value())?;
        writeln!(f, "Line ending: {}", self.line_ending)?;
        writeln!(f, "Line width: {}", self.line_width.value())?;
        writeln!(f, "Trailing newline: {}", self.trailing_newline.value())?;
        writeln!(f, "Prose wrap: {}", self.prose_wrap)
    }
}
