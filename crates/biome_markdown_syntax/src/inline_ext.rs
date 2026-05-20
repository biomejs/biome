use crate::{AnyMdInline, MdInlineItemList};
use biome_rowan::AstNodeList;

impl AnyMdInline {
    /// `true` if this node is a line break: a hard break, soft break,
    /// or a textual token whose content is a newline character.
    fn is_line_break(&self) -> bool {
        match self {
            AnyMdInline::MdHardLine(_) | AnyMdInline::MdSoftBreak(_) => true,
            AnyMdInline::MdTextual(node) => node.is_newline().unwrap_or(false),
            _ => false,
        }
    }

    /// `true` if this node carries visible content (text, inline markup, etc.).
    /// Structural-only nodes (breaks, indent tokens, whitespace-only textuals)
    /// return `false`.
    fn has_content(&self) -> bool {
        match self {
            AnyMdInline::MdTextual(node) => {
                !node.is_newline().unwrap_or(false)
                    && !node.is_empty_and_not_newline().unwrap_or(false)
            }
            AnyMdInline::MdHardLine(_)
            | AnyMdInline::MdSoftBreak(_)
            | AnyMdInline::MdIndentToken(_)
            | AnyMdInline::MdQuotePrefix(_) => false,
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
            AnyMdInline::MdAutolink(node) => node.value().will_break(),
            AnyMdInline::MdInlineEmphasis(node) => node.content().will_break(),
            AnyMdInline::MdInlineItalic(node) => node.content().will_break(),
            AnyMdInline::MdInlineCode(node) => node.content().will_break(),
            AnyMdInline::MdInlineLink(node) => node.text().will_break(),
            AnyMdInline::MdInlineImage(node) => node.alt().will_break(),
            AnyMdInline::MdReferenceLink(node) => node.text().will_break(),
            AnyMdInline::MdReferenceImage(node) => node.alt().will_break(),
            _ => false,
        }
    }
}

impl MdInlineItemList {
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
