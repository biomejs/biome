use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{IndentStyle, IndentWidth, LineWidth};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Options applied to Markdown files
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct MarkdownConfiguration {
    #[bpaf(external(markdown_formatter_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<MarkdownFormatterConfiguration>,
}

pub type MarkdownFormatterEnabled = Bool<false>; // Keep it disabled by default while experimental.
pub type MarkdownLinterEnabled = Bool<true>;
pub type MarkdownAssistEnabled = Bool<true>;
pub type MarkdownParseInterpolation = Bool<false>;

/// Options that change how the Markdown formatter behaves
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct MarkdownFormatterConfiguration {
    /// Control the formatter for Markdown (and its super languages) files.
    #[bpaf(long("markdown-formatter-enabled"), argument("true|false"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<MarkdownFormatterEnabled>,

    /// The indent style applied to Markdown files.
    #[bpaf(
        long("markdown-formatter-indent-style"),
        argument("tab|space"),
        optional
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_style: Option<IndentStyle>,

    /// The size of the indentation applied to Markdown files. Defaults to 2.
    #[bpaf(long("markdown-formatter-indent-width"), argument("NUMBER"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_width: Option<IndentWidth>,

    /// What's the max width of a line applied to Markdown files. Defaults to 80.
    #[bpaf(long("markdown-formatter-line-width"), argument("NUMBER"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,
}
