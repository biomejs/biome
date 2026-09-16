use crate::prelude::*;
use biome_css_syntax::ScssUrlText;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssUrlText;
impl FormatNodeRule<ScssUrlText> for FormatScssUrlText {
    fn fmt_fields(&self, node: &ScssUrlText, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
