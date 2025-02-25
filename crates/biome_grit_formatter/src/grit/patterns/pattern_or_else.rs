use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritPatternOrElse, GritPatternOrElseFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternOrElse;
impl FormatNodeRule<GritPatternOrElse> for FormatGritPatternOrElse {
    fn fmt_fields(&self, node: &GritPatternOrElse, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPatternOrElseFields {
            l_curly_token,
            r_curly_token,
            patterns,
            orelse_token,
        } = node.as_fields();

        write!(f, [orelse_token.format(), space(), l_curly_token.format()])?;
        let should_insert_space_around_brackets = f.options().bracket_spacing().value();
        write!(
            f,
            [group(&soft_block_indent_with_maybe_space(
                &patterns.format(),
                should_insert_space_around_brackets
            ),)]
        )?;

        write!(f, [r_curly_token.format()])
    }
}
