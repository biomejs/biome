use crate::prelude::*;
use biome_markdown_syntax::MdBullet;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdBullet;
impl FormatNodeRule<MdBullet> for FormatMdBullet {
    fn fmt_fields(&self, node: &MdBullet, f: &mut MarkdownFormatter) -> FormatResult<()> {
        debug_assert!(
            false,
            "The bullet should be me formatted by the FormatMdBulletList"
        );
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
