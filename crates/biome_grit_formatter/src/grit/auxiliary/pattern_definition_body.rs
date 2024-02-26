use crate::prelude::*;
use biome_grit_syntax::GritPatternDefinitionBody;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternDefinitionBody;
impl FormatNodeRule<GritPatternDefinitionBody> for FormatGritPatternDefinitionBody {
    fn fmt_fields(
        &self,
        node: &GritPatternDefinitionBody,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
