use crate::prelude::*;
use biome_css_syntax::CssSyntaxComponentList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSyntaxComponentList;
impl FormatRule<CssSyntaxComponentList> for FormatCssSyntaxComponentList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssSyntaxComponentList, f: &mut CssFormatter) -> FormatResult<()> {
        todo!()
        // format_verbatim_node(node.syntax()).fmt(f)
    }
}
