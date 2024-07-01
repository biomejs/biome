use crate::FormatBogusNodeRule;
use biome_graphql_syntax::GraphqlBogus;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlBogus;
impl FormatBogusNodeRule<GraphqlBogus> for FormatGraphqlBogus {}
