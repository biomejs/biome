use crate::JsonCommentStyle;
use crate::comments::{FormatJsonLeadingComment, JsonComments};
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::separated::TrailingSeparator;
use biome_formatter::{BracketSpacing, Expand, IndentWidth, prelude::*};
use biome_formatter::{
    CstFormatContext, FormatContext, FormatOptions, IndentStyle, LineEnding, LineWidth,
    TrailingNewline, TransformSourceMap,
};
use biome_json_syntax::{JsonFileSource, JsonLanguage};
use std::default::Default;
use std::fmt;
use std::rc::Rc;
use std::str::FromStr;

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
    line_ending: LineEnding,
    line_width: LineWidth,
    /// Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "none".
    trailing_commas: TrailingCommas,
    expand: Expand,
    bracket_spacing: BracketSpacing,
    /// Whether to add a trailing newline at the end of the file. Defaults to true.
    trailing_newline: TrailingNewline,
    /// The kind of file
    _file_source: JsonFileSource,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Deserializable, Merge, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub enum TrailingCommas {
    #[default]
    /// The formatter will remove the trailing commas.
    None,
    /// The trailing commas are allowed and advised in JSON and JSONC files.
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
        std::borrow::Cow::Borrowed("JsonTrailingCommas")
    }

    fn json_schema(_generator: &mut schemars::generate::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string",
            "enum": ["none", "all"],
            "description": "Print trailing commas wherever possible in multi-line comma-separated syntactic structures for JSON files."
        })
    }
}

impl JsonFormatOptions {
    pub fn new(file_source: JsonFileSource) -> Self {
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

    pub fn with_trailing_commas(mut self, trailing_commas: TrailingCommas) -> Self {
        self.trailing_commas = trailing_commas;
        self
    }

    pub fn with_expand(mut self, expand: Expand) -> Self {
        self.expand = expand;
        self
    }

    pub fn with_bracket_spacing(mut self, bracket_spacing: BracketSpacing) -> Self {
        self.bracket_spacing = bracket_spacing;
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

    pub fn set_trailing_commas(&mut self, trailing_commas: TrailingCommas) {
        self.trailing_commas = trailing_commas;
    }

    pub fn set_bracket_spacing(&mut self, bracket_spacing: BracketSpacing) {
        self.bracket_spacing = bracket_spacing;
    }

    /// Set `expand_lists`
    pub fn set_expand(&mut self, expand: Expand) {
        self.expand = expand;
    }

    pub fn set_trailing_newline(&mut self, trailing_newline: TrailingNewline) {
        self.trailing_newline = trailing_newline;
    }

    pub fn bracket_spacing(&self) -> BracketSpacing {
        self.bracket_spacing
    }

    pub fn expand(&self) -> Expand {
        self.expand
    }

    pub fn trailing_newline(&self) -> TrailingNewline {
        self.trailing_newline
    }

    pub(crate) fn to_trailing_separator(&self) -> TrailingSeparator {
        match self.trailing_commas {
            TrailingCommas::None => TrailingSeparator::Omit,
            TrailingCommas::All => TrailingSeparator::Allowed,
        }
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

impl fmt::Display for JsonFormatOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Indent style: {}", self.indent_style)?;
        writeln!(f, "Indent width: {}", self.indent_width.value())?;
        writeln!(f, "Line ending: {}", self.line_ending)?;
        writeln!(f, "Line width: {}", self.line_width.value())?;
        writeln!(f, "Trailing commas: {}", self.trailing_commas)?;
        writeln!(f, "Expand: {}", self.expand)?;
        writeln!(f, "Bracket spacing: {}", self.bracket_spacing.value())?;
        writeln!(f, "Trailing newline: {}", self.trailing_newline.value())
    }
}
