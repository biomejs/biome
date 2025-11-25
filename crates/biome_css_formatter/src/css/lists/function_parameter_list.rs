use crate::prelude::*;
use biome_css_syntax::CssFunctionParameterList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFunctionParameterList;
impl FormatRule<CssFunctionParameterList> for FormatCssFunctionParameterList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssFunctionParameterList, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
