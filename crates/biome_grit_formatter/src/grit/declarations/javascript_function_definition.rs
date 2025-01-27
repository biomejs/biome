use crate::prelude::*;
use biome_grit_syntax::GritJavascriptFunctionDefinition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritJavascriptFunctionDefinition;
impl FormatNodeRule<GritJavascriptFunctionDefinition> for FormatGritJavascriptFunctionDefinition {
    fn fmt_fields(
        &self,
        node: &GritJavascriptFunctionDefinition,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
