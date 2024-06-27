use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlDescription, GraphqlDescriptionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlDescription;
impl FormatNodeRule<GraphqlDescription> for FormatGraphqlDescription {
    fn fmt_fields(&self, node: &GraphqlDescription, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlDescriptionFields {
            graphql_string_value,
        } = node.as_fields();

        write!(f, [graphql_string_value.format()])
    }
}
