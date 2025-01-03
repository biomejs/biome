use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPredicateIfElse, GritPredicateIfElseFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateIfElse;
impl FormatNodeRule<GritPredicateIfElse> for FormatGritPredicateIfElse {
    fn fmt_fields(&self, node: &GritPredicateIfElse, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPredicateIfElseFields {
            r_paren_token,
            l_paren_token,
            else_clause,
            if_predicate,
            if_token,
            then_predicate,
        } = node.as_fields();

        write!(
            f,
            [
                if_token.format(),
                space(),
                l_paren_token.format(),
                if_predicate.format(),
                r_paren_token.format(),
                then_predicate.format()
            ]
        )?;

        if let Some(else_clause) = else_clause {
            write!(f, [space(), else_clause.format()])?;
        }

        Ok(())
    }
}
