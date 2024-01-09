use crate::prelude::*;
use biome_css_syntax::CssImportSupports;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssImportSupports;
impl FormatNodeRule<CssImportSupports> for FormatCssImportSupports {
    fn fmt_fields(&self, node: &CssImportSupports, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
