use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlObjectField, GraphqlObjectFieldFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlObjectField;
impl FormatNodeRule<GraphqlObjectField> for FormatGraphqlObjectField {
    fn fmt_fields(&self, node: &GraphqlObjectField, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlObjectFieldFields {
            name,
            colon_token,
            value,
        } = node.as_fields();

        write!(
            f,
            [name.format(), colon_token.format(), space(), value.format()]
        )
    }
}
