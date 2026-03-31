use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteSnippetExpression, SvelteSnippetExpressionFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteSnippetExpression;
impl FormatNodeRule<SvelteSnippetExpression> for FormatSvelteSnippetExpression {
    fn fmt_fields(
        &self,
        node: &SvelteSnippetExpression,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let SvelteSnippetExpressionFields {
            name,
            l_paren_token,
            parameters,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                name.format(),
                l_paren_token.format(),
                parameters.format(),
                r_paren_token.format()
            ]
        )
    }
}
