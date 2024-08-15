use biome_formatter_test::TestFormatLanguage;
use biome_fs::BiomePath;
use biome_graphql_formatter::context::GraphqlFormatContext;
use biome_graphql_formatter::GraphqlFormatLanguage;
use biome_graphql_parser::parse_graphql;
use biome_graphql_syntax::{GraphqlFileSource, GraphqlLanguage};
use biome_parser::AnyParse;
use biome_service::{
    settings::{ServiceLanguage, Settings},
    workspace::DocumentFileSource,
};

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

    fn to_format_language(
        &self,
        settings: &Settings,
        file_source: &DocumentFileSource,
    ) -> Self::FormatLanguage {
        let language_settings = &settings.languages.graphql.formatter;
        let options = Self::ServiceLanguage::resolve_format_options(
            Some(&settings.formatter),
            Some(&settings.override_settings),
            Some(language_settings),
            &BiomePath::new(""),
            file_source,
        );
        GraphqlFormatLanguage::new(options)
    }
}
