use crate::prelude::*;
use biome_css_syntax::CssPropertyAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPropertyAtRule;
impl FormatNodeRule<CssPropertyAtRule> for FormatCssPropertyAtRule {
    fn fmt_fields(&self, node: &CssPropertyAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
