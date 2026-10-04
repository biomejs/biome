//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssControlBlock;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssControlBlock;
impl FormatRule<AnyScssControlBlock> for FormatAnyScssControlBlock {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyScssControlBlock, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyScssControlBlock::CssDeclarationOrRuleBlock(node) => node.format().fmt(f),
            AnyScssControlBlock::CssKeyframesBlock(node) => node.format().fmt(f),
        }
    }
}
