use crate::prelude::*;
use biome_grit_syntax::GritPatternArgList;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternArgList;
impl FormatNodeRule<GritPatternArgList> for FormatGritPatternArgList {
    fn fmt_fields(&self, node: &GritPatternArgList, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
