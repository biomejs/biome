use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlVariableBinding, GraphqlVariableBindingFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlVariableBinding;
impl FormatNodeRule<GraphqlVariableBinding> for FormatGraphqlVariableBinding {
    fn fmt_fields(
        &self,
        node: &GraphqlVariableBinding,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlVariableBindingFields { dollar_token, name } = node.as_fields();
        write!(f, [dollar_token.format(), name.format()])
    }
}
