use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_graphql_syntax::{GraphqlUnionTypeDefinition, GraphqlUnionTypeDefinitionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlUnionTypeDefinition;
impl FormatNodeRule<GraphqlUnionTypeDefinition> for FormatGraphqlUnionTypeDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlUnionTypeDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlUnionTypeDefinitionFields {
            description,
            union_token,
            name,
            directives,
            union_members,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args!(
                format_with(|f| {
                    if let Some(description) = description.as_ref() {
                        write!(f, [description.format(), hard_line_break(),])?;
                    }
                    Ok(())
                }),
                group(&format_args!(
                    union_token.format(),
                    space(),
                    name.format(),
                    directives.format(),
                    union_members.format()
                ))
            ))]
        )
    }
}
