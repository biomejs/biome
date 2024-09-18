//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_markdown_syntax::{MarkdownSyntaxKind, MarkdownSyntaxKind::*, T, *};
use biome_rowan::{
    AstNode, ParsedChildren, RawNodeSlots, RawSyntaxNode, SyntaxFactory, SyntaxKind,
};
#[derive(Debug)]
pub struct MarkdownSyntaxFactory;
impl SyntaxFactory for MarkdownSyntaxFactory {
    type Kind = MarkdownSyntaxKind;
    #[allow(unused_mut)]
    fn make_syntax(
        kind: Self::Kind,
        children: ParsedChildren<Self::Kind>,
    ) -> RawSyntaxNode<Self::Kind> {
        match kind {
            MARKDOWN_BOGUS => RawSyntaxNode::new(kind, children.into_iter().map(Some)),
            MARKDOWN_BREAK_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == MARKDOWN_BREAK_BLOCK_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_BREAK_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_BREAK_BLOCK, children)
            }
            MARKDOWN_BULLET_LIST_ITEM => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownBulletList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_BULLET_LIST_ITEM.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_BULLET_LIST_ITEM, children)
            }
            MARKDOWN_DOCUMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T![UNICODE_BOM] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if MarkdownBlockList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T![EOF] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_DOCUMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_DOCUMENT, children)
            }
            MARKDOWN_FENCED_CODE_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_FENCED_CODE_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_FENCED_CODE_BLOCK, children)
            }
            MARKDOWN_HARD_LINE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == MARKDOWN_HARD_LINE_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_HARD_LINE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_HARD_LINE, children)
            }
            MARKDOWN_HASH => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T ! [#] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_HASH.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_HASH, children)
            }
            MARKDOWN_HEADER => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownHashList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if MarkdownParagraph::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if MarkdownHashList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_HEADER.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_HEADER, children)
            }
            MARKDOWN_HTML_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_HTML_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_HTML_BLOCK, children)
            }
            MARKDOWN_INDENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == MARKDOWN_INDENT_CHUNK_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_INDENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_INDENT, children)
            }
            MARKDOWN_INDENT_CODE_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_INDENT_CODE_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_INDENT_CODE_BLOCK, children)
            }
            MARKDOWN_INLINE_CODE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_INLINE_CODE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_INLINE_CODE, children)
            }
            MARKDOWN_INLINE_EMPHASIS => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_INLINE_EMPHASIS.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_INLINE_EMPHASIS, children)
            }
            MARKDOWN_INLINE_IMAGE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if MarkdownTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if MarkdownTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_INLINE_IMAGE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_INLINE_IMAGE, children)
            }
            MARKDOWN_INLINE_LINK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if MarkdownTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if MarkdownTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_INLINE_LINK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_INLINE_LINK, children)
            }
            MARKDOWN_LINK_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if MarkdownTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if MarkdownTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_LINK_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_LINK_BLOCK, children)
            }
            MARKDOWN_ORDER_LIST_ITEM => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownBulletList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_ORDER_LIST_ITEM.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_ORDER_LIST_ITEM, children)
            }
            MARKDOWN_PARAGRAPH => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownParagraphItemList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_PARAGRAPH.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_PARAGRAPH, children)
            }
            MARKDOWN_QUOTE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if AnyMarkdownBlock::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_QUOTE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_QUOTE, children)
            }
            MARKDOWN_SETEXT_HEADER => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MarkdownParagraph::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_SETEXT_HEADER.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_SETEXT_HEADER, children)
            }
            MARKDOWN_SOFT_BREAK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == MARKDOWN_SOFT_BREAK_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_SOFT_BREAK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_SOFT_BREAK, children)
            }
            MARKDOWN_TEXTUAL => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == MARKDOWN_TEXTUAL_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MARKDOWN_TEXTUAL.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MARKDOWN_TEXTUAL, children)
            }
            MARKDOWN_BLOCK_LIST => {
                Self::make_node_list_syntax(kind, children, AnyMarkdownBlock::can_cast)
            }
            MARKDOWN_BULLET_LIST => {
                Self::make_node_list_syntax(kind, children, AnyCodeBlock::can_cast)
            }
            MARKDOWN_HASH_LIST => {
                Self::make_node_list_syntax(kind, children, MarkdownHash::can_cast)
            }
            MARKDOWN_ORDER_LIST => {
                Self::make_node_list_syntax(kind, children, AnyCodeBlock::can_cast)
            }
            MARKDOWN_PARAGRAPH_ITEM_LIST => {
                Self::make_node_list_syntax(kind, children, AnyMarkdownInline::can_cast)
            }
            _ => unreachable!("Is {:?} a token?", kind),
        }
    }
}
