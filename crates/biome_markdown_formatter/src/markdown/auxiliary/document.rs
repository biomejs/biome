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

        write!(f, [value.format(), format_removed(&eof_token?)])
    }
}
