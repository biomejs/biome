use crate::prelude::*;
use biome_css_syntax::CssPseudoClassFunctionNth;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassFunctionNth;
impl FormatNodeRule<CssPseudoClassFunctionNth> for FormatCssPseudoClassFunctionNth {
    fn fmt_fields(
        &self,
        node: &CssPseudoClassFunctionNth,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
