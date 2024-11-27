use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritBracketedPredicate, GritBracketedPredicateFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBracketedPredicate;
impl FormatNodeRule<GritBracketedPredicate> for FormatGritBracketedPredicate {
    fn fmt_fields(&self, node: &GritBracketedPredicate, f: &mut GritFormatter) -> FormatResult<()> {
        let GritBracketedPredicateFields {
            l_paren_token,
            predicate,
            r_paren_token,
        } = node.as_fields();
        write!(
            f,
            [
                l_paren_token.format(),
                predicate.format(),
                r_paren_token.format(),
            ]
        )
    }
}
