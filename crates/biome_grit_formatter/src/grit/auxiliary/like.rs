use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritLike, GritLikeFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritLike;
impl FormatNodeRule<GritLike> for FormatGritLike {
    fn fmt_fields(&self, node: &GritLike, f: &mut GritFormatter) -> FormatResult<()> {
        let GritLikeFields {
            like_token,
            l_curly_token,
            example,
            threshold,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                like_token.format(),
                space(),
                threshold.format(),
                space(),
                l_curly_token.format(),
            ]
        )?;

        let should_insert_space_around_brackets = f.options().bracket_spacing().value();
        write!(
            f,
            [group(&soft_block_indent_with_maybe_space(
                &example.format(),
                should_insert_space_around_brackets
            ),)]
        )?;

        write!(f, [r_curly_token.format()])
    }
}
