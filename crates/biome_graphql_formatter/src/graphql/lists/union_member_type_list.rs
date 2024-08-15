use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlLanguage, GraphqlNameReference, GraphqlUnionMemberTypeList};
use biome_rowan::AstSeparatedElement;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlUnionMemberTypeList;
impl FormatRule<GraphqlUnionMemberTypeList> for FormatGraphqlUnionMemberTypeList {
    type Context = GraphqlFormatContext;
    fn fmt(&self, node: &GraphqlUnionMemberTypeList, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let last_index = node.len().saturating_sub(1);

        f.join_with(space())
            .entries(
                node.elements()
                    .enumerate()
                    .map(|(index, item)| FormatTypeVariant {
                        last: index == last_index,
                        element: item,
                    }),
            )
            .finish()
    }
}

pub struct FormatTypeVariant {
    last: bool,
    element: AstSeparatedElement<GraphqlLanguage, GraphqlNameReference>,
}

impl Format<GraphqlFormatContext> for FormatTypeVariant {
    fn fmt(&self, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let separator = self.element.trailing_separator()?;
        let node = self.element.node()?;

        write!(f, [node.format()])?;

        if let Some(token) = separator {
            if self.last {
                write!(f, [format_removed(token)])?;
            } else {
                write![f, [soft_line_break_or_space(), token.format()]]?;
            }
        }

        Ok(())
    }
}
