use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPredicateNotEqual, GritPredicateNotEqualFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateNotEqual;
impl FormatNodeRule<GritPredicateNotEqual> for FormatGritPredicateNotEqual {
    fn fmt_fields(&self, node: &GritPredicateNotEqual, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPredicateNotEqualFields {
            right,
            left,
            inequality_token,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                inequality_token.format(),
                space(),
                right.format()
            ]
        )
    }
}
