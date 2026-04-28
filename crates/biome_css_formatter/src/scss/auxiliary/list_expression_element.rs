use crate::prelude::*;
use crate::utils::scss_separator_comments::ScssSeparatorComments;
use biome_css_syntax::{ScssListExpressionElement, ScssListExpressionElementFields};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssListExpressionElement;
impl FormatNodeRule<ScssListExpressionElement> for FormatScssListExpressionElement {
    fn fmt_node(&self, node: &ScssListExpressionElement, f: &mut CssFormatter) -> FormatResult<()> {
        ScssSeparatorComments::around(node.syntax()).fmt_node(f, |f| self.fmt_fields(node, f))
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
        ScssSeparatorComments::around(node.syntax()).fmt_leading_comments(f)
    }
}
