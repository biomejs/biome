use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{GfmStrikethrough, GfmStrikethroughFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGfmStrikethrough;
impl FormatNodeRule<GfmStrikethrough> for FormatGfmStrikethrough {
    fn fmt_fields(&self, node: &GfmStrikethrough, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let GfmStrikethroughFields {
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
