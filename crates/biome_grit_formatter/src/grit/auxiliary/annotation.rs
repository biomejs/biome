use crate::prelude::*;
use biome_grit_syntax::GritAnnotation;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritAnnotation;
impl FormatNodeRule<GritAnnotation> for FormatGritAnnotation {
    fn fmt_fields(&self, node: &GritAnnotation, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
