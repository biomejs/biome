use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{IndentStyle, IndentWidth, LineEnding, LineWidth, QuoteStyle};
use clap::Args;
use serde::{Deserialize, Serialize};

/// Options applied to CSS files
#[derive(
    Args, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CssConfiguration {
    /// CSS parsing options
    #[clap(flatten)]
    pub parser: Option<CssParserConfiguration>,

    /// CSS formatter options
    #[clap(flatten)]
    pub formatter: Option<CssFormatterConfiguration>,

    /// CSS linter options
    #[clap(flatten)]
    pub linter: Option<CssLinterConfiguration>,

    /// CSS assist options
    #[clap(flatten)]
    pub assist: Option<CssAssistConfiguration>,

    /// CSS globals
    #[clap(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub globals: Option<rustc_hash::FxHashSet<Box<str>>>,
}

pub type CssAllowWrongLineCommentsEnabled = Bool<false>;
pub type CssModulesEnabled = Bool<false>;
pub type CssTailwindDirectivesEnabled = Bool<false>;

/// Options that changes how the CSS parser behaves
#[derive(
    Args, Clone, Default, Debug, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CssParserConfiguration {
    /// Allow comments to appear on incorrect lines in `.css` files
    #[arg(
        long = "css-parse-allow-wrong-line-comments",
        value_name = "true|false"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_wrong_line_comments: Option<CssAllowWrongLineCommentsEnabled>,

    /// Enables parsing of CSS Modules specific features.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[arg(long = "css-parse-css-modules", value_name = "true|false")]
    pub css_modules: Option<CssModulesEnabled>,

    /// Enables parsing of Tailwind CSS 4.0 directives and functions.
    #[arg(long = "css-parse-tailwind-directives", value_name = "true|false")]
    pub tailwind_directives: Option<CssTailwindDirectivesEnabled>,
}

pub type CssFormatterEnabled = Bool<true>;

/// Options that changes how the CSS formatter behaves
#[derive(
    Args, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[group(skip)]
pub struct CssFormatterConfiguration {
    /// Control the formatter for CSS (and its super languages) files.
    #[arg(long = "css-formatter-enabled", value_name = "true|false")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<CssFormatterEnabled>,

    /// The indent style applied to CSS (and its super languages) files.
    #[arg(long = "css-formatter-indent-style", value_name = "tab|space")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_style: Option<IndentStyle>,

    /// The size of the indentation applied to CSS (and its super languages) files. Default to 2.
    #[arg(long = "css-formatter-indent-width", value_name = "NUMBER")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_width: Option<IndentWidth>,

    /// The type of line ending applied to CSS (and its super languages) files. `auto` uses CRLF on Windows and LF on other platforms.
    #[arg(long = "css-formatter-line-ending", value_name = "lf|crlf|cr|auto")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to CSS (and its super languages) files. Defaults to 80.
    #[arg(long = "css-formatter-line-width", value_name = "NUMBER")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,

    /// The type of quotes used in CSS code. Defaults to double.
    #[arg(long = "css-formatter-quote-style", value_name = "double|single")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_style: Option<QuoteStyle>,
}

impl CssFormatterConfiguration {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }

    pub fn quote_style_resolved(&self) -> QuoteStyle {
        self.quote_style.unwrap_or_default()
    }
}

pub type CssLinterEnabled = Bool<true>;

/// Options that changes how the CSS linter behaves
#[derive(
    Args, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CssLinterConfiguration {
    /// Control the linter for CSS files.
    #[arg(long = "css-linter-enabled", value_name = "true|false")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<CssLinterEnabled>,
}

impl CssLinterConfiguration {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }
}

pub type CssAssistEnabled = Bool<true>;

/// Options that changes how the CSS assist behaves
#[derive(
    Args, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CssAssistConfiguration {
    /// Control the assist for CSS files.
    #[arg(long = "css-assist-enabled", value_name = "true|false")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<CssAssistEnabled>,
}

impl CssAssistConfiguration {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }
}

#[test]
fn default_css() {
    let css_configuration = CssFormatterConfiguration::default();

    assert!(css_configuration.is_enabled());
    assert_eq!(css_configuration.indent_style, None);
    assert_eq!(css_configuration.indent_width, None);
    assert_eq!(css_configuration.line_ending, None);
    assert_eq!(css_configuration.line_width, None);
    assert_eq!(css_configuration.quote_style, None);
}
