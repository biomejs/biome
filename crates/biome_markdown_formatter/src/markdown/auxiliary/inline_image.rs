use crate::markdown::lists::inline_item_list::FormatMdFormatInlineItemListOptions;
use crate::prelude::*;
use crate::shared::{TextPrintMode, TrimMode};
use biome_formatter::write;
use biome_markdown_syntax::{MdInlineImage, MdInlineImageFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineImage;
impl FormatNodeRule<MdInlineImage> for FormatMdInlineImage {
    fn fmt_fields(&self, node: &MdInlineImage, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdInlineImageFields {
            excl_token,
            l_brack_token,
            alt,
            r_brack_token,
            l_paren_token,
            destination,
            title,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                excl_token.format(),
                l_brack_token.format(),
                alt.format()
                    .with_options(FormatMdFormatInlineItemListOptions {
                        print_mode: TextPrintMode::Trim(TrimMode::All)
                    }),
                r_brack_token.format(),
                l_paren_token.format(),
                destination
                    .format()
                    .with_options(FormatMdFormatInlineItemListOptions {
                        print_mode: TextPrintMode::Trim(TrimMode::AutoLinkLike)
                    })
            ]
        )?;

        if let Some(title) = title {
            write!(f, [space(), title.format()])?;
        }

        write!(f, [r_paren_token.format()])
    }
}
