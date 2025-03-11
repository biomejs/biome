use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritCurlyPattern, GritCurlyPatternFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritCurlyPattern;
impl FormatNodeRule<GritCurlyPattern> for FormatGritCurlyPattern {
    fn fmt_fields(&self, node: &GritCurlyPattern, f: &mut GritFormatter) -> FormatResult<()> {
        let GritCurlyPatternFields {
            pattern,
            l_curly_token,
            r_curly_token,
        } = node.as_fields();

        write!(f, [space(), l_curly_token.format()])?;
        let should_insert_space_around_brackets = f.options().bracket_spacing().value();
        write!(
            f,
            [group(&soft_block_indent_with_maybe_space(
                &pattern.format(),
                should_insert_space_around_brackets
            ),)]
        )?;

        write!(f, [r_curly_token.format()])
    }
}
