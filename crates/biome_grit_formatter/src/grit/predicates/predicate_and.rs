use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPredicateAnd, GritPredicateAndFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateAnd;
impl FormatNodeRule<GritPredicateAnd> for FormatGritPredicateAnd {
    fn fmt_fields(&self, node: &GritPredicateAnd, f: &mut GritFormatter) -> FormatResult<()> {
        // let GritPredicateAndFields {
        //     and_token,
        //     l_curly_token,
        //     predicates,
        //     r_curly_token,
        // } = node.as_fields();

        // write!(
        //     f,
        //     [
        //         and_token.format(),
        //         l_curly_token.format(),
        //         predicates.format(),
        //         r_curly_token.format()
        //     ]
        // )

        format_verbatim_node(node.syntax()).fmt(f)
    }
}
