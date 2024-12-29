//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(unused_mut)]
use biome_markdown_syntax::{MarkdownSyntaxKind, MarkdownSyntaxKind::*, T, *};
use biome_rowan::{
    AstNode, ParsedChildren, RawNodeSlots, RawSyntaxNode, SyntaxFactory, SyntaxKind,
};
#[derive(Debug)]
pub struct MarkdownSyntaxFactory;
impl SyntaxFactory for MarkdownSyntaxFactory {
    type Kind = MarkdownSyntaxKind;
    fn make_syntax(
        kind: Self::Kind,
        children: ParsedChildren<Self::Kind>,
    ) -> RawSyntaxNode<Self::Kind> {
        match kind {
            MD_BOGUS => RawSyntaxNode::new(kind, children.into_iter().map(Some)),
            MD_BULLET_LIST_ITEM => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MdBulletList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MD_BULLET_LIST_ITEM.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MD_BULLET_LIST_ITEM, children)
            }
            MD_DOCUMENT => {
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
                    if MdBlockList::can_cast(element.kind()) {
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
                        MD_DOCUMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MD_DOCUMENT, children)
            }
            MD_FENCED_CODE_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MdTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MD_FENCED_CODE_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MD_FENCED_CODE_BLOCK, children)
            }
            MD_HARD_LINE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == MD_HARD_LINE_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MD_HARD_LINE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MD_HARD_LINE, children)
            }
            MD_HASH => {
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
                    return RawSyntaxNode::new(MD_HASH.to_bogus(), children.into_iter().map(Some));
                }
                slots.into_node(MD_HASH, children)
            }
            MD_HEADER => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MdHashList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if MdParagraph::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if MdHashList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MD_HEADER.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MD_HEADER, children)
            }
            MD_HTML_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MdTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MD_HTML_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MD_HTML_BLOCK, children)
            }
            MD_INDENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == MD_INDENT_CHUNK_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MD_INDENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MD_INDENT, children)
            }
            MD_INDENT_CODE_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MdTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MD_INDENT_CODE_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MD_INDENT_CODE_BLOCK, children)
            }
            MD_INLINE_CODE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MdTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MD_INLINE_CODE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MD_INLINE_CODE, children)
            }
            MD_INLINE_EMPHASIS => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MdTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MD_INLINE_EMPHASIS.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MD_INLINE_EMPHASIS, children)
            }
            MD_INLINE_IMAGE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MdTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if MdTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if MdTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MD_INLINE_IMAGE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MD_INLINE_IMAGE, children)
            }
            MD_INLINE_LINK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MdTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if MdTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if MdTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MD_INLINE_LINK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MD_INLINE_LINK, children)
            }
            MD_LINK_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MdTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if MdTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if MdTextual::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MD_LINK_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MD_LINK_BLOCK, children)
            }
            MD_ORDER_LIST_ITEM => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MdBulletList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MD_ORDER_LIST_ITEM.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MD_ORDER_LIST_ITEM, children)
            }
            MD_PARAGRAPH => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MdParagraphItemList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MD_PARAGRAPH.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MD_PARAGRAPH, children)
            }
            MD_QUOTE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if AnyMdBlock::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(MD_QUOTE.to_bogus(), children.into_iter().map(Some));
                }
                slots.into_node(MD_QUOTE, children)
            }
            MD_SETEXT_HEADER => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MdParagraph::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MD_SETEXT_HEADER.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MD_SETEXT_HEADER, children)
            }
            MD_SOFT_BREAK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == MD_SOFT_BREAK_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MD_SOFT_BREAK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MD_SOFT_BREAK, children)
            }
            MD_TEXTUAL => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == MD_TEXTUAL_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MD_TEXTUAL.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MD_TEXTUAL, children)
            }
            MD_THEMATIC_BREAK_BLOCK => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == MD_THEMATIC_BREAK_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MD_THEMATIC_BREAK_BLOCK.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MD_THEMATIC_BREAK_BLOCK, children)
            }
            MD_BLOCK_LIST => Self::make_node_list_syntax(kind, children, AnyMdBlock::can_cast),
            MD_BULLET_LIST => Self::make_node_list_syntax(kind, children, AnyCodeBlock::can_cast),
            MD_HASH_LIST => Self::make_node_list_syntax(kind, children, MdHash::can_cast),
            MD_ORDER_LIST => Self::make_node_list_syntax(kind, children, AnyCodeBlock::can_cast),
            MD_PARAGRAPH_ITEM_LIST => {
                Self::make_node_list_syntax(kind, children, AnyMdInline::can_cast)
            }
            _ => unreachable!("Is {:?} a token?", kind),
        }
    }
}
