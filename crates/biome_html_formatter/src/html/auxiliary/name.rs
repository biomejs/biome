use crate::prelude::*;
use biome_html_syntax::HtmlName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlName;
impl FormatNodeRule<HtmlName> for FormatHtmlName {
    fn fmt_fields(&self, node: &HtmlName, f: &mut HtmlFormatter) -> FormatResult<()> {
        node.as_fields().ident_token.format().fmt(f)
    }
}
