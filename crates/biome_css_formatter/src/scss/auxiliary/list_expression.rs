use crate::prelude::*;
use biome_css_syntax::{ScssListExpression, ScssListExpressionFields};
use biome_formatter::{format_args, write};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssListExpression;
impl FormatNodeRule<ScssListExpression> for FormatScssListExpression {
    fn fmt_fields(&self, node: &ScssListExpression, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssListExpressionFields { elements } = node.as_fields();

        write!(
            f,
            [group(&indent(&format_args![
                soft_line_break(),
                elements.format()
            ]))]
        )
    }
}
