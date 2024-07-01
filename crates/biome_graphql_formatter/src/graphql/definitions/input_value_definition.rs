use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlInputValueDefinition, GraphqlInputValueDefinitionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlInputValueDefinition;
impl FormatNodeRule<GraphqlInputValueDefinition> for FormatGraphqlInputValueDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlInputValueDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlInputValueDefinitionFields {
            description,
            name,
            colon_token,
            ty,
            default,
            directives,
        } = node.as_fields();

        if let Some(description) = description {
            write!(f, [description.format(), soft_line_break_or_space(),])?;
        }

        write!(
            f,
            [
                name.format(),
                colon_token.format(),
                space(),
                ty.format(),
                default.format(),
                directives.format(),
            ]
        )
    }
}
