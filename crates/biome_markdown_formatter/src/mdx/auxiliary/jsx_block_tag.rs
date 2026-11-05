use crate::prelude::*;
use biome_markdown_syntax::MdxJsxBlockTag;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdxJsxBlockTag;
impl FormatNodeRule<MdxJsxBlockTag> for FormatMdxJsxBlockTag {
    fn fmt_fields(&self, node: &MdxJsxBlockTag, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
