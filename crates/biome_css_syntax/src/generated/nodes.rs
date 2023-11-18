//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::enum_variant_names)]
#![allow(clippy::match_like_matches_macro)]
use crate::{
    macros::map_syntax_node,
    CssLanguage as Language, CssSyntaxElement as SyntaxElement,
    CssSyntaxElementChildren as SyntaxElementChildren,
    CssSyntaxKind::{self as SyntaxKind, *},
    CssSyntaxList as SyntaxList, CssSyntaxNode as SyntaxNode, CssSyntaxToken as SyntaxToken,
};
use biome_rowan::{support, AstNode, RawSyntaxKind, SyntaxKindSet, SyntaxResult};
#[allow(unused)]
use biome_rowan::{
    AstNodeList, AstNodeListIterator, AstSeparatedList, AstSeparatedListNodesIterator,
};
#[cfg(feature = "serde")]
use serde::ser::SerializeSeq;
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};
use std::fmt::{Debug, Formatter};
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAnyFunction {
    pub(crate) syntax: SyntaxNode,
}
impl CssAnyFunction {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssAnyFunctionFields {
        CssAnyFunctionFields {
            css_simple_function: self.css_simple_function(),
        }
    }
    pub fn css_simple_function(&self) -> SyntaxResult<CssSimpleFunction> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAnyFunction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAnyFunctionFields {
    pub css_simple_function: SyntaxResult<CssSimpleFunction>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAtKeyframes {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtKeyframes {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssAtKeyframesFields {
        CssAtKeyframesFields {
            at_token: self.at_token(),
            keyframes_token: self.keyframes_token(),
            name: self.name(),
            css_string: self.css_string(),
            body: self.body(),
        }
    }
    pub fn at_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn keyframes_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn css_string(&self) -> SyntaxResult<CssString> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn body(&self) -> SyntaxResult<CssAtKeyframesBody> {
        support::required_node(&self.syntax, 4usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAtKeyframes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAtKeyframesFields {
    pub at_token: SyntaxResult<SyntaxToken>,
    pub keyframes_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<CssIdentifier>,
    pub css_string: SyntaxResult<CssString>,
    pub body: SyntaxResult<CssAtKeyframesBody>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAtKeyframesBody {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtKeyframesBody {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssAtKeyframesBodyFields {
        CssAtKeyframesBodyFields {
            l_curly_token: self.l_curly_token(),
            items: self.items(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> CssAtKeyframesItemList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAtKeyframesBody {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAtKeyframesBodyFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub items: CssAtKeyframesItemList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAtMedia {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtMedia {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssAtMediaFields {
        CssAtMediaFields {
            at_token: self.at_token(),
            media_token: self.media_token(),
            query_list: self.query_list(),
            l_curly_token: self.l_curly_token(),
            body: self.body(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn at_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn media_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn query_list(&self) -> CssAtMediaQueryList {
        support::list(&self.syntax, 2usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn body(&self) -> SyntaxResult<AnyCssRule> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAtMedia {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAtMediaFields {
    pub at_token: SyntaxResult<SyntaxToken>,
    pub media_token: SyntaxResult<SyntaxToken>,
    pub query_list: CssAtMediaQueryList,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub body: SyntaxResult<AnyCssRule>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAtMediaQuery {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtMediaQuery {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssAtMediaQueryFields {
        CssAtMediaQueryFields {
            condition_token: self.condition_token(),
            or_token: self.or_token(),
            only_token: self.only_token(),
            ty: self.ty(),
            consequent: self.consequent(),
        }
    }
    pub fn condition_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn or_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn only_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
    pub fn ty(&self) -> SyntaxResult<AnyCssAtMediaQueryType> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn consequent(&self) -> Option<CssAtMediaQueryConsequent> {
        support::node(&self.syntax, 4usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAtMediaQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAtMediaQueryFields {
    pub condition_token: SyntaxResult<SyntaxToken>,
    pub or_token: SyntaxResult<SyntaxToken>,
    pub only_token: Option<SyntaxToken>,
    pub ty: SyntaxResult<AnyCssAtMediaQueryType>,
    pub consequent: Option<CssAtMediaQueryConsequent>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAtMediaQueryConsequent {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtMediaQueryConsequent {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssAtMediaQueryConsequentFields {
        CssAtMediaQueryConsequentFields {
            and_token: self.and_token(),
            condition_token: self.condition_token(),
            ty: self.ty(),
        }
    }
    pub fn and_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn condition_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
    pub fn ty(&self) -> SyntaxResult<AnyCssAtMediaQueryType> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAtMediaQueryConsequent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAtMediaQueryConsequentFields {
    pub and_token: SyntaxResult<SyntaxToken>,
    pub condition_token: Option<SyntaxToken>,
    pub ty: SyntaxResult<AnyCssAtMediaQueryType>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAtMediaQueryFeature {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtMediaQueryFeature {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssAtMediaQueryFeatureFields {
        CssAtMediaQueryFeatureFields {
            l_paren_token: self.l_paren_token(),
            feature: self.feature(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn feature(&self) -> SyntaxResult<AnyCssAtMediaQueryFeatureType> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAtMediaQueryFeature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAtMediaQueryFeatureFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub feature: SyntaxResult<AnyCssAtMediaQueryFeatureType>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAtMediaQueryFeatureBoolean {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtMediaQueryFeatureBoolean {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssAtMediaQueryFeatureBooleanFields {
        CssAtMediaQueryFeatureBooleanFields {
            css_identifier: self.css_identifier(),
        }
    }
    pub fn css_identifier(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAtMediaQueryFeatureBoolean {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAtMediaQueryFeatureBooleanFields {
    pub css_identifier: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAtMediaQueryFeatureCompare {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtMediaQueryFeatureCompare {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssAtMediaQueryFeatureCompareFields {
        CssAtMediaQueryFeatureCompareFields {
            name: self.name(),
            range: self.range(),
            value: self.value(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn range(&self) -> SyntaxResult<CssAtMediaQueryRange> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<AnyCssValue> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAtMediaQueryFeatureCompare {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAtMediaQueryFeatureCompareFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub range: SyntaxResult<CssAtMediaQueryRange>,
    pub value: SyntaxResult<AnyCssValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAtMediaQueryFeaturePlain {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtMediaQueryFeaturePlain {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssAtMediaQueryFeaturePlainFields {
        CssAtMediaQueryFeaturePlainFields {
            name: self.name(),
            colon_token: self.colon_token(),
            value: self.value(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<AnyCssValue> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAtMediaQueryFeaturePlain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAtMediaQueryFeaturePlainFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnyCssValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAtMediaQueryFeatureRange {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtMediaQueryFeatureRange {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssAtMediaQueryFeatureRangeFields {
        CssAtMediaQueryFeatureRangeFields {
            first_value: self.first_value(),
            first_range: self.first_range(),
            name: self.name(),
            second_value: self.second_value(),
            second_range: self.second_range(),
        }
    }
    pub fn first_value(&self) -> SyntaxResult<AnyCssValue> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn first_range(&self) -> SyntaxResult<CssAtMediaQueryRange> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn second_value(&self) -> SyntaxResult<AnyCssValue> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn second_range(&self) -> SyntaxResult<CssAtMediaQueryRange> {
        support::required_node(&self.syntax, 4usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAtMediaQueryFeatureRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAtMediaQueryFeatureRangeFields {
    pub first_value: SyntaxResult<AnyCssValue>,
    pub first_range: SyntaxResult<CssAtMediaQueryRange>,
    pub name: SyntaxResult<CssIdentifier>,
    pub second_value: SyntaxResult<AnyCssValue>,
    pub second_range: SyntaxResult<CssAtMediaQueryRange>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAtMediaQueryRange {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtMediaQueryRange {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssAtMediaQueryRangeFields {
        CssAtMediaQueryRangeFields {
            r_angle_token: self.r_angle_token(),
            l_angle_token: self.l_angle_token(),
            greater_than_equal_token: self.greater_than_equal_token(),
            less_than_equal_token: self.less_than_equal_token(),
        }
    }
    pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn greater_than_equal_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn less_than_equal_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAtMediaQueryRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAtMediaQueryRangeFields {
    pub r_angle_token: SyntaxResult<SyntaxToken>,
    pub l_angle_token: SyntaxResult<SyntaxToken>,
    pub greater_than_equal_token: SyntaxResult<SyntaxToken>,
    pub less_than_equal_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAttributeMatcher {
    pub(crate) syntax: SyntaxNode,
}
impl CssAttributeMatcher {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssAttributeMatcherFields {
        CssAttributeMatcherFields {
            operator: self.operator(),
            value: self.value(),
            modifier: self.modifier(),
        }
    }
    pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<CssAttributeMatcherValue> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn modifier(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAttributeMatcher {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAttributeMatcherFields {
    pub operator: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<CssAttributeMatcherValue>,
    pub modifier: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAttributeMatcherValue {
    pub(crate) syntax: SyntaxNode,
}
impl CssAttributeMatcherValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssAttributeMatcherValueFields {
        CssAttributeMatcherValueFields { name: self.name() }
    }
    pub fn name(&self) -> SyntaxResult<AnyCssAttributeMatcherValue> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAttributeMatcherValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAttributeMatcherValueFields {
    pub name: SyntaxResult<AnyCssAttributeMatcherValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAttributeSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssAttributeSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssAttributeSelectorFields {
        CssAttributeSelectorFields {
            l_brack_token: self.l_brack_token(),
            name: self.name(),
            matcher: self.matcher(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn matcher(&self) -> Option<CssAttributeMatcher> {
        support::node(&self.syntax, 2usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAttributeSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAttributeSelectorFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<CssIdentifier>,
    pub matcher: Option<CssAttributeMatcher>,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssBlock {
    pub(crate) syntax: SyntaxNode,
}
impl CssBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssBlockFields {
        CssBlockFields {
            l_curly_token: self.l_curly_token(),
            declaration_list: self.declaration_list(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn declaration_list(&self) -> CssDeclarationList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssBlockFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub declaration_list: CssDeclarationList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssClassSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssClassSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssClassSelectorFields {
        CssClassSelectorFields {
            dot_token: self.dot_token(),
            name: self.name(),
        }
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssClassSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssClassSelectorFields {
    pub dot_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssComplexSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssComplexSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssComplexSelectorFields {
        CssComplexSelectorFields {
            left: self.left(),
            combinator: self.combinator(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyCssSelector> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn combinator(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyCssSelector> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssComplexSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssComplexSelectorFields {
    pub left: SyntaxResult<AnyCssSelector>,
    pub combinator: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyCssSelector>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssCompoundSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssCompoundSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssCompoundSelectorFields {
        CssCompoundSelectorFields {
            nesting_selector_token: self.nesting_selector_token(),
            simple_selector: self.simple_selector(),
            sub_selectors: self.sub_selectors(),
        }
    }
    pub fn nesting_selector_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn simple_selector(&self) -> Option<AnySimpleSelector> {
        support::node(&self.syntax, 1usize)
    }
    pub fn sub_selectors(&self) -> CssSubSelectorList {
        support::list(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssCompoundSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssCompoundSelectorFields {
    pub nesting_selector_token: Option<SyntaxToken>,
    pub simple_selector: Option<AnySimpleSelector>,
    pub sub_selectors: CssSubSelectorList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssCustomProperty {
    pub(crate) syntax: SyntaxNode,
}
impl CssCustomProperty {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssCustomPropertyFields {
        CssCustomPropertyFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssCustomProperty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssCustomPropertyFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssDeclaration {
    pub(crate) syntax: SyntaxNode,
}
impl CssDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssDeclarationFields {
        CssDeclarationFields {
            name: self.name(),
            css_custom_property: self.css_custom_property(),
            colon_token: self.colon_token(),
            value: self.value(),
            important: self.important(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn css_custom_property(&self) -> SyntaxResult<CssCustomProperty> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn value(&self) -> SyntaxResult<AnyCssValue> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn important(&self) -> Option<CssDeclarationImportant> {
        support::node(&self.syntax, 4usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssDeclaration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssDeclarationFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub css_custom_property: SyntaxResult<CssCustomProperty>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnyCssValue>,
    pub important: Option<CssDeclarationImportant>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssDeclarationImportant {
    pub(crate) syntax: SyntaxNode,
}
impl CssDeclarationImportant {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssDeclarationImportantFields {
        CssDeclarationImportantFields {
            excl_token: self.excl_token(),
            important_token: self.important_token(),
        }
    }
    pub fn excl_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn important_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssDeclarationImportant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssDeclarationImportantFields {
    pub excl_token: SyntaxResult<SyntaxToken>,
    pub important_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssDimension {
    pub(crate) syntax: SyntaxNode,
}
impl CssDimension {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssDimensionFields {
        CssDimensionFields {
            value: self.value(),
            unit: self.unit(),
        }
    }
    pub fn value(&self) -> SyntaxResult<CssNumber> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn unit(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssDimension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssDimensionFields {
    pub value: SyntaxResult<CssNumber>,
    pub unit: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssIdSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssIdSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssIdSelectorFields {
        CssIdSelectorFields {
            hash_token: self.hash_token(),
            name: self.name(),
        }
    }
    pub fn hash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssIdSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssIdSelectorFields {
    pub hash_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssIdentifier {
    pub(crate) syntax: SyntaxNode,
}
impl CssIdentifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssIdentifierFields {
        CssIdentifierFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssIdentifierFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssKeyframesBlock {
    pub(crate) syntax: SyntaxNode,
}
impl CssKeyframesBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssKeyframesBlockFields {
        CssKeyframesBlockFields {
            selectors: self.selectors(),
            l_curly_token: self.l_curly_token(),
            declarations: self.declarations(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn selectors(&self) -> CssKeyframesSelectorList {
        support::list(&self.syntax, 0usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn declarations(&self) -> CssDeclarationList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssKeyframesBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssKeyframesBlockFields {
    pub selectors: CssKeyframesSelectorList,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub declarations: CssDeclarationList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssKeyframesSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssKeyframesSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssKeyframesSelectorFields {
        CssKeyframesSelectorFields {
            from_token: self.from_token(),
            to_token: self.to_token(),
            css_percentage: self.css_percentage(),
        }
    }
    pub fn from_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn to_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn css_percentage(&self) -> SyntaxResult<CssPercentage> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssKeyframesSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssKeyframesSelectorFields {
    pub from_token: SyntaxResult<SyntaxToken>,
    pub to_token: SyntaxResult<SyntaxToken>,
    pub css_percentage: SyntaxResult<CssPercentage>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssNthMultiplier {
    pub(crate) syntax: SyntaxNode,
}
impl CssNthMultiplier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssNthMultiplierFields {
        CssNthMultiplierFields {
            sign: self.sign(),
            value: self.value(),
        }
    }
    pub fn sign(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> Option<CssNumber> {
        support::node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssNthMultiplier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssNthMultiplierFields {
    pub sign: SyntaxResult<SyntaxToken>,
    pub value: Option<CssNumber>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssNthOffset {
    pub(crate) syntax: SyntaxNode,
}
impl CssNthOffset {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssNthOffsetFields {
        CssNthOffsetFields {
            sign: self.sign(),
            value: self.value(),
        }
    }
    pub fn sign(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<CssNumber> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssNthOffset {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssNthOffsetFields {
    pub sign: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<CssNumber>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssNumber {
    pub(crate) syntax: SyntaxNode,
}
impl CssNumber {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssNumberFields {
        CssNumberFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssNumberFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssParameter {
    pub(crate) syntax: SyntaxNode,
}
impl CssParameter {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssParameterFields {
        CssParameterFields {
            any_css_value: self.any_css_value(),
        }
    }
    pub fn any_css_value(&self) -> SyntaxResult<AnyCssValue> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssParameter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssParameterFields {
    pub any_css_value: SyntaxResult<AnyCssValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPercentage {
    pub(crate) syntax: SyntaxNode,
}
impl CssPercentage {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPercentageFields {
        CssPercentageFields {
            value: self.value(),
            reminder_token: self.reminder_token(),
        }
    }
    pub fn value(&self) -> SyntaxResult<CssNumber> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn reminder_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPercentage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPercentageFields {
    pub value: SyntaxResult<CssNumber>,
    pub reminder_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassFunctionCompoundSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassFunctionCompoundSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassFunctionCompoundSelectorFields {
        CssPseudoClassFunctionCompoundSelectorFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            selector: self.selector(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn selector(&self) -> SyntaxResult<AnyCssCompoundSelector> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassFunctionCompoundSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassFunctionCompoundSelectorFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub selector: SyntaxResult<AnyCssCompoundSelector>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassFunctionCompoundSelectorList {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassFunctionCompoundSelectorList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassFunctionCompoundSelectorListFields {
        CssPseudoClassFunctionCompoundSelectorListFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            compound_selector_list: self.compound_selector_list(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn compound_selector_list(&self) -> CssCompoundSelectorList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassFunctionCompoundSelectorList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassFunctionCompoundSelectorListFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub compound_selector_list: CssCompoundSelectorList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassFunctionIdentifier {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassFunctionIdentifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassFunctionIdentifierFields {
        CssPseudoClassFunctionIdentifierFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            ident: self.ident(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn ident(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassFunctionIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassFunctionIdentifierFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub ident: SyntaxResult<CssIdentifier>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassFunctionNth {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassFunctionNth {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassFunctionNthFields {
        CssPseudoClassFunctionNthFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            selector: self.selector(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn selector(&self) -> SyntaxResult<AnyCssPseudoClassNthSelector> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassFunctionNth {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassFunctionNthFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub selector: SyntaxResult<AnyCssPseudoClassNthSelector>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassFunctionRelativeSelectorList {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassFunctionRelativeSelectorList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassFunctionRelativeSelectorListFields {
        CssPseudoClassFunctionRelativeSelectorListFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            relative_selector_list: self.relative_selector_list(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn relative_selector_list(&self) -> CssRelativeSelectorList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassFunctionRelativeSelectorList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassFunctionRelativeSelectorListFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub relative_selector_list: CssRelativeSelectorList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassFunctionSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassFunctionSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassFunctionSelectorFields {
        CssPseudoClassFunctionSelectorFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            selector: self.selector(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn selector(&self) -> SyntaxResult<AnyCssSelector> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassFunctionSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassFunctionSelectorFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub selector: SyntaxResult<AnyCssSelector>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassFunctionSelectorList {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassFunctionSelectorList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassFunctionSelectorListFields {
        CssPseudoClassFunctionSelectorListFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            selector_list: self.selector_list(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn selector_list(&self) -> CssSelectorList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassFunctionSelectorList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassFunctionSelectorListFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub selector_list: CssSelectorList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassFunctionValueList {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassFunctionValueList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassFunctionValueListFields {
        CssPseudoClassFunctionValueListFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            value_list: self.value_list(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value_list(&self) -> CssPseudoValueList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassFunctionValueList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassFunctionValueListFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub value_list: CssPseudoValueList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassIdentifier {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassIdentifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassIdentifierFields {
        CssPseudoClassIdentifierFields { name: self.name() }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassIdentifierFields {
    pub name: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassNth {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassNth {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassNthFields {
        CssPseudoClassNthFields {
            sign: self.sign(),
            value: self.value(),
            symbol_token: self.symbol_token(),
            offset: self.offset(),
        }
    }
    pub fn sign(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> Option<CssNumber> {
        support::node(&self.syntax, 1usize)
    }
    pub fn symbol_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn offset(&self) -> Option<CssNthOffset> {
        support::node(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassNth {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassNthFields {
    pub sign: Option<SyntaxToken>,
    pub value: Option<CssNumber>,
    pub symbol_token: SyntaxResult<SyntaxToken>,
    pub offset: Option<CssNthOffset>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassNthIdentifier {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassNthIdentifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassNthIdentifierFields {
        CssPseudoClassNthIdentifierFields {
            value: self.value(),
        }
    }
    pub fn value(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassNthIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassNthIdentifierFields {
    pub value: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassNthNumber {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassNthNumber {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassNthNumberFields {
        CssPseudoClassNthNumberFields {
            sign: self.sign(),
            value: self.value(),
        }
    }
    pub fn sign(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<CssNumber> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassNthNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassNthNumberFields {
    pub sign: Option<SyntaxToken>,
    pub value: SyntaxResult<CssNumber>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassNthSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassNthSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassNthSelectorFields {
        CssPseudoClassNthSelectorFields {
            nth: self.nth(),
            of_selector: self.of_selector(),
        }
    }
    pub fn nth(&self) -> SyntaxResult<AnyCssPseudoClassNth> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn of_selector(&self) -> Option<CssPseudoClassOfNthSelector> {
        support::node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassNthSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassNthSelectorFields {
    pub nth: SyntaxResult<AnyCssPseudoClassNth>,
    pub of_selector: Option<CssPseudoClassOfNthSelector>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassOfNthSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassOfNthSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassOfNthSelectorFields {
        CssPseudoClassOfNthSelectorFields {
            of_token: self.of_token(),
            selector_list: self.selector_list(),
        }
    }
    pub fn of_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn selector_list(&self) -> CssSelectorList {
        support::list(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassOfNthSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassOfNthSelectorFields {
    pub of_token: SyntaxResult<SyntaxToken>,
    pub selector_list: CssSelectorList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassSelectorFields {
        CssPseudoClassSelectorFields {
            colon_token: self.colon_token(),
            class: self.class(),
        }
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn class(&self) -> SyntaxResult<AnyCssPseudoClass> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassSelectorFields {
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub class: SyntaxResult<AnyCssPseudoClass>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoElementFunctionIdentifier {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoElementFunctionIdentifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoElementFunctionIdentifierFields {
        CssPseudoElementFunctionIdentifierFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            ident: self.ident(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn ident(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoElementFunctionIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoElementFunctionIdentifierFields {
    pub name: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub ident: SyntaxResult<CssIdentifier>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoElementFunctionSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoElementFunctionSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoElementFunctionSelectorFields {
        CssPseudoElementFunctionSelectorFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            selector: self.selector(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn selector(&self) -> SyntaxResult<AnyCssSelector> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoElementFunctionSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoElementFunctionSelectorFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub selector: SyntaxResult<AnyCssSelector>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoElementIdentifier {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoElementIdentifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoElementIdentifierFields {
        CssPseudoElementIdentifierFields { name: self.name() }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoElementIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoElementIdentifierFields {
    pub name: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoElementSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoElementSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoElementSelectorFields {
        CssPseudoElementSelectorFields {
            double_colon_token: self.double_colon_token(),
            element: self.element(),
        }
    }
    pub fn double_colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn element(&self) -> SyntaxResult<AnyCssPseudoElement> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoElementSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoElementSelectorFields {
    pub double_colon_token: SyntaxResult<SyntaxToken>,
    pub element: SyntaxResult<AnyCssPseudoElement>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssRatio {
    pub(crate) syntax: SyntaxNode,
}
impl CssRatio {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssRatioFields {
        CssRatioFields {
            numerator: self.numerator(),
            denominator: self.denominator(),
        }
    }
    pub fn numerator(&self) -> SyntaxResult<CssNumber> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn denominator(&self) -> SyntaxResult<CssNumber> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssRatio {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssRatioFields {
    pub numerator: SyntaxResult<CssNumber>,
    pub denominator: SyntaxResult<CssNumber>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssRelativeSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssRelativeSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssRelativeSelectorFields {
        CssRelativeSelectorFields {
            combinator: self.combinator(),
            selector: self.selector(),
        }
    }
    pub fn combinator(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn selector(&self) -> SyntaxResult<AnyCssSelector> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssRelativeSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssRelativeSelectorFields {
    pub combinator: Option<SyntaxToken>,
    pub selector: SyntaxResult<AnyCssSelector>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssRoot {
    pub(crate) syntax: SyntaxNode,
}
impl CssRoot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssRootFields {
        CssRootFields {
            bom_token: self.bom_token(),
            rules: self.rules(),
            eof_token: self.eof_token(),
        }
    }
    pub fn bom_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn rules(&self) -> CssRuleList {
        support::list(&self.syntax, 1usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssRootFields {
    pub bom_token: Option<SyntaxToken>,
    pub rules: CssRuleList,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssRuleFields {
        CssRuleFields {
            prelude: self.prelude(),
            block: self.block(),
        }
    }
    pub fn prelude(&self) -> CssSelectorList {
        support::list(&self.syntax, 0usize)
    }
    pub fn block(&self) -> SyntaxResult<CssBlock> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssRuleFields {
    pub prelude: CssSelectorList,
    pub block: SyntaxResult<CssBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssSimpleFunction {
    pub(crate) syntax: SyntaxNode,
}
impl CssSimpleFunction {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssSimpleFunctionFields {
        CssSimpleFunctionFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            items: self.items(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn items(&self) -> CssParameterList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssSimpleFunction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssSimpleFunctionFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub items: CssParameterList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssString {
    pub(crate) syntax: SyntaxNode,
}
impl CssString {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssStringFields {
        CssStringFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssStringFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssTypeSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssTypeSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssTypeSelectorFields {
        CssTypeSelectorFields {
            ident: self.ident(),
        }
    }
    pub fn ident(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssTypeSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssTypeSelectorFields {
    pub ident: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssUniversalSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssUniversalSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssUniversalSelectorFields {
        CssUniversalSelectorFields {
            star_token: self.star_token(),
        }
    }
    pub fn star_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssUniversalSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssUniversalSelectorFields {
    pub star_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssVarFunction {
    pub(crate) syntax: SyntaxNode,
}
impl CssVarFunction {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssVarFunctionFields {
        CssVarFunctionFields {
            var_token: self.var_token(),
            l_paren_token: self.l_paren_token(),
            property: self.property(),
            value: self.value(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn var_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn property(&self) -> SyntaxResult<CssCustomProperty> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn value(&self) -> Option<CssVarFunctionValue> {
        support::node(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssVarFunction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssVarFunctionFields {
    pub var_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub property: SyntaxResult<CssCustomProperty>,
    pub value: Option<CssVarFunctionValue>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssVarFunctionValue {
    pub(crate) syntax: SyntaxNode,
}
impl CssVarFunctionValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssVarFunctionValueFields {
        CssVarFunctionValueFields {
            comma_token: self.comma_token(),
            value: self.value(),
        }
    }
    pub fn comma_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssVarFunctionValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssVarFunctionValueFields {
    pub comma_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssAtMediaQueryFeatureType {
    CssAtMediaQueryFeatureBoolean(CssAtMediaQueryFeatureBoolean),
    CssAtMediaQueryFeatureCompare(CssAtMediaQueryFeatureCompare),
    CssAtMediaQueryFeaturePlain(CssAtMediaQueryFeaturePlain),
    CssAtMediaQueryFeatureRange(CssAtMediaQueryFeatureRange),
}
impl AnyCssAtMediaQueryFeatureType {
    pub fn as_css_at_media_query_feature_boolean(&self) -> Option<&CssAtMediaQueryFeatureBoolean> {
        match &self {
            AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeatureBoolean(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_at_media_query_feature_compare(&self) -> Option<&CssAtMediaQueryFeatureCompare> {
        match &self {
            AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeatureCompare(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_at_media_query_feature_plain(&self) -> Option<&CssAtMediaQueryFeaturePlain> {
        match &self {
            AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeaturePlain(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_at_media_query_feature_range(&self) -> Option<&CssAtMediaQueryFeatureRange> {
        match &self {
            AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeatureRange(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssAtMediaQueryType {
    CssAtMediaQueryFeature(CssAtMediaQueryFeature),
    CssIdentifier(CssIdentifier),
}
impl AnyCssAtMediaQueryType {
    pub fn as_css_at_media_query_feature(&self) -> Option<&CssAtMediaQueryFeature> {
        match &self {
            AnyCssAtMediaQueryType::CssAtMediaQueryFeature(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_identifier(&self) -> Option<&CssIdentifier> {
        match &self {
            AnyCssAtMediaQueryType::CssIdentifier(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssAtRule {
    CssAtKeyframes(CssAtKeyframes),
    CssAtMedia(CssAtMedia),
}
impl AnyCssAtRule {
    pub fn as_css_at_keyframes(&self) -> Option<&CssAtKeyframes> {
        match &self {
            AnyCssAtRule::CssAtKeyframes(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_at_media(&self) -> Option<&CssAtMedia> {
        match &self {
            AnyCssAtRule::CssAtMedia(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssAttributeMatcherValue {
    CssIdentifier(CssIdentifier),
    CssString(CssString),
}
impl AnyCssAttributeMatcherValue {
    pub fn as_css_identifier(&self) -> Option<&CssIdentifier> {
        match &self {
            AnyCssAttributeMatcherValue::CssIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_string(&self) -> Option<&CssString> {
        match &self {
            AnyCssAttributeMatcherValue::CssString(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssCompoundSelector {
    CssBogusSelector(CssBogusSelector),
    CssCompoundSelector(CssCompoundSelector),
}
impl AnyCssCompoundSelector {
    pub fn as_css_bogus_selector(&self) -> Option<&CssBogusSelector> {
        match &self {
            AnyCssCompoundSelector::CssBogusSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_compound_selector(&self) -> Option<&CssCompoundSelector> {
        match &self {
            AnyCssCompoundSelector::CssCompoundSelector(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssNthMultiplier {
    CssNthMultiplier(CssNthMultiplier),
    CssNumber(CssNumber),
}
impl AnyCssNthMultiplier {
    pub fn as_css_nth_multiplier(&self) -> Option<&CssNthMultiplier> {
        match &self {
            AnyCssNthMultiplier::CssNthMultiplier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_number(&self) -> Option<&CssNumber> {
        match &self {
            AnyCssNthMultiplier::CssNumber(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssPseudoClass {
    CssPseudoClassFunctionCompoundSelector(CssPseudoClassFunctionCompoundSelector),
    CssPseudoClassFunctionCompoundSelectorList(CssPseudoClassFunctionCompoundSelectorList),
    CssPseudoClassFunctionIdentifier(CssPseudoClassFunctionIdentifier),
    CssPseudoClassFunctionNth(CssPseudoClassFunctionNth),
    CssPseudoClassFunctionRelativeSelectorList(CssPseudoClassFunctionRelativeSelectorList),
    CssPseudoClassFunctionSelector(CssPseudoClassFunctionSelector),
    CssPseudoClassFunctionSelectorList(CssPseudoClassFunctionSelectorList),
    CssPseudoClassFunctionValueList(CssPseudoClassFunctionValueList),
    CssPseudoClassIdentifier(CssPseudoClassIdentifier),
}
impl AnyCssPseudoClass {
    pub fn as_css_pseudo_class_function_compound_selector(
        &self,
    ) -> Option<&CssPseudoClassFunctionCompoundSelector> {
        match &self {
            AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_function_compound_selector_list(
        &self,
    ) -> Option<&CssPseudoClassFunctionCompoundSelectorList> {
        match &self {
            AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelectorList(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_function_identifier(
        &self,
    ) -> Option<&CssPseudoClassFunctionIdentifier> {
        match &self {
            AnyCssPseudoClass::CssPseudoClassFunctionIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_function_nth(&self) -> Option<&CssPseudoClassFunctionNth> {
        match &self {
            AnyCssPseudoClass::CssPseudoClassFunctionNth(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_function_relative_selector_list(
        &self,
    ) -> Option<&CssPseudoClassFunctionRelativeSelectorList> {
        match &self {
            AnyCssPseudoClass::CssPseudoClassFunctionRelativeSelectorList(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_function_selector(&self) -> Option<&CssPseudoClassFunctionSelector> {
        match &self {
            AnyCssPseudoClass::CssPseudoClassFunctionSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_function_selector_list(
        &self,
    ) -> Option<&CssPseudoClassFunctionSelectorList> {
        match &self {
            AnyCssPseudoClass::CssPseudoClassFunctionSelectorList(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_function_value_list(
        &self,
    ) -> Option<&CssPseudoClassFunctionValueList> {
        match &self {
            AnyCssPseudoClass::CssPseudoClassFunctionValueList(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_identifier(&self) -> Option<&CssPseudoClassIdentifier> {
        match &self {
            AnyCssPseudoClass::CssPseudoClassIdentifier(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssPseudoClassNth {
    CssPseudoClassNth(CssPseudoClassNth),
    CssPseudoClassNthIdentifier(CssPseudoClassNthIdentifier),
    CssPseudoClassNthNumber(CssPseudoClassNthNumber),
}
impl AnyCssPseudoClassNth {
    pub fn as_css_pseudo_class_nth(&self) -> Option<&CssPseudoClassNth> {
        match &self {
            AnyCssPseudoClassNth::CssPseudoClassNth(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_nth_identifier(&self) -> Option<&CssPseudoClassNthIdentifier> {
        match &self {
            AnyCssPseudoClassNth::CssPseudoClassNthIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_nth_number(&self) -> Option<&CssPseudoClassNthNumber> {
        match &self {
            AnyCssPseudoClassNth::CssPseudoClassNthNumber(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssPseudoClassNthSelector {
    CssBogusSelector(CssBogusSelector),
    CssPseudoClassNthSelector(CssPseudoClassNthSelector),
}
impl AnyCssPseudoClassNthSelector {
    pub fn as_css_bogus_selector(&self) -> Option<&CssBogusSelector> {
        match &self {
            AnyCssPseudoClassNthSelector::CssBogusSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_nth_selector(&self) -> Option<&CssPseudoClassNthSelector> {
        match &self {
            AnyCssPseudoClassNthSelector::CssPseudoClassNthSelector(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssPseudoElement {
    CssPseudoElementFunctionIdentifier(CssPseudoElementFunctionIdentifier),
    CssPseudoElementFunctionSelector(CssPseudoElementFunctionSelector),
    CssPseudoElementIdentifier(CssPseudoElementIdentifier),
}
impl AnyCssPseudoElement {
    pub fn as_css_pseudo_element_function_identifier(
        &self,
    ) -> Option<&CssPseudoElementFunctionIdentifier> {
        match &self {
            AnyCssPseudoElement::CssPseudoElementFunctionIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_element_function_selector(
        &self,
    ) -> Option<&CssPseudoElementFunctionSelector> {
        match &self {
            AnyCssPseudoElement::CssPseudoElementFunctionSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_element_identifier(&self) -> Option<&CssPseudoElementIdentifier> {
        match &self {
            AnyCssPseudoElement::CssPseudoElementIdentifier(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssPseudoValue {
    CssIdentifier(CssIdentifier),
    CssString(CssString),
}
impl AnyCssPseudoValue {
    pub fn as_css_identifier(&self) -> Option<&CssIdentifier> {
        match &self {
            AnyCssPseudoValue::CssIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_string(&self) -> Option<&CssString> {
        match &self {
            AnyCssPseudoValue::CssString(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssRelativeSelector {
    CssBogusSelector(CssBogusSelector),
    CssRelativeSelector(CssRelativeSelector),
}
impl AnyCssRelativeSelector {
    pub fn as_css_bogus_selector(&self) -> Option<&CssBogusSelector> {
        match &self {
            AnyCssRelativeSelector::CssBogusSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_relative_selector(&self) -> Option<&CssRelativeSelector> {
        match &self {
            AnyCssRelativeSelector::CssRelativeSelector(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssRule {
    AnyCssAtRule(AnyCssAtRule),
    CssBogusRule(CssBogusRule),
    CssRule(CssRule),
}
impl AnyCssRule {
    pub fn as_any_css_at_rule(&self) -> Option<&AnyCssAtRule> {
        match &self {
            AnyCssRule::AnyCssAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_bogus_rule(&self) -> Option<&CssBogusRule> {
        match &self {
            AnyCssRule::CssBogusRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_rule(&self) -> Option<&CssRule> {
        match &self {
            AnyCssRule::CssRule(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssSelector {
    CssBogusSelector(CssBogusSelector),
    CssComplexSelector(CssComplexSelector),
    CssCompoundSelector(CssCompoundSelector),
}
impl AnyCssSelector {
    pub fn as_css_bogus_selector(&self) -> Option<&CssBogusSelector> {
        match &self {
            AnyCssSelector::CssBogusSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_complex_selector(&self) -> Option<&CssComplexSelector> {
        match &self {
            AnyCssSelector::CssComplexSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_compound_selector(&self) -> Option<&CssCompoundSelector> {
        match &self {
            AnyCssSelector::CssCompoundSelector(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssSubSelector {
    CssAttributeSelector(CssAttributeSelector),
    CssBogusSubSelector(CssBogusSubSelector),
    CssClassSelector(CssClassSelector),
    CssIdSelector(CssIdSelector),
    CssPseudoClassSelector(CssPseudoClassSelector),
    CssPseudoElementSelector(CssPseudoElementSelector),
}
impl AnyCssSubSelector {
    pub fn as_css_attribute_selector(&self) -> Option<&CssAttributeSelector> {
        match &self {
            AnyCssSubSelector::CssAttributeSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_bogus_sub_selector(&self) -> Option<&CssBogusSubSelector> {
        match &self {
            AnyCssSubSelector::CssBogusSubSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_class_selector(&self) -> Option<&CssClassSelector> {
        match &self {
            AnyCssSubSelector::CssClassSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_id_selector(&self) -> Option<&CssIdSelector> {
        match &self {
            AnyCssSubSelector::CssIdSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_selector(&self) -> Option<&CssPseudoClassSelector> {
        match &self {
            AnyCssSubSelector::CssPseudoClassSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_element_selector(&self) -> Option<&CssPseudoElementSelector> {
        match &self {
            AnyCssSubSelector::CssPseudoElementSelector(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssValue {
    CssAnyFunction(CssAnyFunction),
    CssCustomProperty(CssCustomProperty),
    CssDimension(CssDimension),
    CssIdentifier(CssIdentifier),
    CssNumber(CssNumber),
    CssRatio(CssRatio),
    CssString(CssString),
}
impl AnyCssValue {
    pub fn as_css_any_function(&self) -> Option<&CssAnyFunction> {
        match &self {
            AnyCssValue::CssAnyFunction(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_custom_property(&self) -> Option<&CssCustomProperty> {
        match &self {
            AnyCssValue::CssCustomProperty(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_dimension(&self) -> Option<&CssDimension> {
        match &self {
            AnyCssValue::CssDimension(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_identifier(&self) -> Option<&CssIdentifier> {
        match &self {
            AnyCssValue::CssIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_number(&self) -> Option<&CssNumber> {
        match &self {
            AnyCssValue::CssNumber(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_ratio(&self) -> Option<&CssRatio> {
        match &self {
            AnyCssValue::CssRatio(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_string(&self) -> Option<&CssString> {
        match &self {
            AnyCssValue::CssString(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnySimpleSelector {
    CssTypeSelector(CssTypeSelector),
    CssUniversalSelector(CssUniversalSelector),
}
impl AnySimpleSelector {
    pub fn as_css_type_selector(&self) -> Option<&CssTypeSelector> {
        match &self {
            AnySimpleSelector::CssTypeSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_universal_selector(&self) -> Option<&CssUniversalSelector> {
        match &self {
            AnySimpleSelector::CssUniversalSelector(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for CssAnyFunction {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_ANY_FUNCTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_ANY_FUNCTION
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
impl std::fmt::Debug for CssAnyFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAnyFunction")
            .field(
                "css_simple_function",
                &support::DebugSyntaxResult(self.css_simple_function()),
            )
            .finish()
    }
}
impl From<CssAnyFunction> for SyntaxNode {
    fn from(n: CssAnyFunction) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssAnyFunction> for SyntaxElement {
    fn from(n: CssAnyFunction) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssAtKeyframes {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_KEYFRAMES as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_AT_KEYFRAMES
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
impl std::fmt::Debug for CssAtKeyframes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAtKeyframes")
            .field("at_token", &support::DebugSyntaxResult(self.at_token()))
            .field(
                "keyframes_token",
                &support::DebugSyntaxResult(self.keyframes_token()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field("css_string", &support::DebugSyntaxResult(self.css_string()))
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<CssAtKeyframes> for SyntaxNode {
    fn from(n: CssAtKeyframes) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssAtKeyframes> for SyntaxElement {
    fn from(n: CssAtKeyframes) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssAtKeyframesBody {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_KEYFRAMES_BODY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_AT_KEYFRAMES_BODY
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
impl std::fmt::Debug for CssAtKeyframesBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAtKeyframesBody")
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("items", &self.items())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<CssAtKeyframesBody> for SyntaxNode {
    fn from(n: CssAtKeyframesBody) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssAtKeyframesBody> for SyntaxElement {
    fn from(n: CssAtKeyframesBody) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssAtMedia {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_MEDIA as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_AT_MEDIA
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
impl std::fmt::Debug for CssAtMedia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAtMedia")
            .field("at_token", &support::DebugSyntaxResult(self.at_token()))
            .field(
                "media_token",
                &support::DebugSyntaxResult(self.media_token()),
            )
            .field("query_list", &self.query_list())
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<CssAtMedia> for SyntaxNode {
    fn from(n: CssAtMedia) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssAtMedia> for SyntaxElement {
    fn from(n: CssAtMedia) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssAtMediaQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_MEDIA_QUERY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_AT_MEDIA_QUERY
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
impl std::fmt::Debug for CssAtMediaQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAtMediaQuery")
            .field(
                "condition_token",
                &support::DebugSyntaxResult(self.condition_token()),
            )
            .field("or_token", &support::DebugSyntaxResult(self.or_token()))
            .field(
                "only_token",
                &support::DebugOptionalElement(self.only_token()),
            )
            .field("ty", &support::DebugSyntaxResult(self.ty()))
            .field(
                "consequent",
                &support::DebugOptionalElement(self.consequent()),
            )
            .finish()
    }
}
impl From<CssAtMediaQuery> for SyntaxNode {
    fn from(n: CssAtMediaQuery) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssAtMediaQuery> for SyntaxElement {
    fn from(n: CssAtMediaQuery) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssAtMediaQueryConsequent {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_MEDIA_QUERY_CONSEQUENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_AT_MEDIA_QUERY_CONSEQUENT
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
impl std::fmt::Debug for CssAtMediaQueryConsequent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAtMediaQueryConsequent")
            .field("and_token", &support::DebugSyntaxResult(self.and_token()))
            .field(
                "condition_token",
                &support::DebugOptionalElement(self.condition_token()),
            )
            .field("ty", &support::DebugSyntaxResult(self.ty()))
            .finish()
    }
}
impl From<CssAtMediaQueryConsequent> for SyntaxNode {
    fn from(n: CssAtMediaQueryConsequent) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssAtMediaQueryConsequent> for SyntaxElement {
    fn from(n: CssAtMediaQueryConsequent) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssAtMediaQueryFeature {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_MEDIA_QUERY_FEATURE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_AT_MEDIA_QUERY_FEATURE
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
impl std::fmt::Debug for CssAtMediaQueryFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAtMediaQueryFeature")
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("feature", &support::DebugSyntaxResult(self.feature()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssAtMediaQueryFeature> for SyntaxNode {
    fn from(n: CssAtMediaQueryFeature) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssAtMediaQueryFeature> for SyntaxElement {
    fn from(n: CssAtMediaQueryFeature) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssAtMediaQueryFeatureBoolean {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_MEDIA_QUERY_FEATURE_BOOLEAN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_AT_MEDIA_QUERY_FEATURE_BOOLEAN
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
impl std::fmt::Debug for CssAtMediaQueryFeatureBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAtMediaQueryFeatureBoolean")
            .field(
                "css_identifier",
                &support::DebugSyntaxResult(self.css_identifier()),
            )
            .finish()
    }
}
impl From<CssAtMediaQueryFeatureBoolean> for SyntaxNode {
    fn from(n: CssAtMediaQueryFeatureBoolean) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssAtMediaQueryFeatureBoolean> for SyntaxElement {
    fn from(n: CssAtMediaQueryFeatureBoolean) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssAtMediaQueryFeatureCompare {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_MEDIA_QUERY_FEATURE_COMPARE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_AT_MEDIA_QUERY_FEATURE_COMPARE
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
impl std::fmt::Debug for CssAtMediaQueryFeatureCompare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAtMediaQueryFeatureCompare")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field("range", &support::DebugSyntaxResult(self.range()))
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<CssAtMediaQueryFeatureCompare> for SyntaxNode {
    fn from(n: CssAtMediaQueryFeatureCompare) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssAtMediaQueryFeatureCompare> for SyntaxElement {
    fn from(n: CssAtMediaQueryFeatureCompare) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssAtMediaQueryFeaturePlain {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_MEDIA_QUERY_FEATURE_PLAIN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_AT_MEDIA_QUERY_FEATURE_PLAIN
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
impl std::fmt::Debug for CssAtMediaQueryFeaturePlain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAtMediaQueryFeaturePlain")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<CssAtMediaQueryFeaturePlain> for SyntaxNode {
    fn from(n: CssAtMediaQueryFeaturePlain) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssAtMediaQueryFeaturePlain> for SyntaxElement {
    fn from(n: CssAtMediaQueryFeaturePlain) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssAtMediaQueryFeatureRange {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_MEDIA_QUERY_FEATURE_RANGE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_AT_MEDIA_QUERY_FEATURE_RANGE
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
impl std::fmt::Debug for CssAtMediaQueryFeatureRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAtMediaQueryFeatureRange")
            .field(
                "first_value",
                &support::DebugSyntaxResult(self.first_value()),
            )
            .field(
                "first_range",
                &support::DebugSyntaxResult(self.first_range()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "second_value",
                &support::DebugSyntaxResult(self.second_value()),
            )
            .field(
                "second_range",
                &support::DebugSyntaxResult(self.second_range()),
            )
            .finish()
    }
}
impl From<CssAtMediaQueryFeatureRange> for SyntaxNode {
    fn from(n: CssAtMediaQueryFeatureRange) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssAtMediaQueryFeatureRange> for SyntaxElement {
    fn from(n: CssAtMediaQueryFeatureRange) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssAtMediaQueryRange {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_MEDIA_QUERY_RANGE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_AT_MEDIA_QUERY_RANGE
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
impl std::fmt::Debug for CssAtMediaQueryRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAtMediaQueryRange")
            .field(
                "r_angle_token",
                &support::DebugSyntaxResult(self.r_angle_token()),
            )
            .field(
                "l_angle_token",
                &support::DebugSyntaxResult(self.l_angle_token()),
            )
            .field(
                "greater_than_equal_token",
                &support::DebugSyntaxResult(self.greater_than_equal_token()),
            )
            .field(
                "less_than_equal_token",
                &support::DebugSyntaxResult(self.less_than_equal_token()),
            )
            .finish()
    }
}
impl From<CssAtMediaQueryRange> for SyntaxNode {
    fn from(n: CssAtMediaQueryRange) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssAtMediaQueryRange> for SyntaxElement {
    fn from(n: CssAtMediaQueryRange) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssAttributeMatcher {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_ATTRIBUTE_MATCHER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_ATTRIBUTE_MATCHER
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
impl std::fmt::Debug for CssAttributeMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAttributeMatcher")
            .field("operator", &support::DebugSyntaxResult(self.operator()))
            .field("value", &support::DebugSyntaxResult(self.value()))
            .field("modifier", &support::DebugOptionalElement(self.modifier()))
            .finish()
    }
}
impl From<CssAttributeMatcher> for SyntaxNode {
    fn from(n: CssAttributeMatcher) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssAttributeMatcher> for SyntaxElement {
    fn from(n: CssAttributeMatcher) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssAttributeMatcherValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_ATTRIBUTE_MATCHER_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_ATTRIBUTE_MATCHER_VALUE
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
impl std::fmt::Debug for CssAttributeMatcherValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAttributeMatcherValue")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .finish()
    }
}
impl From<CssAttributeMatcherValue> for SyntaxNode {
    fn from(n: CssAttributeMatcherValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssAttributeMatcherValue> for SyntaxElement {
    fn from(n: CssAttributeMatcherValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssAttributeSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_ATTRIBUTE_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_ATTRIBUTE_SELECTOR
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
impl std::fmt::Debug for CssAttributeSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAttributeSelector")
            .field(
                "l_brack_token",
                &support::DebugSyntaxResult(self.l_brack_token()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field("matcher", &support::DebugOptionalElement(self.matcher()))
            .field(
                "r_brack_token",
                &support::DebugSyntaxResult(self.r_brack_token()),
            )
            .finish()
    }
}
impl From<CssAttributeSelector> for SyntaxNode {
    fn from(n: CssAttributeSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssAttributeSelector> for SyntaxElement {
    fn from(n: CssAttributeSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BLOCK
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
impl std::fmt::Debug for CssBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBlock")
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("declaration_list", &self.declaration_list())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<CssBlock> for SyntaxNode {
    fn from(n: CssBlock) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssBlock> for SyntaxElement {
    fn from(n: CssBlock) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssClassSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CLASS_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CLASS_SELECTOR
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
impl std::fmt::Debug for CssClassSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssClassSelector")
            .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
            .field("name", &support::DebugSyntaxResult(self.name()))
            .finish()
    }
}
impl From<CssClassSelector> for SyntaxNode {
    fn from(n: CssClassSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssClassSelector> for SyntaxElement {
    fn from(n: CssClassSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssComplexSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_COMPLEX_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_COMPLEX_SELECTOR
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
impl std::fmt::Debug for CssComplexSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssComplexSelector")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field("combinator", &support::DebugSyntaxResult(self.combinator()))
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<CssComplexSelector> for SyntaxNode {
    fn from(n: CssComplexSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssComplexSelector> for SyntaxElement {
    fn from(n: CssComplexSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssCompoundSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_COMPOUND_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_COMPOUND_SELECTOR
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
impl std::fmt::Debug for CssCompoundSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssCompoundSelector")
            .field(
                "nesting_selector_token",
                &support::DebugOptionalElement(self.nesting_selector_token()),
            )
            .field(
                "simple_selector",
                &support::DebugOptionalElement(self.simple_selector()),
            )
            .field("sub_selectors", &self.sub_selectors())
            .finish()
    }
}
impl From<CssCompoundSelector> for SyntaxNode {
    fn from(n: CssCompoundSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssCompoundSelector> for SyntaxElement {
    fn from(n: CssCompoundSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssCustomProperty {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CUSTOM_PROPERTY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CUSTOM_PROPERTY
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
impl std::fmt::Debug for CssCustomProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssCustomProperty")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<CssCustomProperty> for SyntaxNode {
    fn from(n: CssCustomProperty) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssCustomProperty> for SyntaxElement {
    fn from(n: CssCustomProperty) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssDeclaration {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_DECLARATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_DECLARATION
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
impl std::fmt::Debug for CssDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssDeclaration")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "css_custom_property",
                &support::DebugSyntaxResult(self.css_custom_property()),
            )
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("value", &support::DebugSyntaxResult(self.value()))
            .field(
                "important",
                &support::DebugOptionalElement(self.important()),
            )
            .finish()
    }
}
impl From<CssDeclaration> for SyntaxNode {
    fn from(n: CssDeclaration) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssDeclaration> for SyntaxElement {
    fn from(n: CssDeclaration) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssDeclarationImportant {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_DECLARATION_IMPORTANT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_DECLARATION_IMPORTANT
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
impl std::fmt::Debug for CssDeclarationImportant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssDeclarationImportant")
            .field("excl_token", &support::DebugSyntaxResult(self.excl_token()))
            .field(
                "important_token",
                &support::DebugSyntaxResult(self.important_token()),
            )
            .finish()
    }
}
impl From<CssDeclarationImportant> for SyntaxNode {
    fn from(n: CssDeclarationImportant) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssDeclarationImportant> for SyntaxElement {
    fn from(n: CssDeclarationImportant) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssDimension {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_DIMENSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_DIMENSION
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
impl std::fmt::Debug for CssDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssDimension")
            .field("value", &support::DebugSyntaxResult(self.value()))
            .field("unit", &support::DebugSyntaxResult(self.unit()))
            .finish()
    }
}
impl From<CssDimension> for SyntaxNode {
    fn from(n: CssDimension) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssDimension> for SyntaxElement {
    fn from(n: CssDimension) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssIdSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_ID_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_ID_SELECTOR
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
impl std::fmt::Debug for CssIdSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssIdSelector")
            .field("hash_token", &support::DebugSyntaxResult(self.hash_token()))
            .field("name", &support::DebugSyntaxResult(self.name()))
            .finish()
    }
}
impl From<CssIdSelector> for SyntaxNode {
    fn from(n: CssIdSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssIdSelector> for SyntaxElement {
    fn from(n: CssIdSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssIdentifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_IDENTIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_IDENTIFIER
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
impl std::fmt::Debug for CssIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssIdentifier")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<CssIdentifier> for SyntaxNode {
    fn from(n: CssIdentifier) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssIdentifier> for SyntaxElement {
    fn from(n: CssIdentifier) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssKeyframesBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_KEYFRAMES_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_KEYFRAMES_BLOCK
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
impl std::fmt::Debug for CssKeyframesBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssKeyframesBlock")
            .field("selectors", &self.selectors())
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("declarations", &self.declarations())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<CssKeyframesBlock> for SyntaxNode {
    fn from(n: CssKeyframesBlock) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssKeyframesBlock> for SyntaxElement {
    fn from(n: CssKeyframesBlock) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssKeyframesSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_KEYFRAMES_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_KEYFRAMES_SELECTOR
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
impl std::fmt::Debug for CssKeyframesSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssKeyframesSelector")
            .field("from_token", &support::DebugSyntaxResult(self.from_token()))
            .field("to_token", &support::DebugSyntaxResult(self.to_token()))
            .field(
                "css_percentage",
                &support::DebugSyntaxResult(self.css_percentage()),
            )
            .finish()
    }
}
impl From<CssKeyframesSelector> for SyntaxNode {
    fn from(n: CssKeyframesSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssKeyframesSelector> for SyntaxElement {
    fn from(n: CssKeyframesSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssNthMultiplier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_NTH_MULTIPLIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_NTH_MULTIPLIER
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
impl std::fmt::Debug for CssNthMultiplier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssNthMultiplier")
            .field("sign", &support::DebugSyntaxResult(self.sign()))
            .field("value", &support::DebugOptionalElement(self.value()))
            .finish()
    }
}
impl From<CssNthMultiplier> for SyntaxNode {
    fn from(n: CssNthMultiplier) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssNthMultiplier> for SyntaxElement {
    fn from(n: CssNthMultiplier) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssNthOffset {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_NTH_OFFSET as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_NTH_OFFSET
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
impl std::fmt::Debug for CssNthOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssNthOffset")
            .field("sign", &support::DebugSyntaxResult(self.sign()))
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<CssNthOffset> for SyntaxNode {
    fn from(n: CssNthOffset) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssNthOffset> for SyntaxElement {
    fn from(n: CssNthOffset) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssNumber {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_NUMBER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_NUMBER
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
impl std::fmt::Debug for CssNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssNumber")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<CssNumber> for SyntaxNode {
    fn from(n: CssNumber) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssNumber> for SyntaxElement {
    fn from(n: CssNumber) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssParameter {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PARAMETER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PARAMETER
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
impl std::fmt::Debug for CssParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssParameter")
            .field(
                "any_css_value",
                &support::DebugSyntaxResult(self.any_css_value()),
            )
            .finish()
    }
}
impl From<CssParameter> for SyntaxNode {
    fn from(n: CssParameter) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssParameter> for SyntaxElement {
    fn from(n: CssParameter) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPercentage {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PERCENTAGE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PERCENTAGE
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
impl std::fmt::Debug for CssPercentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPercentage")
            .field("value", &support::DebugSyntaxResult(self.value()))
            .field(
                "reminder_token",
                &support::DebugSyntaxResult(self.reminder_token()),
            )
            .finish()
    }
}
impl From<CssPercentage> for SyntaxNode {
    fn from(n: CssPercentage) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPercentage> for SyntaxElement {
    fn from(n: CssPercentage) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassFunctionCompoundSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR as u16,
    ));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR
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
impl std::fmt::Debug for CssPseudoClassFunctionCompoundSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassFunctionCompoundSelector")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("selector", &support::DebugSyntaxResult(self.selector()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssPseudoClassFunctionCompoundSelector> for SyntaxNode {
    fn from(n: CssPseudoClassFunctionCompoundSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassFunctionCompoundSelector> for SyntaxElement {
    fn from(n: CssPseudoClassFunctionCompoundSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassFunctionCompoundSelectorList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST as u16,
    ));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST
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
impl std::fmt::Debug for CssPseudoClassFunctionCompoundSelectorList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassFunctionCompoundSelectorList")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("compound_selector_list", &self.compound_selector_list())
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssPseudoClassFunctionCompoundSelectorList> for SyntaxNode {
    fn from(n: CssPseudoClassFunctionCompoundSelectorList) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassFunctionCompoundSelectorList> for SyntaxElement {
    fn from(n: CssPseudoClassFunctionCompoundSelectorList) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassFunctionIdentifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_CLASS_FUNCTION_IDENTIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_FUNCTION_IDENTIFIER
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
impl std::fmt::Debug for CssPseudoClassFunctionIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassFunctionIdentifier")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("ident", &support::DebugSyntaxResult(self.ident()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssPseudoClassFunctionIdentifier> for SyntaxNode {
    fn from(n: CssPseudoClassFunctionIdentifier) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassFunctionIdentifier> for SyntaxElement {
    fn from(n: CssPseudoClassFunctionIdentifier) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassFunctionNth {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_CLASS_FUNCTION_NTH as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_FUNCTION_NTH
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
impl std::fmt::Debug for CssPseudoClassFunctionNth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassFunctionNth")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("selector", &support::DebugSyntaxResult(self.selector()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssPseudoClassFunctionNth> for SyntaxNode {
    fn from(n: CssPseudoClassFunctionNth) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassFunctionNth> for SyntaxElement {
    fn from(n: CssPseudoClassFunctionNth) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassFunctionRelativeSelectorList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        CSS_PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST as u16,
    ));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST
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
impl std::fmt::Debug for CssPseudoClassFunctionRelativeSelectorList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassFunctionRelativeSelectorList")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("relative_selector_list", &self.relative_selector_list())
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssPseudoClassFunctionRelativeSelectorList> for SyntaxNode {
    fn from(n: CssPseudoClassFunctionRelativeSelectorList) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassFunctionRelativeSelectorList> for SyntaxElement {
    fn from(n: CssPseudoClassFunctionRelativeSelectorList) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassFunctionSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_CLASS_FUNCTION_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_FUNCTION_SELECTOR
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
impl std::fmt::Debug for CssPseudoClassFunctionSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassFunctionSelector")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("selector", &support::DebugSyntaxResult(self.selector()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssPseudoClassFunctionSelector> for SyntaxNode {
    fn from(n: CssPseudoClassFunctionSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassFunctionSelector> for SyntaxElement {
    fn from(n: CssPseudoClassFunctionSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassFunctionSelectorList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        CSS_PSEUDO_CLASS_FUNCTION_SELECTOR_LIST as u16,
    ));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_FUNCTION_SELECTOR_LIST
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
impl std::fmt::Debug for CssPseudoClassFunctionSelectorList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassFunctionSelectorList")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("selector_list", &self.selector_list())
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssPseudoClassFunctionSelectorList> for SyntaxNode {
    fn from(n: CssPseudoClassFunctionSelectorList) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassFunctionSelectorList> for SyntaxElement {
    fn from(n: CssPseudoClassFunctionSelectorList) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassFunctionValueList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_CLASS_FUNCTION_VALUE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_FUNCTION_VALUE_LIST
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
impl std::fmt::Debug for CssPseudoClassFunctionValueList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassFunctionValueList")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("value_list", &self.value_list())
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssPseudoClassFunctionValueList> for SyntaxNode {
    fn from(n: CssPseudoClassFunctionValueList) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassFunctionValueList> for SyntaxElement {
    fn from(n: CssPseudoClassFunctionValueList) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassIdentifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_CLASS_IDENTIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_IDENTIFIER
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
impl std::fmt::Debug for CssPseudoClassIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassIdentifier")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .finish()
    }
}
impl From<CssPseudoClassIdentifier> for SyntaxNode {
    fn from(n: CssPseudoClassIdentifier) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassIdentifier> for SyntaxElement {
    fn from(n: CssPseudoClassIdentifier) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassNth {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_CLASS_NTH as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_NTH
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
impl std::fmt::Debug for CssPseudoClassNth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassNth")
            .field("sign", &support::DebugOptionalElement(self.sign()))
            .field("value", &support::DebugOptionalElement(self.value()))
            .field(
                "symbol_token",
                &support::DebugSyntaxResult(self.symbol_token()),
            )
            .field("offset", &support::DebugOptionalElement(self.offset()))
            .finish()
    }
}
impl From<CssPseudoClassNth> for SyntaxNode {
    fn from(n: CssPseudoClassNth) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassNth> for SyntaxElement {
    fn from(n: CssPseudoClassNth) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassNthIdentifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_CLASS_NTH_IDENTIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_NTH_IDENTIFIER
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
impl std::fmt::Debug for CssPseudoClassNthIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassNthIdentifier")
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<CssPseudoClassNthIdentifier> for SyntaxNode {
    fn from(n: CssPseudoClassNthIdentifier) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassNthIdentifier> for SyntaxElement {
    fn from(n: CssPseudoClassNthIdentifier) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassNthNumber {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_CLASS_NTH_NUMBER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_NTH_NUMBER
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
impl std::fmt::Debug for CssPseudoClassNthNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassNthNumber")
            .field("sign", &support::DebugOptionalElement(self.sign()))
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<CssPseudoClassNthNumber> for SyntaxNode {
    fn from(n: CssPseudoClassNthNumber) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassNthNumber> for SyntaxElement {
    fn from(n: CssPseudoClassNthNumber) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassNthSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_CLASS_NTH_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_NTH_SELECTOR
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
impl std::fmt::Debug for CssPseudoClassNthSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassNthSelector")
            .field("nth", &support::DebugSyntaxResult(self.nth()))
            .field(
                "of_selector",
                &support::DebugOptionalElement(self.of_selector()),
            )
            .finish()
    }
}
impl From<CssPseudoClassNthSelector> for SyntaxNode {
    fn from(n: CssPseudoClassNthSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassNthSelector> for SyntaxElement {
    fn from(n: CssPseudoClassNthSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassOfNthSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_CLASS_OF_NTH_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_OF_NTH_SELECTOR
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
impl std::fmt::Debug for CssPseudoClassOfNthSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassOfNthSelector")
            .field("of_token", &support::DebugSyntaxResult(self.of_token()))
            .field("selector_list", &self.selector_list())
            .finish()
    }
}
impl From<CssPseudoClassOfNthSelector> for SyntaxNode {
    fn from(n: CssPseudoClassOfNthSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassOfNthSelector> for SyntaxElement {
    fn from(n: CssPseudoClassOfNthSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_CLASS_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_SELECTOR
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
impl std::fmt::Debug for CssPseudoClassSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassSelector")
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("class", &support::DebugSyntaxResult(self.class()))
            .finish()
    }
}
impl From<CssPseudoClassSelector> for SyntaxNode {
    fn from(n: CssPseudoClassSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassSelector> for SyntaxElement {
    fn from(n: CssPseudoClassSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoElementFunctionIdentifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_ELEMENT_FUNCTION_IDENTIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_ELEMENT_FUNCTION_IDENTIFIER
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
impl std::fmt::Debug for CssPseudoElementFunctionIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoElementFunctionIdentifier")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("ident", &support::DebugSyntaxResult(self.ident()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssPseudoElementFunctionIdentifier> for SyntaxNode {
    fn from(n: CssPseudoElementFunctionIdentifier) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoElementFunctionIdentifier> for SyntaxElement {
    fn from(n: CssPseudoElementFunctionIdentifier) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoElementFunctionSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_ELEMENT_FUNCTION_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_ELEMENT_FUNCTION_SELECTOR
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
impl std::fmt::Debug for CssPseudoElementFunctionSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoElementFunctionSelector")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("selector", &support::DebugSyntaxResult(self.selector()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssPseudoElementFunctionSelector> for SyntaxNode {
    fn from(n: CssPseudoElementFunctionSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoElementFunctionSelector> for SyntaxElement {
    fn from(n: CssPseudoElementFunctionSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoElementIdentifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_ELEMENT_IDENTIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_ELEMENT_IDENTIFIER
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
impl std::fmt::Debug for CssPseudoElementIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoElementIdentifier")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .finish()
    }
}
impl From<CssPseudoElementIdentifier> for SyntaxNode {
    fn from(n: CssPseudoElementIdentifier) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoElementIdentifier> for SyntaxElement {
    fn from(n: CssPseudoElementIdentifier) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoElementSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_ELEMENT_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_ELEMENT_SELECTOR
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
impl std::fmt::Debug for CssPseudoElementSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoElementSelector")
            .field(
                "double_colon_token",
                &support::DebugSyntaxResult(self.double_colon_token()),
            )
            .field("element", &support::DebugSyntaxResult(self.element()))
            .finish()
    }
}
impl From<CssPseudoElementSelector> for SyntaxNode {
    fn from(n: CssPseudoElementSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoElementSelector> for SyntaxElement {
    fn from(n: CssPseudoElementSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssRatio {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_RATIO as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_RATIO
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
impl std::fmt::Debug for CssRatio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssRatio")
            .field("numerator", &support::DebugSyntaxResult(self.numerator()))
            .field(
                "denominator",
                &support::DebugSyntaxResult(self.denominator()),
            )
            .finish()
    }
}
impl From<CssRatio> for SyntaxNode {
    fn from(n: CssRatio) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssRatio> for SyntaxElement {
    fn from(n: CssRatio) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssRelativeSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_RELATIVE_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_RELATIVE_SELECTOR
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
impl std::fmt::Debug for CssRelativeSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssRelativeSelector")
            .field(
                "combinator",
                &support::DebugOptionalElement(self.combinator()),
            )
            .field("selector", &support::DebugSyntaxResult(self.selector()))
            .finish()
    }
}
impl From<CssRelativeSelector> for SyntaxNode {
    fn from(n: CssRelativeSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssRelativeSelector> for SyntaxElement {
    fn from(n: CssRelativeSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssRoot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_ROOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_ROOT
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
impl std::fmt::Debug for CssRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssRoot")
            .field(
                "bom_token",
                &support::DebugOptionalElement(self.bom_token()),
            )
            .field("rules", &self.rules())
            .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
            .finish()
    }
}
impl From<CssRoot> for SyntaxNode {
    fn from(n: CssRoot) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssRoot> for SyntaxElement {
    fn from(n: CssRoot) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_RULE
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
impl std::fmt::Debug for CssRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssRule")
            .field("prelude", &self.prelude())
            .field("block", &support::DebugSyntaxResult(self.block()))
            .finish()
    }
}
impl From<CssRule> for SyntaxNode {
    fn from(n: CssRule) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssRule> for SyntaxElement {
    fn from(n: CssRule) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssSimpleFunction {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SIMPLE_FUNCTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SIMPLE_FUNCTION
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
impl std::fmt::Debug for CssSimpleFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssSimpleFunction")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("items", &self.items())
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssSimpleFunction> for SyntaxNode {
    fn from(n: CssSimpleFunction) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssSimpleFunction> for SyntaxElement {
    fn from(n: CssSimpleFunction) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssString {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_STRING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_STRING
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
impl std::fmt::Debug for CssString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssString")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<CssString> for SyntaxNode {
    fn from(n: CssString) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssString> for SyntaxElement {
    fn from(n: CssString) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssTypeSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_TYPE_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_TYPE_SELECTOR
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
impl std::fmt::Debug for CssTypeSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssTypeSelector")
            .field("ident", &support::DebugSyntaxResult(self.ident()))
            .finish()
    }
}
impl From<CssTypeSelector> for SyntaxNode {
    fn from(n: CssTypeSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssTypeSelector> for SyntaxElement {
    fn from(n: CssTypeSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssUniversalSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_UNIVERSAL_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_UNIVERSAL_SELECTOR
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
impl std::fmt::Debug for CssUniversalSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssUniversalSelector")
            .field("star_token", &support::DebugSyntaxResult(self.star_token()))
            .finish()
    }
}
impl From<CssUniversalSelector> for SyntaxNode {
    fn from(n: CssUniversalSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssUniversalSelector> for SyntaxElement {
    fn from(n: CssUniversalSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssVarFunction {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_VAR_FUNCTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_VAR_FUNCTION
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
impl std::fmt::Debug for CssVarFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssVarFunction")
            .field("var_token", &support::DebugSyntaxResult(self.var_token()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("property", &support::DebugSyntaxResult(self.property()))
            .field("value", &support::DebugOptionalElement(self.value()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssVarFunction> for SyntaxNode {
    fn from(n: CssVarFunction) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssVarFunction> for SyntaxElement {
    fn from(n: CssVarFunction) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssVarFunctionValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_VAR_FUNCTION_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_VAR_FUNCTION_VALUE
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
impl std::fmt::Debug for CssVarFunctionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssVarFunctionValue")
            .field(
                "comma_token",
                &support::DebugSyntaxResult(self.comma_token()),
            )
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<CssVarFunctionValue> for SyntaxNode {
    fn from(n: CssVarFunctionValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssVarFunctionValue> for SyntaxElement {
    fn from(n: CssVarFunctionValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl From<CssAtMediaQueryFeatureBoolean> for AnyCssAtMediaQueryFeatureType {
    fn from(node: CssAtMediaQueryFeatureBoolean) -> AnyCssAtMediaQueryFeatureType {
        AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeatureBoolean(node)
    }
}
impl From<CssAtMediaQueryFeatureCompare> for AnyCssAtMediaQueryFeatureType {
    fn from(node: CssAtMediaQueryFeatureCompare) -> AnyCssAtMediaQueryFeatureType {
        AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeatureCompare(node)
    }
}
impl From<CssAtMediaQueryFeaturePlain> for AnyCssAtMediaQueryFeatureType {
    fn from(node: CssAtMediaQueryFeaturePlain) -> AnyCssAtMediaQueryFeatureType {
        AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeaturePlain(node)
    }
}
impl From<CssAtMediaQueryFeatureRange> for AnyCssAtMediaQueryFeatureType {
    fn from(node: CssAtMediaQueryFeatureRange) -> AnyCssAtMediaQueryFeatureType {
        AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeatureRange(node)
    }
}
impl AstNode for AnyCssAtMediaQueryFeatureType {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssAtMediaQueryFeatureBoolean::KIND_SET
        .union(CssAtMediaQueryFeatureCompare::KIND_SET)
        .union(CssAtMediaQueryFeaturePlain::KIND_SET)
        .union(CssAtMediaQueryFeatureRange::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_AT_MEDIA_QUERY_FEATURE_BOOLEAN
                | CSS_AT_MEDIA_QUERY_FEATURE_COMPARE
                | CSS_AT_MEDIA_QUERY_FEATURE_PLAIN
                | CSS_AT_MEDIA_QUERY_FEATURE_RANGE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_AT_MEDIA_QUERY_FEATURE_BOOLEAN => {
                AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeatureBoolean(
                    CssAtMediaQueryFeatureBoolean { syntax },
                )
            }
            CSS_AT_MEDIA_QUERY_FEATURE_COMPARE => {
                AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeatureCompare(
                    CssAtMediaQueryFeatureCompare { syntax },
                )
            }
            CSS_AT_MEDIA_QUERY_FEATURE_PLAIN => {
                AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeaturePlain(
                    CssAtMediaQueryFeaturePlain { syntax },
                )
            }
            CSS_AT_MEDIA_QUERY_FEATURE_RANGE => {
                AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeatureRange(
                    CssAtMediaQueryFeatureRange { syntax },
                )
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeatureBoolean(it) => &it.syntax,
            AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeatureCompare(it) => &it.syntax,
            AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeaturePlain(it) => &it.syntax,
            AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeatureRange(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeatureBoolean(it) => it.syntax,
            AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeatureCompare(it) => it.syntax,
            AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeaturePlain(it) => it.syntax,
            AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeatureRange(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssAtMediaQueryFeatureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeatureBoolean(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeatureCompare(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeaturePlain(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeatureRange(it) => {
                std::fmt::Debug::fmt(it, f)
            }
        }
    }
}
impl From<AnyCssAtMediaQueryFeatureType> for SyntaxNode {
    fn from(n: AnyCssAtMediaQueryFeatureType) -> SyntaxNode {
        match n {
            AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeatureBoolean(it) => it.into(),
            AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeatureCompare(it) => it.into(),
            AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeaturePlain(it) => it.into(),
            AnyCssAtMediaQueryFeatureType::CssAtMediaQueryFeatureRange(it) => it.into(),
        }
    }
}
impl From<AnyCssAtMediaQueryFeatureType> for SyntaxElement {
    fn from(n: AnyCssAtMediaQueryFeatureType) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssAtMediaQueryFeature> for AnyCssAtMediaQueryType {
    fn from(node: CssAtMediaQueryFeature) -> AnyCssAtMediaQueryType {
        AnyCssAtMediaQueryType::CssAtMediaQueryFeature(node)
    }
}
impl From<CssIdentifier> for AnyCssAtMediaQueryType {
    fn from(node: CssIdentifier) -> AnyCssAtMediaQueryType {
        AnyCssAtMediaQueryType::CssIdentifier(node)
    }
}
impl AstNode for AnyCssAtMediaQueryType {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssAtMediaQueryFeature::KIND_SET.union(CssIdentifier::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_AT_MEDIA_QUERY_FEATURE | CSS_IDENTIFIER)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_AT_MEDIA_QUERY_FEATURE => {
                AnyCssAtMediaQueryType::CssAtMediaQueryFeature(CssAtMediaQueryFeature { syntax })
            }
            CSS_IDENTIFIER => AnyCssAtMediaQueryType::CssIdentifier(CssIdentifier { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssAtMediaQueryType::CssAtMediaQueryFeature(it) => &it.syntax,
            AnyCssAtMediaQueryType::CssIdentifier(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssAtMediaQueryType::CssAtMediaQueryFeature(it) => it.syntax,
            AnyCssAtMediaQueryType::CssIdentifier(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssAtMediaQueryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssAtMediaQueryType::CssAtMediaQueryFeature(it) => std::fmt::Debug::fmt(it, f),
            AnyCssAtMediaQueryType::CssIdentifier(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssAtMediaQueryType> for SyntaxNode {
    fn from(n: AnyCssAtMediaQueryType) -> SyntaxNode {
        match n {
            AnyCssAtMediaQueryType::CssAtMediaQueryFeature(it) => it.into(),
            AnyCssAtMediaQueryType::CssIdentifier(it) => it.into(),
        }
    }
}
impl From<AnyCssAtMediaQueryType> for SyntaxElement {
    fn from(n: AnyCssAtMediaQueryType) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssAtKeyframes> for AnyCssAtRule {
    fn from(node: CssAtKeyframes) -> AnyCssAtRule {
        AnyCssAtRule::CssAtKeyframes(node)
    }
}
impl From<CssAtMedia> for AnyCssAtRule {
    fn from(node: CssAtMedia) -> AnyCssAtRule {
        AnyCssAtRule::CssAtMedia(node)
    }
}
impl AstNode for AnyCssAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssAtKeyframes::KIND_SET.union(CssAtMedia::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_AT_KEYFRAMES | CSS_AT_MEDIA)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_AT_KEYFRAMES => AnyCssAtRule::CssAtKeyframes(CssAtKeyframes { syntax }),
            CSS_AT_MEDIA => AnyCssAtRule::CssAtMedia(CssAtMedia { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssAtRule::CssAtKeyframes(it) => &it.syntax,
            AnyCssAtRule::CssAtMedia(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssAtRule::CssAtKeyframes(it) => it.syntax,
            AnyCssAtRule::CssAtMedia(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssAtRule::CssAtKeyframes(it) => std::fmt::Debug::fmt(it, f),
            AnyCssAtRule::CssAtMedia(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssAtRule> for SyntaxNode {
    fn from(n: AnyCssAtRule) -> SyntaxNode {
        match n {
            AnyCssAtRule::CssAtKeyframes(it) => it.into(),
            AnyCssAtRule::CssAtMedia(it) => it.into(),
        }
    }
}
impl From<AnyCssAtRule> for SyntaxElement {
    fn from(n: AnyCssAtRule) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssIdentifier> for AnyCssAttributeMatcherValue {
    fn from(node: CssIdentifier) -> AnyCssAttributeMatcherValue {
        AnyCssAttributeMatcherValue::CssIdentifier(node)
    }
}
impl From<CssString> for AnyCssAttributeMatcherValue {
    fn from(node: CssString) -> AnyCssAttributeMatcherValue {
        AnyCssAttributeMatcherValue::CssString(node)
    }
}
impl AstNode for AnyCssAttributeMatcherValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssIdentifier::KIND_SET.union(CssString::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_IDENTIFIER | CSS_STRING)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_IDENTIFIER => AnyCssAttributeMatcherValue::CssIdentifier(CssIdentifier { syntax }),
            CSS_STRING => AnyCssAttributeMatcherValue::CssString(CssString { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssAttributeMatcherValue::CssIdentifier(it) => &it.syntax,
            AnyCssAttributeMatcherValue::CssString(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssAttributeMatcherValue::CssIdentifier(it) => it.syntax,
            AnyCssAttributeMatcherValue::CssString(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssAttributeMatcherValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssAttributeMatcherValue::CssIdentifier(it) => std::fmt::Debug::fmt(it, f),
            AnyCssAttributeMatcherValue::CssString(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssAttributeMatcherValue> for SyntaxNode {
    fn from(n: AnyCssAttributeMatcherValue) -> SyntaxNode {
        match n {
            AnyCssAttributeMatcherValue::CssIdentifier(it) => it.into(),
            AnyCssAttributeMatcherValue::CssString(it) => it.into(),
        }
    }
}
impl From<AnyCssAttributeMatcherValue> for SyntaxElement {
    fn from(n: AnyCssAttributeMatcherValue) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusSelector> for AnyCssCompoundSelector {
    fn from(node: CssBogusSelector) -> AnyCssCompoundSelector {
        AnyCssCompoundSelector::CssBogusSelector(node)
    }
}
impl From<CssCompoundSelector> for AnyCssCompoundSelector {
    fn from(node: CssCompoundSelector) -> AnyCssCompoundSelector {
        AnyCssCompoundSelector::CssCompoundSelector(node)
    }
}
impl AstNode for AnyCssCompoundSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssBogusSelector::KIND_SET.union(CssCompoundSelector::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_BOGUS_SELECTOR | CSS_COMPOUND_SELECTOR)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_SELECTOR => {
                AnyCssCompoundSelector::CssBogusSelector(CssBogusSelector { syntax })
            }
            CSS_COMPOUND_SELECTOR => {
                AnyCssCompoundSelector::CssCompoundSelector(CssCompoundSelector { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssCompoundSelector::CssBogusSelector(it) => &it.syntax,
            AnyCssCompoundSelector::CssCompoundSelector(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssCompoundSelector::CssBogusSelector(it) => it.syntax,
            AnyCssCompoundSelector::CssCompoundSelector(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssCompoundSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssCompoundSelector::CssBogusSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssCompoundSelector::CssCompoundSelector(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssCompoundSelector> for SyntaxNode {
    fn from(n: AnyCssCompoundSelector) -> SyntaxNode {
        match n {
            AnyCssCompoundSelector::CssBogusSelector(it) => it.into(),
            AnyCssCompoundSelector::CssCompoundSelector(it) => it.into(),
        }
    }
}
impl From<AnyCssCompoundSelector> for SyntaxElement {
    fn from(n: AnyCssCompoundSelector) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssNthMultiplier> for AnyCssNthMultiplier {
    fn from(node: CssNthMultiplier) -> AnyCssNthMultiplier {
        AnyCssNthMultiplier::CssNthMultiplier(node)
    }
}
impl From<CssNumber> for AnyCssNthMultiplier {
    fn from(node: CssNumber) -> AnyCssNthMultiplier {
        AnyCssNthMultiplier::CssNumber(node)
    }
}
impl AstNode for AnyCssNthMultiplier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssNthMultiplier::KIND_SET.union(CssNumber::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_NTH_MULTIPLIER | CSS_NUMBER)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_NTH_MULTIPLIER => {
                AnyCssNthMultiplier::CssNthMultiplier(CssNthMultiplier { syntax })
            }
            CSS_NUMBER => AnyCssNthMultiplier::CssNumber(CssNumber { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssNthMultiplier::CssNthMultiplier(it) => &it.syntax,
            AnyCssNthMultiplier::CssNumber(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssNthMultiplier::CssNthMultiplier(it) => it.syntax,
            AnyCssNthMultiplier::CssNumber(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssNthMultiplier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssNthMultiplier::CssNthMultiplier(it) => std::fmt::Debug::fmt(it, f),
            AnyCssNthMultiplier::CssNumber(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssNthMultiplier> for SyntaxNode {
    fn from(n: AnyCssNthMultiplier) -> SyntaxNode {
        match n {
            AnyCssNthMultiplier::CssNthMultiplier(it) => it.into(),
            AnyCssNthMultiplier::CssNumber(it) => it.into(),
        }
    }
}
impl From<AnyCssNthMultiplier> for SyntaxElement {
    fn from(n: AnyCssNthMultiplier) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssPseudoClassFunctionCompoundSelector> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassFunctionCompoundSelector) -> AnyCssPseudoClass {
        AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelector(node)
    }
}
impl From<CssPseudoClassFunctionCompoundSelectorList> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassFunctionCompoundSelectorList) -> AnyCssPseudoClass {
        AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelectorList(node)
    }
}
impl From<CssPseudoClassFunctionIdentifier> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassFunctionIdentifier) -> AnyCssPseudoClass {
        AnyCssPseudoClass::CssPseudoClassFunctionIdentifier(node)
    }
}
impl From<CssPseudoClassFunctionNth> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassFunctionNth) -> AnyCssPseudoClass {
        AnyCssPseudoClass::CssPseudoClassFunctionNth(node)
    }
}
impl From<CssPseudoClassFunctionRelativeSelectorList> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassFunctionRelativeSelectorList) -> AnyCssPseudoClass {
        AnyCssPseudoClass::CssPseudoClassFunctionRelativeSelectorList(node)
    }
}
impl From<CssPseudoClassFunctionSelector> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassFunctionSelector) -> AnyCssPseudoClass {
        AnyCssPseudoClass::CssPseudoClassFunctionSelector(node)
    }
}
impl From<CssPseudoClassFunctionSelectorList> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassFunctionSelectorList) -> AnyCssPseudoClass {
        AnyCssPseudoClass::CssPseudoClassFunctionSelectorList(node)
    }
}
impl From<CssPseudoClassFunctionValueList> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassFunctionValueList) -> AnyCssPseudoClass {
        AnyCssPseudoClass::CssPseudoClassFunctionValueList(node)
    }
}
impl From<CssPseudoClassIdentifier> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassIdentifier) -> AnyCssPseudoClass {
        AnyCssPseudoClass::CssPseudoClassIdentifier(node)
    }
}
impl AstNode for AnyCssPseudoClass {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssPseudoClassFunctionCompoundSelector::KIND_SET
        .union(CssPseudoClassFunctionCompoundSelectorList::KIND_SET)
        .union(CssPseudoClassFunctionIdentifier::KIND_SET)
        .union(CssPseudoClassFunctionNth::KIND_SET)
        .union(CssPseudoClassFunctionRelativeSelectorList::KIND_SET)
        .union(CssPseudoClassFunctionSelector::KIND_SET)
        .union(CssPseudoClassFunctionSelectorList::KIND_SET)
        .union(CssPseudoClassFunctionValueList::KIND_SET)
        .union(CssPseudoClassIdentifier::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR
                | CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST
                | CSS_PSEUDO_CLASS_FUNCTION_IDENTIFIER
                | CSS_PSEUDO_CLASS_FUNCTION_NTH
                | CSS_PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST
                | CSS_PSEUDO_CLASS_FUNCTION_SELECTOR
                | CSS_PSEUDO_CLASS_FUNCTION_SELECTOR_LIST
                | CSS_PSEUDO_CLASS_FUNCTION_VALUE_LIST
                | CSS_PSEUDO_CLASS_IDENTIFIER
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR => {
                AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelector(
                    CssPseudoClassFunctionCompoundSelector { syntax },
                )
            }
            CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST => {
                AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelectorList(
                    CssPseudoClassFunctionCompoundSelectorList { syntax },
                )
            }
            CSS_PSEUDO_CLASS_FUNCTION_IDENTIFIER => {
                AnyCssPseudoClass::CssPseudoClassFunctionIdentifier(
                    CssPseudoClassFunctionIdentifier { syntax },
                )
            }
            CSS_PSEUDO_CLASS_FUNCTION_NTH => {
                AnyCssPseudoClass::CssPseudoClassFunctionNth(CssPseudoClassFunctionNth { syntax })
            }
            CSS_PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST => {
                AnyCssPseudoClass::CssPseudoClassFunctionRelativeSelectorList(
                    CssPseudoClassFunctionRelativeSelectorList { syntax },
                )
            }
            CSS_PSEUDO_CLASS_FUNCTION_SELECTOR => {
                AnyCssPseudoClass::CssPseudoClassFunctionSelector(CssPseudoClassFunctionSelector {
                    syntax,
                })
            }
            CSS_PSEUDO_CLASS_FUNCTION_SELECTOR_LIST => {
                AnyCssPseudoClass::CssPseudoClassFunctionSelectorList(
                    CssPseudoClassFunctionSelectorList { syntax },
                )
            }
            CSS_PSEUDO_CLASS_FUNCTION_VALUE_LIST => {
                AnyCssPseudoClass::CssPseudoClassFunctionValueList(
                    CssPseudoClassFunctionValueList { syntax },
                )
            }
            CSS_PSEUDO_CLASS_IDENTIFIER => {
                AnyCssPseudoClass::CssPseudoClassIdentifier(CssPseudoClassIdentifier { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelector(it) => &it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelectorList(it) => &it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionIdentifier(it) => &it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionNth(it) => &it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionRelativeSelectorList(it) => &it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionSelector(it) => &it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionSelectorList(it) => &it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionValueList(it) => &it.syntax,
            AnyCssPseudoClass::CssPseudoClassIdentifier(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelector(it) => it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelectorList(it) => it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionIdentifier(it) => it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionNth(it) => it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionRelativeSelectorList(it) => it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionSelector(it) => it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionSelectorList(it) => it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionValueList(it) => it.syntax,
            AnyCssPseudoClass::CssPseudoClassIdentifier(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssPseudoClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelector(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelectorList(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssPseudoClass::CssPseudoClassFunctionIdentifier(it) => std::fmt::Debug::fmt(it, f),
            AnyCssPseudoClass::CssPseudoClassFunctionNth(it) => std::fmt::Debug::fmt(it, f),
            AnyCssPseudoClass::CssPseudoClassFunctionRelativeSelectorList(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssPseudoClass::CssPseudoClassFunctionSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssPseudoClass::CssPseudoClassFunctionSelectorList(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssPseudoClass::CssPseudoClassFunctionValueList(it) => std::fmt::Debug::fmt(it, f),
            AnyCssPseudoClass::CssPseudoClassIdentifier(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssPseudoClass> for SyntaxNode {
    fn from(n: AnyCssPseudoClass) -> SyntaxNode {
        match n {
            AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelector(it) => it.into(),
            AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelectorList(it) => it.into(),
            AnyCssPseudoClass::CssPseudoClassFunctionIdentifier(it) => it.into(),
            AnyCssPseudoClass::CssPseudoClassFunctionNth(it) => it.into(),
            AnyCssPseudoClass::CssPseudoClassFunctionRelativeSelectorList(it) => it.into(),
            AnyCssPseudoClass::CssPseudoClassFunctionSelector(it) => it.into(),
            AnyCssPseudoClass::CssPseudoClassFunctionSelectorList(it) => it.into(),
            AnyCssPseudoClass::CssPseudoClassFunctionValueList(it) => it.into(),
            AnyCssPseudoClass::CssPseudoClassIdentifier(it) => it.into(),
        }
    }
}
impl From<AnyCssPseudoClass> for SyntaxElement {
    fn from(n: AnyCssPseudoClass) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssPseudoClassNth> for AnyCssPseudoClassNth {
    fn from(node: CssPseudoClassNth) -> AnyCssPseudoClassNth {
        AnyCssPseudoClassNth::CssPseudoClassNth(node)
    }
}
impl From<CssPseudoClassNthIdentifier> for AnyCssPseudoClassNth {
    fn from(node: CssPseudoClassNthIdentifier) -> AnyCssPseudoClassNth {
        AnyCssPseudoClassNth::CssPseudoClassNthIdentifier(node)
    }
}
impl From<CssPseudoClassNthNumber> for AnyCssPseudoClassNth {
    fn from(node: CssPseudoClassNthNumber) -> AnyCssPseudoClassNth {
        AnyCssPseudoClassNth::CssPseudoClassNthNumber(node)
    }
}
impl AstNode for AnyCssPseudoClassNth {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssPseudoClassNth::KIND_SET
        .union(CssPseudoClassNthIdentifier::KIND_SET)
        .union(CssPseudoClassNthNumber::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_PSEUDO_CLASS_NTH | CSS_PSEUDO_CLASS_NTH_IDENTIFIER | CSS_PSEUDO_CLASS_NTH_NUMBER
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_PSEUDO_CLASS_NTH => {
                AnyCssPseudoClassNth::CssPseudoClassNth(CssPseudoClassNth { syntax })
            }
            CSS_PSEUDO_CLASS_NTH_IDENTIFIER => {
                AnyCssPseudoClassNth::CssPseudoClassNthIdentifier(CssPseudoClassNthIdentifier {
                    syntax,
                })
            }
            CSS_PSEUDO_CLASS_NTH_NUMBER => {
                AnyCssPseudoClassNth::CssPseudoClassNthNumber(CssPseudoClassNthNumber { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssPseudoClassNth::CssPseudoClassNth(it) => &it.syntax,
            AnyCssPseudoClassNth::CssPseudoClassNthIdentifier(it) => &it.syntax,
            AnyCssPseudoClassNth::CssPseudoClassNthNumber(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssPseudoClassNth::CssPseudoClassNth(it) => it.syntax,
            AnyCssPseudoClassNth::CssPseudoClassNthIdentifier(it) => it.syntax,
            AnyCssPseudoClassNth::CssPseudoClassNthNumber(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssPseudoClassNth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssPseudoClassNth::CssPseudoClassNth(it) => std::fmt::Debug::fmt(it, f),
            AnyCssPseudoClassNth::CssPseudoClassNthIdentifier(it) => std::fmt::Debug::fmt(it, f),
            AnyCssPseudoClassNth::CssPseudoClassNthNumber(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssPseudoClassNth> for SyntaxNode {
    fn from(n: AnyCssPseudoClassNth) -> SyntaxNode {
        match n {
            AnyCssPseudoClassNth::CssPseudoClassNth(it) => it.into(),
            AnyCssPseudoClassNth::CssPseudoClassNthIdentifier(it) => it.into(),
            AnyCssPseudoClassNth::CssPseudoClassNthNumber(it) => it.into(),
        }
    }
}
impl From<AnyCssPseudoClassNth> for SyntaxElement {
    fn from(n: AnyCssPseudoClassNth) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusSelector> for AnyCssPseudoClassNthSelector {
    fn from(node: CssBogusSelector) -> AnyCssPseudoClassNthSelector {
        AnyCssPseudoClassNthSelector::CssBogusSelector(node)
    }
}
impl From<CssPseudoClassNthSelector> for AnyCssPseudoClassNthSelector {
    fn from(node: CssPseudoClassNthSelector) -> AnyCssPseudoClassNthSelector {
        AnyCssPseudoClassNthSelector::CssPseudoClassNthSelector(node)
    }
}
impl AstNode for AnyCssPseudoClassNthSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssBogusSelector::KIND_SET.union(CssPseudoClassNthSelector::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_BOGUS_SELECTOR | CSS_PSEUDO_CLASS_NTH_SELECTOR)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_SELECTOR => {
                AnyCssPseudoClassNthSelector::CssBogusSelector(CssBogusSelector { syntax })
            }
            CSS_PSEUDO_CLASS_NTH_SELECTOR => {
                AnyCssPseudoClassNthSelector::CssPseudoClassNthSelector(CssPseudoClassNthSelector {
                    syntax,
                })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssPseudoClassNthSelector::CssBogusSelector(it) => &it.syntax,
            AnyCssPseudoClassNthSelector::CssPseudoClassNthSelector(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssPseudoClassNthSelector::CssBogusSelector(it) => it.syntax,
            AnyCssPseudoClassNthSelector::CssPseudoClassNthSelector(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssPseudoClassNthSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssPseudoClassNthSelector::CssBogusSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssPseudoClassNthSelector::CssPseudoClassNthSelector(it) => {
                std::fmt::Debug::fmt(it, f)
            }
        }
    }
}
impl From<AnyCssPseudoClassNthSelector> for SyntaxNode {
    fn from(n: AnyCssPseudoClassNthSelector) -> SyntaxNode {
        match n {
            AnyCssPseudoClassNthSelector::CssBogusSelector(it) => it.into(),
            AnyCssPseudoClassNthSelector::CssPseudoClassNthSelector(it) => it.into(),
        }
    }
}
impl From<AnyCssPseudoClassNthSelector> for SyntaxElement {
    fn from(n: AnyCssPseudoClassNthSelector) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssPseudoElementFunctionIdentifier> for AnyCssPseudoElement {
    fn from(node: CssPseudoElementFunctionIdentifier) -> AnyCssPseudoElement {
        AnyCssPseudoElement::CssPseudoElementFunctionIdentifier(node)
    }
}
impl From<CssPseudoElementFunctionSelector> for AnyCssPseudoElement {
    fn from(node: CssPseudoElementFunctionSelector) -> AnyCssPseudoElement {
        AnyCssPseudoElement::CssPseudoElementFunctionSelector(node)
    }
}
impl From<CssPseudoElementIdentifier> for AnyCssPseudoElement {
    fn from(node: CssPseudoElementIdentifier) -> AnyCssPseudoElement {
        AnyCssPseudoElement::CssPseudoElementIdentifier(node)
    }
}
impl AstNode for AnyCssPseudoElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssPseudoElementFunctionIdentifier::KIND_SET
        .union(CssPseudoElementFunctionSelector::KIND_SET)
        .union(CssPseudoElementIdentifier::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_PSEUDO_ELEMENT_FUNCTION_IDENTIFIER
                | CSS_PSEUDO_ELEMENT_FUNCTION_SELECTOR
                | CSS_PSEUDO_ELEMENT_IDENTIFIER
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_PSEUDO_ELEMENT_FUNCTION_IDENTIFIER => {
                AnyCssPseudoElement::CssPseudoElementFunctionIdentifier(
                    CssPseudoElementFunctionIdentifier { syntax },
                )
            }
            CSS_PSEUDO_ELEMENT_FUNCTION_SELECTOR => {
                AnyCssPseudoElement::CssPseudoElementFunctionSelector(
                    CssPseudoElementFunctionSelector { syntax },
                )
            }
            CSS_PSEUDO_ELEMENT_IDENTIFIER => {
                AnyCssPseudoElement::CssPseudoElementIdentifier(CssPseudoElementIdentifier {
                    syntax,
                })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssPseudoElement::CssPseudoElementFunctionIdentifier(it) => &it.syntax,
            AnyCssPseudoElement::CssPseudoElementFunctionSelector(it) => &it.syntax,
            AnyCssPseudoElement::CssPseudoElementIdentifier(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssPseudoElement::CssPseudoElementFunctionIdentifier(it) => it.syntax,
            AnyCssPseudoElement::CssPseudoElementFunctionSelector(it) => it.syntax,
            AnyCssPseudoElement::CssPseudoElementIdentifier(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssPseudoElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssPseudoElement::CssPseudoElementFunctionIdentifier(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssPseudoElement::CssPseudoElementFunctionSelector(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssPseudoElement::CssPseudoElementIdentifier(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssPseudoElement> for SyntaxNode {
    fn from(n: AnyCssPseudoElement) -> SyntaxNode {
        match n {
            AnyCssPseudoElement::CssPseudoElementFunctionIdentifier(it) => it.into(),
            AnyCssPseudoElement::CssPseudoElementFunctionSelector(it) => it.into(),
            AnyCssPseudoElement::CssPseudoElementIdentifier(it) => it.into(),
        }
    }
}
impl From<AnyCssPseudoElement> for SyntaxElement {
    fn from(n: AnyCssPseudoElement) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssIdentifier> for AnyCssPseudoValue {
    fn from(node: CssIdentifier) -> AnyCssPseudoValue {
        AnyCssPseudoValue::CssIdentifier(node)
    }
}
impl From<CssString> for AnyCssPseudoValue {
    fn from(node: CssString) -> AnyCssPseudoValue {
        AnyCssPseudoValue::CssString(node)
    }
}
impl AstNode for AnyCssPseudoValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssIdentifier::KIND_SET.union(CssString::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_IDENTIFIER | CSS_STRING)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_IDENTIFIER => AnyCssPseudoValue::CssIdentifier(CssIdentifier { syntax }),
            CSS_STRING => AnyCssPseudoValue::CssString(CssString { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssPseudoValue::CssIdentifier(it) => &it.syntax,
            AnyCssPseudoValue::CssString(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssPseudoValue::CssIdentifier(it) => it.syntax,
            AnyCssPseudoValue::CssString(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssPseudoValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssPseudoValue::CssIdentifier(it) => std::fmt::Debug::fmt(it, f),
            AnyCssPseudoValue::CssString(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssPseudoValue> for SyntaxNode {
    fn from(n: AnyCssPseudoValue) -> SyntaxNode {
        match n {
            AnyCssPseudoValue::CssIdentifier(it) => it.into(),
            AnyCssPseudoValue::CssString(it) => it.into(),
        }
    }
}
impl From<AnyCssPseudoValue> for SyntaxElement {
    fn from(n: AnyCssPseudoValue) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusSelector> for AnyCssRelativeSelector {
    fn from(node: CssBogusSelector) -> AnyCssRelativeSelector {
        AnyCssRelativeSelector::CssBogusSelector(node)
    }
}
impl From<CssRelativeSelector> for AnyCssRelativeSelector {
    fn from(node: CssRelativeSelector) -> AnyCssRelativeSelector {
        AnyCssRelativeSelector::CssRelativeSelector(node)
    }
}
impl AstNode for AnyCssRelativeSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssBogusSelector::KIND_SET.union(CssRelativeSelector::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_BOGUS_SELECTOR | CSS_RELATIVE_SELECTOR)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_SELECTOR => {
                AnyCssRelativeSelector::CssBogusSelector(CssBogusSelector { syntax })
            }
            CSS_RELATIVE_SELECTOR => {
                AnyCssRelativeSelector::CssRelativeSelector(CssRelativeSelector { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssRelativeSelector::CssBogusSelector(it) => &it.syntax,
            AnyCssRelativeSelector::CssRelativeSelector(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssRelativeSelector::CssBogusSelector(it) => it.syntax,
            AnyCssRelativeSelector::CssRelativeSelector(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssRelativeSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssRelativeSelector::CssBogusSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssRelativeSelector::CssRelativeSelector(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssRelativeSelector> for SyntaxNode {
    fn from(n: AnyCssRelativeSelector) -> SyntaxNode {
        match n {
            AnyCssRelativeSelector::CssBogusSelector(it) => it.into(),
            AnyCssRelativeSelector::CssRelativeSelector(it) => it.into(),
        }
    }
}
impl From<AnyCssRelativeSelector> for SyntaxElement {
    fn from(n: AnyCssRelativeSelector) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusRule> for AnyCssRule {
    fn from(node: CssBogusRule) -> AnyCssRule {
        AnyCssRule::CssBogusRule(node)
    }
}
impl From<CssRule> for AnyCssRule {
    fn from(node: CssRule) -> AnyCssRule {
        AnyCssRule::CssRule(node)
    }
}
impl AstNode for AnyCssRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyCssAtRule::KIND_SET
        .union(CssBogusRule::KIND_SET)
        .union(CssRule::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_BOGUS_RULE | CSS_RULE => true,
            k if AnyCssAtRule::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_RULE => AnyCssRule::CssBogusRule(CssBogusRule { syntax }),
            CSS_RULE => AnyCssRule::CssRule(CssRule { syntax }),
            _ => {
                if let Some(any_css_at_rule) = AnyCssAtRule::cast(syntax) {
                    return Some(AnyCssRule::AnyCssAtRule(any_css_at_rule));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssRule::CssBogusRule(it) => &it.syntax,
            AnyCssRule::CssRule(it) => &it.syntax,
            AnyCssRule::AnyCssAtRule(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssRule::CssBogusRule(it) => it.syntax,
            AnyCssRule::CssRule(it) => it.syntax,
            AnyCssRule::AnyCssAtRule(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssRule::AnyCssAtRule(it) => std::fmt::Debug::fmt(it, f),
            AnyCssRule::CssBogusRule(it) => std::fmt::Debug::fmt(it, f),
            AnyCssRule::CssRule(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssRule> for SyntaxNode {
    fn from(n: AnyCssRule) -> SyntaxNode {
        match n {
            AnyCssRule::AnyCssAtRule(it) => it.into(),
            AnyCssRule::CssBogusRule(it) => it.into(),
            AnyCssRule::CssRule(it) => it.into(),
        }
    }
}
impl From<AnyCssRule> for SyntaxElement {
    fn from(n: AnyCssRule) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusSelector> for AnyCssSelector {
    fn from(node: CssBogusSelector) -> AnyCssSelector {
        AnyCssSelector::CssBogusSelector(node)
    }
}
impl From<CssComplexSelector> for AnyCssSelector {
    fn from(node: CssComplexSelector) -> AnyCssSelector {
        AnyCssSelector::CssComplexSelector(node)
    }
}
impl From<CssCompoundSelector> for AnyCssSelector {
    fn from(node: CssCompoundSelector) -> AnyCssSelector {
        AnyCssSelector::CssCompoundSelector(node)
    }
}
impl AstNode for AnyCssSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssBogusSelector::KIND_SET
        .union(CssComplexSelector::KIND_SET)
        .union(CssCompoundSelector::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_BOGUS_SELECTOR | CSS_COMPLEX_SELECTOR | CSS_COMPOUND_SELECTOR
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_SELECTOR => AnyCssSelector::CssBogusSelector(CssBogusSelector { syntax }),
            CSS_COMPLEX_SELECTOR => {
                AnyCssSelector::CssComplexSelector(CssComplexSelector { syntax })
            }
            CSS_COMPOUND_SELECTOR => {
                AnyCssSelector::CssCompoundSelector(CssCompoundSelector { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssSelector::CssBogusSelector(it) => &it.syntax,
            AnyCssSelector::CssComplexSelector(it) => &it.syntax,
            AnyCssSelector::CssCompoundSelector(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssSelector::CssBogusSelector(it) => it.syntax,
            AnyCssSelector::CssComplexSelector(it) => it.syntax,
            AnyCssSelector::CssCompoundSelector(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssSelector::CssBogusSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssSelector::CssComplexSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssSelector::CssCompoundSelector(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssSelector> for SyntaxNode {
    fn from(n: AnyCssSelector) -> SyntaxNode {
        match n {
            AnyCssSelector::CssBogusSelector(it) => it.into(),
            AnyCssSelector::CssComplexSelector(it) => it.into(),
            AnyCssSelector::CssCompoundSelector(it) => it.into(),
        }
    }
}
impl From<AnyCssSelector> for SyntaxElement {
    fn from(n: AnyCssSelector) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssAttributeSelector> for AnyCssSubSelector {
    fn from(node: CssAttributeSelector) -> AnyCssSubSelector {
        AnyCssSubSelector::CssAttributeSelector(node)
    }
}
impl From<CssBogusSubSelector> for AnyCssSubSelector {
    fn from(node: CssBogusSubSelector) -> AnyCssSubSelector {
        AnyCssSubSelector::CssBogusSubSelector(node)
    }
}
impl From<CssClassSelector> for AnyCssSubSelector {
    fn from(node: CssClassSelector) -> AnyCssSubSelector {
        AnyCssSubSelector::CssClassSelector(node)
    }
}
impl From<CssIdSelector> for AnyCssSubSelector {
    fn from(node: CssIdSelector) -> AnyCssSubSelector {
        AnyCssSubSelector::CssIdSelector(node)
    }
}
impl From<CssPseudoClassSelector> for AnyCssSubSelector {
    fn from(node: CssPseudoClassSelector) -> AnyCssSubSelector {
        AnyCssSubSelector::CssPseudoClassSelector(node)
    }
}
impl From<CssPseudoElementSelector> for AnyCssSubSelector {
    fn from(node: CssPseudoElementSelector) -> AnyCssSubSelector {
        AnyCssSubSelector::CssPseudoElementSelector(node)
    }
}
impl AstNode for AnyCssSubSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssAttributeSelector::KIND_SET
        .union(CssBogusSubSelector::KIND_SET)
        .union(CssClassSelector::KIND_SET)
        .union(CssIdSelector::KIND_SET)
        .union(CssPseudoClassSelector::KIND_SET)
        .union(CssPseudoElementSelector::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_ATTRIBUTE_SELECTOR
                | CSS_BOGUS_SUB_SELECTOR
                | CSS_CLASS_SELECTOR
                | CSS_ID_SELECTOR
                | CSS_PSEUDO_CLASS_SELECTOR
                | CSS_PSEUDO_ELEMENT_SELECTOR
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_ATTRIBUTE_SELECTOR => {
                AnyCssSubSelector::CssAttributeSelector(CssAttributeSelector { syntax })
            }
            CSS_BOGUS_SUB_SELECTOR => {
                AnyCssSubSelector::CssBogusSubSelector(CssBogusSubSelector { syntax })
            }
            CSS_CLASS_SELECTOR => AnyCssSubSelector::CssClassSelector(CssClassSelector { syntax }),
            CSS_ID_SELECTOR => AnyCssSubSelector::CssIdSelector(CssIdSelector { syntax }),
            CSS_PSEUDO_CLASS_SELECTOR => {
                AnyCssSubSelector::CssPseudoClassSelector(CssPseudoClassSelector { syntax })
            }
            CSS_PSEUDO_ELEMENT_SELECTOR => {
                AnyCssSubSelector::CssPseudoElementSelector(CssPseudoElementSelector { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssSubSelector::CssAttributeSelector(it) => &it.syntax,
            AnyCssSubSelector::CssBogusSubSelector(it) => &it.syntax,
            AnyCssSubSelector::CssClassSelector(it) => &it.syntax,
            AnyCssSubSelector::CssIdSelector(it) => &it.syntax,
            AnyCssSubSelector::CssPseudoClassSelector(it) => &it.syntax,
            AnyCssSubSelector::CssPseudoElementSelector(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssSubSelector::CssAttributeSelector(it) => it.syntax,
            AnyCssSubSelector::CssBogusSubSelector(it) => it.syntax,
            AnyCssSubSelector::CssClassSelector(it) => it.syntax,
            AnyCssSubSelector::CssIdSelector(it) => it.syntax,
            AnyCssSubSelector::CssPseudoClassSelector(it) => it.syntax,
            AnyCssSubSelector::CssPseudoElementSelector(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssSubSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssSubSelector::CssAttributeSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssSubSelector::CssBogusSubSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssSubSelector::CssClassSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssSubSelector::CssIdSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssSubSelector::CssPseudoClassSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssSubSelector::CssPseudoElementSelector(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssSubSelector> for SyntaxNode {
    fn from(n: AnyCssSubSelector) -> SyntaxNode {
        match n {
            AnyCssSubSelector::CssAttributeSelector(it) => it.into(),
            AnyCssSubSelector::CssBogusSubSelector(it) => it.into(),
            AnyCssSubSelector::CssClassSelector(it) => it.into(),
            AnyCssSubSelector::CssIdSelector(it) => it.into(),
            AnyCssSubSelector::CssPseudoClassSelector(it) => it.into(),
            AnyCssSubSelector::CssPseudoElementSelector(it) => it.into(),
        }
    }
}
impl From<AnyCssSubSelector> for SyntaxElement {
    fn from(n: AnyCssSubSelector) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssAnyFunction> for AnyCssValue {
    fn from(node: CssAnyFunction) -> AnyCssValue {
        AnyCssValue::CssAnyFunction(node)
    }
}
impl From<CssCustomProperty> for AnyCssValue {
    fn from(node: CssCustomProperty) -> AnyCssValue {
        AnyCssValue::CssCustomProperty(node)
    }
}
impl From<CssDimension> for AnyCssValue {
    fn from(node: CssDimension) -> AnyCssValue {
        AnyCssValue::CssDimension(node)
    }
}
impl From<CssIdentifier> for AnyCssValue {
    fn from(node: CssIdentifier) -> AnyCssValue {
        AnyCssValue::CssIdentifier(node)
    }
}
impl From<CssNumber> for AnyCssValue {
    fn from(node: CssNumber) -> AnyCssValue {
        AnyCssValue::CssNumber(node)
    }
}
impl From<CssRatio> for AnyCssValue {
    fn from(node: CssRatio) -> AnyCssValue {
        AnyCssValue::CssRatio(node)
    }
}
impl From<CssString> for AnyCssValue {
    fn from(node: CssString) -> AnyCssValue {
        AnyCssValue::CssString(node)
    }
}
impl AstNode for AnyCssValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssAnyFunction::KIND_SET
        .union(CssCustomProperty::KIND_SET)
        .union(CssDimension::KIND_SET)
        .union(CssIdentifier::KIND_SET)
        .union(CssNumber::KIND_SET)
        .union(CssRatio::KIND_SET)
        .union(CssString::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_ANY_FUNCTION
                | CSS_CUSTOM_PROPERTY
                | CSS_DIMENSION
                | CSS_IDENTIFIER
                | CSS_NUMBER
                | CSS_RATIO
                | CSS_STRING
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_ANY_FUNCTION => AnyCssValue::CssAnyFunction(CssAnyFunction { syntax }),
            CSS_CUSTOM_PROPERTY => AnyCssValue::CssCustomProperty(CssCustomProperty { syntax }),
            CSS_DIMENSION => AnyCssValue::CssDimension(CssDimension { syntax }),
            CSS_IDENTIFIER => AnyCssValue::CssIdentifier(CssIdentifier { syntax }),
            CSS_NUMBER => AnyCssValue::CssNumber(CssNumber { syntax }),
            CSS_RATIO => AnyCssValue::CssRatio(CssRatio { syntax }),
            CSS_STRING => AnyCssValue::CssString(CssString { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssValue::CssAnyFunction(it) => &it.syntax,
            AnyCssValue::CssCustomProperty(it) => &it.syntax,
            AnyCssValue::CssDimension(it) => &it.syntax,
            AnyCssValue::CssIdentifier(it) => &it.syntax,
            AnyCssValue::CssNumber(it) => &it.syntax,
            AnyCssValue::CssRatio(it) => &it.syntax,
            AnyCssValue::CssString(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssValue::CssAnyFunction(it) => it.syntax,
            AnyCssValue::CssCustomProperty(it) => it.syntax,
            AnyCssValue::CssDimension(it) => it.syntax,
            AnyCssValue::CssIdentifier(it) => it.syntax,
            AnyCssValue::CssNumber(it) => it.syntax,
            AnyCssValue::CssRatio(it) => it.syntax,
            AnyCssValue::CssString(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssValue::CssAnyFunction(it) => std::fmt::Debug::fmt(it, f),
            AnyCssValue::CssCustomProperty(it) => std::fmt::Debug::fmt(it, f),
            AnyCssValue::CssDimension(it) => std::fmt::Debug::fmt(it, f),
            AnyCssValue::CssIdentifier(it) => std::fmt::Debug::fmt(it, f),
            AnyCssValue::CssNumber(it) => std::fmt::Debug::fmt(it, f),
            AnyCssValue::CssRatio(it) => std::fmt::Debug::fmt(it, f),
            AnyCssValue::CssString(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssValue> for SyntaxNode {
    fn from(n: AnyCssValue) -> SyntaxNode {
        match n {
            AnyCssValue::CssAnyFunction(it) => it.into(),
            AnyCssValue::CssCustomProperty(it) => it.into(),
            AnyCssValue::CssDimension(it) => it.into(),
            AnyCssValue::CssIdentifier(it) => it.into(),
            AnyCssValue::CssNumber(it) => it.into(),
            AnyCssValue::CssRatio(it) => it.into(),
            AnyCssValue::CssString(it) => it.into(),
        }
    }
}
impl From<AnyCssValue> for SyntaxElement {
    fn from(n: AnyCssValue) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssTypeSelector> for AnySimpleSelector {
    fn from(node: CssTypeSelector) -> AnySimpleSelector {
        AnySimpleSelector::CssTypeSelector(node)
    }
}
impl From<CssUniversalSelector> for AnySimpleSelector {
    fn from(node: CssUniversalSelector) -> AnySimpleSelector {
        AnySimpleSelector::CssUniversalSelector(node)
    }
}
impl AstNode for AnySimpleSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssTypeSelector::KIND_SET.union(CssUniversalSelector::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_TYPE_SELECTOR | CSS_UNIVERSAL_SELECTOR)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_TYPE_SELECTOR => AnySimpleSelector::CssTypeSelector(CssTypeSelector { syntax }),
            CSS_UNIVERSAL_SELECTOR => {
                AnySimpleSelector::CssUniversalSelector(CssUniversalSelector { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnySimpleSelector::CssTypeSelector(it) => &it.syntax,
            AnySimpleSelector::CssUniversalSelector(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnySimpleSelector::CssTypeSelector(it) => it.syntax,
            AnySimpleSelector::CssUniversalSelector(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnySimpleSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnySimpleSelector::CssTypeSelector(it) => std::fmt::Debug::fmt(it, f),
            AnySimpleSelector::CssUniversalSelector(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySimpleSelector> for SyntaxNode {
    fn from(n: AnySimpleSelector) -> SyntaxNode {
        match n {
            AnySimpleSelector::CssTypeSelector(it) => it.into(),
            AnySimpleSelector::CssUniversalSelector(it) => it.into(),
        }
    }
}
impl From<AnySimpleSelector> for SyntaxElement {
    fn from(n: AnySimpleSelector) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyCssAtMediaQueryFeatureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssAtMediaQueryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssAttributeMatcherValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssCompoundSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssNthMultiplier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssPseudoClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssPseudoClassNth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssPseudoClassNthSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssPseudoElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssPseudoValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssRelativeSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssSubSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySimpleSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAnyFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtKeyframes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtKeyframesBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtMedia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtMediaQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtMediaQueryConsequent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtMediaQueryFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtMediaQueryFeatureBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtMediaQueryFeatureCompare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtMediaQueryFeaturePlain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtMediaQueryFeatureRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtMediaQueryRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAttributeMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAttributeMatcherValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAttributeSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssClassSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssComplexSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssCompoundSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssCustomProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssDeclarationImportant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssIdSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssKeyframesBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssKeyframesSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssNthMultiplier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssNthOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPercentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassFunctionCompoundSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassFunctionCompoundSelectorList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassFunctionIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassFunctionNth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassFunctionRelativeSelectorList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassFunctionSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassFunctionSelectorList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassFunctionValueList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassNth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassNthIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassNthNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassNthSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassOfNthSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoElementFunctionIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoElementFunctionSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoElementIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoElementSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssRatio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssRelativeSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssSimpleFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssTypeSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssUniversalSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssVarFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssVarFunctionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssBogus {
    syntax: SyntaxNode,
}
impl CssBogus {
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
impl AstNode for CssBogus {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS
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
impl std::fmt::Debug for CssBogus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogus")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogus> for SyntaxNode {
    fn from(n: CssBogus) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssBogus> for SyntaxElement {
    fn from(n: CssBogus) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssBogusBody {
    syntax: SyntaxNode,
}
impl CssBogusBody {
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
impl AstNode for CssBogusBody {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_BODY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_BODY
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
impl std::fmt::Debug for CssBogusBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusBody")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusBody> for SyntaxNode {
    fn from(n: CssBogusBody) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssBogusBody> for SyntaxElement {
    fn from(n: CssBogusBody) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssBogusRule {
    syntax: SyntaxNode,
}
impl CssBogusRule {
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
impl AstNode for CssBogusRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_RULE
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
impl std::fmt::Debug for CssBogusRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusRule")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusRule> for SyntaxNode {
    fn from(n: CssBogusRule) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssBogusRule> for SyntaxElement {
    fn from(n: CssBogusRule) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssBogusSelector {
    syntax: SyntaxNode,
}
impl CssBogusSelector {
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
impl AstNode for CssBogusSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_SELECTOR
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
impl std::fmt::Debug for CssBogusSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusSelector")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusSelector> for SyntaxNode {
    fn from(n: CssBogusSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssBogusSelector> for SyntaxElement {
    fn from(n: CssBogusSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssBogusSubSelector {
    syntax: SyntaxNode,
}
impl CssBogusSubSelector {
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
impl AstNode for CssBogusSubSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_SUB_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_SUB_SELECTOR
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
impl std::fmt::Debug for CssBogusSubSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusSubSelector")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusSubSelector> for SyntaxNode {
    fn from(n: CssBogusSubSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssBogusSubSelector> for SyntaxElement {
    fn from(n: CssBogusSubSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssAtKeyframesItemList {
    syntax_list: SyntaxList,
}
impl CssAtKeyframesItemList {
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
impl AstNode for CssAtKeyframesItemList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_KEYFRAMES_ITEM_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_AT_KEYFRAMES_ITEM_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<CssAtKeyframesItemList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssAtKeyframesItemList {
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
impl Serialize for CssAtKeyframesItemList {
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
impl AstNodeList for CssAtKeyframesItemList {
    type Language = Language;
    type Node = CssKeyframesBlock;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssAtKeyframesItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssAtKeyframesItemList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssAtKeyframesItemList {
    type Item = CssKeyframesBlock;
    type IntoIter = AstNodeListIterator<Language, CssKeyframesBlock>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssAtKeyframesItemList {
    type Item = CssKeyframesBlock;
    type IntoIter = AstNodeListIterator<Language, CssKeyframesBlock>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssAtMediaQueryList {
    syntax_list: SyntaxList,
}
impl CssAtMediaQueryList {
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
impl AstNode for CssAtMediaQueryList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_MEDIA_QUERY_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_AT_MEDIA_QUERY_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<CssAtMediaQueryList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssAtMediaQueryList {
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
impl Serialize for CssAtMediaQueryList {
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
impl AstSeparatedList for CssAtMediaQueryList {
    type Language = Language;
    type Node = CssAtMediaQuery;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssAtMediaQueryList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssAtMediaQueryList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssAtMediaQueryList {
    type Item = SyntaxResult<CssAtMediaQuery>;
    type IntoIter = AstSeparatedListNodesIterator<Language, CssAtMediaQuery>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssAtMediaQueryList {
    type Item = SyntaxResult<CssAtMediaQuery>;
    type IntoIter = AstSeparatedListNodesIterator<Language, CssAtMediaQuery>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssCompoundSelectorList {
    syntax_list: SyntaxList,
}
impl CssCompoundSelectorList {
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
impl AstNode for CssCompoundSelectorList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_COMPOUND_SELECTOR_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_COMPOUND_SELECTOR_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<CssCompoundSelectorList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssCompoundSelectorList {
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
impl Serialize for CssCompoundSelectorList {
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
impl AstSeparatedList for CssCompoundSelectorList {
    type Language = Language;
    type Node = AnyCssCompoundSelector;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssCompoundSelectorList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssCompoundSelectorList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssCompoundSelectorList {
    type Item = SyntaxResult<AnyCssCompoundSelector>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssCompoundSelector>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssCompoundSelectorList {
    type Item = SyntaxResult<AnyCssCompoundSelector>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssCompoundSelector>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssDeclarationList {
    syntax_list: SyntaxList,
}
impl CssDeclarationList {
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
impl AstNode for CssDeclarationList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_DECLARATION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_DECLARATION_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<CssDeclarationList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssDeclarationList {
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
impl Serialize for CssDeclarationList {
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
impl AstNodeList for CssDeclarationList {
    type Language = Language;
    type Node = CssDeclaration;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssDeclarationList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssDeclarationList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssDeclarationList {
    type Item = CssDeclaration;
    type IntoIter = AstNodeListIterator<Language, CssDeclaration>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssDeclarationList {
    type Item = CssDeclaration;
    type IntoIter = AstNodeListIterator<Language, CssDeclaration>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssKeyframesSelectorList {
    syntax_list: SyntaxList,
}
impl CssKeyframesSelectorList {
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
impl AstNode for CssKeyframesSelectorList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_KEYFRAMES_SELECTOR_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_KEYFRAMES_SELECTOR_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<CssKeyframesSelectorList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssKeyframesSelectorList {
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
impl Serialize for CssKeyframesSelectorList {
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
impl AstSeparatedList for CssKeyframesSelectorList {
    type Language = Language;
    type Node = CssKeyframesSelector;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssKeyframesSelectorList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssKeyframesSelectorList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssKeyframesSelectorList {
    type Item = SyntaxResult<CssKeyframesSelector>;
    type IntoIter = AstSeparatedListNodesIterator<Language, CssKeyframesSelector>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssKeyframesSelectorList {
    type Item = SyntaxResult<CssKeyframesSelector>;
    type IntoIter = AstSeparatedListNodesIterator<Language, CssKeyframesSelector>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssParameterList {
    syntax_list: SyntaxList,
}
impl CssParameterList {
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
impl AstNode for CssParameterList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PARAMETER_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PARAMETER_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<CssParameterList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssParameterList {
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
impl Serialize for CssParameterList {
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
impl AstNodeList for CssParameterList {
    type Language = Language;
    type Node = CssParameter;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssParameterList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssParameterList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssParameterList {
    type Item = CssParameter;
    type IntoIter = AstNodeListIterator<Language, CssParameter>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssParameterList {
    type Item = CssParameter;
    type IntoIter = AstNodeListIterator<Language, CssParameter>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssPseudoValueList {
    syntax_list: SyntaxList,
}
impl CssPseudoValueList {
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
impl AstNode for CssPseudoValueList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_VALUE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_VALUE_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<CssPseudoValueList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssPseudoValueList {
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
impl Serialize for CssPseudoValueList {
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
impl AstSeparatedList for CssPseudoValueList {
    type Language = Language;
    type Node = AnyCssPseudoValue;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssPseudoValueList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssPseudoValueList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssPseudoValueList {
    type Item = SyntaxResult<AnyCssPseudoValue>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssPseudoValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssPseudoValueList {
    type Item = SyntaxResult<AnyCssPseudoValue>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssPseudoValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssRelativeSelectorList {
    syntax_list: SyntaxList,
}
impl CssRelativeSelectorList {
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
impl AstNode for CssRelativeSelectorList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_RELATIVE_SELECTOR_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_RELATIVE_SELECTOR_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<CssRelativeSelectorList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssRelativeSelectorList {
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
impl Serialize for CssRelativeSelectorList {
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
impl AstSeparatedList for CssRelativeSelectorList {
    type Language = Language;
    type Node = AnyCssRelativeSelector;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssRelativeSelectorList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssRelativeSelectorList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssRelativeSelectorList {
    type Item = SyntaxResult<AnyCssRelativeSelector>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssRelativeSelector>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssRelativeSelectorList {
    type Item = SyntaxResult<AnyCssRelativeSelector>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssRelativeSelector>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssRuleList {
    syntax_list: SyntaxList,
}
impl CssRuleList {
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
impl AstNode for CssRuleList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_RULE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_RULE_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<CssRuleList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssRuleList {
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
impl Serialize for CssRuleList {
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
impl AstNodeList for CssRuleList {
    type Language = Language;
    type Node = AnyCssRule;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssRuleList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssRuleList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssRuleList {
    type Item = AnyCssRule;
    type IntoIter = AstNodeListIterator<Language, AnyCssRule>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssRuleList {
    type Item = AnyCssRule;
    type IntoIter = AstNodeListIterator<Language, AnyCssRule>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssSelectorList {
    syntax_list: SyntaxList,
}
impl CssSelectorList {
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
impl AstNode for CssSelectorList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SELECTOR_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SELECTOR_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<CssSelectorList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssSelectorList {
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
impl Serialize for CssSelectorList {
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
impl AstSeparatedList for CssSelectorList {
    type Language = Language;
    type Node = AnyCssSelector;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssSelectorList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssSelectorList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssSelectorList {
    type Item = SyntaxResult<AnyCssSelector>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssSelector>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssSelectorList {
    type Item = SyntaxResult<AnyCssSelector>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssSelector>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssSubSelectorList {
    syntax_list: SyntaxList,
}
impl CssSubSelectorList {
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
impl AstNode for CssSubSelectorList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SUB_SELECTOR_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SUB_SELECTOR_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<CssSubSelectorList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssSubSelectorList {
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
impl Serialize for CssSubSelectorList {
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
impl AstNodeList for CssSubSelectorList {
    type Language = Language;
    type Node = AnyCssSubSelector;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssSubSelectorList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssSubSelectorList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssSubSelectorList {
    type Item = AnyCssSubSelector;
    type IntoIter = AstNodeListIterator<Language, AnyCssSubSelector>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssSubSelectorList {
    type Item = AnyCssSubSelector;
    type IntoIter = AstNodeListIterator<Language, AnyCssSubSelector>;
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
