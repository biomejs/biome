use crate::markdown::lists::block_list::FormatMdBlockListOptions;
use crate::prelude::*;
use crate::shared::TextPrintMode;
use biome_formatter::write;
use biome_markdown_syntax::{
    AnyMdBlock, AnyMdInline, AnyMdLeafBlock, MdDocument, MdDocumentFields,
};
use biome_rowan::AstNodeList;

fn content_ends_with_newline(value: &biome_markdown_syntax::MdBlockList) -> bool {
    let mut iter = value.iter();
    // Walk backwards past trailing MdNewline blocks to find the last content block.
    let last_content = loop {
        match iter.next_back() {
            Some(AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdNewline(_))) => {}
            other => break other,
        }
    };
    matches!(
        last_content,
        Some(AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdParagraph(ref p)))
            if p.list().iter().last().is_some_and(|item| matches!(
                item,
                AnyMdInline::MdTextual(ref t) if t.is_newline().unwrap_or(false)
            ))
    )
}

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
        let already_ends_with_newline = content_ends_with_newline(&value);

        write!(
            f,
            [
                value.format().with_options(FormatMdBlockListOptions {
                    paragraph_print_mode: TextPrintMode::Pristine,
                    trim: true
                }),
                format_removed(&eof_token?)
            ]
        )?;

        if !already_ends_with_newline {
            write!(f, [hard_line_break()])?;
        }

        Ok(())
    }
}
