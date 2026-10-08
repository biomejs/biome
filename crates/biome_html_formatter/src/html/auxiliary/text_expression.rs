use crate::prelude::*;
use biome_formatter::write;
use biome_formatter::{CstFormatContext, FormatRuleWithOptions, normalize_newlines};
use biome_html_syntax::HtmlTextExpression;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlTextExpression {
    options: FormatHtmlTextExpressionOptions,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct FormatHtmlTextExpressionOptions {
    /// Whether it should be formatted in compact mode. In compact mode, the
    /// expression is removed.
    pub compact: bool,

    /// Whether the double quotes of the source text are escaped as `&quot;`,
    /// for an expression written between double quotes. The source text is
    /// written when the expression isn't formatted as JavaScript.
    pub escape_double_quotes: bool,
}

impl FormatNodeRule<HtmlTextExpression> for FormatHtmlTextExpression {
    fn fmt_fields(&self, node: &HtmlTextExpression, f: &mut HtmlFormatter) -> FormatResult<()> {
        if f.context().comments().is_suppressed(node.syntax()) {
            return write!(f, [format_suppressed_node(node.syntax())]);
        }
        let token = node.html_literal_token()?;

        if self.options.compact {
            return format_removed(&token).fmt(f);
        }

        let token_text = token.text();
        let trimmed_text = token_text.trim_start().trim_end();
        let mut normalized_text = normalize_newlines(trimmed_text, ['\r']);
        if self.options.escape_double_quotes {
            normalized_text = normalized_text.replace('"', "&quot;").into();
        }

        write!(
            f,
            [format_replaced(
                &token,
                &text(&normalized_text, Some(token.text_range().start()))
            )]
        )
    }
}

impl FormatRuleWithOptions<HtmlTextExpression> for FormatHtmlTextExpression {
    type Options = FormatHtmlTextExpressionOptions;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.options = options;
        self
    }
}
