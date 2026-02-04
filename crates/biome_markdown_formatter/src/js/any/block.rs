//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_markdown_syntax::AnyLeafBlock;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyLeafBlock;
impl FormatRule<AnyLeafBlock> for FormatAnyLeafBlock {
    type Context = MarkdownFormatterContext;
    fn fmt(&self, node: &AnyLeafBlock, f: &mut MarkdownFormatter) -> FormatResult<()> {
        match node {
            AnyLeafBlock::AnyCodeBlock(node) => node.format().fmt(f),
            AnyLeafBlock::MdHeader(node) => node.format().fmt(f),
            AnyLeafBlock::MdHtmlBlock(node) => node.format().fmt(f),
            AnyLeafBlock::MdLinkBlock(node) => node.format().fmt(f),
            AnyLeafBlock::MdParagraph(node) => node.format().fmt(f),
            AnyLeafBlock::MdSetextHeader(node) => node.format().fmt(f),
            AnyLeafBlock::MdThematicBreakBlock(node) => node.format().fmt(f),
        }
    }
}
