use crate::html::lists::element_list::FormatHtmlElementList;
use crate::prelude::*;
use crate::shared::FormatSvelteAwaitBinding;
use biome_formatter::write;
use biome_html_syntax::{SvelteAwaitCatchBlock, SvelteAwaitCatchBlockFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteAwaitCatchBlock;
impl FormatNodeRule<SvelteAwaitCatchBlock> for FormatSvelteAwaitCatchBlock {
    fn fmt_fields(&self, node: &SvelteAwaitCatchBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteAwaitCatchBlockFields {
            name,
            r_curly_token,
            children,
            catch_token,
            sv_curly_colon_token,
        } = node.as_fields();

        sv_curly_colon_token.format().fmt(f)?;
        FormatSvelteAwaitBinding {
            keyword: catch_token?,
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
        _node: &SvelteAwaitCatchBlock,
        _f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        // handled by element list formatter
        Ok(())
    }
}
