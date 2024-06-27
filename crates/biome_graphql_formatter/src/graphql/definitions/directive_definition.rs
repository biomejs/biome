use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlDirectiveDefinition, GraphqlDirectiveDefinitionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlDirectiveDefinition;
impl FormatNodeRule<GraphqlDirectiveDefinition> for FormatGraphqlDirectiveDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlDirectiveDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlDirectiveDefinitionFields {
            description,
            directive_token,
            at_token,
            name,
            arguments,
            repeatable_token,
            on_token,
            bitwise_or_token,
            locations,
        } = node.as_fields();

        if let Some(description) = description {
            write!(f, [description.format(), hard_line_break(),])?;
        }

        if let Some(bitwise_or_token) = bitwise_or_token {
            write!(f, [format_removed(&bitwise_or_token)])?;
        }

        write!(
            f,
            [
                directive_token.format(),
                space(),
                at_token.format(),
                name.format(),
                arguments.format(),
                space(),
                repeatable_token.format(),
                space(),
                on_token.format(),
                space(),
                locations.format(),
            ]
        )
    }
}
