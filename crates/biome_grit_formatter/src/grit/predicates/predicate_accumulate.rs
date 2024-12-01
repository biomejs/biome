use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPredicateAccumulate, GritPredicateAccumulateFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateAccumulate;
impl FormatNodeRule<GritPredicateAccumulate> for FormatGritPredicateAccumulate {
    fn fmt_fields(
        &self,
        node: &GritPredicateAccumulate,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        let GritPredicateAccumulateFields {
            left,
            add_assign_token,
            right,
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
