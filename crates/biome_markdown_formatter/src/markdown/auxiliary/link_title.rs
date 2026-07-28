use crate::markdown::auxiliary::textual::FormatMdTextualOptions;
use crate::markdown::lists::inline_item_list::FormatMdFormatInlineItemListOptions;
use crate::prelude::*;
use crate::shared::{TextContext, TextPrintMode};
use biome_formatter::write;
use biome_markdown_syntax::{AnyMdInline, MdLinkTitle, MdLinkTitleFields, MdTextual};
use biome_rowan::AstNode;
use std::borrow::Cow;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdLinkTitle;
impl FormatNodeRule<MdLinkTitle> for FormatMdLinkTitle {
    fn fmt_fields(&self, node: &MdLinkTitle, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdLinkTitleFields { content } = node.as_fields();

        let Some(normalization) = LinkTitleNormalization::from_node(node) else {
            return write!(
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
            );
        };

        if normalization.is_empty() {
            write!(f, [normalization])
        } else {
            write!(f, [space(), normalization])
        }
    }
}

/// Rewrites a link title to the delimiter form requiring the fewest escapes without changing its
/// CommonMark title string.
///
/// CommonMark removes one layer of backslash escaping from ASCII punctuation in title content. The
/// normalization uses those unescaped characters to select double quotes, single quotes, or
/// parentheses based on how many characters that delimiter form must escape. Ties prefer double
/// quotes, followed by single quotes and parentheses. Literal backslashes and the selected
/// delimiters are escaped when the title is serialized. Escapes of `&`, `#`, and `;` are retained
/// because removing them can form an entity or numeric character reference. Non-delimiter escapes
/// are also retained in multiline titles because unescaping a marker at the start of a physical
/// line can turn title content into block syntax when the formatted document is parsed again.
///
/// See <https://spec.commonmark.org/0.30/#link-title>.
struct LinkTitleNormalization {
    /// The textual nodes replaced by the normalized title, in source order.
    textuals: Vec<MdTextual>,
    /// The opening delimiter, encoded title content, and closing delimiter.
    normalized: String,
    /// Whether the interpreted title content is empty and the title should be omitted.
    is_empty: bool,
}

impl LinkTitleNormalization {
    fn from_node(node: &MdLinkTitle) -> Option<Self> {
        let textuals = node
            .content()
            .iter()
            .map(|item| match item {
                AnyMdInline::MdTextual(textual) => Some(textual),
                _ => None,
            })
            .collect::<Option<Vec<_>>>()?;
        let raw = node.syntax().text_trimmed().to_string();
        let raw = raw.trim();
        let inner = if raw.starts_with('"') && raw.ends_with('"')
            || raw.starts_with('\'') && raw.ends_with('\'')
            || raw.starts_with('(') && raw.ends_with(')')
        {
            raw.get(1..raw.len().checked_sub(1)?)?
        } else {
            return None;
        };
        let is_multiline = inner.contains(['\r', '\n']);

        let mut content = Vec::with_capacity(inner.len());
        let mut chars = inner.chars().peekable();
        while let Some(char) = chars.next() {
            if char == '\r' {
                if chars.peek() == Some(&'\n') {
                    chars.next();
                }
                content.push(('\n', false));
                continue;
            }
            if char == '\\'
                && let Some(next) = chars.peek().copied()
                && next.is_ascii_punctuation()
            {
                let preserve_escape = matches!(next, '&' | '#' | ';')
                    || is_multiline && !matches!(next, '"' | '\'' | '(' | ')' | '\\');
                content.push((next, preserve_escape));
                chars.next();
            } else {
                content.push((char, false));
            }
        }

        let double_quote_escapes = content.iter().filter(|(char, _)| *char == '"').count();
        let single_quote_escapes = content.iter().filter(|(char, _)| *char == '\'').count();
        let parenthesis_escapes = content
            .iter()
            .filter(|(char, _)| matches!(char, '(' | ')'))
            .count();
        let (opening, closing) = if double_quote_escapes <= single_quote_escapes
            && double_quote_escapes <= parenthesis_escapes
        {
            ('"', '"')
        } else if single_quote_escapes <= parenthesis_escapes {
            ('\'', '\'')
        } else {
            ('(', ')')
        };

        let mut normalized = String::with_capacity(inner.len() + 2);
        normalized.push(opening);
        for &(char, preserve_escape) in &content {
            if preserve_escape {
                normalized.push('\\');
                normalized.push(char);
                continue;
            }
            if char == '\\' || char == closing || opening == '(' && char == '(' {
                normalized.push('\\');
            }
            normalized.push(char);
        }
        normalized.push(closing);

        Some(Self {
            textuals,
            normalized,
            is_empty: content.is_empty(),
        })
    }

    fn is_empty(&self) -> bool {
        self.is_empty
    }
}

impl Format<MarkdownFormatContext> for LinkTitleNormalization {
    fn fmt(&self, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let Some(first) = self.textuals.first() else {
            return Ok(());
        };

        let remaining = if self.is_empty {
            self.textuals.as_slice()
        } else {
            f.context().comments().is_suppressed(first.syntax());
            let first_token = first.value_token()?;
            let replacement = syntax_token_cow_slice(
                Cow::Owned(self.normalized.clone()),
                &first_token,
                first_token.text_trimmed_range().start(),
            )
            .with_literal_line_breaks();
            format_replaced(&first_token, &replacement).fmt(f)?;
            &self.textuals[1..]
        };

        for textual in remaining {
            textual
                .format()
                .with_options(FormatMdTextualOptions {
                    print_mode: TextPrintMode::Remove,
                    ..FormatMdTextualOptions::default()
                })
                .fmt(f)?;
        }

        Ok(())
    }
}
