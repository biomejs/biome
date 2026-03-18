use crate::prelude::*;
use biome_css_syntax::{ScssBinaryExpression, ScssBinaryExpressionFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssBinaryExpression;

impl FormatNodeRule<ScssBinaryExpression> for FormatScssBinaryExpression {
    fn fmt_fields(&self, node: &ScssBinaryExpression, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssBinaryExpressionFields {
            left,
            operator,
            right,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                left.format(),
                space(),
                operator.format(),
                soft_line_break_or_space(),
                right.format()
            ])]
        )
    }
}
