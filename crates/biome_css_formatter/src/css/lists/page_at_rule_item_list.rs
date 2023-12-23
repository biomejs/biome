use crate::prelude::*;
use biome_css_syntax::CssPageAtRuleItemList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPageAtRuleItemList;
impl FormatRule<CssPageAtRuleItemList> for FormatCssPageAtRuleItemList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssPageAtRuleItemList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
