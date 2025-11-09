use crate::prelude::*;
use biome_html_syntax::SvelteEachBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteEachBlock;
impl FormatNodeRule<SvelteEachBlock> for FormatSvelteEachBlock {
    fn fmt_fields(&self, node: &SvelteEachBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
