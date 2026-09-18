//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssRuleListItem;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssRuleListItem;
impl FormatRule<AnyCssRuleListItem> for FormatAnyCssRuleListItem {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssRuleListItem, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssRuleListItem::AnyCssRule(node) => node.format().fmt(f),
            AnyCssRuleListItem::ScssVariableDeclaration(node) => node.format().fmt(f),
        }
    }
}
