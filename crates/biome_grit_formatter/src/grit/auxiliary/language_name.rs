use crate::prelude::*;
use biome_grit_syntax::GritLanguageName;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritLanguageName;
impl FormatNodeRule<GritLanguageName> for FormatGritLanguageName {
    fn fmt_fields(&self, node: &GritLanguageName, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
