use super::javascript::PartialJavascriptConfiguration;
use super::json::PartialJsonConfiguration;
use super::{PartialCssConfiguration, PartialGraphqlConfiguration};
use crate::{
    partial_css_configuration, partial_graphql_configuration, partial_javascript_configuration,
    partial_json_configuration, PlainIndentStyle,
};
use biome_deserialize::StringSet;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{AttributePosition, BracketSpacing, IndentWidth, LineEnding, LineWidth};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(
    Bpaf, Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Overrides(#[bpaf(hide)] pub Vec<OverridePattern>);

impl FromStr for Overrides {
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
pub struct OverridePattern {
    /// A list of Unix shell style patterns. The formatter will ignore files/folders that will
    /// match these patterns.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub ignore: Option<StringSet>,

    /// A list of Unix shell style patterns. The formatter will include files/folders that will
    /// match these patterns.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub include: Option<StringSet>,

    /// Specific configuration for the JavaScript language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(partial_javascript_configuration), optional, hide)]
    pub javascript: Option<PartialJavascriptConfiguration>,

    /// Specific configuration for the Json language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(partial_json_configuration), optional, hide)]
    pub json: Option<PartialJsonConfiguration>,

    /// Specific configuration for the Css language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(partial_css_configuration), optional, hide)]
    pub css: Option<PartialCssConfiguration>,

    /// Specific configuration for the Graphql language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(partial_graphql_configuration), optional, hide)]
    pub graphql: Option<PartialGraphqlConfiguration>,

    /// Specific configuration for the Json language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(override_formatter_configuration), optional, hide)]
    pub formatter: Option<OverrideFormatterConfiguration>,

    /// Specific configuration for the Json language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(override_linter_configuration), optional, hide)]
    pub linter: Option<OverrideLinterConfiguration>,

    /// Specific configuration for the Json language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(override_organize_imports_configuration), optional, hide)]
    pub organize_imports: Option<OverrideOrganizeImportsConfiguration>,
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
    pub enabled: Option<bool>,

    /// Stores whether formatting should be allowed to proceed if a given file
    /// has syntax errors
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub format_with_errors: Option<bool>,

    /// The indent style.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("indent-style"), argument("tab|space"), optional)]
    pub indent_style: Option<PlainIndentStyle>,

    /// The size of the indentation, 2 by default (deprecated, use `indent-width`)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deserializable(deprecated(use_instead = "formatter.indentWidth"))]
    #[bpaf(long("indent-size"), argument("NUMBER"), optional)]
    pub indent_size: Option<IndentWidth>,

    /// The size of the indentation, 2 by default
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("indent-width"), argument("NUMBER"), optional)]
    pub indent_width: Option<IndentWidth>,

    /// The type of line ending.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("line-ending"), argument("lf|crlf|cr"), optional)]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line. Defaults to 80.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("line-width"), argument("NUMBER"), optional)]
    pub line_width: Option<LineWidth>,

    /// The attribute position style.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("attribute-position"), argument("multiline|auto"), optional)]
    pub attribute_position: Option<AttributePosition>,

    /// Whether to insert spaces around brackets in object literals. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("bracket-spacing"), argument("true|false"), optional)]
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
    pub enabled: Option<bool>,

    /// List of rules
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(pure(crate::analyzer::linter::Rules::default()), optional, hide)]
    pub rules: Option<crate::analyzer::linter::Rules>,
}

#[derive(
    Bpaf, Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct OverrideOrganizeImportsConfiguration {
    /// if `false`, it disables the feature and the linter won't be executed. `true` by default
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub enabled: Option<bool>,
}

#[derive(
    Bpaf, Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct OverrideAssistsConfiguration {
    /// if `false`, it disables the feature and the linter won't be executed. `true` by default
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub enabled: Option<bool>,

    /// List of rules
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(pure(crate::analyzer::assists::Actions::default()), optional, hide)]
    pub rules: Option<crate::analyzer::assists::Actions>,
}
