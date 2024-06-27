use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlInlineFragment, GraphqlInlineFragmentFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlInlineFragment;
impl FormatNodeRule<GraphqlInlineFragment> for FormatGraphqlInlineFragment {
    fn fmt_fields(
        &self,
        node: &GraphqlInlineFragment,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlInlineFragmentFields {
            dotdotdot_token,
            type_condition,
            directives,
            selection_set,
        } = node.as_fields();

        write!(f, [dotdotdot_token.format()])?;

        if let Some(type_condition) = type_condition {
            write!(f, [space(), type_condition.format()])?;
        }

        write!(f, [directives.format(), space(), selection_set.format(),])
    }
}
