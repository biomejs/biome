use crate::prelude::*;
use biome_css_syntax::CssValueAtRuleImportClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssValueAtRuleImportClause;
impl FormatNodeRule<CssValueAtRuleImportClause> for FormatCssValueAtRuleImportClause {
    fn fmt_fields(
        &self,
        node: &CssValueAtRuleImportClause,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
