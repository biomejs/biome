use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPredicateAny, GritPredicateAnyFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateAny;
impl FormatNodeRule<GritPredicateAny> for FormatGritPredicateAny {
    fn fmt_fields(&self, node: &GritPredicateAny, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPredicateAnyFields {
            any_token,
            l_curly_token,
            predicates,
            r_curly_token,
        } = node.as_fields();
        write!(
            f,
            [
                l_curly_token.format(),
                hard_line_break(),
                any_token.format(),
                hard_line_break(),
                soft_block_indent(&predicates.format()),
                hard_line_break(),
                r_curly_token.format()
            ]
        )
    }
}
