use crate::prelude::*;
use biome_css_syntax::CssFunctionAtRuleDeclarator;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFunctionAtRuleDeclarator;
impl FormatNodeRule<CssFunctionAtRuleDeclarator> for FormatCssFunctionAtRuleDeclarator {
    fn fmt_fields(
        &self,
        node: &CssFunctionAtRuleDeclarator,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
