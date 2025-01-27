use crate::prelude::*;
use crate::separated::FormatAstSeparatedListExtension;
use biome_formatter::separated::TrailingSeparator;
use biome_json_syntax::{JsonFileVariant, JsonMemberList};
use biome_rowan::{AstNode, AstSeparatedList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonMemberList;

impl FormatRule<JsonMemberList> for FormatJsonMemberList {
    type Context = JsonFormatContext;
    fn fmt(&self, node: &JsonMemberList, f: &mut JsonFormatter) -> FormatResult<()> {
        let file_source = f.options().file_source();
        let trailing_separator = if file_source.variant() == JsonFileVariant::Standard {
            TrailingSeparator::Omit
        } else {
            f.options().to_trailing_separator()
        };
        let mut join = f.join_nodes_with_soft_line();

        for (element, formatted) in node
            .elements()
            .zip(node.format_separated(",", trailing_separator))
        {
            join.entry(element.node()?.syntax(), &formatted);
        }

        join.finish()
    }
}
