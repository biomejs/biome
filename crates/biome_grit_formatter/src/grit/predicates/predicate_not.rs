use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPredicateNot, GritPredicateNotFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateNot;
impl FormatNodeRule<GritPredicateNot> for FormatGritPredicateNot {
    fn fmt_fields(&self, node: &GritPredicateNot, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPredicateNotFields { not, predicate } = node.as_fields();

        write!(f, [not.format(), predicate.format()])
    }
}
