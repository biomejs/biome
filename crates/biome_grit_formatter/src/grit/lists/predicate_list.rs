use crate::prelude::*;
use biome_grit_syntax::GritPredicateList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateList;
impl FormatRule<GritPredicateList> for FormatGritPredicateList {
    type Context = GritFormatContext;
    fn fmt(&self, node: &GritPredicateList, f: &mut GritFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();

        // TODO: Add separator
        for predicate in node {
            match predicate {
                Ok(predicate) => {
                    join.entry(predicate.syntax(), &format_or_verbatim(predicate.format()));
                }
                Err(_) => (),
            }
        }

        join.finish()
    }
}
