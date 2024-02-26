use crate::prelude::*;
use biome_grit_syntax::GritVariableList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritVariableList;
impl FormatRule<GritVariableList> for FormatGritVariableList {
    type Context = GritFormatContext;
    fn fmt(&self, node: &GritVariableList, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
