use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlNameReference, GraphqlNameReferenceFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlNameReference;
impl FormatNodeRule<GraphqlNameReference> for FormatGraphqlNameReference {
    fn fmt_fields(
        &self,
        node: &GraphqlNameReference,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlNameReferenceFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
