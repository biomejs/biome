use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{MdParagraph, MdParagraphFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdParagraph;
impl FormatNodeRule<MdParagraph> for FormatMdParagraph {
    fn fmt_fields(&self, node: &MdParagraph, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdParagraphFields { list, hard_line } = node.as_fields();
        write!(f, [list.format()])?;
        if let Some(hard_line) = hard_line {
            write!(f, [hard_line.format()])?;
        }
        Ok(())
    }
}
