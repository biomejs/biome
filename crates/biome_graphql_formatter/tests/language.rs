use biome_formatter_test::TestFormatLanguage;
use biome_fs::BiomePath;
use biome_graphql_formatter::GraphqlFormatLanguage;
use biome_graphql_formatter::context::GraphqlFormatContext;
use biome_graphql_parser::parse_graphql;
use biome_graphql_syntax::GraphqlLanguage;
use biome_languages::{DocumentFileSource, GraphqlFileSource};
use biome_parser::{AnyParse, NodeParse};
use biome_service::settings::Settings;

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

        NodeParse::new(parse.syntax().as_send().unwrap(), parse.into_diagnostics()).into()
    }

    fn to_format_language(
        &self,
        settings: &Settings,
        file_source: &DocumentFileSource,
    ) -> Self::FormatLanguage {
        let path = BiomePath::new("");
        let options = settings.format_options::<Self::ServiceLanguage>(
            &settings.matching_override_indices(path.as_path()),
            file_source,
        );
        GraphqlFormatLanguage::new(options)
    }
}
