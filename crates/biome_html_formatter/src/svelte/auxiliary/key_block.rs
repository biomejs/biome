use crate::html::lists::element_list::{FormatChildrenResult, FormatHtmlElementList};
use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_html_syntax::{SvelteKeyBlock, SvelteKeyBlockFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteKeyBlock;
impl FormatNodeRule<SvelteKeyBlock> for FormatSvelteKeyBlock {
    fn fmt_fields(&self, node: &SvelteKeyBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteKeyBlockFields {
            opening_block,
            children,
            closing_block,
        } = node.as_fields();

        write!(f, [opening_block.format(),])?;
        // The order here is important. First, we must check if we can delegate the formatting
        // of embedded nodes, then we check if we should format them verbatim.
        let format_children = FormatHtmlElementList::default().fmt_children(&children, f)?;
        let attr_group_id = f.group_id("element-attr-group-id");

        match format_children {
            FormatChildrenResult::ForceMultiline(multiline) => {
                write!(f, [multiline])?;
            }
            FormatChildrenResult::BestFitting {
                flat_children,
                expanded_children,
            } => {
                let expanded_children = expanded_children.memoized();
                write!(
                    f,
                    [
                        // If the attribute group breaks, prettier always breaks the children as well.
                        &if_group_breaks(&expanded_children).with_group_id(Some(attr_group_id)),
                        // If the attribute group does NOT break, print whatever fits best for the children.
                        &if_group_fits_on_line(&best_fitting![
                            format_args![flat_children],
                            format_args![expanded_children],
                        ])
                        .with_group_id(Some(attr_group_id)),
                    ]
                )?;
            }
        }

        write!(f, [closing_block.format()])
    }
}
