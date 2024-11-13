use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPredicateGreater, GritPredicateGreaterFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateGreater;
impl FormatNodeRule<GritPredicateGreater> for FormatGritPredicateGreater {
    fn fmt_fields(&self, node: &GritPredicateGreater, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPredicateGreaterFields {
            right,
            left,
            r_angle_token,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                r_angle_token.format(),
                space(),
                right.format()
            ]
        )
    }
}
