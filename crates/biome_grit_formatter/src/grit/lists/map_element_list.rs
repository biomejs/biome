use crate::prelude::*;
use biome_formatter::separated::TrailingSeparator;
use biome_grit_syntax::GritMapElementList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritMapElementList;
impl FormatRule<GritMapElementList> for FormatGritMapElementList {
    type Context = GritFormatContext;
    fn fmt(&self, node: &GritMapElementList, f: &mut GritFormatter) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(
                node.format_separated(",")
                    .with_trailing_separator(TrailingSeparator::Omit),
            )
            .finish()
    }
}
