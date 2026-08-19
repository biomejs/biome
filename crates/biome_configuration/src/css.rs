use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{
    DelimiterSpacing, IndentStyle, IndentWidth, LineEnding, LineWidth, QuoteStyle, TrailingNewline,
};
#[cfg(feature = "cli")]
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Options applied to CSS files.
#[derive(Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CssConfiguration {
    /// CSS parsing options.
    #[cfg_attr(feature = "cli", bpaf(external(css_parser_configuration), optional))]
    pub parser: Option<CssParserConfiguration>,

    /// CSS formatter options.
    #[cfg_attr(feature = "cli", bpaf(external(css_formatter_configuration), optional))]
    pub formatter: Option<CssFormatterConfiguration>,

    /// CSS linter options.
    #[cfg_attr(feature = "cli", bpaf(external(css_linter_configuration), optional))]
    pub linter: Option<CssLinterConfiguration>,

    /// CSS assist options.
    #[cfg_attr(feature = "cli", bpaf(external(css_assist_configuration), optional))]
    pub assist: Option<CssAssistConfiguration>,

    /// CSS globals.
    #[cfg_attr(feature = "cli", bpaf(pure(Default::default()), hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub globals: Option<rustc_hash::FxHashSet<Box<str>>>,
}

pub type CssAllowWrongLineCommentsEnabled = Bool<false>;
pub type CssModulesEnabled = Bool<false>;
pub type CssTailwindDirectivesEnabled = Bool<false>;

/// Options that change how the CSS parser behaves.
#[derive(Clone, Default, Debug, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CssParserConfiguration {
    /// Allows comments to appear on incorrect lines in `.css` files.
    #[cfg_attr(feature = "cli", bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_wrong_line_comments: Option<CssAllowWrongLineCommentsEnabled>,

    /// Enables parsing of CSS Modules-specific features. Enable this feature only
    /// when your files don't end in `.module.css`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "cli",
        bpaf(long("css-parse-css-modules"), argument("true|false"))
    )]
    pub css_modules: Option<CssModulesEnabled>,

    /// Enables parsing of Tailwind CSS 4.0 directives and functions.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("css-parse-tailwind-directives"), argument("true|false"))
    )]
    pub tailwind_directives: Option<CssTailwindDirectivesEnabled>,
}

pub type CssFormatterEnabled = Bool<true>;

/// Options that change how the CSS formatter behaves.
#[derive(Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CssFormatterConfiguration {
    /// Controls the formatter for CSS and languages that extend it.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("css-formatter-enabled"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<CssFormatterEnabled>,

    /// The indent style applied to CSS and languages that extend it. If unset, inherits the global
    /// indentation style.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("css-formatter-indent-style"), argument("tab|space"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_style: Option<IndentStyle>,

    /// The indentation width applied to CSS and languages that extend it. If unset, inherits the
    /// global indentation width.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("css-formatter-indent-width"), argument("NUMBER"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_width: Option<IndentWidth>,

    /// The line ending applied to CSS and languages that extend it. If unset, inherits the global
    /// line ending.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("css-formatter-line-ending"), argument("lf|crlf|cr|auto"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<LineEnding>,

    /// The maximum line width for CSS and languages that extend it. If unset, inherits the global
    /// line width.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("css-formatter-line-width"), argument("NUMBER"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,

    /// The type of quotes used in CSS code. Defaults to `double`.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("css-formatter-quote-style"), argument("double|single"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_style: Option<QuoteStyle>,

    /// Controls spaces inside CSS parentheses and square brackets when their content fits on one
    /// line. When enabled, `rgb(0, 0, 0)` becomes `rgb( 0, 0, 0 )` and `[data-attr]` becomes
    /// `[ data-attr ]`. Empty delimiters are unchanged.
    ///
    /// If unset, inherits the global delimiter spacing setting.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("css-formatter-delimiter-spacing"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter_spacing: Option<DelimiterSpacing>,

    /// Whether to add a trailing newline at the end of the file. If unset, inherits the global
    /// trailing newline setting.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("css-formatter-trailing-newline"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_newline: Option<TrailingNewline>,
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

/// Options that change how the CSS linter behaves.
#[derive(Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CssLinterConfiguration {
    /// Controls the linter for CSS files.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("css-linter-enabled"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<CssLinterEnabled>,
}

impl CssLinterConfiguration {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }
}

pub type CssAssistEnabled = Bool<true>;

/// Options that change how CSS assist behaves.
#[derive(Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CssAssistConfiguration {
    /// Controls assist actions for CSS files.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("css-assist-enabled"), argument("true|false"))
    )]
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
