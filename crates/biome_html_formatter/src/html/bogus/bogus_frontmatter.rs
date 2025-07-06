use crate::FormatBogusNodeRule;
use biome_html_syntax::HtmlBogusFrontmatter;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlBogusFrontmatter;
impl FormatBogusNodeRule<HtmlBogusFrontmatter> for FormatHtmlBogusFrontmatter {}
