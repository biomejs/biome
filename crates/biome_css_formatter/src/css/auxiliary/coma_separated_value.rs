use crate::prelude::*;
use biome_css_syntax::CssComaSeparatedValue;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssComaSeparatedValue;
impl FormatNodeRule<CssComaSeparatedValue> for FormatCssComaSeparatedValue {
    fn fmt_fields(&self, node: &CssComaSeparatedValue, f: &mut CssFormatter) -> FormatResult<()> {
        todo!()
        // format_verbatim_node(node.syntax()).fmt(f)
    }
}
