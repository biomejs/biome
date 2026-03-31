use crate::prelude::*;
use crate::verbatim::format_html_verbatim_node;
use biome_html_syntax::SvelteSnippetParameterList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteSnippetParameterList;
impl FormatRule<SvelteSnippetParameterList> for FormatSvelteSnippetParameterList {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &SvelteSnippetParameterList, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
