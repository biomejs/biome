use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPredicateAnd, GritPredicateAndFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateAnd;
impl FormatNodeRule<GritPredicateAnd> for FormatGritPredicateAnd {
    fn fmt_fields(&self, node: &GritPredicateAnd, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPredicateAndFields {
            l_curly_token,
            and_token,
            predicates,
            r_curly_token,
        } = node.as_fields();
        write!(f, [and_token.format(), space(), l_curly_token.format()])?;
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
