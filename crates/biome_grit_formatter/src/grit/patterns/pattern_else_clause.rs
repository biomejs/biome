use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPatternElseClause, GritPatternElseClauseFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternElseClause;
impl FormatNodeRule<GritPatternElseClause> for FormatGritPatternElseClause {
    fn fmt_fields(&self, node: &GritPatternElseClause, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPatternElseClauseFields {
            else_token,
            else_pattern,
        } = node.as_fields();

        write!(f, [else_token.format(), space(), else_pattern.format()])
    }
}
