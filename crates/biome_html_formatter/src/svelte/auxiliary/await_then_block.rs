use crate::html::lists::element_list::FormatHtmlElementList;
use crate::prelude::*;
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

        write!(
            f,
            [
                sv_curly_colon_token.format(),
                then_token.format(),
                space(),
                name.format(),
                r_curly_token.format()
            ]
        )?;
        // The order here is important. First, we must check if we can delegate the formatting
        // of embedded nodes, then we check if we should format them verbatim.
        let format_children = FormatHtmlElementList::default()
            .with_group_id(f.group_id("svelte-then-group"))
            .fmt_children(&children, f)?;

        write!(f, [format_children, hard_line_break()])
    }
}
