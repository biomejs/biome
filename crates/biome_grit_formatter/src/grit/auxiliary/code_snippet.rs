use crate::prelude::*;
use biome_grit_syntax::GritCodeSnippet;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritCodeSnippet;
impl FormatNodeRule<GritCodeSnippet> for FormatGritCodeSnippet {
    fn fmt_fields(&self, node: &GritCodeSnippet, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
