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
    header: AnyTomlArrayTableHeader,
    members: TomlMemberList,
) -> TomlArrayTable {
    TomlArrayTable::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_ARRAY_TABLE,
        [
            Some(SyntaxElement::Node(header.into_syntax())),
            Some(SyntaxElement::Node(members.into_syntax())),
        ],
    ))
}
pub fn toml_array_table_header(
    l_double_brack_token: SyntaxToken,
    key: AnyTomlKey,
    r_double_brack_token: SyntaxToken,
) -> TomlArrayTableHeader {
    TomlArrayTableHeader::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_ARRAY_TABLE_HEADER,
        [
            Some(SyntaxElement::Token(l_double_brack_token)),
            Some(SyntaxElement::Node(key.into_syntax())),
            Some(SyntaxElement::Token(r_double_brack_token)),
        ],
    ))
}
pub fn toml_bare_key(value_token: SyntaxToken) -> TomlBareKey {
    TomlBareKey::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_BARE_KEY,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn toml_basic_string(value_token: SyntaxToken) -> TomlBasicString {
    TomlBasicString::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_BASIC_STRING,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn toml_boolean(value_token_token: SyntaxToken) -> TomlBoolean {
    TomlBoolean::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_BOOLEAN,
        [Some(SyntaxElement::Token(value_token_token))],
    ))
}
pub fn toml_date(value_token: SyntaxToken) -> TomlDate {
    TomlDate::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_DATE,
        [Some(SyntaxElement::Token(value_token))],
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
    members: TomlInlineTableMemberList,
    r_curly_token: SyntaxToken,
) -> TomlInlineTable {
    TomlInlineTable::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_INLINE_TABLE,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(members.into_syntax())),
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
pub fn toml_key(fragments: TomlKeyFragmentList) -> TomlKey {
    TomlKey::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_KEY,
        [Some(SyntaxElement::Node(fragments.into_syntax()))],
    ))
}
pub fn toml_literal_string(value_token: SyntaxToken) -> TomlLiteralString {
    TomlLiteralString::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_LITERAL_STRING,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn toml_member(key: AnyTomlKey, eq_token: SyntaxToken, value: AnyTomlValue) -> TomlMember {
    TomlMember::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_MEMBER,
        [
            Some(SyntaxElement::Node(key.into_syntax())),
            Some(SyntaxElement::Token(eq_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn toml_multiline_basic_string(value_token: SyntaxToken) -> TomlMultilineBasicString {
    TomlMultilineBasicString::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_MULTILINE_BASIC_STRING,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn toml_multiline_literal_string(value_token: SyntaxToken) -> TomlMultilineLiteralString {
    TomlMultilineLiteralString::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_MULTILINE_LITERAL_STRING,
        [Some(SyntaxElement::Token(value_token))],
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
pub fn toml_table(header: AnyTomlTableHeader, members: TomlMemberList) -> TomlTable {
    TomlTable::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_TABLE,
        [
            Some(SyntaxElement::Node(header.into_syntax())),
            Some(SyntaxElement::Node(members.into_syntax())),
        ],
    ))
}
pub fn toml_table_header(
    l_brack_token: SyntaxToken,
    key: AnyTomlKey,
    r_brack_token: SyntaxToken,
) -> TomlTableHeader {
    TomlTableHeader::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_TABLE_HEADER,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(key.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn toml_time(value_token: SyntaxToken) -> TomlTime {
    TomlTime::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_TIME,
        [Some(SyntaxElement::Token(value_token))],
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
pub fn toml_inline_table_member_list<I, S>(items: I, separators: S) -> TomlInlineTableMemberList
where
    I: IntoIterator<Item = AnyTomlMember>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = TomlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    TomlInlineTableMemberList::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_INLINE_TABLE_MEMBER_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn toml_key_fragment_list<I, S>(items: I, separators: S) -> TomlKeyFragmentList
where
    I: IntoIterator<Item = AnyTomlKeyFragment>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = TomlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    TomlKeyFragmentList::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_KEY_FRAGMENT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn toml_member_list<I>(items: I) -> TomlMemberList
where
    I: IntoIterator<Item = AnyTomlMember>,
    I::IntoIter: ExactSizeIterator,
{
    TomlMemberList::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_MEMBER_LIST,
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
pub fn toml_bogus_key<I>(slots: I) -> TomlBogusKey
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    TomlBogusKey::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_BOGUS_KEY,
        slots,
    ))
}
pub fn toml_bogus_key_fragment<I>(slots: I) -> TomlBogusKeyFragment
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    TomlBogusKeyFragment::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_BOGUS_KEY_FRAGMENT,
        slots,
    ))
}
pub fn toml_bogus_member<I>(slots: I) -> TomlBogusMember
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    TomlBogusMember::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_BOGUS_MEMBER,
        slots,
    ))
}
pub fn toml_bogus_table_header<I>(slots: I) -> TomlBogusTableHeader
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    TomlBogusTableHeader::unwrap_cast(SyntaxNode::new_detached(
        TomlSyntaxKind::TOML_BOGUS_TABLE_HEADER,
        slots,
    ))
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
