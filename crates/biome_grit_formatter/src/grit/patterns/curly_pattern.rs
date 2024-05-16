use crate::prelude::*;
use biome_grit_syntax::GritCurlyPattern;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritCurlyPattern;
impl FormatNodeRule<GritCurlyPattern> for FormatGritCurlyPattern {
    fn fmt_fields(&self, node: &GritCurlyPattern, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
