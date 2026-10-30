//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssControlBlock;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssControlBlock;
impl FormatRule<AnyCssControlBlock> for FormatAnyCssControlBlock {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssControlBlock, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssControlBlock::CssDeclarationOrRuleBlock(node) => node.format().fmt(f),
            AnyCssControlBlock::CssKeyframesBlock(node) => node.format().fmt(f),
        }
    }
}
