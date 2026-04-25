use crate::markdown::lists::inline_item_list::FormatMdFormatInlineItemListOptions;
use crate::prelude::*;
use crate::shared::TextPrintMode;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{MdFencedCodeBlock, MdFencedCodeBlockFields};
use biome_rowan::TextSize;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdFencedCodeBlock {
    /// Whether the fenced code block is inside a list.
    /// When inside a list
    inside_list: bool,
}

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

        // Compute the minimum fence length needed (CommonMark §4.5).
        // The fence must be strictly longer than any same-character sequence
        // in the content, otherwise the inner sequence would be parsed as a
        // closing fence. E.g. if the content contains ``` (3 backticks),
        // the outer fence needs at least 4.
        let max_inner = longest_fence_char_sequence(node, '`');
        let fence_len = (max_inner + 1).max(3);
        let normalized_fence: String = std::iter::repeat_n('`', fence_len).collect();

        // Spaces to remove in case we're inside a list
        let excess = if self.inside_list { indent.len() } else { 0 };

        if excess > 0 {
            for token in indent.iter() {
                let char_token = token.md_indent_char_token()?;
                f.context()
                    .comments()
                    .mark_suppression_checked(token.syntax());
                write!(f, [format_removed(&char_token)])?;
            }
        } else {
            write!(f, [indent.format()])?;
        }

        write!(
            f,
            [
                format_replaced(
                    &l_fence,
                    &text(&normalized_fence, l_fence.text_trimmed_range().start())
                ),
                code_list.format(),
                hard_line_break(),
                content
                    .format()
                    .with_options(FormatMdFormatInlineItemListOptions {
                        print_mode: TextPrintMode::Clean,
                        keep_fences_in_italics: false,
                        inside_list: false,
                    }),
            ]
        )?;

        // The closing fence's indentation is stored entirely in r_fence_indent
        // (unlike the opening fence, there is no separate continuation-indent
        // node preceding it in the block list). Remove the same number of
        // excess spaces that were removed from the opening fence.
        let r_fence_tokens: Vec<_> = r_fence_indent.iter().collect();
        for token in r_fence_tokens.iter().take(excess) {
            let char_token = token.md_indent_char_token()?;
            f.context()
                .comments()
                .mark_suppression_checked(token.syntax());
            write!(f, [format_removed(&char_token)])?;
        }
        for token in r_fence_tokens.iter().skip(excess) {
            write!(f, [token.format()])?;
        }

        if let Ok(r_fence) = r_fence {
            write!(
                f,
                [format_replaced(
                    &r_fence,
                    &text(&normalized_fence, r_fence.text_trimmed_range().start())
                )]
            )?;
        } else {
            write!(f, [text(&normalized_fence, TextSize::default())])?;
        }

        Ok(())
    }
}

pub(crate) struct FormatMdFencedCodeBlockOptions {
    pub(crate) inside_list: bool,
}

impl FormatRuleWithOptions<MdFencedCodeBlock> for FormatMdFencedCodeBlock {
    type Options = FormatMdFencedCodeBlockOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.inside_list = options.inside_list;
        self
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
