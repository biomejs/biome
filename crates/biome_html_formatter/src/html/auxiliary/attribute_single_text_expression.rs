use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_html_syntax::{
    HtmlAttributeSingleTextExpression, HtmlAttributeSingleTextExpressionFields,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAttributeSingleTextExpression {
    compact: bool,
}

impl FormatNodeRule<HtmlAttributeSingleTextExpression> for FormatHtmlAttributeSingleTextExpression {
    fn fmt_fields(
        &self,
        node: &HtmlAttributeSingleTextExpression,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let HtmlAttributeSingleTextExpressionFields {
            l_curly_token,
            expression,
            r_curly_token,
        } = node.as_fields();

        if self.compact {
            let l_curly_token = l_curly_token?;
            let r_curly_token = r_curly_token.clone()?;
            let expression = expression.clone()?;
            format_removed(&l_curly_token).fmt(f)?;
            format_removed(&r_curly_token).fmt(f)?;
            expression.format().with_options(self.compact).fmt(f)
        } else {
            write!(
                f,
                [
                    l_curly_token.format(),
                    expression.format(),
                    r_curly_token.format()
                ]
            )
        }
    }
}

impl FormatRuleWithOptions<HtmlAttributeSingleTextExpression>
    for FormatHtmlAttributeSingleTextExpression
{
    type Options = bool;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.compact = options;
        self
    }
}
