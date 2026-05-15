use crate::{AnyMdInline, MdInlineItemList};
use biome_rowan::AstNodeList;

impl AnyMdInline {
    /// `true` if the inline will break the line. Either as a textual break or a soft break.
    pub fn will_break(&self) -> bool {
        match self {
            AnyMdInline::MdHardLine(_) | AnyMdInline::MdSoftBreak(_) => true,
            AnyMdInline::MdTextual(node) => node.is_newline().unwrap_or(false),
            AnyMdInline::MdAutolink(node) => node.value().will_break(),
            AnyMdInline::MdInlineEmphasis(node) => node.content().will_break(),
            AnyMdInline::MdInlineItalic(node) => node.content().will_break(),
            AnyMdInline::MdInlineCode(node) => node.content().will_break(),
            AnyMdInline::MdInlineLink(node) => node.text().will_break(),
            AnyMdInline::MdInlineImage(node) => node.alt().will_break(),
            AnyMdInline::MdReferenceLink(node) => node.text().will_break(),
            AnyMdInline::MdReferenceImage(node) => node.alt().will_break(),
            // Leaf nodes that never contain line breaks: entity references
            // are single tokens (`&amp;`), inline HTML is opaque markup,
            // indent tokens are structural whitespace, and quote prefixes
            // (`>`) are block-level decoration.
            AnyMdInline::MdEntityReference(_)
            | AnyMdInline::MdHtmlBlock(_)
            | AnyMdInline::MdIndentToken(_)
            | AnyMdInline::MdInlineHtml(_)
            | AnyMdInline::MdQuotePrefix(_) => false,
        }
    }
}

impl MdInlineItemList {
    pub fn will_break(&self) -> bool {
        self.iter().any(|inline| inline.will_break())
    }
}
