use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPatternAs, GritPatternAsFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternAs;
impl FormatNodeRule<GritPatternAs> for FormatGritPatternAs {
    fn fmt_fields(&self, node: &GritPatternAs, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPatternAsFields {
            pattern,
            as_token,
            variable,
        } = node.as_fields();

        write!(
            f,
            [
                pattern.format(),
                space(),
                as_token.format(),
                space(),
                variable.format()
            ]
        )
    }
}
