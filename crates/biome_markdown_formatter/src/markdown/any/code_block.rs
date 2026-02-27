//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_markdown_syntax::AnyMdCodeBlock;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMdCodeBlock;
impl FormatRule<AnyMdCodeBlock> for FormatAnyMdCodeBlock {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &AnyMdCodeBlock, f: &mut MarkdownFormatter) -> FormatResult<()> {
        match node {
            AnyMdCodeBlock::MdFencedCodeBlock(node) => node.format().fmt(f),
            AnyMdCodeBlock::MdIndentCodeBlock(node) => node.format().fmt(f),
        }
    }
}
