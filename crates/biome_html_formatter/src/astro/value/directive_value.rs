use crate::prelude::*;
use biome_html_syntax::AstroDirectiveValue;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAstroDirectiveValue;
impl FormatNodeRule<AstroDirectiveValue> for FormatAstroDirectiveValue {
    fn fmt_fields(&self, node: &AstroDirectiveValue, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
