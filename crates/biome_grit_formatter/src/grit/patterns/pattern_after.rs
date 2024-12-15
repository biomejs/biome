use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPatternAfter, GritPatternAfterFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternAfter;
impl FormatNodeRule<GritPatternAfter> for FormatGritPatternAfter {
    fn fmt_fields(&self, node: &GritPatternAfter, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPatternAfterFields {
            pattern,
            after_token,
        } = node.as_fields();

        write!(f, [after_token.format(), space(), pattern.format()])
    }
}
