use crate::prelude::*;
use biome_grit_syntax::GritPatternOrElse;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternOrElse;
impl FormatNodeRule<GritPatternOrElse> for FormatGritPatternOrElse {
    fn fmt_fields(&self, node: &GritPatternOrElse, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
