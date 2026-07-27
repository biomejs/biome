use crate::markdown::auxiliary::code_content::FormatMdCodeContentOptions;
use crate::markdown::lists::inline_item_list::FormatMdFormatInlineItemListOptions;
use crate::prelude::*;
use crate::shared::{TextContext, TextPrintMode};
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{AnyMdInline, MdFencedCodeBlock, MdFencedCodeBlockFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdFencedCodeBlock {
    text_context: TextContext,
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

        let inside_list = self.text_context.is_list();
        let has_code_content = content
            .iter()
            .any(|item| matches!(item, AnyMdInline::MdCodeContent(_)));
        let opening_fence_indent = indent
            .iter()
            .map(|token| token.md_indent_char_token().map(|token| token.text().len()))
            .sum::<Result<usize, _>>()?;

        for token in indent.iter() {
            let char_token = token.md_indent_char_token()?;
            f.context()
                .comments()
                .mark_suppression_checked(token.syntax());
            write!(f, [format_removed(&char_token)])?;
        }

        write!(
            f,
            [
                format_replaced(
                    &l_fence,
                    &text(
                        &normalized_fence,
                        Some(l_fence.text_trimmed_range().start())
                    )
                ),
                code_list.format(),
                hard_line_break(),
                format_with(|f| {
                    if !has_code_content {
                        content
                            .format()
                            .with_options(FormatMdFormatInlineItemListOptions {
                                print_mode: if inside_list {
                                    TextPrintMode::Fill
                                } else {
                                    TextPrintMode::Clean
                                },
                                keep_fences_in_italics: false,
                                text_context: self.text_context,
                            })
                            .fmt(f)
                    } else {
                        for item in content.iter() {
                            match item {
                                AnyMdInline::MdCodeContent(code) => code
                                    .format()
                                    .with_options(FormatMdCodeContentOptions {
                                        opening_fence_indent,
                                    })
                                    .fmt(f)?,
                                item => item.format().fmt(f)?,
                            }
                        }
                        Ok(())
                    }
                }),
            ]
        )?;

        for token in r_fence_indent.iter() {
            let char_token = token.md_indent_char_token()?;
            f.context()
                .comments()
                .mark_suppression_checked(token.syntax());
            write!(f, [format_removed(&char_token)])?;
        }

        if let Some(r_fence) = r_fence {
            write!(
                f,
                [format_replaced(
                    &r_fence,
                    &text(
                        &normalized_fence,
                        Some(r_fence.text_trimmed_range().start())
                    )
                )]
            )?;
        } else {
            write!(f, [text(&normalized_fence, None)])?;
        }

        Ok(())
    }
}

pub(crate) struct FormatMdFencedCodeBlockOptions {
    pub(crate) text_context: TextContext,
}

impl FormatRuleWithOptions<MdFencedCodeBlock> for FormatMdFencedCodeBlock {
    type Options = FormatMdFencedCodeBlockOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.text_context = options.text_context;
        self
    }
}

/// Find the longest consecutive run of `fence_char` in the code block's content.
fn longest_fence_char_sequence(node: &MdFencedCodeBlock, fence_char: char) -> usize {
    let content = node.content();
    let mut max_len = 0usize;

    for item in content.iter() {
        // Document-level blocks store the whole content in one MdCodeContent
        // literal; nested blocks keep per-line MdTextual nodes.
        let token = match &item {
            AnyMdInline::MdTextual(textual) => textual.value_token(),
            AnyMdInline::MdCodeContent(code) => code.value_token(),
            _ => continue,
        };
        let Ok(token) = token else {
            continue;
        };

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

    max_len
}
