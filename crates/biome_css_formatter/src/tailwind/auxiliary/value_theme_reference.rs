use crate::prelude::*;
use biome_css_syntax::TwValueThemeReference;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwValueThemeReference;
impl FormatNodeRule<TwValueThemeReference> for FormatTwValueThemeReference {
    fn fmt_fields(&self, node: &TwValueThemeReference, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
