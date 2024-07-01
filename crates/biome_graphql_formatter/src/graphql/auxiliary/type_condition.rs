use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlTypeCondition, GraphqlTypeConditionFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlTypeCondition;
impl FormatNodeRule<GraphqlTypeCondition> for FormatGraphqlTypeCondition {
    fn fmt_fields(
        &self,
        node: &GraphqlTypeCondition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlTypeConditionFields { on_token, ty } = node.as_fields();

        write!(f, [on_token.format(), space(), ty.format()])
    }
}
