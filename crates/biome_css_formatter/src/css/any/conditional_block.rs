//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssConditionalBlock;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssConditionalBlock;
impl FormatRule<AnyCssConditionalBlock> for FormatAnyCssConditionalBlock {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssConditionalBlock, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssConditionalBlock::CssBogusBlock(node) => node.format().fmt(f),
            AnyCssConditionalBlock::CssDeclarationOrRuleBlock(node) => node.format().fmt(f),
            AnyCssConditionalBlock::CssRuleBlock(node) => node.format().fmt(f),
        }
    }
}
