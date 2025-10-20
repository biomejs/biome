use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{IndentStyle, IndentWidth, LineEnding, LineWidth};
use clap::Args;
use serde::{Deserialize, Serialize};

/// Options applied to GritQL files
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Args, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct GritConfiguration {
    /// Formatting options
    #[clap(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<GritFormatterConfiguration>,

    /// Formatting options
    #[clap(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linter: Option<GritLinterConfiguration>,

    /// Assist options
    #[clap(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assist: Option<GritAssistConfiguration>,
}

pub type GritFormatterEnabled = Bool<true>;

#[derive(
    Args, Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[group(required = false, multiple = false)]
pub struct GritFormatterConfiguration {
    /// Control the formatter for Grit files.
    #[arg(long = "grit-formatter-enabled", value_name = "true|false")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<GritFormatterEnabled>,

    /// The indent style applied to Grit files.
    #[arg(long = "grit-formatter-indent-style", value_name = "tab|space")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_style: Option<IndentStyle>,

    /// The size of the indentation applied to Grit files. Default to 2.
    #[arg(long = "grit-formatter-indent-width", value_name = "NUMBER")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_width: Option<IndentWidth>,

    /// The type of line ending applied to Grit files.
    #[arg(long = "grit-formatter-line-ending", value_name = "lf|crlf|cr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to Grit files. Defaults to 80.
    #[arg(long = "grit-formatter-line-width", value_name = "NUMBER")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,
}

pub type GritLinterEnabled = Bool<true>;

#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Args, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct GritLinterConfiguration {
    /// Control the linter for Grit files.
    #[arg(long = "grit-linter-enabled", value_name = "true|false")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<GritLinterEnabled>,
}

pub type GritAssistEnabled = Bool<true>;
#[derive(
    Args, Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct GritAssistConfiguration {
    /// Control the assist functionality for Grit files.
    #[arg(long = "grit-assist-enabled", value_name = "true|false")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<GritAssistEnabled>,
}
