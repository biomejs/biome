use crate::prelude::*;
use biome_css_syntax::CssUnknownSyntaxTypeName;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUnknownSyntaxTypeName;
impl FormatNodeRule<CssUnknownSyntaxTypeName> for FormatCssUnknownSyntaxTypeName {
    fn fmt_fields(
        &self,
        node: &CssUnknownSyntaxTypeName,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        todo!()
        // format_verbatim_node(node.syntax()).fmt(f)
    }
}
