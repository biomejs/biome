//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(dead_code)]
#![allow(unused)]
use crate::{
    YamlLanguage as Language, YamlSyntaxElement as SyntaxElement,
    YamlSyntaxElementChildren as SyntaxElementChildren,
    YamlSyntaxKind::{self as SyntaxKind, *},
    YamlSyntaxList as SyntaxList, YamlSyntaxNode as SyntaxNode, YamlSyntaxToken as SyntaxToken,
    macros::map_syntax_node,
};
use biome_rowan::{
    AstNode, AstNodeList, AstNodeListIterator, AstNodeSlotMap, AstSeparatedList,
    AstSeparatedListNodesIterator, RawSyntaxKind, SyntaxKindSet, SyntaxResult, support,
};
use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use std::fmt::{Debug, Formatter};
#[doc = r" Sentinel value indicating a missing element in a dynamic node, where"]
#[doc = r" the slots are not statically known."]
pub(crate) const SLOT_MAP_EMPTY_VALUE: u8 = u8::MAX;
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlAliasNode {
    pub(crate) syntax: SyntaxNode,
}
impl YamlAliasNode {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlAliasNodeFields {
        YamlAliasNodeFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for YamlAliasNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlAliasNodeFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlAnchorProperty {
    pub(crate) syntax: SyntaxNode,
}
impl YamlAnchorProperty {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlAnchorPropertyFields {
        YamlAnchorPropertyFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for YamlAnchorProperty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlAnchorPropertyFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlBlockCollection {
    pub(crate) syntax: SyntaxNode,
}
impl YamlBlockCollection {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlBlockCollectionFields {
        YamlBlockCollectionFields {
            properties: self.properties(),
            content: self.content(),
        }
    }
    pub fn properties(&self) -> SyntaxResult<YamlPropertyList> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn content(&self) -> SyntaxResult<AnyYamlBlockContent> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for YamlBlockCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlBlockCollectionFields {
    pub properties: SyntaxResult<YamlPropertyList>,
    pub content: SyntaxResult<AnyYamlBlockContent>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlBlockMapExplicitEntry {
    pub(crate) syntax: SyntaxNode,
}
impl YamlBlockMapExplicitEntry {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlBlockMapExplicitEntryFields {
        YamlBlockMapExplicitEntryFields {
            key: self.key(),
            value: self.value(),
        }
    }
    pub fn key(&self) -> SyntaxResult<YamlBlockMapExplicitKey> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn value(&self) -> Option<YamlBlockMapExplicitValue> {
        support::node(&self.syntax, 1usize)
    }
}
impl Serialize for YamlBlockMapExplicitEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlBlockMapExplicitEntryFields {
    pub key: SyntaxResult<YamlBlockMapExplicitKey>,
    pub value: Option<YamlBlockMapExplicitValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlBlockMapExplicitKey {
    pub(crate) syntax: SyntaxNode,
}
impl YamlBlockMapExplicitKey {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlBlockMapExplicitKeyFields {
        YamlBlockMapExplicitKeyFields {
            question_mark_token: self.question_mark_token(),
            key: self.key(),
        }
    }
    pub fn question_mark_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn key(&self) -> SyntaxResult<YamlIndentedBlock> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for YamlBlockMapExplicitKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlBlockMapExplicitKeyFields {
    pub question_mark_token: SyntaxResult<SyntaxToken>,
    pub key: SyntaxResult<YamlIndentedBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlBlockMapExplicitValue {
    pub(crate) syntax: SyntaxNode,
}
impl YamlBlockMapExplicitValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlBlockMapExplicitValueFields {
        YamlBlockMapExplicitValueFields {
            colon_token: self.colon_token(),
            value: self.value(),
        }
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<YamlIndentedBlock> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for YamlBlockMapExplicitValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlBlockMapExplicitValueFields {
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<YamlIndentedBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlBlockMapImplicitEntry {
    pub(crate) syntax: SyntaxNode,
}
impl YamlBlockMapImplicitEntry {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlBlockMapImplicitEntryFields {
        YamlBlockMapImplicitEntryFields {
            key: self.key(),
            value: self.value(),
        }
    }
    pub fn key(&self) -> Option<AnyYamlBlockMapImplicitKey> {
        support::node(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<YamlBlockMapImplicitValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for YamlBlockMapImplicitEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlBlockMapImplicitEntryFields {
    pub key: Option<AnyYamlBlockMapImplicitKey>,
    pub value: SyntaxResult<YamlBlockMapImplicitValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlBlockMapImplicitValue {
    pub(crate) syntax: SyntaxNode,
}
impl YamlBlockMapImplicitValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlBlockMapImplicitValueFields {
        YamlBlockMapImplicitValueFields {
            colon_token: self.colon_token(),
            value: self.value(),
        }
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<AnyYamlNode> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for YamlBlockMapImplicitValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlBlockMapImplicitValueFields {
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnyYamlNode>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlBlockMapping {
    pub(crate) syntax: SyntaxNode,
}
impl YamlBlockMapping {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlBlockMappingFields {
        YamlBlockMappingFields {
            indent_token: self.indent_token(),
            entries: self.entries(),
            dedent_token: self.dedent_token(),
        }
    }
    pub fn indent_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn entries(&self) -> YamlBlockMapEntryList {
        support::list(&self.syntax, 1usize)
    }
    pub fn dedent_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
}
impl Serialize for YamlBlockMapping {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlBlockMappingFields {
    pub indent_token: SyntaxResult<SyntaxToken>,
    pub entries: YamlBlockMapEntryList,
    pub dedent_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlBlockSequence {
    pub(crate) syntax: SyntaxNode,
}
impl YamlBlockSequence {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlBlockSequenceFields {
        YamlBlockSequenceFields {
            indent_token: self.indent_token(),
            entries: self.entries(),
            dedent_token: self.dedent_token(),
        }
    }
    pub fn indent_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn entries(&self) -> YamlBlockSequenceEntryList {
        support::list(&self.syntax, 1usize)
    }
    pub fn dedent_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
}
impl Serialize for YamlBlockSequence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlBlockSequenceFields {
    pub indent_token: Option<SyntaxToken>,
    pub entries: YamlBlockSequenceEntryList,
    pub dedent_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlBlockSequenceEntry {
    pub(crate) syntax: SyntaxNode,
}
impl YamlBlockSequenceEntry {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlBlockSequenceEntryFields {
        YamlBlockSequenceEntryFields {
            minus_token: self.minus_token(),
            value: self.value(),
        }
    }
    pub fn minus_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<YamlIndentedBlock> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for YamlBlockSequenceEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlBlockSequenceEntryFields {
    pub minus_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<YamlIndentedBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlCompactMapping {
    pub(crate) syntax: SyntaxNode,
}
impl YamlCompactMapping {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlCompactMappingFields {
        YamlCompactMappingFields {
            entries: self.entries(),
        }
    }
    pub fn entries(&self) -> YamlBlockSequenceEntryList {
        support::list(&self.syntax, 0usize)
    }
}
impl Serialize for YamlCompactMapping {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlCompactMappingFields {
    pub entries: YamlBlockSequenceEntryList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlCompactSequence {
    pub(crate) syntax: SyntaxNode,
}
impl YamlCompactSequence {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlCompactSequenceFields {
        YamlCompactSequenceFields {
            entries: self.entries(),
        }
    }
    pub fn entries(&self) -> YamlBlockSequenceEntryList {
        support::list(&self.syntax, 0usize)
    }
}
impl Serialize for YamlCompactSequence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlCompactSequenceFields {
    pub entries: YamlBlockSequenceEntryList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlDirective {
    pub(crate) syntax: SyntaxNode,
}
impl YamlDirective {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlDirectiveFields {
        YamlDirectiveFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for YamlDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlDirectiveFields {
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
            bom_token: self.bom_token(),
            directives: self.directives(),
            dashdashdash_token: self.dashdashdash_token(),
            node: self.node(),
            dotdotdot_token: self.dotdotdot_token(),
        }
    }
    pub fn bom_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn directives(&self) -> YamlDirectiveList {
        support::list(&self.syntax, 1usize)
    }
    pub fn dashdashdash_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
    pub fn node(&self) -> SyntaxResult<AnyYamlNode> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn dotdotdot_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 4usize)
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
    pub bom_token: Option<SyntaxToken>,
    pub directives: YamlDirectiveList,
    pub dashdashdash_token: Option<SyntaxToken>,
    pub node: SyntaxResult<AnyYamlNode>,
    pub dotdotdot_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlDoubleQuotedScalar {
    pub(crate) syntax: SyntaxNode,
}
impl YamlDoubleQuotedScalar {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlDoubleQuotedScalarFields {
        YamlDoubleQuotedScalarFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for YamlDoubleQuotedScalar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlDoubleQuotedScalarFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlFlowJsonNode {
    pub(crate) syntax: SyntaxNode,
}
impl YamlFlowJsonNode {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlFlowJsonNodeFields {
        YamlFlowJsonNodeFields {
            properties: self.properties(),
            content: self.content(),
        }
    }
    pub fn properties(&self) -> SyntaxResult<YamlPropertyList> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn content(&self) -> Option<AnyYamlJsonContent> {
        support::node(&self.syntax, 1usize)
    }
}
impl Serialize for YamlFlowJsonNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlFlowJsonNodeFields {
    pub properties: SyntaxResult<YamlPropertyList>,
    pub content: Option<AnyYamlJsonContent>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlFlowMapExplicitEntry {
    pub(crate) syntax: SyntaxNode,
}
impl YamlFlowMapExplicitEntry {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlFlowMapExplicitEntryFields {
        YamlFlowMapExplicitEntryFields {
            question_mark_token: self.question_mark_token(),
            entry: self.entry(),
        }
    }
    pub fn question_mark_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn entry(&self) -> Option<YamlFlowMapImplicitEntry> {
        support::node(&self.syntax, 1usize)
    }
}
impl Serialize for YamlFlowMapExplicitEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlFlowMapExplicitEntryFields {
    pub question_mark_token: SyntaxResult<SyntaxToken>,
    pub entry: Option<YamlFlowMapImplicitEntry>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlFlowMapImplicitEntry {
    pub(crate) syntax: SyntaxNode,
}
impl YamlFlowMapImplicitEntry {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlFlowMapImplicitEntryFields {
        YamlFlowMapImplicitEntryFields {
            key: self.key(),
            colon_token: self.colon_token(),
            value: self.value(),
        }
    }
    pub fn key(&self) -> Option<AnyYamlFlowMapImplicitKey> {
        support::node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<AnyYamlFlowNode> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for YamlFlowMapImplicitEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlFlowMapImplicitEntryFields {
    pub key: Option<AnyYamlFlowMapImplicitKey>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnyYamlFlowNode>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlFlowMapping {
    pub(crate) syntax: SyntaxNode,
}
impl YamlFlowMapping {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlFlowMappingFields {
        YamlFlowMappingFields {
            l_curly_token: self.l_curly_token(),
            entries: self.entries(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn entries(&self) -> YamlFlowMapEntryList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for YamlFlowMapping {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlFlowMappingFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub entries: YamlFlowMapEntryList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlFlowSequence {
    pub(crate) syntax: SyntaxNode,
}
impl YamlFlowSequence {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlFlowSequenceFields {
        YamlFlowSequenceFields {
            l_brack_token: self.l_brack_token(),
            entries: self.entries(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn entries(&self) -> YamlFlowSequenceEntryList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for YamlFlowSequence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlFlowSequenceFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub entries: YamlFlowSequenceEntryList,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlFlowYamlNode {
    pub(crate) syntax: SyntaxNode,
}
impl YamlFlowYamlNode {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlFlowYamlNodeFields {
        YamlFlowYamlNodeFields {
            properties: self.properties(),
            content: self.content(),
        }
    }
    pub fn properties(&self) -> SyntaxResult<YamlPropertyList> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn content(&self) -> Option<YamlPlainScalar> {
        support::node(&self.syntax, 1usize)
    }
}
impl Serialize for YamlFlowYamlNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlFlowYamlNodeFields {
    pub properties: SyntaxResult<YamlPropertyList>,
    pub content: Option<YamlPlainScalar>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlFoldedScalar {
    pub(crate) syntax: SyntaxNode,
}
impl YamlFoldedScalar {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlFoldedScalarFields {
        YamlFoldedScalarFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for YamlFoldedScalar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlFoldedScalarFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlLiteralScalar {
    pub(crate) syntax: SyntaxNode,
}
impl YamlLiteralScalar {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlLiteralScalarFields {
        YamlLiteralScalarFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for YamlLiteralScalar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlLiteralScalarFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlPlainScalar {
    pub(crate) syntax: SyntaxNode,
}
impl YamlPlainScalar {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlPlainScalarFields {
        YamlPlainScalarFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for YamlPlainScalar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlPlainScalarFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlPropertyList {
    pub(crate) syntax: SyntaxNode,
}
impl YamlPropertyList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlPropertyListFields {
        YamlPropertyListFields {
            any_yaml_property: self.any_yaml_property(),
        }
    }
    pub fn any_yaml_property(&self) -> SyntaxResult<AnyYamlProperty> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for YamlPropertyList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlPropertyListFields {
    pub any_yaml_property: SyntaxResult<AnyYamlProperty>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlSingleQuotedScalar {
    pub(crate) syntax: SyntaxNode,
}
impl YamlSingleQuotedScalar {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlSingleQuotedScalarFields {
        YamlSingleQuotedScalarFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for YamlSingleQuotedScalar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlSingleQuotedScalarFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlStream {
    pub(crate) syntax: SyntaxNode,
}
impl YamlStream {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlStreamFields {
        YamlStreamFields {
            documents: self.documents(),
            eof_token: self.eof_token(),
        }
    }
    pub fn documents(&self) -> YamlDocumentList {
        support::list(&self.syntax, 0usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for YamlStream {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlStreamFields {
    pub documents: YamlDocumentList,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlTagProperty {
    pub(crate) syntax: SyntaxNode,
}
impl YamlTagProperty {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlTagPropertyFields {
        YamlTagPropertyFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for YamlTagProperty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlTagPropertyFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyYamlBlockContent {
    YamlBlockMapping(YamlBlockMapping),
    YamlBlockSequence(YamlBlockSequence),
}
impl AnyYamlBlockContent {
    pub fn as_yaml_block_mapping(&self) -> Option<&YamlBlockMapping> {
        match &self {
            AnyYamlBlockContent::YamlBlockMapping(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_block_sequence(&self) -> Option<&YamlBlockSequence> {
        match &self {
            AnyYamlBlockContent::YamlBlockSequence(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyYamlBlockMapEntry {
    YamlBlockMapExplicitEntry(YamlBlockMapExplicitEntry),
    YamlBlockMapImplicitEntry(YamlBlockMapImplicitEntry),
}
impl AnyYamlBlockMapEntry {
    pub fn as_yaml_block_map_explicit_entry(&self) -> Option<&YamlBlockMapExplicitEntry> {
        match &self {
            AnyYamlBlockMapEntry::YamlBlockMapExplicitEntry(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_block_map_implicit_entry(&self) -> Option<&YamlBlockMapImplicitEntry> {
        match &self {
            AnyYamlBlockMapEntry::YamlBlockMapImplicitEntry(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyYamlBlockMapImplicitKey {
    YamlFlowJsonNode(YamlFlowJsonNode),
    YamlFlowYamlNode(YamlFlowYamlNode),
}
impl AnyYamlBlockMapImplicitKey {
    pub fn as_yaml_flow_json_node(&self) -> Option<&YamlFlowJsonNode> {
        match &self {
            AnyYamlBlockMapImplicitKey::YamlFlowJsonNode(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_flow_yaml_node(&self) -> Option<&YamlFlowYamlNode> {
        match &self {
            AnyYamlBlockMapImplicitKey::YamlFlowYamlNode(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyYamlBlockNode {
    YamlBlockCollection(YamlBlockCollection),
    YamlFoldedScalar(YamlFoldedScalar),
    YamlLiteralScalar(YamlLiteralScalar),
}
impl AnyYamlBlockNode {
    pub fn as_yaml_block_collection(&self) -> Option<&YamlBlockCollection> {
        match &self {
            AnyYamlBlockNode::YamlBlockCollection(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_folded_scalar(&self) -> Option<&YamlFoldedScalar> {
        match &self {
            AnyYamlBlockNode::YamlFoldedScalar(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_literal_scalar(&self) -> Option<&YamlLiteralScalar> {
        match &self {
            AnyYamlBlockNode::YamlLiteralScalar(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyYamlFlowMapEntry {
    YamlFlowMapExplicitEntry(YamlFlowMapExplicitEntry),
    YamlFlowMapImplicitEntry(YamlFlowMapImplicitEntry),
}
impl AnyYamlFlowMapEntry {
    pub fn as_yaml_flow_map_explicit_entry(&self) -> Option<&YamlFlowMapExplicitEntry> {
        match &self {
            AnyYamlFlowMapEntry::YamlFlowMapExplicitEntry(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_flow_map_implicit_entry(&self) -> Option<&YamlFlowMapImplicitEntry> {
        match &self {
            AnyYamlFlowMapEntry::YamlFlowMapImplicitEntry(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyYamlFlowMapImplicitKey {
    YamlFlowJsonNode(YamlFlowJsonNode),
    YamlFlowYamlNode(YamlFlowYamlNode),
}
impl AnyYamlFlowMapImplicitKey {
    pub fn as_yaml_flow_json_node(&self) -> Option<&YamlFlowJsonNode> {
        match &self {
            AnyYamlFlowMapImplicitKey::YamlFlowJsonNode(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_flow_yaml_node(&self) -> Option<&YamlFlowYamlNode> {
        match &self {
            AnyYamlFlowMapImplicitKey::YamlFlowYamlNode(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyYamlFlowNode {
    YamlAliasNode(YamlAliasNode),
    YamlFlowJsonNode(YamlFlowJsonNode),
    YamlFlowYamlNode(YamlFlowYamlNode),
}
impl AnyYamlFlowNode {
    pub fn as_yaml_alias_node(&self) -> Option<&YamlAliasNode> {
        match &self {
            AnyYamlFlowNode::YamlAliasNode(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_flow_json_node(&self) -> Option<&YamlFlowJsonNode> {
        match &self {
            AnyYamlFlowNode::YamlFlowJsonNode(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_flow_yaml_node(&self) -> Option<&YamlFlowYamlNode> {
        match &self {
            AnyYamlFlowNode::YamlFlowYamlNode(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyYamlFlowSequenceEntry {
    AnyYamlFlowMapEntry(AnyYamlFlowMapEntry),
    AnyYamlFlowNode(AnyYamlFlowNode),
}
impl AnyYamlFlowSequenceEntry {
    pub fn as_any_yaml_flow_map_entry(&self) -> Option<&AnyYamlFlowMapEntry> {
        match &self {
            AnyYamlFlowSequenceEntry::AnyYamlFlowMapEntry(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_any_yaml_flow_node(&self) -> Option<&AnyYamlFlowNode> {
        match &self {
            AnyYamlFlowSequenceEntry::AnyYamlFlowNode(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyYamlJsonContent {
    YamlDoubleQuotedScalar(YamlDoubleQuotedScalar),
    YamlFlowMapping(YamlFlowMapping),
    YamlFlowSequence(YamlFlowSequence),
    YamlSingleQuotedScalar(YamlSingleQuotedScalar),
}
impl AnyYamlJsonContent {
    pub fn as_yaml_double_quoted_scalar(&self) -> Option<&YamlDoubleQuotedScalar> {
        match &self {
            AnyYamlJsonContent::YamlDoubleQuotedScalar(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_flow_mapping(&self) -> Option<&YamlFlowMapping> {
        match &self {
            AnyYamlJsonContent::YamlFlowMapping(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_flow_sequence(&self) -> Option<&YamlFlowSequence> {
        match &self {
            AnyYamlJsonContent::YamlFlowSequence(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_single_quoted_scalar(&self) -> Option<&YamlSingleQuotedScalar> {
        match &self {
            AnyYamlJsonContent::YamlSingleQuotedScalar(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyYamlNode {
    AnyYamlBlockNode(AnyYamlBlockNode),
    AnyYamlFlowNode(AnyYamlFlowNode),
    YamlBogusNode(YamlBogusNode),
}
impl AnyYamlNode {
    pub fn as_any_yaml_block_node(&self) -> Option<&AnyYamlBlockNode> {
        match &self {
            AnyYamlNode::AnyYamlBlockNode(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_any_yaml_flow_node(&self) -> Option<&AnyYamlFlowNode> {
        match &self {
            AnyYamlNode::AnyYamlFlowNode(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_bogus_node(&self) -> Option<&YamlBogusNode> {
        match &self {
            AnyYamlNode::YamlBogusNode(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyYamlProperty {
    YamlAnchorProperty(YamlAnchorProperty),
    YamlTagProperty(YamlTagProperty),
}
impl AnyYamlProperty {
    pub fn as_yaml_anchor_property(&self) -> Option<&YamlAnchorProperty> {
        match &self {
            AnyYamlProperty::YamlAnchorProperty(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_tag_property(&self) -> Option<&YamlTagProperty> {
        match &self {
            AnyYamlProperty::YamlTagProperty(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum YamlIndentedBlock {
    AnyYamlNode(AnyYamlNode),
    YamlCompactMapping(YamlCompactMapping),
    YamlCompactSequence(YamlCompactSequence),
}
impl YamlIndentedBlock {
    pub fn as_any_yaml_node(&self) -> Option<&AnyYamlNode> {
        match &self {
            YamlIndentedBlock::AnyYamlNode(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_compact_mapping(&self) -> Option<&YamlCompactMapping> {
        match &self {
            YamlIndentedBlock::YamlCompactMapping(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_compact_sequence(&self) -> Option<&YamlCompactSequence> {
        match &self {
            YamlIndentedBlock::YamlCompactSequence(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for YamlAliasNode {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_ALIAS_NODE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_ALIAS_NODE
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
impl std::fmt::Debug for YamlAliasNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlAliasNode")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("YamlAliasNode").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlAliasNode> for SyntaxNode {
    fn from(n: YamlAliasNode) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlAliasNode> for SyntaxElement {
    fn from(n: YamlAliasNode) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlAnchorProperty {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_ANCHOR_PROPERTY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_ANCHOR_PROPERTY
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
impl std::fmt::Debug for YamlAnchorProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlAnchorProperty")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("YamlAnchorProperty").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlAnchorProperty> for SyntaxNode {
    fn from(n: YamlAnchorProperty) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlAnchorProperty> for SyntaxElement {
    fn from(n: YamlAnchorProperty) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlBlockCollection {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BLOCK_COLLECTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BLOCK_COLLECTION
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
impl std::fmt::Debug for YamlBlockCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlBlockCollection")
                .field("properties", &support::DebugSyntaxResult(self.properties()))
                .field("content", &support::DebugSyntaxResult(self.content()))
                .finish()
        } else {
            f.debug_struct("YamlBlockCollection").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlBlockCollection> for SyntaxNode {
    fn from(n: YamlBlockCollection) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlBlockCollection> for SyntaxElement {
    fn from(n: YamlBlockCollection) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlBlockMapExplicitEntry {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BLOCK_MAP_EXPLICIT_ENTRY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BLOCK_MAP_EXPLICIT_ENTRY
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
impl std::fmt::Debug for YamlBlockMapExplicitEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlBlockMapExplicitEntry")
                .field("key", &support::DebugSyntaxResult(self.key()))
                .field("value", &support::DebugOptionalElement(self.value()))
                .finish()
        } else {
            f.debug_struct("YamlBlockMapExplicitEntry").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlBlockMapExplicitEntry> for SyntaxNode {
    fn from(n: YamlBlockMapExplicitEntry) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlBlockMapExplicitEntry> for SyntaxElement {
    fn from(n: YamlBlockMapExplicitEntry) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlBlockMapExplicitKey {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BLOCK_MAP_EXPLICIT_KEY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BLOCK_MAP_EXPLICIT_KEY
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
impl std::fmt::Debug for YamlBlockMapExplicitKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlBlockMapExplicitKey")
                .field(
                    "question_mark_token",
                    &support::DebugSyntaxResult(self.question_mark_token()),
                )
                .field("key", &support::DebugSyntaxResult(self.key()))
                .finish()
        } else {
            f.debug_struct("YamlBlockMapExplicitKey").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlBlockMapExplicitKey> for SyntaxNode {
    fn from(n: YamlBlockMapExplicitKey) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlBlockMapExplicitKey> for SyntaxElement {
    fn from(n: YamlBlockMapExplicitKey) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlBlockMapExplicitValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BLOCK_MAP_EXPLICIT_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BLOCK_MAP_EXPLICIT_VALUE
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
impl std::fmt::Debug for YamlBlockMapExplicitValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlBlockMapExplicitValue")
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("YamlBlockMapExplicitValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlBlockMapExplicitValue> for SyntaxNode {
    fn from(n: YamlBlockMapExplicitValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlBlockMapExplicitValue> for SyntaxElement {
    fn from(n: YamlBlockMapExplicitValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlBlockMapImplicitEntry {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BLOCK_MAP_IMPLICIT_ENTRY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BLOCK_MAP_IMPLICIT_ENTRY
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
impl std::fmt::Debug for YamlBlockMapImplicitEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlBlockMapImplicitEntry")
                .field("key", &support::DebugOptionalElement(self.key()))
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("YamlBlockMapImplicitEntry").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlBlockMapImplicitEntry> for SyntaxNode {
    fn from(n: YamlBlockMapImplicitEntry) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlBlockMapImplicitEntry> for SyntaxElement {
    fn from(n: YamlBlockMapImplicitEntry) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlBlockMapImplicitValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BLOCK_MAP_IMPLICIT_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BLOCK_MAP_IMPLICIT_VALUE
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
impl std::fmt::Debug for YamlBlockMapImplicitValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlBlockMapImplicitValue")
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("YamlBlockMapImplicitValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlBlockMapImplicitValue> for SyntaxNode {
    fn from(n: YamlBlockMapImplicitValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlBlockMapImplicitValue> for SyntaxElement {
    fn from(n: YamlBlockMapImplicitValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlBlockMapping {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BLOCK_MAPPING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BLOCK_MAPPING
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
impl std::fmt::Debug for YamlBlockMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlBlockMapping")
                .field(
                    "indent_token",
                    &support::DebugSyntaxResult(self.indent_token()),
                )
                .field("entries", &self.entries())
                .field(
                    "dedent_token",
                    &support::DebugOptionalElement(self.dedent_token()),
                )
                .finish()
        } else {
            f.debug_struct("YamlBlockMapping").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlBlockMapping> for SyntaxNode {
    fn from(n: YamlBlockMapping) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlBlockMapping> for SyntaxElement {
    fn from(n: YamlBlockMapping) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlBlockSequence {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BLOCK_SEQUENCE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BLOCK_SEQUENCE
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
impl std::fmt::Debug for YamlBlockSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlBlockSequence")
                .field(
                    "indent_token",
                    &support::DebugOptionalElement(self.indent_token()),
                )
                .field("entries", &self.entries())
                .field(
                    "dedent_token",
                    &support::DebugOptionalElement(self.dedent_token()),
                )
                .finish()
        } else {
            f.debug_struct("YamlBlockSequence").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlBlockSequence> for SyntaxNode {
    fn from(n: YamlBlockSequence) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlBlockSequence> for SyntaxElement {
    fn from(n: YamlBlockSequence) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlBlockSequenceEntry {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BLOCK_SEQUENCE_ENTRY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BLOCK_SEQUENCE_ENTRY
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
impl std::fmt::Debug for YamlBlockSequenceEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlBlockSequenceEntry")
                .field(
                    "minus_token",
                    &support::DebugSyntaxResult(self.minus_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("YamlBlockSequenceEntry").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlBlockSequenceEntry> for SyntaxNode {
    fn from(n: YamlBlockSequenceEntry) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlBlockSequenceEntry> for SyntaxElement {
    fn from(n: YamlBlockSequenceEntry) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlCompactMapping {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_COMPACT_MAPPING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_COMPACT_MAPPING
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
impl std::fmt::Debug for YamlCompactMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlCompactMapping")
                .field("entries", &self.entries())
                .finish()
        } else {
            f.debug_struct("YamlCompactMapping").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlCompactMapping> for SyntaxNode {
    fn from(n: YamlCompactMapping) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlCompactMapping> for SyntaxElement {
    fn from(n: YamlCompactMapping) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlCompactSequence {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_COMPACT_SEQUENCE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_COMPACT_SEQUENCE
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
impl std::fmt::Debug for YamlCompactSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlCompactSequence")
                .field("entries", &self.entries())
                .finish()
        } else {
            f.debug_struct("YamlCompactSequence").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlCompactSequence> for SyntaxNode {
    fn from(n: YamlCompactSequence) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlCompactSequence> for SyntaxElement {
    fn from(n: YamlCompactSequence) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_DIRECTIVE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_DIRECTIVE
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
impl std::fmt::Debug for YamlDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlDirective")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("YamlDirective").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlDirective> for SyntaxNode {
    fn from(n: YamlDirective) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlDirective> for SyntaxElement {
    fn from(n: YamlDirective) -> SyntaxElement {
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
                    "bom_token",
                    &support::DebugOptionalElement(self.bom_token()),
                )
                .field("directives", &self.directives())
                .field(
                    "dashdashdash_token",
                    &support::DebugOptionalElement(self.dashdashdash_token()),
                )
                .field("node", &support::DebugSyntaxResult(self.node()))
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
impl AstNode for YamlDoubleQuotedScalar {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_DOUBLE_QUOTED_SCALAR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_DOUBLE_QUOTED_SCALAR
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
impl std::fmt::Debug for YamlDoubleQuotedScalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlDoubleQuotedScalar")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("YamlDoubleQuotedScalar").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlDoubleQuotedScalar> for SyntaxNode {
    fn from(n: YamlDoubleQuotedScalar) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlDoubleQuotedScalar> for SyntaxElement {
    fn from(n: YamlDoubleQuotedScalar) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlFlowJsonNode {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_FLOW_JSON_NODE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_FLOW_JSON_NODE
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
impl std::fmt::Debug for YamlFlowJsonNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlFlowJsonNode")
                .field("properties", &support::DebugSyntaxResult(self.properties()))
                .field("content", &support::DebugOptionalElement(self.content()))
                .finish()
        } else {
            f.debug_struct("YamlFlowJsonNode").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlFlowJsonNode> for SyntaxNode {
    fn from(n: YamlFlowJsonNode) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlFlowJsonNode> for SyntaxElement {
    fn from(n: YamlFlowJsonNode) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlFlowMapExplicitEntry {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_FLOW_MAP_EXPLICIT_ENTRY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_FLOW_MAP_EXPLICIT_ENTRY
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
impl std::fmt::Debug for YamlFlowMapExplicitEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlFlowMapExplicitEntry")
                .field(
                    "question_mark_token",
                    &support::DebugSyntaxResult(self.question_mark_token()),
                )
                .field("entry", &support::DebugOptionalElement(self.entry()))
                .finish()
        } else {
            f.debug_struct("YamlFlowMapExplicitEntry").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlFlowMapExplicitEntry> for SyntaxNode {
    fn from(n: YamlFlowMapExplicitEntry) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlFlowMapExplicitEntry> for SyntaxElement {
    fn from(n: YamlFlowMapExplicitEntry) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlFlowMapImplicitEntry {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_FLOW_MAP_IMPLICIT_ENTRY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_FLOW_MAP_IMPLICIT_ENTRY
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
impl std::fmt::Debug for YamlFlowMapImplicitEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlFlowMapImplicitEntry")
                .field("key", &support::DebugOptionalElement(self.key()))
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("YamlFlowMapImplicitEntry").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlFlowMapImplicitEntry> for SyntaxNode {
    fn from(n: YamlFlowMapImplicitEntry) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlFlowMapImplicitEntry> for SyntaxElement {
    fn from(n: YamlFlowMapImplicitEntry) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlFlowMapping {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_FLOW_MAPPING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_FLOW_MAPPING
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
impl std::fmt::Debug for YamlFlowMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlFlowMapping")
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("entries", &self.entries())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("YamlFlowMapping").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlFlowMapping> for SyntaxNode {
    fn from(n: YamlFlowMapping) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlFlowMapping> for SyntaxElement {
    fn from(n: YamlFlowMapping) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlFlowSequence {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_FLOW_SEQUENCE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_FLOW_SEQUENCE
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
impl std::fmt::Debug for YamlFlowSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlFlowSequence")
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("entries", &self.entries())
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .finish()
        } else {
            f.debug_struct("YamlFlowSequence").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlFlowSequence> for SyntaxNode {
    fn from(n: YamlFlowSequence) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlFlowSequence> for SyntaxElement {
    fn from(n: YamlFlowSequence) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlFlowYamlNode {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_FLOW_YAML_NODE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_FLOW_YAML_NODE
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
impl std::fmt::Debug for YamlFlowYamlNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlFlowYamlNode")
                .field("properties", &support::DebugSyntaxResult(self.properties()))
                .field("content", &support::DebugOptionalElement(self.content()))
                .finish()
        } else {
            f.debug_struct("YamlFlowYamlNode").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlFlowYamlNode> for SyntaxNode {
    fn from(n: YamlFlowYamlNode) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlFlowYamlNode> for SyntaxElement {
    fn from(n: YamlFlowYamlNode) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlFoldedScalar {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_FOLDED_SCALAR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_FOLDED_SCALAR
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
impl std::fmt::Debug for YamlFoldedScalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlFoldedScalar")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("YamlFoldedScalar").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlFoldedScalar> for SyntaxNode {
    fn from(n: YamlFoldedScalar) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlFoldedScalar> for SyntaxElement {
    fn from(n: YamlFoldedScalar) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlLiteralScalar {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_LITERAL_SCALAR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_LITERAL_SCALAR
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
impl std::fmt::Debug for YamlLiteralScalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlLiteralScalar")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("YamlLiteralScalar").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlLiteralScalar> for SyntaxNode {
    fn from(n: YamlLiteralScalar) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlLiteralScalar> for SyntaxElement {
    fn from(n: YamlLiteralScalar) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlPlainScalar {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_PLAIN_SCALAR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_PLAIN_SCALAR
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
impl std::fmt::Debug for YamlPlainScalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlPlainScalar")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("YamlPlainScalar").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlPlainScalar> for SyntaxNode {
    fn from(n: YamlPlainScalar) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlPlainScalar> for SyntaxElement {
    fn from(n: YamlPlainScalar) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlPropertyList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_PROPERTY_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_PROPERTY_LIST
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
impl std::fmt::Debug for YamlPropertyList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlPropertyList")
                .field(
                    "any_yaml_property",
                    &support::DebugSyntaxResult(self.any_yaml_property()),
                )
                .finish()
        } else {
            f.debug_struct("YamlPropertyList").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlPropertyList> for SyntaxNode {
    fn from(n: YamlPropertyList) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlPropertyList> for SyntaxElement {
    fn from(n: YamlPropertyList) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlSingleQuotedScalar {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_SINGLE_QUOTED_SCALAR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_SINGLE_QUOTED_SCALAR
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
impl std::fmt::Debug for YamlSingleQuotedScalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlSingleQuotedScalar")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("YamlSingleQuotedScalar").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlSingleQuotedScalar> for SyntaxNode {
    fn from(n: YamlSingleQuotedScalar) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlSingleQuotedScalar> for SyntaxElement {
    fn from(n: YamlSingleQuotedScalar) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlStream {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_STREAM as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_STREAM
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
impl std::fmt::Debug for YamlStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlStream")
                .field("documents", &self.documents())
                .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
                .finish()
        } else {
            f.debug_struct("YamlStream").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlStream> for SyntaxNode {
    fn from(n: YamlStream) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlStream> for SyntaxElement {
    fn from(n: YamlStream) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlTagProperty {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_TAG_PROPERTY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_TAG_PROPERTY
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
impl std::fmt::Debug for YamlTagProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("YamlTagProperty")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("YamlTagProperty").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<YamlTagProperty> for SyntaxNode {
    fn from(n: YamlTagProperty) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlTagProperty> for SyntaxElement {
    fn from(n: YamlTagProperty) -> SyntaxElement {
        n.syntax.into()
    }
}
impl From<YamlBlockMapping> for AnyYamlBlockContent {
    fn from(node: YamlBlockMapping) -> AnyYamlBlockContent {
        AnyYamlBlockContent::YamlBlockMapping(node)
    }
}
impl From<YamlBlockSequence> for AnyYamlBlockContent {
    fn from(node: YamlBlockSequence) -> AnyYamlBlockContent {
        AnyYamlBlockContent::YamlBlockSequence(node)
    }
}
impl AstNode for AnyYamlBlockContent {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        YamlBlockMapping::KIND_SET.union(YamlBlockSequence::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, YAML_BLOCK_MAPPING | YAML_BLOCK_SEQUENCE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            YAML_BLOCK_MAPPING => {
                AnyYamlBlockContent::YamlBlockMapping(YamlBlockMapping { syntax })
            }
            YAML_BLOCK_SEQUENCE => {
                AnyYamlBlockContent::YamlBlockSequence(YamlBlockSequence { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyYamlBlockContent::YamlBlockMapping(it) => &it.syntax,
            AnyYamlBlockContent::YamlBlockSequence(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyYamlBlockContent::YamlBlockMapping(it) => it.syntax,
            AnyYamlBlockContent::YamlBlockSequence(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyYamlBlockContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyYamlBlockContent::YamlBlockMapping(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlBlockContent::YamlBlockSequence(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyYamlBlockContent> for SyntaxNode {
    fn from(n: AnyYamlBlockContent) -> SyntaxNode {
        match n {
            AnyYamlBlockContent::YamlBlockMapping(it) => it.into(),
            AnyYamlBlockContent::YamlBlockSequence(it) => it.into(),
        }
    }
}
impl From<AnyYamlBlockContent> for SyntaxElement {
    fn from(n: AnyYamlBlockContent) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<YamlBlockMapExplicitEntry> for AnyYamlBlockMapEntry {
    fn from(node: YamlBlockMapExplicitEntry) -> AnyYamlBlockMapEntry {
        AnyYamlBlockMapEntry::YamlBlockMapExplicitEntry(node)
    }
}
impl From<YamlBlockMapImplicitEntry> for AnyYamlBlockMapEntry {
    fn from(node: YamlBlockMapImplicitEntry) -> AnyYamlBlockMapEntry {
        AnyYamlBlockMapEntry::YamlBlockMapImplicitEntry(node)
    }
}
impl AstNode for AnyYamlBlockMapEntry {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        YamlBlockMapExplicitEntry::KIND_SET.union(YamlBlockMapImplicitEntry::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            YAML_BLOCK_MAP_EXPLICIT_ENTRY | YAML_BLOCK_MAP_IMPLICIT_ENTRY
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            YAML_BLOCK_MAP_EXPLICIT_ENTRY => {
                AnyYamlBlockMapEntry::YamlBlockMapExplicitEntry(YamlBlockMapExplicitEntry {
                    syntax,
                })
            }
            YAML_BLOCK_MAP_IMPLICIT_ENTRY => {
                AnyYamlBlockMapEntry::YamlBlockMapImplicitEntry(YamlBlockMapImplicitEntry {
                    syntax,
                })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyYamlBlockMapEntry::YamlBlockMapExplicitEntry(it) => &it.syntax,
            AnyYamlBlockMapEntry::YamlBlockMapImplicitEntry(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyYamlBlockMapEntry::YamlBlockMapExplicitEntry(it) => it.syntax,
            AnyYamlBlockMapEntry::YamlBlockMapImplicitEntry(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyYamlBlockMapEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyYamlBlockMapEntry::YamlBlockMapExplicitEntry(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlBlockMapEntry::YamlBlockMapImplicitEntry(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyYamlBlockMapEntry> for SyntaxNode {
    fn from(n: AnyYamlBlockMapEntry) -> SyntaxNode {
        match n {
            AnyYamlBlockMapEntry::YamlBlockMapExplicitEntry(it) => it.into(),
            AnyYamlBlockMapEntry::YamlBlockMapImplicitEntry(it) => it.into(),
        }
    }
}
impl From<AnyYamlBlockMapEntry> for SyntaxElement {
    fn from(n: AnyYamlBlockMapEntry) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<YamlFlowJsonNode> for AnyYamlBlockMapImplicitKey {
    fn from(node: YamlFlowJsonNode) -> AnyYamlBlockMapImplicitKey {
        AnyYamlBlockMapImplicitKey::YamlFlowJsonNode(node)
    }
}
impl From<YamlFlowYamlNode> for AnyYamlBlockMapImplicitKey {
    fn from(node: YamlFlowYamlNode) -> AnyYamlBlockMapImplicitKey {
        AnyYamlBlockMapImplicitKey::YamlFlowYamlNode(node)
    }
}
impl AstNode for AnyYamlBlockMapImplicitKey {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        YamlFlowJsonNode::KIND_SET.union(YamlFlowYamlNode::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, YAML_FLOW_JSON_NODE | YAML_FLOW_YAML_NODE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            YAML_FLOW_JSON_NODE => {
                AnyYamlBlockMapImplicitKey::YamlFlowJsonNode(YamlFlowJsonNode { syntax })
            }
            YAML_FLOW_YAML_NODE => {
                AnyYamlBlockMapImplicitKey::YamlFlowYamlNode(YamlFlowYamlNode { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyYamlBlockMapImplicitKey::YamlFlowJsonNode(it) => &it.syntax,
            AnyYamlBlockMapImplicitKey::YamlFlowYamlNode(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyYamlBlockMapImplicitKey::YamlFlowJsonNode(it) => it.syntax,
            AnyYamlBlockMapImplicitKey::YamlFlowYamlNode(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyYamlBlockMapImplicitKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyYamlBlockMapImplicitKey::YamlFlowJsonNode(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlBlockMapImplicitKey::YamlFlowYamlNode(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyYamlBlockMapImplicitKey> for SyntaxNode {
    fn from(n: AnyYamlBlockMapImplicitKey) -> SyntaxNode {
        match n {
            AnyYamlBlockMapImplicitKey::YamlFlowJsonNode(it) => it.into(),
            AnyYamlBlockMapImplicitKey::YamlFlowYamlNode(it) => it.into(),
        }
    }
}
impl From<AnyYamlBlockMapImplicitKey> for SyntaxElement {
    fn from(n: AnyYamlBlockMapImplicitKey) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<YamlBlockCollection> for AnyYamlBlockNode {
    fn from(node: YamlBlockCollection) -> AnyYamlBlockNode {
        AnyYamlBlockNode::YamlBlockCollection(node)
    }
}
impl From<YamlFoldedScalar> for AnyYamlBlockNode {
    fn from(node: YamlFoldedScalar) -> AnyYamlBlockNode {
        AnyYamlBlockNode::YamlFoldedScalar(node)
    }
}
impl From<YamlLiteralScalar> for AnyYamlBlockNode {
    fn from(node: YamlLiteralScalar) -> AnyYamlBlockNode {
        AnyYamlBlockNode::YamlLiteralScalar(node)
    }
}
impl AstNode for AnyYamlBlockNode {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = YamlBlockCollection::KIND_SET
        .union(YamlFoldedScalar::KIND_SET)
        .union(YamlLiteralScalar::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            YAML_BLOCK_COLLECTION | YAML_FOLDED_SCALAR | YAML_LITERAL_SCALAR
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            YAML_BLOCK_COLLECTION => {
                AnyYamlBlockNode::YamlBlockCollection(YamlBlockCollection { syntax })
            }
            YAML_FOLDED_SCALAR => AnyYamlBlockNode::YamlFoldedScalar(YamlFoldedScalar { syntax }),
            YAML_LITERAL_SCALAR => {
                AnyYamlBlockNode::YamlLiteralScalar(YamlLiteralScalar { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyYamlBlockNode::YamlBlockCollection(it) => &it.syntax,
            AnyYamlBlockNode::YamlFoldedScalar(it) => &it.syntax,
            AnyYamlBlockNode::YamlLiteralScalar(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyYamlBlockNode::YamlBlockCollection(it) => it.syntax,
            AnyYamlBlockNode::YamlFoldedScalar(it) => it.syntax,
            AnyYamlBlockNode::YamlLiteralScalar(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyYamlBlockNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyYamlBlockNode::YamlBlockCollection(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlBlockNode::YamlFoldedScalar(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlBlockNode::YamlLiteralScalar(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyYamlBlockNode> for SyntaxNode {
    fn from(n: AnyYamlBlockNode) -> SyntaxNode {
        match n {
            AnyYamlBlockNode::YamlBlockCollection(it) => it.into(),
            AnyYamlBlockNode::YamlFoldedScalar(it) => it.into(),
            AnyYamlBlockNode::YamlLiteralScalar(it) => it.into(),
        }
    }
}
impl From<AnyYamlBlockNode> for SyntaxElement {
    fn from(n: AnyYamlBlockNode) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<YamlFlowMapExplicitEntry> for AnyYamlFlowMapEntry {
    fn from(node: YamlFlowMapExplicitEntry) -> AnyYamlFlowMapEntry {
        AnyYamlFlowMapEntry::YamlFlowMapExplicitEntry(node)
    }
}
impl From<YamlFlowMapImplicitEntry> for AnyYamlFlowMapEntry {
    fn from(node: YamlFlowMapImplicitEntry) -> AnyYamlFlowMapEntry {
        AnyYamlFlowMapEntry::YamlFlowMapImplicitEntry(node)
    }
}
impl AstNode for AnyYamlFlowMapEntry {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        YamlFlowMapExplicitEntry::KIND_SET.union(YamlFlowMapImplicitEntry::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            YAML_FLOW_MAP_EXPLICIT_ENTRY | YAML_FLOW_MAP_IMPLICIT_ENTRY
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            YAML_FLOW_MAP_EXPLICIT_ENTRY => {
                AnyYamlFlowMapEntry::YamlFlowMapExplicitEntry(YamlFlowMapExplicitEntry { syntax })
            }
            YAML_FLOW_MAP_IMPLICIT_ENTRY => {
                AnyYamlFlowMapEntry::YamlFlowMapImplicitEntry(YamlFlowMapImplicitEntry { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyYamlFlowMapEntry::YamlFlowMapExplicitEntry(it) => &it.syntax,
            AnyYamlFlowMapEntry::YamlFlowMapImplicitEntry(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyYamlFlowMapEntry::YamlFlowMapExplicitEntry(it) => it.syntax,
            AnyYamlFlowMapEntry::YamlFlowMapImplicitEntry(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyYamlFlowMapEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyYamlFlowMapEntry::YamlFlowMapExplicitEntry(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlFlowMapEntry::YamlFlowMapImplicitEntry(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyYamlFlowMapEntry> for SyntaxNode {
    fn from(n: AnyYamlFlowMapEntry) -> SyntaxNode {
        match n {
            AnyYamlFlowMapEntry::YamlFlowMapExplicitEntry(it) => it.into(),
            AnyYamlFlowMapEntry::YamlFlowMapImplicitEntry(it) => it.into(),
        }
    }
}
impl From<AnyYamlFlowMapEntry> for SyntaxElement {
    fn from(n: AnyYamlFlowMapEntry) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<YamlFlowJsonNode> for AnyYamlFlowMapImplicitKey {
    fn from(node: YamlFlowJsonNode) -> AnyYamlFlowMapImplicitKey {
        AnyYamlFlowMapImplicitKey::YamlFlowJsonNode(node)
    }
}
impl From<YamlFlowYamlNode> for AnyYamlFlowMapImplicitKey {
    fn from(node: YamlFlowYamlNode) -> AnyYamlFlowMapImplicitKey {
        AnyYamlFlowMapImplicitKey::YamlFlowYamlNode(node)
    }
}
impl AstNode for AnyYamlFlowMapImplicitKey {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        YamlFlowJsonNode::KIND_SET.union(YamlFlowYamlNode::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, YAML_FLOW_JSON_NODE | YAML_FLOW_YAML_NODE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            YAML_FLOW_JSON_NODE => {
                AnyYamlFlowMapImplicitKey::YamlFlowJsonNode(YamlFlowJsonNode { syntax })
            }
            YAML_FLOW_YAML_NODE => {
                AnyYamlFlowMapImplicitKey::YamlFlowYamlNode(YamlFlowYamlNode { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyYamlFlowMapImplicitKey::YamlFlowJsonNode(it) => &it.syntax,
            AnyYamlFlowMapImplicitKey::YamlFlowYamlNode(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyYamlFlowMapImplicitKey::YamlFlowJsonNode(it) => it.syntax,
            AnyYamlFlowMapImplicitKey::YamlFlowYamlNode(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyYamlFlowMapImplicitKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyYamlFlowMapImplicitKey::YamlFlowJsonNode(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlFlowMapImplicitKey::YamlFlowYamlNode(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyYamlFlowMapImplicitKey> for SyntaxNode {
    fn from(n: AnyYamlFlowMapImplicitKey) -> SyntaxNode {
        match n {
            AnyYamlFlowMapImplicitKey::YamlFlowJsonNode(it) => it.into(),
            AnyYamlFlowMapImplicitKey::YamlFlowYamlNode(it) => it.into(),
        }
    }
}
impl From<AnyYamlFlowMapImplicitKey> for SyntaxElement {
    fn from(n: AnyYamlFlowMapImplicitKey) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<YamlAliasNode> for AnyYamlFlowNode {
    fn from(node: YamlAliasNode) -> AnyYamlFlowNode {
        AnyYamlFlowNode::YamlAliasNode(node)
    }
}
impl From<YamlFlowJsonNode> for AnyYamlFlowNode {
    fn from(node: YamlFlowJsonNode) -> AnyYamlFlowNode {
        AnyYamlFlowNode::YamlFlowJsonNode(node)
    }
}
impl From<YamlFlowYamlNode> for AnyYamlFlowNode {
    fn from(node: YamlFlowYamlNode) -> AnyYamlFlowNode {
        AnyYamlFlowNode::YamlFlowYamlNode(node)
    }
}
impl AstNode for AnyYamlFlowNode {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = YamlAliasNode::KIND_SET
        .union(YamlFlowJsonNode::KIND_SET)
        .union(YamlFlowYamlNode::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            YAML_ALIAS_NODE | YAML_FLOW_JSON_NODE | YAML_FLOW_YAML_NODE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            YAML_ALIAS_NODE => AnyYamlFlowNode::YamlAliasNode(YamlAliasNode { syntax }),
            YAML_FLOW_JSON_NODE => AnyYamlFlowNode::YamlFlowJsonNode(YamlFlowJsonNode { syntax }),
            YAML_FLOW_YAML_NODE => AnyYamlFlowNode::YamlFlowYamlNode(YamlFlowYamlNode { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyYamlFlowNode::YamlAliasNode(it) => &it.syntax,
            AnyYamlFlowNode::YamlFlowJsonNode(it) => &it.syntax,
            AnyYamlFlowNode::YamlFlowYamlNode(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyYamlFlowNode::YamlAliasNode(it) => it.syntax,
            AnyYamlFlowNode::YamlFlowJsonNode(it) => it.syntax,
            AnyYamlFlowNode::YamlFlowYamlNode(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyYamlFlowNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyYamlFlowNode::YamlAliasNode(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlFlowNode::YamlFlowJsonNode(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlFlowNode::YamlFlowYamlNode(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyYamlFlowNode> for SyntaxNode {
    fn from(n: AnyYamlFlowNode) -> SyntaxNode {
        match n {
            AnyYamlFlowNode::YamlAliasNode(it) => it.into(),
            AnyYamlFlowNode::YamlFlowJsonNode(it) => it.into(),
            AnyYamlFlowNode::YamlFlowYamlNode(it) => it.into(),
        }
    }
}
impl From<AnyYamlFlowNode> for SyntaxElement {
    fn from(n: AnyYamlFlowNode) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl AstNode for AnyYamlFlowSequenceEntry {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyYamlFlowMapEntry::KIND_SET.union(AnyYamlFlowNode::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            k if AnyYamlFlowMapEntry::can_cast(k) => true,
            k if AnyYamlFlowNode::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let syntax = match AnyYamlFlowMapEntry::try_cast(syntax) {
            Ok(any_yaml_flow_map_entry) => {
                return Some(AnyYamlFlowSequenceEntry::AnyYamlFlowMapEntry(
                    any_yaml_flow_map_entry,
                ));
            }
            Err(syntax) => syntax,
        };
        if let Some(any_yaml_flow_node) = AnyYamlFlowNode::cast(syntax) {
            return Some(AnyYamlFlowSequenceEntry::AnyYamlFlowNode(
                any_yaml_flow_node,
            ));
        }
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyYamlFlowSequenceEntry::AnyYamlFlowMapEntry(it) => it.syntax(),
            AnyYamlFlowSequenceEntry::AnyYamlFlowNode(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyYamlFlowSequenceEntry::AnyYamlFlowMapEntry(it) => it.into_syntax(),
            AnyYamlFlowSequenceEntry::AnyYamlFlowNode(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyYamlFlowSequenceEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyYamlFlowSequenceEntry::AnyYamlFlowMapEntry(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlFlowSequenceEntry::AnyYamlFlowNode(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyYamlFlowSequenceEntry> for SyntaxNode {
    fn from(n: AnyYamlFlowSequenceEntry) -> SyntaxNode {
        match n {
            AnyYamlFlowSequenceEntry::AnyYamlFlowMapEntry(it) => it.into(),
            AnyYamlFlowSequenceEntry::AnyYamlFlowNode(it) => it.into(),
        }
    }
}
impl From<AnyYamlFlowSequenceEntry> for SyntaxElement {
    fn from(n: AnyYamlFlowSequenceEntry) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<YamlDoubleQuotedScalar> for AnyYamlJsonContent {
    fn from(node: YamlDoubleQuotedScalar) -> AnyYamlJsonContent {
        AnyYamlJsonContent::YamlDoubleQuotedScalar(node)
    }
}
impl From<YamlFlowMapping> for AnyYamlJsonContent {
    fn from(node: YamlFlowMapping) -> AnyYamlJsonContent {
        AnyYamlJsonContent::YamlFlowMapping(node)
    }
}
impl From<YamlFlowSequence> for AnyYamlJsonContent {
    fn from(node: YamlFlowSequence) -> AnyYamlJsonContent {
        AnyYamlJsonContent::YamlFlowSequence(node)
    }
}
impl From<YamlSingleQuotedScalar> for AnyYamlJsonContent {
    fn from(node: YamlSingleQuotedScalar) -> AnyYamlJsonContent {
        AnyYamlJsonContent::YamlSingleQuotedScalar(node)
    }
}
impl AstNode for AnyYamlJsonContent {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = YamlDoubleQuotedScalar::KIND_SET
        .union(YamlFlowMapping::KIND_SET)
        .union(YamlFlowSequence::KIND_SET)
        .union(YamlSingleQuotedScalar::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            YAML_DOUBLE_QUOTED_SCALAR
                | YAML_FLOW_MAPPING
                | YAML_FLOW_SEQUENCE
                | YAML_SINGLE_QUOTED_SCALAR
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            YAML_DOUBLE_QUOTED_SCALAR => {
                AnyYamlJsonContent::YamlDoubleQuotedScalar(YamlDoubleQuotedScalar { syntax })
            }
            YAML_FLOW_MAPPING => AnyYamlJsonContent::YamlFlowMapping(YamlFlowMapping { syntax }),
            YAML_FLOW_SEQUENCE => AnyYamlJsonContent::YamlFlowSequence(YamlFlowSequence { syntax }),
            YAML_SINGLE_QUOTED_SCALAR => {
                AnyYamlJsonContent::YamlSingleQuotedScalar(YamlSingleQuotedScalar { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyYamlJsonContent::YamlDoubleQuotedScalar(it) => &it.syntax,
            AnyYamlJsonContent::YamlFlowMapping(it) => &it.syntax,
            AnyYamlJsonContent::YamlFlowSequence(it) => &it.syntax,
            AnyYamlJsonContent::YamlSingleQuotedScalar(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyYamlJsonContent::YamlDoubleQuotedScalar(it) => it.syntax,
            AnyYamlJsonContent::YamlFlowMapping(it) => it.syntax,
            AnyYamlJsonContent::YamlFlowSequence(it) => it.syntax,
            AnyYamlJsonContent::YamlSingleQuotedScalar(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyYamlJsonContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyYamlJsonContent::YamlDoubleQuotedScalar(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlJsonContent::YamlFlowMapping(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlJsonContent::YamlFlowSequence(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlJsonContent::YamlSingleQuotedScalar(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyYamlJsonContent> for SyntaxNode {
    fn from(n: AnyYamlJsonContent) -> SyntaxNode {
        match n {
            AnyYamlJsonContent::YamlDoubleQuotedScalar(it) => it.into(),
            AnyYamlJsonContent::YamlFlowMapping(it) => it.into(),
            AnyYamlJsonContent::YamlFlowSequence(it) => it.into(),
            AnyYamlJsonContent::YamlSingleQuotedScalar(it) => it.into(),
        }
    }
}
impl From<AnyYamlJsonContent> for SyntaxElement {
    fn from(n: AnyYamlJsonContent) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<YamlBogusNode> for AnyYamlNode {
    fn from(node: YamlBogusNode) -> AnyYamlNode {
        AnyYamlNode::YamlBogusNode(node)
    }
}
impl AstNode for AnyYamlNode {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyYamlBlockNode::KIND_SET
        .union(AnyYamlFlowNode::KIND_SET)
        .union(YamlBogusNode::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            YAML_BOGUS_NODE => true,
            k if AnyYamlBlockNode::can_cast(k) => true,
            k if AnyYamlFlowNode::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            YAML_BOGUS_NODE => AnyYamlNode::YamlBogusNode(YamlBogusNode { syntax }),
            _ => {
                let syntax = match AnyYamlBlockNode::try_cast(syntax) {
                    Ok(any_yaml_block_node) => {
                        return Some(AnyYamlNode::AnyYamlBlockNode(any_yaml_block_node));
                    }
                    Err(syntax) => syntax,
                };
                if let Some(any_yaml_flow_node) = AnyYamlFlowNode::cast(syntax) {
                    return Some(AnyYamlNode::AnyYamlFlowNode(any_yaml_flow_node));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyYamlNode::YamlBogusNode(it) => &it.syntax,
            AnyYamlNode::AnyYamlBlockNode(it) => it.syntax(),
            AnyYamlNode::AnyYamlFlowNode(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyYamlNode::YamlBogusNode(it) => it.syntax,
            AnyYamlNode::AnyYamlBlockNode(it) => it.into_syntax(),
            AnyYamlNode::AnyYamlFlowNode(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyYamlNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyYamlNode::AnyYamlBlockNode(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlNode::AnyYamlFlowNode(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlNode::YamlBogusNode(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyYamlNode> for SyntaxNode {
    fn from(n: AnyYamlNode) -> SyntaxNode {
        match n {
            AnyYamlNode::AnyYamlBlockNode(it) => it.into(),
            AnyYamlNode::AnyYamlFlowNode(it) => it.into(),
            AnyYamlNode::YamlBogusNode(it) => it.into(),
        }
    }
}
impl From<AnyYamlNode> for SyntaxElement {
    fn from(n: AnyYamlNode) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<YamlAnchorProperty> for AnyYamlProperty {
    fn from(node: YamlAnchorProperty) -> AnyYamlProperty {
        AnyYamlProperty::YamlAnchorProperty(node)
    }
}
impl From<YamlTagProperty> for AnyYamlProperty {
    fn from(node: YamlTagProperty) -> AnyYamlProperty {
        AnyYamlProperty::YamlTagProperty(node)
    }
}
impl AstNode for AnyYamlProperty {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        YamlAnchorProperty::KIND_SET.union(YamlTagProperty::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, YAML_ANCHOR_PROPERTY | YAML_TAG_PROPERTY)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            YAML_ANCHOR_PROPERTY => {
                AnyYamlProperty::YamlAnchorProperty(YamlAnchorProperty { syntax })
            }
            YAML_TAG_PROPERTY => AnyYamlProperty::YamlTagProperty(YamlTagProperty { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyYamlProperty::YamlAnchorProperty(it) => &it.syntax,
            AnyYamlProperty::YamlTagProperty(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyYamlProperty::YamlAnchorProperty(it) => it.syntax,
            AnyYamlProperty::YamlTagProperty(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyYamlProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyYamlProperty::YamlAnchorProperty(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlProperty::YamlTagProperty(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyYamlProperty> for SyntaxNode {
    fn from(n: AnyYamlProperty) -> SyntaxNode {
        match n {
            AnyYamlProperty::YamlAnchorProperty(it) => it.into(),
            AnyYamlProperty::YamlTagProperty(it) => it.into(),
        }
    }
}
impl From<AnyYamlProperty> for SyntaxElement {
    fn from(n: AnyYamlProperty) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<YamlCompactMapping> for YamlIndentedBlock {
    fn from(node: YamlCompactMapping) -> YamlIndentedBlock {
        YamlIndentedBlock::YamlCompactMapping(node)
    }
}
impl From<YamlCompactSequence> for YamlIndentedBlock {
    fn from(node: YamlCompactSequence) -> YamlIndentedBlock {
        YamlIndentedBlock::YamlCompactSequence(node)
    }
}
impl AstNode for YamlIndentedBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyYamlNode::KIND_SET
        .union(YamlCompactMapping::KIND_SET)
        .union(YamlCompactSequence::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            YAML_COMPACT_MAPPING | YAML_COMPACT_SEQUENCE => true,
            k if AnyYamlNode::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            YAML_COMPACT_MAPPING => {
                YamlIndentedBlock::YamlCompactMapping(YamlCompactMapping { syntax })
            }
            YAML_COMPACT_SEQUENCE => {
                YamlIndentedBlock::YamlCompactSequence(YamlCompactSequence { syntax })
            }
            _ => {
                if let Some(any_yaml_node) = AnyYamlNode::cast(syntax) {
                    return Some(YamlIndentedBlock::AnyYamlNode(any_yaml_node));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            YamlIndentedBlock::YamlCompactMapping(it) => &it.syntax,
            YamlIndentedBlock::YamlCompactSequence(it) => &it.syntax,
            YamlIndentedBlock::AnyYamlNode(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            YamlIndentedBlock::YamlCompactMapping(it) => it.syntax,
            YamlIndentedBlock::YamlCompactSequence(it) => it.syntax,
            YamlIndentedBlock::AnyYamlNode(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for YamlIndentedBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YamlIndentedBlock::AnyYamlNode(it) => std::fmt::Debug::fmt(it, f),
            YamlIndentedBlock::YamlCompactMapping(it) => std::fmt::Debug::fmt(it, f),
            YamlIndentedBlock::YamlCompactSequence(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<YamlIndentedBlock> for SyntaxNode {
    fn from(n: YamlIndentedBlock) -> SyntaxNode {
        match n {
            YamlIndentedBlock::AnyYamlNode(it) => it.into(),
            YamlIndentedBlock::YamlCompactMapping(it) => it.into(),
            YamlIndentedBlock::YamlCompactSequence(it) => it.into(),
        }
    }
}
impl From<YamlIndentedBlock> for SyntaxElement {
    fn from(n: YamlIndentedBlock) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyYamlBlockContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyYamlBlockMapEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyYamlBlockMapImplicitKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyYamlBlockNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyYamlFlowMapEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyYamlFlowMapImplicitKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyYamlFlowNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyYamlFlowSequenceEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyYamlJsonContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyYamlNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyYamlProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlIndentedBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlAliasNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlAnchorProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlBlockCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlBlockMapExplicitEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlBlockMapExplicitKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlBlockMapExplicitValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlBlockMapImplicitEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlBlockMapImplicitValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlBlockMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlBlockSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlBlockSequenceEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlCompactMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlCompactSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlDoubleQuotedScalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlFlowJsonNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlFlowMapExplicitEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlFlowMapImplicitEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlFlowMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlFlowSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlFlowYamlNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlFoldedScalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlLiteralScalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlPlainScalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlPropertyList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlSingleQuotedScalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlTagProperty {
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
pub struct YamlBogusNode {
    syntax: SyntaxNode,
}
impl YamlBogusNode {
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
impl AstNode for YamlBogusNode {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BOGUS_NODE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BOGUS_NODE
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
impl std::fmt::Debug for YamlBogusNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("YamlBogusNode")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<YamlBogusNode> for SyntaxNode {
    fn from(n: YamlBogusNode) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlBogusNode> for SyntaxElement {
    fn from(n: YamlBogusNode) -> SyntaxElement {
        n.syntax.into()
    }
}
biome_rowan::declare_node_union! { pub AnyYamlBogusNode = YamlBogus | YamlBogusNode }
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct YamlBlockMapEntryList {
    syntax_list: SyntaxList,
}
impl YamlBlockMapEntryList {
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
impl AstNode for YamlBlockMapEntryList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BLOCK_MAP_ENTRY_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BLOCK_MAP_ENTRY_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<YamlBlockMapEntryList> {
        if Self::can_cast(syntax.kind()) {
            Some(YamlBlockMapEntryList {
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
impl Serialize for YamlBlockMapEntryList {
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
impl AstNodeList for YamlBlockMapEntryList {
    type Language = Language;
    type Node = AnyYamlBlockMapEntry;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for YamlBlockMapEntryList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("YamlBlockMapEntryList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &YamlBlockMapEntryList {
    type Item = AnyYamlBlockMapEntry;
    type IntoIter = AstNodeListIterator<Language, AnyYamlBlockMapEntry>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for YamlBlockMapEntryList {
    type Item = AnyYamlBlockMapEntry;
    type IntoIter = AstNodeListIterator<Language, AnyYamlBlockMapEntry>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct YamlBlockSequenceEntryList {
    syntax_list: SyntaxList,
}
impl YamlBlockSequenceEntryList {
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
impl AstNode for YamlBlockSequenceEntryList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BLOCK_SEQUENCE_ENTRY_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BLOCK_SEQUENCE_ENTRY_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<YamlBlockSequenceEntryList> {
        if Self::can_cast(syntax.kind()) {
            Some(YamlBlockSequenceEntryList {
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
impl Serialize for YamlBlockSequenceEntryList {
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
impl AstNodeList for YamlBlockSequenceEntryList {
    type Language = Language;
    type Node = YamlBlockSequenceEntry;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for YamlBlockSequenceEntryList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("YamlBlockSequenceEntryList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &YamlBlockSequenceEntryList {
    type Item = YamlBlockSequenceEntry;
    type IntoIter = AstNodeListIterator<Language, YamlBlockSequenceEntry>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for YamlBlockSequenceEntryList {
    type Item = YamlBlockSequenceEntry;
    type IntoIter = AstNodeListIterator<Language, YamlBlockSequenceEntry>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct YamlDirectiveList {
    syntax_list: SyntaxList,
}
impl YamlDirectiveList {
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
impl AstNode for YamlDirectiveList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_DIRECTIVE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_DIRECTIVE_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<YamlDirectiveList> {
        if Self::can_cast(syntax.kind()) {
            Some(YamlDirectiveList {
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
impl Serialize for YamlDirectiveList {
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
impl AstNodeList for YamlDirectiveList {
    type Language = Language;
    type Node = YamlDirective;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for YamlDirectiveList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("YamlDirectiveList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &YamlDirectiveList {
    type Item = YamlDirective;
    type IntoIter = AstNodeListIterator<Language, YamlDirective>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for YamlDirectiveList {
    type Item = YamlDirective;
    type IntoIter = AstNodeListIterator<Language, YamlDirective>;
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
pub struct YamlFlowMapEntryList {
    syntax_list: SyntaxList,
}
impl YamlFlowMapEntryList {
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
impl AstNode for YamlFlowMapEntryList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_FLOW_MAP_ENTRY_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_FLOW_MAP_ENTRY_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<YamlFlowMapEntryList> {
        if Self::can_cast(syntax.kind()) {
            Some(YamlFlowMapEntryList {
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
impl Serialize for YamlFlowMapEntryList {
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
impl AstSeparatedList for YamlFlowMapEntryList {
    type Language = Language;
    type Node = AnyYamlFlowMapEntry;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for YamlFlowMapEntryList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("YamlFlowMapEntryList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for YamlFlowMapEntryList {
    type Item = SyntaxResult<AnyYamlFlowMapEntry>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyYamlFlowMapEntry>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &YamlFlowMapEntryList {
    type Item = SyntaxResult<AnyYamlFlowMapEntry>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyYamlFlowMapEntry>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct YamlFlowSequenceEntryList {
    syntax_list: SyntaxList,
}
impl YamlFlowSequenceEntryList {
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
impl AstNode for YamlFlowSequenceEntryList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_FLOW_SEQUENCE_ENTRY_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_FLOW_SEQUENCE_ENTRY_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<YamlFlowSequenceEntryList> {
        if Self::can_cast(syntax.kind()) {
            Some(YamlFlowSequenceEntryList {
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
impl Serialize for YamlFlowSequenceEntryList {
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
impl AstSeparatedList for YamlFlowSequenceEntryList {
    type Language = Language;
    type Node = AnyYamlFlowSequenceEntry;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for YamlFlowSequenceEntryList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("YamlFlowSequenceEntryList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for YamlFlowSequenceEntryList {
    type Item = SyntaxResult<AnyYamlFlowSequenceEntry>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyYamlFlowSequenceEntry>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &YamlFlowSequenceEntryList {
    type Item = SyntaxResult<AnyYamlFlowSequenceEntry>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyYamlFlowSequenceEntry>;
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
