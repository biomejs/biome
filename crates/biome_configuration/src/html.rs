use biome_deserialize_macros::{Deserializable, Merge, Partial};
use biome_formatter::{
    AttributePosition, BracketSameLine, IndentStyle, IndentWidth, LineEnding, LineWidth,
};
use biome_html_formatter::context::{IndentScriptAndStyle, WhitespaceSensitivity};
use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{IndentStyle, IndentWidth, LineEnding, LineWidth};
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
    #[partial(type, bpaf(external(partial_html_parser), optional))]
    pub parser: HtmlParser,

    /// HTML formatter options
    #[partial(type, bpaf(external(partial_html_formatter), optional))]
    pub formatter: HtmlFormatter,
}

pub type HtmlFormatterEnabled = Bool<true>;
pub type HtmlLinterEnabled = Bool<true>;
pub type HtmlAssistEnabled = Bool<true>;

/// Options that changes how the HTML parser behaves
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct HtmlParser;

/// Options that changes how the HTML formatter behaves
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct HtmlFormatter {
    /// Control the formatter for HTML (and its super languages) files.
    #[partial(bpaf(long("html-formatter-enabled"), argument("true|false"), optional))]
    pub enabled: bool,

    /// The indent style applied to HTML (and its super languages) files.
    #[partial(bpaf(long("html-formatter-indent-style"), argument("tab|space"), optional))]
    pub indent_style: Option<IndentStyle>,

    /// The size of the indentation applied to HTML (and its super languages) files. Default to 2.
    #[partial(bpaf(long("html-formatter-indent-width"), argument("NUMBER"), optional))]
    pub indent_width: Option<IndentWidth>,

    /// The type of line ending applied to HTML (and its super languages) files.
    #[partial(bpaf(long("html-formatter-line-ending"), argument("lf|crlf|cr"), optional))]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to HTML (and its super languages) files. Defaults to 80.
    #[partial(bpaf(long("html-formatter-line-width"), argument("NUMBER"), optional))]
    pub line_width: Option<LineWidth>,

    /// The attribute position style in HTML elements. Defaults to auto.
    #[partial(bpaf(
        long("html-formatter-attribute-position"),
        argument("multiline|auto"),
        optional
    ))]
    pub attribute_position: Option<AttributePosition>,

    /// Whether to hug the closing bracket of multiline HTMLtags to the end of the last line, rather than being alone on the following line. Defaults to false.
    #[partial(bpaf(
        long("html-formatter-bracket-same-line"),
        argument("true|false"),
        optional
    ))]
    pub bracket_same_line: Option<BracketSameLine>,

    /// Whether or not to account for whitespace sensitivity when formatting HTML (and its super languages). Defaults to "strict".
    #[partial(bpaf(
        long("html-formatter-whitespace-sensitivity"),
        argument("strict|ignore"),
        optional
    ))]
    pub whitespace_sensitivity: Option<WhitespaceSensitivity>,

    /// Whether or not to indent the `<script>` and `<style>` tags for HTML (and its super languages). Defaults to false.
    #[partial(bpaf(
        long("html-formatter-indent-script-and-style"),
        argument("true|false"),
        optional
    ))]
    pub indent_script_and_style: Option<IndentScriptAndStyle>,
}

// impl PartialHtmlFormatter {
//     pub fn get_formatter_configuration(&self) -> HtmlFormatter {
//         HtmlFormatter {
//             enabled: self.enabled.unwrap_or_default(),
//             indent_style: self.indent_style,
//             indent_width: self.indent_width,
//             line_ending: self.line_ending,
//             line_width: self.line_width,
//             attribute_position: self.attribute_position,
//             bracket_same_line: self.bracket_same_line,
//             whitespace_sensitivity: self.whitespace_sensitivity,
//             indent_script_and_style: self.indent_script_and_style,
//         }
//     }
// }
