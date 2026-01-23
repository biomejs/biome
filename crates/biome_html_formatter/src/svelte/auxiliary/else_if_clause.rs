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
                space(),
                expression.format(),
                r_curly_token.format()
            ]
        )?;

        FormatHtmlElementList::default()
            .with_multiline()
            .fmt(&children, f)?;

        write!(f, [hard_line_break(),])
    }
}
