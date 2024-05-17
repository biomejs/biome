use crate::prelude::*;
use biome_grit_syntax::GritAssignmentAsPattern;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritAssignmentAsPattern;
impl FormatNodeRule<GritAssignmentAsPattern> for FormatGritAssignmentAsPattern {
    fn fmt_fields(
        &self,
        node: &GritAssignmentAsPattern,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
