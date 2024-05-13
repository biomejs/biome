use crate::prelude::*;
use biome_grit_syntax::GritRegexLiteral;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritRegexLiteral;
impl FormatNodeRule<GritRegexLiteral> for FormatGritRegexLiteral {
    fn fmt_fields(&self, node: &GritRegexLiteral, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
