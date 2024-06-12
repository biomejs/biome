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
        let separator = space();
        let mut joiner = f.join_with(&separator);

        for formatted in node.format_separated(",") {
            joiner.entry(&formatted);
        }

        joiner.finish()
    }
}
