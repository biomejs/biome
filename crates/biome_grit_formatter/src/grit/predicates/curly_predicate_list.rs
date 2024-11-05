use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritCurlyPredicateList, GritCurlyPredicateListFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritCurlyPredicateList;
impl FormatNodeRule<GritCurlyPredicateList> for FormatGritCurlyPredicateList {
    fn fmt_fields(&self, node: &GritCurlyPredicateList, f: &mut GritFormatter) -> FormatResult<()> {
        let GritCurlyPredicateListFields {
            predicates,
            r_curly_token,
            l_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_curly_token.format(),
                predicates.format(),
                r_curly_token.format()
            ]
        )
    }
}
