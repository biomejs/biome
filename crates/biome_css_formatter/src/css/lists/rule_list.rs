use crate::prelude::*;
use biome_css_syntax::CssRuleList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRuleList;
impl FormatRule<CssRuleList> for FormatCssRuleList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssRuleList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
