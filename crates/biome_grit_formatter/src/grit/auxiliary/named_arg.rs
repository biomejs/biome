use crate::prelude::*;
use biome_grit_syntax::GritNamedArg;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritNamedArg;
impl FormatNodeRule<GritNamedArg> for FormatGritNamedArg {
    fn fmt_fields(&self, node: &GritNamedArg, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
