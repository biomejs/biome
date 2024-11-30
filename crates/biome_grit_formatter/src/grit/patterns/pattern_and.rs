use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPatternAnd, GritPatternAndFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternAnd;
impl FormatNodeRule<GritPatternAnd> for FormatGritPatternAnd {
    fn fmt_fields(&self, node: &GritPatternAnd, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPatternAndFields {
            patterns,
            and_token,
            l_curly_token,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                and_token.format(),
                space(),
                l_curly_token.format(),
                space(),
                patterns.format(),
                space(),
                r_curly_token.format()
            ]
        )
    }
}
