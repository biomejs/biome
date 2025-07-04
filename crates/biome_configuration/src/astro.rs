use crate::bool::Bool;
use biome_deserialize::Merge;
use biome_deserialize_macros::{Deserializable, Merge};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Specific configuration for the Astro language
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct AstroConfiguration {
    /// Formatting options for Astro files
    #[bpaf(external(astro_formatter_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<AstroFormatterConfiguration>,

    /// Linting options for Astro files
    #[bpaf(external(astro_linter_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linter: Option<AstroLinterConfiguration>,

    /// Parsing options for Astro files
    #[bpaf(external(astro_parser_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parser: Option<AstroParserConfiguration>,
}

/// Formatter options for Astro files
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct AstroFormatterConfiguration {
    /// Control the formatting of the Astro language.
    #[bpaf(long("astro-formatter-enabled"), argument("true|false"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Bool>,

    /// Use single quotes instead of double quotes in Astro.
    #[bpaf(long("astro-single-quote"), argument("true|false"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_style: Option<AstroQuoteStyle>,

    /// Use spaces for indentation instead of tabs.
    #[bpaf(long("astro-indent-style"), argument("tab|space"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_style: Option<AstroIndentStyle>,

    /// The number of spaces per indentation level.
    #[bpaf(long("astro-indent-width"), argument("NUMBER"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_width: Option<u8>,

    /// The line width limit for the formatter.
    #[bpaf(long("astro-line-width"), argument("NUMBER"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<u16>,

    /// Allow Astro shorthand attributes (e.g., {prop} instead of prop={prop}).
    #[bpaf(long("astro-allow-shorthand"), argument("true|false"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_shorthand: Option<Bool>,
}

/// Linter options for Astro files
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct AstroLinterConfiguration {
    /// Control the linting of the Astro language.
    #[bpaf(long("astro-linter-enabled"), argument("true|false"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Bool>,
}

/// Parser options for Astro files
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct AstroParserConfiguration {
    /// Allow invalid Astro code in the parser.
    #[bpaf(long("astro-allow-invalid-code"), argument("true|false"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_invalid_code: Option<Bool>,
}

/// Quote style for Astro files
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum AstroQuoteStyle {
    #[default]
    Double,
    Single,
}

/// Indent style for Astro files
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum AstroIndentStyle {
    #[default]
    Tab,
    Space,
}

pub fn astro_configuration() -> AstroConfiguration {
    AstroConfiguration::default()
}

pub fn astro_formatter_configuration() -> AstroFormatterConfiguration {
    AstroFormatterConfiguration::default()
}

pub fn astro_linter_configuration() -> AstroLinterConfiguration {
    AstroLinterConfiguration::default()
}

pub fn astro_parser_configuration() -> AstroParserConfiguration {
    AstroParserConfiguration::default()
}