use crate::prelude::*;
use crate::utils::scss_separator_comments::ScssSeparatorComments;
use biome_css_syntax::{ScssExpression, ScssExpressionFields};
use biome_formatter::{FormatResult, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssExpression;
impl FormatNodeRule<ScssExpression> for FormatScssExpression {
    fn fmt_node(&self, node: &ScssExpression, f: &mut CssFormatter) -> FormatResult<()> {
        ScssSeparatorComments::around(node.syntax()).fmt_node(f, |f| self.fmt_fields(node, f))
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
        ScssSeparatorComments::around(node.syntax()).fmt_leading_comments(f)
    }
}
