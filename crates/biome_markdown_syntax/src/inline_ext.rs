use crate::list_ext::AnyListItem;
use crate::{AnyMdInline, MdInlineCode, MdInlineItemList};
use biome_rowan::{AstNode, AstNodeList, Direction};

impl AnyMdInline {
    /// `true` if this node is a line break: a hard break, soft break,
    /// or a textual token whose content is a newline character.
    fn is_line_break(&self) -> bool {
        match self {
            Self::MdHardLine(_) => true,
            Self::MdTextual(node) => node.is_newline().unwrap_or(false),
            _ => false,
        }
    }

    /// `true` if this node carries visible content (text, inline markup, etc.).
    /// Structural-only nodes (breaks, indent tokens, whitespace-only textuals)
    /// return `false`.
    fn has_content(&self) -> bool {
        match self {
            Self::MdTextual(node) => {
                !node.is_newline().unwrap_or(false)
                    && !node.is_empty_and_not_newline().unwrap_or(false)
            }
            Self::MdHardLine(_) | Self::MdIndentToken(_) | Self::MdQuotePrefix(_) => false,
            _ => true,
        }
    }

    /// `true` if this node's inner content spans multiple lines.
    ///
    /// Only meaningful for container nodes (emphasis, links, etc.) that
    /// hold an `MdInlineItemList`. Leaf nodes always return `false` —
    /// the list-level check in [`MdInlineItemList::will_break`] handles
    /// break classification at the flat level.
    pub fn will_break(&self) -> bool {
        match self {
            Self::MdAutolink(node) => node.value().will_break(),
            Self::MdInlineEmphasis(node) => node.content().will_break(),
            Self::MdInlineItalic(node) => node.content().will_break(),
            Self::MdInlineCode(node) => node.content().will_break(),
            Self::MdInlineLink(node) => node.text().will_break(),
            Self::MdInlineImage(node) => node.alt().will_break(),
            Self::MdReferenceLink(node) => node.text().will_break(),
            Self::MdReferenceImage(node) => node.alt().will_break(),
            _ => false,
        }
    }
}

impl AnyListItem {
    pub const fn is_ordered(&self) -> bool {
        matches!(self, Self::MdOrderedListItem(_))
    }
}

impl MdInlineCode {
    /// Returns code-span content after applying CommonMark whitespace
    /// normalization.
    ///
    /// The returned text excludes the backtick delimiters. Line endings become
    /// spaces, and one surrounding space is removed when the span is not made
    /// entirely of spaces.
    pub fn normalized_text(&self) -> String {
        crate::text_ext::normalize_code_span(&self.content().literal_text())
    }
}

impl MdInlineItemList {
    /// Whether the last item is a line ending (`\n`, `\r\n` or `\r`).
    pub fn ends_with_newline(&self) -> bool {
        self.last().is_some_and(|item| {
            item.as_md_textual()
                .is_some_and(|textual| textual.is_newline().unwrap_or_default())
        })
    }

    /// Reconstructs literal text represented by this inline list.
    ///
    /// Document-level fenced blocks may store their entire body in one
    /// `MdCodeContent`, while nested blocks use textual, indentation, and quote
    /// prefix nodes. Hard line nodes contribute only their line ending. Other
    /// nodes contribute their token text without interpreting escapes or
    /// entities.
    pub fn literal_text(&self) -> String {
        let mut text = String::new();
        for item in self.iter() {
            match item {
                AnyMdInline::MdTextual(textual) => {
                    if let Ok(token) = textual.value_token() {
                        text.push_str(token.text());
                    }
                }
                AnyMdInline::MdHardLine(_) => text.push('\n'),
                AnyMdInline::MdCodeContent(content) => {
                    if let Ok(token) = content.value_token() {
                        text.push_str(token.text());
                    }
                }
                _ => {
                    for token in item
                        .syntax()
                        .descendants_with_tokens(Direction::Next)
                        .filter_map(|element| element.into_token())
                    {
                        text.push_str(token.text());
                    }
                }
            }
        }
        text
    }

    /// `true` if the content spans multiple lines — i.e. there is a line
    /// break with visible content after it. A trailing newline at the end
    /// of the list (with no content following) does not count.
    pub fn will_break(&self) -> bool {
        let mut seen_break = false;
        for item in self.iter() {
            if item.is_line_break() {
                seen_break = true;
            } else if seen_break && item.has_content() {
                return true;
            }
            if item.will_break() {
                return true;
            }
        }
        false
    }
}
