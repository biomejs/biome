use crate::prelude::*;
use biome_css_syntax::CssRegularSyntaxTypeName;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRegularSyntaxTypeName;
impl FormatNodeRule<CssRegularSyntaxTypeName> for FormatCssRegularSyntaxTypeName {
    fn fmt_fields(
        &self,
        node: &CssRegularSyntaxTypeName,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        todo!()
        // format_verbatim_node(node.syntax()).fmt(f)
    }
}
