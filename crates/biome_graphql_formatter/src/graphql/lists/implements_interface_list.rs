use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::GraphqlImplementsInterfaceList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlImplementsInterfaceList;
impl FormatRule<GraphqlImplementsInterfaceList> for FormatGraphqlImplementsInterfaceList {
    type Context = GraphqlFormatContext;
    fn fmt(
        &self,
        node: &GraphqlImplementsInterfaceList,
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
