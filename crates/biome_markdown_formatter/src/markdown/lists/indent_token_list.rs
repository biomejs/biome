use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{MdHeader, MdIndentTokenList, MdListMarkerPrefix};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdIndentTokenList;
impl FormatRule<MdIndentTokenList> for FormatMdIndentTokenList {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &MdIndentTokenList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let inside_list_prefix = node
            .syntax()
            .parent()
            .is_some_and(|node| MdListMarkerPrefix::can_cast(node.kind()));

        let inside_header = node
            .syntax()
            .parent()
            .is_some_and(|node| MdHeader::can_cast(node.kind()));

        // inside lists, alignment is handled by alignments
        if inside_list_prefix || inside_header {
            for token in node.iter() {
                f.context().comments().is_suppressed(token.syntax());
                write!(f, [format_removed(&token.md_indent_char_token()?)])?;
            }
            Ok(())
        } else {
            f.join().entries(node.iter().formatted()).finish()
        }
    }
}
