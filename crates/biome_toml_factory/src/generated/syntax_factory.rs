//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(unused_mut)]
use biome_rowan::{
    AstNode, ParsedChildren, RawNodeSlots, RawSyntaxNode, SyntaxFactory, SyntaxKind,
};
use biome_toml_syntax::{T, TomlSyntaxKind, TomlSyntaxKind::*, *};
#[derive(Debug)]
pub struct TomlSyntaxFactory;
impl SyntaxFactory for TomlSyntaxFactory {
    type Kind = TomlSyntaxKind;
    fn make_syntax(
        kind: Self::Kind,
        children: ParsedChildren<Self::Kind>,
    ) -> RawSyntaxNode<Self::Kind> {
        match kind {
            TOML_BOGUS | TOML_BOGUS_VALUE => {
                RawSyntaxNode::new(kind, children.into_iter().map(Some))
            }
            TOML_ARRAY => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!['[']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && TomlArrayElementList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![']']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_ARRAY.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_ARRAY, children)
            }
            TOML_ARRAY_TABLE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!['[']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['[']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && TomlKey::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![']']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![']']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_ARRAY_TABLE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_ARRAY_TABLE, children)
            }
            TOML_BOOLEAN_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == TOML_BOOLEAN
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_BOOLEAN_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_BOOLEAN_VALUE, children)
            }
            TOML_FLOAT_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == TOML_FLOAT
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_FLOAT_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_FLOAT_VALUE, children)
            }
            TOML_INLINE_TABLE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!['{']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && TomlInlineTableElementList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['}']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_INLINE_TABLE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_INLINE_TABLE, children)
            }
            TOML_INTEGER_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == TOML_INTEGER
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_INTEGER_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_INTEGER_VALUE, children)
            }
            TOML_KEY => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && TomlKeySegmentList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(TOML_KEY.to_bogus(), children.into_iter().map(Some));
                }
                slots.into_node(TOML_KEY, children)
            }
            TOML_KEY_SEGMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && matches!(
                        element.kind(),
                        TOML_BARE_KEY | TOML_BASIC_STRING | TOML_LITERAL_STRING
                    )
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_KEY_SEGMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_KEY_SEGMENT, children)
            }
            TOML_KEY_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && TomlKey::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [=]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnyTomlValue::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_KEY_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_KEY_VALUE, children)
            }
            TOML_LOCAL_DATE_TIME_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == TOML_LOCAL_DATE_TIME
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_LOCAL_DATE_TIME_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_LOCAL_DATE_TIME_VALUE, children)
            }
            TOML_LOCAL_DATE_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == TOML_LOCAL_DATE
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_LOCAL_DATE_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_LOCAL_DATE_VALUE, children)
            }
            TOML_LOCAL_TIME_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == TOML_LOCAL_TIME
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_LOCAL_TIME_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_LOCAL_TIME_VALUE, children)
            }
            TOML_OFFSET_DATE_TIME_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == TOML_OFFSET_DATE_TIME
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_OFFSET_DATE_TIME_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_OFFSET_DATE_TIME_VALUE, children)
            }
            TOML_ROOT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![UNICODE_BOM]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && TomlItemList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![EOF]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_ROOT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_ROOT, children)
            }
            TOML_STRING_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && matches!(element.kind(), TOML_BASIC_STRING | TOML_LITERAL_STRING)
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_STRING_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_STRING_VALUE, children)
            }
            TOML_TABLE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!['[']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && TomlKey::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![']']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_TABLE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_TABLE, children)
            }
            TOML_ARRAY_ELEMENT_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                AnyTomlValue::can_cast,
                T ! [,],
                true,
            ),
            TOML_INLINE_TABLE_ELEMENT_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                AnyTomlInlineTableElement::can_cast,
                T ! [,],
                true,
            ),
            TOML_ITEM_LIST => Self::make_node_list_syntax(kind, children, AnyTomlItem::can_cast),
            TOML_KEY_SEGMENT_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                TomlKeySegment::can_cast,
                T ! [.],
                false,
            ),
            _ => unreachable!("Is {:?} a token?", kind),
        }
    }
}
