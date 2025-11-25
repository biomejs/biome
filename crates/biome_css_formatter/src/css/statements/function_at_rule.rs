use crate::prelude::*;
use biome_css_syntax::CssFunctionAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFunctionAtRule;
impl FormatNodeRule<CssFunctionAtRule> for FormatCssFunctionAtRule {
    fn fmt_fields(&self, node: &CssFunctionAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
