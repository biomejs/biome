use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPatternIfElse, GritPatternIfElseFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternIfElse;
impl FormatNodeRule<GritPatternIfElse> for FormatGritPatternIfElse {
    fn fmt_fields(&self, node: &GritPatternIfElse, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPatternIfElseFields {
            r_paren_token,
            l_paren_token,
            else_clause,
            if_predicate,
            if_token,
            then_pattern,
        } = node.as_fields();

        write!(
            f,
            [
                if_token.format(),
                space(),
                l_paren_token.format(),
                if_predicate.format(),
                r_paren_token.format(),
                then_pattern.format()
            ]
        )?;

        if let Some(else_clause) = else_clause {
            write!(f, [space(), else_clause.format()])?;
        }

        Ok(())
    }
}
