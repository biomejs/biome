use crate::prelude::*;
use biome_grit_syntax::GritMapElement;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritMapElement;
impl FormatNodeRule<GritMapElement> for FormatGritMapElement {
    fn fmt_fields(&self, node: &GritMapElement, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
