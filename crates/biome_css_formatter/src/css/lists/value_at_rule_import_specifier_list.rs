use crate::prelude::*;
use biome_css_syntax::CssValueAtRuleImportSpecifierList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssValueAtRuleImportSpecifierList;
impl FormatRule<CssValueAtRuleImportSpecifierList> for FormatCssValueAtRuleImportSpecifierList {
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &CssValueAtRuleImportSpecifierList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
