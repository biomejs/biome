use crate::prelude::*;
use biome_css_syntax::CssSyntaxComponent;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSyntaxComponent;
impl FormatNodeRule<CssSyntaxComponent> for FormatCssSyntaxComponent {
    fn fmt_fields(&self, node: &CssSyntaxComponent, f: &mut CssFormatter) -> FormatResult<()> {
        todo!()
        // format_verbatim_node(node.syntax()).fmt(f)
    }
}
