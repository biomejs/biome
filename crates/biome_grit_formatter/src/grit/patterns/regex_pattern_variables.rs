use crate::prelude::*;
use biome_grit_syntax::GritRegexPatternVariables;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritRegexPatternVariables;
impl FormatNodeRule<GritRegexPatternVariables> for FormatGritRegexPatternVariables {
    fn fmt_fields(
        &self,
        node: &GritRegexPatternVariables,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
