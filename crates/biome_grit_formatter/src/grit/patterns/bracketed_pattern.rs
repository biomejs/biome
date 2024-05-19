use crate::prelude::*;
use biome_grit_syntax::GritBracketedPattern;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBracketedPattern;
impl FormatNodeRule<GritBracketedPattern> for FormatGritBracketedPattern {
    fn fmt_fields(&self, node: &GritBracketedPattern, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
