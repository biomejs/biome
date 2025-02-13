use crate::comments::{FormatJsonLeadingComment, JsonComments};
use crate::JsonCommentStyle;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::separated::TrailingSeparator;
use biome_formatter::{prelude::*, BracketSpacing, IndentWidth, ObjectWrap};
use biome_formatter::{
    CstFormatContext, FormatContext, FormatOptions, IndentStyle, LineEnding, LineWidth,
    TransformSourceMap,
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
    object_wrap: ObjectWrap,
    /// The kind of file
    file_source: JsonFileSource,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Deserializable, Merge, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum TrailingCommas {
    #[default]
    /// The formatter will remove the trailing commas.
    None,
    /// The trailing commas are allowed and advised only in JSONC files. Trailing commas are removed from JSON files.
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
            TrailingCommas::None => std::write!(f, "None"),
            TrailingCommas::All => std::write!(f, "All"),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Deserializable, Merge, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum Expand {
    Always,
    #[default]
    FollowSource,
}

impl FromStr for Expand {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "always" => Ok(Self::Always),
            "follow-source" => Ok(Self::FollowSource),
            _ => Err(std::format!("unknown expand literal: {}", s)),
        }
    }
}

impl fmt::Display for Expand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expand::Always => std::write!(f, "Always"),
            Expand::FollowSource => std::write!(f, "Follow Source"),
        }
    }
}

impl JsonFormatOptions {
    pub fn new(file_source: JsonFileSource) -> Self {
        Self {
            file_source,
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

    pub fn with_object_wrap(mut self, object_wrap: ObjectWrap) -> Self {
        self.object_wrap = object_wrap;
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

    pub fn set_object_wrap(&mut self, object_wrap: ObjectWrap) {
        self.object_wrap = object_wrap;
    }

    pub fn bracket_spacing(&self) -> BracketSpacing {
        self.bracket_spacing
    }

    pub fn object_wrap(&self) -> ObjectWrap {
        self.object_wrap
    }

    pub(crate) fn to_trailing_separator(&self) -> TrailingSeparator {
        match self.trailing_commas {
            TrailingCommas::None => TrailingSeparator::Omit,
            TrailingCommas::All => TrailingSeparator::Allowed,
        }
    }

    pub(crate) fn file_source(&self) -> &JsonFileSource {
        &self.file_source
    }

    pub(crate) const fn expand(&self) -> bool {
        matches!(self.expand, Expand::Always)
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
        writeln!(f, "Object wrap: {}", self.object_wrap)?;

        Ok(())
    }
}
