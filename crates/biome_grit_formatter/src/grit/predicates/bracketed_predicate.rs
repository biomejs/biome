use crate::prelude::*;
use biome_grit_syntax::GritBracketedPredicate;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBracketedPredicate;
impl FormatNodeRule<GritBracketedPredicate> for FormatGritBracketedPredicate {
    fn fmt_fields(&self, node: &GritBracketedPredicate, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
