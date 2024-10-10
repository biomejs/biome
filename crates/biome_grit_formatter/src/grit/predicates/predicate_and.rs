use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPredicateAnd, GritPredicateAndFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateAnd;
impl FormatNodeRule<GritPredicateAnd> for FormatGritPredicateAnd {
    fn fmt_fields(&self, node: &GritPredicateAnd, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPredicateAndFields {
            l_curly_token,
            and_token,
            predicates,
            r_curly_token,
        } = node.as_fields();
        write!(
            f,
            [
                l_curly_token.format(),
                hard_line_break(),
                and_token.format(),
                hard_line_break(),
                soft_block_indent(&predicates.format()),
                hard_line_break(),
                r_curly_token.format()
            ]
        )
    }
}
