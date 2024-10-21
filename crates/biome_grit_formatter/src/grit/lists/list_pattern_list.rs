use crate::prelude::*;
use biome_grit_syntax::GritListPatternList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritListPatternList;
impl FormatRule<GritListPatternList> for FormatGritListPatternList {
    type Context = GritFormatContext;
    fn fmt(&self, node: &GritListPatternList, f: &mut GritFormatter) -> FormatResult<()> {
        let separator = soft_line_break_or_space();
        let mut joiner = f.join_with(&separator);
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
