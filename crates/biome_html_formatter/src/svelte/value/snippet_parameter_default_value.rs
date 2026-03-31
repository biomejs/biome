use crate::prelude::*;
use crate::verbatim::format_html_verbatim_node;
use biome_html_syntax::SvelteSnippetParameterDefaultValue;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteSnippetParameterDefaultValue;
impl FormatNodeRule<SvelteSnippetParameterDefaultValue>
    for FormatSvelteSnippetParameterDefaultValue
{
    fn fmt_fields(
        &self,
        node: &SvelteSnippetParameterDefaultValue,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
