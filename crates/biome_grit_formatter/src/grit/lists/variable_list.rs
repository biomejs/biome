use crate::prelude::*;
use biome_formatter::separated::TrailingSeparator;
use biome_grit_syntax::GritVariableList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritVariableList;
impl FormatRule<GritVariableList> for FormatGritVariableList {
    type Context = GritFormatContext;
    fn fmt(&self, node: &GritVariableList, f: &mut GritFormatter) -> FormatResult<()> {
        f.join_with(&space())
            .entries(
                node.format_separated(",")
                    .with_trailing_separator(TrailingSeparator::Omit),
            )
            .finish()
    }
}
