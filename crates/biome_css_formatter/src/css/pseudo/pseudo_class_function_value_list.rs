use crate::prelude::*;
use biome_css_syntax::CssPseudoClassFunctionValueList;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassFunctionValueList;
impl FormatNodeRule<CssPseudoClassFunctionValueList> for FormatCssPseudoClassFunctionValueList {
    fn fmt_fields(
        &self,
        node: &CssPseudoClassFunctionValueList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
