use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPatternNot, GritPatternNotFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternNot;
impl FormatNodeRule<GritPatternNot> for FormatGritPatternNot {
    fn fmt_fields(&self, node: &GritPatternNot, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPatternNotFields { pattern, not } = node.as_fields();

        write!(f, [not.format(), pattern.format()])
    }
}
