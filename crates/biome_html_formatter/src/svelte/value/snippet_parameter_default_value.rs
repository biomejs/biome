use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{
    SvelteSnippetParameterDefaultValue, SvelteSnippetParameterDefaultValueFields,
};
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
        let SvelteSnippetParameterDefaultValueFields { eq_token, value } = node.as_fields();

        write!(f, [space(), eq_token.format(), space(), value.format()])
    }
}
