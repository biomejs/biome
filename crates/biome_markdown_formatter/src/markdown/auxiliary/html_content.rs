use crate::FormatEmbedded;
use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{MdHtmlBlock, MdHtmlContent, MdHtmlContentFields, MdRoot};
use biome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdHtmlContent;
impl FormatNodeRule<MdHtmlContent> for FormatMdHtmlContent {
    fn fmt_fields(&self, node: &MdHtmlContent, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdHtmlContentFields { value_token } = node.as_fields();
        let value_token = value_token?;

        let range = value_token.text_range();
        if f.context().is_embedded_node_range(range) && is_unindented_document_block(node) {
            let embedded = FormatEmbedded::new(range, &value_token.format(), f)?;
            write!(f, [embedded])
        } else {
            write!(f, [value_token.format()])
        }
    }
}

/// Returns `true` when the HTML block holding `content` starts at the first
/// column of the document, outside of lists and quotes.
///
/// The lines of other HTML blocks carry the indentation of their block, which
/// the formatted HTML would lose after its first line.
fn is_unindented_document_block(content: &MdHtmlContent) -> bool {
    let Some(block) = content.parent::<MdHtmlBlock>() else {
        return false;
    };
    block.indent().is_empty()
        && block
            .syntax()
            .grand_parent()
            .is_some_and(|node| MdRoot::can_cast(node.kind()))
}
