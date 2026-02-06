//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_markdown_syntax::AnyMdLeafBlock;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMdLeafBlock;
impl FormatRule<AnyMdLeafBlock> for FormatAnyMdLeafBlock {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &AnyMdLeafBlock, f: &mut MarkdownFormatter) -> FormatResult<()> {
        match node {
            AnyMdLeafBlock::AnyMdCodeBlock(node) => node.format().fmt(f),
            AnyMdLeafBlock::MdHeader(node) => node.format().fmt(f),
            AnyMdLeafBlock::MdHtmlBlock(node) => node.format().fmt(f),
            AnyMdLeafBlock::MdLinkBlock(node) => node.format().fmt(f),
            AnyMdLeafBlock::MdLinkReferenceDefinition(node) => node.format().fmt(f),
            AnyMdLeafBlock::MdNewline(node) => node.format().fmt(f),
            AnyMdLeafBlock::MdParagraph(node) => node.format().fmt(f),
            AnyMdLeafBlock::MdSetextHeader(node) => node.format().fmt(f),
            AnyMdLeafBlock::MdThematicBreakBlock(node) => node.format().fmt(f),
        }
    }
}
