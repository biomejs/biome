use crate::html::lists::element_list::FormatHtmlElementList;
use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteSnippetOpeningBlock, SvelteSnippetOpeningBlockFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteSnippetOpeningBlock;
impl FormatNodeRule<SvelteSnippetOpeningBlock> for FormatSvelteSnippetOpeningBlock {
    fn fmt_fields(
        &self,
        node: &SvelteSnippetOpeningBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let SvelteSnippetOpeningBlockFields {
            r_curly_token,
            sv_curly_hash_token,
            children,
            snippet_token,
            expression,
        } = node.as_fields();

        write!(
            f,
            [
                sv_curly_hash_token.format(),
                snippet_token.format(),
                space(),
                expression.format(),
                r_curly_token.format()
            ]
        )?;

        FormatHtmlElementList::default()
            .with_multiline()
            .fmt(&children, f)?;

        write!(f, [hard_line_break()])
    }
}
