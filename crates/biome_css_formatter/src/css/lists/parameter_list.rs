use crate::prelude::*;
use biome_css_syntax::CssParameterList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssParameterList;
impl FormatRule<CssParameterList> for FormatCssParameterList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssParameterList, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
