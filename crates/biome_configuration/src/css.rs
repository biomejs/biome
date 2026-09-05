use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{
    DelimiterSpacing, IndentStyle, IndentWidth, LineEnding, LineWidth, QuoteStyle, TrailingNewline,
};
#[cfg(feature = "cli")]
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Options applied to CSS and languages that extend it.
///
/// Language-specific settings take precedence over corresponding global settings. Global settings
/// apply when their language-specific counterparts are omitted, unless stated otherwise.
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

    /// Reserved for future CSS analyzer support. This option currently has no effect.
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
    /// Enables `//` line comments in plain CSS. Standard CSS treats `//` as delimiter characters
    /// rather than as a comment, and SCSS accepts `//` comments independently of this option.
    /// Defaults to `false`.
    #[cfg_attr(feature = "cli", bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_wrong_line_comments: Option<CssAllowWrongLineCommentsEnabled>,

    /// Enables CSS Modules-specific syntax such as `:local`, `:global`, `composes`, and `@value`.
    /// When unset, Biome enables this syntax automatically for files whose names end in
    /// `.module.css`; otherwise, it defaults to `false`. Enable it explicitly when CSS Module files
    /// use another naming convention.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "cli",
        bpaf(long("css-parse-css-modules"), argument("true|false"))
    )]
    pub css_modules: Option<CssModulesEnabled>,

    /// Enables parsing Tailwind CSS 4.0 directives and functions, including `@theme`, `@utility`,
    /// `@variant`, `@source`, and `@apply`. Defaults to `false`.
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
    /// Enables or disables the formatter for CSS and languages that extend it.
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

    /// The preferred maximum line width for CSS and languages that extend it. If unset, inherits
    /// the global line width.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("css-formatter-line-width"), argument("NUMBER"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,

    /// Selects the preferred quote style for CSS strings. Biome may use the alternate quote when
    /// that avoids additional escaping. Quotes in `@charset` rules are currently preserved.
    /// Defaults to `double`.
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
    /// Enables or disables the linter for CSS.
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
    /// Enables or disables assist actions for CSS.
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
