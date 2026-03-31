use crate::prelude::*;
use crate::separated::FormatAstSeparatedListExtension;
use biome_html_syntax::SvelteSnippetParameterList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteSnippetParameterList;
impl FormatRule<SvelteSnippetParameterList> for FormatSvelteSnippetParameterList {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &SvelteSnippetParameterList, f: &mut HtmlFormatter) -> FormatResult<()> {
        let separator = space();
        let mut joiner = f.join_with(&separator);

        for formatted in node.format_separated(",") {
            joiner.entry(&formatted);
        }

        joiner.finish()
    }
}
