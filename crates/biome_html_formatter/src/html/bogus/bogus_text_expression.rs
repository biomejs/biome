use crate::FormatBogusNodeRule;
use biome_html_syntax::HtmlBogusTextExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlBogusTextExpression;
impl FormatBogusNodeRule<HtmlBogusTextExpression> for FormatHtmlBogusTextExpression {}
