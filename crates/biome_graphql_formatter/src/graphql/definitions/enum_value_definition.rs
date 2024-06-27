use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlEnumValueDefinition, GraphqlEnumValueDefinitionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlEnumValueDefinition;
impl FormatNodeRule<GraphqlEnumValueDefinition> for FormatGraphqlEnumValueDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlEnumValueDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlEnumValueDefinitionFields {
            description,
            value,
            directives,
        } = node.as_fields();

        if let Some(description) = description {
            write!(f, [description.format(), hard_line_break(),])?;
        }

        write!(f, [value.format(), directives.format(),])
    }
}
