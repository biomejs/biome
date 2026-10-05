use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_html_syntax::{HtmlDoubleTextExpression, HtmlDoubleTextExpressionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlDoubleTextExpression {
    r_double_curly_borrowed: bool,
}

pub(crate) struct FormatHtmlDoubleTextExpressionOptions {
    pub r_double_curly_borrowed: bool,
}

impl FormatRuleWithOptions<HtmlDoubleTextExpression> for FormatHtmlDoubleTextExpression {
    type Options = FormatHtmlDoubleTextExpressionOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.r_double_curly_borrowed = options.r_double_curly_borrowed;
        self
    }
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

        write!(
            f,
            [
                l_double_curly_token.format(),
                soft_space_or_block_indent(&expression.format()),
            ]
        )?;
        if !self.r_double_curly_borrowed {
            write!(f, [r_double_curly_token.format()])?;
        }
        Ok(())
    }
}
