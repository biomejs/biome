use crate::prelude::*;
use biome_css_syntax::CssIfBranchList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfBranchList;
impl FormatRule<CssIfBranchList> for FormatCssIfBranchList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssIfBranchList, f: &mut CssFormatter) -> FormatResult<()> {
        todo!()
        // format_verbatim_node(node.syntax()).fmt(f)
    }
}
