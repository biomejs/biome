//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
use biome_json_syntax::{
    JsonSyntaxElement as SyntaxElement, JsonSyntaxNode as SyntaxNode,
    JsonSyntaxToken as SyntaxToken, *,
};
use biome_rowan::AstNode;
pub fn json_array_value(
    l_brack_token: SyntaxToken,
    elements: JsonArrayElementList,
    r_brack_token: SyntaxToken,
) -> JsonArrayValue {
    JsonArrayValue::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_ARRAY_VALUE,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(elements.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn json_boolean_value(value_token_token: SyntaxToken) -> JsonBooleanValue {
    JsonBooleanValue::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_BOOLEAN_VALUE,
        [Some(SyntaxElement::Token(value_token_token))],
    ))
}
pub fn json_member(
    name: JsonMemberName,
    colon_token: SyntaxToken,
    value: AnyJsonValue,
) -> JsonMember {
    JsonMember::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_MEMBER,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn json_member_name(value_token: SyntaxToken) -> JsonMemberName {
    JsonMemberName::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_MEMBER_NAME,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn json_null_value(value_token: SyntaxToken) -> JsonNullValue {
    JsonNullValue::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_NULL_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn json_number_value(value_token: SyntaxToken) -> JsonNumberValue {
    JsonNumberValue::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_NUMBER_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn json_object_value(
    l_curly_token: SyntaxToken,
    json_member_list: JsonMemberList,
    r_curly_token: SyntaxToken,
) -> JsonObjectValue {
    JsonObjectValue::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_OBJECT_VALUE,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(json_member_list.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn json_root(value: AnyJsonValue, eof_token: SyntaxToken) -> JsonRootBuilder {
    JsonRootBuilder {
        value,
        eof_token,
        bom_token: None,
    }
}
pub struct JsonRootBuilder {
    value: AnyJsonValue,
    eof_token: SyntaxToken,
    bom_token: Option<SyntaxToken>,
}
impl JsonRootBuilder {
    pub fn with_bom_token(mut self, bom_token: SyntaxToken) -> Self {
        self.bom_token = Some(bom_token);
        self
    }
    pub fn build(self) -> JsonRoot {
        JsonRoot::unwrap_cast(SyntaxNode::new_detached(
            JsonSyntaxKind::JSON_ROOT,
            [
                self.bom_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.value.into_syntax())),
                Some(SyntaxElement::Token(self.eof_token)),
            ],
        ))
    }
}
pub fn json_string_value(value_token: SyntaxToken) -> JsonStringValue {
    JsonStringValue::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_STRING_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn json_array_element_list<I, S>(items: I, separators: S) -> JsonArrayElementList
where
    I: IntoIterator<Item = AnyJsonValue>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = JsonSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    JsonArrayElementList::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_ARRAY_ELEMENT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn json_member_list<I, S>(items: I, separators: S) -> JsonMemberList
where
    I: IntoIterator<Item = JsonMember>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = JsonSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    JsonMemberList::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_MEMBER_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn json_bogus<I>(slots: I) -> JsonBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    JsonBogus::unwrap_cast(SyntaxNode::new_detached(JsonSyntaxKind::JSON_BOGUS, slots))
}
pub fn json_bogus_value<I>(slots: I) -> JsonBogusValue
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    JsonBogusValue::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_BOGUS_VALUE,
        slots,
    ))
}
