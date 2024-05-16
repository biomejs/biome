use crate::prelude::*;
use biome_grit_syntax::GritFiles;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritFiles;
impl FormatNodeRule<GritFiles> for FormatGritFiles {
    fn fmt_fields(&self, node: &GritFiles, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
