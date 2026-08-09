use crate::context::ProseWrap;
use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{
    AnyMdInline, MarkdownSyntaxToken, MdInlineCode, MdInlineCodeFields, MdInlineImage,
    MdInlineItemList, MdInlineLink, MdLinkReferenceDefinition, MdReferenceImage, MdReferenceLink,
    MdTextual,
};
use biome_rowan::{AstNode, TextSize};
use std::borrow::Cow;

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
        let Some(normalization) = CodeSpanNormalization::from_content(node, &content) else {
            return write!(
                f,
                [
                    l_tick_token.format(),
                    content.format(),
                    r_tick_token.format()
                ]
            );
        };

        write!(
            f,
            [
                format_code_fence(&l_tick_token, normalization.fence_len),
                normalization,
                format_code_fence(&r_tick_token, normalization.fence_len)
            ]
        )
    }
}

fn format_code_fence(
    fence: &MarkdownSyntaxToken,
    fence_len: usize,
) -> impl Format<MarkdownFormatContext> + '_ {
    format_with(move |f| {
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

/// Serializes a Markdown code span without changing the code string defined by CommonMark.
///
/// CommonMark interprets line endings in a code span as spaces. If the interpreted code string
/// starts and ends with a space but does not consist entirely of spaces, it also removes one space
/// from each end. The formatter applies that boundary removal directly to the source, using one
/// space or line ending as a boundary unit.
///
/// The formatter then uses the shortest backtick fence length that does not occur as a sequence of
/// exactly that many consecutive backticks in the content. It inserts one space inside both fences
/// when the remaining content starts or ends with a backtick, or when it starts and ends with a
/// space. These padding spaces are consumed by CommonMark's code-span rules, preventing them from
/// changing the interpreted code string.
///
/// Construction returns `None` for non-textual content and for code spans nested in links, images,
/// or reference definitions. Those nodes retain their source fences and content instead of using
/// this serialization.
///
/// See <https://spec.commonmark.org/0.30/#code-spans>.
struct CodeSpanNormalization {
    /// Every textual node between the opening and closing backtick fences, in source order.
    textuals: Vec<MdTextual>,
    /// Whether the first and last boundary units are removed because both are spaces or line
    /// endings and the content contains a non-space character.
    trim_boundaries: bool,
    /// Whether one space is emitted after the opening fence and before the closing fence so a
    /// boundary backtick or pair of boundary spaces remains part of the interpreted code string.
    needs_padding: bool,
    /// The shortest fence length absent from the content as an equally long sequence of consecutive
    /// backticks.
    fence_len: usize,
}

impl CodeSpanNormalization {
    const SPACE_OR_LINE_ENDING_CHARS: [char; 3] = [' ', '\r', '\n'];

    fn from_content(node: &MdInlineCode, content: &MdInlineItemList) -> Option<Self> {
        if node.syntax().ancestors().skip(1).any(|ancestor| {
            MdInlineImage::can_cast(ancestor.kind())
                || MdInlineLink::can_cast(ancestor.kind())
                || MdLinkReferenceDefinition::can_cast(ancestor.kind())
                || MdReferenceImage::can_cast(ancestor.kind())
                || MdReferenceLink::can_cast(ancestor.kind())
        }) {
            return None;
        }

        let textuals = content
            .iter()
            .map(|item| match item {
                AnyMdInline::MdTextual(textual) => Some(textual),
                _ => None,
            })
            .collect::<Option<Vec<_>>>()?;
        let has_non_space = textuals.iter().try_fold(false, |has_non_space, textual| {
            Some(
                has_non_space
                    || textual
                        .value_token()
                        .ok()?
                        .text()
                        .bytes()
                        .any(|byte| !matches!(byte, b' ' | b'\r' | b'\n')),
            )
        })?;
        let first_token = textuals.first()?.value_token().ok()?;
        let last_token = textuals.last()?.value_token().ok()?;
        let trim_boundaries = Self::leading_space_or_line_ending_len(first_token.text()) > 0
            && Self::trailing_space_or_line_ending_len(last_token.text()) > 0
            && has_non_space;
        let (first, last) = Self::normalized_boundary_chars(&textuals, trim_boundaries)?;
        let needs_padding =
            first == '`' || last == '`' || (first == ' ' && last == ' ' && has_non_space);

        Some(Self {
            textuals,
            trim_boundaries,
            needs_padding,
            fence_len: Self::shortest_unused_backtick_sequence_len(content)?,
        })
    }

    /// Returns the shortest fence length that cannot close the code span from within its content.
    ///
    /// A backtick sequence closes a code span only when its length equals the opening fence length,
    /// so shorter or longer sequences do not disqualify a candidate.
    fn shortest_unused_backtick_sequence_len(content: &MdInlineItemList) -> Option<usize> {
        let mut fence_len = 1;
        'candidate: loop {
            let mut consecutive_backticks = 0;

            for item in content.iter() {
                match item {
                    AnyMdInline::MdTextual(text) => {
                        for char in text.value_token().ok()?.text().chars() {
                            if char == '`' {
                                consecutive_backticks += 1;
                            } else if consecutive_backticks > 0 {
                                if consecutive_backticks == fence_len {
                                    fence_len += 1;
                                    continue 'candidate;
                                }
                                consecutive_backticks = 0;
                            }
                        }
                    }
                    _ => return None,
                }
            }

            if consecutive_backticks != fence_len {
                return Some(fence_len);
            }
            fence_len += 1;
        }
    }

    /// Returns the first and last content characters after prospective boundary trimming.
    ///
    /// Here, normalization means removing one leading and trailing space or line ending when
    /// `trim_boundaries` is set. These characters determine whether the formatted code span needs
    /// padding between its content and backtick fences.
    fn normalized_boundary_chars(
        textuals: &[MdTextual],
        trim_boundaries: bool,
    ) -> Option<(char, char)> {
        let first = textuals.iter().enumerate().find_map(|(index, textual)| {
            let token = textual.value_token().ok()?;
            let start = if trim_boundaries && index == 0 {
                Self::leading_space_or_line_ending_len(token.text())
            } else {
                0
            };
            token.text().get(start..)?.chars().next()
        })?;
        let last_index = textuals.len().saturating_sub(1);
        let last = textuals
            .iter()
            .enumerate()
            .rev()
            .find_map(|(index, textual)| {
                let token = textual.value_token().ok()?;
                let end = if trim_boundaries && index == last_index {
                    token.text().len() - Self::trailing_space_or_line_ending_len(token.text())
                } else {
                    token.text().len()
                };
                token.text().get(..end)?.chars().next_back()
            })?;

        Some((first, last))
    }

    /// Returns the byte length of one leading space or line ending.
    ///
    /// A CRLF sequence is one line ending and therefore occupies two bytes. An ASCII space, CR,
    /// or LF occupies one byte. Any other leading character returns zero.
    fn leading_space_or_line_ending_len(text: &str) -> usize {
        if text.starts_with("\r\n") {
            2
        } else if text.starts_with(Self::SPACE_OR_LINE_ENDING_CHARS) {
            1
        } else {
            0
        }
    }

    /// Returns the byte length of one trailing space or line ending.
    ///
    /// A CRLF sequence is one line ending and therefore occupies two bytes. An ASCII space, CR,
    /// or LF occupies one byte. Any other trailing character returns zero.
    fn trailing_space_or_line_ending_len(text: &str) -> usize {
        if text.ends_with("\r\n") {
            2
        } else if text.ends_with(Self::SPACE_OR_LINE_ENDING_CHARS) {
            1
        } else {
            0
        }
    }

    fn replace_line_endings_with_spaces(text: &str) -> Cow<'_, str> {
        if !text.contains(['\r', '\n']) {
            return Cow::Borrowed(text);
        }

        let mut normalized = String::with_capacity(text.len());
        let mut chars = text.chars().peekable();
        while let Some(char) = chars.next() {
            match char {
                '\r' => {
                    if chars.peek() == Some(&'\n') {
                        chars.next();
                    }
                    normalized.push(' ');
                }
                '\n' => normalized.push(' '),
                char => normalized.push(char),
            }
        }

        Cow::Owned(normalized)
    }
}

