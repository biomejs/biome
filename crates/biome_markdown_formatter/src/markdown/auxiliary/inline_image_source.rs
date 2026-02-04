use crate::prelude::*;
use biome_markdown_syntax::MdInlineImageSource;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineImageSource;
impl FormatNodeRule<MdInlineImageSource> for FormatMdInlineImageSource {
    fn fmt_fields(
        &self,
        node: &MdInlineImageSource,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
