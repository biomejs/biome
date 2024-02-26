use crate::prelude::*;
use biome_grit_syntax::GritRawBacktickSnippetLiteral;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritRawBacktickSnippetLiteral;
impl FormatNodeRule<GritRawBacktickSnippetLiteral> for FormatGritRawBacktickSnippetLiteral {
    fn fmt_fields(
        &self,
        node: &GritRawBacktickSnippetLiteral,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
