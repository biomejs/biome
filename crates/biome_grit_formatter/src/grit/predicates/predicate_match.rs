use crate::prelude::*;
use biome_grit_syntax::GritPredicateMatch;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateMatch;
impl FormatNodeRule<GritPredicateMatch> for FormatGritPredicateMatch {
    fn fmt_fields(&self, node: &GritPredicateMatch, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
