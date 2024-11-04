use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPredicateAssignment, GritPredicateAssignmentFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateAssignment;
impl FormatNodeRule<GritPredicateAssignment> for FormatGritPredicateAssignment {
    fn fmt_fields(
        &self,
        node: &GritPredicateAssignment,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        let GritPredicateAssignmentFields {
            container,
            eq_token,
            pattern,
        } = node.as_fields();

        write!(
            f,
            [
                container.format(),
                space(),
                eq_token.format(),
                space(),
                pattern.format()
            ]
        )
    }
}
