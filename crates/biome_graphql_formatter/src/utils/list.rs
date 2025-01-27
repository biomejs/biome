use crate::context::GraphqlFormatContext;
use crate::prelude::*;
use crate::GraphqlFormatter;
use biome_formatter::prelude::{soft_line_break_or_space, space};
use biome_formatter::write;
use biome_formatter::FormatResult;
use biome_graphql_syntax::GraphqlLanguage;
use biome_rowan::{AstNode, AstSeparatedList};

pub(crate) fn write_interface_like_list<N, I>(
    node: &N,
    f: &mut GraphqlFormatter,
) -> FormatResult<()>
where
    N: AstSeparatedList<Language = GraphqlLanguage, Node = I>,
    I: AstNode<Language = GraphqlLanguage> + AsFormat<GraphqlFormatContext>,
{
    for (index, element) in node.elements().enumerate() {
        let node = element.node();

        if index != 0 {
            if node.is_ok_and(|node| node.syntax().has_leading_comments()) {
                write!(f, [soft_line_break_or_space()])?;
            } else {
                write!(f, [space()])?;
            }
        }

        write!(f, [node.format()])?;

        let trailing_separator = element.trailing_separator()?;

        if let Some(token) = trailing_separator {
            write![f, [space(), token.format()]]?;
        }
    }

    Ok(())
}
