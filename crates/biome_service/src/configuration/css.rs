use crate::configuration::{deserialize_line_width, serialize_line_width, PlainIndentStyle};
use biome_deserialize_macros::{Deserializable, Merge, Partial};
use biome_formatter::{LineEnding, LineWidth, QuoteStyle};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Options applied to CSS files
#[derive(Clone, Default, Debug, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(default, deny_unknown_fields))]
pub struct CssConfiguration {
    /// Parsing options
    #[partial(type, bpaf(external(partial_css_parser), optional))]
    pub parser: CssParser,

    /// Formatting options
    #[partial(type, bpaf(external(partial_css_formatter), optional))]
    pub formatter: CssFormatter,
}

/// Options that changes how the CSS parser behaves
#[derive(Clone, Default, Debug, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct CssParser {
    /// Allow comments to appear on incorrect lines in `.css` files
    #[partial(bpaf(hide))]
    pub allow_wrong_line_comments: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct CssFormatter {
    /// Control the formatter for CSS (and its super languages) files.
    #[partial(bpaf(long("css-formatter-enabled"), argument("true|false"), optional))]
    pub enabled: bool,

    /// The indent style applied to CSS (and its super languages) files.
    #[partial(bpaf(long("css-formatter-indent-style"), argument("tab|space"), optional))]
    pub indent_style: Option<PlainIndentStyle>,

    /// The size of the indentation applied to CSS (and its super languages) files. Default to 2.
    #[partial(bpaf(long("css-formatter-indent-width"), argument("NUMBER"), optional))]
    pub indent_width: Option<u8>,

    /// The size of the indentation applied to CSS (and its super languages) files. Default to 2.
    #[partial(bpaf(long("css-formatter-indent-size"), argument("NUMBER"), optional))]
    #[partial(deserializable(deprecated(use_instead = "css.formatter.indentWidth")))]
    pub indent_size: Option<u8>,

    /// The type of line ending applied to CSS (and its super languages) files.
    #[partial(bpaf(long("css-formatter-line-ending"), argument("lf|crlf|cr"), optional))]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to CSS (and its super languages) files. Defaults to 80.
    #[partial(serde(
        deserialize_with = "deserialize_line_width",
        serialize_with = "serialize_line_width"
    ))]
    #[partial(bpaf(long("css-formatter-line-width"), argument("NUMBER"), optional))]
    pub line_width: Option<LineWidth>,

    #[partial(bpaf(long("css-formatter-quote-style"), argument("double|single"), optional))]
    pub quote_style: QuoteStyle,
}

impl Default for CssFormatter {
    fn default() -> Self {
        Self {
            enabled: true,
            indent_style: Default::default(),
            indent_width: Default::default(),
            indent_size: Default::default(),
            line_ending: Default::default(),
            line_width: Default::default(),
            quote_style: Default::default(),
        }
    }
}
