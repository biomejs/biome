use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPredicateMatch, GritPredicateMatchFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateMatch;
impl FormatNodeRule<GritPredicateMatch> for FormatGritPredicateMatch {
    fn fmt_fields(&self, node: &GritPredicateMatch, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPredicateMatchFields {
            left,
            match_token,
            right,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                match_token.format(),
                space(),
                right.format()
            ]
        )
    }
}
