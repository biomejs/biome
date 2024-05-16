use crate::prelude::*;
use biome_grit_syntax::GritCurlyPredicateList;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritCurlyPredicateList;
impl FormatNodeRule<GritCurlyPredicateList> for FormatGritCurlyPredicateList {
    fn fmt_fields(&self, node: &GritCurlyPredicateList, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
