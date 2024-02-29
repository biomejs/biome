use crate::prelude::*;
use crate::separated::FormatAstSeparatedListExtension;
use biome_json_syntax::JsonMemberList;
use biome_rowan::{AstNode, AstSeparatedList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonMemberList;

impl FormatRule<JsonMemberList> for FormatJsonMemberList {
    type Context = JsonFormatContext;
    fn fmt(&self, node: &JsonMemberList, f: &mut JsonFormatter) -> FormatResult<()> {
        let options = f.options();
        let allow_trailing_separator = options.allows_trailing_separator();
        let mut join = f.join_nodes_with_soft_line();

        for (element, formatted) in node
            .elements()
            .zip(node.format_separated(",", allow_trailing_separator))
        {
            join.entry(element.node()?.syntax(), &formatted);
        }

        join.finish()
    }
}
