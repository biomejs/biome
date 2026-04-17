use crate::markdown::lists::inline_item_list::FormatMdFormatInlineItemListOptions;
use crate::prelude::*;
use crate::shared::TextPrintMode;
use biome_formatter::write;
use biome_markdown_syntax::{MdSetextHeader, MdSetextHeaderFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdSetextHeader;
impl FormatNodeRule<MdSetextHeader> for FormatMdSetextHeader {
    fn fmt_fields(&self, node: &MdSetextHeader, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdSetextHeaderFields {
            content,
            underline_token,
        } = node.as_fields();

        let underline_token = underline_token?;

        // h1
        if underline_token.token_text_trimmed().starts_with('=') {
            write!(f, [token("#"),])?;
        }
        // h2
        else {
            write!(f, [token("##"),])?;
        }

        write!(
            f,
            [
                space(),
                content
                    .format()
                    .with_options(FormatMdFormatInlineItemListOptions {
                        print_mode: TextPrintMode::trim_all(),
                        keep_fences_in_italics: false,
                        inside_list: false,
                    }),
                format_removed(&underline_token)
            ]
        )
    }
}
