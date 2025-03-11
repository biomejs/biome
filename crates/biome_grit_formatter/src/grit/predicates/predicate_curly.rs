use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPredicateCurly, GritPredicateCurlyFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateCurly;
impl FormatNodeRule<GritPredicateCurly> for FormatGritPredicateCurly {
    fn fmt_fields(&self, node: &GritPredicateCurly, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPredicateCurlyFields {
            l_curly_token,
            predicates,
            r_curly_token,
        } = node.as_fields();

        write!(f, [l_curly_token.format()])?;
        let should_insert_space_around_brackets = f.options().bracket_spacing().value();
        write!(
            f,
            [group(&soft_block_indent_with_maybe_space(
                &predicates.format(),
                should_insert_space_around_brackets
            ),)]
        )?;

        write!(f, [r_curly_token.format()])
    }
}
