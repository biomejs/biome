use crate::prelude::*;
use biome_grit_syntax::GritLanguageFlavor;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritLanguageFlavor;
impl FormatNodeRule<GritLanguageFlavor> for FormatGritLanguageFlavor {
    fn fmt_fields(&self, node: &GritLanguageFlavor, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
