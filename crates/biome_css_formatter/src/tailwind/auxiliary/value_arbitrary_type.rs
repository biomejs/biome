use crate::prelude::*;
use biome_css_syntax::TwValueArbitraryType;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwValueArbitraryType;
impl FormatNodeRule<TwValueArbitraryType> for FormatTwValueArbitraryType {
    fn fmt_fields(&self, node: &TwValueArbitraryType, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
