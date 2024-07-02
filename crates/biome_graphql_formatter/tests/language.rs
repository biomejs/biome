use biome_formatter::{FormatResult, Formatted, Printed};
use biome_formatter_test::TestFormatLanguage;
use biome_graphql_formatter::context::{GraphqlFormatContext, GraphqlFormatOptions};
use biome_graphql_formatter::{format_node, format_range, GraphqlFormatLanguage};
use biome_graphql_parser::parse_graphql;
use biome_graphql_syntax::{GraphqlFileSource, GraphqlLanguage};
use biome_parser::AnyParse;
use biome_rowan::{SyntaxNode, TextRange};
use biome_service::settings::{ServiceLanguage, Settings};

#[derive(Default)]
pub struct GraphqlTestFormatLanguage {
    _source_type: GraphqlFileSource,
}

impl TestFormatLanguage for GraphqlTestFormatLanguage {
    type ServiceLanguage = GraphqlLanguage;
    type Context = GraphqlFormatContext;
    type FormatLanguage = GraphqlFormatLanguage;

    fn parse(&self, text: &str) -> AnyParse {
        let parse = parse_graphql(text);

        AnyParse::new(parse.syntax().as_send().unwrap(), parse.into_diagnostics())
    }

    fn to_language_settings<'a>(
        &self,
        settings: &'a Settings,
    ) -> &'a <Self::ServiceLanguage as ServiceLanguage>::FormatterSettings {
        &settings.languages.graphql.formatter
    }

    fn format_node(
        &self,
        options: <Self::ServiceLanguage as ServiceLanguage>::FormatOptions,
        node: &SyntaxNode<Self::ServiceLanguage>,
    ) -> FormatResult<Formatted<Self::Context>> {
        format_node(options, node)
    }

    fn format_range(
        &self,
        options: <Self::ServiceLanguage as ServiceLanguage>::FormatOptions,
        node: &SyntaxNode<Self::ServiceLanguage>,
        range: TextRange,
    ) -> FormatResult<Printed> {
        format_range(options, node, range)
    }

    fn default_options(&self) -> <Self::ServiceLanguage as ServiceLanguage>::FormatOptions {
        GraphqlFormatOptions::default()
    }
}
