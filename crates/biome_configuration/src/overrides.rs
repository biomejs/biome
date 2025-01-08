use crate::analyzer::assist::AssistEnabled;
use crate::analyzer::{LinterEnabled, RuleDomainValue};
use crate::formatter::{FormatWithErrorsEnabled, FormatterEnabled};
use crate::html::HtmlConfiguration;
use crate::{
    CssConfiguration, GraphqlConfiguration, GritConfiguration, JsConfiguration, JsonConfiguration,
    Rules,
};
use biome_analyze::RuleDomain;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{
    AttributePosition, BracketSameLine, BracketSpacing, IndentStyle, IndentWidth, LineEnding,
    LineWidth,
};
use bpaf::Bpaf;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Overrides(pub Vec<OverridePattern>);

impl FromStr for Overrides {
    type Err = String;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self::default())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct OverridePattern {
    /// A list of Unix shell style patterns. The formatter will ignore files/folders that will
    /// match these patterns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore: Option<Vec<Box<str>>>,

    /// A list of Unix shell style patterns. The formatter will include files/folders that will
    /// match these patterns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<Box<str>>>,

    /// Specific configuration for the JavaScript language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub javascript: Option<JsConfiguration>,

    /// Specific configuration for the Json language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<JsonConfiguration>,

    /// Specific configuration for the CSS language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css: Option<CssConfiguration>,

    /// Specific configuration for the Graphql language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql: Option<GraphqlConfiguration>,

    /// Specific configuration for the GritQL language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grit: Option<GritConfiguration>,

    /// Specific configuration for the GritQL language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<HtmlConfiguration>,

    /// Specific configuration for the Json language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<OverrideFormatterConfiguration>,

    /// Specific configuration for the Json language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linter: Option<OverrideLinterConfiguration>,

    /// Specific configuration for the Json language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assist: Option<OverrideAssistConfiguration>,
}

impl FromStr for OverridePattern {
    type Err = String;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self::default())
    }
}

#[derive(
    Bpaf, Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct OverrideFormatterConfiguration {
    // if `false`, it disables the feature. `true` by default
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub enabled: Option<FormatterEnabled>,

    /// Stores whether formatting should be allowed to proceed if a given file
    /// has syntax errors
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub format_with_errors: Option<FormatWithErrorsEnabled>,

    /// The indent style.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("indent-style"), argument("tab|space"))]
    pub indent_style: Option<IndentStyle>,

    /// The size of the indentation, 2 by default (deprecated, use `indent-width`)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deserializable(deprecated(use_instead = "formatter.indentWidth"))]
    #[bpaf(long("indent-size"), argument("NUMBER"))]
    pub indent_size: Option<IndentWidth>,

    /// The size of the indentation, 2 by default
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("indent-width"), argument("NUMBER"))]
    pub indent_width: Option<IndentWidth>,

    /// The type of line ending.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("line-ending"), argument("lf|crlf|cr"))]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line. Defaults to 80.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("line-width"), argument("NUMBER"))]
    pub line_width: Option<LineWidth>,

    /// The attribute position style.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("attribute-position"), argument("multiline|auto"))]
    pub attribute_position: Option<AttributePosition>,

    /// Put the `>` of a multi-line HTML or JSX element at the end of the last line instead of being alone on the next line (does not apply to self closing elements).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("bracket-same-line"), argument("true|false"))]
    pub bracket_same_line: Option<BracketSameLine>,

    /// Whether to insert spaces around brackets in object literals. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("bracket-spacing"), argument("true|false"))]
    pub bracket_spacing: Option<BracketSpacing>,
}

#[derive(
    Bpaf, Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct OverrideLinterConfiguration {
    /// if `false`, it disables the feature and the linter won't be executed. `true` by default
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub enabled: Option<LinterEnabled>,

    /// List of rules
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(pure(Rules::default()), hide)]
    pub rules: Option<Rules>,

    /// List of rules
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(pure(FxHashMap::default()), optional, hide)]
    pub domains: Option<FxHashMap<RuleDomain, RuleDomainValue>>,
}

#[derive(
    Bpaf, Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct OverrideAssistConfiguration {
    /// if `false`, it disables the feature and the assist won't be executed. `true` by default
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub enabled: Option<AssistEnabled>,

    /// List of actions
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(pure(crate::analyzer::assist::Actions::default()), optional, hide)]
    pub actions: Option<crate::analyzer::assist::Actions>,
}
