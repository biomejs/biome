use crate::PlainIndentStyle;
use biome_deserialize_macros::{Deserializable, Merge, Partial};
use biome_formatter::{LineEnding, LineWidth, QuoteStyle};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Options applied to CSS files
#[derive(Clone, Default, Debug, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct CssConfiguration {
    /// CSS parsing options
    #[partial(type, bpaf(external(partial_css_parser), optional))]
    pub parser: CssParser,

    /// CSS formatter options
    #[partial(type, bpaf(external(partial_css_formatter), optional))]
    pub formatter: CssFormatter,

    /// CSS linter options
    #[partial(type, bpaf(external(partial_css_linter), optional))]
    pub linter: CssLinter,
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

/// Options that changes how the CSS formatter behaves
#[derive(Clone, Debug, Default, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct CssFormatter {
    /// Control the formatter for CSS (and its super languages) files.
    #[partial(bpaf(long("css-formatter-enabled"), argument("true|false"), optional))]
    pub enabled: bool,

    /// The indent style applied to CSS (and its super languages) files.
    #[partial(bpaf(long("css-formatter-indent-style"), argument("tab|space"), optional))]
    pub indent_style: PlainIndentStyle,

    /// The size of the indentation applied to CSS (and its super languages) files. Default to 2.
    #[partial(bpaf(long("css-formatter-indent-width"), argument("NUMBER"), optional))]
    pub indent_width: u8,

    /// The type of line ending applied to CSS (and its super languages) files.
    #[partial(bpaf(long("css-formatter-line-ending"), argument("lf|crlf|cr"), optional))]
    pub line_ending: LineEnding,

    /// What's the max width of a line applied to CSS (and its super languages) files. Defaults to 80.
    #[partial(bpaf(long("css-formatter-line-width"), argument("NUMBER"), optional))]
    pub line_width: LineWidth,

    /// The type of quotes used in CSS code. Defaults to double.
    #[partial(bpaf(long("css-formatter-quote-style"), argument("double|single"), optional))]
    pub quote_style: QuoteStyle,
}

impl PartialCssFormatter {
    pub fn get_formatter_configuration(&self) -> CssFormatter {
        CssFormatter {
            enabled: self.enabled.unwrap_or_default(),
            indent_style: self.indent_style.unwrap_or_default(),
            indent_width: self.indent_width.unwrap_or_default(),
            line_ending: self.line_ending.unwrap_or_default(),
            line_width: self.line_width.unwrap_or_default(),
            quote_style: self.quote_style.unwrap_or_default(),
        }
    }
}

/// Options that changes how the CSS linter behaves
#[derive(Clone, Debug, Deserialize, Eq, Partial, PartialEq, Default, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct CssLinter {
    /// Control the linter for CSS (and its super languages) files.
    #[partial(bpaf(long("css-linter-enabled"), argument("true|false"), optional))]
    pub enabled: bool,
}
