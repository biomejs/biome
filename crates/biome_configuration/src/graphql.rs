use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{
    BracketSpacing, IndentStyle, IndentWidth, LineEnding, LineWidth, QuoteStyle,
};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Options applied to GraphQL files
#[derive(
    Bpaf, Clone, Default, Debug, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct GraphqlConfiguration {
    /// GraphQL formatter options
    #[bpaf(external(graphql_formatter_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<GraphqlFormatterConfiguration>,

    // GraphQL linter options
    #[bpaf(external(graphql_linter_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linter: Option<GraphqlLinterConfiguration>,

    /// Assist options
    #[bpaf(external(graphql_assist_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assist: Option<GraphqlAssistConfiguration>,
}

pub type GraphqlFormatterEnabled = Bool<true>;

/// Options that changes how the GraphQL formatter behaves
#[derive(
    Bpaf, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct GraphqlFormatterConfiguration {
    /// Control the formatter for GraphQL files.
    #[bpaf(long("graphql-formatter-enabled"), argument("true|false"))]
    pub enabled: Option<GraphqlFormatterEnabled>,

    /// The indent style applied to GraphQL files.
    #[bpaf(long("graphql-formatter-indent-style"), argument("tab|space"))]
    pub indent_style: Option<IndentStyle>,

    /// The size of the indentation applied to GraphQL files. Default to 2.
    #[bpaf(long("graphql-formatter-indent-width"), argument("NUMBER"))]
    pub indent_width: Option<IndentWidth>,

    /// The type of line ending applied to GraphQL files.
    #[bpaf(long("graphql-formatter-line-ending"), argument("lf|crlf|cr"))]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to GraphQL files. Defaults to 80.
    #[bpaf(long("graphql-formatter-line-width"), argument("NUMBER"))]
    pub line_width: Option<LineWidth>,

    /// The type of quotes used in GraphQL code. Defaults to double.
    #[bpaf(long("graphql-formatter-quote-style"), argument("double|single"))]
    pub quote_style: Option<QuoteStyle>,

    // it's also a top-level configurable property.
    /// Whether to insert spaces around brackets in object literals. Defaults to true.
    #[bpaf(long("bracket-spacing"), argument("true|false"))]
    pub bracket_spacing: Option<BracketSpacing>,
}

impl GraphqlFormatterConfiguration {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }

    pub fn quote_style_resolved(&self) -> QuoteStyle {
        self.quote_style.unwrap_or_default()
    }
}

pub type GraphqlLinterEnabled = Bool<true>;

/// Options that change how the GraphQL linter behaves.
#[derive(
    Bpaf, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct GraphqlLinterConfiguration {
    /// Control the formatter for GraphQL files.
    #[bpaf(long("graphql-linter-enabled"), argument("true|false"))]
    pub enabled: Option<GraphqlLinterEnabled>,
}

pub type GraphqlAssistEnabled = Bool<false>;

/// Options that changes how the GraphQL linter behaves
#[derive(
    Bpaf, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct GraphqlAssistConfiguration {
    /// Control the formatter for GraphQL files.
    #[bpaf(long("graphql-assist-enabled"), argument("true|false"))]
    pub enabled: Option<GraphqlAssistEnabled>,
}

impl GraphqlLinterConfiguration {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }
}

#[test]
fn default_graphql_formatter() {
    let graphql_configuration = GraphqlFormatterConfiguration::default();

    assert!(graphql_configuration.is_enabled());
    assert_eq!(graphql_configuration.indent_style, None);
    assert_eq!(graphql_configuration.indent_width, None);
    assert_eq!(graphql_configuration.line_ending, None);
    assert_eq!(graphql_configuration.line_width, None);
    assert_eq!(graphql_configuration.quote_style, None);
}

#[test]
fn default_graphql_linter() {
    let graphql_configuration = GraphqlLinterConfiguration::default();

    assert!(graphql_configuration.is_enabled());
}
