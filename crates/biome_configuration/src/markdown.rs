use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{
    AttributePosition, BracketSameLine, IndentStyle, IndentWidth, LineEnding, LineWidth,
    TrailingNewline,
};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

pub type ExperimentalFullSupportEnabled = Bool<false>;

/// Options applied to Markdown files
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct MarkdownConfiguration {
    // /// Enables full support for Markdown, Vue, Svelte and Astro files.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // #[bpaf(hide)]
    // pub experimental_full_support_enabled: Option<ExperimentalFullSupportEnabled>,

    // /// Markdown parsing options
    // #[bpaf(hide, pure(Default::default()))]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub parser: Option<MarkdownParserConfiguration>,
    /// Markdown formatter options
    #[bpaf(external(markdown_formatter_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<MarkdownFormatterConfiguration>,
    // /// Markdown linter options
    // #[bpaf(external(markdown_linter_configuration), optional)]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub linter: Option<MarkdownLinterConfiguration>,

    // #[bpaf(external(markdown_assist_configuration), optional)]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub assist: Option<MarkdownAssistConfiguration>,
}

pub type MarkdownFormatterEnabled = Bool<false>; // Keep it disabled by default while experimental.
pub type MarkdownLinterEnabled = Bool<true>;
pub type MarkdownAssistEnabled = Bool<true>;
pub type MarkdownParseInterpolation = Bool<false>;

/// Options that changes how the HTML formatter behaves
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct MarkdownFormatterConfiguration {
    /// Control the formatter for HTML (and its super languages) files.
    #[bpaf(long("html-formatter-enabled"), argument("true|false"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<MarkdownFormatterEnabled>,

    /// The indent style applied to HTML (and its super languages) files.
    #[bpaf(long("html-formatter-indent-style"), argument("tab|space"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_style: Option<IndentStyle>,

    /// The size of the indentation applied to HTML (and its super languages) files. Default to 2.
    #[bpaf(long("html-formatter-indent-width"), argument("NUMBER"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_width: Option<IndentWidth>,

    /// The type of line ending applied to HTML (and its super languages) files. `auto` uses CRLF on Windows and LF on other platforms.
    #[bpaf(
        long("html-formatter-line-ending"),
        argument("lf|crlf|cr|auto"),
        optional
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to HTML (and its super languages) files. Defaults to 80.
    #[bpaf(long("html-formatter-line-width"), argument("NUMBER"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,

    /// The attribute position style in HTML elements. Defaults to auto.
    #[bpaf(
        long("html-formatter-attribute-position"),
        argument("multiline|auto"),
        optional
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_position: Option<AttributePosition>,

    /// Whether to hug the closing bracket of multiline HTML tags to the end of the last line, rather than being alone on the following line. Defaults to false.
    #[bpaf(
        long("html-formatter-bracket-same-line"),
        argument("true|false"),
        optional
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bracket_same_line: Option<BracketSameLine>,

    // /// Whether to account for whitespace sensitivity when formatting HTML (and its super languages). Defaults to "css".
    // #[bpaf(
    //     long("html-formatter-whitespace-sensitivity"),
    //     argument("css|strict|ignore"),
    //     optional
    // )]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub whitespace_sensitivity: Option<WhitespaceSensitivity>,

    // /// Whether to indent the `<script>` and `<style>` tags for HTML (and its super languages). Defaults to false.
    // #[bpaf(
    //     long("html-formatter-indent-script-and-style"),
    //     argument("true|false"),
    //     optional
    // )]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub indent_script_and_style: Option<IndentScriptAndStyle>,

    // /// Whether void elements should be self-closed. Defaults to never.
    // #[bpaf(
    //     long("html-formatter-self-close-void-elements"),
    //     argument("always|never"),
    //     optional
    // )]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub self_close_void_elements: Option<SelfCloseVoidElements>,
    /// Whether to add a trailing newline at the end of the file.
    ///
    /// Setting this option to `false` is **highly discouraged** because it could cause many problems with other tools:
    /// - <https://thoughtbot.com/blog/no-newline-at-end-of-file>
    /// - <https://callmeryan.medium.com/no-newline-at-end-of-file-navigating-gits-warning-for-android-developers-af14e73dd804>
    /// - <https://unix.stackexchange.com/questions/345548/how-to-cat-files-together-adding-missing-newlines-at-end-of-some-files>
    ///
    /// Disable the option at your own risk.
    ///
    /// Defaults to true.
    #[bpaf(
        long("html-formatter-trailing-newline"),
        argument("true|false"),
        optional
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_newline: Option<TrailingNewline>,
}
