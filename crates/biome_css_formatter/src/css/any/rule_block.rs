//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssRuleBlock;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssRuleBlock;
impl FormatRule<AnyCssRuleBlock> for FormatAnyCssRuleBlock {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssRuleBlock, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssRuleBlock::CssBogusBlock(node) => node.format().fmt(f),
            AnyCssRuleBlock::CssRuleBlock(node) => node.format().fmt(f),
        }
    }
}
