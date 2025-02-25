use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritMap, GritMapFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritMap;
impl FormatNodeRule<GritMap> for FormatGritMap {
    fn fmt_fields(&self, node: &GritMap, f: &mut GritFormatter) -> FormatResult<()> {
        let GritMapFields {
            l_curly_token,
            elements,
            r_curly_token,
        } = node.as_fields();

        write!(f, [l_curly_token.format()])?;
        let should_insert_space_around_brackets = f.options().bracket_spacing().value();
        write!(
            f,
            [group(&soft_block_indent_with_maybe_space(
                &elements.format(),
                should_insert_space_around_brackets
            ),)]
        )?;

        write!(f, [r_curly_token.format()])
    }
}
