use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlNamedType, GraphqlNamedTypeFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlNamedType;
impl FormatNodeRule<GraphqlNamedType> for FormatGraphqlNamedType {
    fn fmt_fields(&self, node: &GraphqlNamedType, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlNamedTypeFields { name } = node.as_fields();

        write!(f, [name.format()])
    }
}
