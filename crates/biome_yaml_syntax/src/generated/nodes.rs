//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(dead_code)]
#![allow(unused)]
use crate::{
    YamlLanguage as Language, YamlSyntaxElement as SyntaxElement,
    YamlSyntaxElementChildren as SyntaxElementChildren,
    YamlSyntaxKind::{self as SyntaxKind, *},
    YamlSyntaxList as SyntaxList, YamlSyntaxNode as SyntaxNode, YamlSyntaxToken as SyntaxToken,
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
pub struct YamlBlockContent {
    pub(crate) syntax: SyntaxNode,
}
impl YamlBlockContent {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlBlockContentFields {
        YamlBlockContentFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for YamlBlockContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlBlockContentFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlBlockInBlockNode {
    pub(crate) syntax: SyntaxNode,
}
impl YamlBlockInBlockNode {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlBlockInBlockNodeFields {
        YamlBlockInBlockNodeFields {
            properties: self.properties(),
            content: self.content(),
        }
    }
    pub fn properties(&self) -> YamlPropertyList {
        support::list(&self.syntax, 0usize)
    }
    pub fn content(&self) -> SyntaxResult<AnyYamlBlockInBlockContent> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for YamlBlockInBlockNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlBlockInBlockNodeFields {
    pub properties: YamlPropertyList,
    pub content: SyntaxResult<AnyYamlBlockInBlockContent>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlBlockKeepIndicator {
    pub(crate) syntax: SyntaxNode,
}
impl YamlBlockKeepIndicator {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlBlockKeepIndicatorFields {
        YamlBlockKeepIndicatorFields {
            plus_token: self.plus_token(),
        }
    }
    pub fn plus_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for YamlBlockKeepIndicator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlBlockKeepIndicatorFields {
    pub plus_token: SyntaxResult<SyntaxToken>,
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
            question_mark_token: self.question_mark_token(),
            key: self.key(),
            colon_token: self.colon_token(),
            value: self.value(),
        }
    }
    pub fn question_mark_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn key(&self) -> Option<AnyYamlBlockNode> {
        support::node(&self.syntax, 1usize)
    }
    pub fn colon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
    pub fn value(&self) -> Option<AnyYamlBlockNode> {
        support::node(&self.syntax, 3usize)
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
    pub question_mark_token: SyntaxResult<SyntaxToken>,
    pub key: Option<AnyYamlBlockNode>,
    pub colon_token: Option<SyntaxToken>,
    pub value: Option<AnyYamlBlockNode>,
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
            colon_token: self.colon_token(),
            value: self.value(),
        }
    }
    pub fn key(&self) -> Option<AnyYamlMappingImplicitKey> {
        support::node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> Option<AnyYamlBlockNode> {
        support::node(&self.syntax, 2usize)
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
    pub key: Option<AnyYamlMappingImplicitKey>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: Option<AnyYamlBlockNode>,
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
            mapping_start_token: self.mapping_start_token(),
            entries: self.entries(),
            mapping_end_token: self.mapping_end_token(),
        }
    }
    pub fn mapping_start_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn entries(&self) -> YamlBlockMapEntryList {
        support::list(&self.syntax, 1usize)
    }
    pub fn mapping_end_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
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
    pub mapping_start_token: SyntaxResult<SyntaxToken>,
    pub entries: YamlBlockMapEntryList,
    pub mapping_end_token: SyntaxResult<SyntaxToken>,
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
            sequence_start_token: self.sequence_start_token(),
            entries: self.entries(),
            sequence_end_token: self.sequence_end_token(),
        }
    }
    pub fn sequence_start_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn entries(&self) -> YamlBlockSequenceEntryList {
        support::list(&self.syntax, 1usize)
    }
    pub fn sequence_end_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
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
    pub sequence_start_token: SyntaxResult<SyntaxToken>,
    pub entries: YamlBlockSequenceEntryList,
    pub sequence_end_token: SyntaxResult<SyntaxToken>,
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
    pub fn value(&self) -> Option<AnyYamlBlockNode> {
        support::node(&self.syntax, 1usize)
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
    pub value: Option<AnyYamlBlockNode>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlBlockStripIndicator {
    pub(crate) syntax: SyntaxNode,
}
impl YamlBlockStripIndicator {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlBlockStripIndicatorFields {
        YamlBlockStripIndicatorFields {
            minus_token: self.minus_token(),
        }
    }
    pub fn minus_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for YamlBlockStripIndicator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlBlockStripIndicatorFields {
    pub minus_token: SyntaxResult<SyntaxToken>,
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
    pub fn node(&self) -> Option<AnyYamlBlockNode> {
        support::node(&self.syntax, 3usize)
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
    pub node: Option<AnyYamlBlockNode>,
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
pub struct YamlFlowInBlockNode {
    pub(crate) syntax: SyntaxNode,
}
impl YamlFlowInBlockNode {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlFlowInBlockNodeFields {
        YamlFlowInBlockNodeFields {
            flow_start_token: self.flow_start_token(),
            flow: self.flow(),
            flow_end_token: self.flow_end_token(),
        }
    }
    pub fn flow_start_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn flow(&self) -> SyntaxResult<AnyYamlFlowNode> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn flow_end_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for YamlFlowInBlockNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlFlowInBlockNodeFields {
    pub flow_start_token: SyntaxResult<SyntaxToken>,
    pub flow: SyntaxResult<AnyYamlFlowNode>,
    pub flow_end_token: SyntaxResult<SyntaxToken>,
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
    pub fn properties(&self) -> YamlPropertyList {
        support::list(&self.syntax, 0usize)
    }
    pub fn content(&self) -> SyntaxResult<AnyYamlJsonContent> {
        support::required_node(&self.syntax, 1usize)
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
    pub properties: YamlPropertyList,
    pub content: SyntaxResult<AnyYamlJsonContent>,
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
            key: self.key(),
            colon_token: self.colon_token(),
            value: self.value(),
        }
    }
    pub fn question_mark_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn key(&self) -> Option<AnyYamlMappingImplicitKey> {
        support::node(&self.syntax, 1usize)
    }
    pub fn colon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
    pub fn value(&self) -> Option<AnyYamlFlowNode> {
        support::node(&self.syntax, 3usize)
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
    pub key: Option<AnyYamlMappingImplicitKey>,
    pub colon_token: Option<SyntaxToken>,
    pub value: Option<AnyYamlFlowNode>,
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
    pub fn key(&self) -> Option<AnyYamlMappingImplicitKey> {
        support::node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> Option<AnyYamlFlowNode> {
        support::node(&self.syntax, 2usize)
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
    pub key: Option<AnyYamlMappingImplicitKey>,
    pub colon_token: Option<SyntaxToken>,
    pub value: Option<AnyYamlFlowNode>,
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
    pub fn properties(&self) -> YamlPropertyList {
        support::list(&self.syntax, 0usize)
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
    pub properties: YamlPropertyList,
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
            r_angle_token: self.r_angle_token(),
            headers: self.headers(),
            content: self.content(),
        }
    }
    pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn headers(&self) -> YamlBlockHeaderList {
        support::list(&self.syntax, 1usize)
    }
    pub fn content(&self) -> SyntaxResult<YamlBlockContent> {
        support::required_node(&self.syntax, 2usize)
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
    pub r_angle_token: SyntaxResult<SyntaxToken>,
    pub headers: YamlBlockHeaderList,
    pub content: SyntaxResult<YamlBlockContent>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlIndentationIndicator {
    pub(crate) syntax: SyntaxNode,
}
impl YamlIndentationIndicator {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlIndentationIndicatorFields {
        YamlIndentationIndicatorFields {
            indentation_indicator_token: self.indentation_indicator_token(),
        }
    }
    pub fn indentation_indicator_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for YamlIndentationIndicator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct YamlIndentationIndicatorFields {
    pub indentation_indicator_token: SyntaxResult<SyntaxToken>,
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
            bitwise_or_token: self.bitwise_or_token(),
            headers: self.headers(),
            content: self.content(),
        }
    }
    pub fn bitwise_or_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn headers(&self) -> YamlBlockHeaderList {
        support::list(&self.syntax, 1usize)
    }
    pub fn content(&self) -> SyntaxResult<YamlBlockContent> {
        support::required_node(&self.syntax, 2usize)
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
    pub bitwise_or_token: SyntaxResult<SyntaxToken>,
    pub headers: YamlBlockHeaderList,
    pub content: SyntaxResult<YamlBlockContent>,
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
    pub documents: YamlDocumentList,
    pub eof_token: SyntaxResult<SyntaxToken>,
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
pub enum AnyYamlBlockHeader {
    YamlBlockKeepIndicator(YamlBlockKeepIndicator),
    YamlBlockStripIndicator(YamlBlockStripIndicator),
    YamlBogusBlockHeader(YamlBogusBlockHeader),
    YamlIndentationIndicator(YamlIndentationIndicator),
}
impl AnyYamlBlockHeader {
    pub fn as_yaml_block_keep_indicator(&self) -> Option<&YamlBlockKeepIndicator> {
        match &self {
            Self::YamlBlockKeepIndicator(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_block_strip_indicator(&self) -> Option<&YamlBlockStripIndicator> {
        match &self {
            Self::YamlBlockStripIndicator(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_bogus_block_header(&self) -> Option<&YamlBogusBlockHeader> {
        match &self {
            Self::YamlBogusBlockHeader(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_indentation_indicator(&self) -> Option<&YamlIndentationIndicator> {
        match &self {
            Self::YamlIndentationIndicator(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyYamlBlockInBlockContent {
    YamlBlockMapping(YamlBlockMapping),
    YamlBlockSequence(YamlBlockSequence),
    YamlFoldedScalar(YamlFoldedScalar),
    YamlLiteralScalar(YamlLiteralScalar),
}
impl AnyYamlBlockInBlockContent {
    pub fn as_yaml_block_mapping(&self) -> Option<&YamlBlockMapping> {
        match &self {
            Self::YamlBlockMapping(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_block_sequence(&self) -> Option<&YamlBlockSequence> {
        match &self {
            Self::YamlBlockSequence(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_folded_scalar(&self) -> Option<&YamlFoldedScalar> {
        match &self {
            Self::YamlFoldedScalar(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_literal_scalar(&self) -> Option<&YamlLiteralScalar> {
        match &self {
            Self::YamlLiteralScalar(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyYamlBlockMapEntry {
    YamlBlockMapExplicitEntry(YamlBlockMapExplicitEntry),
    YamlBlockMapImplicitEntry(YamlBlockMapImplicitEntry),
    YamlBogusBlockMapEntry(YamlBogusBlockMapEntry),
}
impl AnyYamlBlockMapEntry {
    pub fn as_yaml_block_map_explicit_entry(&self) -> Option<&YamlBlockMapExplicitEntry> {
        match &self {
            Self::YamlBlockMapExplicitEntry(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_block_map_implicit_entry(&self) -> Option<&YamlBlockMapImplicitEntry> {
        match &self {
            Self::YamlBlockMapImplicitEntry(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_bogus_block_map_entry(&self) -> Option<&YamlBogusBlockMapEntry> {
        match &self {
            Self::YamlBogusBlockMapEntry(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyYamlBlockNode {
    YamlBlockInBlockNode(YamlBlockInBlockNode),
    YamlBogusBlockNode(YamlBogusBlockNode),
    YamlFlowInBlockNode(YamlFlowInBlockNode),
}
impl AnyYamlBlockNode {
    pub fn as_yaml_block_in_block_node(&self) -> Option<&YamlBlockInBlockNode> {
        match &self {
            Self::YamlBlockInBlockNode(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_bogus_block_node(&self) -> Option<&YamlBogusBlockNode> {
        match &self {
            Self::YamlBogusBlockNode(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_flow_in_block_node(&self) -> Option<&YamlFlowInBlockNode> {
        match &self {
            Self::YamlFlowInBlockNode(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyYamlBlockSequenceEntry {
    YamlBlockSequenceEntry(YamlBlockSequenceEntry),
    YamlBogus(YamlBogus),
}
impl AnyYamlBlockSequenceEntry {
    pub fn as_yaml_block_sequence_entry(&self) -> Option<&YamlBlockSequenceEntry> {
        match &self {
            Self::YamlBlockSequenceEntry(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_bogus(&self) -> Option<&YamlBogus> {
        match &self {
            Self::YamlBogus(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyYamlDocument {
    YamlBogus(YamlBogus),
    YamlDocument(YamlDocument),
}
impl AnyYamlDocument {
    pub fn as_yaml_bogus(&self) -> Option<&YamlBogus> {
        match &self {
            Self::YamlBogus(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_document(&self) -> Option<&YamlDocument> {
        match &self {
            Self::YamlDocument(item) => Some(item),
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
            Self::YamlFlowMapExplicitEntry(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_flow_map_implicit_entry(&self) -> Option<&YamlFlowMapImplicitEntry> {
        match &self {
            Self::YamlFlowMapImplicitEntry(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyYamlFlowNode {
    YamlAliasNode(YamlAliasNode),
    YamlBogusFlowNode(YamlBogusFlowNode),
    YamlFlowJsonNode(YamlFlowJsonNode),
    YamlFlowYamlNode(YamlFlowYamlNode),
}
impl AnyYamlFlowNode {
    pub fn as_yaml_alias_node(&self) -> Option<&YamlAliasNode> {
        match &self {
            Self::YamlAliasNode(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_bogus_flow_node(&self) -> Option<&YamlBogusFlowNode> {
        match &self {
            Self::YamlBogusFlowNode(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_flow_json_node(&self) -> Option<&YamlFlowJsonNode> {
        match &self {
            Self::YamlFlowJsonNode(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_flow_yaml_node(&self) -> Option<&YamlFlowYamlNode> {
        match &self {
            Self::YamlFlowYamlNode(item) => Some(item),
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
            Self::AnyYamlFlowMapEntry(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_any_yaml_flow_node(&self) -> Option<&AnyYamlFlowNode> {
        match &self {
            Self::AnyYamlFlowNode(item) => Some(item),
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
            Self::YamlDoubleQuotedScalar(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_flow_mapping(&self) -> Option<&YamlFlowMapping> {
        match &self {
            Self::YamlFlowMapping(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_flow_sequence(&self) -> Option<&YamlFlowSequence> {
        match &self {
            Self::YamlFlowSequence(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_single_quoted_scalar(&self) -> Option<&YamlSingleQuotedScalar> {
        match &self {
            Self::YamlSingleQuotedScalar(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyYamlMappingImplicitKey {
    YamlAliasNode(YamlAliasNode),
    YamlFlowJsonNode(YamlFlowJsonNode),
    YamlFlowYamlNode(YamlFlowYamlNode),
}
impl AnyYamlMappingImplicitKey {
    pub fn as_yaml_alias_node(&self) -> Option<&YamlAliasNode> {
        match &self {
            Self::YamlAliasNode(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_flow_json_node(&self) -> Option<&YamlFlowJsonNode> {
        match &self {
            Self::YamlFlowJsonNode(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_flow_yaml_node(&self) -> Option<&YamlFlowYamlNode> {
        match &self {
            Self::YamlFlowYamlNode(item) => Some(item),
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
            Self::YamlAnchorProperty(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_tag_property(&self) -> Option<&YamlTagProperty> {
        match &self {
            Self::YamlTagProperty(item) => Some(item),
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlAliasNode> for SyntaxNode {
    fn from(n: YamlAliasNode) -> Self {
        n.syntax
    }
}
impl From<YamlAliasNode> for SyntaxElement {
    fn from(n: YamlAliasNode) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlAnchorProperty> for SyntaxNode {
    fn from(n: YamlAnchorProperty) -> Self {
        n.syntax
    }
}
impl From<YamlAnchorProperty> for SyntaxElement {
    fn from(n: YamlAnchorProperty) -> Self {
        n.syntax.into()
    }
}
impl AstNode for YamlBlockContent {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BLOCK_CONTENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BLOCK_CONTENT
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
impl std::fmt::Debug for YamlBlockContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlBlockContent> for SyntaxNode {
    fn from(n: YamlBlockContent) -> Self {
        n.syntax
    }
}
impl From<YamlBlockContent> for SyntaxElement {
    fn from(n: YamlBlockContent) -> Self {
        n.syntax.into()
    }
}
impl AstNode for YamlBlockInBlockNode {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BLOCK_IN_BLOCK_NODE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BLOCK_IN_BLOCK_NODE
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
impl std::fmt::Debug for YamlBlockInBlockNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlBlockInBlockNode> for SyntaxNode {
    fn from(n: YamlBlockInBlockNode) -> Self {
        n.syntax
    }
}
impl From<YamlBlockInBlockNode> for SyntaxElement {
    fn from(n: YamlBlockInBlockNode) -> Self {
        n.syntax.into()
    }
}
impl AstNode for YamlBlockKeepIndicator {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BLOCK_KEEP_INDICATOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BLOCK_KEEP_INDICATOR
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
impl std::fmt::Debug for YamlBlockKeepIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlBlockKeepIndicator> for SyntaxNode {
    fn from(n: YamlBlockKeepIndicator) -> Self {
        n.syntax
    }
}
impl From<YamlBlockKeepIndicator> for SyntaxElement {
    fn from(n: YamlBlockKeepIndicator) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlBlockMapExplicitEntry> for SyntaxNode {
    fn from(n: YamlBlockMapExplicitEntry) -> Self {
        n.syntax
    }
}
impl From<YamlBlockMapExplicitEntry> for SyntaxElement {
    fn from(n: YamlBlockMapExplicitEntry) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlBlockMapImplicitEntry> for SyntaxNode {
    fn from(n: YamlBlockMapImplicitEntry) -> Self {
        n.syntax
    }
}
impl From<YamlBlockMapImplicitEntry> for SyntaxElement {
    fn from(n: YamlBlockMapImplicitEntry) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlBlockMapping> for SyntaxNode {
    fn from(n: YamlBlockMapping) -> Self {
        n.syntax
    }
}
impl From<YamlBlockMapping> for SyntaxElement {
    fn from(n: YamlBlockMapping) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlBlockSequence> for SyntaxNode {
    fn from(n: YamlBlockSequence) -> Self {
        n.syntax
    }
}
impl From<YamlBlockSequence> for SyntaxElement {
    fn from(n: YamlBlockSequence) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlBlockSequenceEntry> for SyntaxNode {
    fn from(n: YamlBlockSequenceEntry) -> Self {
        n.syntax
    }
}
impl From<YamlBlockSequenceEntry> for SyntaxElement {
    fn from(n: YamlBlockSequenceEntry) -> Self {
        n.syntax.into()
    }
}
impl AstNode for YamlBlockStripIndicator {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BLOCK_STRIP_INDICATOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BLOCK_STRIP_INDICATOR
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
impl std::fmt::Debug for YamlBlockStripIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlBlockStripIndicator> for SyntaxNode {
    fn from(n: YamlBlockStripIndicator) -> Self {
        n.syntax
    }
}
impl From<YamlBlockStripIndicator> for SyntaxElement {
    fn from(n: YamlBlockStripIndicator) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlDirective> for SyntaxNode {
    fn from(n: YamlDirective) -> Self {
        n.syntax
    }
}
impl From<YamlDirective> for SyntaxElement {
    fn from(n: YamlDirective) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlDocument> for SyntaxNode {
    fn from(n: YamlDocument) -> Self {
        n.syntax
    }
}
impl From<YamlDocument> for SyntaxElement {
    fn from(n: YamlDocument) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlDoubleQuotedScalar> for SyntaxNode {
    fn from(n: YamlDoubleQuotedScalar) -> Self {
        n.syntax
    }
}
impl From<YamlDoubleQuotedScalar> for SyntaxElement {
    fn from(n: YamlDoubleQuotedScalar) -> Self {
        n.syntax.into()
    }
}
impl AstNode for YamlFlowInBlockNode {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_FLOW_IN_BLOCK_NODE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_FLOW_IN_BLOCK_NODE
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
impl std::fmt::Debug for YamlFlowInBlockNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlFlowInBlockNode> for SyntaxNode {
    fn from(n: YamlFlowInBlockNode) -> Self {
        n.syntax
    }
}
impl From<YamlFlowInBlockNode> for SyntaxElement {
    fn from(n: YamlFlowInBlockNode) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlFlowJsonNode> for SyntaxNode {
    fn from(n: YamlFlowJsonNode) -> Self {
        n.syntax
    }
}
impl From<YamlFlowJsonNode> for SyntaxElement {
    fn from(n: YamlFlowJsonNode) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlFlowMapExplicitEntry> for SyntaxNode {
    fn from(n: YamlFlowMapExplicitEntry) -> Self {
        n.syntax
    }
}
impl From<YamlFlowMapExplicitEntry> for SyntaxElement {
    fn from(n: YamlFlowMapExplicitEntry) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlFlowMapImplicitEntry> for SyntaxNode {
    fn from(n: YamlFlowMapImplicitEntry) -> Self {
        n.syntax
    }
}
impl From<YamlFlowMapImplicitEntry> for SyntaxElement {
    fn from(n: YamlFlowMapImplicitEntry) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlFlowMapping> for SyntaxNode {
    fn from(n: YamlFlowMapping) -> Self {
        n.syntax
    }
}
impl From<YamlFlowMapping> for SyntaxElement {
    fn from(n: YamlFlowMapping) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlFlowSequence> for SyntaxNode {
    fn from(n: YamlFlowSequence) -> Self {
        n.syntax
    }
}
impl From<YamlFlowSequence> for SyntaxElement {
    fn from(n: YamlFlowSequence) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlFlowYamlNode> for SyntaxNode {
    fn from(n: YamlFlowYamlNode) -> Self {
        n.syntax
    }
}
impl From<YamlFlowYamlNode> for SyntaxElement {
    fn from(n: YamlFlowYamlNode) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlFoldedScalar> for SyntaxNode {
    fn from(n: YamlFoldedScalar) -> Self {
        n.syntax
    }
}
impl From<YamlFoldedScalar> for SyntaxElement {
    fn from(n: YamlFoldedScalar) -> Self {
        n.syntax.into()
    }
}
impl AstNode for YamlIndentationIndicator {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_INDENTATION_INDICATOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_INDENTATION_INDICATOR
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
impl std::fmt::Debug for YamlIndentationIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlIndentationIndicator> for SyntaxNode {
    fn from(n: YamlIndentationIndicator) -> Self {
        n.syntax
    }
}
impl From<YamlIndentationIndicator> for SyntaxElement {
    fn from(n: YamlIndentationIndicator) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlLiteralScalar> for SyntaxNode {
    fn from(n: YamlLiteralScalar) -> Self {
        n.syntax
    }
}
impl From<YamlLiteralScalar> for SyntaxElement {
    fn from(n: YamlLiteralScalar) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlPlainScalar> for SyntaxNode {
    fn from(n: YamlPlainScalar) -> Self {
        n.syntax
    }
}
impl From<YamlPlainScalar> for SyntaxElement {
    fn from(n: YamlPlainScalar) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlRoot> for SyntaxNode {
    fn from(n: YamlRoot) -> Self {
        n.syntax
    }
}
impl From<YamlRoot> for SyntaxElement {
    fn from(n: YamlRoot) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlSingleQuotedScalar> for SyntaxNode {
    fn from(n: YamlSingleQuotedScalar) -> Self {
        n.syntax
    }
}
impl From<YamlSingleQuotedScalar> for SyntaxElement {
    fn from(n: YamlSingleQuotedScalar) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlTagProperty> for SyntaxNode {
    fn from(n: YamlTagProperty) -> Self {
        n.syntax
    }
}
impl From<YamlTagProperty> for SyntaxElement {
    fn from(n: YamlTagProperty) -> Self {
        n.syntax.into()
    }
}
impl From<YamlBlockKeepIndicator> for AnyYamlBlockHeader {
    fn from(node: YamlBlockKeepIndicator) -> Self {
        Self::YamlBlockKeepIndicator(node)
    }
}
impl From<YamlBlockStripIndicator> for AnyYamlBlockHeader {
    fn from(node: YamlBlockStripIndicator) -> Self {
        Self::YamlBlockStripIndicator(node)
    }
}
impl From<YamlBogusBlockHeader> for AnyYamlBlockHeader {
    fn from(node: YamlBogusBlockHeader) -> Self {
        Self::YamlBogusBlockHeader(node)
    }
}
impl From<YamlIndentationIndicator> for AnyYamlBlockHeader {
    fn from(node: YamlIndentationIndicator) -> Self {
        Self::YamlIndentationIndicator(node)
    }
}
impl AstNode for AnyYamlBlockHeader {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = YamlBlockKeepIndicator::KIND_SET
        .union(YamlBlockStripIndicator::KIND_SET)
        .union(YamlBogusBlockHeader::KIND_SET)
        .union(YamlIndentationIndicator::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            YAML_BLOCK_KEEP_INDICATOR
                | YAML_BLOCK_STRIP_INDICATOR
                | YAML_BOGUS_BLOCK_HEADER
                | YAML_INDENTATION_INDICATOR
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            YAML_BLOCK_KEEP_INDICATOR => {
                Self::YamlBlockKeepIndicator(YamlBlockKeepIndicator { syntax })
            }
            YAML_BLOCK_STRIP_INDICATOR => {
                Self::YamlBlockStripIndicator(YamlBlockStripIndicator { syntax })
            }
            YAML_BOGUS_BLOCK_HEADER => Self::YamlBogusBlockHeader(YamlBogusBlockHeader { syntax }),
            YAML_INDENTATION_INDICATOR => {
                Self::YamlIndentationIndicator(YamlIndentationIndicator { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::YamlBlockKeepIndicator(it) => it.syntax(),
            Self::YamlBlockStripIndicator(it) => it.syntax(),
            Self::YamlBogusBlockHeader(it) => it.syntax(),
            Self::YamlIndentationIndicator(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::YamlBlockKeepIndicator(it) => it.into_syntax(),
            Self::YamlBlockStripIndicator(it) => it.into_syntax(),
            Self::YamlBogusBlockHeader(it) => it.into_syntax(),
            Self::YamlIndentationIndicator(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyYamlBlockHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyYamlBlockHeader> for SyntaxNode {
    fn from(n: AnyYamlBlockHeader) -> Self {
        match n {
            AnyYamlBlockHeader::YamlBlockKeepIndicator(it) => it.into_syntax(),
            AnyYamlBlockHeader::YamlBlockStripIndicator(it) => it.into_syntax(),
            AnyYamlBlockHeader::YamlBogusBlockHeader(it) => it.into_syntax(),
            AnyYamlBlockHeader::YamlIndentationIndicator(it) => it.into_syntax(),
        }
    }
}
impl From<AnyYamlBlockHeader> for SyntaxElement {
    fn from(n: AnyYamlBlockHeader) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<YamlBlockMapping> for AnyYamlBlockInBlockContent {
    fn from(node: YamlBlockMapping) -> Self {
        Self::YamlBlockMapping(node)
    }
}
impl From<YamlBlockSequence> for AnyYamlBlockInBlockContent {
    fn from(node: YamlBlockSequence) -> Self {
        Self::YamlBlockSequence(node)
    }
}
impl From<YamlFoldedScalar> for AnyYamlBlockInBlockContent {
    fn from(node: YamlFoldedScalar) -> Self {
        Self::YamlFoldedScalar(node)
    }
}
impl From<YamlLiteralScalar> for AnyYamlBlockInBlockContent {
    fn from(node: YamlLiteralScalar) -> Self {
        Self::YamlLiteralScalar(node)
    }
}
impl AstNode for AnyYamlBlockInBlockContent {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = YamlBlockMapping::KIND_SET
        .union(YamlBlockSequence::KIND_SET)
        .union(YamlFoldedScalar::KIND_SET)
        .union(YamlLiteralScalar::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            YAML_BLOCK_MAPPING | YAML_BLOCK_SEQUENCE | YAML_FOLDED_SCALAR | YAML_LITERAL_SCALAR
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            YAML_BLOCK_MAPPING => Self::YamlBlockMapping(YamlBlockMapping { syntax }),
            YAML_BLOCK_SEQUENCE => Self::YamlBlockSequence(YamlBlockSequence { syntax }),
            YAML_FOLDED_SCALAR => Self::YamlFoldedScalar(YamlFoldedScalar { syntax }),
            YAML_LITERAL_SCALAR => Self::YamlLiteralScalar(YamlLiteralScalar { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::YamlBlockMapping(it) => it.syntax(),
            Self::YamlBlockSequence(it) => it.syntax(),
            Self::YamlFoldedScalar(it) => it.syntax(),
            Self::YamlLiteralScalar(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::YamlBlockMapping(it) => it.into_syntax(),
            Self::YamlBlockSequence(it) => it.into_syntax(),
            Self::YamlFoldedScalar(it) => it.into_syntax(),
            Self::YamlLiteralScalar(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyYamlBlockInBlockContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyYamlBlockInBlockContent> for SyntaxNode {
    fn from(n: AnyYamlBlockInBlockContent) -> Self {
        match n {
            AnyYamlBlockInBlockContent::YamlBlockMapping(it) => it.into_syntax(),
            AnyYamlBlockInBlockContent::YamlBlockSequence(it) => it.into_syntax(),
            AnyYamlBlockInBlockContent::YamlFoldedScalar(it) => it.into_syntax(),
            AnyYamlBlockInBlockContent::YamlLiteralScalar(it) => it.into_syntax(),
        }
    }
}
impl From<AnyYamlBlockInBlockContent> for SyntaxElement {
    fn from(n: AnyYamlBlockInBlockContent) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<YamlBlockMapExplicitEntry> for AnyYamlBlockMapEntry {
    fn from(node: YamlBlockMapExplicitEntry) -> Self {
        Self::YamlBlockMapExplicitEntry(node)
    }
}
impl From<YamlBlockMapImplicitEntry> for AnyYamlBlockMapEntry {
    fn from(node: YamlBlockMapImplicitEntry) -> Self {
        Self::YamlBlockMapImplicitEntry(node)
    }
}
impl From<YamlBogusBlockMapEntry> for AnyYamlBlockMapEntry {
    fn from(node: YamlBogusBlockMapEntry) -> Self {
        Self::YamlBogusBlockMapEntry(node)
    }
}
impl AstNode for AnyYamlBlockMapEntry {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = YamlBlockMapExplicitEntry::KIND_SET
        .union(YamlBlockMapImplicitEntry::KIND_SET)
        .union(YamlBogusBlockMapEntry::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            YAML_BLOCK_MAP_EXPLICIT_ENTRY
                | YAML_BLOCK_MAP_IMPLICIT_ENTRY
                | YAML_BOGUS_BLOCK_MAP_ENTRY
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            YAML_BLOCK_MAP_EXPLICIT_ENTRY => {
                Self::YamlBlockMapExplicitEntry(YamlBlockMapExplicitEntry { syntax })
            }
            YAML_BLOCK_MAP_IMPLICIT_ENTRY => {
                Self::YamlBlockMapImplicitEntry(YamlBlockMapImplicitEntry { syntax })
            }
            YAML_BOGUS_BLOCK_MAP_ENTRY => {
                Self::YamlBogusBlockMapEntry(YamlBogusBlockMapEntry { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::YamlBlockMapExplicitEntry(it) => it.syntax(),
            Self::YamlBlockMapImplicitEntry(it) => it.syntax(),
            Self::YamlBogusBlockMapEntry(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::YamlBlockMapExplicitEntry(it) => it.into_syntax(),
            Self::YamlBlockMapImplicitEntry(it) => it.into_syntax(),
            Self::YamlBogusBlockMapEntry(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyYamlBlockMapEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyYamlBlockMapEntry> for SyntaxNode {
    fn from(n: AnyYamlBlockMapEntry) -> Self {
        match n {
            AnyYamlBlockMapEntry::YamlBlockMapExplicitEntry(it) => it.into_syntax(),
            AnyYamlBlockMapEntry::YamlBlockMapImplicitEntry(it) => it.into_syntax(),
            AnyYamlBlockMapEntry::YamlBogusBlockMapEntry(it) => it.into_syntax(),
        }
    }
}
impl From<AnyYamlBlockMapEntry> for SyntaxElement {
    fn from(n: AnyYamlBlockMapEntry) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<YamlBlockInBlockNode> for AnyYamlBlockNode {
    fn from(node: YamlBlockInBlockNode) -> Self {
        Self::YamlBlockInBlockNode(node)
    }
}
impl From<YamlBogusBlockNode> for AnyYamlBlockNode {
    fn from(node: YamlBogusBlockNode) -> Self {
        Self::YamlBogusBlockNode(node)
    }
}
impl From<YamlFlowInBlockNode> for AnyYamlBlockNode {
    fn from(node: YamlFlowInBlockNode) -> Self {
        Self::YamlFlowInBlockNode(node)
    }
}
impl AstNode for AnyYamlBlockNode {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = YamlBlockInBlockNode::KIND_SET
        .union(YamlBogusBlockNode::KIND_SET)
        .union(YamlFlowInBlockNode::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            YAML_BLOCK_IN_BLOCK_NODE | YAML_BOGUS_BLOCK_NODE | YAML_FLOW_IN_BLOCK_NODE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            YAML_BLOCK_IN_BLOCK_NODE => Self::YamlBlockInBlockNode(YamlBlockInBlockNode { syntax }),
            YAML_BOGUS_BLOCK_NODE => Self::YamlBogusBlockNode(YamlBogusBlockNode { syntax }),
            YAML_FLOW_IN_BLOCK_NODE => Self::YamlFlowInBlockNode(YamlFlowInBlockNode { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::YamlBlockInBlockNode(it) => it.syntax(),
            Self::YamlBogusBlockNode(it) => it.syntax(),
            Self::YamlFlowInBlockNode(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::YamlBlockInBlockNode(it) => it.into_syntax(),
            Self::YamlBogusBlockNode(it) => it.into_syntax(),
            Self::YamlFlowInBlockNode(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyYamlBlockNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyYamlBlockNode> for SyntaxNode {
    fn from(n: AnyYamlBlockNode) -> Self {
        match n {
            AnyYamlBlockNode::YamlBlockInBlockNode(it) => it.into_syntax(),
            AnyYamlBlockNode::YamlBogusBlockNode(it) => it.into_syntax(),
            AnyYamlBlockNode::YamlFlowInBlockNode(it) => it.into_syntax(),
        }
    }
}
impl From<AnyYamlBlockNode> for SyntaxElement {
    fn from(n: AnyYamlBlockNode) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<YamlBlockSequenceEntry> for AnyYamlBlockSequenceEntry {
    fn from(node: YamlBlockSequenceEntry) -> Self {
        Self::YamlBlockSequenceEntry(node)
    }
}
impl From<YamlBogus> for AnyYamlBlockSequenceEntry {
    fn from(node: YamlBogus) -> Self {
        Self::YamlBogus(node)
    }
}
impl AstNode for AnyYamlBlockSequenceEntry {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        YamlBlockSequenceEntry::KIND_SET.union(YamlBogus::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, YAML_BLOCK_SEQUENCE_ENTRY | YAML_BOGUS)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            YAML_BLOCK_SEQUENCE_ENTRY => {
                Self::YamlBlockSequenceEntry(YamlBlockSequenceEntry { syntax })
            }
            YAML_BOGUS => Self::YamlBogus(YamlBogus { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::YamlBlockSequenceEntry(it) => it.syntax(),
            Self::YamlBogus(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::YamlBlockSequenceEntry(it) => it.into_syntax(),
            Self::YamlBogus(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyYamlBlockSequenceEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyYamlBlockSequenceEntry> for SyntaxNode {
    fn from(n: AnyYamlBlockSequenceEntry) -> Self {
        match n {
            AnyYamlBlockSequenceEntry::YamlBlockSequenceEntry(it) => it.into_syntax(),
            AnyYamlBlockSequenceEntry::YamlBogus(it) => it.into_syntax(),
        }
    }
}
impl From<AnyYamlBlockSequenceEntry> for SyntaxElement {
    fn from(n: AnyYamlBlockSequenceEntry) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<YamlBogus> for AnyYamlDocument {
    fn from(node: YamlBogus) -> Self {
        Self::YamlBogus(node)
    }
}
impl From<YamlDocument> for AnyYamlDocument {
    fn from(node: YamlDocument) -> Self {
        Self::YamlDocument(node)
    }
}
impl AstNode for AnyYamlDocument {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = YamlBogus::KIND_SET.union(YamlDocument::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, YAML_BOGUS | YAML_DOCUMENT)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            YAML_BOGUS => Self::YamlBogus(YamlBogus { syntax }),
            YAML_DOCUMENT => Self::YamlDocument(YamlDocument { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::YamlBogus(it) => it.syntax(),
            Self::YamlDocument(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::YamlBogus(it) => it.into_syntax(),
            Self::YamlDocument(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyYamlDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyYamlDocument> for SyntaxNode {
    fn from(n: AnyYamlDocument) -> Self {
        match n {
            AnyYamlDocument::YamlBogus(it) => it.into_syntax(),
            AnyYamlDocument::YamlDocument(it) => it.into_syntax(),
        }
    }
}
impl From<AnyYamlDocument> for SyntaxElement {
    fn from(n: AnyYamlDocument) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<YamlFlowMapExplicitEntry> for AnyYamlFlowMapEntry {
    fn from(node: YamlFlowMapExplicitEntry) -> Self {
        Self::YamlFlowMapExplicitEntry(node)
    }
}
impl From<YamlFlowMapImplicitEntry> for AnyYamlFlowMapEntry {
    fn from(node: YamlFlowMapImplicitEntry) -> Self {
        Self::YamlFlowMapImplicitEntry(node)
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
                Self::YamlFlowMapExplicitEntry(YamlFlowMapExplicitEntry { syntax })
            }
            YAML_FLOW_MAP_IMPLICIT_ENTRY => {
                Self::YamlFlowMapImplicitEntry(YamlFlowMapImplicitEntry { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::YamlFlowMapExplicitEntry(it) => it.syntax(),
            Self::YamlFlowMapImplicitEntry(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::YamlFlowMapExplicitEntry(it) => it.into_syntax(),
            Self::YamlFlowMapImplicitEntry(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyYamlFlowMapEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyYamlFlowMapEntry> for SyntaxNode {
    fn from(n: AnyYamlFlowMapEntry) -> Self {
        match n {
            AnyYamlFlowMapEntry::YamlFlowMapExplicitEntry(it) => it.into_syntax(),
            AnyYamlFlowMapEntry::YamlFlowMapImplicitEntry(it) => it.into_syntax(),
        }
    }
}
impl From<AnyYamlFlowMapEntry> for SyntaxElement {
    fn from(n: AnyYamlFlowMapEntry) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<YamlAliasNode> for AnyYamlFlowNode {
    fn from(node: YamlAliasNode) -> Self {
        Self::YamlAliasNode(node)
    }
}
impl From<YamlBogusFlowNode> for AnyYamlFlowNode {
    fn from(node: YamlBogusFlowNode) -> Self {
        Self::YamlBogusFlowNode(node)
    }
}
impl From<YamlFlowJsonNode> for AnyYamlFlowNode {
    fn from(node: YamlFlowJsonNode) -> Self {
        Self::YamlFlowJsonNode(node)
    }
}
impl From<YamlFlowYamlNode> for AnyYamlFlowNode {
    fn from(node: YamlFlowYamlNode) -> Self {
        Self::YamlFlowYamlNode(node)
    }
}
impl AstNode for AnyYamlFlowNode {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = YamlAliasNode::KIND_SET
        .union(YamlBogusFlowNode::KIND_SET)
        .union(YamlFlowJsonNode::KIND_SET)
        .union(YamlFlowYamlNode::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            YAML_ALIAS_NODE | YAML_BOGUS_FLOW_NODE | YAML_FLOW_JSON_NODE | YAML_FLOW_YAML_NODE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            YAML_ALIAS_NODE => Self::YamlAliasNode(YamlAliasNode { syntax }),
            YAML_BOGUS_FLOW_NODE => Self::YamlBogusFlowNode(YamlBogusFlowNode { syntax }),
            YAML_FLOW_JSON_NODE => Self::YamlFlowJsonNode(YamlFlowJsonNode { syntax }),
            YAML_FLOW_YAML_NODE => Self::YamlFlowYamlNode(YamlFlowYamlNode { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::YamlAliasNode(it) => it.syntax(),
            Self::YamlBogusFlowNode(it) => it.syntax(),
            Self::YamlFlowJsonNode(it) => it.syntax(),
            Self::YamlFlowYamlNode(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::YamlAliasNode(it) => it.into_syntax(),
            Self::YamlBogusFlowNode(it) => it.into_syntax(),
            Self::YamlFlowJsonNode(it) => it.into_syntax(),
            Self::YamlFlowYamlNode(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyYamlFlowNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyYamlFlowNode> for SyntaxNode {
    fn from(n: AnyYamlFlowNode) -> Self {
        match n {
            AnyYamlFlowNode::YamlAliasNode(it) => it.into_syntax(),
            AnyYamlFlowNode::YamlBogusFlowNode(it) => it.into_syntax(),
            AnyYamlFlowNode::YamlFlowJsonNode(it) => it.into_syntax(),
            AnyYamlFlowNode::YamlFlowYamlNode(it) => it.into_syntax(),
        }
    }
}
impl From<AnyYamlFlowNode> for SyntaxElement {
    fn from(n: AnyYamlFlowNode) -> Self {
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
                return Some(Self::AnyYamlFlowMapEntry(any_yaml_flow_map_entry));
            }
            Err(syntax) => syntax,
        };
        if let Some(any_yaml_flow_node) = AnyYamlFlowNode::cast(syntax) {
            return Some(Self::AnyYamlFlowNode(any_yaml_flow_node));
        }
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::AnyYamlFlowMapEntry(it) => it.syntax(),
            Self::AnyYamlFlowNode(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::AnyYamlFlowMapEntry(it) => it.into_syntax(),
            Self::AnyYamlFlowNode(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyYamlFlowSequenceEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyYamlFlowSequenceEntry> for SyntaxNode {
    fn from(n: AnyYamlFlowSequenceEntry) -> Self {
        match n {
            AnyYamlFlowSequenceEntry::AnyYamlFlowMapEntry(it) => it.into_syntax(),
            AnyYamlFlowSequenceEntry::AnyYamlFlowNode(it) => it.into_syntax(),
        }
    }
}
impl From<AnyYamlFlowSequenceEntry> for SyntaxElement {
    fn from(n: AnyYamlFlowSequenceEntry) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<YamlDoubleQuotedScalar> for AnyYamlJsonContent {
    fn from(node: YamlDoubleQuotedScalar) -> Self {
        Self::YamlDoubleQuotedScalar(node)
    }
}
impl From<YamlFlowMapping> for AnyYamlJsonContent {
    fn from(node: YamlFlowMapping) -> Self {
        Self::YamlFlowMapping(node)
    }
}
impl From<YamlFlowSequence> for AnyYamlJsonContent {
    fn from(node: YamlFlowSequence) -> Self {
        Self::YamlFlowSequence(node)
    }
}
impl From<YamlSingleQuotedScalar> for AnyYamlJsonContent {
    fn from(node: YamlSingleQuotedScalar) -> Self {
        Self::YamlSingleQuotedScalar(node)
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
                Self::YamlDoubleQuotedScalar(YamlDoubleQuotedScalar { syntax })
            }
            YAML_FLOW_MAPPING => Self::YamlFlowMapping(YamlFlowMapping { syntax }),
            YAML_FLOW_SEQUENCE => Self::YamlFlowSequence(YamlFlowSequence { syntax }),
            YAML_SINGLE_QUOTED_SCALAR => {
                Self::YamlSingleQuotedScalar(YamlSingleQuotedScalar { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::YamlDoubleQuotedScalar(it) => it.syntax(),
            Self::YamlFlowMapping(it) => it.syntax(),
            Self::YamlFlowSequence(it) => it.syntax(),
            Self::YamlSingleQuotedScalar(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::YamlDoubleQuotedScalar(it) => it.into_syntax(),
            Self::YamlFlowMapping(it) => it.into_syntax(),
            Self::YamlFlowSequence(it) => it.into_syntax(),
            Self::YamlSingleQuotedScalar(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyYamlJsonContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyYamlJsonContent> for SyntaxNode {
    fn from(n: AnyYamlJsonContent) -> Self {
        match n {
            AnyYamlJsonContent::YamlDoubleQuotedScalar(it) => it.into_syntax(),
            AnyYamlJsonContent::YamlFlowMapping(it) => it.into_syntax(),
            AnyYamlJsonContent::YamlFlowSequence(it) => it.into_syntax(),
            AnyYamlJsonContent::YamlSingleQuotedScalar(it) => it.into_syntax(),
        }
    }
}
impl From<AnyYamlJsonContent> for SyntaxElement {
    fn from(n: AnyYamlJsonContent) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<YamlAliasNode> for AnyYamlMappingImplicitKey {
    fn from(node: YamlAliasNode) -> Self {
        Self::YamlAliasNode(node)
    }
}
impl From<YamlFlowJsonNode> for AnyYamlMappingImplicitKey {
    fn from(node: YamlFlowJsonNode) -> Self {
        Self::YamlFlowJsonNode(node)
    }
}
impl From<YamlFlowYamlNode> for AnyYamlMappingImplicitKey {
    fn from(node: YamlFlowYamlNode) -> Self {
        Self::YamlFlowYamlNode(node)
    }
}
impl AstNode for AnyYamlMappingImplicitKey {
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
            YAML_ALIAS_NODE => Self::YamlAliasNode(YamlAliasNode { syntax }),
            YAML_FLOW_JSON_NODE => Self::YamlFlowJsonNode(YamlFlowJsonNode { syntax }),
            YAML_FLOW_YAML_NODE => Self::YamlFlowYamlNode(YamlFlowYamlNode { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::YamlAliasNode(it) => it.syntax(),
            Self::YamlFlowJsonNode(it) => it.syntax(),
            Self::YamlFlowYamlNode(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::YamlAliasNode(it) => it.into_syntax(),
            Self::YamlFlowJsonNode(it) => it.into_syntax(),
            Self::YamlFlowYamlNode(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyYamlMappingImplicitKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyYamlMappingImplicitKey> for SyntaxNode {
    fn from(n: AnyYamlMappingImplicitKey) -> Self {
        match n {
            AnyYamlMappingImplicitKey::YamlAliasNode(it) => it.into_syntax(),
            AnyYamlMappingImplicitKey::YamlFlowJsonNode(it) => it.into_syntax(),
            AnyYamlMappingImplicitKey::YamlFlowYamlNode(it) => it.into_syntax(),
        }
    }
}
impl From<AnyYamlMappingImplicitKey> for SyntaxElement {
    fn from(n: AnyYamlMappingImplicitKey) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<YamlAnchorProperty> for AnyYamlProperty {
    fn from(node: YamlAnchorProperty) -> Self {
        Self::YamlAnchorProperty(node)
    }
}
impl From<YamlTagProperty> for AnyYamlProperty {
    fn from(node: YamlTagProperty) -> Self {
        Self::YamlTagProperty(node)
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
            YAML_ANCHOR_PROPERTY => Self::YamlAnchorProperty(YamlAnchorProperty { syntax }),
            YAML_TAG_PROPERTY => Self::YamlTagProperty(YamlTagProperty { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::YamlAnchorProperty(it) => it.syntax(),
            Self::YamlTagProperty(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::YamlAnchorProperty(it) => it.into_syntax(),
            Self::YamlTagProperty(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyYamlProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyYamlProperty> for SyntaxNode {
    fn from(n: AnyYamlProperty) -> Self {
        match n {
            AnyYamlProperty::YamlAnchorProperty(it) => it.into_syntax(),
            AnyYamlProperty::YamlTagProperty(it) => it.into_syntax(),
        }
    }
}
impl From<AnyYamlProperty> for SyntaxElement {
    fn from(n: AnyYamlProperty) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyYamlBlockHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyYamlBlockInBlockContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyYamlBlockMapEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyYamlBlockNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyYamlBlockSequenceEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyYamlDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyYamlFlowMapEntry {
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
impl std::fmt::Display for AnyYamlMappingImplicitKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyYamlProperty {
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
impl std::fmt::Display for YamlBlockContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlBlockInBlockNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlBlockKeepIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlBlockMapExplicitEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlBlockMapImplicitEntry {
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
impl std::fmt::Display for YamlBlockStripIndicator {
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
impl std::fmt::Display for YamlFlowInBlockNode {
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
impl std::fmt::Display for YamlIndentationIndicator {
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
impl std::fmt::Display for YamlRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlSingleQuotedScalar {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlBogus> for SyntaxNode {
    fn from(n: YamlBogus) -> Self {
        n.syntax
    }
}
impl From<YamlBogus> for SyntaxElement {
    fn from(n: YamlBogus) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct YamlBogusBlockHeader {
    syntax: SyntaxNode,
}
impl YamlBogusBlockHeader {
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
impl AstNode for YamlBogusBlockHeader {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BOGUS_BLOCK_HEADER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BOGUS_BLOCK_HEADER
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
impl std::fmt::Debug for YamlBogusBlockHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlBogusBlockHeader> for SyntaxNode {
    fn from(n: YamlBogusBlockHeader) -> Self {
        n.syntax
    }
}
impl From<YamlBogusBlockHeader> for SyntaxElement {
    fn from(n: YamlBogusBlockHeader) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct YamlBogusBlockMapEntry {
    syntax: SyntaxNode,
}
impl YamlBogusBlockMapEntry {
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
impl AstNode for YamlBogusBlockMapEntry {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BOGUS_BLOCK_MAP_ENTRY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BOGUS_BLOCK_MAP_ENTRY
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
impl std::fmt::Debug for YamlBogusBlockMapEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlBogusBlockMapEntry> for SyntaxNode {
    fn from(n: YamlBogusBlockMapEntry) -> Self {
        n.syntax
    }
}
impl From<YamlBogusBlockMapEntry> for SyntaxElement {
    fn from(n: YamlBogusBlockMapEntry) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct YamlBogusBlockNode {
    syntax: SyntaxNode,
}
impl YamlBogusBlockNode {
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
impl AstNode for YamlBogusBlockNode {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BOGUS_BLOCK_NODE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BOGUS_BLOCK_NODE
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
impl std::fmt::Debug for YamlBogusBlockNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlBogusBlockNode> for SyntaxNode {
    fn from(n: YamlBogusBlockNode) -> Self {
        n.syntax
    }
}
impl From<YamlBogusBlockNode> for SyntaxElement {
    fn from(n: YamlBogusBlockNode) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct YamlBogusFlowNode {
    syntax: SyntaxNode,
}
impl YamlBogusFlowNode {
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
impl AstNode for YamlBogusFlowNode {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BOGUS_FLOW_NODE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BOGUS_FLOW_NODE
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
impl std::fmt::Debug for YamlBogusFlowNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<YamlBogusFlowNode> for SyntaxNode {
    fn from(n: YamlBogusFlowNode) -> Self {
        n.syntax
    }
}
impl From<YamlBogusFlowNode> for SyntaxElement {
    fn from(n: YamlBogusFlowNode) -> Self {
        n.syntax.into()
    }
}
biome_rowan::declare_node_union! { pub AnyYamlBogusNode = YamlBogus | YamlBogusBlockHeader | YamlBogusBlockMapEntry | YamlBogusBlockNode | YamlBogusFlowNode }
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct YamlBlockHeaderList {
    syntax_list: SyntaxList,
}
impl YamlBlockHeaderList {
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
impl AstNode for YamlBlockHeaderList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_BLOCK_HEADER_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_BLOCK_HEADER_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
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
impl Serialize for YamlBlockHeaderList {
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
impl AstNodeList for YamlBlockHeaderList {
    type Language = Language;
    type Node = AnyYamlBlockHeader;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for YamlBlockHeaderList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax_list.node(), f)
    }
}
impl IntoIterator for &YamlBlockHeaderList {
    type Item = AnyYamlBlockHeader;
    type IntoIter = AstNodeListIterator<Language, AnyYamlBlockHeader>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for YamlBlockHeaderList {
    type Item = AnyYamlBlockHeader;
    type IntoIter = AstNodeListIterator<Language, AnyYamlBlockHeader>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
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
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
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
        debug_syntax_node(self.syntax_list.node(), f)
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
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
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
    type Node = AnyYamlBlockSequenceEntry;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for YamlBlockSequenceEntryList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax_list.node(), f)
    }
}
impl IntoIterator for &YamlBlockSequenceEntryList {
    type Item = AnyYamlBlockSequenceEntry;
    type IntoIter = AstNodeListIterator<Language, AnyYamlBlockSequenceEntry>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for YamlBlockSequenceEntryList {
    type Item = AnyYamlBlockSequenceEntry;
    type IntoIter = AstNodeListIterator<Language, AnyYamlBlockSequenceEntry>;
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
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
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
        debug_syntax_node(self.syntax_list.node(), f)
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
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
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
    type Node = AnyYamlDocument;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for YamlDocumentList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax_list.node(), f)
    }
}
impl IntoIterator for &YamlDocumentList {
    type Item = AnyYamlDocument;
    type IntoIter = AstNodeListIterator<Language, AnyYamlDocument>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for YamlDocumentList {
    type Item = AnyYamlDocument;
    type IntoIter = AstNodeListIterator<Language, AnyYamlDocument>;
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
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
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
        debug_syntax_node(self.syntax_list.node(), f)
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
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
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
        debug_syntax_node(self.syntax_list.node(), f)
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
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct YamlPropertyList {
    syntax_list: SyntaxList,
}
impl YamlPropertyList {
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
impl AstNode for YamlPropertyList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_PROPERTY_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_PROPERTY_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
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
impl Serialize for YamlPropertyList {
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
impl AstNodeList for YamlPropertyList {
    type Language = Language;
    type Node = AnyYamlProperty;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for YamlPropertyList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax_list.node(), f)
    }
}
impl IntoIterator for &YamlPropertyList {
    type Item = AnyYamlProperty;
    type IntoIter = AstNodeListIterator<Language, AnyYamlProperty>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for YamlPropertyList {
    type Item = AnyYamlProperty;
    type IntoIter = AstNodeListIterator<Language, AnyYamlProperty>;
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
            SyntaxElement::Node(node) => debug_syntax_node(node, f),
            SyntaxElement::Token(token) => Debug::fmt(token, f),
        }
    }
}
#[doc = r" Describes how the nodes of this language are printed."]
struct DebugShapes;
impl support::DebugNodeShapes for DebugShapes {
    type Language = Language;
    const SHAPES: &'static str = concat!(
        "nYamlAliasNode,value_token\n",
        "nYamlAnchorProperty,value_token\n",
        "nYamlBlockContent,value_token\n",
        "nYamlBlockInBlockNode,properties,content\n",
        "nYamlBlockKeepIndicator,plus_token\n",
        "nYamlBlockMapExplicitEntry,question_mark_token,?key,?colon_token,?value\n",
        "nYamlBlockMapImplicitEntry,?key,colon_token,?value\n",
        "nYamlBlockMapping,mapping_start_token,entries,mapping_end_token\n",
        "nYamlBlockSequence,sequence_start_token,entries,sequence_end_token\n",
        "nYamlBlockSequenceEntry,minus_token,?value\n",
        "nYamlBlockStripIndicator,minus_token\n",
        "nYamlDirective,value_token\n",
        "nYamlDocument,?bom_token,directives,?dashdashdash_token,?node,?dotdotdot_token\n",
        "nYamlDoubleQuotedScalar,value_token\n",
        "nYamlFlowInBlockNode,flow_start_token,flow,flow_end_token\n",
        "nYamlFlowJsonNode,properties,content\n",
        "nYamlFlowMapExplicitEntry,question_mark_token,?key,?colon_token,?value\n",
        "nYamlFlowMapImplicitEntry,?key,?colon_token,?value\n",
        "nYamlFlowMapping,l_curly_token,entries,r_curly_token\n",
        "nYamlFlowSequence,l_brack_token,entries,r_brack_token\n",
        "nYamlFlowYamlNode,properties,?content\n",
        "nYamlFoldedScalar,r_angle_token,headers,content\n",
        "nYamlIndentationIndicator,indentation_indicator_token\n",
        "nYamlLiteralScalar,bitwise_or_token,headers,content\n",
        "nYamlPlainScalar,value_token\n",
        "nYamlRoot,documents,eof_token\n",
        "nYamlSingleQuotedScalar,value_token\n",
        "nYamlTagProperty,value_token\n",
        "bYamlBogus\n",
        "bYamlBogusBlockHeader\n",
        "bYamlBogusBlockMapEntry\n",
        "bYamlBogusBlockNode\n",
        "bYamlBogusFlowNode\n",
        "lYamlBlockHeaderList\n",
        "lYamlBlockMapEntryList\n",
        "lYamlBlockSequenceEntryList\n",
        "lYamlDirectiveList\n",
        "lYamlDocumentList\n",
        "sYamlFlowMapEntryList\n",
        "sYamlFlowSequenceEntryList\n",
        "lYamlPropertyList\n"
    );
    fn shape_offset(kind: SyntaxKind) -> u32 {
        match kind {
            YAML_ALIAS_NODE => 0u32,
            YAML_ANCHOR_PROPERTY => 27u32,
            YAML_BLOCK_CONTENT => 59u32,
            YAML_BLOCK_IN_BLOCK_NODE => 89u32,
            YAML_BLOCK_KEEP_INDICATOR => 130u32,
            YAML_BLOCK_MAP_EXPLICIT_ENTRY => 165u32,
            YAML_BLOCK_MAP_IMPLICIT_ENTRY => 237u32,
            YAML_BLOCK_MAPPING => 288u32,
            YAML_BLOCK_SEQUENCE => 352u32,
            YAML_BLOCK_SEQUENCE_ENTRY => 419u32,
            YAML_BLOCK_STRIP_INDICATOR => 462u32,
            YAML_DIRECTIVE => 499u32,
            YAML_DOCUMENT => 526u32,
            YAML_DOUBLE_QUOTED_SCALAR => 605u32,
            YAML_FLOW_IN_BLOCK_NODE => 641u32,
            YAML_FLOW_JSON_NODE => 699u32,
            YAML_FLOW_MAP_EXPLICIT_ENTRY => 736u32,
            YAML_FLOW_MAP_IMPLICIT_ENTRY => 807u32,
            YAML_FLOW_MAPPING => 858u32,
            YAML_FLOW_SEQUENCE => 911u32,
            YAML_FLOW_YAML_NODE => 965u32,
            YAML_FOLDED_SCALAR => 1003u32,
            YAML_INDENTATION_INDICATOR => 1051u32,
            YAML_LITERAL_SCALAR => 1105u32,
            YAML_PLAIN_SCALAR => 1157u32,
            YAML_ROOT => 1186u32,
            YAML_SINGLE_QUOTED_SCALAR => 1216u32,
            YAML_TAG_PROPERTY => 1252u32,
            YAML_BOGUS => 1281u32,
            YAML_BOGUS_BLOCK_HEADER => 1292u32,
            YAML_BOGUS_BLOCK_MAP_ENTRY => 1314u32,
            YAML_BOGUS_BLOCK_NODE => 1338u32,
            YAML_BOGUS_FLOW_NODE => 1358u32,
            YAML_BLOCK_HEADER_LIST => 1377u32,
            YAML_BLOCK_MAP_ENTRY_LIST => 1398u32,
            YAML_BLOCK_SEQUENCE_ENTRY_LIST => 1421u32,
            YAML_DIRECTIVE_LIST => 1449u32,
            YAML_DOCUMENT_LIST => 1468u32,
            YAML_FLOW_MAP_ENTRY_LIST => 1486u32,
            YAML_FLOW_SEQUENCE_ENTRY_LIST => 1508u32,
            YAML_PROPERTY_LIST => 1535u32,
            _ => u32::MAX,
        }
    }
    fn with_depth<R>(raw_kind: usize, f: impl FnOnce(&std::cell::Cell<u8>) -> R) -> R {
        thread_local! { static DEPTHS : [std :: cell :: Cell < u8 > ; SyntaxKind :: __LAST as usize] = const { [const { std :: cell :: Cell :: new (0) } ; SyntaxKind :: __LAST as usize] } ; }
        DEPTHS.with(|depths| f(&depths[raw_kind]))
    }
    fn debug_unshaped(_: &SyntaxNode, _: &mut Formatter<'_>) -> std::fmt::Result {
        unreachable!("every kind of node has a shape")
    }
}
#[doc = r" Prints `node` the way the AST node of its kind is printed."]
fn debug_syntax_node(node: &SyntaxNode, f: &mut Formatter<'_>) -> std::fmt::Result {
    support::debug_any_node::<DebugShapes>(node, f)
}
