use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteSnippetParameter, SvelteSnippetParameterFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteSnippetParameter;
impl FormatNodeRule<SvelteSnippetParameter> for FormatSvelteSnippetParameter {
    fn fmt_fields(&self, node: &SvelteSnippetParameter, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteSnippetParameterFields { binding, default } = node.as_fields();

        write!(f, [binding.format(), default.format()])
    }
}
