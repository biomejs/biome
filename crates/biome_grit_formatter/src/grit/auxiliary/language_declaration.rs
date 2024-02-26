use crate::prelude::*;
use biome_grit_syntax::GritLanguageDeclaration;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritLanguageDeclaration;
impl FormatNodeRule<GritLanguageDeclaration> for FormatGritLanguageDeclaration {
    fn fmt_fields(
        &self,
        node: &GritLanguageDeclaration,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
