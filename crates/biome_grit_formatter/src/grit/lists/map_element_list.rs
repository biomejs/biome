use crate::prelude::*;
use biome_grit_syntax::GritMapElementList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritMapElementList;
impl FormatRule<GritMapElementList> for FormatGritMapElementList {
    type Context = GritFormatContext;
    fn fmt(&self, node: &GritMapElementList, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
