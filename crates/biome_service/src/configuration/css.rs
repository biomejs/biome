use crate::configuration::{deserialize_line_width, serialize_line_width, PlainIndentStyle};
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{LineEnding, LineWidth, QuoteStyle};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Options applied to CSS files
#[derive(
    Bpaf, Clone, Default, Debug, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default, deny_unknown_fields)]
pub struct CssConfiguration {
    /// Parsing options
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(css_parser), optional)]
    pub parser: Option<CssParser>,

    /// Formatting options
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(css_formatter), optional)]
    pub formatter: Option<CssFormatter>,
}

/// Options that changes how the CSS parser behaves
#[derive(
    Bpaf, Clone, Default, Debug, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CssParser {
    #[bpaf(hide)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Allow comments to appear on incorrect lines in `.css` files
    pub allow_wrong_line_comments: Option<bool>,
}

#[derive(
    Bpaf, Clone, Default, Debug, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CssFormatter {
    /// Control the formatter for CSS (and its super languages) files.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("css-formatter-enabled"), argument("true|false"), optional)]
    pub enabled: Option<bool>,

    /// The indent style applied to CSS (and its super languages) files.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("css-formatter-indent-style"), argument("tab|space"), optional)]
    pub indent_style: Option<PlainIndentStyle>,

    /// The size of the indentation applied to CSS (and its super languages) files. Default to 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("css-formatter-indent-width"), argument("NUMBER"), optional)]
    pub indent_width: Option<u8>,

    /// The size of the indentation applied to CSS (and its super languages) files. Default to 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("css-formatter-indent-size"), argument("NUMBER"), optional)]
    #[deprecated = "Please use indent_width instead"]
    #[deserializable(deprecated(use_instead = "css.formatter.indentWidth"))]
    pub indent_size: Option<u8>,

    /// The type of line ending applied to CSS (and its super languages) files.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("css-formatter-line-ending"), argument("lf|crlf|cr"), optional)]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to CSS (and its super languages) files. Defaults to 80.
    #[serde(
        deserialize_with = "deserialize_line_width",
        serialize_with = "serialize_line_width"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("css-formatter-line-width"), argument("NUMBER"), optional)]
    pub line_width: Option<LineWidth>,

    #[bpaf(long("css-formatter-quote-style"), argument("double|single"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_style: Option<QuoteStyle>,
}
