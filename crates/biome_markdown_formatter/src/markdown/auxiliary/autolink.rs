use crate::markdown::lists::inline_item_list::FormatMdFormatInlineItemListOptions;
use crate::prelude::*;
use crate::shared::{TextPrintMode, TrimMode};
use biome_formatter::write;
use biome_markdown_syntax::{MdAutolink, MdAutolinkFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdAutolink;
impl FormatNodeRule<MdAutolink> for FormatMdAutolink {
    fn fmt_fields(&self, node: &MdAutolink, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdAutolinkFields {
            l_angle_token,
            value,
            r_angle_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_angle_token.format(),
                value
                    .format()
                    .with_options(FormatMdFormatInlineItemListOptions {
                        print_mode: TextPrintMode::Trim(TrimMode::NormalizeWords)
                    }),
                r_angle_token.format()
            ]
        )
    }
}
