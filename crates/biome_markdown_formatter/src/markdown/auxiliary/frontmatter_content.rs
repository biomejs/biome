use crate::prelude::*;
use biome_markdown_syntax::MdFrontmatterContent;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdFrontmatterContent;
impl FormatNodeRule<MdFrontmatterContent> for FormatMdFrontmatterContent {
    fn fmt_fields(
        &self,
        node: &MdFrontmatterContent,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
