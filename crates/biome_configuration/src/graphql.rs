use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{
    BracketSpacing, IndentStyle, IndentWidth, LineEnding, LineWidth, QuoteStyle, TrailingNewline,
};
#[cfg(feature = "cli")]
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Options applied to GraphQL files.
#[derive(Clone, Default, Debug, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct GraphqlConfiguration {
    /// GraphQL formatter options.
    #[cfg_attr(
        feature = "cli",
        bpaf(external(graphql_formatter_configuration), optional)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<GraphqlFormatterConfiguration>,

    /// GraphQL linter options.
    #[cfg_attr(
        feature = "cli",
        bpaf(external(graphql_linter_configuration), optional)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linter: Option<GraphqlLinterConfiguration>,

    /// GraphQL assist options.
    #[cfg_attr(
        feature = "cli",
        bpaf(external(graphql_assist_configuration), optional)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assist: Option<GraphqlAssistConfiguration>,
}

pub type GraphqlFormatterEnabled = Bool<true>;

/// Options that change how the GraphQL formatter behaves.
#[derive(Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct GraphqlFormatterConfiguration {
    /// Controls the formatter for GraphQL files.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("graphql-formatter-enabled"), argument("true|false"))
    )]
    pub enabled: Option<GraphqlFormatterEnabled>,

    /// The indent style applied to GraphQL files. If unset, inherits the global indentation style.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("graphql-formatter-indent-style"), argument("tab|space"))
    )]
    pub indent_style: Option<IndentStyle>,

    /// The indentation width applied to GraphQL files. If unset, inherits the global indentation
    /// width.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("graphql-formatter-indent-width"), argument("NUMBER"))
    )]
    pub indent_width: Option<IndentWidth>,

    /// The line ending applied to GraphQL files. If unset, inherits the global line ending.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("graphql-formatter-line-ending"), argument("lf|crlf|cr|auto"))
    )]
    pub line_ending: Option<LineEnding>,

    /// The maximum line width for GraphQL files. If unset, inherits the global line width.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("graphql-formatter-line-width"), argument("NUMBER"))
    )]
    pub line_width: Option<LineWidth>,

    /// The type of quotes used in GraphQL code. Defaults to `double`.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("graphql-formatter-quote-style"), argument("double|single"))
    )]
    pub quote_style: Option<QuoteStyle>,

    // it's also a top-level configurable property.
    /// Whether to insert spaces inside braces in object literals. If unset, inherits the global
    /// bracket spacing setting.
    #[cfg_attr(feature = "cli", bpaf(long("bracket-spacing"), argument("true|false")))]
    pub bracket_spacing: Option<BracketSpacing>,

    /// Whether to add a trailing newline at the end of the file. If unset, inherits the global
    /// trailing newline setting.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("graphql-formatter-trailing-newline"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_newline: Option<TrailingNewline>,
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
#[derive(Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct GraphqlLinterConfiguration {
    /// Controls the linter for GraphQL files.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("graphql-linter-enabled"), argument("true|false"))
    )]
    pub enabled: Option<GraphqlLinterEnabled>,
}

pub type GraphqlAssistEnabled = Bool<false>;

/// Options that change how GraphQL assist behaves.
#[derive(Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct GraphqlAssistConfiguration {
    /// Controls assist actions for GraphQL files.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("graphql-assist-enabled"), argument("true|false"))
    )]
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
