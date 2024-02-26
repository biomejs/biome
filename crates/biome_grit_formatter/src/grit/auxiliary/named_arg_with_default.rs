use crate::prelude::*;
use biome_grit_syntax::GritNamedArgWithDefault;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritNamedArgWithDefault;
impl FormatNodeRule<GritNamedArgWithDefault> for FormatGritNamedArgWithDefault {
    fn fmt_fields(
        &self,
        node: &GritNamedArgWithDefault,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
