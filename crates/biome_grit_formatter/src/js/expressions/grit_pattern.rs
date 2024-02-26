use crate::prelude::*;
use biome_grit_syntax::BracketedGritPattern;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatBracketedGritPattern;
impl FormatNodeRule<BracketedGritPattern> for FormatBracketedGritPattern {
    fn fmt_fields(&self, node: &BracketedGritPattern, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
