use crate::html::auxiliary::text_expression::FormatHtmlTextExpressionOptions;
use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_html_syntax::{
    HtmlAttributeSingleTextExpression, HtmlAttributeSingleTextExpressionFields,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAttributeSingleTextExpression {
    layout: HtmlAttributeSingleTextExpressionLayout,
}

/// Selects how an attribute value such as `{value}` is emitted.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub(crate) enum HtmlAttributeSingleTextExpressionLayout {
    /// Formats both braces and the inner expression.
    #[default]
    Normal,
    /// Consumes the braces and inner expression without emitting them.
    Remove,
    /// Consumes the braces and formats only the inner expression, which remains
    /// the fallback when embedded formatting is unavailable.
    Unwrap,
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

        match self.layout {
            HtmlAttributeSingleTextExpressionLayout::Normal => write!(
                f,
                [
                    l_curly_token.format(),
                    expression.format(),
                    r_curly_token.format()
                ]
            ),
            HtmlAttributeSingleTextExpressionLayout::Remove
            | HtmlAttributeSingleTextExpressionLayout::Unwrap => {
                let l_curly_token = l_curly_token?;
                let r_curly_token = r_curly_token.clone()?;
                let expression = expression.clone()?;
                format_removed(&l_curly_token).fmt(f)?;
                format_removed(&r_curly_token).fmt(f)?;
                if self.layout == HtmlAttributeSingleTextExpressionLayout::Remove {
                    expression
                        .format()
                        .with_options(FormatHtmlTextExpressionOptions {
                            compact: true,
                            ..Default::default()
                        })
                        .fmt(f)
                } else {
                    expression.format().fmt(f)
                }
            }
        }
    }
}

impl FormatRuleWithOptions<HtmlAttributeSingleTextExpression>
    for FormatHtmlAttributeSingleTextExpression
{
    type Options = HtmlAttributeSingleTextExpressionLayout;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.layout = options;
        self
    }
}
