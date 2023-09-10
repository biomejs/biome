use crate::prelude::*;
use crate::separated::FormatAstSeparatedListExtension;
use biome_rowan::{AstNode, AstSeparatedList};
use rome_json_syntax::JsonMemberList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonMemberList;

impl FormatRule<JsonMemberList> for FormatJsonMemberList {
    type Context = JsonFormatContext;
    fn fmt(&self, node: &JsonMemberList, f: &mut JsonFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_soft_line();

        for (element, formatted) in node.elements().zip(node.format_separated(",")) {
            join.entry(element.node()?.syntax(), &formatted);
        }

        join.finish()
    }
}
