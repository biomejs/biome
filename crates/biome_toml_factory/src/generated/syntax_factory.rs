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
            TOML_BOGUS
            | TOML_BOGUS_KEY
            | TOML_BOGUS_KEY_FRAGMENT
            | TOML_BOGUS_MEMBER
            | TOML_BOGUS_TABLE_HEADER
            | TOML_BOGUS_VALUE => RawSyntaxNode::new(kind, children.into_iter().map(Some)),
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
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && AnyTomlArrayTableHeader::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && TomlMemberList::can_cast(element.kind())
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
            TOML_ARRAY_TABLE_HEADER => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!["[["]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnyTomlKey::can_cast(element.kind())
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
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_ARRAY_TABLE_HEADER.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_ARRAY_TABLE_HEADER, children)
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
            TOML_BASIC_STRING => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == TOML_BASIC_STRING_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_BASIC_STRING.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_BASIC_STRING, children)
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
            TOML_DATE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == TOML_DATE_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_DATE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_DATE, children)
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
                    && TomlInlineTableMemberList::can_cast(element.kind())
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
                    && TomlKeyFragmentList::can_cast(element.kind())
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
            TOML_LITERAL_STRING => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == TOML_LITERAL_STRING_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_LITERAL_STRING.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_LITERAL_STRING, children)
            }
            TOML_MEMBER => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && AnyTomlKey::can_cast(element.kind())
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
                        TOML_MEMBER.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_MEMBER, children)
            }
            TOML_MULTILINE_BASIC_STRING => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == TOML_ML_BASIC_STRING_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_MULTILINE_BASIC_STRING.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_MULTILINE_BASIC_STRING, children)
            }
            TOML_MULTILINE_LITERAL_STRING => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == TOML_ML_LITERAL_STRING_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_MULTILINE_LITERAL_STRING.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_MULTILINE_LITERAL_STRING, children)
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
            TOML_TABLE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && AnyTomlTableHeader::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && TomlMemberList::can_cast(element.kind())
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
            TOML_TABLE_HEADER => {
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
                    && AnyTomlKey::can_cast(element.kind())
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
                        TOML_TABLE_HEADER.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_TABLE_HEADER, children)
            }
            TOML_TIME => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == TOML_TIME_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        TOML_TIME.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(TOML_TIME, children)
            }
            TOML_ARRAY_ELEMENT_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                AnyTomlValue::can_cast,
                T ! [,],
                true,
            ),
            TOML_INLINE_TABLE_MEMBER_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                AnyTomlMember::can_cast,
                T ! [,],
                true,
            ),
            TOML_KEY_FRAGMENT_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                AnyTomlKeyFragment::can_cast,
                T ! [.],
                false,
            ),
            TOML_MEMBER_LIST => {
                Self::make_node_list_syntax(kind, children, AnyTomlMember::can_cast)
            }
            TOML_ROOT_ITEM_LIST => {
                Self::make_node_list_syntax(kind, children, AnyTomlRootItem::can_cast)
            }
            _ => unreachable!("Is {:?} a token?", kind),
        }
    }
}
