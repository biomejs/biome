use crate::prelude::*;
use biome_css_syntax::{ScssExpression, ScssExpressionFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssExpression;
impl FormatNodeRule<ScssExpression> for FormatScssExpression {
    fn fmt_fields(&self, node: &ScssExpression, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssExpressionFields { items } = node.as_fields();
        write!(f, [items.format()])
    }
}
