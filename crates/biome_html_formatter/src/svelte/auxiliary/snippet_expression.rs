use crate::prelude::*;
use crate::verbatim::format_html_verbatim_node;
use biome_html_syntax::SvelteSnippetExpression;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteSnippetExpression;
impl FormatNodeRule<SvelteSnippetExpression> for FormatSvelteSnippetExpression {
    fn fmt_fields(
        &self,
        node: &SvelteSnippetExpression,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
