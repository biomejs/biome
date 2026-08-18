use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{IndentStyle, IndentWidth, LineEnding, LineWidth, TrailingNewline};
use biome_markdown_formatter::context::ProseWrap;
#[cfg(feature = "cli")]
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Options applied to Markdown files
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Deserializable, Merge)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct MarkdownConfiguration {
    /// Parsing options
    #[cfg_attr(
        feature = "cli",
        bpaf(external(markdown_parser_configuration), optional, hide)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parser: Option<MarkdownParserConfiguration>,

    #[cfg_attr(
        feature = "cli",
        bpaf(external(markdown_formatter_configuration), optional, hide)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<MarkdownFormatterConfiguration>,

    #[cfg_attr(
        feature = "cli",
        bpaf(external(markdown_linter_configuration), optional, hide)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linter: Option<MarkdownLinterConfiguration>,
}

pub type MarkdownFormatterEnabled = Bool<false>; // Keep it disabled by default while experimental.
pub type MarkdownLinterEnabled = Bool<true>;
pub type MarkdownAssistEnabled = Bool<true>;
pub type MarkdownParseInterpolation = Bool<false>;
pub type MarkdownParseFrontmatter = Bool<false>;

/// Options that change how the Markdown parser behaves
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Deserializable, Merge)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct MarkdownParserConfiguration {
    /// Enables parsing frontmatter at the start of the file. Defaults to `false`.
    #[cfg_attr(all(feature = "cli", feature = "lang_md"), bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frontmatter: Option<MarkdownParseFrontmatter>,
}

/// Options that change how the Markdown formatter behaves
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Deserializable, Merge)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct MarkdownFormatterConfiguration {
    /// Control the formatter for Markdown (and its super languages) files.
    #[cfg_attr(all(feature = "cli", feature = "lang_md"), bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<MarkdownFormatterEnabled>,

    /// The indent style applied to Markdown files.
    #[cfg_attr(all(feature = "cli", feature = "lang_md"), bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_style: Option<IndentStyle>,

    /// The size of the indentation applied to Markdown files. Defaults to 2.
    #[cfg_attr(all(feature = "cli", feature = "lang_md"), bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_width: Option<IndentWidth>,

    /// What's the max width of a line applied to Markdown files. Defaults to 80.
    #[cfg_attr(all(feature = "cli", feature = "lang_md"), bpaf(hide))]
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
    #[cfg_attr(all(feature = "cli", feature = "lang_md"), bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_newline: Option<TrailingNewline>,

    /// The type of line ending applied to Markdown (and its super languages) files. `auto` uses CRLF on Windows and LF on other platforms.
    #[cfg_attr(all(feature = "cli", feature = "lang_md"), bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<LineEnding>,

    /// Controls whether Biome keeps, adds, or removes line breaks in Markdown paragraphs.
    ///
    /// Manual line breaks are always kept. In Markdown, a manual line break is created by ending a
    /// line with two spaces or a backslash.
    #[cfg_attr(all(feature = "cli", feature = "lang_md"), bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prose_wrap: Option<ProseWrap>,
}

/// Options that change how the Markdown linter behaves
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Deserializable, Merge)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct MarkdownLinterConfiguration {
    /// Control the linter for Markdown files.
    #[cfg_attr(all(feature = "cli", feature = "lang_md"), bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<MarkdownLinterEnabled>,
}
