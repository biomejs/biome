//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssStartingStyleBlock;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssStartingStyleBlock;
impl FormatRule<AnyCssStartingStyleBlock> for FormatAnyCssStartingStyleBlock {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssStartingStyleBlock, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssStartingStyleBlock::CssBogusBlock(node) => node.format().fmt(f),
            AnyCssStartingStyleBlock::CssDeclarationListBlock(node) => node.format().fmt(f),
            AnyCssStartingStyleBlock::CssRuleListBlock(node) => node.format().fmt(f),
        }
    }
}
