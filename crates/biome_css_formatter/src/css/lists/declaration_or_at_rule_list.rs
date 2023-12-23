use crate::prelude::*;
use biome_css_syntax::CssDeclarationOrAtRuleList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationOrAtRuleList;
impl FormatRule<CssDeclarationOrAtRuleList> for FormatCssDeclarationOrAtRuleList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssDeclarationOrAtRuleList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
