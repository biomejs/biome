//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssPageAtRuleBlock;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssPageAtRuleBlock;
impl FormatRule<AnyCssPageAtRuleBlock> for FormatAnyCssPageAtRuleBlock {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssPageAtRuleBlock, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssPageAtRuleBlock::CssBogusBlock(node) => node.format().fmt(f),
            AnyCssPageAtRuleBlock::CssPageAtRuleBlock(node) => node.format().fmt(f),
        }
    }
}
