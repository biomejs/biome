use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{
    AttributePosition, BracketSameLine, IndentStyle, IndentWidth, LineEnding, LineWidth,
};
use biome_html_formatter::context::{IndentScriptAndStyle, WhitespaceSensitivity};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Options applied to HTML files
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct HtmlConfiguration {
    /// HTML parsing options
    #[bpaf(hide, pure(Default::default()))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parser: Option<HtmlParserConfiguration>,

    /// HTML formatter options
    #[bpaf(external(html_formatter_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<HtmlFormatterConfiguration>,
}

pub type HtmlFormatterEnabled = Bool<true>;
pub type HtmlLinterEnabled = Bool<false>;
pub type HtmlAssistEnabled = Bool<false>;

/// Options that changes how the HTML parser behaves
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HtmlParserConfiguration;

/// Options that changes how the HTML formatter behaves
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct HtmlFormatterConfiguration {
    /// Control the formatter for HTML (and its super languages) files.
    #[bpaf(long("html-formatter-enabled"), argument("true|false"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<HtmlFormatterEnabled>,

    /// The indent style applied to HTML (and its super languages) files.
    #[bpaf(long("html-formatter-indent-style"), argument("tab|space"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_style: Option<IndentStyle>,

    /// The size of the indentation applied to HTML (and its super languages) files. Default to 2.
    #[bpaf(long("html-formatter-indent-width"), argument("NUMBER"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_width: Option<IndentWidth>,

    /// The type of line ending applied to HTML (and its super languages) files.
    #[bpaf(long("html-formatter-line-ending"), argument("lf|crlf|cr"), optional)]
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

    /// Whether to account for whitespace sensitivity when formatting HTML (and its super languages). Defaults to "css".
    #[bpaf(
        long("html-formatter-whitespace-sensitivity"),
        argument("css|strict|ignore"),
        optional
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitespace_sensitivity: Option<WhitespaceSensitivity>,

    /// Whether to indent the `<script>` and `<style>` tags for HTML (and its super languages). Defaults to false.
    #[bpaf(
        long("html-formatter-indent-script-and-style"),
        argument("true|false"),
        optional
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_script_and_style: Option<IndentScriptAndStyle>,
}
