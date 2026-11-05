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
            l_double_brack_token: self.l_double_brack_token(),
            key: self.key(),
            r_double_brack_token: self.r_double_brack_token(),
            entries: self.entries(),
        }
    }
    pub fn l_double_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn key(&self) -> SyntaxResult<TomlKey> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_double_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn entries(&self) -> TomlKeyValueList {
        support::list(&self.syntax, 3usize)
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
    pub l_double_brack_token: SyntaxResult<SyntaxToken>,
    pub key: SyntaxResult<TomlKey>,
    pub r_double_brack_token: SyntaxResult<SyntaxToken>,
    pub entries: TomlKeyValueList,
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
pub struct TomlDateTime {
    pub(crate) syntax: SyntaxNode,
}
impl TomlDateTime {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlDateTimeFields {
        TomlDateTimeFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TomlDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlDateTimeFields {
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
            entries: self.entries(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn entries(&self) -> TomlInlineTableEntryList {
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
    pub entries: TomlInlineTableEntryList,
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
            parts: self.parts(),
        }
    }
    pub fn parts(&self) -> TomlKeyPartList {
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
    pub parts: TomlKeyPartList,
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
pub struct TomlString {
    pub(crate) syntax: SyntaxNode,
}
impl TomlString {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TomlStringFields {
        TomlStringFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TomlString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TomlStringFields {
    pub value_token: SyntaxResult<SyntaxToken>,
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
            key: self.key(),
            r_brack_token: self.r_brack_token(),
            entries: self.entries(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn key(&self) -> SyntaxResult<TomlKey> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn entries(&self) -> TomlKeyValueList {
        support::list(&self.syntax, 3usize)
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
    pub key: SyntaxResult<TomlKey>,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
    pub entries: TomlKeyValueList,
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyTomlKeyPart {
    TomlBareKey(TomlBareKey),
    TomlBogus(TomlBogus),
    TomlQuotedKey(TomlQuotedKey),
}
impl AnyTomlKeyPart {
    pub fn as_toml_bare_key(&self) -> Option<&TomlBareKey> {
        match &self {
            Self::TomlBareKey(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_bogus(&self) -> Option<&TomlBogus> {
        match &self {
            Self::TomlBogus(item) => Some(item),
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
pub enum AnyTomlKeyValue {
    TomlBogus(TomlBogus),
    TomlKeyValue(TomlKeyValue),
}
impl AnyTomlKeyValue {
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
pub enum AnyTomlRootItem {
    TomlArrayTable(TomlArrayTable),
    TomlBogus(TomlBogus),
    TomlKeyValue(TomlKeyValue),
    TomlTable(TomlTable),
}
impl AnyTomlRootItem {
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
    TomlBoolean(TomlBoolean),
    TomlDateTime(TomlDateTime),
    TomlFloat(TomlFloat),
    TomlInlineTable(TomlInlineTable),
    TomlInteger(TomlInteger),
    TomlString(TomlString),
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
    pub fn as_toml_boolean(&self) -> Option<&TomlBoolean> {
        match &self {
            Self::TomlBoolean(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_toml_date_time(&self) -> Option<&TomlDateTime> {
        match &self {
            Self::TomlDateTime(item) => Some(item),
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
    pub fn as_toml_string(&self) -> Option<&TomlString> {
        match &self {
            Self::TomlString(item) => Some(item),
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
                    "l_double_brack_token",
                    &support::DebugSyntaxResult(self.l_double_brack_token()),
                )
                .field("key", &support::DebugSyntaxResult(self.key()))
                .field(
                    "r_double_brack_token",
                    &support::DebugSyntaxResult(self.r_double_brack_token()),
                )
                .field("entries", &self.entries())
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
impl AstNode for TomlDateTime {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_DATE_TIME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_DATE_TIME
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
impl std::fmt::Debug for TomlDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlDateTime")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("TomlDateTime").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlDateTime> for SyntaxNode {
    fn from(n: TomlDateTime) -> Self {
        n.syntax
    }
}
impl From<TomlDateTime> for SyntaxElement {
    fn from(n: TomlDateTime) -> Self {
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
                .field("entries", &self.entries())
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
                .field("parts", &self.parts())
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
impl AstNode for TomlString {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_STRING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_STRING
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
impl std::fmt::Debug for TomlString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TomlString")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("TomlString").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TomlString> for SyntaxNode {
    fn from(n: TomlString) -> Self {
        n.syntax
    }
}
impl From<TomlString> for SyntaxElement {
    fn from(n: TomlString) -> Self {
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
                .field("key", &support::DebugSyntaxResult(self.key()))
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .field("entries", &self.entries())
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
impl From<TomlBareKey> for AnyTomlKeyPart {
    fn from(node: TomlBareKey) -> Self {
        Self::TomlBareKey(node)
    }
}
impl From<TomlBogus> for AnyTomlKeyPart {
    fn from(node: TomlBogus) -> Self {
        Self::TomlBogus(node)
    }
}
impl From<TomlQuotedKey> for AnyTomlKeyPart {
    fn from(node: TomlQuotedKey) -> Self {
        Self::TomlQuotedKey(node)
    }
}
impl AstNode for AnyTomlKeyPart {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = TomlBareKey::KIND_SET
        .union(TomlBogus::KIND_SET)
        .union(TomlQuotedKey::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, TOML_BARE_KEY | TOML_BOGUS | TOML_QUOTED_KEY)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TOML_BARE_KEY => Self::TomlBareKey(TomlBareKey { syntax }),
            TOML_BOGUS => Self::TomlBogus(TomlBogus { syntax }),
            TOML_QUOTED_KEY => Self::TomlQuotedKey(TomlQuotedKey { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::TomlBareKey(it) => it.syntax(),
            Self::TomlBogus(it) => it.syntax(),
            Self::TomlQuotedKey(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::TomlBareKey(it) => it.into_syntax(),
            Self::TomlBogus(it) => it.into_syntax(),
            Self::TomlQuotedKey(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyTomlKeyPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TomlBareKey(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlBogus(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlQuotedKey(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyTomlKeyPart> for SyntaxNode {
    fn from(n: AnyTomlKeyPart) -> Self {
        match n {
            AnyTomlKeyPart::TomlBareKey(it) => it.into_syntax(),
            AnyTomlKeyPart::TomlBogus(it) => it.into_syntax(),
            AnyTomlKeyPart::TomlQuotedKey(it) => it.into_syntax(),
        }
    }
}
impl From<AnyTomlKeyPart> for SyntaxElement {
    fn from(n: AnyTomlKeyPart) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TomlBogus> for AnyTomlKeyValue {
    fn from(node: TomlBogus) -> Self {
        Self::TomlBogus(node)
    }
}
impl From<TomlKeyValue> for AnyTomlKeyValue {
    fn from(node: TomlKeyValue) -> Self {
        Self::TomlKeyValue(node)
    }
}
impl AstNode for AnyTomlKeyValue {
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
impl std::fmt::Debug for AnyTomlKeyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TomlBogus(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlKeyValue(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyTomlKeyValue> for SyntaxNode {
    fn from(n: AnyTomlKeyValue) -> Self {
        match n {
            AnyTomlKeyValue::TomlBogus(it) => it.into_syntax(),
            AnyTomlKeyValue::TomlKeyValue(it) => it.into_syntax(),
        }
    }
}
impl From<AnyTomlKeyValue> for SyntaxElement {
    fn from(n: AnyTomlKeyValue) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TomlArrayTable> for AnyTomlRootItem {
    fn from(node: TomlArrayTable) -> Self {
        Self::TomlArrayTable(node)
    }
}
impl From<TomlBogus> for AnyTomlRootItem {
    fn from(node: TomlBogus) -> Self {
        Self::TomlBogus(node)
    }
}
impl From<TomlKeyValue> for AnyTomlRootItem {
    fn from(node: TomlKeyValue) -> Self {
        Self::TomlKeyValue(node)
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
impl std::fmt::Debug for AnyTomlRootItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TomlArrayTable(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlBogus(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlKeyValue(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlTable(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyTomlRootItem> for SyntaxNode {
    fn from(n: AnyTomlRootItem) -> Self {
        match n {
            AnyTomlRootItem::TomlArrayTable(it) => it.into_syntax(),
            AnyTomlRootItem::TomlBogus(it) => it.into_syntax(),
            AnyTomlRootItem::TomlKeyValue(it) => it.into_syntax(),
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
impl From<TomlDateTime> for AnyTomlValue {
    fn from(node: TomlDateTime) -> Self {
        Self::TomlDateTime(node)
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
impl From<TomlString> for AnyTomlValue {
    fn from(node: TomlString) -> Self {
        Self::TomlString(node)
    }
}
impl AstNode for AnyTomlValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = TomlArray::KIND_SET
        .union(TomlBogusValue::KIND_SET)
        .union(TomlBoolean::KIND_SET)
        .union(TomlDateTime::KIND_SET)
        .union(TomlFloat::KIND_SET)
        .union(TomlInlineTable::KIND_SET)
        .union(TomlInteger::KIND_SET)
        .union(TomlString::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            TOML_ARRAY
                | TOML_BOGUS_VALUE
                | TOML_BOOLEAN
                | TOML_DATE_TIME
                | TOML_FLOAT
                | TOML_INLINE_TABLE
                | TOML_INTEGER
                | TOML_STRING
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TOML_ARRAY => Self::TomlArray(TomlArray { syntax }),
            TOML_BOGUS_VALUE => Self::TomlBogusValue(TomlBogusValue { syntax }),
            TOML_BOOLEAN => Self::TomlBoolean(TomlBoolean { syntax }),
            TOML_DATE_TIME => Self::TomlDateTime(TomlDateTime { syntax }),
            TOML_FLOAT => Self::TomlFloat(TomlFloat { syntax }),
            TOML_INLINE_TABLE => Self::TomlInlineTable(TomlInlineTable { syntax }),
            TOML_INTEGER => Self::TomlInteger(TomlInteger { syntax }),
            TOML_STRING => Self::TomlString(TomlString { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::TomlArray(it) => it.syntax(),
            Self::TomlBogusValue(it) => it.syntax(),
            Self::TomlBoolean(it) => it.syntax(),
            Self::TomlDateTime(it) => it.syntax(),
            Self::TomlFloat(it) => it.syntax(),
            Self::TomlInlineTable(it) => it.syntax(),
            Self::TomlInteger(it) => it.syntax(),
            Self::TomlString(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::TomlArray(it) => it.into_syntax(),
            Self::TomlBogusValue(it) => it.into_syntax(),
            Self::TomlBoolean(it) => it.into_syntax(),
            Self::TomlDateTime(it) => it.into_syntax(),
            Self::TomlFloat(it) => it.into_syntax(),
            Self::TomlInlineTable(it) => it.into_syntax(),
            Self::TomlInteger(it) => it.into_syntax(),
            Self::TomlString(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyTomlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TomlArray(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlBogusValue(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlBoolean(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlDateTime(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlFloat(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlInlineTable(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlInteger(it) => std::fmt::Debug::fmt(it, f),
            Self::TomlString(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyTomlValue> for SyntaxNode {
    fn from(n: AnyTomlValue) -> Self {
        match n {
            AnyTomlValue::TomlArray(it) => it.into_syntax(),
            AnyTomlValue::TomlBogusValue(it) => it.into_syntax(),
            AnyTomlValue::TomlBoolean(it) => it.into_syntax(),
            AnyTomlValue::TomlDateTime(it) => it.into_syntax(),
            AnyTomlValue::TomlFloat(it) => it.into_syntax(),
            AnyTomlValue::TomlInlineTable(it) => it.into_syntax(),
            AnyTomlValue::TomlInteger(it) => it.into_syntax(),
            AnyTomlValue::TomlString(it) => it.into_syntax(),
        }
    }
}
impl From<AnyTomlValue> for SyntaxElement {
    fn from(n: AnyTomlValue) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyTomlKeyPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyTomlKeyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyTomlRootItem {
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
impl std::fmt::Display for TomlBareKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TomlDateTime {
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
impl std::fmt::Display for TomlKeyValue {
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
impl std::fmt::Display for TomlString {
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
pub struct TomlInlineTableEntryList {
    syntax_list: SyntaxList,
}
impl TomlInlineTableEntryList {
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
impl AstNode for TomlInlineTableEntryList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_INLINE_TABLE_ENTRY_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_INLINE_TABLE_ENTRY_LIST
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
impl Serialize for TomlInlineTableEntryList {
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
impl AstSeparatedList for TomlInlineTableEntryList {
    type Language = Language;
    type Node = AnyTomlKeyValue;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for TomlInlineTableEntryList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TomlInlineTableEntryList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for TomlInlineTableEntryList {
    type Item = SyntaxResult<AnyTomlKeyValue>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyTomlKeyValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &TomlInlineTableEntryList {
    type Item = SyntaxResult<AnyTomlKeyValue>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyTomlKeyValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TomlKeyPartList {
    syntax_list: SyntaxList,
}
impl TomlKeyPartList {
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
impl AstNode for TomlKeyPartList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_KEY_PART_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_KEY_PART_LIST
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
impl Serialize for TomlKeyPartList {
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
impl AstSeparatedList for TomlKeyPartList {
    type Language = Language;
    type Node = AnyTomlKeyPart;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for TomlKeyPartList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TomlKeyPartList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for TomlKeyPartList {
    type Item = SyntaxResult<AnyTomlKeyPart>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyTomlKeyPart>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &TomlKeyPartList {
    type Item = SyntaxResult<AnyTomlKeyPart>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyTomlKeyPart>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TomlKeyValueList {
    syntax_list: SyntaxList,
}
impl TomlKeyValueList {
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
impl AstNode for TomlKeyValueList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TOML_KEY_VALUE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOML_KEY_VALUE_LIST
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
impl Serialize for TomlKeyValueList {
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
impl AstNodeList for TomlKeyValueList {
    type Language = Language;
    type Node = AnyTomlKeyValue;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for TomlKeyValueList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TomlKeyValueList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &TomlKeyValueList {
    type Item = AnyTomlKeyValue;
    type IntoIter = AstNodeListIterator<Language, AnyTomlKeyValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for TomlKeyValueList {
    type Item = AnyTomlKeyValue;
    type IntoIter = AstNodeListIterator<Language, AnyTomlKeyValue>;
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
