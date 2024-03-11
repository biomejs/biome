//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssDeclarationOrRuleBlock;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssDeclarationOrRuleBlock;
impl FormatRule<AnyCssDeclarationOrRuleBlock> for FormatAnyCssDeclarationOrRuleBlock {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssDeclarationOrRuleBlock, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssDeclarationOrRuleBlock::CssBogusBlock(node) => node.format().fmt(f),
            AnyCssDeclarationOrRuleBlock::CssDeclarationOrRuleBlock(node) => node.format().fmt(f),
        }
    }
}
