use crate::prelude::*;
use biome_html_syntax::SvelteEachOpeningBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteEachOpeningBlock;
impl FormatNodeRule<SvelteEachOpeningBlock> for FormatSvelteEachOpeningBlock {
    fn fmt_fields(&self, node: &SvelteEachOpeningBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
