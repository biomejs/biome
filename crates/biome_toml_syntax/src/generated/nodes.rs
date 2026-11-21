//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(dead_code)]
#![allow(unused)]
use crate::{
    TomlLanguage as Language, TomlSyntaxElement as SyntaxElement,
    TomlSyntaxElementChildren as SyntaxElementChildren,
    TomlSyntaxKind::{self as SyntaxKind, *},
    TomlSyntaxList as SyntaxList, TomlSyntaxNode as SyntaxNode, TomlSyntaxToken as SyntaxToken,
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
pub struct TomlArray {
    pub(crate) syntax: SyntaxNode,
}
impl TomlArray {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlArrayFields {
        TomlArrayFields {
            l_brack_token: self.l_brack_token(),
            elements: self.elements(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn elements(&self) -> TomlArrayElementList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for TomlArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlArrayFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub elements: TomlArrayElementList,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlArrayTable {
    pub(crate) syntax: SyntaxNode,
}
impl TomlArrayTable {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlArrayTableFields {
        TomlArrayTableFields {
            header: self.header(),
            members: self.members(),
        }
    }
    pub fn header(&self) -> SyntaxResult<AnyTomlArrayTableHeader> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn members(&self) -> TomlMemberList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for TomlArrayTable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlArrayTableFields {
    pub header: SyntaxResult<AnyTomlArrayTableHeader>,
    pub members: TomlMemberList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlArrayTableHeader {
    pub(crate) syntax: SyntaxNode,
}
impl TomlArrayTableHeader {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlArrayTableHeaderFields {
        TomlArrayTableHeaderFields {
            l_double_brack_token: self.l_double_brack_token(),
            key: self.key(),
            r_double_brack_token: self.r_double_brack_token(),
        }
    }
    pub fn l_double_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn key(&self) -> SyntaxResult<AnyTomlKey> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_double_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for TomlArrayTableHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlArrayTableHeaderFields {
    pub l_double_brack_token: SyntaxResult<SyntaxToken>,
    pub key: SyntaxResult<AnyTomlKey>,
    pub r_double_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlBareKey {
    pub(crate) syntax: SyntaxNode,
}
impl TomlBareKey {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlBareKeyFields {
        TomlBareKeyFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TomlBareKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlBareKeyFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlBasicString {
    pub(crate) syntax: SyntaxNode,
}
impl TomlBasicString {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlBasicStringFields {
        TomlBasicStringFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TomlBasicString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlBasicStringFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlBoolean {
    pub(crate) syntax: SyntaxNode,
}
impl TomlBoolean {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlBooleanFields {
        TomlBooleanFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TomlBoolean {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlBooleanFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlDate {
    pub(crate) syntax: SyntaxNode,
}
impl TomlDate {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlDateFields {
        TomlDateFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TomlDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlDateFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlFloat {
    pub(crate) syntax: SyntaxNode,
}
impl TomlFloat {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlFloatFields {
        TomlFloatFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TomlFloat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlFloatFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlInlineTable {
    pub(crate) syntax: SyntaxNode,
}
impl TomlInlineTable {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlInlineTableFields {
        TomlInlineTableFields {
            l_curly_token: self.l_curly_token(),
            members: self.members(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn members(&self) -> TomlInlineTableMemberList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for TomlInlineTable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlInlineTableFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub members: TomlInlineTableMemberList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlInteger {
    pub(crate) syntax: SyntaxNode,
}
impl TomlInteger {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlIntegerFields {
        TomlIntegerFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TomlInteger {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlIntegerFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlKey {
    pub(crate) syntax: SyntaxNode,
}
impl TomlKey {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlKeyFields {
        TomlKeyFields {
            fragments: self.fragments(),
        }
    }
    pub fn fragments(&self) -> TomlKeyFragmentList {
        support::list(&self.syntax, 0usize)
    }
}
impl Serialize for TomlKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlKeyFields {
    pub fragments: TomlKeyFragmentList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlLiteralString {
    pub(crate) syntax: SyntaxNode,
}
impl TomlLiteralString {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlLiteralStringFields {
        TomlLiteralStringFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TomlLiteralString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlLiteralStringFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlMember {
    pub(crate) syntax: SyntaxNode,
}
impl TomlMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlMemberFields {
        TomlMemberFields {
            key: self.key(),
            eq_token: self.eq_token(),
            value: self.value(),
        }
    }
    pub fn key(&self) -> SyntaxResult<AnyTomlKey> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<AnyTomlValue> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for TomlMember {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlMemberFields {
    pub key: SyntaxResult<AnyTomlKey>,
    pub eq_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnyTomlValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlMultilineBasicString {
    pub(crate) syntax: SyntaxNode,
}
impl TomlMultilineBasicString {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlMultilineBasicStringFields {
        TomlMultilineBasicStringFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TomlMultilineBasicString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlMultilineBasicStringFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlMultilineLiteralString {
    pub(crate) syntax: SyntaxNode,
}
impl TomlMultilineLiteralString {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlMultilineLiteralStringFields {
        TomlMultilineLiteralStringFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TomlMultilineLiteralString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlMultilineLiteralStringFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlQuotedKey {
    pub(crate) syntax: SyntaxNode,
}
impl TomlQuotedKey {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlQuotedKeyFields {
        TomlQuotedKeyFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TomlQuotedKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlQuotedKeyFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlRoot {
    pub(crate) syntax: SyntaxNode,
}
impl TomlRoot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlRootFields {
        TomlRootFields {
            bom_token: self.bom_token(),
            items: self.items(),
            eof_token: self.eof_token(),
        }
    }
    pub fn bom_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> TomlRootItemList {
        support::list(&self.syntax, 1usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for TomlRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlRootFields {
    pub bom_token: Option<SyntaxToken>,
    pub items: TomlRootItemList,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlTable {
    pub(crate) syntax: SyntaxNode,
}
impl TomlTable {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlTableFields {
        TomlTableFields {
            header: self.header(),
            members: self.members(),
        }
    }
    pub fn header(&self) -> SyntaxResult<AnyTomlTableHeader> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn members(&self) -> TomlMemberList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for TomlTable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlTableFields {
    pub header: SyntaxResult<AnyTomlTableHeader>,
    pub members: TomlMemberList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlTableHeader {
    pub(crate) syntax: SyntaxNode,
}
impl TomlTableHeader {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlTableHeaderFields {
        TomlTableHeaderFields {
            l_brack_token: self.l_brack_token(),
            key: self.key(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn key(&self) -> SyntaxResult<AnyTomlKey> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for TomlTableHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlTableHeaderFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub key: SyntaxResult<AnyTomlKey>,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlTime {
    pub(crate) syntax: SyntaxNode,
}
impl TomlTime {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlTimeFields {
        TomlTimeFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TomlTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlTimeFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyTomlArrayTableHeader {
    TomlArrayTableHeader(TomlArrayTableHeader),
    TomlBogusTableHeader(TomlBogusTableHeader),
}
impl AnyTomlArrayTableHeader {
    pub fn as_toml_array_table_header(&self) -> Option<&TomlArrayTableHeader> {
        match &self {
            Self::TomlArrayTableHeader(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_bogus_table_header(&self) -> Option<&TomlBogusTableHeader> {
        match &self {
            Self::TomlBogusTableHeader(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyTomlKey {
    TomlBogusKey(TomlBogusKey),
    TomlKey(TomlKey),
}
impl AnyTomlKey {
    pub fn as_toml_bogus_key(&self) -> Option<&TomlBogusKey> {
        match &self {
            Self::TomlBogusKey(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_key(&self) -> Option<&TomlKey> {
        match &self {
            Self::TomlKey(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyTomlKeyFragment {
    TomlBareKey(TomlBareKey),
    TomlBogusKeyFragment(TomlBogusKeyFragment),
    TomlQuotedKey(TomlQuotedKey),
}
impl AnyTomlKeyFragment {
    pub fn as_toml_bare_key(&self) -> Option<&TomlBareKey> {
        match &self {
            Self::TomlBareKey(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_bogus_key_fragment(&self) -> Option<&TomlBogusKeyFragment> {
        match &self {
            Self::TomlBogusKeyFragment(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_quoted_key(&self) -> Option<&TomlQuotedKey> {
        match &self {
            Self::TomlQuotedKey(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyTomlMember {
    TomlBogusMember(TomlBogusMember),
    TomlMember(TomlMember),
}
impl AnyTomlMember {
    pub fn as_toml_bogus_member(&self) -> Option<&TomlBogusMember> {
        match &self {
            Self::TomlBogusMember(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_member(&self) -> Option<&TomlMember> {
        match &self {
            Self::TomlMember(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyTomlRootItem {
    TomlArrayTable(TomlArrayTable),
    TomlBogusMember(TomlBogusMember),
    TomlMember(TomlMember),
    TomlTable(TomlTable),
}
impl AnyTomlRootItem {
    pub fn as_toml_array_table(&self) -> Option<&TomlArrayTable> {
        match &self {
            Self::TomlArrayTable(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_bogus_member(&self) -> Option<&TomlBogusMember> {
        match &self {
            Self::TomlBogusMember(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_member(&self) -> Option<&TomlMember> {
        match &self {
            Self::TomlMember(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_table(&self) -> Option<&TomlTable> {
        match &self {
            Self::TomlTable(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyTomlString {
    TomlBasicString(TomlBasicString),
    TomlLiteralString(TomlLiteralString),
    TomlMultilineBasicString(TomlMultilineBasicString),
    TomlMultilineLiteralString(TomlMultilineLiteralString),
}
impl AnyTomlString {
    pub fn as_toml_basic_string(&self) -> Option<&TomlBasicString> {
        match &self {
            Self::TomlBasicString(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_literal_string(&self) -> Option<&TomlLiteralString> {
        match &self {
            Self::TomlLiteralString(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_multiline_basic_string(&self) -> Option<&TomlMultilineBasicString> {
        match &self {
            Self::TomlMultilineBasicString(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_multiline_literal_string(&self) -> Option<&TomlMultilineLiteralString> {
        match &self {
            Self::TomlMultilineLiteralString(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyTomlTableHeader {
    TomlBogusTableHeader(TomlBogusTableHeader),
    TomlTableHeader(TomlTableHeader),
}
impl AnyTomlTableHeader {
    pub fn as_toml_bogus_table_header(&self) -> Option<&TomlBogusTableHeader> {
        match &self {
            Self::TomlBogusTableHeader(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_table_header(&self) -> Option<&TomlTableHeader> {
        match &self {
            Self::TomlTableHeader(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyTomlValue {
    AnyTomlString(AnyTomlString),
    TomlArray(TomlArray),
    TomlBogusValue(TomlBogusValue),
    TomlBoolean(TomlBoolean),
    TomlDate(TomlDate),
    TomlFloat(TomlFloat),
    TomlInlineTable(TomlInlineTable),
    TomlInteger(TomlInteger),
    TomlTime(TomlTime),
}
impl AnyTomlValue {
    pub fn as_any_toml_string(&self) -> Option<&AnyTomlString> {
        match &self {
            Self::AnyTomlString(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_array(&self) -> Option<&TomlArray> {
        match &self {
            Self::TomlArray(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_bogus_value(&self) -> Option<&TomlBogusValue> {
        match &self {
            Self::TomlBogusValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_boolean(&self) -> Option<&TomlBoolean> {
        match &self {
            Self::TomlBoolean(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_date(&self) -> Option<&TomlDate> {
        match &self {
            Self::TomlDate(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_float(&self) -> Option<&TomlFloat> {
        match &self {
            Self::TomlFloat(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_inline_table(&self) -> Option<&TomlInlineTable> {
        match &self {
            Self::TomlInlineTable(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_integer(&self) -> Option<&TomlInteger> {
        match &self {
            Self::TomlInteger(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_time(&self) -> Option<&TomlTime> {
        match &self {
            Self::TomlTime(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for TomlArray {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_ARRAY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_ARRAY
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
impl std::fmt::Debug for TomlArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlArray")
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
        } else {
            f.debug_struct("TomlArray").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlArray> for SyntaxNode {
    fn from(n: TomlArray) -> Self {
        n.syntax
    }
}
impl From<TomlArray> for SyntaxElement {
    fn from(n: TomlArray) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TomlArrayTable {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_ARRAY_TABLE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_ARRAY_TABLE
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
impl std::fmt::Debug for TomlArrayTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlArrayTable")
                .field("header", &support::DebugSyntaxResult(self.header()))
                .field("members", &self.members())
                .finish()
        } else {
            f.debug_struct("TomlArrayTable").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlArrayTable> for SyntaxNode {
    fn from(n: TomlArrayTable) -> Self {
        n.syntax
    }
}
impl From<TomlArrayTable> for SyntaxElement {
    fn from(n: TomlArrayTable) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TomlArrayTableHeader {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_ARRAY_TABLE_HEADER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_ARRAY_TABLE_HEADER
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
impl std::fmt::Debug for TomlArrayTableHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlArrayTableHeader")
                .field(
                    "l_double_brack_token",
                    &support::DebugSyntaxResult(self.l_double_brack_token()),
                )
                .field("key", &support::DebugSyntaxResult(self.key()))
                .field(
                    "r_double_brack_token",
                    &support::DebugSyntaxResult(self.r_double_brack_token()),
                )
                .finish()
        } else {
            f.debug_struct("TomlArrayTableHeader").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlArrayTableHeader> for SyntaxNode {
    fn from(n: TomlArrayTableHeader) -> Self {
        n.syntax
    }
}
impl From<TomlArrayTableHeader> for SyntaxElement {
    fn from(n: TomlArrayTableHeader) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TomlBareKey {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_BARE_KEY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_BARE_KEY
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
impl std::fmt::Debug for TomlBareKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlBareKey")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("TomlBareKey").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlBareKey> for SyntaxNode {
    fn from(n: TomlBareKey) -> Self {
        n.syntax
    }
}
impl From<TomlBareKey> for SyntaxElement {
    fn from(n: TomlBareKey) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TomlBasicString {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_BASIC_STRING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_BASIC_STRING
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
impl std::fmt::Debug for TomlBasicString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlBasicString")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("TomlBasicString").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlBasicString> for SyntaxNode {
    fn from(n: TomlBasicString) -> Self {
        n.syntax
    }
}
impl From<TomlBasicString> for SyntaxElement {
    fn from(n: TomlBasicString) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TomlBoolean {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_BOOLEAN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_BOOLEAN
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
impl std::fmt::Debug for TomlBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlBoolean")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("TomlBoolean").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlBoolean> for SyntaxNode {
    fn from(n: TomlBoolean) -> Self {
        n.syntax
    }
}
impl From<TomlBoolean> for SyntaxElement {
    fn from(n: TomlBoolean) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TomlDate {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_DATE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_DATE
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
impl std::fmt::Debug for TomlDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlDate")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("TomlDate").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlDate> for SyntaxNode {
    fn from(n: TomlDate) -> Self {
        n.syntax
    }
}
impl From<TomlDate> for SyntaxElement {
    fn from(n: TomlDate) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TomlFloat {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_FLOAT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_FLOAT
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
impl std::fmt::Debug for TomlFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlFloat")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("TomlFloat").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlFloat> for SyntaxNode {
    fn from(n: TomlFloat) -> Self {
        n.syntax
    }
}
impl From<TomlFloat> for SyntaxElement {
    fn from(n: TomlFloat) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TomlInlineTable {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_INLINE_TABLE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_INLINE_TABLE
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
impl std::fmt::Debug for TomlInlineTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlInlineTable")
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("members", &self.members())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("TomlInlineTable").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlInlineTable> for SyntaxNode {
    fn from(n: TomlInlineTable) -> Self {
        n.syntax
    }
}
impl From<TomlInlineTable> for SyntaxElement {
    fn from(n: TomlInlineTable) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TomlInteger {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_INTEGER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_INTEGER
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
impl std::fmt::Debug for TomlInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlInteger")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("TomlInteger").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlInteger> for SyntaxNode {
    fn from(n: TomlInteger) -> Self {
        n.syntax
    }
}
impl From<TomlInteger> for SyntaxElement {
    fn from(n: TomlInteger) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TomlKey {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_KEY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_KEY
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
impl std::fmt::Debug for TomlKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlKey")
                .field("fragments", &self.fragments())
                .finish()
        } else {
            f.debug_struct("TomlKey").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlKey> for SyntaxNode {
    fn from(n: TomlKey) -> Self {
        n.syntax
    }
}
impl From<TomlKey> for SyntaxElement {
    fn from(n: TomlKey) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TomlLiteralString {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_LITERAL_STRING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_LITERAL_STRING
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
impl std::fmt::Debug for TomlLiteralString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlLiteralString")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("TomlLiteralString").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlLiteralString> for SyntaxNode {
    fn from(n: TomlLiteralString) -> Self {
        n.syntax
    }
}
impl From<TomlLiteralString> for SyntaxElement {
    fn from(n: TomlLiteralString) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TomlMember {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_MEMBER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_MEMBER
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
impl std::fmt::Debug for TomlMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlMember")
                .field("key", &support::DebugSyntaxResult(self.key()))
                .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("TomlMember").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlMember> for SyntaxNode {
    fn from(n: TomlMember) -> Self {
        n.syntax
    }
}
impl From<TomlMember> for SyntaxElement {
    fn from(n: TomlMember) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TomlMultilineBasicString {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_MULTILINE_BASIC_STRING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_MULTILINE_BASIC_STRING
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
impl std::fmt::Debug for TomlMultilineBasicString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlMultilineBasicString")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("TomlMultilineBasicString").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlMultilineBasicString> for SyntaxNode {
    fn from(n: TomlMultilineBasicString) -> Self {
        n.syntax
    }
}
impl From<TomlMultilineBasicString> for SyntaxElement {
    fn from(n: TomlMultilineBasicString) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TomlMultilineLiteralString {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_MULTILINE_LITERAL_STRING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_MULTILINE_LITERAL_STRING
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
impl std::fmt::Debug for TomlMultilineLiteralString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlMultilineLiteralString")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("TomlMultilineLiteralString").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlMultilineLiteralString> for SyntaxNode {
    fn from(n: TomlMultilineLiteralString) -> Self {
        n.syntax
    }
}
impl From<TomlMultilineLiteralString> for SyntaxElement {
    fn from(n: TomlMultilineLiteralString) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TomlQuotedKey {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_QUOTED_KEY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_QUOTED_KEY
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
impl std::fmt::Debug for TomlQuotedKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlQuotedKey")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("TomlQuotedKey").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlQuotedKey> for SyntaxNode {
    fn from(n: TomlQuotedKey) -> Self {
        n.syntax
    }
}
impl From<TomlQuotedKey> for SyntaxElement {
    fn from(n: TomlQuotedKey) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TomlRoot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_ROOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_ROOT
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
impl std::fmt::Debug for TomlRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlRoot")
                .field(
                    "bom_token",
                    &support::DebugOptionalElement(self.bom_token()),
                )
                .field("items", &self.items())
                .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
                .finish()
        } else {
            f.debug_struct("TomlRoot").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlRoot> for SyntaxNode {
    fn from(n: TomlRoot) -> Self {
        n.syntax
    }
}
impl From<TomlRoot> for SyntaxElement {
    fn from(n: TomlRoot) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TomlTable {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_TABLE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_TABLE
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
impl std::fmt::Debug for TomlTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlTable")
                .field("header", &support::DebugSyntaxResult(self.header()))
                .field("members", &self.members())
                .finish()
        } else {
            f.debug_struct("TomlTable").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlTable> for SyntaxNode {
    fn from(n: TomlTable) -> Self {
        n.syntax
    }
}
impl From<TomlTable> for SyntaxElement {
    fn from(n: TomlTable) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TomlTableHeader {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_TABLE_HEADER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_TABLE_HEADER
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
impl std::fmt::Debug for TomlTableHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlTableHeader")
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("key", &support::DebugSyntaxResult(self.key()))
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .finish()
        } else {
            f.debug_struct("TomlTableHeader").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlTableHeader> for SyntaxNode {
    fn from(n: TomlTableHeader) -> Self {
        n.syntax
    }
}
impl From<TomlTableHeader> for SyntaxElement {
    fn from(n: TomlTableHeader) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TomlTime {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_TIME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_TIME
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
impl std::fmt::Debug for TomlTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlTime")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("TomlTime").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlTime> for SyntaxNode {
    fn from(n: TomlTime) -> Self {
        n.syntax
    }
}
impl From<TomlTime> for SyntaxElement {
    fn from(n: TomlTime) -> Self {
        n.syntax.into()
    }
}
impl From<TomlArrayTableHeader> for AnyTomlArrayTableHeader {
    fn from(node: TomlArrayTableHeader) -> Self {
        Self::TomlArrayTableHeader(node)
    }
}
impl From<TomlBogusTableHeader> for AnyTomlArrayTableHeader {
    fn from(node: TomlBogusTableHeader) -> Self {
        Self::TomlBogusTableHeader(node)
    }
}
impl AstNode for AnyTomlArrayTableHeader {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        TomlArrayTableHeader::KIND_SET.union(TomlBogusTableHeader::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, TOML_ARRAY_TABLE_HEADER | TOML_BOGUS_TABLE_HEADER)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TOML_ARRAY_TABLE_HEADER => Self::TomlArrayTableHeader(TomlArrayTableHeader { syntax }),
            TOML_BOGUS_TABLE_HEADER => Self::TomlBogusTableHeader(TomlBogusTableHeader { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::TomlArrayTableHeader(it) => it.syntax(),
            Self::TomlBogusTableHeader(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::TomlArrayTableHeader(it) => it.into_syntax(),
            Self::TomlBogusTableHeader(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyTomlArrayTableHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TomlArrayTableHeader(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlBogusTableHeader(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyTomlArrayTableHeader> for SyntaxNode {
    fn from(n: AnyTomlArrayTableHeader) -> Self {
        match n {
            AnyTomlArrayTableHeader::TomlArrayTableHeader(it) => it.into_syntax(),
            AnyTomlArrayTableHeader::TomlBogusTableHeader(it) => it.into_syntax(),
        }
    }
}
impl From<AnyTomlArrayTableHeader> for SyntaxElement {
    fn from(n: AnyTomlArrayTableHeader) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TomlBogusKey> for AnyTomlKey {
    fn from(node: TomlBogusKey) -> Self {
        Self::TomlBogusKey(node)
    }
}
impl From<TomlKey> for AnyTomlKey {
    fn from(node: TomlKey) -> Self {
        Self::TomlKey(node)
    }
}
impl AstNode for AnyTomlKey {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = TomlBogusKey::KIND_SET.union(TomlKey::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, TOML_BOGUS_KEY | TOML_KEY)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TOML_BOGUS_KEY => Self::TomlBogusKey(TomlBogusKey { syntax }),
            TOML_KEY => Self::TomlKey(TomlKey { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::TomlBogusKey(it) => it.syntax(),
            Self::TomlKey(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::TomlBogusKey(it) => it.into_syntax(),
            Self::TomlKey(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyTomlKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TomlBogusKey(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlKey(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyTomlKey> for SyntaxNode {
    fn from(n: AnyTomlKey) -> Self {
        match n {
            AnyTomlKey::TomlBogusKey(it) => it.into_syntax(),
            AnyTomlKey::TomlKey(it) => it.into_syntax(),
        }
    }
}
impl From<AnyTomlKey> for SyntaxElement {
    fn from(n: AnyTomlKey) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TomlBareKey> for AnyTomlKeyFragment {
    fn from(node: TomlBareKey) -> Self {
        Self::TomlBareKey(node)
    }
}
impl From<TomlBogusKeyFragment> for AnyTomlKeyFragment {
    fn from(node: TomlBogusKeyFragment) -> Self {
        Self::TomlBogusKeyFragment(node)
    }
}
impl From<TomlQuotedKey> for AnyTomlKeyFragment {
    fn from(node: TomlQuotedKey) -> Self {
        Self::TomlQuotedKey(node)
    }
}
impl AstNode for AnyTomlKeyFragment {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = TomlBareKey::KIND_SET
        .union(TomlBogusKeyFragment::KIND_SET)
        .union(TomlQuotedKey::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            TOML_BARE_KEY | TOML_BOGUS_KEY_FRAGMENT | TOML_QUOTED_KEY
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TOML_BARE_KEY => Self::TomlBareKey(TomlBareKey { syntax }),
            TOML_BOGUS_KEY_FRAGMENT => Self::TomlBogusKeyFragment(TomlBogusKeyFragment { syntax }),
            TOML_QUOTED_KEY => Self::TomlQuotedKey(TomlQuotedKey { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::TomlBareKey(it) => it.syntax(),
            Self::TomlBogusKeyFragment(it) => it.syntax(),
            Self::TomlQuotedKey(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::TomlBareKey(it) => it.into_syntax(),
            Self::TomlBogusKeyFragment(it) => it.into_syntax(),
            Self::TomlQuotedKey(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyTomlKeyFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TomlBareKey(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlBogusKeyFragment(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlQuotedKey(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyTomlKeyFragment> for SyntaxNode {
    fn from(n: AnyTomlKeyFragment) -> Self {
        match n {
            AnyTomlKeyFragment::TomlBareKey(it) => it.into_syntax(),
            AnyTomlKeyFragment::TomlBogusKeyFragment(it) => it.into_syntax(),
            AnyTomlKeyFragment::TomlQuotedKey(it) => it.into_syntax(),
        }
    }
}
impl From<AnyTomlKeyFragment> for SyntaxElement {
    fn from(n: AnyTomlKeyFragment) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TomlBogusMember> for AnyTomlMember {
    fn from(node: TomlBogusMember) -> Self {
        Self::TomlBogusMember(node)
    }
}
impl From<TomlMember> for AnyTomlMember {
    fn from(node: TomlMember) -> Self {
        Self::TomlMember(node)
    }
}
impl AstNode for AnyTomlMember {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = TomlBogusMember::KIND_SET.union(TomlMember::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, TOML_BOGUS_MEMBER | TOML_MEMBER)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TOML_BOGUS_MEMBER => Self::TomlBogusMember(TomlBogusMember { syntax }),
            TOML_MEMBER => Self::TomlMember(TomlMember { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::TomlBogusMember(it) => it.syntax(),
            Self::TomlMember(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::TomlBogusMember(it) => it.into_syntax(),
            Self::TomlMember(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyTomlMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TomlBogusMember(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlMember(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyTomlMember> for SyntaxNode {
    fn from(n: AnyTomlMember) -> Self {
        match n {
            AnyTomlMember::TomlBogusMember(it) => it.into_syntax(),
            AnyTomlMember::TomlMember(it) => it.into_syntax(),
        }
    }
}
impl From<AnyTomlMember> for SyntaxElement {
    fn from(n: AnyTomlMember) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TomlArrayTable> for AnyTomlRootItem {
    fn from(node: TomlArrayTable) -> Self {
        Self::TomlArrayTable(node)
    }
}
impl From<TomlBogusMember> for AnyTomlRootItem {
    fn from(node: TomlBogusMember) -> Self {
        Self::TomlBogusMember(node)
    }
}
impl From<TomlMember> for AnyTomlRootItem {
    fn from(node: TomlMember) -> Self {
        Self::TomlMember(node)
    }
}
impl From<TomlTable> for AnyTomlRootItem {
    fn from(node: TomlTable) -> Self {
        Self::TomlTable(node)
    }
}
impl AstNode for AnyTomlRootItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = TomlArrayTable::KIND_SET
        .union(TomlBogusMember::KIND_SET)
        .union(TomlMember::KIND_SET)
        .union(TomlTable::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            TOML_ARRAY_TABLE | TOML_BOGUS_MEMBER | TOML_MEMBER | TOML_TABLE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TOML_ARRAY_TABLE => Self::TomlArrayTable(TomlArrayTable { syntax }),
            TOML_BOGUS_MEMBER => Self::TomlBogusMember(TomlBogusMember { syntax }),
            TOML_MEMBER => Self::TomlMember(TomlMember { syntax }),
            TOML_TABLE => Self::TomlTable(TomlTable { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::TomlArrayTable(it) => it.syntax(),
            Self::TomlBogusMember(it) => it.syntax(),
            Self::TomlMember(it) => it.syntax(),
            Self::TomlTable(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::TomlArrayTable(it) => it.into_syntax(),
            Self::TomlBogusMember(it) => it.into_syntax(),
            Self::TomlMember(it) => it.into_syntax(),
            Self::TomlTable(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyTomlRootItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TomlArrayTable(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlBogusMember(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlMember(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlTable(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyTomlRootItem> for SyntaxNode {
    fn from(n: AnyTomlRootItem) -> Self {
        match n {
            AnyTomlRootItem::TomlArrayTable(it) => it.into_syntax(),
            AnyTomlRootItem::TomlBogusMember(it) => it.into_syntax(),
            AnyTomlRootItem::TomlMember(it) => it.into_syntax(),
            AnyTomlRootItem::TomlTable(it) => it.into_syntax(),
        }
    }
}
impl From<AnyTomlRootItem> for SyntaxElement {
    fn from(n: AnyTomlRootItem) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TomlBasicString> for AnyTomlString {
    fn from(node: TomlBasicString) -> Self {
        Self::TomlBasicString(node)
    }
}
impl From<TomlLiteralString> for AnyTomlString {
    fn from(node: TomlLiteralString) -> Self {
        Self::TomlLiteralString(node)
    }
}
impl From<TomlMultilineBasicString> for AnyTomlString {
    fn from(node: TomlMultilineBasicString) -> Self {
        Self::TomlMultilineBasicString(node)
    }
}
impl From<TomlMultilineLiteralString> for AnyTomlString {
    fn from(node: TomlMultilineLiteralString) -> Self {
        Self::TomlMultilineLiteralString(node)
    }
}
impl AstNode for AnyTomlString {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = TomlBasicString::KIND_SET
        .union(TomlLiteralString::KIND_SET)
        .union(TomlMultilineBasicString::KIND_SET)
        .union(TomlMultilineLiteralString::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            TOML_BASIC_STRING
                | TOML_LITERAL_STRING
                | TOML_MULTILINE_BASIC_STRING
                | TOML_MULTILINE_LITERAL_STRING
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TOML_BASIC_STRING => Self::TomlBasicString(TomlBasicString { syntax }),
            TOML_LITERAL_STRING => Self::TomlLiteralString(TomlLiteralString { syntax }),
            TOML_MULTILINE_BASIC_STRING => {
                Self::TomlMultilineBasicString(TomlMultilineBasicString { syntax })
            }
            TOML_MULTILINE_LITERAL_STRING => {
                Self::TomlMultilineLiteralString(TomlMultilineLiteralString { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::TomlBasicString(it) => it.syntax(),
            Self::TomlLiteralString(it) => it.syntax(),
            Self::TomlMultilineBasicString(it) => it.syntax(),
            Self::TomlMultilineLiteralString(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::TomlBasicString(it) => it.into_syntax(),
            Self::TomlLiteralString(it) => it.into_syntax(),
            Self::TomlMultilineBasicString(it) => it.into_syntax(),
            Self::TomlMultilineLiteralString(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyTomlString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TomlBasicString(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlLiteralString(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlMultilineBasicString(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlMultilineLiteralString(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyTomlString> for SyntaxNode {
    fn from(n: AnyTomlString) -> Self {
        match n {
            AnyTomlString::TomlBasicString(it) => it.into_syntax(),
            AnyTomlString::TomlLiteralString(it) => it.into_syntax(),
            AnyTomlString::TomlMultilineBasicString(it) => it.into_syntax(),
            AnyTomlString::TomlMultilineLiteralString(it) => it.into_syntax(),
        }
    }
}
impl From<AnyTomlString> for SyntaxElement {
    fn from(n: AnyTomlString) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TomlBogusTableHeader> for AnyTomlTableHeader {
    fn from(node: TomlBogusTableHeader) -> Self {
        Self::TomlBogusTableHeader(node)
    }
}
impl From<TomlTableHeader> for AnyTomlTableHeader {
    fn from(node: TomlTableHeader) -> Self {
        Self::TomlTableHeader(node)
    }
}
impl AstNode for AnyTomlTableHeader {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        TomlBogusTableHeader::KIND_SET.union(TomlTableHeader::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, TOML_BOGUS_TABLE_HEADER | TOML_TABLE_HEADER)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TOML_BOGUS_TABLE_HEADER => Self::TomlBogusTableHeader(TomlBogusTableHeader { syntax }),
            TOML_TABLE_HEADER => Self::TomlTableHeader(TomlTableHeader { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::TomlBogusTableHeader(it) => it.syntax(),
            Self::TomlTableHeader(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::TomlBogusTableHeader(it) => it.into_syntax(),
            Self::TomlTableHeader(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyTomlTableHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TomlBogusTableHeader(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlTableHeader(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyTomlTableHeader> for SyntaxNode {
    fn from(n: AnyTomlTableHeader) -> Self {
        match n {
            AnyTomlTableHeader::TomlBogusTableHeader(it) => it.into_syntax(),
            AnyTomlTableHeader::TomlTableHeader(it) => it.into_syntax(),
        }
    }
}
impl From<AnyTomlTableHeader> for SyntaxElement {
    fn from(n: AnyTomlTableHeader) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TomlArray> for AnyTomlValue {
    fn from(node: TomlArray) -> Self {
        Self::TomlArray(node)
    }
}
impl From<TomlBogusValue> for AnyTomlValue {
    fn from(node: TomlBogusValue) -> Self {
        Self::TomlBogusValue(node)
    }
}
impl From<TomlBoolean> for AnyTomlValue {
    fn from(node: TomlBoolean) -> Self {
        Self::TomlBoolean(node)
    }
}
impl From<TomlDate> for AnyTomlValue {
    fn from(node: TomlDate) -> Self {
        Self::TomlDate(node)
    }
}
impl From<TomlFloat> for AnyTomlValue {
    fn from(node: TomlFloat) -> Self {
        Self::TomlFloat(node)
    }
}
impl From<TomlInlineTable> for AnyTomlValue {
    fn from(node: TomlInlineTable) -> Self {
        Self::TomlInlineTable(node)
    }
}
impl From<TomlInteger> for AnyTomlValue {
    fn from(node: TomlInteger) -> Self {
        Self::TomlInteger(node)
    }
}
impl From<TomlTime> for AnyTomlValue {
    fn from(node: TomlTime) -> Self {
        Self::TomlTime(node)
    }
}
impl AstNode for AnyTomlValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyTomlString::KIND_SET
        .union(TomlArray::KIND_SET)
        .union(TomlBogusValue::KIND_SET)
        .union(TomlBoolean::KIND_SET)
        .union(TomlDate::KIND_SET)
        .union(TomlFloat::KIND_SET)
        .union(TomlInlineTable::KIND_SET)
        .union(TomlInteger::KIND_SET)
        .union(TomlTime::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TOML_ARRAY | TOML_BOGUS_VALUE | TOML_BOOLEAN | TOML_DATE | TOML_FLOAT
            | TOML_INLINE_TABLE | TOML_INTEGER | TOML_TIME => true,
            k if AnyTomlString::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TOML_ARRAY => Self::TomlArray(TomlArray { syntax }),
            TOML_BOGUS_VALUE => Self::TomlBogusValue(TomlBogusValue { syntax }),
            TOML_BOOLEAN => Self::TomlBoolean(TomlBoolean { syntax }),
            TOML_DATE => Self::TomlDate(TomlDate { syntax }),
            TOML_FLOAT => Self::TomlFloat(TomlFloat { syntax }),
            TOML_INLINE_TABLE => Self::TomlInlineTable(TomlInlineTable { syntax }),
            TOML_INTEGER => Self::TomlInteger(TomlInteger { syntax }),
            TOML_TIME => Self::TomlTime(TomlTime { syntax }),
            _ => {
                if let Some(any_toml_string) = AnyTomlString::cast(syntax) {
                    return Some(Self::AnyTomlString(any_toml_string));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::TomlArray(it) => it.syntax(),
            Self::TomlBogusValue(it) => it.syntax(),
            Self::TomlBoolean(it) => it.syntax(),
            Self::TomlDate(it) => it.syntax(),
            Self::TomlFloat(it) => it.syntax(),
            Self::TomlInlineTable(it) => it.syntax(),
            Self::TomlInteger(it) => it.syntax(),
            Self::TomlTime(it) => it.syntax(),
            Self::AnyTomlString(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::TomlArray(it) => it.into_syntax(),
            Self::TomlBogusValue(it) => it.into_syntax(),
            Self::TomlBoolean(it) => it.into_syntax(),
            Self::TomlDate(it) => it.into_syntax(),
            Self::TomlFloat(it) => it.into_syntax(),
            Self::TomlInlineTable(it) => it.into_syntax(),
            Self::TomlInteger(it) => it.into_syntax(),
            Self::TomlTime(it) => it.into_syntax(),
            Self::AnyTomlString(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyTomlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyTomlString(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlArray(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlBogusValue(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlBoolean(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlDate(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlFloat(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlInlineTable(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlInteger(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlTime(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyTomlValue> for SyntaxNode {
    fn from(n: AnyTomlValue) -> Self {
        match n {
            AnyTomlValue::AnyTomlString(it) => it.into_syntax(),
            AnyTomlValue::TomlArray(it) => it.into_syntax(),
            AnyTomlValue::TomlBogusValue(it) => it.into_syntax(),
            AnyTomlValue::TomlBoolean(it) => it.into_syntax(),
            AnyTomlValue::TomlDate(it) => it.into_syntax(),
            AnyTomlValue::TomlFloat(it) => it.into_syntax(),
            AnyTomlValue::TomlInlineTable(it) => it.into_syntax(),
            AnyTomlValue::TomlInteger(it) => it.into_syntax(),
            AnyTomlValue::TomlTime(it) => it.into_syntax(),
        }
    }
}
impl From<AnyTomlValue> for SyntaxElement {
    fn from(n: AnyTomlValue) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyTomlArrayTableHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyTomlKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyTomlKeyFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyTomlMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyTomlRootItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyTomlString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyTomlTableHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyTomlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlArrayTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlArrayTableHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlBareKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlBasicString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlInlineTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlLiteralString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlMultilineBasicString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlMultilineLiteralString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlQuotedKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlTableHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct TomlBogus {
    syntax: SyntaxNode,
}
impl TomlBogus {
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
impl AstNode for TomlBogus {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_BOGUS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_BOGUS
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
impl std::fmt::Debug for TomlBogus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TomlBogus")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<TomlBogus> for SyntaxNode {
    fn from(n: TomlBogus) -> Self {
        n.syntax
    }
}
impl From<TomlBogus> for SyntaxElement {
    fn from(n: TomlBogus) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct TomlBogusKey {
    syntax: SyntaxNode,
}
impl TomlBogusKey {
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
impl AstNode for TomlBogusKey {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_BOGUS_KEY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_BOGUS_KEY
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
impl std::fmt::Debug for TomlBogusKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TomlBogusKey")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<TomlBogusKey> for SyntaxNode {
    fn from(n: TomlBogusKey) -> Self {
        n.syntax
    }
}
impl From<TomlBogusKey> for SyntaxElement {
    fn from(n: TomlBogusKey) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct TomlBogusKeyFragment {
    syntax: SyntaxNode,
}
impl TomlBogusKeyFragment {
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
impl AstNode for TomlBogusKeyFragment {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_BOGUS_KEY_FRAGMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_BOGUS_KEY_FRAGMENT
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
impl std::fmt::Debug for TomlBogusKeyFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TomlBogusKeyFragment")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<TomlBogusKeyFragment> for SyntaxNode {
    fn from(n: TomlBogusKeyFragment) -> Self {
        n.syntax
    }
}
impl From<TomlBogusKeyFragment> for SyntaxElement {
    fn from(n: TomlBogusKeyFragment) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct TomlBogusMember {
    syntax: SyntaxNode,
}
impl TomlBogusMember {
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
impl AstNode for TomlBogusMember {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_BOGUS_MEMBER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_BOGUS_MEMBER
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
impl std::fmt::Debug for TomlBogusMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TomlBogusMember")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<TomlBogusMember> for SyntaxNode {
    fn from(n: TomlBogusMember) -> Self {
        n.syntax
    }
}
impl From<TomlBogusMember> for SyntaxElement {
    fn from(n: TomlBogusMember) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct TomlBogusTableHeader {
    syntax: SyntaxNode,
}
impl TomlBogusTableHeader {
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
impl AstNode for TomlBogusTableHeader {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_BOGUS_TABLE_HEADER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_BOGUS_TABLE_HEADER
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
impl std::fmt::Debug for TomlBogusTableHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TomlBogusTableHeader")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<TomlBogusTableHeader> for SyntaxNode {
    fn from(n: TomlBogusTableHeader) -> Self {
        n.syntax
    }
}
impl From<TomlBogusTableHeader> for SyntaxElement {
    fn from(n: TomlBogusTableHeader) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct TomlBogusValue {
    syntax: SyntaxNode,
}
impl TomlBogusValue {
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
impl AstNode for TomlBogusValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_BOGUS_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_BOGUS_VALUE
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
impl std::fmt::Debug for TomlBogusValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TomlBogusValue")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<TomlBogusValue> for SyntaxNode {
    fn from(n: TomlBogusValue) -> Self {
        n.syntax
    }
}
impl From<TomlBogusValue> for SyntaxElement {
    fn from(n: TomlBogusValue) -> Self {
        n.syntax.into()
    }
}
biome_rowan::declare_node_union! { pub AnyTomlBogusNode = TomlBogus | TomlBogusKey | TomlBogusKeyFragment | TomlBogusMember | TomlBogusTableHeader | TomlBogusValue }
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TomlArrayElementList {
    syntax_list: SyntaxList,
}
impl TomlArrayElementList {
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
impl AstNode for TomlArrayElementList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_ARRAY_ELEMENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_ARRAY_ELEMENT_LIST
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
impl Serialize for TomlArrayElementList {
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
impl AstSeparatedList for TomlArrayElementList {
    type Language = Language;
    type Node = AnyTomlValue;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for TomlArrayElementList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TomlArrayElementList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for TomlArrayElementList {
    type Item = SyntaxResult<AnyTomlValue>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyTomlValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &TomlArrayElementList {
    type Item = SyntaxResult<AnyTomlValue>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyTomlValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TomlInlineTableMemberList {
    syntax_list: SyntaxList,
}
impl TomlInlineTableMemberList {
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
impl AstNode for TomlInlineTableMemberList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_INLINE_TABLE_MEMBER_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_INLINE_TABLE_MEMBER_LIST
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
impl Serialize for TomlInlineTableMemberList {
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
impl AstSeparatedList for TomlInlineTableMemberList {
    type Language = Language;
    type Node = AnyTomlMember;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for TomlInlineTableMemberList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TomlInlineTableMemberList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for TomlInlineTableMemberList {
    type Item = SyntaxResult<AnyTomlMember>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyTomlMember>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &TomlInlineTableMemberList {
    type Item = SyntaxResult<AnyTomlMember>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyTomlMember>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TomlKeyFragmentList {
    syntax_list: SyntaxList,
}
impl TomlKeyFragmentList {
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
impl AstNode for TomlKeyFragmentList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_KEY_FRAGMENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_KEY_FRAGMENT_LIST
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
impl Serialize for TomlKeyFragmentList {
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
impl AstSeparatedList for TomlKeyFragmentList {
    type Language = Language;
    type Node = AnyTomlKeyFragment;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for TomlKeyFragmentList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TomlKeyFragmentList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for TomlKeyFragmentList {
    type Item = SyntaxResult<AnyTomlKeyFragment>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyTomlKeyFragment>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &TomlKeyFragmentList {
    type Item = SyntaxResult<AnyTomlKeyFragment>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyTomlKeyFragment>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TomlMemberList {
    syntax_list: SyntaxList,
}
impl TomlMemberList {
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
impl AstNode for TomlMemberList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_MEMBER_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_MEMBER_LIST
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
impl Serialize for TomlMemberList {
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
impl AstNodeList for TomlMemberList {
    type Language = Language;
    type Node = AnyTomlMember;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for TomlMemberList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TomlMemberList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &TomlMemberList {
    type Item = AnyTomlMember;
    type IntoIter = AstNodeListIterator<Language, AnyTomlMember>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for TomlMemberList {
    type Item = AnyTomlMember;
    type IntoIter = AstNodeListIterator<Language, AnyTomlMember>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TomlRootItemList {
    syntax_list: SyntaxList,
}
impl TomlRootItemList {
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
impl AstNode for TomlRootItemList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_ROOT_ITEM_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_ROOT_ITEM_LIST
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
impl Serialize for TomlRootItemList {
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
impl AstNodeList for TomlRootItemList {
    type Language = Language;
    type Node = AnyTomlRootItem;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for TomlRootItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TomlRootItemList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &TomlRootItemList {
    type Item = AnyTomlRootItem;
    type IntoIter = AstNodeListIterator<Language, AnyTomlRootItem>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for TomlRootItemList {
    type Item = AnyTomlRootItem;
    type IntoIter = AstNodeListIterator<Language, AnyTomlRootItem>;
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
