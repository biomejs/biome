use crate::prelude::*;
use biome_markdown_syntax::MdFrontmatter;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdFrontmatter;
impl FormatNodeRule<MdFrontmatter> for FormatMdFrontmatter {
    fn fmt_fields(&self, node: &MdFrontmatter, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
