//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssRuleListBlock;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssRuleListBlock;
impl FormatRule<AnyCssRuleListBlock> for FormatAnyCssRuleListBlock {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssRuleListBlock, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssRuleListBlock::CssRuleListBlock(node) => node.format().fmt(f),
            AnyCssRuleListBlock::CssBogusBlock(node) => node.format().fmt(f),
        }
    }
}
