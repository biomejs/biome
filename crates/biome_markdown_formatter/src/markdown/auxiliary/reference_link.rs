use crate::markdown::lists::inline_item_list::FormatMdFormatInlineItemListOptions;
use crate::prelude::*;
use crate::shared::TextPrintMode;
use biome_formatter::write;
use biome_markdown_syntax::{MdReferenceLink, MdReferenceLinkFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdReferenceLink;
impl FormatNodeRule<MdReferenceLink> for FormatMdReferenceLink {
    fn fmt_fields(&self, node: &MdReferenceLink, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdReferenceLinkFields {
            l_brack_token,
            text,
            r_brack_token,
            label,
        } = node.as_fields();

        write!(
            f,
            [
                l_brack_token.format(),
                text.format()
                    .with_options(FormatMdFormatInlineItemListOptions {
                        print_mode: TextPrintMode::trim_all(),
                        keep_fences_in_italics: true
                    }),
                r_brack_token.format(),
            ]
        )?;

        if let Some(label) = label {
            write!(f, [label.format()])?;
        }

        Ok(())
    }
}
