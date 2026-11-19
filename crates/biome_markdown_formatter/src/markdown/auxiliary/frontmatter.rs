use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{MdFrontmatter, MdFrontmatterFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdFrontmatter;
impl FormatNodeRule<MdFrontmatter> for FormatMdFrontmatter {
    fn fmt_fields(&self, node: &MdFrontmatter, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdFrontmatterFields {
            l_fence_token,
            content,
            r_fence_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_fence_token.format(),
                content.format(),
                r_fence_token.format()
            ]
        )
    }
}
