use crate::prelude::*;
use biome_grit_syntax::GritListPatternList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritListPatternList;
impl FormatRule<GritListPatternList> for FormatGritListPatternList {
    type Context = GritFormatContext;
    fn fmt(&self, node: &GritListPatternList, f: &mut GritFormatter) -> FormatResult<()> {
        let mut joiner = f.fill();

        for (_, formatted) in node.elements().zip(node.format_separated(",")) {
            joiner.entry(&soft_line_break_or_space(), &formatted);
        }

        joiner.finish()
    }
}
