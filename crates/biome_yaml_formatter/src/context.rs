use crate::YamlCommentStyle;
use crate::comments::{FormatYamlLeadingComment, YamlComments};
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::rowan::RowanSource;
use biome_formatter::separated::TrailingSeparator;
use biome_formatter::{
    BracketSpacing, CommentSpacing, CstFormatContext, FormatContext, FormatOptions, IndentStyle,
    LineEnding, LineWidth, QuoteStyle, TrailingNewline, TransformSourceMap,
};
use biome_formatter::{IndentWidth, rowan::prelude::*};
use biome_yaml_syntax::YamlLanguage;
use std::default::Default;
use std::fmt;
use std::rc::Rc;
use std::str::FromStr;

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
    type Source = RowanSource;

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

    fn comment_spacing(&self) -> CommentSpacing {
        self.options.comment_spacing
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct YamlFormatOptions {
    indent_width: IndentWidth,
    line_ending: LineEnding,
    line_width: LineWidth,
    /// Whether to add a trailing newline at the end of the file. Defaults to true.
    trailing_newline: TrailingNewline,
    /// The preferred quote style for quoted scalars that work with either
    /// kind of quote. Defaults to double quotes.
    quote_style: QuoteStyle,
    /// Whether to insert spaces inside non-empty flow mappings and sequences
    /// that fit on one line. When unset, mappings have spaces and sequences do not.
    bracket_spacing: Option<BracketSpacing>,
    /// The exact number of spaces between content and a comment on the same line. Defaults to 2.
    comment_spacing: CommentSpacing,
    /// Whether multiline flow mappings and sequences end with a comma. Defaults to all.
    trailing_commas: TrailingCommas,
}

impl YamlFormatOptions {
    pub fn new() -> Self {
        Self::default()
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

    pub fn with_quote_style(mut self, quote_style: QuoteStyle) -> Self {
        self.quote_style = quote_style;
        self
    }

    pub fn set_quote_style(&mut self, quote_style: QuoteStyle) {
        self.quote_style = quote_style;
    }

    pub fn quote_style(&self) -> QuoteStyle {
        self.quote_style
    }

    pub fn with_bracket_spacing(mut self, bracket_spacing: BracketSpacing) -> Self {
        self.bracket_spacing = Some(bracket_spacing);
        self
    }

    pub fn set_bracket_spacing(&mut self, bracket_spacing: BracketSpacing) {
        self.bracket_spacing = Some(bracket_spacing);
    }

    pub fn bracket_spacing(&self) -> Option<BracketSpacing> {
        self.bracket_spacing
    }

    pub fn with_comment_spacing(mut self, comment_spacing: CommentSpacing) -> Self {
        self.comment_spacing = comment_spacing;
        self
    }

    pub fn set_comment_spacing(&mut self, comment_spacing: CommentSpacing) {
        self.comment_spacing = comment_spacing;
    }

    pub fn comment_spacing(&self) -> CommentSpacing {
        self.comment_spacing
    }

    pub fn with_trailing_commas(mut self, trailing_commas: TrailingCommas) -> Self {
        self.trailing_commas = trailing_commas;
        self
    }

    pub fn set_trailing_commas(&mut self, trailing_commas: TrailingCommas) {
        self.trailing_commas = trailing_commas;
    }

    pub fn trailing_commas(&self) -> TrailingCommas {
        self.trailing_commas
    }

    pub(crate) fn to_trailing_separator(&self) -> TrailingSeparator {
        match self.trailing_commas {
            TrailingCommas::None => TrailingSeparator::Omit,
            TrailingCommas::All => TrailingSeparator::Allowed,
        }
    }
}

impl FormatOptions for YamlFormatOptions {
    fn indent_style(&self) -> IndentStyle {
        IndentStyle::Space
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
        writeln!(f, "Indent width: {}", self.indent_width.value())?;
        writeln!(f, "Line ending: {}", self.line_ending)?;
        writeln!(f, "Line width: {}", self.line_width.value())?;
        writeln!(f, "Trailing newline: {}", self.trailing_newline.value())?;
        writeln!(f, "Quote style: {}", self.quote_style)?;
        if let Some(bracket_spacing) = self.bracket_spacing {
            writeln!(f, "Bracket spacing: {}", bracket_spacing.value())?;
        } else {
            writeln!(f, "Bracket spacing: unset")?;
        }
        writeln!(f, "Comment spacing: {}", self.comment_spacing)?;
        writeln!(f, "Trailing commas: {}", self.trailing_commas)?;

        Ok(())
    }
}

/// Controls whether multiline flow mappings (`{ ... }`) and sequences (`[ ... ]`) end with a
/// comma after their last entry.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Deserializable, Merge, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub enum TrailingCommas {
    /// Removes the comma after the last entry.
    None,
    /// Adds a comma after the last entry when the collection spans multiple lines. This is the
    /// default.
    #[default]
    All,
}

impl FromStr for TrailingCommas {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "all" => Ok(Self::All),
            _ => Err("Value not supported for TrailingCommas"),
        }
    }
}

impl fmt::Display for TrailingCommas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => std::write!(f, "None"),
            Self::All => std::write!(f, "All"),
        }
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for TrailingCommas {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("YamlTrailingCommas")
    }

    fn json_schema(_generator: &mut schemars::generate::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string",
            "enum": ["none", "all"],
            "description": "Controls whether multiline flow mappings (`{ ... }`) and sequences (`[ ... ]`) end with a comma after their last entry."
        })
    }
}
