use crate::markdown::lists::inline_item_list::FormatMdFormatInlineItemListOptions;
use crate::prelude::*;
use crate::shared::{TextPrintMode, TrimMode};
use biome_formatter::write;
use biome_markdown_syntax::{MdInlineLink, MdInlineLinkFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineLink;
impl FormatNodeRule<MdInlineLink> for FormatMdInlineLink {
    fn fmt_fields(&self, node: &MdInlineLink, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdInlineLinkFields {
            title,
            text,
            destination,
            r_brack_token,
            r_paren_token,
            l_brack_token,
            l_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_brack_token.format(),
                text.format()
                    .with_options(FormatMdFormatInlineItemListOptions {
                        print_mode: TextPrintMode::Trim(TrimMode::All)
                    }),
                r_brack_token.format(),
                l_paren_token.format(),
                destination
                    .format()
                    .with_options(FormatMdFormatInlineItemListOptions {
                        print_mode: TextPrintMode::Trim(TrimMode::All)
                    })
            ]
        )?;
        if let Some(title) = title {
            write!(f, [space(), title.format()])?;
        }

        write!(f, [r_paren_token.format()])
    }
}
