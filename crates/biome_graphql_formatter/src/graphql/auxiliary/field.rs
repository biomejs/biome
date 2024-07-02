use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlField, GraphqlFieldFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlField;
impl FormatNodeRule<GraphqlField> for FormatGraphqlField {
    fn fmt_fields(&self, node: &GraphqlField, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlFieldFields {
            alias,
            name,
            arguments,
            directives,
            selection_set,
        } = node.as_fields();

        write!(
            f,
            [
                alias.format(),
                name.format(),
                arguments.format(),
                directives.format(),
            ]
        )?;

        if let Some(selection_set) = selection_set {
            write!(f, [space(), selection_set.format()])?;
        }

        Ok(())
    }
}
