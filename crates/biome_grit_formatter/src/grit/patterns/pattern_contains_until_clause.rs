use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPatternContainsUntilClause, GritPatternContainsUntilClauseFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternContainsUntilClause;
impl FormatNodeRule<GritPatternContainsUntilClause> for FormatGritPatternContainsUntilClause {
    fn fmt_fields(
        &self,
        node: &GritPatternContainsUntilClause,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        let GritPatternContainsUntilClauseFields { until, until_token } = node.as_fields();

        write!(f, [until_token.format(), space(), until.format()])
    }
}
