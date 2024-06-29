use crate::FormatBogusNodeRule;
use biome_graphql_syntax::GraphqlBogusType;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlBogusType;
impl FormatBogusNodeRule<GraphqlBogusType> for FormatGraphqlBogusType {}
