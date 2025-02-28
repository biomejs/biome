//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(unused_mut)]
use biome_rowan::{
    AstNode, ParsedChildren, RawNodeSlots, RawSyntaxNode, SyntaxFactory, SyntaxKind,
};
use biome_yaml_syntax::{YamlSyntaxKind, YamlSyntaxKind::*, T, *};
#[derive(Debug)]
pub struct YamlSyntaxFactory;
impl SyntaxFactory for YamlSyntaxFactory {
    type Kind = YamlSyntaxKind;
    fn make_syntax(
        kind: Self::Kind,
        children: ParsedChildren<Self::Kind>,
    ) -> RawSyntaxNode<Self::Kind> {
        match kind {
            YAML_BOGUS | YAML_BOGUS_VALUE => {
                RawSyntaxNode::new(kind, children.into_iter().map(Some))
            }
            YAML_ARRAY => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if YamlArrayItemList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_ARRAY.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_ARRAY, children)
            }
            YAML_ARRAY_INLINE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T!['['] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if YamlArrayInlineList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T![']'] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_ARRAY_INLINE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_ARRAY_INLINE, children)
            }
            YAML_ARRAY_ITEM => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T ! [-] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if AnyYamlValue::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_ARRAY_ITEM.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_ARRAY_ITEM, children)
            }
            YAML_BLOCK_FOLDED => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T ! [>] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if YamlBlockValue::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_BLOCK_FOLDED.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_BLOCK_FOLDED, children)
            }
            YAML_BLOCK_LITERAL => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T ! [|] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if YamlBlockValue::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_BLOCK_LITERAL.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_BLOCK_LITERAL, children)
            }
            YAML_BLOCK_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == YAML_BLOCK_VALUE {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_BLOCK_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_BLOCK_VALUE, children)
            }
            YAML_BOOLEAN_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == YAML_BOOLEAN_VALUE {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_BOOLEAN_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_BOOLEAN_VALUE, children)
            }
            YAML_DOCUMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T ! [---] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if AnyYamlValue::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T ! [...] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_DOCUMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_DOCUMENT, children)
            }
            YAML_IDENTIFIER => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == YAML_IDENTIFIER {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_IDENTIFIER.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_IDENTIFIER, children)
            }
            YAML_NULL_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == YAML_NULL_VALUE {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_NULL_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_NULL_VALUE, children)
            }
            YAML_NUMBER_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == YAML_NUMBER_VALUE {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_NUMBER_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_NUMBER_VALUE, children)
            }
            YAML_OBJECT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if YamlObjectMemberList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_OBJECT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_OBJECT, children)
            }
            YAML_OBJECT_MEMBER => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if YamlIdentifier::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T ! [:] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if AnyYamlValue::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_OBJECT_MEMBER.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_OBJECT_MEMBER, children)
            }
            YAML_ROOT => {
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
                    if YamlDocumentList::can_cast(element.kind()) {
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
                        YAML_ROOT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_ROOT, children)
            }
            YAML_STRING_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == YAML_STRING_VALUE {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        YAML_STRING_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(YAML_STRING_VALUE, children)
            }
            YAML_ARRAY_INLINE_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                AnyYamlScalar::can_cast,
                T ! [,],
                true,
            ),
            YAML_ARRAY_ITEM_LIST => {
                Self::make_node_list_syntax(kind, children, YamlArrayItem::can_cast)
            }
            YAML_DOCUMENT_LIST => {
                Self::make_node_list_syntax(kind, children, YamlDocument::can_cast)
            }
            YAML_OBJECT_MEMBER_LIST => {
                Self::make_node_list_syntax(kind, children, YamlObjectMember::can_cast)
            }
            _ => unreachable!("Is {:?} a token?", kind),
        }
    }
}
