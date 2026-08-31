//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
use biome_rowan::AstNode;
use biome_toml_syntax::{
    TomlSyntaxElement as SyntaxElement, TomlSyntaxNode as SyntaxNode,
    TomlSyntaxToken as SyntaxToken, *,
};
pub fn toml_array(
    l_brack_token: SyntaxToken,
    elements: TomlArrayElementList,
    r_brack_token: SyntaxToken,
) -> TomlArray {
    TomlArray::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_ARRAY,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(elements.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn toml_array_table(
    opening_outer_token: SyntaxToken,
    opening_inner_token: SyntaxToken,
    name: TomlKey,
    closing_inner_token: SyntaxToken,
    closing_outer_token: SyntaxToken,
) -> TomlArrayTable {
    TomlArrayTable::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_ARRAY_TABLE,
        [
            Some(SyntaxElement::Token(opening_outer_token)),
            Some(SyntaxElement::Token(opening_inner_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(closing_inner_token)),
            Some(SyntaxElement::Token(closing_outer_token)),
        ],
    ))
}
pub fn toml_boolean_value(value_token: SyntaxToken) -> TomlBooleanValue {
    TomlBooleanValue::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_BOOLEAN_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn toml_float_value(value_token: SyntaxToken) -> TomlFloatValue {
    TomlFloatValue::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_FLOAT_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn toml_inline_table(
    l_curly_token: SyntaxToken,
    elements: TomlInlineTableElementList,
    r_curly_token: SyntaxToken,
) -> TomlInlineTable {
    TomlInlineTable::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_INLINE_TABLE,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(elements.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn toml_integer_value(value_token: SyntaxToken) -> TomlIntegerValue {
    TomlIntegerValue::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_INTEGER_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn toml_key(segments: TomlKeySegmentList) -> TomlKey {
    TomlKey::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_KEY,
        [Some(SyntaxElement::Node(segments.into_syntax()))],
    ))
}
pub fn toml_key_segment(value_token: SyntaxToken) -> TomlKeySegment {
    TomlKeySegment::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_KEY_SEGMENT,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn toml_key_value(key: TomlKey, eq_token: SyntaxToken, value: AnyTomlValue) -> TomlKeyValue {
    TomlKeyValue::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_KEY_VALUE,
        [
            Some(SyntaxElement::Node(key.into_syntax())),
            Some(SyntaxElement::Token(eq_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn toml_local_date_time_value(value_token: SyntaxToken) -> TomlLocalDateTimeValue {
    TomlLocalDateTimeValue::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_LOCAL_DATE_TIME_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn toml_local_date_value(value_token: SyntaxToken) -> TomlLocalDateValue {
    TomlLocalDateValue::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_LOCAL_DATE_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn toml_local_time_value(value_token: SyntaxToken) -> TomlLocalTimeValue {
    TomlLocalTimeValue::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_LOCAL_TIME_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn toml_offset_date_time_value(value_token: SyntaxToken) -> TomlOffsetDateTimeValue {
    TomlOffsetDateTimeValue::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_OFFSET_DATE_TIME_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn toml_root(items: TomlItemList, eof_token: SyntaxToken) -> TomlRootBuilder {
    TomlRootBuilder {
        items,
        eof_token,
        bom_token: None,
    }
}
pub struct TomlRootBuilder {
    items: TomlItemList,
    eof_token: SyntaxToken,
    bom_token: Option<SyntaxToken>,
}
impl TomlRootBuilder {
    pub fn with_bom_token(mut self, bom_token: SyntaxToken) -> Self {
        self.bom_token = Some(bom_token);
        self
    }
    pub fn build(self) -> TomlRoot {
        TomlRoot::unwrap_cast(SyntaxNode::new_detached(
            TomlSyntaxKind::TOML_ROOT,
            [
                self.bom_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.items.into_syntax())),
                Some(SyntaxElement::Token(self.eof_token)),
            ],
        ))
    }
}
pub fn toml_string_value(value_token: SyntaxToken) -> TomlStringValue {
    TomlStringValue::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_STRING_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn toml_table(
    l_brack_token: SyntaxToken,
    name: TomlKey,
    r_brack_token: SyntaxToken,
) -> TomlTable {
    TomlTable::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_TABLE,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn toml_array_element_list<I, S>(items: I, separators: S) -> TomlArrayElementList
where
    I: IntoIterator<Item = AnyTomlValue>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = TomlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    TomlArrayElementList::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_ARRAY_ELEMENT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn toml_inline_table_element_list<I, S>(items: I, separators: S) -> TomlInlineTableElementList
where
    I: IntoIterator<Item = AnyTomlInlineTableElement>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = TomlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    TomlInlineTableElementList::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_INLINE_TABLE_ELEMENT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn toml_item_list<I>(items: I) -> TomlItemList
where
    I: IntoIterator<Item = AnyTomlItem>,
    I::IntoIter: ExactSizeIterator,
{
    TomlItemList::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_ITEM_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn toml_key_segment_list<I, S>(items: I, separators: S) -> TomlKeySegmentList
where
    I: IntoIterator<Item = TomlKeySegment>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = TomlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    TomlKeySegmentList::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_KEY_SEGMENT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn toml_bogus<I>(slots: I) -> TomlBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    TomlBogus::unwrap_cast(SyntaxNode::new_detached(TomlSyntaxKind::TOML_BOGUS, slots))
}
pub fn toml_bogus_value<I>(slots: I) -> TomlBogusValue
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    TomlBogusValue::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_BOGUS_VALUE,
        slots,
    ))
}
