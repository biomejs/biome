use crate::prelude::*;
use biome_css_syntax::CssDocumentCustomMatcher;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDocumentCustomMatcher;
impl FormatNodeRule<CssDocumentCustomMatcher> for FormatCssDocumentCustomMatcher {
    fn fmt_fields(
        &self,
        node: &CssDocumentCustomMatcher,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
