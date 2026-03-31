use crate::prelude::*;
use crate::verbatim::format_html_verbatim_node;
use biome_html_syntax::SvelteSnippetParameter;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteSnippetParameter;
impl FormatNodeRule<SvelteSnippetParameter> for FormatSvelteSnippetParameter {
    fn fmt_fields(&self, node: &SvelteSnippetParameter, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
