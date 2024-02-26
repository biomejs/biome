use crate::prelude::*;
use biome_grit_syntax::GritPredicateDefinition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateDefinition;
impl FormatNodeRule<GritPredicateDefinition> for FormatGritPredicateDefinition {
    fn fmt_fields(
        &self,
        node: &GritPredicateDefinition,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
