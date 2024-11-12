use crate::prelude::*;
use biome_formatter::separated::TrailingSeparator;
use biome_grit_syntax::GritPatternList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternList;
impl FormatRule<GritPatternList> for FormatGritPatternList {
    type Context = GritFormatContext;
    fn fmt(&self, node: &GritPatternList, f: &mut GritFormatter) -> FormatResult<()> {
        f.join_with(&hard_line_break())
            .entries(
                node.format_separated(",")
                    .with_trailing_separator(TrailingSeparator::Omit),
            )
            .finish()
    }
}
