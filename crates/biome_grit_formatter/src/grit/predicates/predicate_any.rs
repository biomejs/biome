use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPredicateAny, GritPredicateAnyFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateAny;
impl FormatNodeRule<GritPredicateAny> for FormatGritPredicateAny {
    fn fmt_fields(&self, node: &GritPredicateAny, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPredicateAnyFields {
            any_token,
            l_curly_token,
            predicates,
            r_curly_token,
        } = node.as_fields();
        write!(f, [any_token.format(), space(), l_curly_token.format()])?;
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
