//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_markdown_syntax::AnyMdContainerBlock;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMdContainerBlock;
impl FormatRule<AnyMdContainerBlock> for FormatAnyMdContainerBlock {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &AnyMdContainerBlock, f: &mut MarkdownFormatter) -> FormatResult<()> {
        match node {
            AnyMdContainerBlock::MdBulletListItem(node) => node.format().fmt(f),
            AnyMdContainerBlock::MdOrderedListItem(node) => node.format().fmt(f),
            AnyMdContainerBlock::MdQuote(node) => node.format().fmt(f),
        }
    }
}
