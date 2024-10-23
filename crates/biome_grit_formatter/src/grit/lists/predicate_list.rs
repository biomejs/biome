use crate::prelude::*;
use biome_grit_syntax::GritPredicateList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateList;
impl FormatRule<GritPredicateList> for FormatGritPredicateList {
    type Context = GritFormatContext;
    fn fmt(&self, node: &GritPredicateList, f: &mut GritFormatter) -> FormatResult<()> {
        let separator = soft_line_break_or_space();
        let mut joiner = f.join_with(&separator);

        for formatted in node.format_separated(",") {
            joiner.entry(&group(&indent(&formatted)));
        }

        joiner.finish()
    }
}
