use crate::PlainIndentStyle;
use biome_deserialize_macros::{Deserializable, Merge, Partial};
use biome_formatter::{BracketSpacing, IndentWidth, LineEnding, LineWidth, QuoteStyle};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Options applied to GraphQL files
#[derive(Clone, Default, Debug, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct GraphqlConfiguration {
    /// GraphQL formatter options
    #[partial(type, bpaf(external(partial_graphql_formatter), optional))]
    pub formatter: GraphqlFormatter,

    // GraphQL linter options
    #[partial(type, bpaf(external(partial_graphql_linter), optional))]
    pub linter: GraphqlLinter,
}

/// Options that changes how the GraphQL formatter behaves
#[derive(Clone, Debug, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct GraphqlFormatter {
    /// Control the formatter for GraphQL files.
    #[partial(bpaf(long("graphql-formatter-enabled"), argument("true|false"), optional))]
    pub enabled: Option<bool>,

    /// The indent style applied to GraphQL files.
    #[partial(bpaf(
        long("graphql-formatter-indent-style"),
        argument("tab|space"),
        optional
    ))]
    pub indent_style: Option<PlainIndentStyle>,

    /// The size of the indentation applied to GraphQL files. Default to 2.
    #[partial(bpaf(long("graphql-formatter-indent-width"), argument("NUMBER"), optional))]
    pub indent_width: Option<IndentWidth>,

    /// The type of line ending applied to GraphQL files.
    #[partial(bpaf(
        long("graphql-formatter-line-ending"),
        argument("lf|crlf|cr"),
        optional
    ))]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to GraphQL files. Defaults to 80.
    #[partial(bpaf(long("graphql-formatter-line-width"), argument("NUMBER"), optional))]
    pub line_width: Option<LineWidth>,

    /// The type of quotes used in GraphQL code. Defaults to double.
    #[partial(bpaf(
        long("graphql-formatter-quote-style"),
        argument("double|single"),
        optional
    ))]
    pub quote_style: Option<QuoteStyle>,

    // it's also a top-level configurable property.
    /// Whether to insert spaces around brackets in object literals. Defaults to true.
    #[partial(bpaf(long("bracket-spacing"), argument("true|false"), optional))]
    pub bracket_spacing: Option<BracketSpacing>,
}

impl Default for GraphqlFormatter {
    fn default() -> Self {
        Self {
            enabled: Some(false),
            indent_style: Default::default(),
            indent_width: Default::default(),
            line_ending: Default::default(),
            line_width: Default::default(),
            quote_style: Default::default(),
            bracket_spacing: Default::default(),
        }
    }
}

impl PartialGraphqlFormatter {
    pub fn get_formatter_configuration(&self) -> GraphqlFormatter {
        GraphqlFormatter {
            enabled: self.enabled,
            indent_style: self.indent_style,
            indent_width: self.indent_width,
            line_ending: self.line_ending,
            line_width: self.line_width,
            quote_style: self.quote_style,
            bracket_spacing: self.bracket_spacing,
        }
    }
}

/// Options that changes how the GraphQL linter behaves
#[derive(Clone, Debug, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct GraphqlLinter {
    /// Control the formatter for GraphQL files.
    #[partial(bpaf(long("graphql-linter-enabled"), argument("true|false"), optional))]
    pub enabled: Option<bool>,
}

impl Default for GraphqlLinter {
    fn default() -> Self {
        Self {
            enabled: Some(false),
        }
    }
}

impl PartialGraphqlLinter {
    pub fn get_linter_configuration(&self) -> GraphqlLinter {
        GraphqlLinter {
            enabled: self.enabled,
        }
    }
}

#[test]
fn default_graphql_formatter() {
    let graphql_configuration = GraphqlFormatter::default();

    assert_eq!(graphql_configuration.enabled, Some(false));
    assert_eq!(graphql_configuration.indent_style, None);
    assert_eq!(graphql_configuration.indent_width, None);
    assert_eq!(graphql_configuration.line_ending, None);
    assert_eq!(graphql_configuration.line_width, None);
    assert_eq!(graphql_configuration.quote_style, None);
}

#[test]
fn default_graphql_linter() {
    let graphql_configuration = GraphqlLinter::default();

    assert_eq!(graphql_configuration.enabled, Some(false));
}
