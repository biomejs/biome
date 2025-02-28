//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(dead_code)]
#![allow(unused)]
use crate::{
    macros::map_syntax_node,
    YamlLanguage as Language, YamlSyntaxElement as SyntaxElement,
    YamlSyntaxElementChildren as SyntaxElementChildren,
    YamlSyntaxKind::{self as SyntaxKind, *},
    YamlSyntaxList as SyntaxList, YamlSyntaxNode as SyntaxNode, YamlSyntaxToken as SyntaxToken,
};
use biome_rowan::{
    support, AstNode, AstNodeList, AstNodeListIterator, AstNodeSlotMap, AstSeparatedList,
    AstSeparatedListNodesIterator, RawSyntaxKind, SyntaxKindSet, SyntaxResult,
};
use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use std::fmt::{Debug, Formatter};
#[doc = r" Sentinel value indicating a missing element in a dynamic node, where"]
#[doc = r" the slots are not statically known."]
pub(crate) const SLOT_MAP_EMPTY_VALUE: u8 = u8::MAX;
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlArray {
    pub(crate) syntax: SyntaxNode,
}
impl YamlArray {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlArrayFields {
        YamlArrayFields {
            items: self.items(),
        }
    }
    pub fn items(&self) -> YamlArrayItemList {
        support::list(&self.syntax, 0usize)
    }
}
impl Serialize for YamlArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlArrayFields {
    pub items: YamlArrayItemList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlArrayInline {
    pub(crate) syntax: SyntaxNode,
}
impl YamlArrayInline {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlArrayInlineFields {
        YamlArrayInlineFields {
            l_brack_token: self.l_brack_token(),
            items: self.items(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> YamlArrayInlineList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for YamlArrayInline {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlArrayInlineFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub items: YamlArrayInlineList,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlArrayItem {
    pub(crate) syntax: SyntaxNode,
}
impl YamlArrayItem {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlArrayItemFields {
        YamlArrayItemFields {
            minus_token: self.minus_token(),
            item: self.item(),
        }
    }
    pub fn minus_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn item(&self) -> SyntaxResult<AnyYamlValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for YamlArrayItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlArrayItemFields {
    pub minus_token: SyntaxResult<SyntaxToken>,
    pub item: SyntaxResult<AnyYamlValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlBlockFolded {
    pub(crate) syntax: SyntaxNode,
}
impl YamlBlockFolded {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlBlockFoldedFields {
        YamlBlockFoldedFields {
            r_angle_token: self.r_angle_token(),
            value: self.value(),
        }
    }
    pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<YamlBlockValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for YamlBlockFolded {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlBlockFoldedFields {
    pub r_angle_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<YamlBlockValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlBlockLiteral {
    pub(crate) syntax: SyntaxNode,
}
impl YamlBlockLiteral {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlBlockLiteralFields {
        YamlBlockLiteralFields {
            bitwise_or_token: self.bitwise_or_token(),
            value: self.value(),
        }
    }
    pub fn bitwise_or_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<YamlBlockValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for YamlBlockLiteral {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlBlockLiteralFields {
    pub bitwise_or_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<YamlBlockValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlBlockValue {
    pub(crate) syntax: SyntaxNode,
}
impl YamlBlockValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlBlockValueFields {
        YamlBlockValueFields {
            yaml_block_value_token: self.yaml_block_value_token(),
        }
    }
    pub fn yaml_block_value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for YamlBlockValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlBlockValueFields {
    pub yaml_block_value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlBooleanValue {
    pub(crate) syntax: SyntaxNode,
}
impl YamlBooleanValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlBooleanValueFields {
        YamlBooleanValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for YamlBooleanValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlBooleanValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlDocument {
    pub(crate) syntax: SyntaxNode,
}
impl YamlDocument {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlDocumentFields {
        YamlDocumentFields {
            dashdashdash_token: self.dashdashdash_token(),
            body: self.body(),
            dotdotdot_token: self.dotdotdot_token(),
        }
    }
    pub fn dashdashdash_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn body(&self) -> SyntaxResult<AnyYamlValue> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn dotdotdot_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
}
impl Serialize for YamlDocument {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlDocumentFields {
    pub dashdashdash_token: Option<SyntaxToken>,
    pub body: SyntaxResult<AnyYamlValue>,
    pub dotdotdot_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlIdentifier {
    pub(crate) syntax: SyntaxNode,
}
impl YamlIdentifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlIdentifierFields {
        YamlIdentifierFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for YamlIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlIdentifierFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlNullValue {
    pub(crate) syntax: SyntaxNode,
}
impl YamlNullValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlNullValueFields {
        YamlNullValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for YamlNullValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlNullValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlNumberValue {
    pub(crate) syntax: SyntaxNode,
}
impl YamlNumberValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlNumberValueFields {
        YamlNumberValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for YamlNumberValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlNumberValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlObject {
    pub(crate) syntax: SyntaxNode,
}
impl YamlObject {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlObjectFields {
        YamlObjectFields {
            members: self.members(),
        }
    }
    pub fn members(&self) -> YamlObjectMemberList {
        support::list(&self.syntax, 0usize)
    }
}
impl Serialize for YamlObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlObjectFields {
    pub members: YamlObjectMemberList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlObjectMember {
    pub(crate) syntax: SyntaxNode,
}
impl YamlObjectMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlObjectMemberFields {
        YamlObjectMemberFields {
            key: self.key(),
            colon_token: self.colon_token(),
            value: self.value(),
        }
    }
    pub fn key(&self) -> SyntaxResult<YamlIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<AnyYamlValue> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for YamlObjectMember {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlObjectMemberFields {
    pub key: SyntaxResult<YamlIdentifier>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnyYamlValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlRoot {
    pub(crate) syntax: SyntaxNode,
}
impl YamlRoot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlRootFields {
        YamlRootFields {
            bom_token: self.bom_token(),
            documents: self.documents(),
            eof_token: self.eof_token(),
        }
    }
    pub fn bom_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn documents(&self) -> YamlDocumentList {
        support::list(&self.syntax, 1usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for YamlRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlRootFields {
    pub bom_token: Option<SyntaxToken>,
    pub documents: YamlDocumentList,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlStringValue {
    pub(crate) syntax: SyntaxNode,
}
impl YamlStringValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlStringValueFields {
        YamlStringValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for YamlStringValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlStringValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyYamlScalar {
    YamlBooleanValue(YamlBooleanValue),
    YamlNullValue(YamlNullValue),
    YamlNumberValue(YamlNumberValue),
    YamlStringValue(YamlStringValue),
}
impl AnyYamlScalar {
    pub fn as_yaml_boolean_value(&self) -> Option<&YamlBooleanValue> {
        match &self {
            AnyYamlScalar::YamlBooleanValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_null_value(&self) -> Option<&YamlNullValue> {
        match &self {
            AnyYamlScalar::YamlNullValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_number_value(&self) -> Option<&YamlNumberValue> {
        match &self {
            AnyYamlScalar::YamlNumberValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_string_value(&self) -> Option<&YamlStringValue> {
        match &self {
            AnyYamlScalar::YamlStringValue(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyYamlValue {
    AnyYamlScalar(AnyYamlScalar),
    YamlArray(YamlArray),
    YamlArrayInline(YamlArrayInline),
    YamlBogusValue(YamlBogusValue),
    YamlObject(YamlObject),
}
impl AnyYamlValue {
    pub fn as_any_yaml_scalar(&self) -> Option<&AnyYamlScalar> {
        match &self {
            AnyYamlValue::AnyYamlScalar(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_array(&self) -> Option<&YamlArray> {
        match &self {
            AnyYamlValue::YamlArray(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_array_inline(&self) -> Option<&YamlArrayInline> {
        match &self {
            AnyYamlValue::YamlArrayInline(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_bogus_value(&self) -> Option<&YamlBogusValue> {
        match &self {
            AnyYamlValue::YamlBogusValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_object(&self) -> Option<&YamlObject> {
        match &self {
            AnyYamlValue::YamlObject(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for YamlArray {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_ARRAY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_ARRAY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for YamlArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlArray")
                .field("items", &self.items())
                .finish()
        } else {
            f.debug_struct("YamlArray").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlArray> for SyntaxNode {
    fn from(n: YamlArray) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlArray> for SyntaxElement {
    fn from(n: YamlArray) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlArrayInline {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_ARRAY_INLINE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_ARRAY_INLINE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for YamlArrayInline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlArrayInline")
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("items", &self.items())
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .finish()
        } else {
            f.debug_struct("YamlArrayInline").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlArrayInline> for SyntaxNode {
    fn from(n: YamlArrayInline) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlArrayInline> for SyntaxElement {
    fn from(n: YamlArrayInline) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlArrayItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_ARRAY_ITEM as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_ARRAY_ITEM
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for YamlArrayItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlArrayItem")
                .field(
                    "minus_token",
                    &support::DebugSyntaxResult(self.minus_token()),
                )
                .field("item", &support::DebugSyntaxResult(self.item()))
                .finish()
        } else {
            f.debug_struct("YamlArrayItem").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlArrayItem> for SyntaxNode {
    fn from(n: YamlArrayItem) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlArrayItem> for SyntaxElement {
    fn from(n: YamlArrayItem) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlBlockFolded {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BLOCK_FOLDED as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BLOCK_FOLDED
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for YamlBlockFolded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlBlockFolded")
                .field(
                    "r_angle_token",
                    &support::DebugSyntaxResult(self.r_angle_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("YamlBlockFolded").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlBlockFolded> for SyntaxNode {
    fn from(n: YamlBlockFolded) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlBlockFolded> for SyntaxElement {
    fn from(n: YamlBlockFolded) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlBlockLiteral {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BLOCK_LITERAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BLOCK_LITERAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for YamlBlockLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlBlockLiteral")
                .field(
                    "bitwise_or_token",
                    &support::DebugSyntaxResult(self.bitwise_or_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("YamlBlockLiteral").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlBlockLiteral> for SyntaxNode {
    fn from(n: YamlBlockLiteral) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlBlockLiteral> for SyntaxElement {
    fn from(n: YamlBlockLiteral) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlBlockValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BLOCK_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BLOCK_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for YamlBlockValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlBlockValue")
                .field(
                    "yaml_block_value_token",
                    &support::DebugSyntaxResult(self.yaml_block_value_token()),
                )
                .finish()
        } else {
            f.debug_struct("YamlBlockValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlBlockValue> for SyntaxNode {
    fn from(n: YamlBlockValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlBlockValue> for SyntaxElement {
    fn from(n: YamlBlockValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlBooleanValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BOOLEAN_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BOOLEAN_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for YamlBooleanValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlBooleanValue")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("YamlBooleanValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlBooleanValue> for SyntaxNode {
    fn from(n: YamlBooleanValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlBooleanValue> for SyntaxElement {
    fn from(n: YamlBooleanValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlDocument {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_DOCUMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_DOCUMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for YamlDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlDocument")
                .field(
                    "dashdashdash_token",
                    &support::DebugOptionalElement(self.dashdashdash_token()),
                )
                .field("body", &support::DebugSyntaxResult(self.body()))
                .field(
                    "dotdotdot_token",
                    &support::DebugOptionalElement(self.dotdotdot_token()),
                )
                .finish()
        } else {
            f.debug_struct("YamlDocument").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlDocument> for SyntaxNode {
    fn from(n: YamlDocument) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlDocument> for SyntaxElement {
    fn from(n: YamlDocument) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlIdentifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_IDENTIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_IDENTIFIER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for YamlIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlIdentifier")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("YamlIdentifier").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlIdentifier> for SyntaxNode {
    fn from(n: YamlIdentifier) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlIdentifier> for SyntaxElement {
    fn from(n: YamlIdentifier) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlNullValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_NULL_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_NULL_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for YamlNullValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlNullValue")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("YamlNullValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlNullValue> for SyntaxNode {
    fn from(n: YamlNullValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlNullValue> for SyntaxElement {
    fn from(n: YamlNullValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlNumberValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_NUMBER_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_NUMBER_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for YamlNumberValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlNumberValue")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("YamlNumberValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlNumberValue> for SyntaxNode {
    fn from(n: YamlNumberValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlNumberValue> for SyntaxElement {
    fn from(n: YamlNumberValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlObject {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_OBJECT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_OBJECT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for YamlObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlObject")
                .field("members", &self.members())
                .finish()
        } else {
            f.debug_struct("YamlObject").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlObject> for SyntaxNode {
    fn from(n: YamlObject) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlObject> for SyntaxElement {
    fn from(n: YamlObject) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlObjectMember {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_OBJECT_MEMBER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_OBJECT_MEMBER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for YamlObjectMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlObjectMember")
                .field("key", &support::DebugSyntaxResult(self.key()))
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("YamlObjectMember").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlObjectMember> for SyntaxNode {
    fn from(n: YamlObjectMember) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlObjectMember> for SyntaxElement {
    fn from(n: YamlObjectMember) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlRoot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_ROOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_ROOT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for YamlRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlRoot")
                .field(
                    "bom_token",
                    &support::DebugOptionalElement(self.bom_token()),
                )
                .field("documents", &self.documents())
                .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
                .finish()
        } else {
            f.debug_struct("YamlRoot").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlRoot> for SyntaxNode {
    fn from(n: YamlRoot) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlRoot> for SyntaxElement {
    fn from(n: YamlRoot) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlStringValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_STRING_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_STRING_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for YamlStringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlStringValue")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("YamlStringValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlStringValue> for SyntaxNode {
    fn from(n: YamlStringValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlStringValue> for SyntaxElement {
    fn from(n: YamlStringValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl From<YamlBooleanValue> for AnyYamlScalar {
    fn from(node: YamlBooleanValue) -> AnyYamlScalar {
        AnyYamlScalar::YamlBooleanValue(node)
    }
}
impl From<YamlNullValue> for AnyYamlScalar {
    fn from(node: YamlNullValue) -> AnyYamlScalar {
        AnyYamlScalar::YamlNullValue(node)
    }
}
impl From<YamlNumberValue> for AnyYamlScalar {
    fn from(node: YamlNumberValue) -> AnyYamlScalar {
        AnyYamlScalar::YamlNumberValue(node)
    }
}
impl From<YamlStringValue> for AnyYamlScalar {
    fn from(node: YamlStringValue) -> AnyYamlScalar {
        AnyYamlScalar::YamlStringValue(node)
    }
}
impl AstNode for AnyYamlScalar {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = YamlBooleanValue::KIND_SET
        .union(YamlNullValue::KIND_SET)
        .union(YamlNumberValue::KIND_SET)
        .union(YamlStringValue::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            YAML_BOOLEAN_VALUE | YAML_NULL_VALUE | YAML_NUMBER_VALUE | YAML_STRING_VALUE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            YAML_BOOLEAN_VALUE => AnyYamlScalar::YamlBooleanValue(YamlBooleanValue { syntax }),
            YAML_NULL_VALUE => AnyYamlScalar::YamlNullValue(YamlNullValue { syntax }),
            YAML_NUMBER_VALUE => AnyYamlScalar::YamlNumberValue(YamlNumberValue { syntax }),
            YAML_STRING_VALUE => AnyYamlScalar::YamlStringValue(YamlStringValue { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyYamlScalar::YamlBooleanValue(it) => &it.syntax,
            AnyYamlScalar::YamlNullValue(it) => &it.syntax,
            AnyYamlScalar::YamlNumberValue(it) => &it.syntax,
            AnyYamlScalar::YamlStringValue(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyYamlScalar::YamlBooleanValue(it) => it.syntax,
            AnyYamlScalar::YamlNullValue(it) => it.syntax,
            AnyYamlScalar::YamlNumberValue(it) => it.syntax,
            AnyYamlScalar::YamlStringValue(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyYamlScalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyYamlScalar::YamlBooleanValue(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlScalar::YamlNullValue(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlScalar::YamlNumberValue(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlScalar::YamlStringValue(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyYamlScalar> for SyntaxNode {
    fn from(n: AnyYamlScalar) -> SyntaxNode {
        match n {
            AnyYamlScalar::YamlBooleanValue(it) => it.into(),
            AnyYamlScalar::YamlNullValue(it) => it.into(),
            AnyYamlScalar::YamlNumberValue(it) => it.into(),
            AnyYamlScalar::YamlStringValue(it) => it.into(),
        }
    }
}
impl From<AnyYamlScalar> for SyntaxElement {
    fn from(n: AnyYamlScalar) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<YamlArray> for AnyYamlValue {
    fn from(node: YamlArray) -> AnyYamlValue {
        AnyYamlValue::YamlArray(node)
    }
}
impl From<YamlArrayInline> for AnyYamlValue {
    fn from(node: YamlArrayInline) -> AnyYamlValue {
        AnyYamlValue::YamlArrayInline(node)
    }
}
impl From<YamlBogusValue> for AnyYamlValue {
    fn from(node: YamlBogusValue) -> AnyYamlValue {
        AnyYamlValue::YamlBogusValue(node)
    }
}
impl From<YamlObject> for AnyYamlValue {
    fn from(node: YamlObject) -> AnyYamlValue {
        AnyYamlValue::YamlObject(node)
    }
}
impl AstNode for AnyYamlValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyYamlScalar::KIND_SET
        .union(YamlArray::KIND_SET)
        .union(YamlArrayInline::KIND_SET)
        .union(YamlBogusValue::KIND_SET)
        .union(YamlObject::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            YAML_ARRAY | YAML_ARRAY_INLINE | YAML_BOGUS_VALUE | YAML_OBJECT => true,
            k if AnyYamlScalar::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            YAML_ARRAY => AnyYamlValue::YamlArray(YamlArray { syntax }),
            YAML_ARRAY_INLINE => AnyYamlValue::YamlArrayInline(YamlArrayInline { syntax }),
            YAML_BOGUS_VALUE => AnyYamlValue::YamlBogusValue(YamlBogusValue { syntax }),
            YAML_OBJECT => AnyYamlValue::YamlObject(YamlObject { syntax }),
            _ => {
                if let Some(any_yaml_scalar) = AnyYamlScalar::cast(syntax) {
                    return Some(AnyYamlValue::AnyYamlScalar(any_yaml_scalar));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyYamlValue::YamlArray(it) => &it.syntax,
            AnyYamlValue::YamlArrayInline(it) => &it.syntax,
            AnyYamlValue::YamlBogusValue(it) => &it.syntax,
            AnyYamlValue::YamlObject(it) => &it.syntax,
            AnyYamlValue::AnyYamlScalar(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyYamlValue::YamlArray(it) => it.syntax,
            AnyYamlValue::YamlArrayInline(it) => it.syntax,
            AnyYamlValue::YamlBogusValue(it) => it.syntax,
            AnyYamlValue::YamlObject(it) => it.syntax,
            AnyYamlValue::AnyYamlScalar(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyYamlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyYamlValue::AnyYamlScalar(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlValue::YamlArray(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlValue::YamlArrayInline(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlValue::YamlBogusValue(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlValue::YamlObject(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyYamlValue> for SyntaxNode {
    fn from(n: AnyYamlValue) -> SyntaxNode {
        match n {
            AnyYamlValue::AnyYamlScalar(it) => it.into(),
            AnyYamlValue::YamlArray(it) => it.into(),
            AnyYamlValue::YamlArrayInline(it) => it.into(),
            AnyYamlValue::YamlBogusValue(it) => it.into(),
            AnyYamlValue::YamlObject(it) => it.into(),
        }
    }
}
impl From<AnyYamlValue> for SyntaxElement {
    fn from(n: AnyYamlValue) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyYamlScalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyYamlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlArrayInline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlArrayItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlBlockFolded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlBlockLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlBlockValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlBooleanValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlNullValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlNumberValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlObjectMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlStringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct YamlBogus {
    syntax: SyntaxNode,
}
impl YamlBogus {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for YamlBogus {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BOGUS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BOGUS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for YamlBogus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("YamlBogus")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<YamlBogus> for SyntaxNode {
    fn from(n: YamlBogus) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlBogus> for SyntaxElement {
    fn from(n: YamlBogus) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct YamlBogusValue {
    syntax: SyntaxNode,
}
impl YamlBogusValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for YamlBogusValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BOGUS_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BOGUS_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for YamlBogusValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("YamlBogusValue")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<YamlBogusValue> for SyntaxNode {
    fn from(n: YamlBogusValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlBogusValue> for SyntaxElement {
    fn from(n: YamlBogusValue) -> SyntaxElement {
        n.syntax.into()
    }
}
biome_rowan::declare_node_union! { pub AnyYamlBogusNode = YamlBogus | YamlBogusValue }
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct YamlArrayInlineList {
    syntax_list: SyntaxList,
}
impl YamlArrayInlineList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for YamlArrayInlineList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_ARRAY_INLINE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_ARRAY_INLINE_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<YamlArrayInlineList> {
        if Self::can_cast(syntax.kind()) {
            Some(YamlArrayInlineList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for YamlArrayInlineList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for YamlArrayInlineList {
    type Language = Language;
    type Node = AnyYamlScalar;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for YamlArrayInlineList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("YamlArrayInlineList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for YamlArrayInlineList {
    type Item = SyntaxResult<AnyYamlScalar>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyYamlScalar>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &YamlArrayInlineList {
    type Item = SyntaxResult<AnyYamlScalar>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyYamlScalar>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct YamlArrayItemList {
    syntax_list: SyntaxList,
}
impl YamlArrayItemList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for YamlArrayItemList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_ARRAY_ITEM_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_ARRAY_ITEM_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<YamlArrayItemList> {
        if Self::can_cast(syntax.kind()) {
            Some(YamlArrayItemList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for YamlArrayItemList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstNodeList for YamlArrayItemList {
    type Language = Language;
    type Node = YamlArrayItem;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for YamlArrayItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("YamlArrayItemList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &YamlArrayItemList {
    type Item = YamlArrayItem;
    type IntoIter = AstNodeListIterator<Language, YamlArrayItem>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for YamlArrayItemList {
    type Item = YamlArrayItem;
    type IntoIter = AstNodeListIterator<Language, YamlArrayItem>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct YamlDocumentList {
    syntax_list: SyntaxList,
}
impl YamlDocumentList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for YamlDocumentList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_DOCUMENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_DOCUMENT_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<YamlDocumentList> {
        if Self::can_cast(syntax.kind()) {
            Some(YamlDocumentList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for YamlDocumentList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstNodeList for YamlDocumentList {
    type Language = Language;
    type Node = YamlDocument;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for YamlDocumentList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("YamlDocumentList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &YamlDocumentList {
    type Item = YamlDocument;
    type IntoIter = AstNodeListIterator<Language, YamlDocument>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for YamlDocumentList {
    type Item = YamlDocument;
    type IntoIter = AstNodeListIterator<Language, YamlDocument>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct YamlObjectMemberList {
    syntax_list: SyntaxList,
}
impl YamlObjectMemberList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for YamlObjectMemberList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_OBJECT_MEMBER_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_OBJECT_MEMBER_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<YamlObjectMemberList> {
        if Self::can_cast(syntax.kind()) {
            Some(YamlObjectMemberList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for YamlObjectMemberList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstNodeList for YamlObjectMemberList {
    type Language = Language;
    type Node = YamlObjectMember;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for YamlObjectMemberList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("YamlObjectMemberList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &YamlObjectMemberList {
    type Item = YamlObjectMember;
    type IntoIter = AstNodeListIterator<Language, YamlObjectMember>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for YamlObjectMemberList {
    type Item = YamlObjectMember;
    type IntoIter = AstNodeListIterator<Language, YamlObjectMember>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone)]
pub struct DebugSyntaxElementChildren(pub SyntaxElementChildren);
impl Debug for DebugSyntaxElementChildren {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.clone().0.map(DebugSyntaxElement))
            .finish()
    }
}
struct DebugSyntaxElement(SyntaxElement);
impl Debug for DebugSyntaxElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            SyntaxElement::Node(node) => {
                map_syntax_node ! (node . clone () , node => std :: fmt :: Debug :: fmt (& node , f))
            }
            SyntaxElement::Token(token) => Debug::fmt(token, f),
        }
    }
}
