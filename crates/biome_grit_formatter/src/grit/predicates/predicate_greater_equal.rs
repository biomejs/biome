use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPredicateGreaterEqual, GritPredicateGreaterEqualFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateGreaterEqual;
impl FormatNodeRule<GritPredicateGreaterEqual> for FormatGritPredicateGreaterEqual {
    fn fmt_fields(
        &self,
        node: &GritPredicateGreaterEqual,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        let GritPredicateGreaterEqualFields {
            right,
            left,
            greater_than_equal_token,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                greater_than_equal_token.format(),
                space(),
                right.format()
            ]
        )
    }
}
