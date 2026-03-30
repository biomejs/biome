use crate::markdown::lists::inline_item_list::FormatMdFormatInlineItemListOptions;
use crate::prelude::*;
use crate::shared::{TextPrintMode, TrimMode};
use biome_formatter::write;
use biome_markdown_syntax::{MdLinkTitle, MdLinkTitleFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdLinkTitle;
impl FormatNodeRule<MdLinkTitle> for FormatMdLinkTitle {
    fn fmt_fields(&self, node: &MdLinkTitle, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdLinkTitleFields { content } = node.as_fields();

        write!(
            f,
            [content
                .format()
                .with_options(FormatMdFormatInlineItemListOptions {
                    print_mode: TextPrintMode::Trim(TrimMode::All)
                })]
        )
    }
}
