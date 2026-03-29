use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{AnyMdBlock, AnyMdLeafBlock, MdDocument, MdDocumentFields};

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

        // Skip leading MdNewline blocks so documents don't start with blank lines.
        // Trailing newlines are preserved for idempotency (the parser may emit a
        // trailing MdNewline from the paragraph's final newline on re-parse).
        let items: Vec<_> = value.iter().collect();
        let start = items
            .iter()
            .position(|item| !is_md_newline(item))
            .unwrap_or(items.len());

        for (i, item) in items.iter().enumerate() {
            if i < start
                && let AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdNewline(newline)) = item
            {
                f.context()
                    .comments()
                    .mark_suppression_checked(newline.syntax());
                write!(f, [format_removed(&newline.as_fields().value_token?)])?;
                continue;
            }
            write!(f, [item.format()])?;
        }

        write!(f, [format_removed(&eof_token?)])
    }
}

fn is_md_newline(block: &AnyMdBlock) -> bool {
    matches!(
        block,
        AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdNewline(_))
    )
}
