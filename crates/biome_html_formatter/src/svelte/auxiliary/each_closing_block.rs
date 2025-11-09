use crate::prelude::*;
use biome_html_syntax::SvelteEachClosingBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteEachClosingBlock;
impl FormatNodeRule<SvelteEachClosingBlock> for FormatSvelteEachClosingBlock {
    fn fmt_fields(&self, node: &SvelteEachClosingBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
