use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{MdHtmlContent, MdHtmlContentFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdHtmlContent;
impl FormatNodeRule<MdHtmlContent> for FormatMdHtmlContent {
    fn fmt_fields(&self, node: &MdHtmlContent, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdHtmlContentFields { value_token } = node.as_fields();
        let value_token = value_token?;

        // TODO: update this node once embedding of markdown in supported
        write!(f, [value_token.format()])
    }
}
