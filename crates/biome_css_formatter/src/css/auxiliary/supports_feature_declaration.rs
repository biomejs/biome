use crate::prelude::*;
use biome_css_syntax::CssSupportsFeatureDeclaration;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSupportsFeatureDeclaration;
impl FormatNodeRule<CssSupportsFeatureDeclaration> for FormatCssSupportsFeatureDeclaration {
    fn fmt_fields(
        &self,
        node: &CssSupportsFeatureDeclaration,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
