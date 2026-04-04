use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{IndentStyle, IndentWidth, LineEnding, LineWidth, TrailingNewline};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Options applied to Markdown files
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct MarkdownConfiguration {
    #[bpaf(external(markdown_formatter_configuration), optional, hide)]
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
    #[cfg_attr(feature = "markdown", bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<MarkdownFormatterEnabled>,

    /// The indent style applied to Markdown files.
    #[cfg_attr(feature = "markdown", bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_style: Option<IndentStyle>,

    /// The size of the indentation applied to Markdown files. Defaults to 2.
    #[cfg_attr(feature = "markdown", bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_width: Option<IndentWidth>,

    /// What's the max width of a line applied to Markdown files. Defaults to 80.
    #[cfg_attr(feature = "markdown", bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,

    /// Whether to add a trailing newline at the end of the file.
    ///
    /// Setting this option to `false` is **highly discouraged** because it could cause many problems with other tools:
    /// - https://thoughtbot.com/blog/no-newline-at-end-of-file
    /// - https://callmeryan.medium.com/no-newline-at-end-of-file-navigating-gits-warning-for-android-developers-af14e73dd804
    /// - https://unix.stackexchange.com/questions/345548/how-to-cat-files-together-adding-missing-newlines-at-end-of-some-files
    ///
    /// Disable the option at your own risk.
    ///
    /// Defaults to true.
    #[cfg_attr(feature = "markdown", bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_newline: Option<TrailingNewline>,

    /// The type of line ending applied to Markdown (and its super languages) files. `auto` uses CRLF on Windows and LF on other platforms.
    #[cfg_attr(feature = "markdown", bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<LineEnding>,
}
