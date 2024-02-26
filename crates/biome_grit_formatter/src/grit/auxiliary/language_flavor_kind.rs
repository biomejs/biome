use crate::prelude::*;
use biome_grit_syntax::GritLanguageFlavorKind;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritLanguageFlavorKind;
impl FormatNodeRule<GritLanguageFlavorKind> for FormatGritLanguageFlavorKind {
    fn fmt_fields(&self, node: &GritLanguageFlavorKind, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
