use crate::prelude::*;
use biome_css_syntax::CssBracketedValueList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBracketedValueList;
impl FormatRule<CssBracketedValueList> for FormatCssBracketedValueList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssBracketedValueList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join_with(&space())
            .entries(node.iter().formatted())
            .finish()
    }
}
