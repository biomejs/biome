use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlNameBinding, GraphqlNameBindingFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlNameBinding;
impl FormatNodeRule<GraphqlNameBinding> for FormatGraphqlNameBinding {
    fn fmt_fields(&self, node: &GraphqlNameBinding, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlNameBindingFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
