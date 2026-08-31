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
            opening_outer_token: self.opening_outer_token(),
            opening_inner_token: self.opening_inner_token(),
            name: self.name(),
            closing_inner_token: self.closing_inner_token(),
            closing_outer_token: self.closing_outer_token(),
        }
    }
    pub fn opening_outer_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn opening_inner_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<TomlKey> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn closing_inner_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn closing_outer_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
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
    pub opening_outer_token: SyntaxResult<SyntaxToken>,
    pub opening_inner_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<TomlKey>,
    pub closing_inner_token: SyntaxResult<SyntaxToken>,
    pub closing_outer_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlBooleanValue {
    pub(crate) syntax: SyntaxNode,
}
impl TomlBooleanValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlBooleanValueFields {
        TomlBooleanValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TomlBooleanValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlBooleanValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlFloatValue {
    pub(crate) syntax: SyntaxNode,
}
impl TomlFloatValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlFloatValueFields {
        TomlFloatValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TomlFloatValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlFloatValueFields {
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
            elements: self.elements(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn elements(&self) -> TomlInlineTableElementList {
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
    pub elements: TomlInlineTableElementList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlIntegerValue {
    pub(crate) syntax: SyntaxNode,
}
impl TomlIntegerValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlIntegerValueFields {
        TomlIntegerValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TomlIntegerValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlIntegerValueFields {
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
            segments: self.segments(),
        }
    }
    pub fn segments(&self) -> TomlKeySegmentList {
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
    pub segments: TomlKeySegmentList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlKeySegment {
    pub(crate) syntax: SyntaxNode,
}
impl TomlKeySegment {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlKeySegmentFields {
        TomlKeySegmentFields {
            value: self.value(),
        }
    }
    pub fn value(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TomlKeySegment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlKeySegmentFields {
    pub value: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlKeyValue {
    pub(crate) syntax: SyntaxNode,
}
impl TomlKeyValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlKeyValueFields {
        TomlKeyValueFields {
            key: self.key(),
            eq_token: self.eq_token(),
            value: self.value(),
        }
    }
    pub fn key(&self) -> SyntaxResult<TomlKey> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<AnyTomlValue> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for TomlKeyValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlKeyValueFields {
    pub key: SyntaxResult<TomlKey>,
    pub eq_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnyTomlValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlLocalDateTimeValue {
    pub(crate) syntax: SyntaxNode,
}
impl TomlLocalDateTimeValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlLocalDateTimeValueFields {
        TomlLocalDateTimeValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TomlLocalDateTimeValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlLocalDateTimeValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlLocalDateValue {
    pub(crate) syntax: SyntaxNode,
}
impl TomlLocalDateValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlLocalDateValueFields {
        TomlLocalDateValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TomlLocalDateValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlLocalDateValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlLocalTimeValue {
    pub(crate) syntax: SyntaxNode,
}
impl TomlLocalTimeValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlLocalTimeValueFields {
        TomlLocalTimeValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TomlLocalTimeValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlLocalTimeValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlOffsetDateTimeValue {
    pub(crate) syntax: SyntaxNode,
}
impl TomlOffsetDateTimeValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlOffsetDateTimeValueFields {
        TomlOffsetDateTimeValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TomlOffsetDateTimeValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlOffsetDateTimeValueFields {
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
    pub fn items(&self) -> TomlItemList {
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
    pub items: TomlItemList,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TomlStringValue {
    pub(crate) syntax: SyntaxNode,
}
impl TomlStringValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlStringValueFields {
        TomlStringValueFields {
            value: self.value(),
        }
    }
    pub fn value(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TomlStringValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlStringValueFields {
    pub value: SyntaxResult<SyntaxToken>,
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
            l_brack_token: self.l_brack_token(),
            name: self.name(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<TomlKey> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
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
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<TomlKey>,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyTomlInlineTableElement {
    TomlBogus(TomlBogus),
    TomlKeyValue(TomlKeyValue),
}
impl AnyTomlInlineTableElement {
    pub fn as_toml_bogus(&self) -> Option<&TomlBogus> {
        match &self {
            Self::TomlBogus(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_key_value(&self) -> Option<&TomlKeyValue> {
        match &self {
            Self::TomlKeyValue(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyTomlItem {
    TomlArrayTable(TomlArrayTable),
    TomlBogus(TomlBogus),
    TomlKeyValue(TomlKeyValue),
    TomlTable(TomlTable),
}
impl AnyTomlItem {
    pub fn as_toml_array_table(&self) -> Option<&TomlArrayTable> {
        match &self {
            Self::TomlArrayTable(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_bogus(&self) -> Option<&TomlBogus> {
        match &self {
            Self::TomlBogus(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_key_value(&self) -> Option<&TomlKeyValue> {
        match &self {
            Self::TomlKeyValue(item) => Some(item),
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
pub enum AnyTomlValue {
    TomlArray(TomlArray),
    TomlBogusValue(TomlBogusValue),
    TomlBooleanValue(TomlBooleanValue),
    TomlFloatValue(TomlFloatValue),
    TomlInlineTable(TomlInlineTable),
    TomlIntegerValue(TomlIntegerValue),
    TomlLocalDateTimeValue(TomlLocalDateTimeValue),
    TomlLocalDateValue(TomlLocalDateValue),
    TomlLocalTimeValue(TomlLocalTimeValue),
    TomlOffsetDateTimeValue(TomlOffsetDateTimeValue),
    TomlStringValue(TomlStringValue),
}
impl AnyTomlValue {
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
    pub fn as_toml_boolean_value(&self) -> Option<&TomlBooleanValue> {
        match &self {
            Self::TomlBooleanValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_float_value(&self) -> Option<&TomlFloatValue> {
        match &self {
            Self::TomlFloatValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_inline_table(&self) -> Option<&TomlInlineTable> {
        match &self {
            Self::TomlInlineTable(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_integer_value(&self) -> Option<&TomlIntegerValue> {
        match &self {
            Self::TomlIntegerValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_local_date_time_value(&self) -> Option<&TomlLocalDateTimeValue> {
        match &self {
            Self::TomlLocalDateTimeValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_local_date_value(&self) -> Option<&TomlLocalDateValue> {
        match &self {
            Self::TomlLocalDateValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_local_time_value(&self) -> Option<&TomlLocalTimeValue> {
        match &self {
            Self::TomlLocalTimeValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_offset_date_time_value(&self) -> Option<&TomlOffsetDateTimeValue> {
        match &self {
            Self::TomlOffsetDateTimeValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_string_value(&self) -> Option<&TomlStringValue> {
        match &self {
            Self::TomlStringValue(item) => Some(item),
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
                .field(
                    "opening_outer_token",
                    &support::DebugSyntaxResult(self.opening_outer_token()),
                )
                .field(
                    "opening_inner_token",
                    &support::DebugSyntaxResult(self.opening_inner_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "closing_inner_token",
                    &support::DebugSyntaxResult(self.closing_inner_token()),
                )
                .field(
                    "closing_outer_token",
                    &support::DebugSyntaxResult(self.closing_outer_token()),
                )
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
impl AstNode for TomlBooleanValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_BOOLEAN_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_BOOLEAN_VALUE
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
impl std::fmt::Debug for TomlBooleanValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlBooleanValue")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("TomlBooleanValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlBooleanValue> for SyntaxNode {
    fn from(n: TomlBooleanValue) -> Self {
        n.syntax
    }
}
impl From<TomlBooleanValue> for SyntaxElement {
    fn from(n: TomlBooleanValue) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TomlFloatValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_FLOAT_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_FLOAT_VALUE
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
impl std::fmt::Debug for TomlFloatValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlFloatValue")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("TomlFloatValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlFloatValue> for SyntaxNode {
    fn from(n: TomlFloatValue) -> Self {
        n.syntax
    }
}
impl From<TomlFloatValue> for SyntaxElement {
    fn from(n: TomlFloatValue) -> Self {
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
                .field("elements", &self.elements())
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
impl AstNode for TomlIntegerValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_INTEGER_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_INTEGER_VALUE
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
impl std::fmt::Debug for TomlIntegerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlIntegerValue")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("TomlIntegerValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlIntegerValue> for SyntaxNode {
    fn from(n: TomlIntegerValue) -> Self {
        n.syntax
    }
}
impl From<TomlIntegerValue> for SyntaxElement {
    fn from(n: TomlIntegerValue) -> Self {
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
                .field("segments", &self.segments())
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
impl AstNode for TomlKeySegment {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_KEY_SEGMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_KEY_SEGMENT
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
impl std::fmt::Debug for TomlKeySegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlKeySegment")
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("TomlKeySegment").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlKeySegment> for SyntaxNode {
    fn from(n: TomlKeySegment) -> Self {
        n.syntax
    }
}
impl From<TomlKeySegment> for SyntaxElement {
    fn from(n: TomlKeySegment) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TomlKeyValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_KEY_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_KEY_VALUE
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
impl std::fmt::Debug for TomlKeyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlKeyValue")
                .field("key", &support::DebugSyntaxResult(self.key()))
                .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("TomlKeyValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlKeyValue> for SyntaxNode {
    fn from(n: TomlKeyValue) -> Self {
        n.syntax
    }
}
impl From<TomlKeyValue> for SyntaxElement {
    fn from(n: TomlKeyValue) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TomlLocalDateTimeValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_LOCAL_DATE_TIME_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_LOCAL_DATE_TIME_VALUE
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
impl std::fmt::Debug for TomlLocalDateTimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlLocalDateTimeValue")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("TomlLocalDateTimeValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlLocalDateTimeValue> for SyntaxNode {
    fn from(n: TomlLocalDateTimeValue) -> Self {
        n.syntax
    }
}
impl From<TomlLocalDateTimeValue> for SyntaxElement {
    fn from(n: TomlLocalDateTimeValue) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TomlLocalDateValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_LOCAL_DATE_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_LOCAL_DATE_VALUE
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
impl std::fmt::Debug for TomlLocalDateValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlLocalDateValue")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("TomlLocalDateValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlLocalDateValue> for SyntaxNode {
    fn from(n: TomlLocalDateValue) -> Self {
        n.syntax
    }
}
impl From<TomlLocalDateValue> for SyntaxElement {
    fn from(n: TomlLocalDateValue) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TomlLocalTimeValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_LOCAL_TIME_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_LOCAL_TIME_VALUE
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
impl std::fmt::Debug for TomlLocalTimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlLocalTimeValue")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("TomlLocalTimeValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlLocalTimeValue> for SyntaxNode {
    fn from(n: TomlLocalTimeValue) -> Self {
        n.syntax
    }
}
impl From<TomlLocalTimeValue> for SyntaxElement {
    fn from(n: TomlLocalTimeValue) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TomlOffsetDateTimeValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_OFFSET_DATE_TIME_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_OFFSET_DATE_TIME_VALUE
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
impl std::fmt::Debug for TomlOffsetDateTimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlOffsetDateTimeValue")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("TomlOffsetDateTimeValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlOffsetDateTimeValue> for SyntaxNode {
    fn from(n: TomlOffsetDateTimeValue) -> Self {
        n.syntax
    }
}
impl From<TomlOffsetDateTimeValue> for SyntaxElement {
    fn from(n: TomlOffsetDateTimeValue) -> Self {
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
impl AstNode for TomlStringValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_STRING_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_STRING_VALUE
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
impl std::fmt::Debug for TomlStringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlStringValue")
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("TomlStringValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlStringValue> for SyntaxNode {
    fn from(n: TomlStringValue) -> Self {
        n.syntax
    }
}
impl From<TomlStringValue> for SyntaxElement {
    fn from(n: TomlStringValue) -> Self {
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
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
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
impl From<TomlBogus> for AnyTomlInlineTableElement {
    fn from(node: TomlBogus) -> Self {
        Self::TomlBogus(node)
    }
}
impl From<TomlKeyValue> for AnyTomlInlineTableElement {
    fn from(node: TomlKeyValue) -> Self {
        Self::TomlKeyValue(node)
    }
}
impl AstNode for AnyTomlInlineTableElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = TomlBogus::KIND_SET.union(TomlKeyValue::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, TOML_BOGUS | TOML_KEY_VALUE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TOML_BOGUS => Self::TomlBogus(TomlBogus { syntax }),
            TOML_KEY_VALUE => Self::TomlKeyValue(TomlKeyValue { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::TomlBogus(it) => it.syntax(),
            Self::TomlKeyValue(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::TomlBogus(it) => it.into_syntax(),
            Self::TomlKeyValue(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyTomlInlineTableElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TomlBogus(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlKeyValue(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyTomlInlineTableElement> for SyntaxNode {
    fn from(n: AnyTomlInlineTableElement) -> Self {
        match n {
            AnyTomlInlineTableElement::TomlBogus(it) => it.into_syntax(),
            AnyTomlInlineTableElement::TomlKeyValue(it) => it.into_syntax(),
        }
    }
}
impl From<AnyTomlInlineTableElement> for SyntaxElement {
    fn from(n: AnyTomlInlineTableElement) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TomlArrayTable> for AnyTomlItem {
    fn from(node: TomlArrayTable) -> Self {
        Self::TomlArrayTable(node)
    }
}
impl From<TomlBogus> for AnyTomlItem {
    fn from(node: TomlBogus) -> Self {
        Self::TomlBogus(node)
    }
}
impl From<TomlKeyValue> for AnyTomlItem {
    fn from(node: TomlKeyValue) -> Self {
        Self::TomlKeyValue(node)
    }
}
impl From<TomlTable> for AnyTomlItem {
    fn from(node: TomlTable) -> Self {
        Self::TomlTable(node)
    }
}
impl AstNode for AnyTomlItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = TomlArrayTable::KIND_SET
        .union(TomlBogus::KIND_SET)
        .union(TomlKeyValue::KIND_SET)
        .union(TomlTable::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            TOML_ARRAY_TABLE | TOML_BOGUS | TOML_KEY_VALUE | TOML_TABLE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TOML_ARRAY_TABLE => Self::TomlArrayTable(TomlArrayTable { syntax }),
            TOML_BOGUS => Self::TomlBogus(TomlBogus { syntax }),
            TOML_KEY_VALUE => Self::TomlKeyValue(TomlKeyValue { syntax }),
            TOML_TABLE => Self::TomlTable(TomlTable { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::TomlArrayTable(it) => it.syntax(),
            Self::TomlBogus(it) => it.syntax(),
            Self::TomlKeyValue(it) => it.syntax(),
            Self::TomlTable(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::TomlArrayTable(it) => it.into_syntax(),
            Self::TomlBogus(it) => it.into_syntax(),
            Self::TomlKeyValue(it) => it.into_syntax(),
            Self::TomlTable(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyTomlItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TomlArrayTable(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlBogus(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlKeyValue(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlTable(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyTomlItem> for SyntaxNode {
    fn from(n: AnyTomlItem) -> Self {
        match n {
            AnyTomlItem::TomlArrayTable(it) => it.into_syntax(),
            AnyTomlItem::TomlBogus(it) => it.into_syntax(),
            AnyTomlItem::TomlKeyValue(it) => it.into_syntax(),
            AnyTomlItem::TomlTable(it) => it.into_syntax(),
        }
    }
}
impl From<AnyTomlItem> for SyntaxElement {
    fn from(n: AnyTomlItem) -> Self {
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
impl From<TomlBooleanValue> for AnyTomlValue {
    fn from(node: TomlBooleanValue) -> Self {
        Self::TomlBooleanValue(node)
    }
}
impl From<TomlFloatValue> for AnyTomlValue {
    fn from(node: TomlFloatValue) -> Self {
        Self::TomlFloatValue(node)
    }
}
impl From<TomlInlineTable> for AnyTomlValue {
    fn from(node: TomlInlineTable) -> Self {
        Self::TomlInlineTable(node)
    }
}
impl From<TomlIntegerValue> for AnyTomlValue {
    fn from(node: TomlIntegerValue) -> Self {
        Self::TomlIntegerValue(node)
    }
}
impl From<TomlLocalDateTimeValue> for AnyTomlValue {
    fn from(node: TomlLocalDateTimeValue) -> Self {
        Self::TomlLocalDateTimeValue(node)
    }
}
impl From<TomlLocalDateValue> for AnyTomlValue {
    fn from(node: TomlLocalDateValue) -> Self {
        Self::TomlLocalDateValue(node)
    }
}
impl From<TomlLocalTimeValue> for AnyTomlValue {
    fn from(node: TomlLocalTimeValue) -> Self {
        Self::TomlLocalTimeValue(node)
    }
}
impl From<TomlOffsetDateTimeValue> for AnyTomlValue {
    fn from(node: TomlOffsetDateTimeValue) -> Self {
        Self::TomlOffsetDateTimeValue(node)
    }
}
impl From<TomlStringValue> for AnyTomlValue {
    fn from(node: TomlStringValue) -> Self {
        Self::TomlStringValue(node)
    }
}
impl AstNode for AnyTomlValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = TomlArray::KIND_SET
        .union(TomlBogusValue::KIND_SET)
        .union(TomlBooleanValue::KIND_SET)
        .union(TomlFloatValue::KIND_SET)
        .union(TomlInlineTable::KIND_SET)
        .union(TomlIntegerValue::KIND_SET)
        .union(TomlLocalDateTimeValue::KIND_SET)
        .union(TomlLocalDateValue::KIND_SET)
        .union(TomlLocalTimeValue::KIND_SET)
        .union(TomlOffsetDateTimeValue::KIND_SET)
        .union(TomlStringValue::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            TOML_ARRAY
                | TOML_BOGUS_VALUE
                | TOML_BOOLEAN_VALUE
                | TOML_FLOAT_VALUE
                | TOML_INLINE_TABLE
                | TOML_INTEGER_VALUE
                | TOML_LOCAL_DATE_TIME_VALUE
                | TOML_LOCAL_DATE_VALUE
                | TOML_LOCAL_TIME_VALUE
                | TOML_OFFSET_DATE_TIME_VALUE
                | TOML_STRING_VALUE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TOML_ARRAY => Self::TomlArray(TomlArray { syntax }),
            TOML_BOGUS_VALUE => Self::TomlBogusValue(TomlBogusValue { syntax }),
            TOML_BOOLEAN_VALUE => Self::TomlBooleanValue(TomlBooleanValue { syntax }),
            TOML_FLOAT_VALUE => Self::TomlFloatValue(TomlFloatValue { syntax }),
            TOML_INLINE_TABLE => Self::TomlInlineTable(TomlInlineTable { syntax }),
            TOML_INTEGER_VALUE => Self::TomlIntegerValue(TomlIntegerValue { syntax }),
            TOML_LOCAL_DATE_TIME_VALUE => {
                Self::TomlLocalDateTimeValue(TomlLocalDateTimeValue { syntax })
            }
            TOML_LOCAL_DATE_VALUE => Self::TomlLocalDateValue(TomlLocalDateValue { syntax }),
            TOML_LOCAL_TIME_VALUE => Self::TomlLocalTimeValue(TomlLocalTimeValue { syntax }),
            TOML_OFFSET_DATE_TIME_VALUE => {
                Self::TomlOffsetDateTimeValue(TomlOffsetDateTimeValue { syntax })
            }
            TOML_STRING_VALUE => Self::TomlStringValue(TomlStringValue { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::TomlArray(it) => it.syntax(),
            Self::TomlBogusValue(it) => it.syntax(),
            Self::TomlBooleanValue(it) => it.syntax(),
            Self::TomlFloatValue(it) => it.syntax(),
            Self::TomlInlineTable(it) => it.syntax(),
            Self::TomlIntegerValue(it) => it.syntax(),
            Self::TomlLocalDateTimeValue(it) => it.syntax(),
            Self::TomlLocalDateValue(it) => it.syntax(),
            Self::TomlLocalTimeValue(it) => it.syntax(),
            Self::TomlOffsetDateTimeValue(it) => it.syntax(),
            Self::TomlStringValue(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::TomlArray(it) => it.into_syntax(),
            Self::TomlBogusValue(it) => it.into_syntax(),
            Self::TomlBooleanValue(it) => it.into_syntax(),
            Self::TomlFloatValue(it) => it.into_syntax(),
            Self::TomlInlineTable(it) => it.into_syntax(),
            Self::TomlIntegerValue(it) => it.into_syntax(),
            Self::TomlLocalDateTimeValue(it) => it.into_syntax(),
            Self::TomlLocalDateValue(it) => it.into_syntax(),
            Self::TomlLocalTimeValue(it) => it.into_syntax(),
            Self::TomlOffsetDateTimeValue(it) => it.into_syntax(),
            Self::TomlStringValue(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyTomlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TomlArray(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlBogusValue(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlBooleanValue(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlFloatValue(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlInlineTable(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlIntegerValue(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlLocalDateTimeValue(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlLocalDateValue(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlLocalTimeValue(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlOffsetDateTimeValue(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlStringValue(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyTomlValue> for SyntaxNode {
    fn from(n: AnyTomlValue) -> Self {
        match n {
            AnyTomlValue::TomlArray(it) => it.into_syntax(),
            AnyTomlValue::TomlBogusValue(it) => it.into_syntax(),
            AnyTomlValue::TomlBooleanValue(it) => it.into_syntax(),
            AnyTomlValue::TomlFloatValue(it) => it.into_syntax(),
            AnyTomlValue::TomlInlineTable(it) => it.into_syntax(),
            AnyTomlValue::TomlIntegerValue(it) => it.into_syntax(),
            AnyTomlValue::TomlLocalDateTimeValue(it) => it.into_syntax(),
            AnyTomlValue::TomlLocalDateValue(it) => it.into_syntax(),
            AnyTomlValue::TomlLocalTimeValue(it) => it.into_syntax(),
            AnyTomlValue::TomlOffsetDateTimeValue(it) => it.into_syntax(),
            AnyTomlValue::TomlStringValue(it) => it.into_syntax(),
        }
    }
}
impl From<AnyTomlValue> for SyntaxElement {
    fn from(n: AnyTomlValue) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyTomlInlineTableElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyTomlItem {
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
impl std::fmt::Display for TomlBooleanValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlFloatValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlInlineTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlIntegerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlKeySegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlKeyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlLocalDateTimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlLocalDateValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlLocalTimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlOffsetDateTimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlStringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlTable {
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
biome_rowan::declare_node_union! { pub AnyTomlBogusNode = TomlBogus | TomlBogusValue }
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
pub struct TomlInlineTableElementList {
    syntax_list: SyntaxList,
}
impl TomlInlineTableElementList {
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
impl AstNode for TomlInlineTableElementList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_INLINE_TABLE_ELEMENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_INLINE_TABLE_ELEMENT_LIST
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
impl Serialize for TomlInlineTableElementList {
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
impl AstSeparatedList for TomlInlineTableElementList {
    type Language = Language;
    type Node = AnyTomlInlineTableElement;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for TomlInlineTableElementList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TomlInlineTableElementList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for TomlInlineTableElementList {
    type Item = SyntaxResult<AnyTomlInlineTableElement>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyTomlInlineTableElement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &TomlInlineTableElementList {
    type Item = SyntaxResult<AnyTomlInlineTableElement>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyTomlInlineTableElement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TomlItemList {
    syntax_list: SyntaxList,
}
impl TomlItemList {
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
impl AstNode for TomlItemList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_ITEM_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_ITEM_LIST
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
impl Serialize for TomlItemList {
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
impl AstNodeList for TomlItemList {
    type Language = Language;
    type Node = AnyTomlItem;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for TomlItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TomlItemList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &TomlItemList {
    type Item = AnyTomlItem;
    type IntoIter = AstNodeListIterator<Language, AnyTomlItem>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for TomlItemList {
    type Item = AnyTomlItem;
    type IntoIter = AstNodeListIterator<Language, AnyTomlItem>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TomlKeySegmentList {
    syntax_list: SyntaxList,
}
impl TomlKeySegmentList {
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
impl AstNode for TomlKeySegmentList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_KEY_SEGMENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_KEY_SEGMENT_LIST
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
impl Serialize for TomlKeySegmentList {
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
impl AstSeparatedList for TomlKeySegmentList {
    type Language = Language;
    type Node = TomlKeySegment;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for TomlKeySegmentList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TomlKeySegmentList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for TomlKeySegmentList {
    type Item = SyntaxResult<TomlKeySegment>;
    type IntoIter = AstSeparatedListNodesIterator<Language, TomlKeySegment>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &TomlKeySegmentList {
    type Item = SyntaxResult<TomlKeySegment>;
    type IntoIter = AstSeparatedListNodesIterator<Language, TomlKeySegment>;
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
