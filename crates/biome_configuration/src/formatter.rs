use crate::bool::Bool;
use biome_deserialize::StringSet;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{
    AttributePosition, BracketSpacing, IndentStyle, IndentWidth, LineEnding, LineWidth,
};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub type FormatterEnabled = Bool<true>;
pub type UseEditorconfigEnabled = Bool<false>;
pub type FormatWithErrorsEnabled = Bool<false>;

/// Generic options applied to all files
#[derive(
    Bpaf, Clone, Deserializable, Debug, Default, Deserialize, Eq, PartialEq, Merge, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct FormatterConfiguration {
    // if `false`, it disables the feature. `true` by default
    #[bpaf(hide)]
    pub enabled: Option<FormatterEnabled>,

    /// Stores whether formatting should be allowed to proceed if a given file
    /// has syntax errors
    #[bpaf(hide)]
    pub format_with_errors: Option<FormatWithErrorsEnabled>,

    /// The indent style.
    #[bpaf(long("indent-style"), argument("tab|space"))]
    pub indent_style: Option<PlainIndentStyle>,

    /// The size of the indentation, 2 by default (deprecated, use `indent-width`)
    #[bpaf(long("indent-size"), argument("NUMBER"))]
    #[deserializable(deprecated(use_instead = "formatter.indentWidth"))]
    pub indent_size: Option<IndentWidth>,

    /// The size of the indentation, 2 by default
    #[bpaf(long("indent-width"), argument("NUMBER"))]
    pub indent_width: Option<IndentWidth>,

    /// The type of line ending.
    #[bpaf(long("line-ending"), argument("lf|crlf|cr"))]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line. Defaults to 80.
    #[bpaf(long("line-width"), argument("NUMBER"))]
    pub line_width: Option<LineWidth>,

    /// The attribute position style in HTMLish languages. By default auto.
    #[bpaf(long("attribute-position"), argument("multiline|auto"))]
    pub attribute_position: Option<AttributePosition>,

    /// Whether to insert spaces around brackets in object literals. Defaults to true.
    #[bpaf(long("bracket-spacing"), argument("true|false"))]
    pub bracket_spacing: Option<BracketSpacing>,

    /// Use any `.editorconfig` files to configure the formatter. Configuration in `biome.json` will override `.editorconfig` configuration. Default: false.
    #[bpaf(long("use-editorconfig"), argument("true|false"))]
    pub use_editorconfig: Option<UseEditorconfigEnabled>,

    /// A list of Unix shell style patterns. The formatter will ignore files/folders that will
    /// match these patterns.
    #[bpaf(hide)]
    pub ignore: Option<StringSet>,

    /// A list of Unix shell style patterns. The formatter will include files/folders that will
    /// match these patterns.
    #[bpaf(hide)]
    pub include: Option<StringSet>,
}

impl FormatterConfiguration {
    pub fn enabled_resolved(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }

    pub fn format_with_errors_resolved(&self) -> bool {
        self.format_with_errors.unwrap_or_default().into()
    }

    pub fn indent_style_resolved(&self) -> PlainIndentStyle {
        self.indent_style.unwrap_or_default()
    }

    pub fn indent_width_resolved(&self) -> IndentWidth {
        self.indent_width.unwrap_or_default()
    }

    pub fn line_ending_resolved(&self) -> LineEnding {
        self.line_ending.unwrap_or_default()
    }

    pub fn line_width_resolved(&self) -> LineWidth {
        self.line_width.unwrap_or_default()
    }

    pub fn attribute_position_resolved(&self) -> AttributePosition {
        self.attribute_position.unwrap_or_default()
    }

    pub fn bracket_spacing_resolved(&self) -> BracketSpacing {
        self.bracket_spacing.unwrap_or_default()
    }

    pub fn use_editorconfig_resolved(&self) -> bool {
        self.use_editorconfig.unwrap_or_default().into()
    }

    pub fn ignore_resolved(&self) -> StringSet {
        self.ignore.clone().unwrap_or_default()
    }

    pub fn include_resolved(&self) -> StringSet {
        self.include.clone().unwrap_or_default()
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum PlainIndentStyle {
    /// Tab
    #[default]
    Tab,
    /// Space
    Space,
}

impl FromStr for PlainIndentStyle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tab" => Ok(PlainIndentStyle::Tab),
            "space" => Ok(PlainIndentStyle::Space),
            _ => Err("Unsupported value for this option"),
        }
    }
}

impl From<PlainIndentStyle> for IndentStyle {
    fn from(value: PlainIndentStyle) -> Self {
        match value {
            PlainIndentStyle::Tab => IndentStyle::Tab,
            PlainIndentStyle::Space => IndentStyle::Space,
        }
    }
}
