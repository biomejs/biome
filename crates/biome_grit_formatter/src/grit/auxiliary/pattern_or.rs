use crate::prelude::*;
use biome_grit_syntax::GritPatternOr;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternOr;
impl FormatNodeRule<GritPatternOr> for FormatGritPatternOr {
    fn fmt_fields(&self, node: &GritPatternOr, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
