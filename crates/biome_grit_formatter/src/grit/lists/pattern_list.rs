use crate::prelude::*;
use biome_grit_syntax::GritPatternList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternList;
impl FormatRule<GritPatternList> for FormatGritPatternList {
    type Context = GritFormatContext;
    fn fmt(&self, node: &GritPatternList, f: &mut GritFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
