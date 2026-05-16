use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, format_args, write};
use biome_html_syntax::{HtmlDoubleTextExpression, HtmlDoubleTextExpressionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlDoubleTextExpression {
    preserve_expression_newlines: bool,
}
impl FormatNodeRule<HtmlDoubleTextExpression> for FormatHtmlDoubleTextExpression {
    fn fmt_fields(
        &self,
        node: &HtmlDoubleTextExpression,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let HtmlDoubleTextExpressionFields {
            l_double_curly_token,
            expression,
            r_double_curly_token,
        } = node.as_fields();

        let should_preserve_newlines = self.preserve_expression_newlines
            && expression
                .as_ref()
                .ok()
                .and_then(|expression| expression.html_literal_token().ok())
                .is_some_and(|token| token.text().contains('\n') || token.text().contains('\r'));

        if should_preserve_newlines {
            write!(
                f,
                [group(&format_args![
                    l_double_curly_token.format(),
                    block_indent(&expression.format()),
                    r_double_curly_token.format(),
                ])]
            )
        } else {
            write!(
                f,
                [group(&format_args![
                    l_double_curly_token.format(),
                    soft_space_or_block_indent(&expression.format()),
                    r_double_curly_token.format(),
                ])]
            )
        }
    }
}

impl FormatRuleWithOptions<HtmlDoubleTextExpression> for FormatHtmlDoubleTextExpression {
    type Options = bool;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.preserve_expression_newlines = options;
        self
    }
}
