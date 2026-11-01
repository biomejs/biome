use crate::prelude::*;
use crate::shared::FormatMdxSnippet;
use biome_formatter::write;
use biome_markdown_syntax::{MdxJsxInlineTag, MdxJsxInlineTagFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdxJsxInlineTag;
impl FormatNodeRule<MdxJsxInlineTag> for FormatMdxJsxInlineTag {
    fn fmt_fields(&self, node: &MdxJsxInlineTag, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdxJsxInlineTagFields { value_token } = node.as_fields();
        let value_token = value_token?;

        write!(
            f,
            [FormatMdxSnippet {
                token: &value_token
            }]
        )
    }
}
