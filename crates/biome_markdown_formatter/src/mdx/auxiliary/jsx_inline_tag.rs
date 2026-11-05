use crate::prelude::*;
use biome_markdown_syntax::MdxJsxInlineTag;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdxJsxInlineTag;
impl FormatNodeRule<MdxJsxInlineTag> for FormatMdxJsxInlineTag {
    fn fmt_fields(&self, node: &MdxJsxInlineTag, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
