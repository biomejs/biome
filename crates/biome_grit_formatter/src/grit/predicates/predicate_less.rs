use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPredicateLess, GritPredicateLessFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateLess;
impl FormatNodeRule<GritPredicateLess> for FormatGritPredicateLess {
    fn fmt_fields(&self, node: &GritPredicateLess, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPredicateLessFields {
            right,
            left,
            l_angle_token,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                l_angle_token.format(),
                space(),
                right.format()
            ]
        )
    }
}
