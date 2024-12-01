use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPredicateLessEqual, GritPredicateLessEqualFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateLessEqual;
impl FormatNodeRule<GritPredicateLessEqual> for FormatGritPredicateLessEqual {
    fn fmt_fields(&self, node: &GritPredicateLessEqual, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPredicateLessEqualFields {
            right,
            left,
            less_than_equal_token,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                less_than_equal_token.format(),
                space(),
                right.format()
            ]
        )
    }
}
