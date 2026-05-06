use crate::prelude::*;
use crate::utils::scss_separator_comments::FormatScssSeparatorComments;
use biome_css_syntax::{ScssListExpressionElement, ScssListExpressionElementFields};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssListExpressionElement;
impl FormatNodeRule<ScssListExpressionElement> for FormatScssListExpressionElement {
    fn fmt_node(&self, node: &ScssListExpressionElement, f: &mut CssFormatter) -> FormatResult<()> {
        self.fmt_node_with_scss_separator_comments(node, f)
    }

    fn fmt_fields(
        &self,
        node: &ScssListExpressionElement,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssListExpressionElementFields { value } = node.as_fields();

        write!(f, [value.format()])
    }

    fn fmt_leading_comments(
        &self,
        node: &ScssListExpressionElement,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        self.fmt_leading_scss_separator_comments(node, f)
    }
}
