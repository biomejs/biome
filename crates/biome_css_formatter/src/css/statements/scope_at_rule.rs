use crate::prelude::*;
use biome_css_syntax::CssScopeAtRule;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssScopeAtRule;
impl FormatNodeRule<CssScopeAtRule> for FormatCssScopeAtRule {
    fn fmt_fields(&self, node: &CssScopeAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
