use crate::comments::{FormatJsonLeadingComment, JsonComments};
use crate::JsonCommentStyle;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::separated::TrailingSeparator;
use biome_formatter::{prelude::*, AttributePosition, IndentWidth};
use biome_formatter::{
    CstFormatContext, FormatContext, FormatOptions, IndentStyle, LineEnding, LineWidth,
    TransformSourceMap,
};
use biome_json_syntax::JsonLanguage;
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
    attribute_position: AttributePosition,
    /// Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "none".
    trailing_commas: TrailingCommas,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Deserializable, Merge, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema),
    serde(rename_all = "camelCase")
)]
pub enum TrailingCommas {
    #[default]
    /// The formatter will remove the trailing commas
    None,
    /// The trailing commas are allowed and advised
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrailingCommas::None => std::write!(f, "None"),
            TrailingCommas::All => std::write!(f, "All"),
        }
    }
}

impl JsonFormatOptions {
    pub fn new() -> Self {
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

    pub fn with_trailing_commas(mut self, trailing_commas: TrailingCommas) -> Self {
        self.trailing_commas = trailing_commas;
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

    fn line_ending(&self) -> LineEnding {
        self.line_ending
    }

    fn line_width(&self) -> LineWidth {
        self.line_width
    }

    fn as_print_options(&self) -> PrinterOptions {
        PrinterOptions::from(self)
    }

    fn attribute_position(&self) -> AttributePosition {
        self.attribute_position
    }
}

impl fmt::Display for JsonFormatOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Indent style: {}", self.indent_style)?;
        writeln!(f, "Indent width: {}", self.indent_width.value())?;
        writeln!(f, "Line ending: {}", self.line_ending)?;
        writeln!(f, "Line width: {}", self.line_width.get())?;
        writeln!(f, "Trailing commas: {}", self.trailing_commas)
    }
}
