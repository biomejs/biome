use crate::prelude::*;
use biome_grit_syntax::GritBacktickSnippetLiteral;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBacktickSnippetLiteral;
impl FormatNodeRule<GritBacktickSnippetLiteral> for FormatGritBacktickSnippetLiteral {
    fn fmt_fields(
        &self,
        node: &GritBacktickSnippetLiteral,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
