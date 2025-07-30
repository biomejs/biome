use crate::prelude::*;
use biome_html_syntax::SvelteTextExpression;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteTextExpression;
impl FormatNodeRule<SvelteTextExpression> for FormatSvelteTextExpression {
    fn fmt_fields(&self, node: &SvelteTextExpression, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
