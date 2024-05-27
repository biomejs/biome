use crate::prelude::*;
use biome_css_syntax::CssValueAtRuleNamedImportSpecifier;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssValueAtRuleNamedImportSpecifier;
impl FormatNodeRule<CssValueAtRuleNamedImportSpecifier>
    for FormatCssValueAtRuleNamedImportSpecifier
{
    fn fmt_fields(
        &self,
        node: &CssValueAtRuleNamedImportSpecifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
