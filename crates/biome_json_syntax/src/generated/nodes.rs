//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(dead_code)]
#![allow(unused)]
use crate::{
    JsonLanguage as Language, JsonSyntaxElement as SyntaxElement,
    JsonSyntaxElementChildren as SyntaxElementChildren,
    JsonSyntaxKind::{self as SyntaxKind, *},
    JsonSyntaxList as SyntaxList, JsonSyntaxNode as SyntaxNode, JsonSyntaxToken as SyntaxToken,
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
pub struct JsonArrayValue {
    pub(crate) syntax: SyntaxNode,
}
impl JsonArrayValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> JsonArrayValueFields {
        JsonArrayValueFields {
            l_brack_token: self.l_brack_token(),
            elements: self.elements(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn elements(&self) -> JsonArrayElementList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for JsonArrayValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct JsonArrayValueFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub elements: JsonArrayElementList,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonBooleanValue {
    pub(crate) syntax: SyntaxNode,
}
impl JsonBooleanValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> JsonBooleanValueFields {
        JsonBooleanValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for JsonBooleanValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct JsonBooleanValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonMember {
    pub(crate) syntax: SyntaxNode,
}
impl JsonMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> JsonMemberFields {
        JsonMemberFields {
            name: self.name(),
            colon_token: self.colon_token(),
            value: self.value(),
        }
    }
    pub fn name(&self) -> SyntaxResult<AnyJsonMemberName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<AnyJsonValue> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for JsonMember {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct JsonMemberFields {
    pub name: SyntaxResult<AnyJsonMemberName>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnyJsonValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonMemberName {
    pub(crate) syntax: SyntaxNode,
}
impl JsonMemberName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> JsonMemberNameFields {
        JsonMemberNameFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for JsonMemberName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct JsonMemberNameFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonMetavariable {
    pub(crate) syntax: SyntaxNode,
}
impl JsonMetavariable {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> JsonMetavariableFields {
        JsonMetavariableFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for JsonMetavariable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct JsonMetavariableFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonNullValue {
    pub(crate) syntax: SyntaxNode,
}
impl JsonNullValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> JsonNullValueFields {
        JsonNullValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for JsonNullValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct JsonNullValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonNumberValue {
    pub(crate) syntax: SyntaxNode,
}
impl JsonNumberValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> JsonNumberValueFields {
        JsonNumberValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for JsonNumberValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct JsonNumberValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonObjectValue {
    pub(crate) syntax: SyntaxNode,
}
impl JsonObjectValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> JsonObjectValueFields {
        JsonObjectValueFields {
            l_curly_token: self.l_curly_token(),
            json_member_list: self.json_member_list(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn json_member_list(&self) -> JsonMemberList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for JsonObjectValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct JsonObjectValueFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub json_member_list: JsonMemberList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonRoot {
    pub(crate) syntax: SyntaxNode,
}
impl JsonRoot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> JsonRootFields {
        JsonRootFields {
            bom_token: self.bom_token(),
            value: self.value(),
            eof_token: self.eof_token(),
        }
    }
    pub fn bom_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<AnyJsonValue> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for JsonRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct JsonRootFields {
    pub bom_token: Option<SyntaxToken>,
    pub value: SyntaxResult<AnyJsonValue>,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonStringValue {
    pub(crate) syntax: SyntaxNode,
}
impl JsonStringValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> JsonStringValueFields {
        JsonStringValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for JsonStringValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct JsonStringValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyJsonMemberName {
    JsonBogusName(JsonBogusName),
    JsonMemberName(JsonMemberName),
    JsonMetavariable(JsonMetavariable),
}
impl AnyJsonMemberName {
    pub fn as_json_bogus_name(&self) -> Option<&JsonBogusName> {
        match &self {
            Self::JsonBogusName(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_json_member_name(&self) -> Option<&JsonMemberName> {
        match &self {
            Self::JsonMemberName(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_json_metavariable(&self) -> Option<&JsonMetavariable> {
        match &self {
            Self::JsonMetavariable(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyJsonValue {
    JsonArrayValue(JsonArrayValue),
    JsonBogusValue(JsonBogusValue),
    JsonBooleanValue(JsonBooleanValue),
    JsonMetavariable(JsonMetavariable),
    JsonNullValue(JsonNullValue),
    JsonNumberValue(JsonNumberValue),
    JsonObjectValue(JsonObjectValue),
    JsonStringValue(JsonStringValue),
}
impl AnyJsonValue {
    pub fn as_json_array_value(&self) -> Option<&JsonArrayValue> {
        match &self {
            Self::JsonArrayValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_json_bogus_value(&self) -> Option<&JsonBogusValue> {
        match &self {
            Self::JsonBogusValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_json_boolean_value(&self) -> Option<&JsonBooleanValue> {
        match &self {
            Self::JsonBooleanValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_json_metavariable(&self) -> Option<&JsonMetavariable> {
        match &self {
            Self::JsonMetavariable(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_json_null_value(&self) -> Option<&JsonNullValue> {
        match &self {
            Self::JsonNullValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_json_number_value(&self) -> Option<&JsonNumberValue> {
        match &self {
            Self::JsonNumberValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_json_object_value(&self) -> Option<&JsonObjectValue> {
        match &self {
            Self::JsonObjectValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_json_string_value(&self) -> Option<&JsonStringValue> {
        match &self {
            Self::JsonStringValue(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for JsonArrayValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_ARRAY_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == JSON_ARRAY_VALUE
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
impl std::fmt::Debug for JsonArrayValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<JsonArrayValue> for SyntaxNode {
    fn from(n: JsonArrayValue) -> Self {
        n.syntax
    }
}
impl From<JsonArrayValue> for SyntaxElement {
    fn from(n: JsonArrayValue) -> Self {
        n.syntax.into()
    }
}
impl AstNode for JsonBooleanValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_BOOLEAN_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == JSON_BOOLEAN_VALUE
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
impl std::fmt::Debug for JsonBooleanValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<JsonBooleanValue> for SyntaxNode {
    fn from(n: JsonBooleanValue) -> Self {
        n.syntax
    }
}
impl From<JsonBooleanValue> for SyntaxElement {
    fn from(n: JsonBooleanValue) -> Self {
        n.syntax.into()
    }
}
impl AstNode for JsonMember {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_MEMBER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == JSON_MEMBER
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
impl std::fmt::Debug for JsonMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<JsonMember> for SyntaxNode {
    fn from(n: JsonMember) -> Self {
        n.syntax
    }
}
impl From<JsonMember> for SyntaxElement {
    fn from(n: JsonMember) -> Self {
        n.syntax.into()
    }
}
impl AstNode for JsonMemberName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_MEMBER_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == JSON_MEMBER_NAME
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
impl std::fmt::Debug for JsonMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<JsonMemberName> for SyntaxNode {
    fn from(n: JsonMemberName) -> Self {
        n.syntax
    }
}
impl From<JsonMemberName> for SyntaxElement {
    fn from(n: JsonMemberName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for JsonMetavariable {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_METAVARIABLE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == JSON_METAVARIABLE
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
impl std::fmt::Debug for JsonMetavariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<JsonMetavariable> for SyntaxNode {
    fn from(n: JsonMetavariable) -> Self {
        n.syntax
    }
}
impl From<JsonMetavariable> for SyntaxElement {
    fn from(n: JsonMetavariable) -> Self {
        n.syntax.into()
    }
}
impl AstNode for JsonNullValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_NULL_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == JSON_NULL_VALUE
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
impl std::fmt::Debug for JsonNullValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<JsonNullValue> for SyntaxNode {
    fn from(n: JsonNullValue) -> Self {
        n.syntax
    }
}
impl From<JsonNullValue> for SyntaxElement {
    fn from(n: JsonNullValue) -> Self {
        n.syntax.into()
    }
}
impl AstNode for JsonNumberValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_NUMBER_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == JSON_NUMBER_VALUE
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
impl std::fmt::Debug for JsonNumberValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<JsonNumberValue> for SyntaxNode {
    fn from(n: JsonNumberValue) -> Self {
        n.syntax
    }
}
impl From<JsonNumberValue> for SyntaxElement {
    fn from(n: JsonNumberValue) -> Self {
        n.syntax.into()
    }
}
impl AstNode for JsonObjectValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_OBJECT_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == JSON_OBJECT_VALUE
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
impl std::fmt::Debug for JsonObjectValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<JsonObjectValue> for SyntaxNode {
    fn from(n: JsonObjectValue) -> Self {
        n.syntax
    }
}
impl From<JsonObjectValue> for SyntaxElement {
    fn from(n: JsonObjectValue) -> Self {
        n.syntax.into()
    }
}
impl AstNode for JsonRoot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_ROOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == JSON_ROOT
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
impl std::fmt::Debug for JsonRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<JsonRoot> for SyntaxNode {
    fn from(n: JsonRoot) -> Self {
        n.syntax
    }
}
impl From<JsonRoot> for SyntaxElement {
    fn from(n: JsonRoot) -> Self {
        n.syntax.into()
    }
}
impl AstNode for JsonStringValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_STRING_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == JSON_STRING_VALUE
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
impl std::fmt::Debug for JsonStringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<JsonStringValue> for SyntaxNode {
    fn from(n: JsonStringValue) -> Self {
        n.syntax
    }
}
impl From<JsonStringValue> for SyntaxElement {
    fn from(n: JsonStringValue) -> Self {
        n.syntax.into()
    }
}
impl From<JsonBogusName> for AnyJsonMemberName {
    fn from(node: JsonBogusName) -> Self {
        Self::JsonBogusName(node)
    }
}
impl From<JsonMemberName> for AnyJsonMemberName {
    fn from(node: JsonMemberName) -> Self {
        Self::JsonMemberName(node)
    }
}
impl From<JsonMetavariable> for AnyJsonMemberName {
    fn from(node: JsonMetavariable) -> Self {
        Self::JsonMetavariable(node)
    }
}
impl AstNode for AnyJsonMemberName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = JsonBogusName::KIND_SET
        .union(JsonMemberName::KIND_SET)
        .union(JsonMetavariable::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, JSON_BOGUS_NAME | JSON_MEMBER_NAME | JSON_METAVARIABLE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JSON_BOGUS_NAME => Self::JsonBogusName(JsonBogusName { syntax }),
            JSON_MEMBER_NAME => Self::JsonMemberName(JsonMemberName { syntax }),
            JSON_METAVARIABLE => Self::JsonMetavariable(JsonMetavariable { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::JsonBogusName(it) => it.syntax(),
            Self::JsonMemberName(it) => it.syntax(),
            Self::JsonMetavariable(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::JsonBogusName(it) => it.into_syntax(),
            Self::JsonMemberName(it) => it.into_syntax(),
            Self::JsonMetavariable(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyJsonMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyJsonMemberName> for SyntaxNode {
    fn from(n: AnyJsonMemberName) -> Self {
        match n {
            AnyJsonMemberName::JsonBogusName(it) => it.into_syntax(),
            AnyJsonMemberName::JsonMemberName(it) => it.into_syntax(),
            AnyJsonMemberName::JsonMetavariable(it) => it.into_syntax(),
        }
    }
}
impl From<AnyJsonMemberName> for SyntaxElement {
    fn from(n: AnyJsonMemberName) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsonArrayValue> for AnyJsonValue {
    fn from(node: JsonArrayValue) -> Self {
        Self::JsonArrayValue(node)
    }
}
impl From<JsonBogusValue> for AnyJsonValue {
    fn from(node: JsonBogusValue) -> Self {
        Self::JsonBogusValue(node)
    }
}
impl From<JsonBooleanValue> for AnyJsonValue {
    fn from(node: JsonBooleanValue) -> Self {
        Self::JsonBooleanValue(node)
    }
}
impl From<JsonMetavariable> for AnyJsonValue {
    fn from(node: JsonMetavariable) -> Self {
        Self::JsonMetavariable(node)
    }
}
impl From<JsonNullValue> for AnyJsonValue {
    fn from(node: JsonNullValue) -> Self {
        Self::JsonNullValue(node)
    }
}
impl From<JsonNumberValue> for AnyJsonValue {
    fn from(node: JsonNumberValue) -> Self {
        Self::JsonNumberValue(node)
    }
}
impl From<JsonObjectValue> for AnyJsonValue {
    fn from(node: JsonObjectValue) -> Self {
        Self::JsonObjectValue(node)
    }
}
impl From<JsonStringValue> for AnyJsonValue {
    fn from(node: JsonStringValue) -> Self {
        Self::JsonStringValue(node)
    }
}
impl AstNode for AnyJsonValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = JsonArrayValue::KIND_SET
        .union(JsonBogusValue::KIND_SET)
        .union(JsonBooleanValue::KIND_SET)
        .union(JsonMetavariable::KIND_SET)
        .union(JsonNullValue::KIND_SET)
        .union(JsonNumberValue::KIND_SET)
        .union(JsonObjectValue::KIND_SET)
        .union(JsonStringValue::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            JSON_ARRAY_VALUE
                | JSON_BOGUS_VALUE
                | JSON_BOOLEAN_VALUE
                | JSON_METAVARIABLE
                | JSON_NULL_VALUE
                | JSON_NUMBER_VALUE
                | JSON_OBJECT_VALUE
                | JSON_STRING_VALUE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JSON_ARRAY_VALUE => Self::JsonArrayValue(JsonArrayValue { syntax }),
            JSON_BOGUS_VALUE => Self::JsonBogusValue(JsonBogusValue { syntax }),
            JSON_BOOLEAN_VALUE => Self::JsonBooleanValue(JsonBooleanValue { syntax }),
            JSON_METAVARIABLE => Self::JsonMetavariable(JsonMetavariable { syntax }),
            JSON_NULL_VALUE => Self::JsonNullValue(JsonNullValue { syntax }),
            JSON_NUMBER_VALUE => Self::JsonNumberValue(JsonNumberValue { syntax }),
            JSON_OBJECT_VALUE => Self::JsonObjectValue(JsonObjectValue { syntax }),
            JSON_STRING_VALUE => Self::JsonStringValue(JsonStringValue { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::JsonArrayValue(it) => it.syntax(),
            Self::JsonBogusValue(it) => it.syntax(),
            Self::JsonBooleanValue(it) => it.syntax(),
            Self::JsonMetavariable(it) => it.syntax(),
            Self::JsonNullValue(it) => it.syntax(),
            Self::JsonNumberValue(it) => it.syntax(),
            Self::JsonObjectValue(it) => it.syntax(),
            Self::JsonStringValue(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::JsonArrayValue(it) => it.into_syntax(),
            Self::JsonBogusValue(it) => it.into_syntax(),
            Self::JsonBooleanValue(it) => it.into_syntax(),
            Self::JsonMetavariable(it) => it.into_syntax(),
            Self::JsonNullValue(it) => it.into_syntax(),
            Self::JsonNumberValue(it) => it.into_syntax(),
            Self::JsonObjectValue(it) => it.into_syntax(),
            Self::JsonStringValue(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyJsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyJsonValue> for SyntaxNode {
    fn from(n: AnyJsonValue) -> Self {
        match n {
            AnyJsonValue::JsonArrayValue(it) => it.into_syntax(),
            AnyJsonValue::JsonBogusValue(it) => it.into_syntax(),
            AnyJsonValue::JsonBooleanValue(it) => it.into_syntax(),
            AnyJsonValue::JsonMetavariable(it) => it.into_syntax(),
            AnyJsonValue::JsonNullValue(it) => it.into_syntax(),
            AnyJsonValue::JsonNumberValue(it) => it.into_syntax(),
            AnyJsonValue::JsonObjectValue(it) => it.into_syntax(),
            AnyJsonValue::JsonStringValue(it) => it.into_syntax(),
        }
    }
}
impl From<AnyJsonValue> for SyntaxElement {
    fn from(n: AnyJsonValue) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyJsonMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyJsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonArrayValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonBooleanValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonMetavariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonNullValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonNumberValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonObjectValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonStringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct JsonBogus {
    syntax: SyntaxNode,
}
impl JsonBogus {
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
impl AstNode for JsonBogus {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_BOGUS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == JSON_BOGUS
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
impl std::fmt::Debug for JsonBogus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<JsonBogus> for SyntaxNode {
    fn from(n: JsonBogus) -> Self {
        n.syntax
    }
}
impl From<JsonBogus> for SyntaxElement {
    fn from(n: JsonBogus) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct JsonBogusName {
    syntax: SyntaxNode,
}
impl JsonBogusName {
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
impl AstNode for JsonBogusName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_BOGUS_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == JSON_BOGUS_NAME
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
impl std::fmt::Debug for JsonBogusName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<JsonBogusName> for SyntaxNode {
    fn from(n: JsonBogusName) -> Self {
        n.syntax
    }
}
impl From<JsonBogusName> for SyntaxElement {
    fn from(n: JsonBogusName) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct JsonBogusValue {
    syntax: SyntaxNode,
}
impl JsonBogusValue {
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
impl AstNode for JsonBogusValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_BOGUS_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == JSON_BOGUS_VALUE
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
impl std::fmt::Debug for JsonBogusValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<JsonBogusValue> for SyntaxNode {
    fn from(n: JsonBogusValue) -> Self {
        n.syntax
    }
}
impl From<JsonBogusValue> for SyntaxElement {
    fn from(n: JsonBogusValue) -> Self {
        n.syntax.into()
    }
}
biome_rowan::declare_node_union! { pub AnyJsonBogusNode = JsonBogus | JsonBogusName | JsonBogusValue }
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsonArrayElementList {
    syntax_list: SyntaxList,
}
impl JsonArrayElementList {
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
impl AstNode for JsonArrayElementList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_ARRAY_ELEMENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == JSON_ARRAY_ELEMENT_LIST
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
impl Serialize for JsonArrayElementList {
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
impl AstSeparatedList for JsonArrayElementList {
    type Language = Language;
    type Node = AnyJsonValue;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for JsonArrayElementList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax_list.node(), f)
    }
}
impl IntoIterator for JsonArrayElementList {
    type Item = SyntaxResult<AnyJsonValue>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyJsonValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &JsonArrayElementList {
    type Item = SyntaxResult<AnyJsonValue>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyJsonValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsonMemberList {
    syntax_list: SyntaxList,
}
impl JsonMemberList {
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
impl AstNode for JsonMemberList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_MEMBER_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == JSON_MEMBER_LIST
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
impl Serialize for JsonMemberList {
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
impl AstSeparatedList for JsonMemberList {
    type Language = Language;
    type Node = JsonMember;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for JsonMemberList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax_list.node(), f)
    }
}
impl IntoIterator for JsonMemberList {
    type Item = SyntaxResult<JsonMember>;
    type IntoIter = AstSeparatedListNodesIterator<Language, JsonMember>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &JsonMemberList {
    type Item = SyntaxResult<JsonMember>;
    type IntoIter = AstSeparatedListNodesIterator<Language, JsonMember>;
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
        "nJsonArrayValue,l_brack_token,elements,r_brack_token\n",
        "nJsonBooleanValue,value_token\n",
        "nJsonMember,name,colon_token,value\n",
        "nJsonMemberName,value_token\n",
        "nJsonMetavariable,value_token\n",
        "nJsonNullValue,value_token\n",
        "nJsonNumberValue,value_token\n",
        "nJsonObjectValue,l_curly_token,json_member_list,r_curly_token\n",
        "nJsonRoot,?bom_token,value,eof_token\n",
        "nJsonStringValue,value_token\n",
        "bJsonBogus\n",
        "bJsonBogusName\n",
        "bJsonBogusValue\n",
        "sJsonArrayElementList\n",
        "sJsonMemberList\n"
    );
    fn shape_offset(kind: SyntaxKind) -> u32 {
        match kind {
            JSON_ARRAY_VALUE => 0u32,
            JSON_BOOLEAN_VALUE => 53u32,
            JSON_MEMBER => 83u32,
            JSON_MEMBER_NAME => 118u32,
            JSON_METAVARIABLE => 146u32,
            JSON_NULL_VALUE => 176u32,
            JSON_NUMBER_VALUE => 203u32,
            JSON_OBJECT_VALUE => 232u32,
            JSON_ROOT => 294u32,
            JSON_STRING_VALUE => 331u32,
            JSON_BOGUS => 360u32,
            JSON_BOGUS_NAME => 371u32,
            JSON_BOGUS_VALUE => 386u32,
            JSON_ARRAY_ELEMENT_LIST => 402u32,
            JSON_MEMBER_LIST => 424u32,
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