impl Format<MarkdownFormatContext> for CodeSpanNormalization {
    fn fmt(&self, f: &mut MarkdownFormatter) -> FormatResult<()> {
        if self.needs_padding {
            write!(f, [space()])?;
        }

        let last_index = self.textuals.len().saturating_sub(1);
        let prose_wrap = f.options().prose_wrap();

        for (index, textual) in self.textuals.iter().enumerate() {
            if prose_wrap == ProseWrap::Preserve
                && (!self.trim_boundaries || (index != 0 && index != last_index))
            {
                write!(f, [textual.format()])?;
                continue;
            }

            let value_token = textual.value_token()?;
            let token_text = value_token.text();
            let start = if self.trim_boundaries && index == 0 {
                Self::leading_space_or_line_ending_len(token_text)
            } else {
                0
            };
            let end = if self.trim_boundaries && index == last_index {
                token_text.len() - Self::trailing_space_or_line_ending_len(token_text)
            } else {
                token_text.len()
            };

            f.context()
                .comments()
                .mark_suppression_checked(textual.syntax());
            if start == end {
                format_removed(&value_token).fmt(f)?;
            } else {
                let content = &token_text[start..end];
                let content = if prose_wrap == ProseWrap::Preserve {
                    Cow::Borrowed(content)
                } else {
                    Self::replace_line_endings_with_spaces(content)
                };
                let replacement = syntax_token_cow_slice(
                    content,
                    &value_token,
                    value_token.text_range().start() + TextSize::from(start as u32),
                )
                .with_literal_line_breaks();
                format_replaced(&value_token, &replacement).fmt(f)?;
            }
        }

        if self.needs_padding {
            write!(f, [space()])?;
        }

        Ok(())
    }
}
