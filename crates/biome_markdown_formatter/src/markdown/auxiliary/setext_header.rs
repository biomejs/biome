use crate::markdown::lists::inline_item_list::FormatMdFormatInlineItemListOptions;
use crate::prelude::*;
use crate::shared::{TextContext, TextPrintMode};
use biome_formatter::write;
use biome_markdown_syntax::{MdSetextHeader, MdSetextHeaderFields};

use crate::context::ProseWrap;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdSetextHeader;
impl FormatNodeRule<MdSetextHeader> for FormatMdSetextHeader {
    fn fmt_fields(&self, node: &MdSetextHeader, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdSetextHeaderFields {
            content,
            underline_token,
        } = node.as_fields();

        let underline_token = underline_token?;

        if setext_header_keeps_underline(node, f) {
            write!(
                f,
                [
                    content
                        .format()
                        .with_options(FormatMdFormatInlineItemListOptions {
                            print_mode: TextPrintMode::fill(),
                            keep_fences_in_italics: false,
                            text_context: TextContext::Neutral,
                        }),
                    underline_token.format()
                ]
            )
        } else {
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
                            print_mode: TextPrintMode::fill(),
                            keep_fences_in_italics: false,
                            text_context: TextContext::Neutral,
                        }),
                    format_removed(&underline_token),
                ]
            )
        }
    }
}

/// Whether `node` is printed with its underline. Otherwise, it is printed as
/// an ATX heading, such as `## Heading`.
pub(crate) fn setext_header_keeps_underline(node: &MdSetextHeader, f: &MarkdownFormatter) -> bool {
    let prose_wrap = f.options().prose_wrap();
    prose_wrap == ProseWrap::Always
        || prose_wrap == ProseWrap::Preserve && node.content().will_break()
}
