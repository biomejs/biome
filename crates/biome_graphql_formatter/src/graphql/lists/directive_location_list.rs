use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::GraphqlDirectiveLocationList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlDirectiveLocationList;
impl FormatRule<GraphqlDirectiveLocationList> for FormatGraphqlDirectiveLocationList {
    type Context = GraphqlFormatContext;
    fn fmt(
        &self,
        node: &GraphqlDirectiveLocationList,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        for (index, element) in node.elements().enumerate() {
            let node = element.node();

            if index != 0 {
                write!(f, [space()])?;
            }

            write!(f, [node.format()])?;

            let trailing_separator = element.trailing_separator()?;

            if let Some(token) = trailing_separator {
                write![f, [space(), token.format()]]?;
            }
        }

        Ok(())
    }
}
