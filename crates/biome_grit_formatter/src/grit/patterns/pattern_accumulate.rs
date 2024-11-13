use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPatternAccumulate, GritPatternAccumulateFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternAccumulate;
impl FormatNodeRule<GritPatternAccumulate> for FormatGritPatternAccumulate {
    fn fmt_fields(&self, node: &GritPatternAccumulate, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPatternAccumulateFields {
            right,
            left,
            add_assign_token,
        } = node.as_fields();
        write!(
            f,
            [
                left.format(),
                space(),
                add_assign_token.format(),
                space(),
                right.format()
            ]
        )
    }
}
