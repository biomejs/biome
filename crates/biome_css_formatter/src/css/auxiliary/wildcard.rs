use crate::prelude::*;
use biome_css_syntax::CssWildcard;
use biome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssWildcard;

impl FormatNodeRule<CssWildcard> for FormatCssWildcard {
    fn fmt_fields(&self, node: &CssWildcard, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
