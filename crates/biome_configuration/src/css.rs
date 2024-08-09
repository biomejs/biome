use biome_deserialize_macros::{Deserializable, Merge, Partial};
use biome_formatter::{IndentStyle, IndentWidth, LineEnding, LineWidth, QuoteStyle};
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

    /// CSS assists options
    #[partial(type, bpaf(external(partial_css_assists), optional))]
    pub assists: CssAssists,
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

    /// Enables parsing of CSS Modules specific features.
    #[partial(bpaf(hide))]
    pub css_modules: bool,
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
    pub indent_style: Option<IndentStyle>,

    /// The size of the indentation applied to CSS (and its super languages) files. Default to 2.
    #[partial(bpaf(long("css-formatter-indent-width"), argument("NUMBER"), optional))]
    pub indent_width: Option<IndentWidth>,

    /// The type of line ending applied to CSS (and its super languages) files.
    #[partial(bpaf(long("css-formatter-line-ending"), argument("lf|crlf|cr"), optional))]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to CSS (and its super languages) files. Defaults to 80.
    #[partial(bpaf(long("css-formatter-line-width"), argument("NUMBER"), optional))]
    pub line_width: Option<LineWidth>,

    /// The type of quotes used in CSS code. Defaults to double.
    #[partial(bpaf(long("css-formatter-quote-style"), argument("double|single"), optional))]
    pub quote_style: QuoteStyle,
}

impl PartialCssFormatter {
    pub fn get_formatter_configuration(&self) -> CssFormatter {
        CssFormatter {
            enabled: self.enabled.unwrap_or_default(),
            indent_style: self.indent_style,
            indent_width: self.indent_width,
            line_ending: self.line_ending,
            line_width: self.line_width,
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
    /// Control the linter for CSS files.
    #[partial(bpaf(long("css-linter-enabled"), argument("true|false"), optional))]
    pub enabled: bool,
}

impl PartialCssLinter {
    pub fn get_linter_configuration(&self) -> CssLinter {
        CssLinter {
            enabled: self.enabled.unwrap_or_default(),
        }
    }
}

/// Options that changes how the CSS assists behaves
#[derive(Clone, Debug, Deserialize, Eq, Partial, PartialEq, Default, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct CssAssists {
    /// Control the assists for CSS files.
    #[partial(bpaf(long("css-assists-enabled"), argument("true|false"), optional))]
    pub enabled: bool,
}

#[test]
fn default_css() {
    let css_configuration = CssFormatter::default();

    assert!(!css_configuration.enabled);
    assert_eq!(css_configuration.indent_style, None);
    assert_eq!(css_configuration.indent_width, None);
    assert_eq!(css_configuration.line_ending, None);
    assert_eq!(css_configuration.line_width, None);
    assert_eq!(css_configuration.quote_style, QuoteStyle::Double);
}
