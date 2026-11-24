use crate::html::lists::element_list::FormatHtmlElementList;
use crate::prelude::*;
use crate::shared::FormatSvelteAwaitBinding;
use biome_formatter::write;
use biome_html_syntax::{SvelteAwaitThenBlock, SvelteAwaitThenBlockFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteAwaitThenBlock;
impl FormatNodeRule<SvelteAwaitThenBlock> for FormatSvelteAwaitThenBlock {
    fn fmt_fields(&self, node: &SvelteAwaitThenBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteAwaitThenBlockFields {
            name,
            sv_curly_colon_token,
            r_curly_token,
            children,
            then_token,
        } = node.as_fields();

        sv_curly_colon_token.format().fmt(f)?;
        FormatSvelteAwaitBinding {
            keyword: then_token?,
            name,
        }
        .fmt(f)?;

        write!(f, [r_curly_token.format()])?;

        FormatHtmlElementList::default()
            .with_multiline()
            .fmt(&children, f)?;

        write!(f, [hard_line_break()])
    }

    fn fmt_leading_comments(
        &self,
        _node: &SvelteAwaitThenBlock,
        _f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        // handled by element list formatter
        Ok(())
    }
}
