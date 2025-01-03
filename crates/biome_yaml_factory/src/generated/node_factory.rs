//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
use biome_rowan::AstNode;
use biome_yaml_syntax::{
    YamlSyntaxElement as SyntaxElement, YamlSyntaxNode as SyntaxNode,
    YamlSyntaxToken as SyntaxToken, *,
};
pub fn yaml_array(items: YamlArrayItemList) -> YamlArray {
    YamlArray::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_ARRAY,
        [Some(SyntaxElement::Node(items.into_syntax()))],
    ))
}
pub fn yaml_array_inline(
    l_brack_token: SyntaxToken,
    items: YamlArrayInlineList,
    r_brack_token: SyntaxToken,
) -> YamlArrayInline {
    YamlArrayInline::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_ARRAY_INLINE,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn yaml_array_item(minus_token: SyntaxToken, item: AnyYamlValue) -> YamlArrayItem {
    YamlArrayItem::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_ARRAY_ITEM,
        [
            Some(SyntaxElement::Token(minus_token)),
            Some(SyntaxElement::Node(item.into_syntax())),
        ],
    ))
}
pub fn yaml_block_folded(r_angle_token: SyntaxToken, value: YamlBlockValue) -> YamlBlockFolded {
    YamlBlockFolded::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_BLOCK_FOLDED,
        [
            Some(SyntaxElement::Token(r_angle_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn yaml_block_literal(
    bitwise_or_token: SyntaxToken,
    value: YamlBlockValue,
) -> YamlBlockLiteral {
    YamlBlockLiteral::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_BLOCK_LITERAL,
        [
            Some(SyntaxElement::Token(bitwise_or_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn yaml_block_value(yaml_block_value_token: SyntaxToken) -> YamlBlockValue {
    YamlBlockValue::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_BLOCK_VALUE,
        [Some(SyntaxElement::Token(yaml_block_value_token))],
    ))
}
pub fn yaml_boolean_value(value_token: SyntaxToken) -> YamlBooleanValue {
    YamlBooleanValue::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_BOOLEAN_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn yaml_document(body: AnyYamlValue) -> YamlDocumentBuilder {
    YamlDocumentBuilder {
        body,
        dashdashdash_token: None,
        dotdotdot_token: None,
    }
}
pub struct YamlDocumentBuilder {
    body: AnyYamlValue,
    dashdashdash_token: Option<SyntaxToken>,
    dotdotdot_token: Option<SyntaxToken>,
}
impl YamlDocumentBuilder {
    pub fn with_dashdashdash_token(mut self, dashdashdash_token: SyntaxToken) -> Self {
        self.dashdashdash_token = Some(dashdashdash_token);
        self
    }
    pub fn with_dotdotdot_token(mut self, dotdotdot_token: SyntaxToken) -> Self {
        self.dotdotdot_token = Some(dotdotdot_token);
        self
    }
    pub fn build(self) -> YamlDocument {
        YamlDocument::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_DOCUMENT,
            [
                self.dashdashdash_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.body.into_syntax())),
                self.dotdotdot_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn yaml_identifier(value_token: SyntaxToken) -> YamlIdentifier {
    YamlIdentifier::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_IDENTIFIER,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn yaml_null_value(value_token: SyntaxToken) -> YamlNullValue {
    YamlNullValue::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_NULL_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn yaml_number_value(value_token: SyntaxToken) -> YamlNumberValue {
    YamlNumberValue::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_NUMBER_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn yaml_object(members: YamlObjectMemberList) -> YamlObject {
    YamlObject::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_OBJECT,
        [Some(SyntaxElement::Node(members.into_syntax()))],
    ))
}
pub fn yaml_object_member(
    key: YamlIdentifier,
    colon_token: SyntaxToken,
    value: AnyYamlValue,
) -> YamlObjectMember {
    YamlObjectMember::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_OBJECT_MEMBER,
        [
            Some(SyntaxElement::Node(key.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn yaml_root(documents: YamlDocumentList, eof_token: SyntaxToken) -> YamlRootBuilder {
    YamlRootBuilder {
        documents,
        eof_token,
        bom_token: None,
    }
}
pub struct YamlRootBuilder {
    documents: YamlDocumentList,
    eof_token: SyntaxToken,
    bom_token: Option<SyntaxToken>,
}
impl YamlRootBuilder {
    pub fn with_bom_token(mut self, bom_token: SyntaxToken) -> Self {
        self.bom_token = Some(bom_token);
        self
    }
    pub fn build(self) -> YamlRoot {
        YamlRoot::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_ROOT,
            [
                self.bom_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.documents.into_syntax())),
                Some(SyntaxElement::Token(self.eof_token)),
            ],
        ))
    }
}
pub fn yaml_string_value(value_token: SyntaxToken) -> YamlStringValue {
    YamlStringValue::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_STRING_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn yaml_array_inline_list<I, S>(items: I, separators: S) -> YamlArrayInlineList
where
    I: IntoIterator<Item = AnyYamlScalar>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = YamlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    YamlArrayInlineList::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_ARRAY_INLINE_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn yaml_array_item_list<I>(items: I) -> YamlArrayItemList
where
    I: IntoIterator<Item = YamlArrayItem>,
    I::IntoIter: ExactSizeIterator,
{
    YamlArrayItemList::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_ARRAY_ITEM_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn yaml_document_list<I>(items: I) -> YamlDocumentList
where
    I: IntoIterator<Item = YamlDocument>,
    I::IntoIter: ExactSizeIterator,
{
    YamlDocumentList::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_DOCUMENT_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn yaml_object_member_list<I>(items: I) -> YamlObjectMemberList
where
    I: IntoIterator<Item = YamlObjectMember>,
    I::IntoIter: ExactSizeIterator,
{
    YamlObjectMemberList::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_OBJECT_MEMBER_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn yaml_bogus<I>(slots: I) -> YamlBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    YamlBogus::unwrap_cast(SyntaxNode::new_detached(YamlSyntaxKind::YAML_BOGUS, slots))
}
pub fn yaml_bogus_value<I>(slots: I) -> YamlBogusValue
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    YamlBogusValue::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_BOGUS_VALUE,
        slots,
    ))
}
