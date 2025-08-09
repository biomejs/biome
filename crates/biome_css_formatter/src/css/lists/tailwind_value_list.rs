use crate::prelude::*;
use biome_css_syntax::CssTailwindValueList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssTailwindValueList;
impl FormatRule<CssTailwindValueList> for FormatCssTailwindValueList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssTailwindValueList, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
