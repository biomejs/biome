use crate::prelude::*;
use biome_grit_syntax::BracketedGritPredicate;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatBracketedGritPredicate;
impl FormatNodeRule<BracketedGritPredicate> for FormatBracketedGritPredicate {
    fn fmt_fields(&self, node: &BracketedGritPredicate, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
