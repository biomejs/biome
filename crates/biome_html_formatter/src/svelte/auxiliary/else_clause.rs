use crate::html::lists::element_list::FormatHtmlElementList;
use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteElseClause, SvelteElseClauseFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteElseClause;
impl FormatNodeRule<SvelteElseClause> for FormatSvelteElseClause {
    fn fmt_fields(&self, node: &SvelteElseClause, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteElseClauseFields {
            r_curly_token,
            children,
            else_token,
            sv_curly_colon_token,
        } = node.as_fields();

        write!(
            f,
            [
                sv_curly_colon_token.format(),
                else_token.format(),
                r_curly_token.format(),
            ]
        )?;
        // The order here is important. First, we must check if we can delegate the formatting
        // of embedded nodes, then we check if we should format them verbatim.
        let format_children = FormatHtmlElementList::default()
            .with_group_id(f.group_id("svelte-else-group"))
            .fmt_children(&children, f)?;

        write!(f, [format_children, hard_line_break(),])
    }
}
