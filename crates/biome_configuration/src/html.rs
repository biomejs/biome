use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{
    AttributePosition, BracketSameLine, IndentStyle, IndentWidth, LineEnding, LineWidth,
};
use biome_html_formatter::context::{
    IndentScriptAndStyle, SelfCloseVoidElements, WhitespaceSensitivity,
};
use clap::Args;
use serde::{Deserialize, Serialize};

pub type ExperimentalFullSupportEnabled = Bool<false>;

/// Options applied to HTML files
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Args, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct HtmlConfiguration {
    /// Enables full support for HTML, Vue, Svelte and Astro files.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[arg(
        long = "html-experimental-full-support-enabled",
        value_name = "true|false"
    )]
    pub experimental_full_support_enabled: Option<ExperimentalFullSupportEnabled>,

    /// HTML parsing options
    #[clap(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parser: Option<HtmlParserConfiguration>,

    /// HTML formatter options
    #[clap(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<HtmlFormatterConfiguration>,

    /// HTML linter options
    #[clap(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linter: Option<HtmlLinterConfiguration>,

    #[clap(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assist: Option<HtmlAssistConfiguration>,
}

pub type HtmlFormatterEnabled = Bool<false>; // Keep it disabled by default while experimental.
pub type HtmlLinterEnabled = Bool<true>;
pub type HtmlAssistEnabled = Bool<true>;
pub type HtmlParseInterpolation = Bool<false>;

/// Options that changes how the HTML parser behaves
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Args, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HtmlParserConfiguration {
    /// Enables the parsing of double text expressions such as `{{ expression }}` inside `.html` files
    #[arg(long = "html-parse-interpolation", value_name = "true|false")]
    pub interpolation: Option<HtmlParseInterpolation>,
}

/// Options that changes how the HTML formatter behaves
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Args, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[group(skip)]
pub struct HtmlFormatterConfiguration {
    /// Control the formatter for HTML (and its super languages) files.
    #[arg(long = "html-formatter-enabled", value_name = "true|false")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<HtmlFormatterEnabled>,

    /// The indent style applied to HTML (and its super languages) files.
    #[arg(long = "html-formatter-indent-style", value_name = "tab|space")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_style: Option<IndentStyle>,

    /// The size of the indentation applied to HTML (and its super languages) files. Default to 2.
    #[arg(long = "html-formatter-indent-width", value_name = "NUMBER")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_width: Option<IndentWidth>,

    /// The type of line ending applied to HTML (and its super languages) files. `auto` uses CRLF on Windows and LF on other platforms.
    #[arg(long = "html-formatter-line-ending", value_name = "lf|crlf|cr|auto")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to HTML (and its super languages) files. Defaults to 80.
    #[arg(long = "html-formatter-line-width", value_name = "NUMBER")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,

    /// The attribute position style in HTML elements. Defaults to auto.
    #[arg(
        long = "html-formatter-attribute-position",
        value_name = "multiline|auto"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_position: Option<AttributePosition>,

    /// Whether to hug the closing bracket of multiline HTML tags to the end of the last line, rather than being alone on the following line. Defaults to false.
    #[arg(long = "html-formatter-bracket-same-line", value_name = "true|false")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bracket_same_line: Option<BracketSameLine>,

    /// Whether to account for whitespace sensitivity when formatting HTML (and its super languages). Defaults to "css".
    #[arg(
        long = "html-formatter-whitespace-sensitivity",
        value_name = "css|strict|ignore"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitespace_sensitivity: Option<WhitespaceSensitivity>,

    /// Whether to indent the `<script>` and `<style>` tags for HTML (and its super languages). Defaults to false.
    #[arg(
        long = "html-formatter-indent-script-and-style",
        value_name = "true|false"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_script_and_style: Option<IndentScriptAndStyle>,

    /// Whether void elements should be self-closed. Defaults to never.
    #[arg(
        long = "html-formatter-self-close-void-elements",
        value_name = "always|never"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_close_void_elements: Option<SelfCloseVoidElements>,
}

/// Options that changes how the HTML linter behaves
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Args, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[group(required = false, multiple = false)]
pub struct HtmlLinterConfiguration {
    /// Control the linter for HTML (and its super languages) files.
    #[arg(long = "html-linter-enabled", value_name = "true|false")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<HtmlLinterEnabled>,
}

/// Options that changes how the HTML assist behaves
#[derive(
    Args, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct HtmlAssistConfiguration {
    /// Control the assist for HTML (and its super languages) files.
    #[arg(long = "html-assist-enabled", value_name = "true|false")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<HtmlAssistEnabled>,
}
