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
    l_double_brack_token: SyntaxToken,
    key: TomlKey,
    r_double_brack_token: SyntaxToken,
    entries: TomlKeyValueList,
) -> TomlArrayTable {
    TomlArrayTable::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_ARRAY_TABLE,
        [
            Some(SyntaxElement::Token(l_double_brack_token)),
            Some(SyntaxElement::Node(key.into_syntax())),
            Some(SyntaxElement::Token(r_double_brack_token)),
            Some(SyntaxElement::Node(entries.into_syntax())),
        ],
    ))
}
pub fn toml_bare_key(value_token: SyntaxToken) -> TomlBareKey {
    TomlBareKey::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_BARE_KEY,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn toml_boolean(value_token_token: SyntaxToken) -> TomlBoolean {
    TomlBoolean::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_BOOLEAN,
        [Some(SyntaxElement::Token(value_token_token))],
    ))
}
pub fn toml_date_time(value_token_token: SyntaxToken) -> TomlDateTime {
    TomlDateTime::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_DATE_TIME,
        [Some(SyntaxElement::Token(value_token_token))],
    ))
}
pub fn toml_float(value_token: SyntaxToken) -> TomlFloat {
    TomlFloat::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_FLOAT,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn toml_inline_table(
    l_curly_token: SyntaxToken,
    entries: TomlInlineTableEntryList,
    r_curly_token: SyntaxToken,
) -> TomlInlineTable {
    TomlInlineTable::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_INLINE_TABLE,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(entries.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn toml_integer(value_token: SyntaxToken) -> TomlInteger {
    TomlInteger::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_INTEGER,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn toml_key(parts: TomlKeyPartList) -> TomlKey {
    TomlKey::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_KEY,
        [Some(SyntaxElement::Node(parts.into_syntax()))],
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
pub fn toml_quoted_key(value_token_token: SyntaxToken) -> TomlQuotedKey {
    TomlQuotedKey::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_QUOTED_KEY,
        [Some(SyntaxElement::Token(value_token_token))],
    ))
}
pub fn toml_root(items: TomlRootItemList, eof_token: SyntaxToken) -> TomlRootBuilder {
    TomlRootBuilder {
        items,
        eof_token,
        bom_token: None,
    }
}
pub struct TomlRootBuilder {
    items: TomlRootItemList,
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
pub fn toml_string(value_token_token: SyntaxToken) -> TomlString {
    TomlString::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_STRING,
        [Some(SyntaxElement::Token(value_token_token))],
    ))
}
pub fn toml_table(
    l_brack_token: SyntaxToken,
    key: TomlKey,
    r_brack_token: SyntaxToken,
    entries: TomlKeyValueList,
) -> TomlTable {
    TomlTable::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_TABLE,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(key.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
            Some(SyntaxElement::Node(entries.into_syntax())),
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
pub fn toml_inline_table_entry_list<I, S>(items: I, separators: S) -> TomlInlineTableEntryList
where
    I: IntoIterator<Item = AnyTomlKeyValue>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = TomlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    TomlInlineTableEntryList::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_INLINE_TABLE_ENTRY_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn toml_key_part_list<I, S>(items: I, separators: S) -> TomlKeyPartList
where
    I: IntoIterator<Item = AnyTomlKeyPart>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = TomlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    TomlKeyPartList::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_KEY_PART_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn toml_key_value_list<I>(items: I) -> TomlKeyValueList
where
    I: IntoIterator<Item = AnyTomlKeyValue>,
    I::IntoIter: ExactSizeIterator,
{
    TomlKeyValueList::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_KEY_VALUE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn toml_root_item_list<I>(items: I) -> TomlRootItemList
where
    I: IntoIterator<Item = AnyTomlRootItem>,
    I::IntoIter: ExactSizeIterator,
{
    TomlRootItemList::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_ROOT_ITEM_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
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
