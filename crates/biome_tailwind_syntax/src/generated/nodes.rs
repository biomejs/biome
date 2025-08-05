//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(dead_code)]
#![allow(unused)]
use crate::{
    TailwindLanguage as Language, TailwindSyntaxElement as SyntaxElement,
    TailwindSyntaxElementChildren as SyntaxElementChildren,
    TailwindSyntaxKind::{self as SyntaxKind, *},
    TailwindSyntaxList as SyntaxList, TailwindSyntaxNode as SyntaxNode,
    TailwindSyntaxToken as SyntaxToken,
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
pub struct TwArbitraryCandidate {
    pub(crate) syntax: SyntaxNode,
}
impl TwArbitraryCandidate {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TwArbitraryCandidateFields {
        TwArbitraryCandidateFields {
            l_brack_token: self.l_brack_token(),
            property_token: self.property_token(),
            colon_token: self.colon_token(),
            value_token: self.value_token(),
            r_brack_token: self.r_brack_token(),
            modifier: self.modifier(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn property_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn modifier(&self) -> Option<AnyTwModifier> {
        support::node(&self.syntax, 5usize)
    }
}
impl Serialize for TwArbitraryCandidate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TwArbitraryCandidateFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub property_token: SyntaxResult<SyntaxToken>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value_token: SyntaxResult<SyntaxToken>,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
    pub modifier: Option<AnyTwModifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TwArbitraryValue {
    pub(crate) syntax: SyntaxNode,
}
impl TwArbitraryValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TwArbitraryValueFields {
        TwArbitraryValueFields {
            l_brack_token: self.l_brack_token(),
            value_token: self.value_token(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for TwArbitraryValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TwArbitraryValueFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub value_token: SyntaxResult<SyntaxToken>,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TwArbitraryVariant {
    pub(crate) syntax: SyntaxNode,
}
impl TwArbitraryVariant {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TwArbitraryVariantFields {
        TwArbitraryVariantFields {
            l_brack_token: self.l_brack_token(),
            selector_token: self.selector_token(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn selector_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for TwArbitraryVariant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TwArbitraryVariantFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub selector_token: SyntaxResult<SyntaxToken>,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TwCssVariableValue {
    pub(crate) syntax: SyntaxNode,
}
impl TwCssVariableValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TwCssVariableValueFields {
        TwCssVariableValueFields {
            l_paren_token: self.l_paren_token(),
            value_token: self.value_token(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for TwCssVariableValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TwCssVariableValueFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub value_token: SyntaxResult<SyntaxToken>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TwFullCandidate {
    pub(crate) syntax: SyntaxNode,
}
impl TwFullCandidate {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TwFullCandidateFields {
        TwFullCandidateFields {
            variants: self.variants(),
            candidate: self.candidate(),
            excl_token: self.excl_token(),
        }
    }
    pub fn variants(&self) -> TwVariantList {
        support::list(&self.syntax, 0usize)
    }
    pub fn candidate(&self) -> SyntaxResult<AnyTwCandidate> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn excl_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
}
impl Serialize for TwFullCandidate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TwFullCandidateFields {
    pub variants: TwVariantList,
    pub candidate: SyntaxResult<AnyTwCandidate>,
    pub excl_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TwFunctionalCandidate {
    pub(crate) syntax: SyntaxNode,
}
impl TwFunctionalCandidate {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TwFunctionalCandidateFields {
        TwFunctionalCandidateFields {
            base_token: self.base_token(),
            minus_token: self.minus_token(),
            value: self.value(),
            modifier: self.modifier(),
        }
    }
    pub fn base_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn minus_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<AnyTwValue> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn modifier(&self) -> Option<AnyTwModifier> {
        support::node(&self.syntax, 3usize)
    }
}
impl Serialize for TwFunctionalCandidate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TwFunctionalCandidateFields {
    pub base_token: SyntaxResult<SyntaxToken>,
    pub minus_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnyTwValue>,
    pub modifier: Option<AnyTwModifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TwFunctionalVariant {
    pub(crate) syntax: SyntaxNode,
}
impl TwFunctionalVariant {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TwFunctionalVariantFields {
        TwFunctionalVariantFields {
            base_token: self.base_token(),
            minus_token: self.minus_token(),
            value: self.value(),
        }
    }
    pub fn base_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn minus_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<AnyTwValue> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for TwFunctionalVariant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TwFunctionalVariantFields {
    pub base_token: SyntaxResult<SyntaxToken>,
    pub minus_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnyTwValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TwModifier {
    pub(crate) syntax: SyntaxNode,
}
impl TwModifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TwModifierFields {
        TwModifierFields {
            slash_token: self.slash_token(),
            value: self.value(),
        }
    }
    pub fn slash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<AnyTwValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for TwModifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TwModifierFields {
    pub slash_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnyTwValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TwNamedValue {
    pub(crate) syntax: SyntaxNode,
}
impl TwNamedValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TwNamedValueFields {
        TwNamedValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TwNamedValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TwNamedValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TwRoot {
    pub(crate) syntax: SyntaxNode,
}
impl TwRoot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TwRootFields {
        TwRootFields {
            bom_token: self.bom_token(),
            candidates: self.candidates(),
            eof_token: self.eof_token(),
        }
    }
    pub fn bom_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn candidates(&self) -> TwCandidateList {
        support::list(&self.syntax, 1usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for TwRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TwRootFields {
    pub bom_token: Option<SyntaxToken>,
    pub candidates: TwCandidateList,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TwStaticCandidate {
    pub(crate) syntax: SyntaxNode,
}
impl TwStaticCandidate {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TwStaticCandidateFields {
        TwStaticCandidateFields {
            base_token: self.base_token(),
        }
    }
    pub fn base_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TwStaticCandidate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TwStaticCandidateFields {
    pub base_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TwStaticVariant {
    pub(crate) syntax: SyntaxNode,
}
impl TwStaticVariant {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TwStaticVariantFields {
        TwStaticVariantFields {
            base_token: self.base_token(),
        }
    }
    pub fn base_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TwStaticVariant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TwStaticVariantFields {
    pub base_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyTwCandidate {
    TwArbitraryCandidate(TwArbitraryCandidate),
    TwBogusCandidate(TwBogusCandidate),
    TwFunctionalCandidate(TwFunctionalCandidate),
    TwStaticCandidate(TwStaticCandidate),
}
impl AnyTwCandidate {
    pub fn as_tw_arbitrary_candidate(&self) -> Option<&TwArbitraryCandidate> {
        match &self {
            Self::TwArbitraryCandidate(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_tw_bogus_candidate(&self) -> Option<&TwBogusCandidate> {
        match &self {
            Self::TwBogusCandidate(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_tw_functional_candidate(&self) -> Option<&TwFunctionalCandidate> {
        match &self {
            Self::TwFunctionalCandidate(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_tw_static_candidate(&self) -> Option<&TwStaticCandidate> {
        match &self {
            Self::TwStaticCandidate(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyTwFullCandidate {
    TwBogusCandidate(TwBogusCandidate),
    TwFullCandidate(TwFullCandidate),
}
impl AnyTwFullCandidate {
    pub fn as_tw_bogus_candidate(&self) -> Option<&TwBogusCandidate> {
        match &self {
            Self::TwBogusCandidate(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_tw_full_candidate(&self) -> Option<&TwFullCandidate> {
        match &self {
            Self::TwFullCandidate(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyTwModifier {
    TwBogusModifier(TwBogusModifier),
    TwModifier(TwModifier),
}
impl AnyTwModifier {
    pub fn as_tw_bogus_modifier(&self) -> Option<&TwBogusModifier> {
        match &self {
            Self::TwBogusModifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_tw_modifier(&self) -> Option<&TwModifier> {
        match &self {
            Self::TwModifier(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyTwValue {
    TwArbitraryValue(TwArbitraryValue),
    TwBogusValue(TwBogusValue),
    TwCssVariableValue(TwCssVariableValue),
    TwNamedValue(TwNamedValue),
}
impl AnyTwValue {
    pub fn as_tw_arbitrary_value(&self) -> Option<&TwArbitraryValue> {
        match &self {
            Self::TwArbitraryValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_tw_bogus_value(&self) -> Option<&TwBogusValue> {
        match &self {
            Self::TwBogusValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_tw_css_variable_value(&self) -> Option<&TwCssVariableValue> {
        match &self {
            Self::TwCssVariableValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_tw_named_value(&self) -> Option<&TwNamedValue> {
        match &self {
            Self::TwNamedValue(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyTwVariant {
    TwArbitraryVariant(TwArbitraryVariant),
    TwBogusVariant(TwBogusVariant),
    TwFunctionalVariant(TwFunctionalVariant),
    TwStaticVariant(TwStaticVariant),
}
impl AnyTwVariant {
    pub fn as_tw_arbitrary_variant(&self) -> Option<&TwArbitraryVariant> {
        match &self {
            Self::TwArbitraryVariant(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_tw_bogus_variant(&self) -> Option<&TwBogusVariant> {
        match &self {
            Self::TwBogusVariant(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_tw_functional_variant(&self) -> Option<&TwFunctionalVariant> {
        match &self {
            Self::TwFunctionalVariant(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_tw_static_variant(&self) -> Option<&TwStaticVariant> {
        match &self {
            Self::TwStaticVariant(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for TwArbitraryCandidate {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_ARBITRARY_CANDIDATE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_ARBITRARY_CANDIDATE
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
impl std::fmt::Debug for TwArbitraryCandidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TwArbitraryCandidate")
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field(
                    "property_token",
                    &support::DebugSyntaxResult(self.property_token()),
                )
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .field("modifier", &support::DebugOptionalElement(self.modifier()))
                .finish()
        } else {
            f.debug_struct("TwArbitraryCandidate").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TwArbitraryCandidate> for SyntaxNode {
    fn from(n: TwArbitraryCandidate) -> Self {
        n.syntax
    }
}
impl From<TwArbitraryCandidate> for SyntaxElement {
    fn from(n: TwArbitraryCandidate) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TwArbitraryValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_ARBITRARY_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_ARBITRARY_VALUE
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
impl std::fmt::Debug for TwArbitraryValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TwArbitraryValue")
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .finish()
        } else {
            f.debug_struct("TwArbitraryValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TwArbitraryValue> for SyntaxNode {
    fn from(n: TwArbitraryValue) -> Self {
        n.syntax
    }
}
impl From<TwArbitraryValue> for SyntaxElement {
    fn from(n: TwArbitraryValue) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TwArbitraryVariant {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_ARBITRARY_VARIANT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_ARBITRARY_VARIANT
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
impl std::fmt::Debug for TwArbitraryVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TwArbitraryVariant")
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field(
                    "selector_token",
                    &support::DebugSyntaxResult(self.selector_token()),
                )
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .finish()
        } else {
            f.debug_struct("TwArbitraryVariant").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TwArbitraryVariant> for SyntaxNode {
    fn from(n: TwArbitraryVariant) -> Self {
        n.syntax
    }
}
impl From<TwArbitraryVariant> for SyntaxElement {
    fn from(n: TwArbitraryVariant) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TwCssVariableValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_CSS_VARIABLE_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_CSS_VARIABLE_VALUE
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
impl std::fmt::Debug for TwCssVariableValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TwCssVariableValue")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("TwCssVariableValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TwCssVariableValue> for SyntaxNode {
    fn from(n: TwCssVariableValue) -> Self {
        n.syntax
    }
}
impl From<TwCssVariableValue> for SyntaxElement {
    fn from(n: TwCssVariableValue) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TwFullCandidate {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_FULL_CANDIDATE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_FULL_CANDIDATE
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
impl std::fmt::Debug for TwFullCandidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TwFullCandidate")
                .field("variants", &self.variants())
                .field("candidate", &support::DebugSyntaxResult(self.candidate()))
                .field(
                    "excl_token",
                    &support::DebugOptionalElement(self.excl_token()),
                )
                .finish()
        } else {
            f.debug_struct("TwFullCandidate").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TwFullCandidate> for SyntaxNode {
    fn from(n: TwFullCandidate) -> Self {
        n.syntax
    }
}
impl From<TwFullCandidate> for SyntaxElement {
    fn from(n: TwFullCandidate) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TwFunctionalCandidate {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_FUNCTIONAL_CANDIDATE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_FUNCTIONAL_CANDIDATE
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
impl std::fmt::Debug for TwFunctionalCandidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TwFunctionalCandidate")
                .field("base_token", &support::DebugSyntaxResult(self.base_token()))
                .field(
                    "minus_token",
                    &support::DebugSyntaxResult(self.minus_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .field("modifier", &support::DebugOptionalElement(self.modifier()))
                .finish()
        } else {
            f.debug_struct("TwFunctionalCandidate").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TwFunctionalCandidate> for SyntaxNode {
    fn from(n: TwFunctionalCandidate) -> Self {
        n.syntax
    }
}
impl From<TwFunctionalCandidate> for SyntaxElement {
    fn from(n: TwFunctionalCandidate) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TwFunctionalVariant {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_FUNCTIONAL_VARIANT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_FUNCTIONAL_VARIANT
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
impl std::fmt::Debug for TwFunctionalVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TwFunctionalVariant")
                .field("base_token", &support::DebugSyntaxResult(self.base_token()))
                .field(
                    "minus_token",
                    &support::DebugSyntaxResult(self.minus_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("TwFunctionalVariant").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TwFunctionalVariant> for SyntaxNode {
    fn from(n: TwFunctionalVariant) -> Self {
        n.syntax
    }
}
impl From<TwFunctionalVariant> for SyntaxElement {
    fn from(n: TwFunctionalVariant) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TwModifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_MODIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_MODIFIER
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
impl std::fmt::Debug for TwModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TwModifier")
                .field(
                    "slash_token",
                    &support::DebugSyntaxResult(self.slash_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("TwModifier").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TwModifier> for SyntaxNode {
    fn from(n: TwModifier) -> Self {
        n.syntax
    }
}
impl From<TwModifier> for SyntaxElement {
    fn from(n: TwModifier) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TwNamedValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_NAMED_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_NAMED_VALUE
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
impl std::fmt::Debug for TwNamedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TwNamedValue")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("TwNamedValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TwNamedValue> for SyntaxNode {
    fn from(n: TwNamedValue) -> Self {
        n.syntax
    }
}
impl From<TwNamedValue> for SyntaxElement {
    fn from(n: TwNamedValue) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TwRoot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_ROOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_ROOT
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
impl std::fmt::Debug for TwRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TwRoot")
                .field(
                    "bom_token",
                    &support::DebugOptionalElement(self.bom_token()),
                )
                .field("candidates", &self.candidates())
                .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
                .finish()
        } else {
            f.debug_struct("TwRoot").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TwRoot> for SyntaxNode {
    fn from(n: TwRoot) -> Self {
        n.syntax
    }
}
impl From<TwRoot> for SyntaxElement {
    fn from(n: TwRoot) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TwStaticCandidate {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_STATIC_CANDIDATE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_STATIC_CANDIDATE
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
impl std::fmt::Debug for TwStaticCandidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TwStaticCandidate")
                .field("base_token", &support::DebugSyntaxResult(self.base_token()))
                .finish()
        } else {
            f.debug_struct("TwStaticCandidate").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TwStaticCandidate> for SyntaxNode {
    fn from(n: TwStaticCandidate) -> Self {
        n.syntax
    }
}
impl From<TwStaticCandidate> for SyntaxElement {
    fn from(n: TwStaticCandidate) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TwStaticVariant {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_STATIC_VARIANT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_STATIC_VARIANT
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
impl std::fmt::Debug for TwStaticVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("TwStaticVariant")
                .field("base_token", &support::DebugSyntaxResult(self.base_token()))
                .finish()
        } else {
            f.debug_struct("TwStaticVariant").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<TwStaticVariant> for SyntaxNode {
    fn from(n: TwStaticVariant) -> Self {
        n.syntax
    }
}
impl From<TwStaticVariant> for SyntaxElement {
    fn from(n: TwStaticVariant) -> Self {
        n.syntax.into()
    }
}
impl From<TwArbitraryCandidate> for AnyTwCandidate {
    fn from(node: TwArbitraryCandidate) -> Self {
        Self::TwArbitraryCandidate(node)
    }
}
impl From<TwBogusCandidate> for AnyTwCandidate {
    fn from(node: TwBogusCandidate) -> Self {
        Self::TwBogusCandidate(node)
    }
}
impl From<TwFunctionalCandidate> for AnyTwCandidate {
    fn from(node: TwFunctionalCandidate) -> Self {
        Self::TwFunctionalCandidate(node)
    }
}
impl From<TwStaticCandidate> for AnyTwCandidate {
    fn from(node: TwStaticCandidate) -> Self {
        Self::TwStaticCandidate(node)
    }
}
impl AstNode for AnyTwCandidate {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = TwArbitraryCandidate::KIND_SET
        .union(TwBogusCandidate::KIND_SET)
        .union(TwFunctionalCandidate::KIND_SET)
        .union(TwStaticCandidate::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            TW_ARBITRARY_CANDIDATE
                | TW_BOGUS_CANDIDATE
                | TW_FUNCTIONAL_CANDIDATE
                | TW_STATIC_CANDIDATE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TW_ARBITRARY_CANDIDATE => Self::TwArbitraryCandidate(TwArbitraryCandidate { syntax }),
            TW_BOGUS_CANDIDATE => Self::TwBogusCandidate(TwBogusCandidate { syntax }),
            TW_FUNCTIONAL_CANDIDATE => {
                Self::TwFunctionalCandidate(TwFunctionalCandidate { syntax })
            }
            TW_STATIC_CANDIDATE => Self::TwStaticCandidate(TwStaticCandidate { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::TwArbitraryCandidate(it) => &it.syntax,
            Self::TwBogusCandidate(it) => &it.syntax,
            Self::TwFunctionalCandidate(it) => &it.syntax,
            Self::TwStaticCandidate(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::TwArbitraryCandidate(it) => it.syntax,
            Self::TwBogusCandidate(it) => it.syntax,
            Self::TwFunctionalCandidate(it) => it.syntax,
            Self::TwStaticCandidate(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyTwCandidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TwArbitraryCandidate(it) => std::fmt::Debug::fmt(it, f),
            Self::TwBogusCandidate(it) => std::fmt::Debug::fmt(it, f),
            Self::TwFunctionalCandidate(it) => std::fmt::Debug::fmt(it, f),
            Self::TwStaticCandidate(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyTwCandidate> for SyntaxNode {
    fn from(n: AnyTwCandidate) -> Self {
        match n {
            AnyTwCandidate::TwArbitraryCandidate(it) => it.into(),
            AnyTwCandidate::TwBogusCandidate(it) => it.into(),
            AnyTwCandidate::TwFunctionalCandidate(it) => it.into(),
            AnyTwCandidate::TwStaticCandidate(it) => it.into(),
        }
    }
}
impl From<AnyTwCandidate> for SyntaxElement {
    fn from(n: AnyTwCandidate) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TwBogusCandidate> for AnyTwFullCandidate {
    fn from(node: TwBogusCandidate) -> Self {
        Self::TwBogusCandidate(node)
    }
}
impl From<TwFullCandidate> for AnyTwFullCandidate {
    fn from(node: TwFullCandidate) -> Self {
        Self::TwFullCandidate(node)
    }
}
impl AstNode for AnyTwFullCandidate {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        TwBogusCandidate::KIND_SET.union(TwFullCandidate::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, TW_BOGUS_CANDIDATE | TW_FULL_CANDIDATE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TW_BOGUS_CANDIDATE => Self::TwBogusCandidate(TwBogusCandidate { syntax }),
            TW_FULL_CANDIDATE => Self::TwFullCandidate(TwFullCandidate { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::TwBogusCandidate(it) => &it.syntax,
            Self::TwFullCandidate(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::TwBogusCandidate(it) => it.syntax,
            Self::TwFullCandidate(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyTwFullCandidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TwBogusCandidate(it) => std::fmt::Debug::fmt(it, f),
            Self::TwFullCandidate(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyTwFullCandidate> for SyntaxNode {
    fn from(n: AnyTwFullCandidate) -> Self {
        match n {
            AnyTwFullCandidate::TwBogusCandidate(it) => it.into(),
            AnyTwFullCandidate::TwFullCandidate(it) => it.into(),
        }
    }
}
impl From<AnyTwFullCandidate> for SyntaxElement {
    fn from(n: AnyTwFullCandidate) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TwBogusModifier> for AnyTwModifier {
    fn from(node: TwBogusModifier) -> Self {
        Self::TwBogusModifier(node)
    }
}
impl From<TwModifier> for AnyTwModifier {
    fn from(node: TwModifier) -> Self {
        Self::TwModifier(node)
    }
}
impl AstNode for AnyTwModifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = TwBogusModifier::KIND_SET.union(TwModifier::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, TW_BOGUS_MODIFIER | TW_MODIFIER)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TW_BOGUS_MODIFIER => Self::TwBogusModifier(TwBogusModifier { syntax }),
            TW_MODIFIER => Self::TwModifier(TwModifier { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::TwBogusModifier(it) => &it.syntax,
            Self::TwModifier(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::TwBogusModifier(it) => it.syntax,
            Self::TwModifier(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyTwModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TwBogusModifier(it) => std::fmt::Debug::fmt(it, f),
            Self::TwModifier(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyTwModifier> for SyntaxNode {
    fn from(n: AnyTwModifier) -> Self {
        match n {
            AnyTwModifier::TwBogusModifier(it) => it.into(),
            AnyTwModifier::TwModifier(it) => it.into(),
        }
    }
}
impl From<AnyTwModifier> for SyntaxElement {
    fn from(n: AnyTwModifier) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TwArbitraryValue> for AnyTwValue {
    fn from(node: TwArbitraryValue) -> Self {
        Self::TwArbitraryValue(node)
    }
}
impl From<TwBogusValue> for AnyTwValue {
    fn from(node: TwBogusValue) -> Self {
        Self::TwBogusValue(node)
    }
}
impl From<TwCssVariableValue> for AnyTwValue {
    fn from(node: TwCssVariableValue) -> Self {
        Self::TwCssVariableValue(node)
    }
}
impl From<TwNamedValue> for AnyTwValue {
    fn from(node: TwNamedValue) -> Self {
        Self::TwNamedValue(node)
    }
}
impl AstNode for AnyTwValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = TwArbitraryValue::KIND_SET
        .union(TwBogusValue::KIND_SET)
        .union(TwCssVariableValue::KIND_SET)
        .union(TwNamedValue::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            TW_ARBITRARY_VALUE | TW_BOGUS_VALUE | TW_CSS_VARIABLE_VALUE | TW_NAMED_VALUE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TW_ARBITRARY_VALUE => Self::TwArbitraryValue(TwArbitraryValue { syntax }),
            TW_BOGUS_VALUE => Self::TwBogusValue(TwBogusValue { syntax }),
            TW_CSS_VARIABLE_VALUE => Self::TwCssVariableValue(TwCssVariableValue { syntax }),
            TW_NAMED_VALUE => Self::TwNamedValue(TwNamedValue { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::TwArbitraryValue(it) => &it.syntax,
            Self::TwBogusValue(it) => &it.syntax,
            Self::TwCssVariableValue(it) => &it.syntax,
            Self::TwNamedValue(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::TwArbitraryValue(it) => it.syntax,
            Self::TwBogusValue(it) => it.syntax,
            Self::TwCssVariableValue(it) => it.syntax,
            Self::TwNamedValue(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyTwValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TwArbitraryValue(it) => std::fmt::Debug::fmt(it, f),
            Self::TwBogusValue(it) => std::fmt::Debug::fmt(it, f),
            Self::TwCssVariableValue(it) => std::fmt::Debug::fmt(it, f),
            Self::TwNamedValue(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyTwValue> for SyntaxNode {
    fn from(n: AnyTwValue) -> Self {
        match n {
            AnyTwValue::TwArbitraryValue(it) => it.into(),
            AnyTwValue::TwBogusValue(it) => it.into(),
            AnyTwValue::TwCssVariableValue(it) => it.into(),
            AnyTwValue::TwNamedValue(it) => it.into(),
        }
    }
}
impl From<AnyTwValue> for SyntaxElement {
    fn from(n: AnyTwValue) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TwArbitraryVariant> for AnyTwVariant {
    fn from(node: TwArbitraryVariant) -> Self {
        Self::TwArbitraryVariant(node)
    }
}
impl From<TwBogusVariant> for AnyTwVariant {
    fn from(node: TwBogusVariant) -> Self {
        Self::TwBogusVariant(node)
    }
}
impl From<TwFunctionalVariant> for AnyTwVariant {
    fn from(node: TwFunctionalVariant) -> Self {
        Self::TwFunctionalVariant(node)
    }
}
impl From<TwStaticVariant> for AnyTwVariant {
    fn from(node: TwStaticVariant) -> Self {
        Self::TwStaticVariant(node)
    }
}
impl AstNode for AnyTwVariant {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = TwArbitraryVariant::KIND_SET
        .union(TwBogusVariant::KIND_SET)
        .union(TwFunctionalVariant::KIND_SET)
        .union(TwStaticVariant::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            TW_ARBITRARY_VARIANT | TW_BOGUS_VARIANT | TW_FUNCTIONAL_VARIANT | TW_STATIC_VARIANT
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TW_ARBITRARY_VARIANT => Self::TwArbitraryVariant(TwArbitraryVariant { syntax }),
            TW_BOGUS_VARIANT => Self::TwBogusVariant(TwBogusVariant { syntax }),
            TW_FUNCTIONAL_VARIANT => Self::TwFunctionalVariant(TwFunctionalVariant { syntax }),
            TW_STATIC_VARIANT => Self::TwStaticVariant(TwStaticVariant { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::TwArbitraryVariant(it) => &it.syntax,
            Self::TwBogusVariant(it) => &it.syntax,
            Self::TwFunctionalVariant(it) => &it.syntax,
            Self::TwStaticVariant(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::TwArbitraryVariant(it) => it.syntax,
            Self::TwBogusVariant(it) => it.syntax,
            Self::TwFunctionalVariant(it) => it.syntax,
            Self::TwStaticVariant(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyTwVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TwArbitraryVariant(it) => std::fmt::Debug::fmt(it, f),
            Self::TwBogusVariant(it) => std::fmt::Debug::fmt(it, f),
            Self::TwFunctionalVariant(it) => std::fmt::Debug::fmt(it, f),
            Self::TwStaticVariant(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyTwVariant> for SyntaxNode {
    fn from(n: AnyTwVariant) -> Self {
        match n {
            AnyTwVariant::TwArbitraryVariant(it) => it.into(),
            AnyTwVariant::TwBogusVariant(it) => it.into(),
            AnyTwVariant::TwFunctionalVariant(it) => it.into(),
            AnyTwVariant::TwStaticVariant(it) => it.into(),
        }
    }
}
impl From<AnyTwVariant> for SyntaxElement {
    fn from(n: AnyTwVariant) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyTwCandidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyTwFullCandidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyTwModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyTwValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyTwVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TwArbitraryCandidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TwArbitraryValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TwArbitraryVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TwCssVariableValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TwFullCandidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TwFunctionalCandidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TwFunctionalVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TwModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TwNamedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TwRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TwStaticCandidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TwStaticVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct TwBogus {
    syntax: SyntaxNode,
}
impl TwBogus {
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
impl AstNode for TwBogus {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_BOGUS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_BOGUS
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
impl std::fmt::Debug for TwBogus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TwBogus")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<TwBogus> for SyntaxNode {
    fn from(n: TwBogus) -> Self {
        n.syntax
    }
}
impl From<TwBogus> for SyntaxElement {
    fn from(n: TwBogus) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct TwBogusCandidate {
    syntax: SyntaxNode,
}
impl TwBogusCandidate {
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
impl AstNode for TwBogusCandidate {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_BOGUS_CANDIDATE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_BOGUS_CANDIDATE
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
impl std::fmt::Debug for TwBogusCandidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TwBogusCandidate")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<TwBogusCandidate> for SyntaxNode {
    fn from(n: TwBogusCandidate) -> Self {
        n.syntax
    }
}
impl From<TwBogusCandidate> for SyntaxElement {
    fn from(n: TwBogusCandidate) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct TwBogusModifier {
    syntax: SyntaxNode,
}
impl TwBogusModifier {
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
impl AstNode for TwBogusModifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_BOGUS_MODIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_BOGUS_MODIFIER
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
impl std::fmt::Debug for TwBogusModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TwBogusModifier")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<TwBogusModifier> for SyntaxNode {
    fn from(n: TwBogusModifier) -> Self {
        n.syntax
    }
}
impl From<TwBogusModifier> for SyntaxElement {
    fn from(n: TwBogusModifier) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct TwBogusValue {
    syntax: SyntaxNode,
}
impl TwBogusValue {
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
impl AstNode for TwBogusValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_BOGUS_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_BOGUS_VALUE
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
impl std::fmt::Debug for TwBogusValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TwBogusValue")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<TwBogusValue> for SyntaxNode {
    fn from(n: TwBogusValue) -> Self {
        n.syntax
    }
}
impl From<TwBogusValue> for SyntaxElement {
    fn from(n: TwBogusValue) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct TwBogusVariant {
    syntax: SyntaxNode,
}
impl TwBogusVariant {
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
impl AstNode for TwBogusVariant {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_BOGUS_VARIANT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_BOGUS_VARIANT
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
impl std::fmt::Debug for TwBogusVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TwBogusVariant")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<TwBogusVariant> for SyntaxNode {
    fn from(n: TwBogusVariant) -> Self {
        n.syntax
    }
}
impl From<TwBogusVariant> for SyntaxElement {
    fn from(n: TwBogusVariant) -> Self {
        n.syntax.into()
    }
}
biome_rowan::declare_node_union! { pub AnyTwBogusNode = TwBogus | TwBogusCandidate | TwBogusModifier | TwBogusValue | TwBogusVariant }
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TwCandidateList {
    syntax_list: SyntaxList,
}
impl TwCandidateList {
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
impl AstNode for TwCandidateList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_CANDIDATE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_CANDIDATE_LIST
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
impl Serialize for TwCandidateList {
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
impl AstSeparatedList for TwCandidateList {
    type Language = Language;
    type Node = AnyTwFullCandidate;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for TwCandidateList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TwCandidateList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for TwCandidateList {
    type Item = SyntaxResult<AnyTwFullCandidate>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyTwFullCandidate>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &TwCandidateList {
    type Item = SyntaxResult<AnyTwFullCandidate>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyTwFullCandidate>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TwVariantList {
    syntax_list: SyntaxList,
}
impl TwVariantList {
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
impl AstNode for TwVariantList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_VARIANT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_VARIANT_LIST
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
impl Serialize for TwVariantList {
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
impl AstSeparatedList for TwVariantList {
    type Language = Language;
    type Node = AnyTwVariant;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for TwVariantList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TwVariantList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for TwVariantList {
    type Item = SyntaxResult<AnyTwVariant>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyTwVariant>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &TwVariantList {
    type Item = SyntaxResult<AnyTwVariant>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyTwVariant>;
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
