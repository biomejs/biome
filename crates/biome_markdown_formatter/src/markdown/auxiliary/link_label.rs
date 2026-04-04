use crate::markdown::lists::inline_item_list::FormatMdFormatInlineItemListOptions;
use crate::prelude::*;
use crate::shared::{TextPrintMode, TrimMode};
use biome_markdown_syntax::MdLinkLabel;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdLinkLabel;
impl FormatNodeRule<MdLinkLabel> for FormatMdLinkLabel {
    fn fmt_fields(&self, node: &MdLinkLabel, f: &mut MarkdownFormatter) -> FormatResult<()> {
        node.content()
            .format()
            .with_options(FormatMdFormatInlineItemListOptions {
                print_mode: TextPrintMode::Trim(TrimMode::NormalizeWords),
            })
            .fmt(f)
    }
}
