use crate::prelude::*;
use biome_css_syntax::CssMediaQueryList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaQueryList;
impl FormatRule<CssMediaQueryList> for FormatCssMediaQueryList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssMediaQueryList, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
