use crate::prelude::*;
use biome_css_syntax::CssSelectorList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSelectorList;
impl FormatRule<CssSelectorList> for FormatCssSelectorList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssSelectorList, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
