use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_grit_syntax::{GritPatternDefinitionBody, GritPatternDefinitionBodyFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternDefinitionBody;
impl FormatNodeRule<GritPatternDefinitionBody> for FormatGritPatternDefinitionBody {
    fn fmt_fields(
        &self,
        node: &GritPatternDefinitionBody,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        let GritPatternDefinitionBodyFields {
            l_curly_token,
            r_curly_token,
            patterns,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                l_curly_token.format(),
                soft_line_indent_or_space(&format_with(|f| { write!(f, [patterns.format()]) })),
                soft_line_break_or_space(),
                r_curly_token.format()
            ])]
        )
    }
}
