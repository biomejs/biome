use crate::html::lists::element_list::FormatHtmlElementList;
use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteElseIfClause, SvelteElseIfClauseFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteElseIfClause;
impl FormatNodeRule<SvelteElseIfClause> for FormatSvelteElseIfClause {
    fn fmt_fields(&self, node: &SvelteElseIfClause, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteElseIfClauseFields {
            r_curly_token,
            expression,
            children,
            if_token,
            sv_curly_colon_token,
            else_token,
        } = node.as_fields();

        write!(
            f,
            [
                sv_curly_colon_token.format(),
                else_token.format(),
                space(),
                if_token.format(),
                expression.format(),
                r_curly_token.format()
            ]
        )?;

        // The order here is important. First, we must check if we can delegate the formatting
        // of embedded nodes, then we check if we should format them verbatim.
        let format_children = FormatHtmlElementList::default()
            .with_group_id(f.group_id("svelte-else-if-group"))
            .fmt_children(&children, f)?;

        write!(f, [format_children, hard_line_break(),])
    }
}
