use crate::markdown::lists::inline_item_list::FormatMdFormatInlineItemListOptions;
use crate::prelude::*;
use crate::shared::TextPrintMode;
use biome_formatter::write;
use biome_markdown_syntax::{MdInlineCode, MdInlineCodeFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineCode;
impl FormatNodeRule<MdInlineCode> for FormatMdInlineCode {
    fn fmt_fields(&self, node: &MdInlineCode, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdInlineCodeFields {
            l_tick_token,
            content,
            r_tick_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_tick_token.format(),
                content
                    .format()
                    .with_options(FormatMdFormatInlineItemListOptions {
                        print_mode: TextPrintMode::Pristine
                    }),
                r_tick_token.format()
            ]
        )
    }
}
