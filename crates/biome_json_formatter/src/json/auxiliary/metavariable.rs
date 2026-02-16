use crate::prelude::*;
use crate::verbatim::format_json_verbatim_node;
use biome_json_syntax::JsonMetavariable;
use biome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonMetavariable;
impl FormatNodeRule<JsonMetavariable> for FormatJsonMetavariable {
    fn fmt_fields(&self, node: &JsonMetavariable, f: &mut JsonFormatter) -> FormatResult<()> {
        format_json_verbatim_node(node.syntax()).fmt(f)
    }
}
