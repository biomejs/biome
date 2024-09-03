use crate::FormatBogusNodeRule;
use biome_html_syntax::HtmlBogus;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlBogus;
impl FormatBogusNodeRule<HtmlBogus> for FormatHtmlBogus {}
