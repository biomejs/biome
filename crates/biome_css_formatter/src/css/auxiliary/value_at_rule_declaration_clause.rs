use crate::prelude::*;
use biome_css_syntax::CssValueAtRuleDeclarationClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssValueAtRuleDeclarationClause;
impl FormatNodeRule<CssValueAtRuleDeclarationClause> for FormatCssValueAtRuleDeclarationClause {
    fn fmt_fields(
        &self,
        node: &CssValueAtRuleDeclarationClause,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
