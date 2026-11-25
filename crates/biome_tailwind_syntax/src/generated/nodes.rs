//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(dead_code)]
#![allow(unused)]
use crate::{
    TailwindLanguage as Language, TailwindSyntaxElement as SyntaxElement,
    TailwindSyntaxElementChildren as SyntaxElementChildren,
    TailwindSyntaxKind::{self as SyntaxKind, *},
    TailwindSyntaxList as SyntaxList, TailwindSyntaxNode as SyntaxNode,
    TailwindSyntaxToken as SyntaxToken,
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
pub struct CssBinaryExpression {
    pub(crate) syntax: SyntaxNode,
}
impl CssBinaryExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssBinaryExpressionFields {
        CssBinaryExpressionFields {
            left: self.left(),
            operator: self.operator(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyCssExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyCssExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssBinaryExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssBinaryExpressionFields {
    pub left: SyntaxResult<AnyCssExpression>,
    pub operator: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyCssExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssColor {
    pub(crate) syntax: SyntaxNode,
}
impl CssColor {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssColorFields {
        CssColorFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for CssColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssColorFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssDashedIdentifier {
    pub(crate) syntax: SyntaxNode,
}
impl CssDashedIdentifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssDashedIdentifierFields {
        CssDashedIdentifierFields {
            ident_token: self.ident_token(),
        }
    }
    pub fn ident_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for CssDashedIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssDashedIdentifierFields {
    pub ident_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssFunction {
    pub(crate) syntax: SyntaxNode,
}
impl CssFunction {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssFunctionFields {
        CssFunctionFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            parameters: self.parameters(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn parameters(&self) -> CssParameterList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for CssFunction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssFunctionFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub parameters: CssParameterList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssGenericDelimiter {
    pub(crate) syntax: SyntaxNode,
}
impl CssGenericDelimiter {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssGenericDelimiterFields {
        CssGenericDelimiterFields {
            value: self.value(),
        }
    }
    pub fn value(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for CssGenericDelimiter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssGenericDelimiterFields {
    pub value: SyntaxResult<SyntaxToken>,
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
            ident_token: self.ident_token(),
        }
    }
    pub fn ident_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for CssIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssIdentifierFields {
    pub ident_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssListOfComponentValuesExpression {
    pub(crate) syntax: SyntaxNode,
}
impl CssListOfComponentValuesExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssListOfComponentValuesExpressionFields {
        CssListOfComponentValuesExpressionFields {
            css_component_value_list: self.css_component_value_list(),
        }
    }
    pub fn css_component_value_list(&self) -> CssComponentValueList {
        support::list(&self.syntax, 0usize)
    }
}
impl Serialize for CssListOfComponentValuesExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssListOfComponentValuesExpressionFields {
    pub css_component_value_list: CssComponentValueList,
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
impl Serialize for CssNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssNumberFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssParenthesizedExpression {
    pub(crate) syntax: SyntaxNode,
}
impl CssParenthesizedExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssParenthesizedExpressionFields {
        CssParenthesizedExpressionFields {
            l_paren_token: self.l_paren_token(),
            expression: self.expression(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expression(&self) -> Option<AnyCssExpression> {
        support::node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for CssParenthesizedExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssParenthesizedExpressionFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub expression: Option<AnyCssExpression>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
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
            value_token: self.value_token(),
            remainder_token: self.remainder_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn remainder_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for CssPercentage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssPercentageFields {
    pub value_token: SyntaxResult<SyntaxToken>,
    pub remainder_token: SyntaxResult<SyntaxToken>,
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
            left: self.left(),
            slash_token: self.slash_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<CssNumber> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn slash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<CssNumber> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssRatio {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssRatioFields {
    pub left: SyntaxResult<CssNumber>,
    pub slash_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<CssNumber>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssRegularDimension {
    pub(crate) syntax: SyntaxNode,
}
impl CssRegularDimension {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssRegularDimensionFields {
        CssRegularDimensionFields {
            value_token: self.value_token(),
            unit_token: self.unit_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn unit_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for CssRegularDimension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssRegularDimensionFields {
    pub value_token: SyntaxResult<SyntaxToken>,
    pub unit_token: SyntaxResult<SyntaxToken>,
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
impl Serialize for CssString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssStringFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssUnaryExpression {
    pub(crate) syntax: SyntaxNode,
}
impl CssUnaryExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssUnaryExpressionFields {
        CssUnaryExpressionFields {
            operator: self.operator(),
            argument: self.argument(),
        }
    }
    pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn argument(&self) -> SyntaxResult<AnyCssExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for CssUnaryExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssUnaryExpressionFields {
    pub operator: SyntaxResult<SyntaxToken>,
    pub argument: SyntaxResult<AnyCssExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssUnknownDimension {
    pub(crate) syntax: SyntaxNode,
}
impl CssUnknownDimension {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssUnknownDimensionFields {
        CssUnknownDimensionFields {
            value_token: self.value_token(),
            unit_token: self.unit_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn unit_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for CssUnknownDimension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssUnknownDimensionFields {
    pub value_token: SyntaxResult<SyntaxToken>,
    pub unit_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssUrlFunction {
    pub(crate) syntax: SyntaxNode,
}
impl CssUrlFunction {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssUrlFunctionFields {
        CssUrlFunctionFields {
            url_token: self.url_token(),
            l_paren_token: self.l_paren_token(),
            value: self.value(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn url_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> Option<AnyCssUrlValue> {
        support::node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for CssUrlFunction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssUrlFunctionFields {
    pub url_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub value: Option<AnyCssUrlValue>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssUrlValueRaw {
    pub(crate) syntax: SyntaxNode,
}
impl CssUrlValueRaw {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssUrlValueRawFields {
        CssUrlValueRawFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for CssUrlValueRaw {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssUrlValueRawFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
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
            value: self.value(),
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
    pub fn value(&self) -> CssGenericComponentValueList {
        support::list(&self.syntax, 3usize)
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
    pub value: CssGenericComponentValueList,
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
            value: self.value(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> CssGenericComponentValueList {
        support::list(&self.syntax, 1usize)
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
    pub value: CssGenericComponentValueList,
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
pub struct TwArbitraryVariantSegment {
    pub(crate) syntax: SyntaxNode,
}
impl TwArbitraryVariantSegment {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TwArbitraryVariantSegmentFields {
        TwArbitraryVariantSegmentFields {
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
impl Serialize for TwArbitraryVariantSegment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TwArbitraryVariantSegmentFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub value_token: SyntaxResult<SyntaxToken>,
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
            type_hint: self.type_hint(),
            value: self.value(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn type_hint(&self) -> Option<TwTypeHint> {
        support::node(&self.syntax, 1usize)
    }
    pub fn value(&self) -> CssParameterList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
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
    pub type_hint: Option<TwTypeHint>,
    pub value: CssParameterList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TwCssVariableVariantSegment {
    pub(crate) syntax: SyntaxNode,
}
impl TwCssVariableVariantSegment {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TwCssVariableVariantSegmentFields {
        TwCssVariableVariantSegmentFields {
            l_paren_token: self.l_paren_token(),
            value: self.value(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> CssParameterList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for TwCssVariableVariantSegment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TwCssVariableVariantSegmentFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub value: CssParameterList,
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
            legacy_important_token: self.legacy_important_token(),
            negative_token: self.negative_token(),
            candidate: self.candidate(),
            excl_token: self.excl_token(),
        }
    }
    pub fn variants(&self) -> TwVariantList {
        support::list(&self.syntax, 0usize)
    }
    pub fn legacy_important_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
    pub fn negative_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
    pub fn candidate(&self) -> SyntaxResult<AnyTwCandidate> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn excl_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 4usize)
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
    pub legacy_important_token: Option<SyntaxToken>,
    pub negative_token: Option<SyntaxToken>,
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
pub struct TwNamedVariantSegment {
    pub(crate) syntax: SyntaxNode,
}
impl TwNamedVariantSegment {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TwNamedVariantSegmentFields {
        TwNamedVariantSegmentFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TwNamedVariantSegment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TwNamedVariantSegmentFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TwNumberValue {
    pub(crate) syntax: SyntaxNode,
}
impl TwNumberValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TwNumberValueFields {
        TwNumberValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for TwNumberValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TwNumberValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TwPercentageValue {
    pub(crate) syntax: SyntaxNode,
}
impl TwPercentageValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TwPercentageValueFields {
        TwPercentageValueFields {
            value_token: self.value_token(),
            remainder_token: self.remainder_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn remainder_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for TwPercentageValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TwPercentageValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
    pub remainder_token: SyntaxResult<SyntaxToken>,
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
            leading_whitespace_token: self.leading_whitespace_token(),
            candidates: self.candidates(),
            eof_token: self.eof_token(),
        }
    }
    pub fn bom_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn leading_whitespace_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
    pub fn candidates(&self) -> TwCandidateList {
        support::list(&self.syntax, 2usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
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
    pub leading_whitespace_token: Option<SyntaxToken>,
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
            modifier: self.modifier(),
        }
    }
    pub fn base_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn modifier(&self) -> Option<AnyTwModifier> {
        support::node(&self.syntax, 1usize)
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
    pub modifier: Option<AnyTwModifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TwTypeHint {
    pub(crate) syntax: SyntaxNode,
}
impl TwTypeHint {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TwTypeHintFields {
        TwTypeHintFields {
            name_token: self.name_token(),
            colon_token: self.colon_token(),
        }
    }
    pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for TwTypeHint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TwTypeHintFields {
    pub name_token: SyntaxResult<SyntaxToken>,
    pub colon_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TwVariantExpression {
    pub(crate) syntax: SyntaxNode,
}
impl TwVariantExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> TwVariantExpressionFields {
        TwVariantExpressionFields {
            segments: self.segments(),
            glued_value: self.glued_value(),
            modifier: self.modifier(),
        }
    }
    pub fn segments(&self) -> TwVariantSegmentList {
        support::list(&self.syntax, 0usize)
    }
    pub fn glued_value(&self) -> Option<TwArbitraryVariantSegment> {
        support::node(&self.syntax, 1usize)
    }
    pub fn modifier(&self) -> Option<AnyTwModifier> {
        support::node(&self.syntax, 2usize)
    }
}
impl Serialize for TwVariantExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct TwVariantExpressionFields {
    pub segments: TwVariantSegmentList,
    pub glued_value: Option<TwArbitraryVariantSegment>,
    pub modifier: Option<AnyTwModifier>,
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssDimension {
    CssRegularDimension(CssRegularDimension),
    CssUnknownDimension(CssUnknownDimension),
}
impl AnyCssDimension {
    pub fn as_css_regular_dimension(&self) -> Option<&CssRegularDimension> {
        match &self {
            Self::CssRegularDimension(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_unknown_dimension(&self) -> Option<&CssUnknownDimension> {
        match &self {
            Self::CssUnknownDimension(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssExpression {
    CssBinaryExpression(CssBinaryExpression),
    CssBogusPropertyValue(CssBogusPropertyValue),
    CssListOfComponentValuesExpression(CssListOfComponentValuesExpression),
    CssParenthesizedExpression(CssParenthesizedExpression),
    CssUnaryExpression(CssUnaryExpression),
}
impl AnyCssExpression {
    pub fn as_css_binary_expression(&self) -> Option<&CssBinaryExpression> {
        match &self {
            Self::CssBinaryExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_bogus_property_value(&self) -> Option<&CssBogusPropertyValue> {
        match &self {
            Self::CssBogusPropertyValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_list_of_component_values_expression(
        &self,
    ) -> Option<&CssListOfComponentValuesExpression> {
        match &self {
            Self::CssListOfComponentValuesExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_parenthesized_expression(&self) -> Option<&CssParenthesizedExpression> {
        match &self {
            Self::CssParenthesizedExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_unary_expression(&self) -> Option<&CssUnaryExpression> {
        match &self {
            Self::CssUnaryExpression(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssFunction {
    CssFunction(CssFunction),
    CssUrlFunction(CssUrlFunction),
}
impl AnyCssFunction {
    pub fn as_css_function(&self) -> Option<&CssFunction> {
        match &self {
            Self::CssFunction(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_url_function(&self) -> Option<&CssUrlFunction> {
        match &self {
            Self::CssUrlFunction(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssGenericComponentValue {
    AnyCssValue(AnyCssValue),
    CssGenericDelimiter(CssGenericDelimiter),
}
impl AnyCssGenericComponentValue {
    pub fn as_any_css_value(&self) -> Option<&AnyCssValue> {
        match &self {
            Self::AnyCssValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_generic_delimiter(&self) -> Option<&CssGenericDelimiter> {
        match &self {
            Self::CssGenericDelimiter(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssUrlValue {
    CssParameterList(CssParameterList),
    CssString(CssString),
    CssUrlValueRaw(CssUrlValueRaw),
}
impl AnyCssUrlValue {
    pub fn as_css_parameter_list(&self) -> Option<&CssParameterList> {
        match &self {
            Self::CssParameterList(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_string(&self) -> Option<&CssString> {
        match &self {
            Self::CssString(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_url_value_raw(&self) -> Option<&CssUrlValueRaw> {
        match &self {
            Self::CssUrlValueRaw(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssValue {
    AnyCssDimension(AnyCssDimension),
    AnyCssFunction(AnyCssFunction),
    CssBinaryExpression(CssBinaryExpression),
    CssBogusPropertyValue(CssBogusPropertyValue),
    CssColor(CssColor),
    CssDashedIdentifier(CssDashedIdentifier),
    CssIdentifier(CssIdentifier),
    CssNumber(CssNumber),
    CssParenthesizedExpression(CssParenthesizedExpression),
    CssPercentage(CssPercentage),
    CssRatio(CssRatio),
    CssString(CssString),
    CssUnaryExpression(CssUnaryExpression),
}
impl AnyCssValue {
    pub fn as_any_css_dimension(&self) -> Option<&AnyCssDimension> {
        match &self {
            Self::AnyCssDimension(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_any_css_function(&self) -> Option<&AnyCssFunction> {
        match &self {
            Self::AnyCssFunction(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_binary_expression(&self) -> Option<&CssBinaryExpression> {
        match &self {
            Self::CssBinaryExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_bogus_property_value(&self) -> Option<&CssBogusPropertyValue> {
        match &self {
            Self::CssBogusPropertyValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_color(&self) -> Option<&CssColor> {
        match &self {
            Self::CssColor(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_dashed_identifier(&self) -> Option<&CssDashedIdentifier> {
        match &self {
            Self::CssDashedIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_identifier(&self) -> Option<&CssIdentifier> {
        match &self {
            Self::CssIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_number(&self) -> Option<&CssNumber> {
        match &self {
            Self::CssNumber(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_parenthesized_expression(&self) -> Option<&CssParenthesizedExpression> {
        match &self {
            Self::CssParenthesizedExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_percentage(&self) -> Option<&CssPercentage> {
        match &self {
            Self::CssPercentage(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_ratio(&self) -> Option<&CssRatio> {
        match &self {
            Self::CssRatio(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_string(&self) -> Option<&CssString> {
        match &self {
            Self::CssString(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_unary_expression(&self) -> Option<&CssUnaryExpression> {
        match &self {
            Self::CssUnaryExpression(item) => Some(item),
            _ => None,
        }
    }
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
    TwNumberValue(TwNumberValue),
    TwPercentageValue(TwPercentageValue),
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
    pub fn as_tw_number_value(&self) -> Option<&TwNumberValue> {
        match &self {
            Self::TwNumberValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_tw_percentage_value(&self) -> Option<&TwPercentageValue> {
        match &self {
            Self::TwPercentageValue(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyTwVariant {
    TwArbitraryVariant(TwArbitraryVariant),
    TwBogusVariant(TwBogusVariant),
    TwVariantExpression(TwVariantExpression),
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
    pub fn as_tw_variant_expression(&self) -> Option<&TwVariantExpression> {
        match &self {
            Self::TwVariantExpression(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyTwVariantSegment {
    TwArbitraryVariantSegment(TwArbitraryVariantSegment),
    TwBogusVariantSegment(TwBogusVariantSegment),
    TwCssVariableVariantSegment(TwCssVariableVariantSegment),
    TwNamedVariantSegment(TwNamedVariantSegment),
}
impl AnyTwVariantSegment {
    pub fn as_tw_arbitrary_variant_segment(&self) -> Option<&TwArbitraryVariantSegment> {
        match &self {
            Self::TwArbitraryVariantSegment(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_tw_bogus_variant_segment(&self) -> Option<&TwBogusVariantSegment> {
        match &self {
            Self::TwBogusVariantSegment(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_tw_css_variable_variant_segment(&self) -> Option<&TwCssVariableVariantSegment> {
        match &self {
            Self::TwCssVariableVariantSegment(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_tw_named_variant_segment(&self) -> Option<&TwNamedVariantSegment> {
        match &self {
            Self::TwNamedVariantSegment(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for CssBinaryExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BINARY_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BINARY_EXPRESSION
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
impl std::fmt::Debug for CssBinaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<CssBinaryExpression> for SyntaxNode {
    fn from(n: CssBinaryExpression) -> Self {
        n.syntax
    }
}
impl From<CssBinaryExpression> for SyntaxElement {
    fn from(n: CssBinaryExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssColor {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_COLOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_COLOR
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
impl std::fmt::Debug for CssColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<CssColor> for SyntaxNode {
    fn from(n: CssColor) -> Self {
        n.syntax
    }
}
impl From<CssColor> for SyntaxElement {
    fn from(n: CssColor) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssDashedIdentifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_DASHED_IDENTIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_DASHED_IDENTIFIER
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
impl std::fmt::Debug for CssDashedIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<CssDashedIdentifier> for SyntaxNode {
    fn from(n: CssDashedIdentifier) -> Self {
        n.syntax
    }
}
impl From<CssDashedIdentifier> for SyntaxElement {
    fn from(n: CssDashedIdentifier) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssFunction {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_FUNCTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_FUNCTION
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
impl std::fmt::Debug for CssFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<CssFunction> for SyntaxNode {
    fn from(n: CssFunction) -> Self {
        n.syntax
    }
}
impl From<CssFunction> for SyntaxElement {
    fn from(n: CssFunction) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssGenericDelimiter {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_GENERIC_DELIMITER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_GENERIC_DELIMITER
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
impl std::fmt::Debug for CssGenericDelimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<CssGenericDelimiter> for SyntaxNode {
    fn from(n: CssGenericDelimiter) -> Self {
        n.syntax
    }
}
impl From<CssGenericDelimiter> for SyntaxElement {
    fn from(n: CssGenericDelimiter) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<CssIdentifier> for SyntaxNode {
    fn from(n: CssIdentifier) -> Self {
        n.syntax
    }
}
impl From<CssIdentifier> for SyntaxElement {
    fn from(n: CssIdentifier) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssListOfComponentValuesExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        CSS_LIST_OF_COMPONENT_VALUES_EXPRESSION as u16,
    ));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_LIST_OF_COMPONENT_VALUES_EXPRESSION
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
impl std::fmt::Debug for CssListOfComponentValuesExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<CssListOfComponentValuesExpression> for SyntaxNode {
    fn from(n: CssListOfComponentValuesExpression) -> Self {
        n.syntax
    }
}
impl From<CssListOfComponentValuesExpression> for SyntaxElement {
    fn from(n: CssListOfComponentValuesExpression) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<CssNumber> for SyntaxNode {
    fn from(n: CssNumber) -> Self {
        n.syntax
    }
}
impl From<CssNumber> for SyntaxElement {
    fn from(n: CssNumber) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssParenthesizedExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PARENTHESIZED_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PARENTHESIZED_EXPRESSION
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
impl std::fmt::Debug for CssParenthesizedExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<CssParenthesizedExpression> for SyntaxNode {
    fn from(n: CssParenthesizedExpression) -> Self {
        n.syntax
    }
}
impl From<CssParenthesizedExpression> for SyntaxElement {
    fn from(n: CssParenthesizedExpression) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<CssPercentage> for SyntaxNode {
    fn from(n: CssPercentage) -> Self {
        n.syntax
    }
}
impl From<CssPercentage> for SyntaxElement {
    fn from(n: CssPercentage) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<CssRatio> for SyntaxNode {
    fn from(n: CssRatio) -> Self {
        n.syntax
    }
}
impl From<CssRatio> for SyntaxElement {
    fn from(n: CssRatio) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssRegularDimension {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_REGULAR_DIMENSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_REGULAR_DIMENSION
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
impl std::fmt::Debug for CssRegularDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<CssRegularDimension> for SyntaxNode {
    fn from(n: CssRegularDimension) -> Self {
        n.syntax
    }
}
impl From<CssRegularDimension> for SyntaxElement {
    fn from(n: CssRegularDimension) -> Self {
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
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<CssString> for SyntaxNode {
    fn from(n: CssString) -> Self {
        n.syntax
    }
}
impl From<CssString> for SyntaxElement {
    fn from(n: CssString) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssUnaryExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_UNARY_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_UNARY_EXPRESSION
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
impl std::fmt::Debug for CssUnaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<CssUnaryExpression> for SyntaxNode {
    fn from(n: CssUnaryExpression) -> Self {
        n.syntax
    }
}
impl From<CssUnaryExpression> for SyntaxElement {
    fn from(n: CssUnaryExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssUnknownDimension {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_UNKNOWN_DIMENSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_UNKNOWN_DIMENSION
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
impl std::fmt::Debug for CssUnknownDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<CssUnknownDimension> for SyntaxNode {
    fn from(n: CssUnknownDimension) -> Self {
        n.syntax
    }
}
impl From<CssUnknownDimension> for SyntaxElement {
    fn from(n: CssUnknownDimension) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssUrlFunction {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_URL_FUNCTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_URL_FUNCTION
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
impl std::fmt::Debug for CssUrlFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<CssUrlFunction> for SyntaxNode {
    fn from(n: CssUrlFunction) -> Self {
        n.syntax
    }
}
impl From<CssUrlFunction> for SyntaxElement {
    fn from(n: CssUrlFunction) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssUrlValueRaw {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_URL_VALUE_RAW as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_URL_VALUE_RAW
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
impl std::fmt::Debug for CssUrlValueRaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<CssUrlValueRaw> for SyntaxNode {
    fn from(n: CssUrlValueRaw) -> Self {
        n.syntax
    }
}
impl From<CssUrlValueRaw> for SyntaxElement {
    fn from(n: CssUrlValueRaw) -> Self {
        n.syntax.into()
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
        debug_syntax_node(&self.syntax, f)
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
        debug_syntax_node(&self.syntax, f)
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
        debug_syntax_node(&self.syntax, f)
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
impl AstNode for TwArbitraryVariantSegment {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_ARBITRARY_VARIANT_SEGMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_ARBITRARY_VARIANT_SEGMENT
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
impl std::fmt::Debug for TwArbitraryVariantSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<TwArbitraryVariantSegment> for SyntaxNode {
    fn from(n: TwArbitraryVariantSegment) -> Self {
        n.syntax
    }
}
impl From<TwArbitraryVariantSegment> for SyntaxElement {
    fn from(n: TwArbitraryVariantSegment) -> Self {
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
        debug_syntax_node(&self.syntax, f)
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
impl AstNode for TwCssVariableVariantSegment {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_CSS_VARIABLE_VARIANT_SEGMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_CSS_VARIABLE_VARIANT_SEGMENT
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
impl std::fmt::Debug for TwCssVariableVariantSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<TwCssVariableVariantSegment> for SyntaxNode {
    fn from(n: TwCssVariableVariantSegment) -> Self {
        n.syntax
    }
}
impl From<TwCssVariableVariantSegment> for SyntaxElement {
    fn from(n: TwCssVariableVariantSegment) -> Self {
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
        debug_syntax_node(&self.syntax, f)
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
        debug_syntax_node(&self.syntax, f)
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
        debug_syntax_node(&self.syntax, f)
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
        debug_syntax_node(&self.syntax, f)
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
impl AstNode for TwNamedVariantSegment {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_NAMED_VARIANT_SEGMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_NAMED_VARIANT_SEGMENT
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
impl std::fmt::Debug for TwNamedVariantSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<TwNamedVariantSegment> for SyntaxNode {
    fn from(n: TwNamedVariantSegment) -> Self {
        n.syntax
    }
}
impl From<TwNamedVariantSegment> for SyntaxElement {
    fn from(n: TwNamedVariantSegment) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TwNumberValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_NUMBER_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_NUMBER_VALUE
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
impl std::fmt::Debug for TwNumberValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<TwNumberValue> for SyntaxNode {
    fn from(n: TwNumberValue) -> Self {
        n.syntax
    }
}
impl From<TwNumberValue> for SyntaxElement {
    fn from(n: TwNumberValue) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TwPercentageValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_PERCENTAGE_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_PERCENTAGE_VALUE
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
impl std::fmt::Debug for TwPercentageValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<TwPercentageValue> for SyntaxNode {
    fn from(n: TwPercentageValue) -> Self {
        n.syntax
    }
}
impl From<TwPercentageValue> for SyntaxElement {
    fn from(n: TwPercentageValue) -> Self {
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
        debug_syntax_node(&self.syntax, f)
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
        debug_syntax_node(&self.syntax, f)
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
impl AstNode for TwTypeHint {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_TYPE_HINT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_TYPE_HINT
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
impl std::fmt::Debug for TwTypeHint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<TwTypeHint> for SyntaxNode {
    fn from(n: TwTypeHint) -> Self {
        n.syntax
    }
}
impl From<TwTypeHint> for SyntaxElement {
    fn from(n: TwTypeHint) -> Self {
        n.syntax.into()
    }
}
impl AstNode for TwVariantExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_VARIANT_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_VARIANT_EXPRESSION
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
impl std::fmt::Debug for TwVariantExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<TwVariantExpression> for SyntaxNode {
    fn from(n: TwVariantExpression) -> Self {
        n.syntax
    }
}
impl From<TwVariantExpression> for SyntaxElement {
    fn from(n: TwVariantExpression) -> Self {
        n.syntax.into()
    }
}
impl From<CssRegularDimension> for AnyCssDimension {
    fn from(node: CssRegularDimension) -> Self {
        Self::CssRegularDimension(node)
    }
}
impl From<CssUnknownDimension> for AnyCssDimension {
    fn from(node: CssUnknownDimension) -> Self {
        Self::CssUnknownDimension(node)
    }
}
impl AstNode for AnyCssDimension {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssRegularDimension::KIND_SET.union(CssUnknownDimension::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_REGULAR_DIMENSION | CSS_UNKNOWN_DIMENSION)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_REGULAR_DIMENSION => Self::CssRegularDimension(CssRegularDimension { syntax }),
            CSS_UNKNOWN_DIMENSION => Self::CssUnknownDimension(CssUnknownDimension { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssRegularDimension(it) => it.syntax(),
            Self::CssUnknownDimension(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssRegularDimension(it) => it.into_syntax(),
            Self::CssUnknownDimension(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyCssDimension> for SyntaxNode {
    fn from(n: AnyCssDimension) -> Self {
        match n {
            AnyCssDimension::CssRegularDimension(it) => it.into_syntax(),
            AnyCssDimension::CssUnknownDimension(it) => it.into_syntax(),
        }
    }
}
impl From<AnyCssDimension> for SyntaxElement {
    fn from(n: AnyCssDimension) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBinaryExpression> for AnyCssExpression {
    fn from(node: CssBinaryExpression) -> Self {
        Self::CssBinaryExpression(node)
    }
}
impl From<CssBogusPropertyValue> for AnyCssExpression {
    fn from(node: CssBogusPropertyValue) -> Self {
        Self::CssBogusPropertyValue(node)
    }
}
impl From<CssListOfComponentValuesExpression> for AnyCssExpression {
    fn from(node: CssListOfComponentValuesExpression) -> Self {
        Self::CssListOfComponentValuesExpression(node)
    }
}
impl From<CssParenthesizedExpression> for AnyCssExpression {
    fn from(node: CssParenthesizedExpression) -> Self {
        Self::CssParenthesizedExpression(node)
    }
}
impl From<CssUnaryExpression> for AnyCssExpression {
    fn from(node: CssUnaryExpression) -> Self {
        Self::CssUnaryExpression(node)
    }
}
impl AstNode for AnyCssExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssBinaryExpression::KIND_SET
        .union(CssBogusPropertyValue::KIND_SET)
        .union(CssListOfComponentValuesExpression::KIND_SET)
        .union(CssParenthesizedExpression::KIND_SET)
        .union(CssUnaryExpression::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_BINARY_EXPRESSION
                | CSS_BOGUS_PROPERTY_VALUE
                | CSS_LIST_OF_COMPONENT_VALUES_EXPRESSION
                | CSS_PARENTHESIZED_EXPRESSION
                | CSS_UNARY_EXPRESSION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BINARY_EXPRESSION => Self::CssBinaryExpression(CssBinaryExpression { syntax }),
            CSS_BOGUS_PROPERTY_VALUE => {
                Self::CssBogusPropertyValue(CssBogusPropertyValue { syntax })
            }
            CSS_LIST_OF_COMPONENT_VALUES_EXPRESSION => {
                Self::CssListOfComponentValuesExpression(CssListOfComponentValuesExpression {
                    syntax,
                })
            }
            CSS_PARENTHESIZED_EXPRESSION => {
                Self::CssParenthesizedExpression(CssParenthesizedExpression { syntax })
            }
            CSS_UNARY_EXPRESSION => Self::CssUnaryExpression(CssUnaryExpression { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBinaryExpression(it) => it.syntax(),
            Self::CssBogusPropertyValue(it) => it.syntax(),
            Self::CssListOfComponentValuesExpression(it) => it.syntax(),
            Self::CssParenthesizedExpression(it) => it.syntax(),
            Self::CssUnaryExpression(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBinaryExpression(it) => it.into_syntax(),
            Self::CssBogusPropertyValue(it) => it.into_syntax(),
            Self::CssListOfComponentValuesExpression(it) => it.into_syntax(),
            Self::CssParenthesizedExpression(it) => it.into_syntax(),
            Self::CssUnaryExpression(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyCssExpression> for SyntaxNode {
    fn from(n: AnyCssExpression) -> Self {
        match n {
            AnyCssExpression::CssBinaryExpression(it) => it.into_syntax(),
            AnyCssExpression::CssBogusPropertyValue(it) => it.into_syntax(),
            AnyCssExpression::CssListOfComponentValuesExpression(it) => it.into_syntax(),
            AnyCssExpression::CssParenthesizedExpression(it) => it.into_syntax(),
            AnyCssExpression::CssUnaryExpression(it) => it.into_syntax(),
        }
    }
}
impl From<AnyCssExpression> for SyntaxElement {
    fn from(n: AnyCssExpression) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssFunction> for AnyCssFunction {
    fn from(node: CssFunction) -> Self {
        Self::CssFunction(node)
    }
}
impl From<CssUrlFunction> for AnyCssFunction {
    fn from(node: CssUrlFunction) -> Self {
        Self::CssUrlFunction(node)
    }
}
impl AstNode for AnyCssFunction {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssFunction::KIND_SET.union(CssUrlFunction::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_FUNCTION | CSS_URL_FUNCTION)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_FUNCTION => Self::CssFunction(CssFunction { syntax }),
            CSS_URL_FUNCTION => Self::CssUrlFunction(CssUrlFunction { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssFunction(it) => it.syntax(),
            Self::CssUrlFunction(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssFunction(it) => it.into_syntax(),
            Self::CssUrlFunction(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyCssFunction> for SyntaxNode {
    fn from(n: AnyCssFunction) -> Self {
        match n {
            AnyCssFunction::CssFunction(it) => it.into_syntax(),
            AnyCssFunction::CssUrlFunction(it) => it.into_syntax(),
        }
    }
}
impl From<AnyCssFunction> for SyntaxElement {
    fn from(n: AnyCssFunction) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssGenericDelimiter> for AnyCssGenericComponentValue {
    fn from(node: CssGenericDelimiter) -> Self {
        Self::CssGenericDelimiter(node)
    }
}
impl AstNode for AnyCssGenericComponentValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyCssValue::KIND_SET.union(CssGenericDelimiter::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_GENERIC_DELIMITER => true,
            k if AnyCssValue::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_GENERIC_DELIMITER => Self::CssGenericDelimiter(CssGenericDelimiter { syntax }),
            _ => {
                if let Some(any_css_value) = AnyCssValue::cast(syntax) {
                    return Some(Self::AnyCssValue(any_css_value));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssGenericDelimiter(it) => it.syntax(),
            Self::AnyCssValue(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssGenericDelimiter(it) => it.into_syntax(),
            Self::AnyCssValue(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssGenericComponentValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyCssGenericComponentValue> for SyntaxNode {
    fn from(n: AnyCssGenericComponentValue) -> Self {
        match n {
            AnyCssGenericComponentValue::AnyCssValue(it) => it.into_syntax(),
            AnyCssGenericComponentValue::CssGenericDelimiter(it) => it.into_syntax(),
        }
    }
}
impl From<AnyCssGenericComponentValue> for SyntaxElement {
    fn from(n: AnyCssGenericComponentValue) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssParameterList> for AnyCssUrlValue {
    fn from(node: CssParameterList) -> Self {
        Self::CssParameterList(node)
    }
}
impl From<CssString> for AnyCssUrlValue {
    fn from(node: CssString) -> Self {
        Self::CssString(node)
    }
}
impl From<CssUrlValueRaw> for AnyCssUrlValue {
    fn from(node: CssUrlValueRaw) -> Self {
        Self::CssUrlValueRaw(node)
    }
}
impl AstNode for AnyCssUrlValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssParameterList::KIND_SET
        .union(CssString::KIND_SET)
        .union(CssUrlValueRaw::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_PARAMETER_LIST | CSS_STRING | CSS_URL_VALUE_RAW)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_PARAMETER_LIST => Self::CssParameterList(CssParameterList::cast(syntax)?),
            CSS_STRING => Self::CssString(CssString { syntax }),
            CSS_URL_VALUE_RAW => Self::CssUrlValueRaw(CssUrlValueRaw { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssParameterList(it) => it.syntax(),
            Self::CssString(it) => it.syntax(),
            Self::CssUrlValueRaw(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssParameterList(it) => it.into_syntax(),
            Self::CssString(it) => it.into_syntax(),
            Self::CssUrlValueRaw(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssUrlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyCssUrlValue> for SyntaxNode {
    fn from(n: AnyCssUrlValue) -> Self {
        match n {
            AnyCssUrlValue::CssParameterList(it) => it.into_syntax(),
            AnyCssUrlValue::CssString(it) => it.into_syntax(),
            AnyCssUrlValue::CssUrlValueRaw(it) => it.into_syntax(),
        }
    }
}
impl From<AnyCssUrlValue> for SyntaxElement {
    fn from(n: AnyCssUrlValue) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBinaryExpression> for AnyCssValue {
    fn from(node: CssBinaryExpression) -> Self {
        Self::CssBinaryExpression(node)
    }
}
impl From<CssBogusPropertyValue> for AnyCssValue {
    fn from(node: CssBogusPropertyValue) -> Self {
        Self::CssBogusPropertyValue(node)
    }
}
impl From<CssColor> for AnyCssValue {
    fn from(node: CssColor) -> Self {
        Self::CssColor(node)
    }
}
impl From<CssDashedIdentifier> for AnyCssValue {
    fn from(node: CssDashedIdentifier) -> Self {
        Self::CssDashedIdentifier(node)
    }
}
impl From<CssIdentifier> for AnyCssValue {
    fn from(node: CssIdentifier) -> Self {
        Self::CssIdentifier(node)
    }
}
impl From<CssNumber> for AnyCssValue {
    fn from(node: CssNumber) -> Self {
        Self::CssNumber(node)
    }
}
impl From<CssParenthesizedExpression> for AnyCssValue {
    fn from(node: CssParenthesizedExpression) -> Self {
        Self::CssParenthesizedExpression(node)
    }
}
impl From<CssPercentage> for AnyCssValue {
    fn from(node: CssPercentage) -> Self {
        Self::CssPercentage(node)
    }
}
impl From<CssRatio> for AnyCssValue {
    fn from(node: CssRatio) -> Self {
        Self::CssRatio(node)
    }
}
impl From<CssString> for AnyCssValue {
    fn from(node: CssString) -> Self {
        Self::CssString(node)
    }
}
impl From<CssUnaryExpression> for AnyCssValue {
    fn from(node: CssUnaryExpression) -> Self {
        Self::CssUnaryExpression(node)
    }
}
impl AstNode for AnyCssValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyCssDimension::KIND_SET
        .union(AnyCssFunction::KIND_SET)
        .union(CssBinaryExpression::KIND_SET)
        .union(CssBogusPropertyValue::KIND_SET)
        .union(CssColor::KIND_SET)
        .union(CssDashedIdentifier::KIND_SET)
        .union(CssIdentifier::KIND_SET)
        .union(CssNumber::KIND_SET)
        .union(CssParenthesizedExpression::KIND_SET)
        .union(CssPercentage::KIND_SET)
        .union(CssRatio::KIND_SET)
        .union(CssString::KIND_SET)
        .union(CssUnaryExpression::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_BINARY_EXPRESSION
            | CSS_BOGUS_PROPERTY_VALUE
            | CSS_COLOR
            | CSS_DASHED_IDENTIFIER
            | CSS_IDENTIFIER
            | CSS_NUMBER
            | CSS_PARENTHESIZED_EXPRESSION
            | CSS_PERCENTAGE
            | CSS_RATIO
            | CSS_STRING
            | CSS_UNARY_EXPRESSION => true,
            k if AnyCssDimension::can_cast(k) => true,
            k if AnyCssFunction::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BINARY_EXPRESSION => Self::CssBinaryExpression(CssBinaryExpression { syntax }),
            CSS_BOGUS_PROPERTY_VALUE => {
                Self::CssBogusPropertyValue(CssBogusPropertyValue { syntax })
            }
            CSS_COLOR => Self::CssColor(CssColor { syntax }),
            CSS_DASHED_IDENTIFIER => Self::CssDashedIdentifier(CssDashedIdentifier { syntax }),
            CSS_IDENTIFIER => Self::CssIdentifier(CssIdentifier { syntax }),
            CSS_NUMBER => Self::CssNumber(CssNumber { syntax }),
            CSS_PARENTHESIZED_EXPRESSION => {
                Self::CssParenthesizedExpression(CssParenthesizedExpression { syntax })
            }
            CSS_PERCENTAGE => Self::CssPercentage(CssPercentage { syntax }),
            CSS_RATIO => Self::CssRatio(CssRatio { syntax }),
            CSS_STRING => Self::CssString(CssString { syntax }),
            CSS_UNARY_EXPRESSION => Self::CssUnaryExpression(CssUnaryExpression { syntax }),
            _ => {
                let syntax = match AnyCssDimension::try_cast(syntax) {
                    Ok(any_css_dimension) => {
                        return Some(Self::AnyCssDimension(any_css_dimension));
                    }
                    Err(syntax) => syntax,
                };
                if let Some(any_css_function) = AnyCssFunction::cast(syntax) {
                    return Some(Self::AnyCssFunction(any_css_function));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBinaryExpression(it) => it.syntax(),
            Self::CssBogusPropertyValue(it) => it.syntax(),
            Self::CssColor(it) => it.syntax(),
            Self::CssDashedIdentifier(it) => it.syntax(),
            Self::CssIdentifier(it) => it.syntax(),
            Self::CssNumber(it) => it.syntax(),
            Self::CssParenthesizedExpression(it) => it.syntax(),
            Self::CssPercentage(it) => it.syntax(),
            Self::CssRatio(it) => it.syntax(),
            Self::CssString(it) => it.syntax(),
            Self::CssUnaryExpression(it) => it.syntax(),
            Self::AnyCssDimension(it) => it.syntax(),
            Self::AnyCssFunction(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBinaryExpression(it) => it.into_syntax(),
            Self::CssBogusPropertyValue(it) => it.into_syntax(),
            Self::CssColor(it) => it.into_syntax(),
            Self::CssDashedIdentifier(it) => it.into_syntax(),
            Self::CssIdentifier(it) => it.into_syntax(),
            Self::CssNumber(it) => it.into_syntax(),
            Self::CssParenthesizedExpression(it) => it.into_syntax(),
            Self::CssPercentage(it) => it.into_syntax(),
            Self::CssRatio(it) => it.into_syntax(),
            Self::CssString(it) => it.into_syntax(),
            Self::CssUnaryExpression(it) => it.into_syntax(),
            Self::AnyCssDimension(it) => it.into_syntax(),
            Self::AnyCssFunction(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyCssValue> for SyntaxNode {
    fn from(n: AnyCssValue) -> Self {
        match n {
            AnyCssValue::AnyCssDimension(it) => it.into_syntax(),
            AnyCssValue::AnyCssFunction(it) => it.into_syntax(),
            AnyCssValue::CssBinaryExpression(it) => it.into_syntax(),
            AnyCssValue::CssBogusPropertyValue(it) => it.into_syntax(),
            AnyCssValue::CssColor(it) => it.into_syntax(),
            AnyCssValue::CssDashedIdentifier(it) => it.into_syntax(),
            AnyCssValue::CssIdentifier(it) => it.into_syntax(),
            AnyCssValue::CssNumber(it) => it.into_syntax(),
            AnyCssValue::CssParenthesizedExpression(it) => it.into_syntax(),
            AnyCssValue::CssPercentage(it) => it.into_syntax(),
            AnyCssValue::CssRatio(it) => it.into_syntax(),
            AnyCssValue::CssString(it) => it.into_syntax(),
            AnyCssValue::CssUnaryExpression(it) => it.into_syntax(),
        }
    }
}
impl From<AnyCssValue> for SyntaxElement {
    fn from(n: AnyCssValue) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
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
            Self::TwArbitraryCandidate(it) => it.syntax(),
            Self::TwBogusCandidate(it) => it.syntax(),
            Self::TwFunctionalCandidate(it) => it.syntax(),
            Self::TwStaticCandidate(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::TwArbitraryCandidate(it) => it.into_syntax(),
            Self::TwBogusCandidate(it) => it.into_syntax(),
            Self::TwFunctionalCandidate(it) => it.into_syntax(),
            Self::TwStaticCandidate(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyTwCandidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyTwCandidate> for SyntaxNode {
    fn from(n: AnyTwCandidate) -> Self {
        match n {
            AnyTwCandidate::TwArbitraryCandidate(it) => it.into_syntax(),
            AnyTwCandidate::TwBogusCandidate(it) => it.into_syntax(),
            AnyTwCandidate::TwFunctionalCandidate(it) => it.into_syntax(),
            AnyTwCandidate::TwStaticCandidate(it) => it.into_syntax(),
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
            Self::TwBogusCandidate(it) => it.syntax(),
            Self::TwFullCandidate(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::TwBogusCandidate(it) => it.into_syntax(),
            Self::TwFullCandidate(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyTwFullCandidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyTwFullCandidate> for SyntaxNode {
    fn from(n: AnyTwFullCandidate) -> Self {
        match n {
            AnyTwFullCandidate::TwBogusCandidate(it) => it.into_syntax(),
            AnyTwFullCandidate::TwFullCandidate(it) => it.into_syntax(),
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
            Self::TwBogusModifier(it) => it.syntax(),
            Self::TwModifier(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::TwBogusModifier(it) => it.into_syntax(),
            Self::TwModifier(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyTwModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyTwModifier> for SyntaxNode {
    fn from(n: AnyTwModifier) -> Self {
        match n {
            AnyTwModifier::TwBogusModifier(it) => it.into_syntax(),
            AnyTwModifier::TwModifier(it) => it.into_syntax(),
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
impl From<TwNumberValue> for AnyTwValue {
    fn from(node: TwNumberValue) -> Self {
        Self::TwNumberValue(node)
    }
}
impl From<TwPercentageValue> for AnyTwValue {
    fn from(node: TwPercentageValue) -> Self {
        Self::TwPercentageValue(node)
    }
}
impl AstNode for AnyTwValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = TwArbitraryValue::KIND_SET
        .union(TwBogusValue::KIND_SET)
        .union(TwCssVariableValue::KIND_SET)
        .union(TwNamedValue::KIND_SET)
        .union(TwNumberValue::KIND_SET)
        .union(TwPercentageValue::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            TW_ARBITRARY_VALUE
                | TW_BOGUS_VALUE
                | TW_CSS_VARIABLE_VALUE
                | TW_NAMED_VALUE
                | TW_NUMBER_VALUE
                | TW_PERCENTAGE_VALUE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TW_ARBITRARY_VALUE => Self::TwArbitraryValue(TwArbitraryValue { syntax }),
            TW_BOGUS_VALUE => Self::TwBogusValue(TwBogusValue { syntax }),
            TW_CSS_VARIABLE_VALUE => Self::TwCssVariableValue(TwCssVariableValue { syntax }),
            TW_NAMED_VALUE => Self::TwNamedValue(TwNamedValue { syntax }),
            TW_NUMBER_VALUE => Self::TwNumberValue(TwNumberValue { syntax }),
            TW_PERCENTAGE_VALUE => Self::TwPercentageValue(TwPercentageValue { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::TwArbitraryValue(it) => it.syntax(),
            Self::TwBogusValue(it) => it.syntax(),
            Self::TwCssVariableValue(it) => it.syntax(),
            Self::TwNamedValue(it) => it.syntax(),
            Self::TwNumberValue(it) => it.syntax(),
            Self::TwPercentageValue(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::TwArbitraryValue(it) => it.into_syntax(),
            Self::TwBogusValue(it) => it.into_syntax(),
            Self::TwCssVariableValue(it) => it.into_syntax(),
            Self::TwNamedValue(it) => it.into_syntax(),
            Self::TwNumberValue(it) => it.into_syntax(),
            Self::TwPercentageValue(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyTwValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyTwValue> for SyntaxNode {
    fn from(n: AnyTwValue) -> Self {
        match n {
            AnyTwValue::TwArbitraryValue(it) => it.into_syntax(),
            AnyTwValue::TwBogusValue(it) => it.into_syntax(),
            AnyTwValue::TwCssVariableValue(it) => it.into_syntax(),
            AnyTwValue::TwNamedValue(it) => it.into_syntax(),
            AnyTwValue::TwNumberValue(it) => it.into_syntax(),
            AnyTwValue::TwPercentageValue(it) => it.into_syntax(),
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
impl From<TwVariantExpression> for AnyTwVariant {
    fn from(node: TwVariantExpression) -> Self {
        Self::TwVariantExpression(node)
    }
}
impl AstNode for AnyTwVariant {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = TwArbitraryVariant::KIND_SET
        .union(TwBogusVariant::KIND_SET)
        .union(TwVariantExpression::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            TW_ARBITRARY_VARIANT | TW_BOGUS_VARIANT | TW_VARIANT_EXPRESSION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TW_ARBITRARY_VARIANT => Self::TwArbitraryVariant(TwArbitraryVariant { syntax }),
            TW_BOGUS_VARIANT => Self::TwBogusVariant(TwBogusVariant { syntax }),
            TW_VARIANT_EXPRESSION => Self::TwVariantExpression(TwVariantExpression { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::TwArbitraryVariant(it) => it.syntax(),
            Self::TwBogusVariant(it) => it.syntax(),
            Self::TwVariantExpression(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::TwArbitraryVariant(it) => it.into_syntax(),
            Self::TwBogusVariant(it) => it.into_syntax(),
            Self::TwVariantExpression(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyTwVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyTwVariant> for SyntaxNode {
    fn from(n: AnyTwVariant) -> Self {
        match n {
            AnyTwVariant::TwArbitraryVariant(it) => it.into_syntax(),
            AnyTwVariant::TwBogusVariant(it) => it.into_syntax(),
            AnyTwVariant::TwVariantExpression(it) => it.into_syntax(),
        }
    }
}
impl From<AnyTwVariant> for SyntaxElement {
    fn from(n: AnyTwVariant) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TwArbitraryVariantSegment> for AnyTwVariantSegment {
    fn from(node: TwArbitraryVariantSegment) -> Self {
        Self::TwArbitraryVariantSegment(node)
    }
}
impl From<TwBogusVariantSegment> for AnyTwVariantSegment {
    fn from(node: TwBogusVariantSegment) -> Self {
        Self::TwBogusVariantSegment(node)
    }
}
impl From<TwCssVariableVariantSegment> for AnyTwVariantSegment {
    fn from(node: TwCssVariableVariantSegment) -> Self {
        Self::TwCssVariableVariantSegment(node)
    }
}
impl From<TwNamedVariantSegment> for AnyTwVariantSegment {
    fn from(node: TwNamedVariantSegment) -> Self {
        Self::TwNamedVariantSegment(node)
    }
}
impl AstNode for AnyTwVariantSegment {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = TwArbitraryVariantSegment::KIND_SET
        .union(TwBogusVariantSegment::KIND_SET)
        .union(TwCssVariableVariantSegment::KIND_SET)
        .union(TwNamedVariantSegment::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            TW_ARBITRARY_VARIANT_SEGMENT
                | TW_BOGUS_VARIANT_SEGMENT
                | TW_CSS_VARIABLE_VARIANT_SEGMENT
                | TW_NAMED_VARIANT_SEGMENT
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TW_ARBITRARY_VARIANT_SEGMENT => {
                Self::TwArbitraryVariantSegment(TwArbitraryVariantSegment { syntax })
            }
            TW_BOGUS_VARIANT_SEGMENT => {
                Self::TwBogusVariantSegment(TwBogusVariantSegment { syntax })
            }
            TW_CSS_VARIABLE_VARIANT_SEGMENT => {
                Self::TwCssVariableVariantSegment(TwCssVariableVariantSegment { syntax })
            }
            TW_NAMED_VARIANT_SEGMENT => {
                Self::TwNamedVariantSegment(TwNamedVariantSegment { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::TwArbitraryVariantSegment(it) => it.syntax(),
            Self::TwBogusVariantSegment(it) => it.syntax(),
            Self::TwCssVariableVariantSegment(it) => it.syntax(),
            Self::TwNamedVariantSegment(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::TwArbitraryVariantSegment(it) => it.into_syntax(),
            Self::TwBogusVariantSegment(it) => it.into_syntax(),
            Self::TwCssVariableVariantSegment(it) => it.into_syntax(),
            Self::TwNamedVariantSegment(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyTwVariantSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax(), f)
    }
}
impl From<AnyTwVariantSegment> for SyntaxNode {
    fn from(n: AnyTwVariantSegment) -> Self {
        match n {
            AnyTwVariantSegment::TwArbitraryVariantSegment(it) => it.into_syntax(),
            AnyTwVariantSegment::TwBogusVariantSegment(it) => it.into_syntax(),
            AnyTwVariantSegment::TwCssVariableVariantSegment(it) => it.into_syntax(),
            AnyTwVariantSegment::TwNamedVariantSegment(it) => it.into_syntax(),
        }
    }
}
impl From<AnyTwVariantSegment> for SyntaxElement {
    fn from(n: AnyTwVariantSegment) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyCssDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssGenericComponentValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssUrlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
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
impl std::fmt::Display for AnyTwVariantSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssBinaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssDashedIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssGenericDelimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssListOfComponentValuesExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssParenthesizedExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPercentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssRatio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssRegularDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssUnaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssUnknownDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssUrlFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssUrlValueRaw {
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
impl std::fmt::Display for TwArbitraryVariantSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TwCssVariableValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TwCssVariableVariantSegment {
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
impl std::fmt::Display for TwNamedVariantSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TwNumberValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TwPercentageValue {
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
impl std::fmt::Display for TwTypeHint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TwVariantExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct CssBogusPropertyValue {
    syntax: SyntaxNode,
}
impl CssBogusPropertyValue {
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
impl AstNode for CssBogusPropertyValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_PROPERTY_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_PROPERTY_VALUE
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
impl std::fmt::Debug for CssBogusPropertyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<CssBogusPropertyValue> for SyntaxNode {
    fn from(n: CssBogusPropertyValue) -> Self {
        n.syntax
    }
}
impl From<CssBogusPropertyValue> for SyntaxElement {
    fn from(n: CssBogusPropertyValue) -> Self {
        n.syntax.into()
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
        debug_syntax_node(&self.syntax, f)
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
        debug_syntax_node(&self.syntax, f)
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
        debug_syntax_node(&self.syntax, f)
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
        debug_syntax_node(&self.syntax, f)
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
        debug_syntax_node(&self.syntax, f)
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
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct TwBogusVariantSegment {
    syntax: SyntaxNode,
}
impl TwBogusVariantSegment {
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
impl AstNode for TwBogusVariantSegment {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_BOGUS_VARIANT_SEGMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_BOGUS_VARIANT_SEGMENT
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
impl std::fmt::Debug for TwBogusVariantSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(&self.syntax, f)
    }
}
impl From<TwBogusVariantSegment> for SyntaxNode {
    fn from(n: TwBogusVariantSegment) -> Self {
        n.syntax
    }
}
impl From<TwBogusVariantSegment> for SyntaxElement {
    fn from(n: TwBogusVariantSegment) -> Self {
        n.syntax.into()
    }
}
biome_rowan::declare_node_union! { pub AnyTwBogusNode = CssBogusPropertyValue | TwBogus | TwBogusCandidate | TwBogusModifier | TwBogusValue | TwBogusVariant | TwBogusVariantSegment }
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssComponentValueList {
    syntax_list: SyntaxList,
}
impl CssComponentValueList {
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
impl AstNode for CssComponentValueList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_COMPONENT_VALUE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_COMPONENT_VALUE_LIST
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
impl Serialize for CssComponentValueList {
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
impl AstNodeList for CssComponentValueList {
    type Language = Language;
    type Node = AnyCssValue;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssComponentValueList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax_list.node(), f)
    }
}
impl IntoIterator for &CssComponentValueList {
    type Item = AnyCssValue;
    type IntoIter = AstNodeListIterator<Language, AnyCssValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssComponentValueList {
    type Item = AnyCssValue;
    type IntoIter = AstNodeListIterator<Language, AnyCssValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssGenericComponentValueList {
    syntax_list: SyntaxList,
}
impl CssGenericComponentValueList {
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
impl AstNode for CssGenericComponentValueList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_GENERIC_COMPONENT_VALUE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_GENERIC_COMPONENT_VALUE_LIST
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
impl Serialize for CssGenericComponentValueList {
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
impl AstNodeList for CssGenericComponentValueList {
    type Language = Language;
    type Node = AnyCssGenericComponentValue;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssGenericComponentValueList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax_list.node(), f)
    }
}
impl IntoIterator for &CssGenericComponentValueList {
    type Item = AnyCssGenericComponentValue;
    type IntoIter = AstNodeListIterator<Language, AnyCssGenericComponentValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssGenericComponentValueList {
    type Item = AnyCssGenericComponentValue;
    type IntoIter = AstNodeListIterator<Language, AnyCssGenericComponentValue>;
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
impl AstSeparatedList for CssParameterList {
    type Language = Language;
    type Node = AnyCssExpression;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssParameterList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax_list.node(), f)
    }
}
impl IntoIterator for CssParameterList {
    type Item = SyntaxResult<AnyCssExpression>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssExpression>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssParameterList {
    type Item = SyntaxResult<AnyCssExpression>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssExpression>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
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
        debug_syntax_node(self.syntax_list.node(), f)
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
        debug_syntax_node(self.syntax_list.node(), f)
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
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TwVariantSegmentList {
    syntax_list: SyntaxList,
}
impl TwVariantSegmentList {
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
impl AstNode for TwVariantSegmentList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TW_VARIANT_SEGMENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TW_VARIANT_SEGMENT_LIST
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
impl Serialize for TwVariantSegmentList {
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
impl AstSeparatedList for TwVariantSegmentList {
    type Language = Language;
    type Node = AnyTwVariantSegment;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for TwVariantSegmentList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_syntax_node(self.syntax_list.node(), f)
    }
}
impl IntoIterator for TwVariantSegmentList {
    type Item = SyntaxResult<AnyTwVariantSegment>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyTwVariantSegment>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &TwVariantSegmentList {
    type Item = SyntaxResult<AnyTwVariantSegment>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyTwVariantSegment>;
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
        "nCssBinaryExpression,left,operator,right\n",
        "nCssColor,value_token\n",
        "nCssDashedIdentifier,ident_token\n",
        "nCssFunction,name,l_paren_token,parameters,r_paren_token\n",
        "nCssGenericDelimiter,value\n",
        "nCssIdentifier,ident_token\n",
        "nCssListOfComponentValuesExpression,css_component_value_list\n",
        "nCssNumber,value_token\n",
        "nCssParenthesizedExpression,l_paren_token,?expression,r_paren_token\n",
        "nCssPercentage,value_token,remainder_token\n",
        "nCssRatio,left,slash_token,right\n",
        "nCssRegularDimension,value_token,unit_token\n",
        "nCssString,value_token\n",
        "nCssUnaryExpression,operator,argument\n",
        "nCssUnknownDimension,value_token,unit_token\n",
        "nCssUrlFunction,url_token,l_paren_token,?value,r_paren_token\n",
        "nCssUrlValueRaw,value_token\n",
        "nTwArbitraryCandidate,l_brack_token,property_token,colon_token,value,r_brack_token,?modifier\n",
        "nTwArbitraryValue,l_brack_token,value,r_brack_token\n",
        "nTwArbitraryVariant,l_brack_token,selector_token,r_brack_token\n",
        "nTwArbitraryVariantSegment,l_brack_token,value_token,r_brack_token\n",
        "nTwCssVariableValue,l_paren_token,?type_hint,value,r_paren_token\n",
        "nTwCssVariableVariantSegment,l_paren_token,value,r_paren_token\n",
        "nTwFullCandidate,variants,?legacy_important_token,?negative_token,candidate,?excl_token\n",
        "nTwFunctionalCandidate,base_token,minus_token,value,?modifier\n",
        "nTwModifier,slash_token,value\n",
        "nTwNamedValue,value_token\n",
        "nTwNamedVariantSegment,value_token\n",
        "nTwNumberValue,value_token\n",
        "nTwPercentageValue,value_token,remainder_token\n",
        "nTwRoot,?bom_token,?leading_whitespace_token,candidates,eof_token\n",
        "nTwStaticCandidate,base_token,?modifier\n",
        "nTwTypeHint,name_token,colon_token\n",
        "nTwVariantExpression,segments,?glued_value,?modifier\n",
        "bCssBogusPropertyValue\n",
        "bTwBogus\n",
        "bTwBogusCandidate\n",
        "bTwBogusModifier\n",
        "bTwBogusValue\n",
        "bTwBogusVariant\n",
        "bTwBogusVariantSegment\n",
        "lCssComponentValueList\n",
        "lCssGenericComponentValueList\n",
        "sCssParameterList\n",
        "sTwCandidateList\n",
        "sTwVariantList\n",
        "sTwVariantSegmentList\n"
    );
    fn shape_offset(kind: SyntaxKind) -> u32 {
        match kind {
            CSS_BINARY_EXPRESSION => 0u32,
            CSS_COLOR => 41u32,
            CSS_DASHED_IDENTIFIER => 63u32,
            CSS_FUNCTION => 96u32,
            CSS_GENERIC_DELIMITER => 153u32,
            CSS_IDENTIFIER => 180u32,
            CSS_LIST_OF_COMPONENT_VALUES_EXPRESSION => 207u32,
            CSS_NUMBER => 268u32,
            CSS_PARENTHESIZED_EXPRESSION => 291u32,
            CSS_PERCENTAGE => 359u32,
            CSS_RATIO => 402u32,
            CSS_REGULAR_DIMENSION => 435u32,
            CSS_STRING => 479u32,
            CSS_UNARY_EXPRESSION => 502u32,
            CSS_UNKNOWN_DIMENSION => 540u32,
            CSS_URL_FUNCTION => 584u32,
            CSS_URL_VALUE_RAW => 645u32,
            TW_ARBITRARY_CANDIDATE => 673u32,
            TW_ARBITRARY_VALUE => 766u32,
            TW_ARBITRARY_VARIANT => 818u32,
            TW_ARBITRARY_VARIANT_SEGMENT => 881u32,
            TW_CSS_VARIABLE_VALUE => 948u32,
            TW_CSS_VARIABLE_VARIANT_SEGMENT => 1013u32,
            TW_FULL_CANDIDATE => 1076u32,
            TW_FUNCTIONAL_CANDIDATE => 1164u32,
            TW_MODIFIER => 1226u32,
            TW_NAMED_VALUE => 1256u32,
            TW_NAMED_VARIANT_SEGMENT => 1282u32,
            TW_NUMBER_VALUE => 1317u32,
            TW_PERCENTAGE_VALUE => 1344u32,
            TW_ROOT => 1391u32,
            TW_STATIC_CANDIDATE => 1457u32,
            TW_TYPE_HINT => 1497u32,
            TW_VARIANT_EXPRESSION => 1532u32,
            CSS_BOGUS_PROPERTY_VALUE => 1585u32,
            TW_BOGUS => 1608u32,
            TW_BOGUS_CANDIDATE => 1617u32,
            TW_BOGUS_MODIFIER => 1635u32,
            TW_BOGUS_VALUE => 1652u32,
            TW_BOGUS_VARIANT => 1666u32,
            TW_BOGUS_VARIANT_SEGMENT => 1682u32,
            CSS_COMPONENT_VALUE_LIST => 1705u32,
            CSS_GENERIC_COMPONENT_VALUE_LIST => 1728u32,
            CSS_PARAMETER_LIST => 1758u32,
            TW_CANDIDATE_LIST => 1776u32,
            TW_VARIANT_LIST => 1793u32,
            TW_VARIANT_SEGMENT_LIST => 1808u32,
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
