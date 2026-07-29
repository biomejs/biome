use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{MdCodeContent, MdCodeContentFields};
use biome_rowan::TextSize;
use std::borrow::Cow;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdCodeContent {
    opening_fence_indent: usize,
}
impl FormatNodeRule<MdCodeContent> for FormatMdCodeContent {
    fn fmt_fields(&self, node: &MdCodeContent, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdCodeContentFields { value_token } = node.as_fields();
        let value_token = value_token?;

        // The literal starts with the newline that ends the opening-fence
        // line. The fenced code block formatter writes that line break and
        // normalizes the optional fence indentation separately.
        let replacement = format_with(|f| {
            let text = value_token.text();
            let bytes = text.as_bytes();
            let token_start = value_token.text_range().start();
            let format_slice = |start: usize, end: usize, f: &mut MarkdownFormatter| {
                syntax_token_cow_slice(
                    Cow::Borrowed(&text[start..end]),
                    &value_token,
                    token_start + TextSize::from(start as u32),
                )
                .with_literal_line_breaks()
                .fmt(f)
            };
            let mut line_start = match bytes {
                [b'\r', b'\n', ..] => 2,
                [b'\r' | b'\n', ..] => 1,
                _ => 0,
            };

            while line_start < bytes.len() {
                let mut content_start = line_start;
                let max_content_start = line_start + self.opening_fence_indent;
                while content_start < bytes.len()
                    && content_start < max_content_start
                    && bytes[content_start] == b' '
                {
                    content_start += 1;
                }

                let mut line_end = content_start;
                while line_end < bytes.len() && !matches!(bytes[line_end], b'\r' | b'\n') {
                    line_end += 1;
                }

                match bytes.get(line_end) {
                    Some(b'\n') => {
                        format_slice(content_start, line_end + 1, f)?;
                        line_start = line_end + 1;
                    }
                    Some(b'\r') => {
                        if content_start < line_end {
                            format_slice(content_start, line_end, f)?;
                        }

                        if bytes.get(line_end + 1) == Some(&b'\n') {
                            format_slice(line_end + 1, line_end + 2, f)?;
                            line_start = line_end + 2;
                        } else {
                            literal_line_break_without_parent().fmt(f)?;
                            line_start = line_end + 1;
                        }
                    }
                    _ => {
                        if content_start < line_end {
                            format_slice(content_start, line_end, f)?;
                        }
                        break;
                    }
                }
            }

            Ok(())
        });

        write!(f, [format_replaced(&value_token, &replacement)])
    }
}

pub(crate) struct FormatMdCodeContentOptions {
    pub(crate) opening_fence_indent: usize,
}

impl FormatRuleWithOptions<MdCodeContent> for FormatMdCodeContent {
    type Options = FormatMdCodeContentOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.opening_fence_indent = options.opening_fence_indent;
        self
    }
}
