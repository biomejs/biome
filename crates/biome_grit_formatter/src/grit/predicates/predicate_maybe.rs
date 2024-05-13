use crate::prelude::*;
use biome_grit_syntax::GritPredicateMaybe;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateMaybe;
impl FormatNodeRule<GritPredicateMaybe> for FormatGritPredicateMaybe {
    fn fmt_fields(&self, node: &GritPredicateMaybe, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
