use crate::prelude::*;
use crate::shared::FormatMdxSnippet;
use biome_formatter::write;
use biome_markdown_syntax::{MdxEsmBlock, MdxEsmBlockFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdxEsmBlock;
impl FormatNodeRule<MdxEsmBlock> for FormatMdxEsmBlock {
    fn fmt_fields(&self, node: &MdxEsmBlock, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdxEsmBlockFields { content_token } = node.as_fields();
        let content_token = content_token?;

        write!(
            f,
            [FormatMdxSnippet {
                token: &content_token
            }]
        )
    }
}
