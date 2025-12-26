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
                space(),
                expression.format(),
                r_curly_token.format(),
            ]
        )?;

        let format_children = FormatHtmlElementList::default()
            .with_multiline()
            .fmt_children(&children, f)?;

        write!(f, [format_children, hard_line_break()])
    }
}
