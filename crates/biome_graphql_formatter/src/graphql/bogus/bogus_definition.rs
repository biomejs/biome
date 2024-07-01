use crate::FormatBogusNodeRule;
use biome_graphql_syntax::GraphqlBogusDefinition;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlBogusDefinition;
impl FormatBogusNodeRule<GraphqlBogusDefinition> for FormatGraphqlBogusDefinition {}
