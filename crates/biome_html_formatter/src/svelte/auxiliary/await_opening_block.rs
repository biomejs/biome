use crate::html::lists::element_list::FormatHtmlElementList;
use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteAwaitOpeningBlock, SvelteAwaitOpeningBlockFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteAwaitOpeningBlock;
impl FormatNodeRule<SvelteAwaitOpeningBlock> for FormatSvelteAwaitOpeningBlock {
    fn fmt_fields(
        &self,
        node: &SvelteAwaitOpeningBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let SvelteAwaitOpeningBlockFields {
            expression,
            r_curly_token,
            sv_curly_hash_token,
            await_token,
            catch_clause,
            children,
            then_clause,
        } = node.as_fields();

        write!(
            f,
            [
                sv_curly_hash_token.format(),
                await_token.format(),
                space(),
                expression.format(),
            ]
        )?;

        if let Some(then_clause) = then_clause {
            write!(f, [space(), then_clause.format()])?;
        }

        if let Some(catch_clause) = catch_clause {
            write!(f, [space(), catch_clause.format()])?;
        }

        write!(f, [r_curly_token.format()])?;

        FormatHtmlElementList::default()
            .with_multiline()
            .fmt(&children, f)?;

        write!(f, [hard_line_break()])
    }
}
