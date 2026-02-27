use crate::prelude::*;
use biome_formatter::write;
use biome_formatter::{CstFormatContext, FormatRuleWithOptions};
use biome_html_syntax::HtmlTextExpression;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlTextExpression {
    compact: bool,
}
impl FormatNodeRule<HtmlTextExpression> for FormatHtmlTextExpression {
    fn fmt_fields(&self, node: &HtmlTextExpression, f: &mut HtmlFormatter) -> FormatResult<()> {
        if f.context().comments().is_suppressed(node.syntax()) {
            return write!(f, [format_suppressed_node(node.syntax())]);
        }
        let token = node.html_literal_token()?;

        if self.compact {
            return format_removed(&token).fmt(f);
        }

        let token_text = token.text();
        let trimmed_text = token_text.trim_start().trim_end();

        write!(
            f,
            [format_replaced(
                &token,
                &text(trimmed_text, token.text_range().start())
            )]
        )
    }
}

impl FormatRuleWithOptions<HtmlTextExpression> for FormatHtmlTextExpression {
    type Options = bool;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.compact = options;
        self
    }
}
