use crate::prelude::*;
use biome_grit_syntax::GritRegexPattern;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritRegexPattern;
impl FormatNodeRule<GritRegexPattern> for FormatGritRegexPattern {
    fn fmt_fields(&self, node: &GritRegexPattern, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
