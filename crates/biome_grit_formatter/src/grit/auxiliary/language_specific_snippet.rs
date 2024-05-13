use crate::prelude::*;
use biome_grit_syntax::GritLanguageSpecificSnippet;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritLanguageSpecificSnippet;
impl FormatNodeRule<GritLanguageSpecificSnippet> for FormatGritLanguageSpecificSnippet {
    fn fmt_fields(
        &self,
        node: &GritLanguageSpecificSnippet,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
