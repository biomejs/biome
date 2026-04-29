use crate::prelude::*;
use crate::utils::scss_separator_comments::FormatScssSeparatorComments;
use biome_css_syntax::{ScssExpression, ScssExpressionFields};
use biome_formatter::{FormatResult, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssExpression;
impl FormatNodeRule<ScssExpression> for FormatScssExpression {
    fn fmt_node(&self, node: &ScssExpression, f: &mut CssFormatter) -> FormatResult<()> {
        self.fmt_node_with_scss_separator_comments(node, f)
    }

    fn fmt_fields(&self, node: &ScssExpression, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssExpressionFields { items } = node.as_fields();
        write!(f, [items.format()])
    }

    fn fmt_leading_comments(
        &self,
        node: &ScssExpression,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        self.fmt_leading_scss_separator_comments(node, f)
    }
}
