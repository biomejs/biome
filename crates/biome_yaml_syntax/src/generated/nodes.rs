//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::enum_variant_names)]
#![allow(clippy::match_like_matches_macro)]
use crate::{
    macros::map_syntax_node,
    YamlLanguage as Language, YamlSyntaxElement as SyntaxElement,
    YamlSyntaxElementChildren as SyntaxElementChildren,
    YamlSyntaxKind::{self as SyntaxKind, *},
    YamlSyntaxList as SyntaxList, YamlSyntaxNode as SyntaxNode, YamlSyntaxToken as SyntaxToken,
};
use biome_rowan::{support, AstNode, RawSyntaxKind, SyntaxKindSet, SyntaxResult};
#[allow(unused)]
use biome_rowan::{
    AstNodeList, AstNodeListIterator, AstNodeSlotMap, AstSeparatedList,
    AstSeparatedListNodesIterator,
};
#[cfg(feature = "serde")]
use serde::ser::SerializeSeq;
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};
use std::fmt::{Debug, Formatter};
#[doc = r" Sentinel value indicating a missing element in a dynamic node, where"]
#[doc = r" the slots are not statically known."]
#[allow(dead_code)]
pub(crate) const SLOT_MAP_EMPTY_VALUE: u8 = u8::MAX;
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlAlias {
    pub(crate) syntax: SyntaxNode,
}
impl YamlAlias {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlAliasFields {
        YamlAliasFields {
            star_token: self.star_token(),
            label_token: self.label_token(),
        }
    }
    pub fn star_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn label_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for YamlAlias {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct YamlAliasFields {
    pub star_token: SyntaxResult<SyntaxToken>,
    pub label_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlComment {
    pub(crate) syntax: SyntaxNode,
}
impl YamlComment {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlCommentFields {
        YamlCommentFields {
            hash_token: self.hash_token(),
            text_token: self.text_token(),
        }
    }
    pub fn hash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn text_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for YamlComment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct YamlCommentFields {
    pub hash_token: SyntaxResult<SyntaxToken>,
    pub text_token: SyntaxResult<SyntaxToken>,
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
            content: self.content(),
        }
    }
    pub fn content(&self) -> Option<AnyYamlNode> {
        support::node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for YamlDocument {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct YamlDocumentFields {
    pub content: Option<AnyYamlNode>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlMapping {
    pub(crate) syntax: SyntaxNode,
}
impl YamlMapping {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlMappingFields {
        YamlMappingFields {
            l_curly_token: self.l_curly_token(),
            entries: self.entries(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn entries(&self) -> YamlMappingEntries {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for YamlMapping {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct YamlMappingFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub entries: YamlMappingEntries,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlMappingEntry {
    pub(crate) syntax: SyntaxNode,
}
impl YamlMappingEntry {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlMappingEntryFields {
        YamlMappingEntryFields {
            key: self.key(),
            colon_token: self.colon_token(),
            value: self.value(),
        }
    }
    pub fn key(&self) -> SyntaxResult<YamlScalar> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<AnyYamlNode> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for YamlMappingEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct YamlMappingEntryFields {
    pub key: SyntaxResult<YamlScalar>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnyYamlNode>,
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
#[cfg(feature = "serde")]
impl Serialize for YamlRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct YamlRootFields {
    pub bom_token: Option<SyntaxToken>,
    pub documents: YamlDocumentList,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlScalar {
    pub(crate) syntax: SyntaxNode,
}
impl YamlScalar {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlScalarFields {
        YamlScalarFields {
            SCALAR_VALUE_token: self.SCALAR_VALUE_token(),
        }
    }
    pub fn SCALAR_VALUE_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for YamlScalar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct YamlScalarFields {
    pub SCALAR_VALUE_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct YamlSequence {
    pub(crate) syntax: SyntaxNode,
}
impl YamlSequence {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> YamlSequenceFields {
        YamlSequenceFields {
            l_brack_token: self.l_brack_token(),
            elements: self.elements(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn elements(&self) -> YamlSequenceElements {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for YamlSequence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct YamlSequenceFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub elements: YamlSequenceElements,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyYamlNode {
    YamlAlias(YamlAlias),
    YamlBogus(YamlBogus),
    YamlScalar(YamlScalar),
    YamlSequence(YamlSequence),
}
impl AnyYamlNode {
    pub fn as_yaml_alias(&self) -> Option<&YamlAlias> {
        match &self {
            AnyYamlNode::YamlAlias(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_bogus(&self) -> Option<&YamlBogus> {
        match &self {
            AnyYamlNode::YamlBogus(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_scalar(&self) -> Option<&YamlScalar> {
        match &self {
            AnyYamlNode::YamlScalar(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_yaml_sequence(&self) -> Option<&YamlSequence> {
        match &self {
            AnyYamlNode::YamlSequence(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for YamlAlias {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_ALIAS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_ALIAS
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
impl std::fmt::Debug for YamlAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("YamlAlias")
            .field("star_token", &support::DebugSyntaxResult(self.star_token()))
            .field(
                "label_token",
                &support::DebugSyntaxResult(self.label_token()),
            )
            .finish()
    }
}
impl From<YamlAlias> for SyntaxNode {
    fn from(n: YamlAlias) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlAlias> for SyntaxElement {
    fn from(n: YamlAlias) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlComment {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_COMMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_COMMENT
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
impl std::fmt::Debug for YamlComment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("YamlComment")
            .field("hash_token", &support::DebugSyntaxResult(self.hash_token()))
            .field("text_token", &support::DebugSyntaxResult(self.text_token()))
            .finish()
    }
}
impl From<YamlComment> for SyntaxNode {
    fn from(n: YamlComment) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlComment> for SyntaxElement {
    fn from(n: YamlComment) -> SyntaxElement {
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
        f.debug_struct("YamlDocument")
            .field("content", &support::DebugOptionalElement(self.content()))
            .finish()
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
impl AstNode for YamlMapping {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_MAPPING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_MAPPING
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
impl std::fmt::Debug for YamlMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("YamlMapping")
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
    }
}
impl From<YamlMapping> for SyntaxNode {
    fn from(n: YamlMapping) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlMapping> for SyntaxElement {
    fn from(n: YamlMapping) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlMappingEntry {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_MAPPING_ENTRY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_MAPPING_ENTRY
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
impl std::fmt::Debug for YamlMappingEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("YamlMappingEntry")
            .field("key", &support::DebugSyntaxResult(self.key()))
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<YamlMappingEntry> for SyntaxNode {
    fn from(n: YamlMappingEntry) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlMappingEntry> for SyntaxElement {
    fn from(n: YamlMappingEntry) -> SyntaxElement {
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
        f.debug_struct("YamlRoot")
            .field(
                "bom_token",
                &support::DebugOptionalElement(self.bom_token()),
            )
            .field("documents", &self.documents())
            .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
            .finish()
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
impl AstNode for YamlScalar {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_SCALAR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_SCALAR
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
impl std::fmt::Debug for YamlScalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("YamlScalar")
            .field(
                "SCALAR_VALUE_token",
                &support::DebugSyntaxResult(self.SCALAR_VALUE_token()),
            )
            .finish()
    }
}
impl From<YamlScalar> for SyntaxNode {
    fn from(n: YamlScalar) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlScalar> for SyntaxElement {
    fn from(n: YamlScalar) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for YamlSequence {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_SEQUENCE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_SEQUENCE
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
impl std::fmt::Debug for YamlSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("YamlSequence")
            .field(
                "l_brack_token",
                &support::DebugSyntaxResult(self.l_brack_token()),
            )
            .field("elements", &self.elements())
            .field(
                "r_brack_token",
                &support::DebugSyntaxResult(self.r_brack_token()),
            )
            .finish()
    }
}
impl From<YamlSequence> for SyntaxNode {
    fn from(n: YamlSequence) -> SyntaxNode {
        n.syntax
    }
}
impl From<YamlSequence> for SyntaxElement {
    fn from(n: YamlSequence) -> SyntaxElement {
        n.syntax.into()
    }
}
impl From<YamlAlias> for AnyYamlNode {
    fn from(node: YamlAlias) -> AnyYamlNode {
        AnyYamlNode::YamlAlias(node)
    }
}
impl From<YamlBogus> for AnyYamlNode {
    fn from(node: YamlBogus) -> AnyYamlNode {
        AnyYamlNode::YamlBogus(node)
    }
}
impl From<YamlScalar> for AnyYamlNode {
    fn from(node: YamlScalar) -> AnyYamlNode {
        AnyYamlNode::YamlScalar(node)
    }
}
impl From<YamlSequence> for AnyYamlNode {
    fn from(node: YamlSequence) -> AnyYamlNode {
        AnyYamlNode::YamlSequence(node)
    }
}
impl AstNode for AnyYamlNode {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = YamlAlias::KIND_SET
        .union(YamlBogus::KIND_SET)
        .union(YamlScalar::KIND_SET)
        .union(YamlSequence::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, YAML_ALIAS | YAML_BOGUS | YAML_SCALAR | YAML_SEQUENCE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            YAML_ALIAS => AnyYamlNode::YamlAlias(YamlAlias { syntax }),
            YAML_BOGUS => AnyYamlNode::YamlBogus(YamlBogus { syntax }),
            YAML_SCALAR => AnyYamlNode::YamlScalar(YamlScalar { syntax }),
            YAML_SEQUENCE => AnyYamlNode::YamlSequence(YamlSequence { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyYamlNode::YamlAlias(it) => &it.syntax,
            AnyYamlNode::YamlBogus(it) => &it.syntax,
            AnyYamlNode::YamlScalar(it) => &it.syntax,
            AnyYamlNode::YamlSequence(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyYamlNode::YamlAlias(it) => it.syntax,
            AnyYamlNode::YamlBogus(it) => it.syntax,
            AnyYamlNode::YamlScalar(it) => it.syntax,
            AnyYamlNode::YamlSequence(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyYamlNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyYamlNode::YamlAlias(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlNode::YamlBogus(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlNode::YamlScalar(it) => std::fmt::Debug::fmt(it, f),
            AnyYamlNode::YamlSequence(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyYamlNode> for SyntaxNode {
    fn from(n: AnyYamlNode) -> SyntaxNode {
        match n {
            AnyYamlNode::YamlAlias(it) => it.into(),
            AnyYamlNode::YamlBogus(it) => it.into(),
            AnyYamlNode::YamlScalar(it) => it.into(),
            AnyYamlNode::YamlSequence(it) => it.into(),
        }
    }
}
impl From<AnyYamlNode> for SyntaxElement {
    fn from(n: AnyYamlNode) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyYamlNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlComment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlMappingEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlScalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for YamlSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
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
#[cfg(feature = "serde")]
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
pub struct YamlMappingEntries {
    syntax_list: SyntaxList,
}
impl YamlMappingEntries {
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
impl AstNode for YamlMappingEntries {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_MAPPING_ENTRIES as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_MAPPING_ENTRIES
    }
    fn cast(syntax: SyntaxNode) -> Option<YamlMappingEntries> {
        if Self::can_cast(syntax.kind()) {
            Some(YamlMappingEntries {
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
#[cfg(feature = "serde")]
impl Serialize for YamlMappingEntries {
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
impl AstSeparatedList for YamlMappingEntries {
    type Language = Language;
    type Node = YamlMappingEntry;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for YamlMappingEntries {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("YamlMappingEntries ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for YamlMappingEntries {
    type Item = SyntaxResult<YamlMappingEntry>;
    type IntoIter = AstSeparatedListNodesIterator<Language, YamlMappingEntry>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &YamlMappingEntries {
    type Item = SyntaxResult<YamlMappingEntry>;
    type IntoIter = AstSeparatedListNodesIterator<Language, YamlMappingEntry>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct YamlSequenceElements {
    syntax_list: SyntaxList,
}
impl YamlSequenceElements {
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
impl AstNode for YamlSequenceElements {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(YAML_SEQUENCE_ELEMENTS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == YAML_SEQUENCE_ELEMENTS
    }
    fn cast(syntax: SyntaxNode) -> Option<YamlSequenceElements> {
        if Self::can_cast(syntax.kind()) {
            Some(YamlSequenceElements {
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
#[cfg(feature = "serde")]
impl Serialize for YamlSequenceElements {
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
impl AstSeparatedList for YamlSequenceElements {
    type Language = Language;
    type Node = AnyYamlNode;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for YamlSequenceElements {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("YamlSequenceElements ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for YamlSequenceElements {
    type Item = SyntaxResult<AnyYamlNode>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyYamlNode>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &YamlSequenceElements {
    type Item = SyntaxResult<AnyYamlNode>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyYamlNode>;
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
