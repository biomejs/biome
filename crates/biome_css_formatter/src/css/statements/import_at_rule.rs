use crate::prelude::*;
use biome_css_syntax::CssImportAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssImportAtRule;
impl FormatNodeRule<CssImportAtRule> for FormatCssImportAtRule {
    fn fmt_fields(&self, node: &CssImportAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
