use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPredicateElseClause, GritPredicateElseClauseFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateElseClause;
impl FormatNodeRule<GritPredicateElseClause> for FormatGritPredicateElseClause {
    fn fmt_fields(
        &self,
        node: &GritPredicateElseClause,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        let GritPredicateElseClauseFields {
            else_predicate,
            else_token,
        } = node.as_fields();

        write!(f, [else_token.format(), space(), else_predicate.format()])
    }
}
