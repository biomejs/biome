use crate::prelude::*;
use biome_grit_syntax::GritRewrite;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritRewrite;
impl FormatNodeRule<GritRewrite> for FormatGritRewrite {
    fn fmt_fields(&self, node: &GritRewrite, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
