use crate::prelude::*;
use biome_grit_syntax::GritPatternWhere;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternWhere;
impl FormatNodeRule<GritPatternWhere> for FormatGritPatternWhere {
    fn fmt_fields(&self, node: &GritPatternWhere, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
