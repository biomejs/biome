use crate::prelude::*;
use biome_css_syntax::CssIfBranch;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfBranch;
impl FormatNodeRule<CssIfBranch> for FormatCssIfBranch {
    fn fmt_fields(&self, node: &CssIfBranch, f: &mut CssFormatter) -> FormatResult<()> {
        todo!()
        // format_verbatim_node(node.syntax()).fmt(f)
    }
}
