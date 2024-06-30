use crate::{bool::Bool, PlainIndentStyle};
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{IndentWidth, LineEnding, LineWidth, QuoteStyle};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Options applied to CSS files
#[derive(
    Bpaf, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CssConfiguration {
    /// CSS parsing options
    #[bpaf(external(css_parser_configuration), optional)]
    pub parser: Option<CssParserConfiguration>,

    /// CSS formatter options
    #[bpaf(external(css_formatter_configuration), optional)]
    pub formatter: Option<CssFormatterConfiguration>,

    /// CSS linter options
    #[bpaf(external(css_linter_configuration), optional)]
    pub linter: Option<CssLinterConfiguration>,
}

pub type AllowWrongLineCommentsEnabled = Bool<false>;
pub type CssModulesEnabled = Bool<false>;

/// Options that changes how the CSS parser behaves
#[derive(
    Bpaf, Clone, Default, Debug, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CssParserConfiguration {
    /// Allow comments to appear on incorrect lines in `.css` files
    #[bpaf(hide)]
    pub allow_wrong_line_comments: Option<AllowWrongLineCommentsEnabled>,

    /// Enables parsing of CSS Modules specific features.
    #[bpaf(hide)]
    pub css_modules: Option<CssModulesEnabled>,
}

pub type CssFormatterEnabled = Bool<false>;

/// Options that changes how the CSS formatter behaves
#[derive(
    Bpaf, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CssFormatterConfiguration {
    /// Control the formatter for CSS (and its super languages) files.
    #[bpaf(long("css-formatter-enabled"), argument("true|false"))]
    pub enabled: Option<CssFormatterEnabled>,

    /// The indent style applied to CSS (and its super languages) files.
    #[bpaf(long("css-formatter-indent-style"), argument("tab|space"))]
    pub indent_style: Option<PlainIndentStyle>,

    /// The size of the indentation applied to CSS (and its super languages) files. Default to 2.
    #[bpaf(long("css-formatter-indent-width"), argument("NUMBER"))]
    pub indent_width: Option<IndentWidth>,

    /// The type of line ending applied to CSS (and its super languages) files.
    #[bpaf(long("css-formatter-line-ending"), argument("lf|crlf|cr"))]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to CSS (and its super languages) files. Defaults to 80.
    #[bpaf(long("css-formatter-line-width"), argument("NUMBER"))]
    pub line_width: Option<LineWidth>,

    /// The type of quotes used in CSS code. Defaults to double.
    #[bpaf(long("css-formatter-quote-style"), argument("double|single"))]
    pub quote_style: Option<QuoteStyle>,
}

impl CssFormatterConfiguration {
    pub fn enabled_resolved(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }

    pub fn quote_style_resolved(&self) -> QuoteStyle {
        self.quote_style.unwrap_or_default()
    }
}

pub type CssLinterEnabled = Bool<false>;
/// Options that changes how the CSS linter behaves
#[derive(
    Bpaf, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CssLinterConfiguration {
    /// Control the linter for CSS (and its super languages) files.
    #[bpaf(long("css-linter-enabled"), argument("true|false"))]
    pub enabled: Option<CssLinterEnabled>,
}

impl CssLinterConfiguration {
    pub fn enabled_resolved(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }
}

#[test]
fn default_css() {
    let css_configuration = CssFormatterConfiguration::default();

    assert!(!css_configuration.enabled_resolved());
    assert_eq!(css_configuration.indent_style, None);
    assert_eq!(css_configuration.indent_width, None);
    assert_eq!(css_configuration.line_ending, None);
    assert_eq!(css_configuration.line_width, None);
    assert_eq!(css_configuration.quote_style, None);
}
