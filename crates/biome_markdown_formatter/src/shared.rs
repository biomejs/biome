use crate::markdown::auxiliary::newline::FormatMdNewlineOptions;
use crate::markdown::auxiliary::quote_prefix::FormatMdQuotePrefixOptions;
use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{AnyMdBlock, AnyMdLeafBlock, MarkdownSyntaxToken};
use biome_rowan::TextSize;
use std::borrow::Cow;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub(crate) enum TextPrintMode {
    /// Keep the original formatting. Don't attempt to optimize it. This is usually achieved
    /// by formatting the code as verbatim.
    #[default]
    Pristine,
    /// Usually used inside code blocks. It keeps the original formatting of the content,
    /// but it removes possible spaces if there's empty hard line.
    ///
    /// In the following example, the first line will keep only the newline, as there are only spaces
    /// ``````md
    /// ```
    ///
    /// ```
    /// ``````
    /// However, in the following example, spaces are keep as is because there's text (code):
    ///
    /// ``````md
    /// ```js
    ///    function f() {}
    /// ```
    /// ``````
    Clean,
    /// It removes the token/node
    Remove,
    /// It cleans the code by using a trimming strategy
    Trim(TrimMode),
    /// Split prose into words and emit fill IR for line-width-aware wrapping.
    Fill,
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub(crate) enum TrimMode {
    /// Trim the start of the list
    Start,
    /// Trim start and end of the list
    All,
    /// If the first and last [MdTextual] are `<` and `>` respectively, they are trimmed.
    /// If no link has been detected, if falls back to [Self::All]
    AutoLinkLike,
    /// This mode works similarly to [TrimMode::All], however, text that contains
    /// words and have more than trailing/leading spaces are normalized to one
    NormalizeWords,
    /// Don't trim anything
    #[default]
    None,
}

/// Where the inline text being formatted is located in the document structure.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub(crate) enum TextContext {
    #[default]
    Neutral,
    List,
    Header,
}

impl TextContext {
    pub(crate) const fn is_list(&self) -> bool {
        matches!(self, Self::List)
    }
}

impl TextPrintMode {
    pub(crate) const fn is_trim_start(&self) -> bool {
        matches!(self, Self::Trim(TrimMode::Start))
    }

    pub(crate) const fn is_trim_all(&self) -> bool {
        matches!(self, Self::Trim(TrimMode::All))
    }

    pub(crate) const fn is_normalize_words(&self) -> bool {
        matches!(self, Self::Trim(TrimMode::NormalizeWords))
    }

    pub(crate) const fn is_auto_link_like(&self) -> bool {
        matches!(self, Self::Trim(TrimMode::AutoLinkLike))
    }

    pub(crate) const fn is_pristine(&self) -> bool {
        matches!(self, Self::Pristine)
    }

    pub(crate) const fn is_clean(&self) -> bool {
        matches!(self, Self::Clean)
    }

    pub(crate) const fn is_remove(&self) -> bool {
        matches!(self, Self::Remove)
    }

    pub(crate) const fn is_fill(&self) -> bool {
        matches!(self, Self::Fill)
    }

    pub(crate) const fn trim_all() -> Self {
        Self::Trim(TrimMode::All)
    }

    pub(crate) const fn fill() -> Self {
        Self::Fill
    }
}

pub(crate) fn format_removed_quote_boundary(
    node: &AnyMdBlock,
    f: &mut MarkdownFormatter,
) -> FormatResult<()> {
    match node {
        AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdNewline(newline)) => {
            write!(
                f,
                [newline.format().with_options(FormatMdNewlineOptions {
                    print_mode: TextPrintMode::Remove,
                })]
            )
        }
        AnyMdBlock::MdQuotePrefix(prefix) => write!(
            f,
            [prefix.format().with_options(FormatMdQuotePrefixOptions {
                should_remove: true,
            })]
        ),
        _ => write!(f, [node.format()]),
    }
}

/// Writes the lines of a block's literal token as they're written in the
/// source, such as the code of a fenced code block.
///
/// The literal starts with the line break that ends the block's opening line.
/// That line break isn't written, because the node that writes the opening
/// line ends it. Each line loses up to `max_indent` leading spaces, and line
/// breaks are written as literal line breaks.
pub(crate) struct FormatLiteralLines<'a> {
    pub(crate) token: &'a MarkdownSyntaxToken,
    pub(crate) max_indent: usize,
}

impl Format<MarkdownFormatContext> for FormatLiteralLines<'_> {
    fn fmt(&self, f: &mut MarkdownFormatter) -> FormatResult<()> {
        // Trivia is excluded on both sides: the opening fence line's trimmed
        // info-string whitespace is attached to the token as leading
        // whitespace trivia, and printing it would insert it as an extra
        // content line.
        let text = self.token.text_trimmed();
        let bytes = text.as_bytes();
        let token_start = self.token.text_trimmed_range().start();
        let format_slice = |start: usize, end: usize, f: &mut MarkdownFormatter| {
            syntax_token_cow_slice(
                Cow::Borrowed(&text[start..end]),
                self.token,
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
            let max_content_start = line_start + self.max_indent;
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
    }
}
