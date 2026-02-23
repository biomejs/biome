use crate::prelude::*;
use biome_css_syntax::{ScssMapExpression, ScssMapExpressionFields};
use biome_formatter::{format_args, write};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssMapExpression;
impl FormatNodeRule<ScssMapExpression> for FormatScssMapExpression {
    fn fmt_fields(&self, node: &ScssMapExpression, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssMapExpressionFields {
            l_paren_token,
            pairs,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                indent(&format_args![soft_line_break(), pairs.format()]),
                soft_line_break(),
                r_paren_token.format()
            ])
            .should_expand(!pairs.is_empty())]
        )
    }
}
