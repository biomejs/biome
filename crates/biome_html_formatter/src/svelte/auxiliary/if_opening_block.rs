use crate::html::lists::element_list::FormatHtmlElementList;
use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteIfOpeningBlock, SvelteIfOpeningBlockFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteIfOpeningBlock;
impl FormatNodeRule<SvelteIfOpeningBlock> for FormatSvelteIfOpeningBlock {
    fn fmt_fields(&self, node: &SvelteIfOpeningBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteIfOpeningBlockFields {
            children,
            r_curly_token,
            if_token,
            expression,
            sv_curly_hash_token,
        } = node.as_fields();

        write!(
            f,
            [
                sv_curly_hash_token.format(),
                if_token.format(),
                expression.format(),
                r_curly_token.format(),
            ]
        )?;

        // The order here is important. First, we must check if we can delegate the formatting
        // of embedded nodes, then we check if we should format them verbatim.
        let format_children = FormatHtmlElementList::default()
            .with_group_id(f.group_id("svelte-if-group"))
            .fmt_children(&children, f)?;

        write!(f, [format_children, hard_line_break()])
    }
}
