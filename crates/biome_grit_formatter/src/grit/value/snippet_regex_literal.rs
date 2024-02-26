use crate::prelude::*;
use biome_grit_syntax::GritSnippetRegexLiteral;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritSnippetRegexLiteral;
impl FormatNodeRule<GritSnippetRegexLiteral> for FormatGritSnippetRegexLiteral {
    fn fmt_fields(
        &self,
        node: &GritSnippetRegexLiteral,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
