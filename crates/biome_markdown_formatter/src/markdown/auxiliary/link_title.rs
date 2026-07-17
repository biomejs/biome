use crate::markdown::lists::inline_item_list::FormatMdFormatInlineItemListOptions;
use crate::prelude::*;
use crate::shared::{TextContext, TextPrintMode};
use biome_formatter::write;
use biome_markdown_syntax::{
    AnyMdInline, MarkdownSyntaxToken, MdLinkTitle, MdLinkTitleFields, inner_string_text,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdLinkTitle;
impl FormatNodeRule<MdLinkTitle> for FormatMdLinkTitle {
    fn fmt_fields(&self, node: &MdLinkTitle, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdLinkTitleFields { content } = node.as_fields();

        if is_empty_link_title(node)? {
            return write!(
                f,
                [content
                    .format()
                    .with_options(FormatMdFormatInlineItemListOptions {
                        print_mode: TextPrintMode::Remove,
                        keep_fences_in_italics: false,
                        text_context: TextContext::Neutral,
                    })]
            );
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
                        text_context: TextContext::Neutral,
                    })
            ]
        )
    }
}

fn is_empty_link_title(title: &MdLinkTitle) -> FormatResult<bool> {
    let mut has_empty_quoted_title = false;

    for item in title.content().iter() {
        let AnyMdInline::MdTextual(textual) = item else {
            return Ok(false);
        };
        let token = textual.value_token()?;

        if token.token_text_trimmed().text().trim().is_empty() {
            continue;
        }

        if is_empty_quoted_title_token(&token) {
            has_empty_quoted_title = true;
        } else {
            return Ok(false);
        }
    }

    Ok(has_empty_quoted_title)
}

fn is_empty_quoted_title_token(token: &MarkdownSyntaxToken) -> bool {
    let text = token.token_text_trimmed().trim_token();
    let text = text.text();

    text.len() >= 2
        && (text.starts_with('"') && text.ends_with('"')
            || text.starts_with('\'') && text.ends_with('\''))
        && inner_string_text(token).is_empty()
}
