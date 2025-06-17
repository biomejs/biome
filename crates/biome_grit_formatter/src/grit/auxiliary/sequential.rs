use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritSequential, GritSequentialFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritSequential;
impl FormatNodeRule<GritSequential> for FormatGritSequential {
    fn fmt_fields(&self, node: &GritSequential, f: &mut GritFormatter) -> FormatResult<()> {
        let GritSequentialFields {
            l_curly_token,
            sequential_token,
            sequential,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [sequential_token.format(), space(), l_curly_token.format(),]
        )?;

        let should_insert_space_around_brackets = f.options().bracket_spacing().value();
        write!(
            f,
            [group(&soft_block_indent_with_maybe_space(
                &sequential.format(),
                should_insert_space_around_brackets
            ),)]
        )?;

        write!(f, [r_curly_token.format()])
    }
}
