use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{
    AttributePosition, BracketSameLine, BracketSpacing, Expand, IndentStyle, IndentWidth,
    LineEnding, LineWidth,
};
use clap::Args;
use serde::{Deserialize, Serialize};

pub type FormatterEnabled = Bool<true>;
pub type UseEditorconfigEnabled = Bool<false>;
pub type FormatWithErrorsEnabled = Bool<false>;

/// Generic options applied to all files
#[derive(
    Args, Clone, Deserializable, Debug, Default, Deserialize, Eq, PartialEq, Merge, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[group(skip)]
pub struct FormatterConfiguration {
    // if `false`, it disables the feature. `true` by default
    // #[arg(long = "formatter-enabled", value_name = "true|false")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<FormatterEnabled>,

    /// Whether formatting should be allowed to proceed if a given file
    /// has syntax errors
    // #[arg(long = "format-with-errors", value_name = "true|false")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_with_errors: Option<FormatWithErrorsEnabled>,

    /// The indent style.
    #[arg(long = "indent-style", value_name = "tab|space")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_style: Option<IndentStyle>,

    /// The size of the indentation, 2 by default
    #[arg(long = "indent-width", value_name = "NUMBER")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_width: Option<IndentWidth>,

    /// The type of line ending.
    #[arg(long = "line-ending", value_name = "lf|crlf|cr|auto")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line. Defaults to 80.
    #[arg(long = "line-width", value_name = "NUMBER")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,

    /// The attribute position style in HTML-ish languages. Defaults to auto.
    #[arg(long = "attribute-position", value_name = "multiline|auto")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_position: Option<AttributePosition>,

    /// Put the `>` of a multi-line HTML or JSX element at the end of the last line instead of being alone on the next line (does not apply to self closing elements).
    #[arg(long = "bracket-same-line", value_name = "true|false")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bracket_same_line: Option<BracketSameLine>,

    /// Whether to insert spaces around brackets in object literals. Defaults to true.
    #[arg(long = "bracket-spacing", value_name = "true|false")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bracket_spacing: Option<BracketSpacing>,

    /// Whether to expand arrays and objects on multiple lines.
    /// When set to `auto`, object literals are formatted on multiple lines if the first property has a newline,
    /// and array literals are formatted on a single line if it fits in the line.
    /// When set to `always`, these literals are formatted on multiple lines, regardless of length of the list.
    /// When set to `never`, these literals are formatted on a single line if it fits in the line.
    /// When formatting `package.json`, Biome will use `always` unless configured otherwise. Defaults to "auto".
    #[arg(long = "expand", value_name = "auto|always|never")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Expand>,

    /// Use any `.editorconfig` files to configure the formatter. Configuration
    /// in `biome.json` will override `.editorconfig` configuration.
    ///
    /// Default: `true`.
    #[arg(long = "use-editorconfig", value_name = "true|false")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_editorconfig: Option<UseEditorconfigEnabled>,

    /// A list of glob patterns. The formatter will include files/folders that will
    /// match these patterns.
    #[clap(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<biome_glob::NormalizedGlob>>,
}

impl FormatterConfiguration {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }

    pub fn format_with_errors_resolved(&self) -> bool {
        self.format_with_errors.unwrap_or_default().into()
    }

    pub fn indent_style_resolved(&self) -> IndentStyle {
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

    pub fn expand_resolved(&self) -> Expand {
        self.expand.unwrap_or_default()
    }

    pub fn use_editorconfig_resolved(&self) -> bool {
        self.use_editorconfig.unwrap_or_default().into()
    }
}
