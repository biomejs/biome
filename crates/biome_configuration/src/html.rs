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
    /// Formatting options
    #[bpaf(external(html_formatter_configuration), optional)]
    pub formatter: Option<HtmlFormatterConfiguration>,
}

pub type HtmlFormatterEnabled = Bool<true>;
pub type HtmlLinterEnabled = Bool<true>;
pub type HtmlAssistEnabled = Bool<true>;

/// HTML format options
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct HtmlFormatterConfiguration {
    /// Control the formatter for HTML files.
    #[bpaf(long("html-formatter-enabled"), argument("true|false"), optional)]
    pub enabled: Option<HtmlFormatterEnabled>,

    /// The indent style applied to HTML files.
    #[bpaf(long("html-formatter-indent-style"), argument("tab|space"), optional)]
    pub indent_style: Option<IndentStyle>,

    /// The size of the indentation applied to HTML files. Default to 2.
    #[bpaf(long("html-formatter-indent-width"), argument("NUMBER"), optional)]
    pub indent_width: Option<IndentWidth>,

    /// The type of line ending applied to HTML files.
    #[bpaf(long("html-formatter-line-ending"), argument("lf|crlf|cr"), optional)]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to HTML files. Defaults to 80.
    #[bpaf(long("html-formatter-line-width"), argument("NUMBER"), optional)]
    pub line_width: Option<LineWidth>,
}
