use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPatternContains, GritPatternContainsFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternContains;
impl FormatNodeRule<GritPatternContains> for FormatGritPatternContains {
    fn fmt_fields(&self, node: &GritPatternContains, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPatternContainsFields {
            contains,
            contains_token,
            until_clause,
        } = node.as_fields();

        write!(f, [contains_token.format(), space(), contains.format()])?;

        if let Some(until_clause) = until_clause {
            write!(f, [space(), until_clause.format()])?;
        }

        Ok(())
    }
}
