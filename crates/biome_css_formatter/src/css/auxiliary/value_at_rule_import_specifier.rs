use crate::prelude::*;
use biome_css_syntax::CssValueAtRuleImportSpecifier;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssValueAtRuleImportSpecifier;
impl FormatNodeRule<CssValueAtRuleImportSpecifier> for FormatCssValueAtRuleImportSpecifier {
    fn fmt_fields(
        &self,
        node: &CssValueAtRuleImportSpecifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
