use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_graphql_syntax::{GraphqlDirectiveList, GraphqlSyntaxKind};
use biome_rowan::SyntaxNodeOptionExt;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlDirectiveList;
impl FormatRule<GraphqlDirectiveList> for FormatGraphqlDirectiveList {
    type Context = GraphqlFormatContext;
    fn fmt(&self, node: &GraphqlDirectiveList, f: &mut GraphqlFormatter) -> FormatResult<()> {
        if node.len() == 0 {
            return Ok(());
        }

        let list = format_once(|f| {
            f.join_with(soft_line_break_or_space())
                .entries(node.iter().formatted())
                .finish()
        });

        if matches!(
            node.syntax().parent().kind(),
            Some(
                GraphqlSyntaxKind::GRAPHQL_FRAGMENT_DEFINITION
                    | GraphqlSyntaxKind::GRAPHQL_OPERATION_DEFINITION
            )
        ) {
            write!(
                f,
                [group(&format_args!(&soft_line_break_or_space(), &list))]
            )
        } else {
            write!(f, [space(), group(&soft_block_indent(&list))])
        }
    }
}
