use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPatternBefore, GritPatternBeforeFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternBefore;
impl FormatNodeRule<GritPatternBefore> for FormatGritPatternBefore {
    fn fmt_fields(&self, node: &GritPatternBefore, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPatternBeforeFields {
            pattern,
            before_token,
        } = node.as_fields();

        write!(f, [before_token.format(), space(), pattern.format()])
    }
}
