//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_markdown_syntax::AnyMdBlock;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMdBlock;
impl FormatRule<AnyMdBlock> for FormatAnyMdBlock {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &AnyMdBlock, f: &mut MarkdownFormatter) -> FormatResult<()> {
        match node {
            AnyMdBlock::AnyMdContainerBlock(node) => node.format().fmt(f),
            AnyMdBlock::AnyMdLeafBlock(node) => node.format().fmt(f),
            AnyMdBlock::MdQuotePrefix(node) => node.format().fmt(f),
        }
    }
}
