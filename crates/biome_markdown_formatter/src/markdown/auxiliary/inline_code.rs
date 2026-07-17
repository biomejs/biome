use crate::markdown::lists::inline_item_list::FormatMdFormatInlineItemListOptions;
use crate::prelude::*;
use crate::shared::{TextContext, TextPrintMode};
use biome_formatter::write;
use biome_markdown_syntax::{
    AnyMdInline, MarkdownSyntaxToken, MdInlineCode, MdInlineCodeFields, MdInlineItemList,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineCode;
impl FormatNodeRule<MdInlineCode> for FormatMdInlineCode {
    fn fmt_fields(&self, node: &MdInlineCode, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdInlineCodeFields {
            l_tick_token,
            content,
            r_tick_token,
        } = node.as_fields();
        let l_tick_token = l_tick_token?;
        let r_tick_token = r_tick_token?;

        write!(
            f,
            [
                format_code_fence(&l_tick_token, &content),
                content
                    .format()
                    .with_options(FormatMdFormatInlineItemListOptions {
                        print_mode: TextPrintMode::Pristine,
                        keep_fences_in_italics: false,
                        text_context: TextContext::Neutral,
                    }),
                format_code_fence(&r_tick_token, &content)
            ]
        )
    }
}

fn format_code_fence<'a>(
    fence: &'a MarkdownSyntaxToken,
    content: &'a MdInlineItemList,
) -> impl Format<MarkdownFormatContext> + 'a {
    format_with(move |f| {
        let Some(fence_len) = shortest_unused_backtick_sequence_len(content) else {
            return write!(f, [fence.format()]);
        };

        if fence.text_trimmed().len() == fence_len {
            write!(f, [fence.format()])
        } else {
            let replacement = format_with(|f| {
                for _ in 0..fence_len {
                    write!(f, [token("`")])?;
                }
                Ok(())
            });
            write!(f, [format_replaced(fence, &replacement)])
        }
    })
}

fn shortest_unused_backtick_sequence_len(content: &MdInlineItemList) -> Option<usize> {
    if inline_code_content_has_newline(content)? {
        return None;
    }
    if !has_backtick(content)? {
        return None;
    }

    let mut fence_len = 1;
    while has_backtick_run(content, fence_len)? {
        fence_len += 1;
    }

    Some(fence_len)
}

fn inline_code_content_has_newline(content: &MdInlineItemList) -> Option<bool> {
    for item in content.iter() {
        match item {
            AnyMdInline::MdTextual(text) => {
                if text.value_token().ok()?.text().contains(['\n', '\r']) {
                    return Some(true);
                }
            }
            _ => return None,
        }
    }

    Some(false)
}

fn has_backtick(content: &MdInlineItemList) -> Option<bool> {
    for item in content.iter() {
        match item {
            AnyMdInline::MdTextual(text) => {
                if text.value_token().ok()?.text().contains('`') {
                    return Some(true);
                }
            }
            _ => return None,
        }
    }

    Some(false)
}

fn has_backtick_run(content: &MdInlineItemList, expected_len: usize) -> Option<bool> {
    let mut current_run = 0;

    for item in content.iter() {
        match item {
            AnyMdInline::MdTextual(text) => {
                for char in text.value_token().ok()?.text().chars() {
                    if char == '`' {
                        current_run += 1;
                    } else if current_run > 0 {
                        if current_run == expected_len {
                            return Some(true);
                        }
                        current_run = 0;
                    }
                }
            }
            _ => return None,
        }
    }

    Some(current_run == expected_len)
}
