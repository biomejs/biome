use crate::prelude::*;
use biome_html_syntax::HtmlTextExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlTextExpression;
impl FormatNodeRule<HtmlTextExpression> for FormatHtmlTextExpression {
    fn fmt_fields(&self, node: &HtmlTextExpression, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_skipped(node.syntax()).fmt(f)
    }
}
