use crate::markdown::lists::inline_item_list::FormatMdFormatInlineItemListOptions;
use crate::prelude::*;
use crate::shared::{TextPrintMode, TrimMode};
use biome_markdown_syntax::{MdLinkDestination, MdLinkDestinationFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdLinkDestination;
impl FormatNodeRule<MdLinkDestination> for FormatMdLinkDestination {
    fn fmt_fields(&self, node: &MdLinkDestination, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdLinkDestinationFields { content } = node.as_fields();

        content
            .format()
            .with_options(FormatMdFormatInlineItemListOptions {
                print_mode: TextPrintMode::Trim(TrimMode::All),
            })
            .fmt(f)
    }
}
