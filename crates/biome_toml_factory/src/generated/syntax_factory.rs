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
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!["[["]
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
                    && element.kind() == T!["]]"]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && TomlKeyValueList::can_cast(element.kind())
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
            TOML_BARE_KEY => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == TOML_BARE_KEY_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_BARE_KEY.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_BARE_KEY, children)
            }
            TOML_BOOLEAN => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![true] | T![false])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_BOOLEAN.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_BOOLEAN, children)
            }
            TOML_DATE_TIME => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && matches!(
                        element.kind(),
                        TOML_OFFSET_DATE_TIME_LITERAL
                            | TOML_LOCAL_DATE_TIME_LITERAL
                            | TOML_LOCAL_DATE_LITERAL
                            | TOML_LOCAL_TIME_LITERAL
                    )
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_DATE_TIME.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_DATE_TIME, children)
            }
            TOML_FLOAT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == TOML_FLOAT_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_FLOAT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_FLOAT, children)
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
                    && TomlInlineTableEntryList::can_cast(element.kind())
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
            TOML_INTEGER => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == TOML_INTEGER_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_INTEGER.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_INTEGER, children)
            }
            TOML_KEY => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && TomlKeyPartList::can_cast(element.kind())
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
            TOML_QUOTED_KEY => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && matches!(
                        element.kind(),
                        TOML_BASIC_STRING_LITERAL | TOML_LITERAL_STRING_LITERAL
                    )
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_QUOTED_KEY.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_QUOTED_KEY, children)
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
                    && TomlRootItemList::can_cast(element.kind())
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
            TOML_STRING => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && matches!(
                        element.kind(),
                        TOML_BASIC_STRING_LITERAL
                            | TOML_LITERAL_STRING_LITERAL
                            | TOML_ML_BASIC_STRING_LITERAL
                            | TOML_ML_LITERAL_STRING_LITERAL
                    )
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_STRING.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_STRING, children)
            }
            TOML_TABLE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
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
                if let Some(element) = &current_element
                    && TomlKeyValueList::can_cast(element.kind())
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
            TOML_INLINE_TABLE_ENTRY_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                AnyTomlKeyValue::can_cast,
                T ! [,],
                true,
            ),
            TOML_KEY_PART_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                AnyTomlKeyPart::can_cast,
                T ! [.],
                false,
            ),
            TOML_KEY_VALUE_LIST => {
                Self::make_node_list_syntax(kind, children, AnyTomlKeyValue::can_cast)
            }
            TOML_ROOT_ITEM_LIST => {
                Self::make_node_list_syntax(kind, children, AnyTomlRootItem::can_cast)
            }
            _ => unreachable!("Is {:?} a token?", kind),
        }
    }
}
