//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssPageAtRuleItem;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssPageAtRuleItem;
impl FormatRule<AnyCssPageAtRuleItem> for FormatAnyCssPageAtRuleItem {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssPageAtRuleItem, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssPageAtRuleItem::CssAtRule(node) => node.format().fmt(f),
            AnyCssPageAtRuleItem::CssDeclarationWithSemicolon(node) => node.format().fmt(f),
            AnyCssPageAtRuleItem::CssMarginAtRule(node) => node.format().fmt(f),
        }
    }
}
