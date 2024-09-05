use crate::FormatBogusNodeRule;
use biome_html_syntax::HtmlBogusAttribute;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlBogusAttribute;
impl FormatBogusNodeRule<HtmlBogusAttribute> for FormatHtmlBogusAttribute {}
