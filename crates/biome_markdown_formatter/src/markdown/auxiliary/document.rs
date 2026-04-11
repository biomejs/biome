use crate::markdown::lists::block_list::FormatMdBlockListOptions;
use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{MdDocument, MdDocumentFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdDocument;
impl FormatNodeRule<MdDocument> for FormatMdDocument {
    fn fmt_fields(&self, node: &MdDocument, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdDocumentFields {
            bom_token,
            value,
            eof_token,
        } = node.as_fields();

        if let Some(bom) = bom_token {
            write!(f, [bom.format()])?;
        }

        write!(
            f,
            [
                value
                    .format()
                    .with_options(FormatMdBlockListOptions { trim: true }),
                format_removed(&eof_token?)
            ]
        )?;

        // when trimming, we remove the last newline, so we add it back here
        write!(f, [hard_line_break()])
    }
}
