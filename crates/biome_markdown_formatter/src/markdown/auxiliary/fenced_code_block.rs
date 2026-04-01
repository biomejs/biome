use crate::markdown::lists::inline_item_list::FormatMdFormatInlineItemListOptions;
use crate::prelude::*;
use crate::shared::TextPrintMode;
use biome_formatter::write;
use biome_markdown_syntax::{MdFencedCodeBlock, MdFencedCodeBlockFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdFencedCodeBlock;
impl FormatNodeRule<MdFencedCodeBlock> for FormatMdFencedCodeBlock {
    fn fmt_fields(&self, node: &MdFencedCodeBlock, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdFencedCodeBlockFields {
            l_fence,
            r_fence,
            r_fence_indent,
            content,
            code_list,
            indent,
        } = node.as_fields();

        let l_fence = l_fence?;
        let fence_text = l_fence.text();
        // SAFETY: fence_text has at least one character.
        let fence_char = fence_text.as_bytes()[0] as char;

        // Compute the minimum fence length needed (CommonMark §4.5).
        // The fence must be strictly longer than any same-character sequence
        // in the content, otherwise the inner sequence would be parsed as a
        // closing fence. E.g. if the content contains ``` (3 backticks),
        // the outer fence needs at least 4.
        let max_inner = longest_fence_char_sequence(node, fence_char);
        let fence_len = (max_inner + 1).max(3);
        let normalized_fence: String = std::iter::repeat_n(fence_char, fence_len).collect();

        write!(
            f,
            [
                indent.format(),
                format_replaced(
                    &l_fence,
                    &text(&normalized_fence, l_fence.text_trimmed_range().start())
                ),
                code_list.format(),
                hard_line_break(),
                content
                    .format()
                    .with_options(FormatMdFormatInlineItemListOptions {
                        print_mode: TextPrintMode::Clean
                    }),
                hard_line_break(),
                r_fence_indent.format(),
            ]
        )?;

        if let Ok(r_fence) = r_fence {
            write!(
                f,
                [format_replaced(
                    &r_fence,
                    &text(&normalized_fence, r_fence.text_trimmed_range().start())
                )]
            )?;
        }

        Ok(())
    }
}

/// Find the longest consecutive run of `fence_char` in the code block's content.
fn longest_fence_char_sequence(node: &MdFencedCodeBlock, fence_char: char) -> usize {
    let content = node.content();
    let mut max_len = 0usize;

    for item in content.iter() {
        if let Some(textual) = item.as_md_textual()
            && let Ok(token) = textual.value_token()
        {
            let mut consecutive_count = 0usize;
            for ch in token.text().chars() {
                if ch == fence_char {
                    consecutive_count += 1;
                    max_len = max_len.max(consecutive_count);
                } else {
                    consecutive_count = 0;
                }
            }
        }
    }

    max_len
}
