//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(dead_code)]
#![allow(unused)]
use crate::{
    CssLanguage as Language, CssSyntaxElement as SyntaxElement,
    CssSyntaxElementChildren as SyntaxElementChildren,
    CssSyntaxKind::{self as SyntaxKind, *},
    CssSyntaxList as SyntaxList, CssSyntaxNode as SyntaxNode, CssSyntaxToken as SyntaxToken,
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
pub struct CssAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssAtRuleFields {
        CssAtRuleFields {
            at_token: self.at_token(),
            rule: self.rule(),
        }
    }
    pub fn at_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn rule(&self) -> SyntaxResult<AnyCssAtRule> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for CssAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssAtRuleFields {
    pub at_token: SyntaxResult<SyntaxToken>,
    pub rule: SyntaxResult<AnyCssAtRule>,
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
impl Serialize for CssAttributeMatcher {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
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
impl Serialize for CssAttributeMatcherValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssAttributeMatcherValueFields {
    pub name: SyntaxResult<AnyCssAttributeMatcherValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAttributeName {
    pub(crate) syntax: SyntaxNode,
}
impl CssAttributeName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssAttributeNameFields {
        CssAttributeNameFields {
            namespace: self.namespace(),
            name: self.name(),
        }
    }
    pub fn namespace(&self) -> Option<CssNamespace> {
        support::node(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for CssAttributeName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssAttributeNameFields {
    pub namespace: Option<CssNamespace>,
    pub name: SyntaxResult<CssIdentifier>,
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
    pub fn name(&self) -> SyntaxResult<CssAttributeName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn matcher(&self) -> Option<CssAttributeMatcher> {
        support::node(&self.syntax, 2usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for CssAttributeSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssAttributeSelectorFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<CssAttributeName>,
    pub matcher: Option<CssAttributeMatcher>,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
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
            operator_token: self.operator_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyCssExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn operator_token(&self) -> SyntaxResult<SyntaxToken> {
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
    pub operator_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyCssExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssBracketedValue {
    pub(crate) syntax: SyntaxNode,
}
impl CssBracketedValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssBracketedValueFields {
        CssBracketedValueFields {
            l_brack_token: self.l_brack_token(),
            items: self.items(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> CssBracketedValueList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for CssBracketedValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssBracketedValueFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub items: CssBracketedValueList,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssCharsetAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssCharsetAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssCharsetAtRuleFields {
        CssCharsetAtRuleFields {
            charset_token: self.charset_token(),
            encoding: self.encoding(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn charset_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn encoding(&self) -> SyntaxResult<CssString> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn semicolon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for CssCharsetAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssCharsetAtRuleFields {
    pub charset_token: SyntaxResult<SyntaxToken>,
    pub encoding: SyntaxResult<CssString>,
    pub semicolon_token: SyntaxResult<SyntaxToken>,
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
    pub fn name(&self) -> SyntaxResult<CssCustomIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for CssClassSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssClassSelectorFields {
    pub dot_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<CssCustomIdentifier>,
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
            hash_token: self.hash_token(),
            value_token: self.value_token(),
        }
    }
    pub fn hash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
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
    pub hash_token: SyntaxResult<SyntaxToken>,
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssColorProfileAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssColorProfileAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssColorProfileAtRuleFields {
        CssColorProfileAtRuleFields {
            color_profile_token: self.color_profile_token(),
            name: self.name(),
            block: self.block(),
        }
    }
    pub fn color_profile_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<CssCustomIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssDeclarationBlock> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssColorProfileAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssColorProfileAtRuleFields {
    pub color_profile_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<CssCustomIdentifier>,
    pub block: SyntaxResult<AnyCssDeclarationBlock>,
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
impl Serialize for CssComplexSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssComplexSelectorFields {
    pub left: SyntaxResult<AnyCssSelector>,
    pub combinator: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyCssSelector>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssComposesImportSpecifier {
    pub(crate) syntax: SyntaxNode,
}
impl CssComposesImportSpecifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssComposesImportSpecifierFields {
        CssComposesImportSpecifierFields {
            from_token: self.from_token(),
            source: self.source(),
        }
    }
    pub fn from_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn source(&self) -> SyntaxResult<AnyCssComposesImportSource> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for CssComposesImportSpecifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssComposesImportSpecifierFields {
    pub from_token: SyntaxResult<SyntaxToken>,
    pub source: SyntaxResult<AnyCssComposesImportSource>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssComposesProperty {
    pub(crate) syntax: SyntaxNode,
}
impl CssComposesProperty {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssComposesPropertyFields {
        CssComposesPropertyFields {
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
    pub fn value(&self) -> SyntaxResult<CssComposesPropertyValue> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssComposesProperty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssComposesPropertyFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<CssComposesPropertyValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssComposesPropertyValue {
    pub(crate) syntax: SyntaxNode,
}
impl CssComposesPropertyValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssComposesPropertyValueFields {
        CssComposesPropertyValueFields {
            classes: self.classes(),
            specifier: self.specifier(),
        }
    }
    pub fn classes(&self) -> CssComposesClassList {
        support::list(&self.syntax, 0usize)
    }
    pub fn specifier(&self) -> Option<CssComposesImportSpecifier> {
        support::node(&self.syntax, 1usize)
    }
}
impl Serialize for CssComposesPropertyValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssComposesPropertyValueFields {
    pub classes: CssComposesClassList,
    pub specifier: Option<CssComposesImportSpecifier>,
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
            nesting_selectors: self.nesting_selectors(),
            simple_selector: self.simple_selector(),
            sub_selectors: self.sub_selectors(),
        }
    }
    pub fn nesting_selectors(&self) -> CssNestedSelectorList {
        support::list(&self.syntax, 0usize)
    }
    pub fn simple_selector(&self) -> Option<AnyCssSimpleSelector> {
        support::node(&self.syntax, 1usize)
    }
    pub fn sub_selectors(&self) -> CssSubSelectorList {
        support::list(&self.syntax, 2usize)
    }
}
impl Serialize for CssCompoundSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssCompoundSelectorFields {
    pub nesting_selectors: CssNestedSelectorList,
    pub simple_selector: Option<AnyCssSimpleSelector>,
    pub sub_selectors: CssSubSelectorList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssContainerAndQuery {
    pub(crate) syntax: SyntaxNode,
}
impl CssContainerAndQuery {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssContainerAndQueryFields {
        CssContainerAndQueryFields {
            left: self.left(),
            and_token: self.and_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyCssContainerQueryInParens> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn and_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyCssContainerAndCombinableQuery> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssContainerAndQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssContainerAndQueryFields {
    pub left: SyntaxResult<AnyCssContainerQueryInParens>,
    pub and_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyCssContainerAndCombinableQuery>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssContainerAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssContainerAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssContainerAtRuleFields {
        CssContainerAtRuleFields {
            container_token: self.container_token(),
            name: self.name(),
            query: self.query(),
            block: self.block(),
        }
    }
    pub fn container_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> Option<CssCustomIdentifier> {
        support::node(&self.syntax, 1usize)
    }
    pub fn query(&self) -> SyntaxResult<AnyCssContainerQuery> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssConditionalBlock> {
        support::required_node(&self.syntax, 3usize)
    }
}
impl Serialize for CssContainerAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssContainerAtRuleFields {
    pub container_token: SyntaxResult<SyntaxToken>,
    pub name: Option<CssCustomIdentifier>,
    pub query: SyntaxResult<AnyCssContainerQuery>,
    pub block: SyntaxResult<AnyCssConditionalBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssContainerNotQuery {
    pub(crate) syntax: SyntaxNode,
}
impl CssContainerNotQuery {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssContainerNotQueryFields {
        CssContainerNotQueryFields {
            not_token: self.not_token(),
            query: self.query(),
        }
    }
    pub fn not_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn query(&self) -> SyntaxResult<AnyCssContainerQueryInParens> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for CssContainerNotQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssContainerNotQueryFields {
    pub not_token: SyntaxResult<SyntaxToken>,
    pub query: SyntaxResult<AnyCssContainerQueryInParens>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssContainerOrQuery {
    pub(crate) syntax: SyntaxNode,
}
impl CssContainerOrQuery {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssContainerOrQueryFields {
        CssContainerOrQueryFields {
            left: self.left(),
            or_token: self.or_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyCssContainerQueryInParens> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn or_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyCssContainerOrCombinableQuery> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssContainerOrQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssContainerOrQueryFields {
    pub left: SyntaxResult<AnyCssContainerQueryInParens>,
    pub or_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyCssContainerOrCombinableQuery>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssContainerQueryInParens {
    pub(crate) syntax: SyntaxNode,
}
impl CssContainerQueryInParens {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssContainerQueryInParensFields {
        CssContainerQueryInParensFields {
            l_paren_token: self.l_paren_token(),
            query: self.query(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn query(&self) -> SyntaxResult<AnyCssContainerQuery> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for CssContainerQueryInParens {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssContainerQueryInParensFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub query: SyntaxResult<AnyCssContainerQuery>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssContainerSizeFeatureInParens {
    pub(crate) syntax: SyntaxNode,
}
impl CssContainerSizeFeatureInParens {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssContainerSizeFeatureInParensFields {
        CssContainerSizeFeatureInParensFields {
            l_paren_token: self.l_paren_token(),
            feature: self.feature(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn feature(&self) -> SyntaxResult<AnyCssQueryFeature> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for CssContainerSizeFeatureInParens {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssContainerSizeFeatureInParensFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub feature: SyntaxResult<AnyCssQueryFeature>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssContainerStyleAndQuery {
    pub(crate) syntax: SyntaxNode,
}
impl CssContainerStyleAndQuery {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssContainerStyleAndQueryFields {
        CssContainerStyleAndQueryFields {
            left: self.left(),
            and_token: self.and_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<CssContainerStyleInParens> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn and_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyCssContainerStyleAndCombinableQuery> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssContainerStyleAndQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssContainerStyleAndQueryFields {
    pub left: SyntaxResult<CssContainerStyleInParens>,
    pub and_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyCssContainerStyleAndCombinableQuery>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssContainerStyleInParens {
    pub(crate) syntax: SyntaxNode,
}
impl CssContainerStyleInParens {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssContainerStyleInParensFields {
        CssContainerStyleInParensFields {
            l_paren_token: self.l_paren_token(),
            query: self.query(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn query(&self) -> SyntaxResult<AnyCssContainerStyleInParens> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for CssContainerStyleInParens {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssContainerStyleInParensFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub query: SyntaxResult<AnyCssContainerStyleInParens>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssContainerStyleNotQuery {
    pub(crate) syntax: SyntaxNode,
}
impl CssContainerStyleNotQuery {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssContainerStyleNotQueryFields {
        CssContainerStyleNotQueryFields {
            not_token: self.not_token(),
            query: self.query(),
        }
    }
    pub fn not_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn query(&self) -> SyntaxResult<CssContainerStyleInParens> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for CssContainerStyleNotQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssContainerStyleNotQueryFields {
    pub not_token: SyntaxResult<SyntaxToken>,
    pub query: SyntaxResult<CssContainerStyleInParens>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssContainerStyleOrQuery {
    pub(crate) syntax: SyntaxNode,
}
impl CssContainerStyleOrQuery {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssContainerStyleOrQueryFields {
        CssContainerStyleOrQueryFields {
            left: self.left(),
            or_token: self.or_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<CssContainerStyleInParens> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn or_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyCssContainerStyleOrCombinableQuery> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssContainerStyleOrQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssContainerStyleOrQueryFields {
    pub left: SyntaxResult<CssContainerStyleInParens>,
    pub or_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyCssContainerStyleOrCombinableQuery>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssContainerStyleQueryInParens {
    pub(crate) syntax: SyntaxNode,
}
impl CssContainerStyleQueryInParens {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssContainerStyleQueryInParensFields {
        CssContainerStyleQueryInParensFields {
            style_token: self.style_token(),
            l_paren_token: self.l_paren_token(),
            query: self.query(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn style_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn query(&self) -> SyntaxResult<AnyCssContainerStyleQuery> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for CssContainerStyleQueryInParens {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssContainerStyleQueryInParensFields {
    pub style_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub query: SyntaxResult<AnyCssContainerStyleQuery>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssCounterStyleAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssCounterStyleAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssCounterStyleAtRuleFields {
        CssCounterStyleAtRuleFields {
            counter_style_token: self.counter_style_token(),
            name: self.name(),
            block: self.block(),
        }
    }
    pub fn counter_style_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<CssCustomIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssDeclarationBlock> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssCounterStyleAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssCounterStyleAtRuleFields {
    pub counter_style_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<CssCustomIdentifier>,
    pub block: SyntaxResult<AnyCssDeclarationBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssCustomIdentifier {
    pub(crate) syntax: SyntaxNode,
}
impl CssCustomIdentifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssCustomIdentifierFields {
        CssCustomIdentifierFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for CssCustomIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssCustomIdentifierFields {
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
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
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
            property: self.property(),
            important: self.important(),
        }
    }
    pub fn property(&self) -> SyntaxResult<AnyCssProperty> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn important(&self) -> Option<CssDeclarationImportant> {
        support::node(&self.syntax, 1usize)
    }
}
impl Serialize for CssDeclaration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssDeclarationFields {
    pub property: SyntaxResult<AnyCssProperty>,
    pub important: Option<CssDeclarationImportant>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssDeclarationBlock {
    pub(crate) syntax: SyntaxNode,
}
impl CssDeclarationBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssDeclarationBlockFields {
        CssDeclarationBlockFields {
            l_curly_token: self.l_curly_token(),
            declarations: self.declarations(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn declarations(&self) -> CssDeclarationList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for CssDeclarationBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssDeclarationBlockFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub declarations: CssDeclarationList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
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
impl Serialize for CssDeclarationImportant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssDeclarationImportantFields {
    pub excl_token: SyntaxResult<SyntaxToken>,
    pub important_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssDeclarationOrAtRuleBlock {
    pub(crate) syntax: SyntaxNode,
}
impl CssDeclarationOrAtRuleBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssDeclarationOrAtRuleBlockFields {
        CssDeclarationOrAtRuleBlockFields {
            l_curly_token: self.l_curly_token(),
            items: self.items(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> CssDeclarationOrAtRuleList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for CssDeclarationOrAtRuleBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssDeclarationOrAtRuleBlockFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub items: CssDeclarationOrAtRuleList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssDeclarationOrRuleBlock {
    pub(crate) syntax: SyntaxNode,
}
impl CssDeclarationOrRuleBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssDeclarationOrRuleBlockFields {
        CssDeclarationOrRuleBlockFields {
            l_curly_token: self.l_curly_token(),
            items: self.items(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> CssDeclarationOrRuleList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for CssDeclarationOrRuleBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssDeclarationOrRuleBlockFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub items: CssDeclarationOrRuleList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssDeclarationWithSemicolon {
    pub(crate) syntax: SyntaxNode,
}
impl CssDeclarationWithSemicolon {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssDeclarationWithSemicolonFields {
        CssDeclarationWithSemicolonFields {
            declaration: self.declaration(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn declaration(&self) -> SyntaxResult<CssDeclaration> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
}
impl Serialize for CssDeclarationWithSemicolon {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssDeclarationWithSemicolonFields {
    pub declaration: SyntaxResult<CssDeclaration>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssDocumentAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssDocumentAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssDocumentAtRuleFields {
        CssDocumentAtRuleFields {
            document_token: self.document_token(),
            matchers: self.matchers(),
            block: self.block(),
        }
    }
    pub fn document_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn matchers(&self) -> CssDocumentMatcherList {
        support::list(&self.syntax, 1usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssRuleBlock> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssDocumentAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssDocumentAtRuleFields {
    pub document_token: SyntaxResult<SyntaxToken>,
    pub matchers: CssDocumentMatcherList,
    pub block: SyntaxResult<AnyCssRuleBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssDocumentCustomMatcher {
    pub(crate) syntax: SyntaxNode,
}
impl CssDocumentCustomMatcher {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssDocumentCustomMatcherFields {
        CssDocumentCustomMatcherFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            value: self.value(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<SyntaxToken> {
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
impl Serialize for CssDocumentCustomMatcher {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssDocumentCustomMatcherFields {
    pub name: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub value: Option<AnyCssUrlValue>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssEmptyDeclaration {
    pub(crate) syntax: SyntaxNode,
}
impl CssEmptyDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssEmptyDeclarationFields {
        CssEmptyDeclarationFields {
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn semicolon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for CssEmptyDeclaration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssEmptyDeclarationFields {
    pub semicolon_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssFontFaceAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssFontFaceAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssFontFaceAtRuleFields {
        CssFontFaceAtRuleFields {
            font_face_token: self.font_face_token(),
            block: self.block(),
        }
    }
    pub fn font_face_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssDeclarationBlock> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for CssFontFaceAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssFontFaceAtRuleFields {
    pub font_face_token: SyntaxResult<SyntaxToken>,
    pub block: SyntaxResult<AnyCssDeclarationBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssFontFamilyName {
    pub(crate) syntax: SyntaxNode,
}
impl CssFontFamilyName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssFontFamilyNameFields {
        CssFontFamilyNameFields {
            names: self.names(),
        }
    }
    pub fn names(&self) -> CssCustomIdentifierList {
        support::list(&self.syntax, 0usize)
    }
}
impl Serialize for CssFontFamilyName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssFontFamilyNameFields {
    pub names: CssCustomIdentifierList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssFontFeatureValuesAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssFontFeatureValuesAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssFontFeatureValuesAtRuleFields {
        CssFontFeatureValuesAtRuleFields {
            font_feature_values_token: self.font_feature_values_token(),
            names: self.names(),
            block: self.block(),
        }
    }
    pub fn font_feature_values_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn names(&self) -> CssFontFamilyNameList {
        support::list(&self.syntax, 1usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssFontFeatureValuesBlock> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssFontFeatureValuesAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssFontFeatureValuesAtRuleFields {
    pub font_feature_values_token: SyntaxResult<SyntaxToken>,
    pub names: CssFontFamilyNameList,
    pub block: SyntaxResult<AnyCssFontFeatureValuesBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssFontFeatureValuesBlock {
    pub(crate) syntax: SyntaxNode,
}
impl CssFontFeatureValuesBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssFontFeatureValuesBlockFields {
        CssFontFeatureValuesBlockFields {
            l_curly_token: self.l_curly_token(),
            items: self.items(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> CssFontFeatureValuesItemList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for CssFontFeatureValuesBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssFontFeatureValuesBlockFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub items: CssFontFeatureValuesItemList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssFontFeatureValuesItem {
    pub(crate) syntax: SyntaxNode,
}
impl CssFontFeatureValuesItem {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssFontFeatureValuesItemFields {
        CssFontFeatureValuesItemFields {
            at_token: self.at_token(),
            name: self.name(),
            block: self.block(),
        }
    }
    pub fn at_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssDeclarationBlock> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssFontFeatureValuesItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssFontFeatureValuesItemFields {
    pub at_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<SyntaxToken>,
    pub block: SyntaxResult<AnyCssDeclarationBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssFontPaletteValuesAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssFontPaletteValuesAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssFontPaletteValuesAtRuleFields {
        CssFontPaletteValuesAtRuleFields {
            font_palette_values_token: self.font_palette_values_token(),
            name: self.name(),
            block: self.block(),
        }
    }
    pub fn font_palette_values_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<CssDashedIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssDeclarationBlock> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssFontPaletteValuesAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssFontPaletteValuesAtRuleFields {
    pub font_palette_values_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<CssDashedIdentifier>,
    pub block: SyntaxResult<AnyCssDeclarationBlock>,
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
    pub items: CssParameterList,
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
pub struct CssGenericProperty {
    pub(crate) syntax: SyntaxNode,
}
impl CssGenericProperty {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssGenericPropertyFields {
        CssGenericPropertyFields {
            name: self.name(),
            colon_token: self.colon_token(),
            value: self.value(),
        }
    }
    pub fn name(&self) -> SyntaxResult<AnyCssDeclarationName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> CssGenericComponentValueList {
        support::list(&self.syntax, 2usize)
    }
}
impl Serialize for CssGenericProperty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssGenericPropertyFields {
    pub name: SyntaxResult<AnyCssDeclarationName>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: CssGenericComponentValueList,
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
    pub fn name(&self) -> SyntaxResult<CssCustomIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for CssIdSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssIdSelectorFields {
    pub hash_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<CssCustomIdentifier>,
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
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssImportAnonymousLayer {
    pub(crate) syntax: SyntaxNode,
}
impl CssImportAnonymousLayer {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssImportAnonymousLayerFields {
        CssImportAnonymousLayerFields {
            layer_token: self.layer_token(),
        }
    }
    pub fn layer_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for CssImportAnonymousLayer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssImportAnonymousLayerFields {
    pub layer_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssImportAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssImportAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssImportAtRuleFields {
        CssImportAtRuleFields {
            import_token: self.import_token(),
            url: self.url(),
            layer: self.layer(),
            supports: self.supports(),
            media: self.media(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn import_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn url(&self) -> SyntaxResult<AnyCssImportUrl> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn layer(&self) -> Option<AnyCssImportLayer> {
        support::node(&self.syntax, 2usize)
    }
    pub fn supports(&self) -> Option<CssImportSupports> {
        support::node(&self.syntax, 3usize)
    }
    pub fn media(&self) -> CssMediaQueryList {
        support::list(&self.syntax, 4usize)
    }
    pub fn semicolon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
}
impl Serialize for CssImportAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssImportAtRuleFields {
    pub import_token: SyntaxResult<SyntaxToken>,
    pub url: SyntaxResult<AnyCssImportUrl>,
    pub layer: Option<AnyCssImportLayer>,
    pub supports: Option<CssImportSupports>,
    pub media: CssMediaQueryList,
    pub semicolon_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssImportNamedLayer {
    pub(crate) syntax: SyntaxNode,
}
impl CssImportNamedLayer {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssImportNamedLayerFields {
        CssImportNamedLayerFields {
            layer_token: self.layer_token(),
            l_paren_token: self.l_paren_token(),
            name: self.name(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn layer_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> CssLayerNameList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for CssImportNamedLayer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssImportNamedLayerFields {
    pub layer_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub name: CssLayerNameList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssImportSupports {
    pub(crate) syntax: SyntaxNode,
}
impl CssImportSupports {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssImportSupportsFields {
        CssImportSupportsFields {
            supports_token: self.supports_token(),
            l_paren_token: self.l_paren_token(),
            condition: self.condition(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn supports_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn condition(&self) -> SyntaxResult<AnyCssImportSupportsCondition> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for CssImportSupports {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssImportSupportsFields {
    pub supports_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub condition: SyntaxResult<AnyCssImportSupportsCondition>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssKeyframesAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssKeyframesAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssKeyframesAtRuleFields {
        CssKeyframesAtRuleFields {
            keyframes_token: self.keyframes_token(),
            name: self.name(),
            block: self.block(),
        }
    }
    pub fn keyframes_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<AnyCssKeyframesName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssKeyframesBlock> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssKeyframesAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssKeyframesAtRuleFields {
    pub keyframes_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<AnyCssKeyframesName>,
    pub block: SyntaxResult<AnyCssKeyframesBlock>,
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
            l_curly_token: self.l_curly_token(),
            items: self.items(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> CssKeyframesItemList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for CssKeyframesBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssKeyframesBlockFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub items: CssKeyframesItemList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssKeyframesIdentSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssKeyframesIdentSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssKeyframesIdentSelectorFields {
        CssKeyframesIdentSelectorFields {
            selector: self.selector(),
        }
    }
    pub fn selector(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for CssKeyframesIdentSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssKeyframesIdentSelectorFields {
    pub selector: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssKeyframesItem {
    pub(crate) syntax: SyntaxNode,
}
impl CssKeyframesItem {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssKeyframesItemFields {
        CssKeyframesItemFields {
            selectors: self.selectors(),
            block: self.block(),
        }
    }
    pub fn selectors(&self) -> CssKeyframesSelectorList {
        support::list(&self.syntax, 0usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssDeclarationBlock> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for CssKeyframesItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssKeyframesItemFields {
    pub selectors: CssKeyframesSelectorList,
    pub block: SyntaxResult<AnyCssDeclarationBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssKeyframesPercentageSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssKeyframesPercentageSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssKeyframesPercentageSelectorFields {
        CssKeyframesPercentageSelectorFields {
            selector: self.selector(),
        }
    }
    pub fn selector(&self) -> SyntaxResult<CssPercentage> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for CssKeyframesPercentageSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssKeyframesPercentageSelectorFields {
    pub selector: SyntaxResult<CssPercentage>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssKeyframesScopeFunction {
    pub(crate) syntax: SyntaxNode,
}
impl CssKeyframesScopeFunction {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssKeyframesScopeFunctionFields {
        CssKeyframesScopeFunctionFields {
            scope: self.scope(),
            l_paren_token: self.l_paren_token(),
            name: self.name(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn scope(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<AnyCssKeyframesIdentifier> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for CssKeyframesScopeFunction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssKeyframesScopeFunctionFields {
    pub scope: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<AnyCssKeyframesIdentifier>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssKeyframesScopePrefix {
    pub(crate) syntax: SyntaxNode,
}
impl CssKeyframesScopePrefix {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssKeyframesScopePrefixFields {
        CssKeyframesScopePrefixFields {
            scope: self.scope(),
            name: self.name(),
        }
    }
    pub fn scope(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<AnyCssKeyframesIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for CssKeyframesScopePrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssKeyframesScopePrefixFields {
    pub scope: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<AnyCssKeyframesIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssKeyframesScopedName {
    pub(crate) syntax: SyntaxNode,
}
impl CssKeyframesScopedName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssKeyframesScopedNameFields {
        CssKeyframesScopedNameFields {
            colon_token: self.colon_token(),
            scope: self.scope(),
        }
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn scope(&self) -> SyntaxResult<AnyCssKeyframesScope> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for CssKeyframesScopedName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssKeyframesScopedNameFields {
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub scope: SyntaxResult<AnyCssKeyframesScope>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssLayerAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssLayerAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssLayerAtRuleFields {
        CssLayerAtRuleFields {
            layer_token: self.layer_token(),
            layer: self.layer(),
        }
    }
    pub fn layer_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn layer(&self) -> SyntaxResult<AnyCssLayer> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for CssLayerAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssLayerAtRuleFields {
    pub layer_token: SyntaxResult<SyntaxToken>,
    pub layer: SyntaxResult<AnyCssLayer>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssLayerDeclaration {
    pub(crate) syntax: SyntaxNode,
}
impl CssLayerDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssLayerDeclarationFields {
        CssLayerDeclarationFields {
            references: self.references(),
            block: self.block(),
        }
    }
    pub fn references(&self) -> CssLayerReferenceList {
        support::list(&self.syntax, 0usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssConditionalBlock> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for CssLayerDeclaration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssLayerDeclarationFields {
    pub references: CssLayerReferenceList,
    pub block: SyntaxResult<AnyCssConditionalBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssLayerReference {
    pub(crate) syntax: SyntaxNode,
}
impl CssLayerReference {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssLayerReferenceFields {
        CssLayerReferenceFields {
            references: self.references(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn references(&self) -> CssLayerReferenceList {
        support::list(&self.syntax, 0usize)
    }
    pub fn semicolon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for CssLayerReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssLayerReferenceFields {
    pub references: CssLayerReferenceList,
    pub semicolon_token: SyntaxResult<SyntaxToken>,
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
pub struct CssMarginAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssMarginAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMarginAtRuleFields {
        CssMarginAtRuleFields {
            at_token: self.at_token(),
            name: self.name(),
            block: self.block(),
        }
    }
    pub fn at_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssDeclarationOrAtRuleBlock> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssMarginAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssMarginAtRuleFields {
    pub at_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<SyntaxToken>,
    pub block: SyntaxResult<AnyCssDeclarationOrAtRuleBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaAndCondition {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaAndCondition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaAndConditionFields {
        CssMediaAndConditionFields {
            left: self.left(),
            and_token: self.and_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyCssMediaInParens> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn and_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyCssMediaAndCombinableCondition> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssMediaAndCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssMediaAndConditionFields {
    pub left: SyntaxResult<AnyCssMediaInParens>,
    pub and_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyCssMediaAndCombinableCondition>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaAndTypeQuery {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaAndTypeQuery {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaAndTypeQueryFields {
        CssMediaAndTypeQueryFields {
            left: self.left(),
            and_token: self.and_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<CssMediaTypeQuery> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn and_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyCssMediaTypeCondition> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssMediaAndTypeQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssMediaAndTypeQueryFields {
    pub left: SyntaxResult<CssMediaTypeQuery>,
    pub and_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyCssMediaTypeCondition>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaAtRuleFields {
        CssMediaAtRuleFields {
            media_token: self.media_token(),
            queries: self.queries(),
            block: self.block(),
        }
    }
    pub fn media_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn queries(&self) -> CssMediaQueryList {
        support::list(&self.syntax, 1usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssConditionalBlock> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssMediaAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssMediaAtRuleFields {
    pub media_token: SyntaxResult<SyntaxToken>,
    pub queries: CssMediaQueryList,
    pub block: SyntaxResult<AnyCssConditionalBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaConditionInParens {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaConditionInParens {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaConditionInParensFields {
        CssMediaConditionInParensFields {
            l_paren_token: self.l_paren_token(),
            condition: self.condition(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn condition(&self) -> SyntaxResult<AnyCssMediaCondition> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for CssMediaConditionInParens {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssMediaConditionInParensFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub condition: SyntaxResult<AnyCssMediaCondition>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaConditionQuery {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaConditionQuery {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaConditionQueryFields {
        CssMediaConditionQueryFields {
            condition: self.condition(),
        }
    }
    pub fn condition(&self) -> SyntaxResult<AnyCssMediaCondition> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for CssMediaConditionQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssMediaConditionQueryFields {
    pub condition: SyntaxResult<AnyCssMediaCondition>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaFeatureInParens {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaFeatureInParens {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaFeatureInParensFields {
        CssMediaFeatureInParensFields {
            l_paren_token: self.l_paren_token(),
            feature: self.feature(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn feature(&self) -> SyntaxResult<AnyCssQueryFeature> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for CssMediaFeatureInParens {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssMediaFeatureInParensFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub feature: SyntaxResult<AnyCssQueryFeature>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaNotCondition {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaNotCondition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaNotConditionFields {
        CssMediaNotConditionFields {
            not_token: self.not_token(),
            condition: self.condition(),
        }
    }
    pub fn not_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn condition(&self) -> SyntaxResult<AnyCssMediaInParens> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for CssMediaNotCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssMediaNotConditionFields {
    pub not_token: SyntaxResult<SyntaxToken>,
    pub condition: SyntaxResult<AnyCssMediaInParens>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaOrCondition {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaOrCondition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaOrConditionFields {
        CssMediaOrConditionFields {
            left: self.left(),
            or_token: self.or_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyCssMediaInParens> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn or_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyCssMediaOrCombinableCondition> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssMediaOrCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssMediaOrConditionFields {
    pub left: SyntaxResult<AnyCssMediaInParens>,
    pub or_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyCssMediaOrCombinableCondition>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaType {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaTypeFields {
        CssMediaTypeFields {
            value: self.value(),
        }
    }
    pub fn value(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for CssMediaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssMediaTypeFields {
    pub value: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaTypeQuery {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaTypeQuery {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaTypeQueryFields {
        CssMediaTypeQueryFields {
            modifier: self.modifier(),
            ty: self.ty(),
        }
    }
    pub fn modifier(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn ty(&self) -> SyntaxResult<CssMediaType> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for CssMediaTypeQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssMediaTypeQueryFields {
    pub modifier: Option<SyntaxToken>,
    pub ty: SyntaxResult<CssMediaType>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMetavariable {
    pub(crate) syntax: SyntaxNode,
}
impl CssMetavariable {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMetavariableFields {
        CssMetavariableFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for CssMetavariable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssMetavariableFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssNamedNamespacePrefix {
    pub(crate) syntax: SyntaxNode,
}
impl CssNamedNamespacePrefix {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssNamedNamespacePrefixFields {
        CssNamedNamespacePrefixFields { name: self.name() }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for CssNamedNamespacePrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssNamedNamespacePrefixFields {
    pub name: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssNamespace {
    pub(crate) syntax: SyntaxNode,
}
impl CssNamespace {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssNamespaceFields {
        CssNamespaceFields {
            prefix: self.prefix(),
            bitwise_or_token: self.bitwise_or_token(),
        }
    }
    pub fn prefix(&self) -> Option<AnyCssNamespacePrefix> {
        support::node(&self.syntax, 0usize)
    }
    pub fn bitwise_or_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for CssNamespace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssNamespaceFields {
    pub prefix: Option<AnyCssNamespacePrefix>,
    pub bitwise_or_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssNamespaceAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssNamespaceAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssNamespaceAtRuleFields {
        CssNamespaceAtRuleFields {
            namespace_token: self.namespace_token(),
            prefix: self.prefix(),
            url: self.url(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn namespace_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn prefix(&self) -> Option<CssIdentifier> {
        support::node(&self.syntax, 1usize)
    }
    pub fn url(&self) -> SyntaxResult<AnyCssNamespaceUrl> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn semicolon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for CssNamespaceAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssNamespaceAtRuleFields {
    pub namespace_token: SyntaxResult<SyntaxToken>,
    pub prefix: Option<CssIdentifier>,
    pub url: SyntaxResult<AnyCssNamespaceUrl>,
    pub semicolon_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssNestedQualifiedRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssNestedQualifiedRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssNestedQualifiedRuleFields {
        CssNestedQualifiedRuleFields {
            prelude: self.prelude(),
            block: self.block(),
        }
    }
    pub fn prelude(&self) -> CssRelativeSelectorList {
        support::list(&self.syntax, 0usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssDeclarationOrRuleBlock> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for CssNestedQualifiedRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssNestedQualifiedRuleFields {
    pub prelude: CssRelativeSelectorList,
    pub block: SyntaxResult<AnyCssDeclarationOrRuleBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssNestedSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssNestedSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssNestedSelectorFields {
        CssNestedSelectorFields {
            amp_token: self.amp_token(),
        }
    }
    pub fn amp_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for CssNestedSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssNestedSelectorFields {
    pub amp_token: SyntaxResult<SyntaxToken>,
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
impl Serialize for CssNthOffset {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
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
pub struct CssPageAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssPageAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPageAtRuleFields {
        CssPageAtRuleFields {
            page_token: self.page_token(),
            selectors: self.selectors(),
            block: self.block(),
        }
    }
    pub fn page_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn selectors(&self) -> CssPageSelectorList {
        support::list(&self.syntax, 1usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssPageAtRuleBlock> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssPageAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssPageAtRuleFields {
    pub page_token: SyntaxResult<SyntaxToken>,
    pub selectors: CssPageSelectorList,
    pub block: SyntaxResult<AnyCssPageAtRuleBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPageAtRuleBlock {
    pub(crate) syntax: SyntaxNode,
}
impl CssPageAtRuleBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPageAtRuleBlockFields {
        CssPageAtRuleBlockFields {
            l_curly_token: self.l_curly_token(),
            items: self.items(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> CssPageAtRuleItemList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for CssPageAtRuleBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssPageAtRuleBlockFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub items: CssPageAtRuleItemList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPageSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssPageSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPageSelectorFields {
        CssPageSelectorFields {
            ty: self.ty(),
            pseudos: self.pseudos(),
        }
    }
    pub fn ty(&self) -> Option<CssCustomIdentifier> {
        support::node(&self.syntax, 0usize)
    }
    pub fn pseudos(&self) -> CssPageSelectorPseudoList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for CssPageSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssPageSelectorFields {
    pub ty: Option<CssCustomIdentifier>,
    pub pseudos: CssPageSelectorPseudoList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPageSelectorPseudo {
    pub(crate) syntax: SyntaxNode,
}
impl CssPageSelectorPseudo {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPageSelectorPseudoFields {
        CssPageSelectorPseudoFields {
            colon_token: self.colon_token(),
            selector: self.selector(),
        }
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn selector(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for CssPageSelectorPseudo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssPageSelectorPseudoFields {
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub selector: SyntaxResult<CssIdentifier>,
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
            any_css_expression: self.any_css_expression(),
        }
    }
    pub fn any_css_expression(&self) -> SyntaxResult<AnyCssExpression> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for CssParameter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssParameterFields {
    pub any_css_expression: SyntaxResult<AnyCssExpression>,
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
            percent_token: self.percent_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn percent_token(&self) -> SyntaxResult<SyntaxToken> {
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
    pub percent_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPositionTryAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssPositionTryAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPositionTryAtRuleFields {
        CssPositionTryAtRuleFields {
            position_try_token: self.position_try_token(),
            name: self.name(),
            block: self.block(),
        }
    }
    pub fn position_try_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<CssDashedIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssDeclarationBlock> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssPositionTryAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssPositionTryAtRuleFields {
    pub position_try_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<CssDashedIdentifier>,
    pub block: SyntaxResult<AnyCssDeclarationBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPropertyAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssPropertyAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPropertyAtRuleFields {
        CssPropertyAtRuleFields {
            property_token: self.property_token(),
            name: self.name(),
            block: self.block(),
        }
    }
    pub fn property_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<CssDashedIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssDeclarationBlock> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssPropertyAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssPropertyAtRuleFields {
    pub property_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<CssDashedIdentifier>,
    pub block: SyntaxResult<AnyCssDeclarationBlock>,
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
impl Serialize for CssPseudoClassFunctionCompoundSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
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
            compound_selectors: self.compound_selectors(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn compound_selectors(&self) -> CssCompoundSelectorList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for CssPseudoClassFunctionCompoundSelectorList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssPseudoClassFunctionCompoundSelectorListFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub compound_selectors: CssCompoundSelectorList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassFunctionCustomIdentifierList {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassFunctionCustomIdentifierList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassFunctionCustomIdentifierListFields {
        CssPseudoClassFunctionCustomIdentifierListFields {
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
    pub fn items(&self) -> CssCustomIdentifierList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for CssPseudoClassFunctionCustomIdentifierList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssPseudoClassFunctionCustomIdentifierListFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub items: CssCustomIdentifierList,
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
impl Serialize for CssPseudoClassFunctionIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
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
impl Serialize for CssPseudoClassFunctionNth {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
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
            relative_selectors: self.relative_selectors(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn relative_selectors(&self) -> CssRelativeSelectorList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for CssPseudoClassFunctionRelativeSelectorList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssPseudoClassFunctionRelativeSelectorListFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub relative_selectors: CssRelativeSelectorList,
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
impl Serialize for CssPseudoClassFunctionSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
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
            selectors: self.selectors(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn selectors(&self) -> CssSelectorList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for CssPseudoClassFunctionSelectorList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssPseudoClassFunctionSelectorListFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub selectors: CssSelectorList,
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
            values: self.values(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn values(&self) -> CssPseudoValueList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for CssPseudoClassFunctionValueList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssPseudoClassFunctionValueListFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub values: CssPseudoValueList,
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
impl Serialize for CssPseudoClassIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
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
impl Serialize for CssPseudoClassNth {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
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
impl Serialize for CssPseudoClassNthIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
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
impl Serialize for CssPseudoClassNthNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
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
impl Serialize for CssPseudoClassNthSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
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
            selectors: self.selectors(),
        }
    }
    pub fn of_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn selectors(&self) -> CssSelectorList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for CssPseudoClassOfNthSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssPseudoClassOfNthSelectorFields {
    pub of_token: SyntaxResult<SyntaxToken>,
    pub selectors: CssSelectorList,
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
impl Serialize for CssPseudoClassSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssPseudoClassSelectorFields {
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub class: SyntaxResult<AnyCssPseudoClass>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoElementFunction {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoElementFunction {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoElementFunctionFields {
        CssPseudoElementFunctionFields {
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
    pub fn items(&self) -> CssPseudoElementFunctionParameterList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for CssPseudoElementFunction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssPseudoElementFunctionFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub items: CssPseudoElementFunctionParameterList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoElementFunctionCustomIdentifier {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoElementFunctionCustomIdentifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoElementFunctionCustomIdentifierFields {
        CssPseudoElementFunctionCustomIdentifierFields {
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
    pub fn ident(&self) -> SyntaxResult<CssCustomIdentifier> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for CssPseudoElementFunctionCustomIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssPseudoElementFunctionCustomIdentifierFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub ident: SyntaxResult<CssCustomIdentifier>,
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
impl Serialize for CssPseudoElementFunctionSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
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
impl Serialize for CssPseudoElementIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
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
impl Serialize for CssPseudoElementSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssPseudoElementSelectorFields {
    pub double_colon_token: SyntaxResult<SyntaxToken>,
    pub element: SyntaxResult<AnyCssPseudoElement>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssQualifiedRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssQualifiedRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssQualifiedRuleFields {
        CssQualifiedRuleFields {
            prelude: self.prelude(),
            block: self.block(),
        }
    }
    pub fn prelude(&self) -> CssSelectorList {
        support::list(&self.syntax, 0usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssDeclarationOrRuleBlock> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for CssQualifiedRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssQualifiedRuleFields {
    pub prelude: CssSelectorList,
    pub block: SyntaxResult<AnyCssDeclarationOrRuleBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssQueryFeatureBoolean {
    pub(crate) syntax: SyntaxNode,
}
impl CssQueryFeatureBoolean {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssQueryFeatureBooleanFields {
        CssQueryFeatureBooleanFields { name: self.name() }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for CssQueryFeatureBoolean {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssQueryFeatureBooleanFields {
    pub name: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssQueryFeaturePlain {
    pub(crate) syntax: SyntaxNode,
}
impl CssQueryFeaturePlain {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssQueryFeaturePlainFields {
        CssQueryFeaturePlainFields {
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
    pub fn value(&self) -> SyntaxResult<AnyCssQueryFeatureValue> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssQueryFeaturePlain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssQueryFeaturePlainFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnyCssQueryFeatureValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssQueryFeatureRange {
    pub(crate) syntax: SyntaxNode,
}
impl CssQueryFeatureRange {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssQueryFeatureRangeFields {
        CssQueryFeatureRangeFields {
            left: self.left(),
            comparison: self.comparison(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn comparison(&self) -> SyntaxResult<CssQueryFeatureRangeComparison> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyCssQueryFeatureValue> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssQueryFeatureRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssQueryFeatureRangeFields {
    pub left: SyntaxResult<CssIdentifier>,
    pub comparison: SyntaxResult<CssQueryFeatureRangeComparison>,
    pub right: SyntaxResult<AnyCssQueryFeatureValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssQueryFeatureRangeComparison {
    pub(crate) syntax: SyntaxNode,
}
impl CssQueryFeatureRangeComparison {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssQueryFeatureRangeComparisonFields {
        CssQueryFeatureRangeComparisonFields {
            operator: self.operator(),
        }
    }
    pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for CssQueryFeatureRangeComparison {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssQueryFeatureRangeComparisonFields {
    pub operator: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssQueryFeatureRangeInterval {
    pub(crate) syntax: SyntaxNode,
}
impl CssQueryFeatureRangeInterval {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssQueryFeatureRangeIntervalFields {
        CssQueryFeatureRangeIntervalFields {
            left: self.left(),
            left_comparison: self.left_comparison(),
            name: self.name(),
            right_comparison: self.right_comparison(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyCssQueryFeatureValue> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn left_comparison(&self) -> SyntaxResult<CssQueryFeatureRangeComparison> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn right_comparison(&self) -> SyntaxResult<CssQueryFeatureRangeComparison> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyCssQueryFeatureValue> {
        support::required_node(&self.syntax, 4usize)
    }
}
impl Serialize for CssQueryFeatureRangeInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssQueryFeatureRangeIntervalFields {
    pub left: SyntaxResult<AnyCssQueryFeatureValue>,
    pub left_comparison: SyntaxResult<CssQueryFeatureRangeComparison>,
    pub name: SyntaxResult<CssIdentifier>,
    pub right_comparison: SyntaxResult<CssQueryFeatureRangeComparison>,
    pub right: SyntaxResult<AnyCssQueryFeatureValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssQueryFeatureReverseRange {
    pub(crate) syntax: SyntaxNode,
}
impl CssQueryFeatureReverseRange {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssQueryFeatureReverseRangeFields {
        CssQueryFeatureReverseRangeFields {
            left: self.left(),
            comparison: self.comparison(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyCssQueryFeatureValue> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn comparison(&self) -> SyntaxResult<CssQueryFeatureRangeComparison> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssQueryFeatureReverseRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssQueryFeatureReverseRangeFields {
    pub left: SyntaxResult<AnyCssQueryFeatureValue>,
    pub comparison: SyntaxResult<CssQueryFeatureRangeComparison>,
    pub right: SyntaxResult<CssIdentifier>,
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
            slash_token: self.slash_token(),
            denominator: self.denominator(),
        }
    }
    pub fn numerator(&self) -> SyntaxResult<CssNumber> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn slash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn denominator(&self) -> SyntaxResult<CssNumber> {
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
    pub numerator: SyntaxResult<CssNumber>,
    pub slash_token: SyntaxResult<SyntaxToken>,
    pub denominator: SyntaxResult<CssNumber>,
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
impl Serialize for CssRelativeSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
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
impl Serialize for CssRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssRootFields {
    pub bom_token: Option<SyntaxToken>,
    pub rules: CssRuleList,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssRuleBlock {
    pub(crate) syntax: SyntaxNode,
}
impl CssRuleBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssRuleBlockFields {
        CssRuleBlockFields {
            l_curly_token: self.l_curly_token(),
            rules: self.rules(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn rules(&self) -> CssRuleList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for CssRuleBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssRuleBlockFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub rules: CssRuleList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssScopeAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssScopeAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssScopeAtRuleFields {
        CssScopeAtRuleFields {
            scope_token: self.scope_token(),
            range: self.range(),
            block: self.block(),
        }
    }
    pub fn scope_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn range(&self) -> Option<AnyCssScopeRange> {
        support::node(&self.syntax, 1usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssConditionalBlock> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssScopeAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssScopeAtRuleFields {
    pub scope_token: SyntaxResult<SyntaxToken>,
    pub range: Option<AnyCssScopeRange>,
    pub block: SyntaxResult<AnyCssConditionalBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssScopeEdge {
    pub(crate) syntax: SyntaxNode,
}
impl CssScopeEdge {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssScopeEdgeFields {
        CssScopeEdgeFields {
            l_paren_token: self.l_paren_token(),
            selectors: self.selectors(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn selectors(&self) -> CssSelectorList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for CssScopeEdge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssScopeEdgeFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub selectors: CssSelectorList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssScopeRangeEnd {
    pub(crate) syntax: SyntaxNode,
}
impl CssScopeRangeEnd {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssScopeRangeEndFields {
        CssScopeRangeEndFields {
            to_token: self.to_token(),
            end: self.end(),
        }
    }
    pub fn to_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn end(&self) -> SyntaxResult<CssScopeEdge> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for CssScopeRangeEnd {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssScopeRangeEndFields {
    pub to_token: SyntaxResult<SyntaxToken>,
    pub end: SyntaxResult<CssScopeEdge>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssScopeRangeInterval {
    pub(crate) syntax: SyntaxNode,
}
impl CssScopeRangeInterval {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssScopeRangeIntervalFields {
        CssScopeRangeIntervalFields {
            start: self.start(),
            to_token: self.to_token(),
            end: self.end(),
        }
    }
    pub fn start(&self) -> SyntaxResult<CssScopeEdge> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn to_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn end(&self) -> SyntaxResult<CssScopeEdge> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssScopeRangeInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssScopeRangeIntervalFields {
    pub start: SyntaxResult<CssScopeEdge>,
    pub to_token: SyntaxResult<SyntaxToken>,
    pub end: SyntaxResult<CssScopeEdge>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssScopeRangeStart {
    pub(crate) syntax: SyntaxNode,
}
impl CssScopeRangeStart {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssScopeRangeStartFields {
        CssScopeRangeStartFields {
            start: self.start(),
        }
    }
    pub fn start(&self) -> SyntaxResult<CssScopeEdge> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for CssScopeRangeStart {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssScopeRangeStartFields {
    pub start: SyntaxResult<CssScopeEdge>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssStartingStyleAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssStartingStyleAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssStartingStyleAtRuleFields {
        CssStartingStyleAtRuleFields {
            starting_style_token: self.starting_style_token(),
            block: self.block(),
        }
    }
    pub fn starting_style_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssConditionalBlock> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for CssStartingStyleAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssStartingStyleAtRuleFields {
    pub starting_style_token: SyntaxResult<SyntaxToken>,
    pub block: SyntaxResult<AnyCssConditionalBlock>,
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
pub struct CssSupportsAndCondition {
    pub(crate) syntax: SyntaxNode,
}
impl CssSupportsAndCondition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssSupportsAndConditionFields {
        CssSupportsAndConditionFields {
            left: self.left(),
            and_token: self.and_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyCssSupportsInParens> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn and_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyCssSupportsAndCombinableCondition> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssSupportsAndCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssSupportsAndConditionFields {
    pub left: SyntaxResult<AnyCssSupportsInParens>,
    pub and_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyCssSupportsAndCombinableCondition>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssSupportsAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssSupportsAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssSupportsAtRuleFields {
        CssSupportsAtRuleFields {
            supports_token: self.supports_token(),
            condition: self.condition(),
            block: self.block(),
        }
    }
    pub fn supports_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn condition(&self) -> SyntaxResult<AnyCssSupportsCondition> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssConditionalBlock> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssSupportsAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssSupportsAtRuleFields {
    pub supports_token: SyntaxResult<SyntaxToken>,
    pub condition: SyntaxResult<AnyCssSupportsCondition>,
    pub block: SyntaxResult<AnyCssConditionalBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssSupportsConditionInParens {
    pub(crate) syntax: SyntaxNode,
}
impl CssSupportsConditionInParens {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssSupportsConditionInParensFields {
        CssSupportsConditionInParensFields {
            l_paren_token: self.l_paren_token(),
            condition: self.condition(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn condition(&self) -> SyntaxResult<AnyCssSupportsCondition> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for CssSupportsConditionInParens {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssSupportsConditionInParensFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub condition: SyntaxResult<AnyCssSupportsCondition>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssSupportsFeatureDeclaration {
    pub(crate) syntax: SyntaxNode,
}
impl CssSupportsFeatureDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssSupportsFeatureDeclarationFields {
        CssSupportsFeatureDeclarationFields {
            l_paren_token: self.l_paren_token(),
            declaration: self.declaration(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn declaration(&self) -> SyntaxResult<CssDeclaration> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for CssSupportsFeatureDeclaration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssSupportsFeatureDeclarationFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub declaration: SyntaxResult<CssDeclaration>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssSupportsFeatureSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssSupportsFeatureSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssSupportsFeatureSelectorFields {
        CssSupportsFeatureSelectorFields {
            selector_token: self.selector_token(),
            l_paren_token: self.l_paren_token(),
            selector: self.selector(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn selector_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
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
impl Serialize for CssSupportsFeatureSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssSupportsFeatureSelectorFields {
    pub selector_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub selector: SyntaxResult<AnyCssSelector>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssSupportsNotCondition {
    pub(crate) syntax: SyntaxNode,
}
impl CssSupportsNotCondition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssSupportsNotConditionFields {
        CssSupportsNotConditionFields {
            not_token: self.not_token(),
            query: self.query(),
        }
    }
    pub fn not_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn query(&self) -> SyntaxResult<AnyCssSupportsInParens> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for CssSupportsNotCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssSupportsNotConditionFields {
    pub not_token: SyntaxResult<SyntaxToken>,
    pub query: SyntaxResult<AnyCssSupportsInParens>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssSupportsOrCondition {
    pub(crate) syntax: SyntaxNode,
}
impl CssSupportsOrCondition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssSupportsOrConditionFields {
        CssSupportsOrConditionFields {
            left: self.left(),
            or_token: self.or_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyCssSupportsInParens> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn or_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyCssSupportsOrCombinableCondition> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssSupportsOrCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssSupportsOrConditionFields {
    pub left: SyntaxResult<AnyCssSupportsInParens>,
    pub or_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyCssSupportsOrCombinableCondition>,
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
            namespace: self.namespace(),
            ident: self.ident(),
        }
    }
    pub fn namespace(&self) -> Option<CssNamespace> {
        support::node(&self.syntax, 0usize)
    }
    pub fn ident(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for CssTypeSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssTypeSelectorFields {
    pub namespace: Option<CssNamespace>,
    pub ident: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssUnicodeCodepoint {
    pub(crate) syntax: SyntaxNode,
}
impl CssUnicodeCodepoint {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssUnicodeCodepointFields {
        CssUnicodeCodepointFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for CssUnicodeCodepoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssUnicodeCodepointFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssUnicodeRange {
    pub(crate) syntax: SyntaxNode,
}
impl CssUnicodeRange {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssUnicodeRangeFields {
        CssUnicodeRangeFields {
            prefix_token: self.prefix_token(),
            value: self.value(),
        }
    }
    pub fn prefix_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<AnyCssUnicodeValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for CssUnicodeRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssUnicodeRangeFields {
    pub prefix_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnyCssUnicodeValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssUnicodeRangeInterval {
    pub(crate) syntax: SyntaxNode,
}
impl CssUnicodeRangeInterval {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssUnicodeRangeIntervalFields {
        CssUnicodeRangeIntervalFields {
            start: self.start(),
            minus_token: self.minus_token(),
            end: self.end(),
        }
    }
    pub fn start(&self) -> SyntaxResult<CssUnicodeCodepoint> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn minus_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn end(&self) -> SyntaxResult<CssUnicodeCodepoint> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssUnicodeRangeInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssUnicodeRangeIntervalFields {
    pub start: SyntaxResult<CssUnicodeCodepoint>,
    pub minus_token: SyntaxResult<SyntaxToken>,
    pub end: SyntaxResult<CssUnicodeCodepoint>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssUnicodeRangeWildcard {
    pub(crate) syntax: SyntaxNode,
}
impl CssUnicodeRangeWildcard {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssUnicodeRangeWildcardFields {
        CssUnicodeRangeWildcardFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for CssUnicodeRangeWildcard {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssUnicodeRangeWildcardFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssUniversalNamespacePrefix {
    pub(crate) syntax: SyntaxNode,
}
impl CssUniversalNamespacePrefix {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssUniversalNamespacePrefixFields {
        CssUniversalNamespacePrefixFields {
            star_token: self.star_token(),
        }
    }
    pub fn star_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for CssUniversalNamespacePrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssUniversalNamespacePrefixFields {
    pub star_token: SyntaxResult<SyntaxToken>,
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
            namespace: self.namespace(),
            star_token: self.star_token(),
        }
    }
    pub fn namespace(&self) -> Option<CssNamespace> {
        support::node(&self.syntax, 0usize)
    }
    pub fn star_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for CssUniversalSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssUniversalSelectorFields {
    pub namespace: Option<CssNamespace>,
    pub star_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssUnknownBlockAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssUnknownBlockAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssUnknownBlockAtRuleFields {
        CssUnknownBlockAtRuleFields {
            name: self.name(),
            components: self.components(),
            block: self.block(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn components(&self) -> SyntaxResult<CssUnknownAtRuleComponentList> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssDeclarationOrRuleBlock> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssUnknownBlockAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssUnknownBlockAtRuleFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub components: SyntaxResult<CssUnknownAtRuleComponentList>,
    pub block: SyntaxResult<AnyCssDeclarationOrRuleBlock>,
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
pub struct CssUnknownValueAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssUnknownValueAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssUnknownValueAtRuleFields {
        CssUnknownValueAtRuleFields {
            name: self.name(),
            components: self.components(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn components(&self) -> SyntaxResult<CssUnknownAtRuleComponentList> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn semicolon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for CssUnknownValueAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssUnknownValueAtRuleFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub components: SyntaxResult<CssUnknownAtRuleComponentList>,
    pub semicolon_token: SyntaxResult<SyntaxToken>,
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
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            value: self.value(),
            modifiers: self.modifiers(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> Option<AnyCssUrlValue> {
        support::node(&self.syntax, 2usize)
    }
    pub fn modifiers(&self) -> CssUrlModifierList {
        support::list(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
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
    pub name: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub value: Option<AnyCssUrlValue>,
    pub modifiers: CssUrlModifierList,
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
pub struct CssValueAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssValueAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssValueAtRuleFields {
        CssValueAtRuleFields {
            value_token: self.value_token(),
            clause: self.clause(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn clause(&self) -> SyntaxResult<AnyCssValueAtRuleClause> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn semicolon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for CssValueAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssValueAtRuleFields {
    pub value_token: SyntaxResult<SyntaxToken>,
    pub clause: SyntaxResult<AnyCssValueAtRuleClause>,
    pub semicolon_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssValueAtRuleDeclarationClause {
    pub(crate) syntax: SyntaxNode,
}
impl CssValueAtRuleDeclarationClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssValueAtRuleDeclarationClauseFields {
        CssValueAtRuleDeclarationClauseFields {
            properties: self.properties(),
        }
    }
    pub fn properties(&self) -> CssValueAtRulePropertyList {
        support::list(&self.syntax, 0usize)
    }
}
impl Serialize for CssValueAtRuleDeclarationClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssValueAtRuleDeclarationClauseFields {
    pub properties: CssValueAtRulePropertyList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssValueAtRuleGenericProperty {
    pub(crate) syntax: SyntaxNode,
}
impl CssValueAtRuleGenericProperty {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssValueAtRuleGenericPropertyFields {
        CssValueAtRuleGenericPropertyFields {
            name: self.name(),
            colon_token: self.colon_token(),
            value: self.value(),
        }
    }
    pub fn name(&self) -> SyntaxResult<AnyCssDeclarationName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<CssValueAtRuleGenericValue> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssValueAtRuleGenericProperty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssValueAtRuleGenericPropertyFields {
    pub name: SyntaxResult<AnyCssDeclarationName>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<CssValueAtRuleGenericValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssValueAtRuleImportClause {
    pub(crate) syntax: SyntaxNode,
}
impl CssValueAtRuleImportClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssValueAtRuleImportClauseFields {
        CssValueAtRuleImportClauseFields {
            specifiers: self.specifiers(),
            from_token: self.from_token(),
            source: self.source(),
        }
    }
    pub fn specifiers(&self) -> CssValueAtRuleImportSpecifierList {
        support::list(&self.syntax, 0usize)
    }
    pub fn from_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn source(&self) -> SyntaxResult<AnyCssValueAtRuleImportSource> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssValueAtRuleImportClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssValueAtRuleImportClauseFields {
    pub specifiers: CssValueAtRuleImportSpecifierList,
    pub from_token: SyntaxResult<SyntaxToken>,
    pub source: SyntaxResult<AnyCssValueAtRuleImportSource>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssValueAtRuleImportSpecifier {
    pub(crate) syntax: SyntaxNode,
}
impl CssValueAtRuleImportSpecifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssValueAtRuleImportSpecifierFields {
        CssValueAtRuleImportSpecifierFields { name: self.name() }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for CssValueAtRuleImportSpecifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssValueAtRuleImportSpecifierFields {
    pub name: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssValueAtRuleNamedImportSpecifier {
    pub(crate) syntax: SyntaxNode,
}
impl CssValueAtRuleNamedImportSpecifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssValueAtRuleNamedImportSpecifierFields {
        CssValueAtRuleNamedImportSpecifierFields {
            name: self.name(),
            as_token: self.as_token(),
            local_name: self.local_name(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn local_name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for CssValueAtRuleNamedImportSpecifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssValueAtRuleNamedImportSpecifierFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub as_token: SyntaxResult<SyntaxToken>,
    pub local_name: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssViewTransitionAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssViewTransitionAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssViewTransitionAtRuleFields {
        CssViewTransitionAtRuleFields {
            view_transition_token: self.view_transition_token(),
            block: self.block(),
        }
    }
    pub fn view_transition_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssDeclarationBlock> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for CssViewTransitionAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct CssViewTransitionAtRuleFields {
    pub view_transition_token: SyntaxResult<SyntaxToken>,
    pub block: SyntaxResult<AnyCssDeclarationBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssAtRule {
    CssBogusAtRule(CssBogusAtRule),
    CssCharsetAtRule(CssCharsetAtRule),
    CssColorProfileAtRule(CssColorProfileAtRule),
    CssContainerAtRule(CssContainerAtRule),
    CssCounterStyleAtRule(CssCounterStyleAtRule),
    CssDocumentAtRule(CssDocumentAtRule),
    CssFontFaceAtRule(CssFontFaceAtRule),
    CssFontFeatureValuesAtRule(CssFontFeatureValuesAtRule),
    CssFontPaletteValuesAtRule(CssFontPaletteValuesAtRule),
    CssImportAtRule(CssImportAtRule),
    CssKeyframesAtRule(CssKeyframesAtRule),
    CssLayerAtRule(CssLayerAtRule),
    CssMediaAtRule(CssMediaAtRule),
    CssNamespaceAtRule(CssNamespaceAtRule),
    CssPageAtRule(CssPageAtRule),
    CssPositionTryAtRule(CssPositionTryAtRule),
    CssPropertyAtRule(CssPropertyAtRule),
    CssScopeAtRule(CssScopeAtRule),
    CssStartingStyleAtRule(CssStartingStyleAtRule),
    CssSupportsAtRule(CssSupportsAtRule),
    CssUnknownBlockAtRule(CssUnknownBlockAtRule),
    CssUnknownValueAtRule(CssUnknownValueAtRule),
    CssValueAtRule(CssValueAtRule),
    CssViewTransitionAtRule(CssViewTransitionAtRule),
}
impl AnyCssAtRule {
    pub fn as_css_bogus_at_rule(&self) -> Option<&CssBogusAtRule> {
        match &self {
            Self::CssBogusAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_charset_at_rule(&self) -> Option<&CssCharsetAtRule> {
        match &self {
            Self::CssCharsetAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_color_profile_at_rule(&self) -> Option<&CssColorProfileAtRule> {
        match &self {
            Self::CssColorProfileAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_at_rule(&self) -> Option<&CssContainerAtRule> {
        match &self {
            Self::CssContainerAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_counter_style_at_rule(&self) -> Option<&CssCounterStyleAtRule> {
        match &self {
            Self::CssCounterStyleAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_document_at_rule(&self) -> Option<&CssDocumentAtRule> {
        match &self {
            Self::CssDocumentAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_font_face_at_rule(&self) -> Option<&CssFontFaceAtRule> {
        match &self {
            Self::CssFontFaceAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_font_feature_values_at_rule(&self) -> Option<&CssFontFeatureValuesAtRule> {
        match &self {
            Self::CssFontFeatureValuesAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_font_palette_values_at_rule(&self) -> Option<&CssFontPaletteValuesAtRule> {
        match &self {
            Self::CssFontPaletteValuesAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_import_at_rule(&self) -> Option<&CssImportAtRule> {
        match &self {
            Self::CssImportAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_keyframes_at_rule(&self) -> Option<&CssKeyframesAtRule> {
        match &self {
            Self::CssKeyframesAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_layer_at_rule(&self) -> Option<&CssLayerAtRule> {
        match &self {
            Self::CssLayerAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_at_rule(&self) -> Option<&CssMediaAtRule> {
        match &self {
            Self::CssMediaAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_namespace_at_rule(&self) -> Option<&CssNamespaceAtRule> {
        match &self {
            Self::CssNamespaceAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_page_at_rule(&self) -> Option<&CssPageAtRule> {
        match &self {
            Self::CssPageAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_position_try_at_rule(&self) -> Option<&CssPositionTryAtRule> {
        match &self {
            Self::CssPositionTryAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_property_at_rule(&self) -> Option<&CssPropertyAtRule> {
        match &self {
            Self::CssPropertyAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_scope_at_rule(&self) -> Option<&CssScopeAtRule> {
        match &self {
            Self::CssScopeAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_starting_style_at_rule(&self) -> Option<&CssStartingStyleAtRule> {
        match &self {
            Self::CssStartingStyleAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_supports_at_rule(&self) -> Option<&CssSupportsAtRule> {
        match &self {
            Self::CssSupportsAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_unknown_block_at_rule(&self) -> Option<&CssUnknownBlockAtRule> {
        match &self {
            Self::CssUnknownBlockAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_unknown_value_at_rule(&self) -> Option<&CssUnknownValueAtRule> {
        match &self {
            Self::CssUnknownValueAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_value_at_rule(&self) -> Option<&CssValueAtRule> {
        match &self {
            Self::CssValueAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_view_transition_at_rule(&self) -> Option<&CssViewTransitionAtRule> {
        match &self {
            Self::CssViewTransitionAtRule(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssAttributeMatcherValue {
    CssIdentifier(CssIdentifier),
    CssString(CssString),
}
impl AnyCssAttributeMatcherValue {
    pub fn as_css_identifier(&self) -> Option<&CssIdentifier> {
        match &self {
            Self::CssIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_string(&self) -> Option<&CssString> {
        match &self {
            Self::CssString(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssComposesImportSource {
    CssIdentifier(CssIdentifier),
    CssString(CssString),
}
impl AnyCssComposesImportSource {
    pub fn as_css_identifier(&self) -> Option<&CssIdentifier> {
        match &self {
            Self::CssIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_string(&self) -> Option<&CssString> {
        match &self {
            Self::CssString(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssCompoundSelector {
    CssBogusSelector(CssBogusSelector),
    CssCompoundSelector(CssCompoundSelector),
}
impl AnyCssCompoundSelector {
    pub fn as_css_bogus_selector(&self) -> Option<&CssBogusSelector> {
        match &self {
            Self::CssBogusSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_compound_selector(&self) -> Option<&CssCompoundSelector> {
        match &self {
            Self::CssCompoundSelector(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssConditionalBlock {
    CssBogusBlock(CssBogusBlock),
    CssDeclarationOrRuleBlock(CssDeclarationOrRuleBlock),
    CssRuleBlock(CssRuleBlock),
}
impl AnyCssConditionalBlock {
    pub fn as_css_bogus_block(&self) -> Option<&CssBogusBlock> {
        match &self {
            Self::CssBogusBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_declaration_or_rule_block(&self) -> Option<&CssDeclarationOrRuleBlock> {
        match &self {
            Self::CssDeclarationOrRuleBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_rule_block(&self) -> Option<&CssRuleBlock> {
        match &self {
            Self::CssRuleBlock(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssContainerAndCombinableQuery {
    AnyCssContainerQueryInParens(AnyCssContainerQueryInParens),
    CssContainerAndQuery(CssContainerAndQuery),
}
impl AnyCssContainerAndCombinableQuery {
    pub fn as_any_css_container_query_in_parens(&self) -> Option<&AnyCssContainerQueryInParens> {
        match &self {
            Self::AnyCssContainerQueryInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_and_query(&self) -> Option<&CssContainerAndQuery> {
        match &self {
            Self::CssContainerAndQuery(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssContainerOrCombinableQuery {
    AnyCssContainerQueryInParens(AnyCssContainerQueryInParens),
    CssContainerOrQuery(CssContainerOrQuery),
}
impl AnyCssContainerOrCombinableQuery {
    pub fn as_any_css_container_query_in_parens(&self) -> Option<&AnyCssContainerQueryInParens> {
        match &self {
            Self::AnyCssContainerQueryInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_or_query(&self) -> Option<&CssContainerOrQuery> {
        match &self {
            Self::CssContainerOrQuery(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssContainerQuery {
    AnyCssContainerQueryInParens(AnyCssContainerQueryInParens),
    CssContainerAndQuery(CssContainerAndQuery),
    CssContainerNotQuery(CssContainerNotQuery),
    CssContainerOrQuery(CssContainerOrQuery),
}
impl AnyCssContainerQuery {
    pub fn as_any_css_container_query_in_parens(&self) -> Option<&AnyCssContainerQueryInParens> {
        match &self {
            Self::AnyCssContainerQueryInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_and_query(&self) -> Option<&CssContainerAndQuery> {
        match &self {
            Self::CssContainerAndQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_not_query(&self) -> Option<&CssContainerNotQuery> {
        match &self {
            Self::CssContainerNotQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_or_query(&self) -> Option<&CssContainerOrQuery> {
        match &self {
            Self::CssContainerOrQuery(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssContainerQueryInParens {
    CssContainerQueryInParens(CssContainerQueryInParens),
    CssContainerSizeFeatureInParens(CssContainerSizeFeatureInParens),
    CssContainerStyleQueryInParens(CssContainerStyleQueryInParens),
}
impl AnyCssContainerQueryInParens {
    pub fn as_css_container_query_in_parens(&self) -> Option<&CssContainerQueryInParens> {
        match &self {
            Self::CssContainerQueryInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_size_feature_in_parens(
        &self,
    ) -> Option<&CssContainerSizeFeatureInParens> {
        match &self {
            Self::CssContainerSizeFeatureInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_style_query_in_parens(
        &self,
    ) -> Option<&CssContainerStyleQueryInParens> {
        match &self {
            Self::CssContainerStyleQueryInParens(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssContainerStyleAndCombinableQuery {
    CssContainerStyleAndQuery(CssContainerStyleAndQuery),
    CssContainerStyleInParens(CssContainerStyleInParens),
}
impl AnyCssContainerStyleAndCombinableQuery {
    pub fn as_css_container_style_and_query(&self) -> Option<&CssContainerStyleAndQuery> {
        match &self {
            Self::CssContainerStyleAndQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_style_in_parens(&self) -> Option<&CssContainerStyleInParens> {
        match &self {
            Self::CssContainerStyleInParens(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssContainerStyleInParens {
    AnyCssContainerStyleQuery(AnyCssContainerStyleQuery),
    CssDeclaration(CssDeclaration),
}
impl AnyCssContainerStyleInParens {
    pub fn as_any_css_container_style_query(&self) -> Option<&AnyCssContainerStyleQuery> {
        match &self {
            Self::AnyCssContainerStyleQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_declaration(&self) -> Option<&CssDeclaration> {
        match &self {
            Self::CssDeclaration(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssContainerStyleOrCombinableQuery {
    CssContainerStyleInParens(CssContainerStyleInParens),
    CssContainerStyleOrQuery(CssContainerStyleOrQuery),
}
impl AnyCssContainerStyleOrCombinableQuery {
    pub fn as_css_container_style_in_parens(&self) -> Option<&CssContainerStyleInParens> {
        match &self {
            Self::CssContainerStyleInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_style_or_query(&self) -> Option<&CssContainerStyleOrQuery> {
        match &self {
            Self::CssContainerStyleOrQuery(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssContainerStyleQuery {
    CssContainerStyleAndQuery(CssContainerStyleAndQuery),
    CssContainerStyleInParens(CssContainerStyleInParens),
    CssContainerStyleNotQuery(CssContainerStyleNotQuery),
    CssContainerStyleOrQuery(CssContainerStyleOrQuery),
    CssDeclaration(CssDeclaration),
}
impl AnyCssContainerStyleQuery {
    pub fn as_css_container_style_and_query(&self) -> Option<&CssContainerStyleAndQuery> {
        match &self {
            Self::CssContainerStyleAndQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_style_in_parens(&self) -> Option<&CssContainerStyleInParens> {
        match &self {
            Self::CssContainerStyleInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_style_not_query(&self) -> Option<&CssContainerStyleNotQuery> {
        match &self {
            Self::CssContainerStyleNotQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_style_or_query(&self) -> Option<&CssContainerStyleOrQuery> {
        match &self {
            Self::CssContainerStyleOrQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_declaration(&self) -> Option<&CssDeclaration> {
        match &self {
            Self::CssDeclaration(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssCustomIdentifier {
    CssBogusCustomIdentifier(CssBogusCustomIdentifier),
    CssCustomIdentifier(CssCustomIdentifier),
}
impl AnyCssCustomIdentifier {
    pub fn as_css_bogus_custom_identifier(&self) -> Option<&CssBogusCustomIdentifier> {
        match &self {
            Self::CssBogusCustomIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_custom_identifier(&self) -> Option<&CssCustomIdentifier> {
        match &self {
            Self::CssCustomIdentifier(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssDeclaration {
    CssDeclarationWithSemicolon(CssDeclarationWithSemicolon),
    CssEmptyDeclaration(CssEmptyDeclaration),
}
impl AnyCssDeclaration {
    pub fn as_css_declaration_with_semicolon(&self) -> Option<&CssDeclarationWithSemicolon> {
        match &self {
            Self::CssDeclarationWithSemicolon(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_empty_declaration(&self) -> Option<&CssEmptyDeclaration> {
        match &self {
            Self::CssEmptyDeclaration(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssDeclarationBlock {
    CssBogusBlock(CssBogusBlock),
    CssDeclarationBlock(CssDeclarationBlock),
}
impl AnyCssDeclarationBlock {
    pub fn as_css_bogus_block(&self) -> Option<&CssBogusBlock> {
        match &self {
            Self::CssBogusBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_declaration_block(&self) -> Option<&CssDeclarationBlock> {
        match &self {
            Self::CssDeclarationBlock(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssDeclarationName {
    CssDashedIdentifier(CssDashedIdentifier),
    CssIdentifier(CssIdentifier),
}
impl AnyCssDeclarationName {
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
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssDeclarationOrAtRule {
    CssAtRule(CssAtRule),
    CssDeclarationWithSemicolon(CssDeclarationWithSemicolon),
    CssEmptyDeclaration(CssEmptyDeclaration),
}
impl AnyCssDeclarationOrAtRule {
    pub fn as_css_at_rule(&self) -> Option<&CssAtRule> {
        match &self {
            Self::CssAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_declaration_with_semicolon(&self) -> Option<&CssDeclarationWithSemicolon> {
        match &self {
            Self::CssDeclarationWithSemicolon(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_empty_declaration(&self) -> Option<&CssEmptyDeclaration> {
        match &self {
            Self::CssEmptyDeclaration(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssDeclarationOrAtRuleBlock {
    CssBogusBlock(CssBogusBlock),
    CssDeclarationOrAtRuleBlock(CssDeclarationOrAtRuleBlock),
}
impl AnyCssDeclarationOrAtRuleBlock {
    pub fn as_css_bogus_block(&self) -> Option<&CssBogusBlock> {
        match &self {
            Self::CssBogusBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_declaration_or_at_rule_block(&self) -> Option<&CssDeclarationOrAtRuleBlock> {
        match &self {
            Self::CssDeclarationOrAtRuleBlock(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssDeclarationOrRule {
    AnyCssRule(AnyCssRule),
    CssBogus(CssBogus),
    CssDeclarationWithSemicolon(CssDeclarationWithSemicolon),
    CssEmptyDeclaration(CssEmptyDeclaration),
    CssMetavariable(CssMetavariable),
}
impl AnyCssDeclarationOrRule {
    pub fn as_any_css_rule(&self) -> Option<&AnyCssRule> {
        match &self {
            Self::AnyCssRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_bogus(&self) -> Option<&CssBogus> {
        match &self {
            Self::CssBogus(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_declaration_with_semicolon(&self) -> Option<&CssDeclarationWithSemicolon> {
        match &self {
            Self::CssDeclarationWithSemicolon(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_empty_declaration(&self) -> Option<&CssEmptyDeclaration> {
        match &self {
            Self::CssEmptyDeclaration(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_metavariable(&self) -> Option<&CssMetavariable> {
        match &self {
            Self::CssMetavariable(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssDeclarationOrRuleBlock {
    CssBogusBlock(CssBogusBlock),
    CssDeclarationOrRuleBlock(CssDeclarationOrRuleBlock),
}
impl AnyCssDeclarationOrRuleBlock {
    pub fn as_css_bogus_block(&self) -> Option<&CssBogusBlock> {
        match &self {
            Self::CssBogusBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_declaration_or_rule_block(&self) -> Option<&CssDeclarationOrRuleBlock> {
        match &self {
            Self::CssDeclarationOrRuleBlock(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssDimension {
    CssPercentage(CssPercentage),
    CssRegularDimension(CssRegularDimension),
    CssUnknownDimension(CssUnknownDimension),
}
impl AnyCssDimension {
    pub fn as_css_percentage(&self) -> Option<&CssPercentage> {
        match &self {
            Self::CssPercentage(item) => Some(item),
            _ => None,
        }
    }
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
pub enum AnyCssDocumentMatcher {
    CssBogusDocumentMatcher(CssBogusDocumentMatcher),
    CssDocumentCustomMatcher(CssDocumentCustomMatcher),
    CssUrlFunction(CssUrlFunction),
}
impl AnyCssDocumentMatcher {
    pub fn as_css_bogus_document_matcher(&self) -> Option<&CssBogusDocumentMatcher> {
        match &self {
            Self::CssBogusDocumentMatcher(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_document_custom_matcher(&self) -> Option<&CssDocumentCustomMatcher> {
        match &self {
            Self::CssDocumentCustomMatcher(item) => Some(item),
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
pub enum AnyCssExpression {
    CssBinaryExpression(CssBinaryExpression),
    CssListOfComponentValuesExpression(CssListOfComponentValuesExpression),
    CssParenthesizedExpression(CssParenthesizedExpression),
}
impl AnyCssExpression {
    pub fn as_css_binary_expression(&self) -> Option<&CssBinaryExpression> {
        match &self {
            Self::CssBinaryExpression(item) => Some(item),
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
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssFontFamilyName {
    CssBogusFontFamilyName(CssBogusFontFamilyName),
    CssFontFamilyName(CssFontFamilyName),
    CssString(CssString),
}
impl AnyCssFontFamilyName {
    pub fn as_css_bogus_font_family_name(&self) -> Option<&CssBogusFontFamilyName> {
        match &self {
            Self::CssBogusFontFamilyName(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_font_family_name(&self) -> Option<&CssFontFamilyName> {
        match &self {
            Self::CssFontFamilyName(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_string(&self) -> Option<&CssString> {
        match &self {
            Self::CssString(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssFontFeatureValuesBlock {
    CssBogusBlock(CssBogusBlock),
    CssFontFeatureValuesBlock(CssFontFeatureValuesBlock),
}
impl AnyCssFontFeatureValuesBlock {
    pub fn as_css_bogus_block(&self) -> Option<&CssBogusBlock> {
        match &self {
            Self::CssBogusBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_font_feature_values_block(&self) -> Option<&CssFontFeatureValuesBlock> {
        match &self {
            Self::CssFontFeatureValuesBlock(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssFontFeatureValuesItem {
    CssBogusFontFeatureValuesItem(CssBogusFontFeatureValuesItem),
    CssFontFeatureValuesItem(CssFontFeatureValuesItem),
}
impl AnyCssFontFeatureValuesItem {
    pub fn as_css_bogus_font_feature_values_item(&self) -> Option<&CssBogusFontFeatureValuesItem> {
        match &self {
            Self::CssBogusFontFeatureValuesItem(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_font_feature_values_item(&self) -> Option<&CssFontFeatureValuesItem> {
        match &self {
            Self::CssFontFeatureValuesItem(item) => Some(item),
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
pub enum AnyCssImportLayer {
    CssImportAnonymousLayer(CssImportAnonymousLayer),
    CssImportNamedLayer(CssImportNamedLayer),
}
impl AnyCssImportLayer {
    pub fn as_css_import_anonymous_layer(&self) -> Option<&CssImportAnonymousLayer> {
        match &self {
            Self::CssImportAnonymousLayer(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_import_named_layer(&self) -> Option<&CssImportNamedLayer> {
        match &self {
            Self::CssImportNamedLayer(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssImportSupportsCondition {
    AnyCssSupportsCondition(AnyCssSupportsCondition),
    CssDeclaration(CssDeclaration),
}
impl AnyCssImportSupportsCondition {
    pub fn as_any_css_supports_condition(&self) -> Option<&AnyCssSupportsCondition> {
        match &self {
            Self::AnyCssSupportsCondition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_declaration(&self) -> Option<&CssDeclaration> {
        match &self {
            Self::CssDeclaration(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssImportUrl {
    CssString(CssString),
    CssUrlFunction(CssUrlFunction),
}
impl AnyCssImportUrl {
    pub fn as_css_string(&self) -> Option<&CssString> {
        match &self {
            Self::CssString(item) => Some(item),
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
pub enum AnyCssKeyframesBlock {
    CssBogusBlock(CssBogusBlock),
    CssKeyframesBlock(CssKeyframesBlock),
}
impl AnyCssKeyframesBlock {
    pub fn as_css_bogus_block(&self) -> Option<&CssBogusBlock> {
        match &self {
            Self::CssBogusBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_keyframes_block(&self) -> Option<&CssKeyframesBlock> {
        match &self {
            Self::CssKeyframesBlock(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssKeyframesIdentifier {
    CssCustomIdentifier(CssCustomIdentifier),
    CssString(CssString),
}
impl AnyCssKeyframesIdentifier {
    pub fn as_css_custom_identifier(&self) -> Option<&CssCustomIdentifier> {
        match &self {
            Self::CssCustomIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_string(&self) -> Option<&CssString> {
        match &self {
            Self::CssString(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssKeyframesItem {
    CssBogusKeyframesItem(CssBogusKeyframesItem),
    CssKeyframesItem(CssKeyframesItem),
}
impl AnyCssKeyframesItem {
    pub fn as_css_bogus_keyframes_item(&self) -> Option<&CssBogusKeyframesItem> {
        match &self {
            Self::CssBogusKeyframesItem(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_keyframes_item(&self) -> Option<&CssKeyframesItem> {
        match &self {
            Self::CssKeyframesItem(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssKeyframesName {
    AnyCssKeyframesIdentifier(AnyCssKeyframesIdentifier),
    CssBogusKeyframesName(CssBogusKeyframesName),
    CssKeyframesScopedName(CssKeyframesScopedName),
}
impl AnyCssKeyframesName {
    pub fn as_any_css_keyframes_identifier(&self) -> Option<&AnyCssKeyframesIdentifier> {
        match &self {
            Self::AnyCssKeyframesIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_bogus_keyframes_name(&self) -> Option<&CssBogusKeyframesName> {
        match &self {
            Self::CssBogusKeyframesName(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_keyframes_scoped_name(&self) -> Option<&CssKeyframesScopedName> {
        match &self {
            Self::CssKeyframesScopedName(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssKeyframesScope {
    CssKeyframesScopeFunction(CssKeyframesScopeFunction),
    CssKeyframesScopePrefix(CssKeyframesScopePrefix),
}
impl AnyCssKeyframesScope {
    pub fn as_css_keyframes_scope_function(&self) -> Option<&CssKeyframesScopeFunction> {
        match &self {
            Self::CssKeyframesScopeFunction(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_keyframes_scope_prefix(&self) -> Option<&CssKeyframesScopePrefix> {
        match &self {
            Self::CssKeyframesScopePrefix(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssKeyframesSelector {
    CssBogusSelector(CssBogusSelector),
    CssKeyframesIdentSelector(CssKeyframesIdentSelector),
    CssKeyframesPercentageSelector(CssKeyframesPercentageSelector),
}
impl AnyCssKeyframesSelector {
    pub fn as_css_bogus_selector(&self) -> Option<&CssBogusSelector> {
        match &self {
            Self::CssBogusSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_keyframes_ident_selector(&self) -> Option<&CssKeyframesIdentSelector> {
        match &self {
            Self::CssKeyframesIdentSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_keyframes_percentage_selector(&self) -> Option<&CssKeyframesPercentageSelector> {
        match &self {
            Self::CssKeyframesPercentageSelector(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssLayer {
    CssBogusLayer(CssBogusLayer),
    CssLayerDeclaration(CssLayerDeclaration),
    CssLayerReference(CssLayerReference),
}
impl AnyCssLayer {
    pub fn as_css_bogus_layer(&self) -> Option<&CssBogusLayer> {
        match &self {
            Self::CssBogusLayer(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_layer_declaration(&self) -> Option<&CssLayerDeclaration> {
        match &self {
            Self::CssLayerDeclaration(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_layer_reference(&self) -> Option<&CssLayerReference> {
        match &self {
            Self::CssLayerReference(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssMediaAndCombinableCondition {
    AnyCssMediaInParens(AnyCssMediaInParens),
    CssMediaAndCondition(CssMediaAndCondition),
}
impl AnyCssMediaAndCombinableCondition {
    pub fn as_any_css_media_in_parens(&self) -> Option<&AnyCssMediaInParens> {
        match &self {
            Self::AnyCssMediaInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_and_condition(&self) -> Option<&CssMediaAndCondition> {
        match &self {
            Self::CssMediaAndCondition(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssMediaCondition {
    AnyCssMediaInParens(AnyCssMediaInParens),
    CssMediaAndCondition(CssMediaAndCondition),
    CssMediaNotCondition(CssMediaNotCondition),
    CssMediaOrCondition(CssMediaOrCondition),
}
impl AnyCssMediaCondition {
    pub fn as_any_css_media_in_parens(&self) -> Option<&AnyCssMediaInParens> {
        match &self {
            Self::AnyCssMediaInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_and_condition(&self) -> Option<&CssMediaAndCondition> {
        match &self {
            Self::CssMediaAndCondition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_not_condition(&self) -> Option<&CssMediaNotCondition> {
        match &self {
            Self::CssMediaNotCondition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_or_condition(&self) -> Option<&CssMediaOrCondition> {
        match &self {
            Self::CssMediaOrCondition(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssMediaInParens {
    CssMediaConditionInParens(CssMediaConditionInParens),
    CssMediaFeatureInParens(CssMediaFeatureInParens),
}
impl AnyCssMediaInParens {
    pub fn as_css_media_condition_in_parens(&self) -> Option<&CssMediaConditionInParens> {
        match &self {
            Self::CssMediaConditionInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_feature_in_parens(&self) -> Option<&CssMediaFeatureInParens> {
        match &self {
            Self::CssMediaFeatureInParens(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssMediaOrCombinableCondition {
    AnyCssMediaInParens(AnyCssMediaInParens),
    CssMediaOrCondition(CssMediaOrCondition),
}
impl AnyCssMediaOrCombinableCondition {
    pub fn as_any_css_media_in_parens(&self) -> Option<&AnyCssMediaInParens> {
        match &self {
            Self::AnyCssMediaInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_or_condition(&self) -> Option<&CssMediaOrCondition> {
        match &self {
            Self::CssMediaOrCondition(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssMediaQuery {
    AnyCssMediaTypeQuery(AnyCssMediaTypeQuery),
    CssBogusMediaQuery(CssBogusMediaQuery),
    CssMediaConditionQuery(CssMediaConditionQuery),
    CssMetavariable(CssMetavariable),
}
impl AnyCssMediaQuery {
    pub fn as_any_css_media_type_query(&self) -> Option<&AnyCssMediaTypeQuery> {
        match &self {
            Self::AnyCssMediaTypeQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_bogus_media_query(&self) -> Option<&CssBogusMediaQuery> {
        match &self {
            Self::CssBogusMediaQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_condition_query(&self) -> Option<&CssMediaConditionQuery> {
        match &self {
            Self::CssMediaConditionQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_metavariable(&self) -> Option<&CssMetavariable> {
        match &self {
            Self::CssMetavariable(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssMediaTypeCondition {
    AnyCssMediaInParens(AnyCssMediaInParens),
    CssMediaAndCondition(CssMediaAndCondition),
    CssMediaNotCondition(CssMediaNotCondition),
}
impl AnyCssMediaTypeCondition {
    pub fn as_any_css_media_in_parens(&self) -> Option<&AnyCssMediaInParens> {
        match &self {
            Self::AnyCssMediaInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_and_condition(&self) -> Option<&CssMediaAndCondition> {
        match &self {
            Self::CssMediaAndCondition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_not_condition(&self) -> Option<&CssMediaNotCondition> {
        match &self {
            Self::CssMediaNotCondition(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssMediaTypeQuery {
    CssMediaAndTypeQuery(CssMediaAndTypeQuery),
    CssMediaTypeQuery(CssMediaTypeQuery),
}
impl AnyCssMediaTypeQuery {
    pub fn as_css_media_and_type_query(&self) -> Option<&CssMediaAndTypeQuery> {
        match &self {
            Self::CssMediaAndTypeQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_type_query(&self) -> Option<&CssMediaTypeQuery> {
        match &self {
            Self::CssMediaTypeQuery(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssNamespacePrefix {
    CssNamedNamespacePrefix(CssNamedNamespacePrefix),
    CssUniversalNamespacePrefix(CssUniversalNamespacePrefix),
}
impl AnyCssNamespacePrefix {
    pub fn as_css_named_namespace_prefix(&self) -> Option<&CssNamedNamespacePrefix> {
        match &self {
            Self::CssNamedNamespacePrefix(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_universal_namespace_prefix(&self) -> Option<&CssUniversalNamespacePrefix> {
        match &self {
            Self::CssUniversalNamespacePrefix(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssNamespaceUrl {
    CssString(CssString),
    CssUrlFunction(CssUrlFunction),
}
impl AnyCssNamespaceUrl {
    pub fn as_css_string(&self) -> Option<&CssString> {
        match &self {
            Self::CssString(item) => Some(item),
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
pub enum AnyCssPageAtRuleBlock {
    CssBogusBlock(CssBogusBlock),
    CssPageAtRuleBlock(CssPageAtRuleBlock),
}
impl AnyCssPageAtRuleBlock {
    pub fn as_css_bogus_block(&self) -> Option<&CssBogusBlock> {
        match &self {
            Self::CssBogusBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_page_at_rule_block(&self) -> Option<&CssPageAtRuleBlock> {
        match &self {
            Self::CssPageAtRuleBlock(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssPageAtRuleItem {
    CssAtRule(CssAtRule),
    CssDeclarationWithSemicolon(CssDeclarationWithSemicolon),
    CssEmptyDeclaration(CssEmptyDeclaration),
    CssMarginAtRule(CssMarginAtRule),
}
impl AnyCssPageAtRuleItem {
    pub fn as_css_at_rule(&self) -> Option<&CssAtRule> {
        match &self {
            Self::CssAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_declaration_with_semicolon(&self) -> Option<&CssDeclarationWithSemicolon> {
        match &self {
            Self::CssDeclarationWithSemicolon(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_empty_declaration(&self) -> Option<&CssEmptyDeclaration> {
        match &self {
            Self::CssEmptyDeclaration(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_margin_at_rule(&self) -> Option<&CssMarginAtRule> {
        match &self {
            Self::CssMarginAtRule(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssPageSelector {
    CssBogusSelector(CssBogusSelector),
    CssPageSelector(CssPageSelector),
}
impl AnyCssPageSelector {
    pub fn as_css_bogus_selector(&self) -> Option<&CssBogusSelector> {
        match &self {
            Self::CssBogusSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_page_selector(&self) -> Option<&CssPageSelector> {
        match &self {
            Self::CssPageSelector(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssPageSelectorPseudo {
    CssBogusPageSelectorPseudo(CssBogusPageSelectorPseudo),
    CssPageSelectorPseudo(CssPageSelectorPseudo),
}
impl AnyCssPageSelectorPseudo {
    pub fn as_css_bogus_page_selector_pseudo(&self) -> Option<&CssBogusPageSelectorPseudo> {
        match &self {
            Self::CssBogusPageSelectorPseudo(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_page_selector_pseudo(&self) -> Option<&CssPageSelectorPseudo> {
        match &self {
            Self::CssPageSelectorPseudo(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssProperty {
    CssBogusProperty(CssBogusProperty),
    CssComposesProperty(CssComposesProperty),
    CssGenericProperty(CssGenericProperty),
}
impl AnyCssProperty {
    pub fn as_css_bogus_property(&self) -> Option<&CssBogusProperty> {
        match &self {
            Self::CssBogusProperty(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_composes_property(&self) -> Option<&CssComposesProperty> {
        match &self {
            Self::CssComposesProperty(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_generic_property(&self) -> Option<&CssGenericProperty> {
        match &self {
            Self::CssGenericProperty(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssPseudoClass {
    CssBogusPseudoClass(CssBogusPseudoClass),
    CssPseudoClassFunctionCompoundSelector(CssPseudoClassFunctionCompoundSelector),
    CssPseudoClassFunctionCompoundSelectorList(CssPseudoClassFunctionCompoundSelectorList),
    CssPseudoClassFunctionCustomIdentifierList(CssPseudoClassFunctionCustomIdentifierList),
    CssPseudoClassFunctionIdentifier(CssPseudoClassFunctionIdentifier),
    CssPseudoClassFunctionNth(CssPseudoClassFunctionNth),
    CssPseudoClassFunctionRelativeSelectorList(CssPseudoClassFunctionRelativeSelectorList),
    CssPseudoClassFunctionSelector(CssPseudoClassFunctionSelector),
    CssPseudoClassFunctionSelectorList(CssPseudoClassFunctionSelectorList),
    CssPseudoClassFunctionValueList(CssPseudoClassFunctionValueList),
    CssPseudoClassIdentifier(CssPseudoClassIdentifier),
}
impl AnyCssPseudoClass {
    pub fn as_css_bogus_pseudo_class(&self) -> Option<&CssBogusPseudoClass> {
        match &self {
            Self::CssBogusPseudoClass(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_function_compound_selector(
        &self,
    ) -> Option<&CssPseudoClassFunctionCompoundSelector> {
        match &self {
            Self::CssPseudoClassFunctionCompoundSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_function_compound_selector_list(
        &self,
    ) -> Option<&CssPseudoClassFunctionCompoundSelectorList> {
        match &self {
            Self::CssPseudoClassFunctionCompoundSelectorList(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_function_custom_identifier_list(
        &self,
    ) -> Option<&CssPseudoClassFunctionCustomIdentifierList> {
        match &self {
            Self::CssPseudoClassFunctionCustomIdentifierList(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_function_identifier(
        &self,
    ) -> Option<&CssPseudoClassFunctionIdentifier> {
        match &self {
            Self::CssPseudoClassFunctionIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_function_nth(&self) -> Option<&CssPseudoClassFunctionNth> {
        match &self {
            Self::CssPseudoClassFunctionNth(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_function_relative_selector_list(
        &self,
    ) -> Option<&CssPseudoClassFunctionRelativeSelectorList> {
        match &self {
            Self::CssPseudoClassFunctionRelativeSelectorList(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_function_selector(&self) -> Option<&CssPseudoClassFunctionSelector> {
        match &self {
            Self::CssPseudoClassFunctionSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_function_selector_list(
        &self,
    ) -> Option<&CssPseudoClassFunctionSelectorList> {
        match &self {
            Self::CssPseudoClassFunctionSelectorList(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_function_value_list(
        &self,
    ) -> Option<&CssPseudoClassFunctionValueList> {
        match &self {
            Self::CssPseudoClassFunctionValueList(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_identifier(&self) -> Option<&CssPseudoClassIdentifier> {
        match &self {
            Self::CssPseudoClassIdentifier(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssPseudoClassNth {
    CssPseudoClassNth(CssPseudoClassNth),
    CssPseudoClassNthIdentifier(CssPseudoClassNthIdentifier),
    CssPseudoClassNthNumber(CssPseudoClassNthNumber),
}
impl AnyCssPseudoClassNth {
    pub fn as_css_pseudo_class_nth(&self) -> Option<&CssPseudoClassNth> {
        match &self {
            Self::CssPseudoClassNth(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_nth_identifier(&self) -> Option<&CssPseudoClassNthIdentifier> {
        match &self {
            Self::CssPseudoClassNthIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_nth_number(&self) -> Option<&CssPseudoClassNthNumber> {
        match &self {
            Self::CssPseudoClassNthNumber(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssPseudoClassNthSelector {
    CssBogusSelector(CssBogusSelector),
    CssPseudoClassNthSelector(CssPseudoClassNthSelector),
}
impl AnyCssPseudoClassNthSelector {
    pub fn as_css_bogus_selector(&self) -> Option<&CssBogusSelector> {
        match &self {
            Self::CssBogusSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_nth_selector(&self) -> Option<&CssPseudoClassNthSelector> {
        match &self {
            Self::CssPseudoClassNthSelector(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssPseudoElement {
    CssBogusPseudoElement(CssBogusPseudoElement),
    CssPseudoElementFunction(CssPseudoElementFunction),
    CssPseudoElementFunctionCustomIdentifier(CssPseudoElementFunctionCustomIdentifier),
    CssPseudoElementFunctionSelector(CssPseudoElementFunctionSelector),
    CssPseudoElementIdentifier(CssPseudoElementIdentifier),
}
impl AnyCssPseudoElement {
    pub fn as_css_bogus_pseudo_element(&self) -> Option<&CssBogusPseudoElement> {
        match &self {
            Self::CssBogusPseudoElement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_element_function(&self) -> Option<&CssPseudoElementFunction> {
        match &self {
            Self::CssPseudoElementFunction(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_element_function_custom_identifier(
        &self,
    ) -> Option<&CssPseudoElementFunctionCustomIdentifier> {
        match &self {
            Self::CssPseudoElementFunctionCustomIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_element_function_selector(
        &self,
    ) -> Option<&CssPseudoElementFunctionSelector> {
        match &self {
            Self::CssPseudoElementFunctionSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_element_identifier(&self) -> Option<&CssPseudoElementIdentifier> {
        match &self {
            Self::CssPseudoElementIdentifier(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssPseudoValue {
    CssIdentifier(CssIdentifier),
    CssString(CssString),
}
impl AnyCssPseudoValue {
    pub fn as_css_identifier(&self) -> Option<&CssIdentifier> {
        match &self {
            Self::CssIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_string(&self) -> Option<&CssString> {
        match &self {
            Self::CssString(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssQueryFeature {
    CssQueryFeatureBoolean(CssQueryFeatureBoolean),
    CssQueryFeaturePlain(CssQueryFeaturePlain),
    CssQueryFeatureRange(CssQueryFeatureRange),
    CssQueryFeatureRangeInterval(CssQueryFeatureRangeInterval),
    CssQueryFeatureReverseRange(CssQueryFeatureReverseRange),
}
impl AnyCssQueryFeature {
    pub fn as_css_query_feature_boolean(&self) -> Option<&CssQueryFeatureBoolean> {
        match &self {
            Self::CssQueryFeatureBoolean(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_query_feature_plain(&self) -> Option<&CssQueryFeaturePlain> {
        match &self {
            Self::CssQueryFeaturePlain(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_query_feature_range(&self) -> Option<&CssQueryFeatureRange> {
        match &self {
            Self::CssQueryFeatureRange(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_query_feature_range_interval(&self) -> Option<&CssQueryFeatureRangeInterval> {
        match &self {
            Self::CssQueryFeatureRangeInterval(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_query_feature_reverse_range(&self) -> Option<&CssQueryFeatureReverseRange> {
        match &self {
            Self::CssQueryFeatureReverseRange(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssQueryFeatureValue {
    AnyCssDimension(AnyCssDimension),
    AnyCssFunction(AnyCssFunction),
    CssIdentifier(CssIdentifier),
    CssNumber(CssNumber),
    CssRatio(CssRatio),
}
impl AnyCssQueryFeatureValue {
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
    pub fn as_css_ratio(&self) -> Option<&CssRatio> {
        match &self {
            Self::CssRatio(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssRelativeSelector {
    CssBogusSelector(CssBogusSelector),
    CssRelativeSelector(CssRelativeSelector),
}
impl AnyCssRelativeSelector {
    pub fn as_css_bogus_selector(&self) -> Option<&CssBogusSelector> {
        match &self {
            Self::CssBogusSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_relative_selector(&self) -> Option<&CssRelativeSelector> {
        match &self {
            Self::CssRelativeSelector(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssRule {
    CssAtRule(CssAtRule),
    CssBogusRule(CssBogusRule),
    CssNestedQualifiedRule(CssNestedQualifiedRule),
    CssQualifiedRule(CssQualifiedRule),
}
impl AnyCssRule {
    pub fn as_css_at_rule(&self) -> Option<&CssAtRule> {
        match &self {
            Self::CssAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_bogus_rule(&self) -> Option<&CssBogusRule> {
        match &self {
            Self::CssBogusRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_nested_qualified_rule(&self) -> Option<&CssNestedQualifiedRule> {
        match &self {
            Self::CssNestedQualifiedRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_qualified_rule(&self) -> Option<&CssQualifiedRule> {
        match &self {
            Self::CssQualifiedRule(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssRuleBlock {
    CssBogusBlock(CssBogusBlock),
    CssRuleBlock(CssRuleBlock),
}
impl AnyCssRuleBlock {
    pub fn as_css_bogus_block(&self) -> Option<&CssBogusBlock> {
        match &self {
            Self::CssBogusBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_rule_block(&self) -> Option<&CssRuleBlock> {
        match &self {
            Self::CssRuleBlock(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssScopeRange {
    CssBogusScopeRange(CssBogusScopeRange),
    CssScopeRangeEnd(CssScopeRangeEnd),
    CssScopeRangeInterval(CssScopeRangeInterval),
    CssScopeRangeStart(CssScopeRangeStart),
}
impl AnyCssScopeRange {
    pub fn as_css_bogus_scope_range(&self) -> Option<&CssBogusScopeRange> {
        match &self {
            Self::CssBogusScopeRange(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_scope_range_end(&self) -> Option<&CssScopeRangeEnd> {
        match &self {
            Self::CssScopeRangeEnd(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_scope_range_interval(&self) -> Option<&CssScopeRangeInterval> {
        match &self {
            Self::CssScopeRangeInterval(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_scope_range_start(&self) -> Option<&CssScopeRangeStart> {
        match &self {
            Self::CssScopeRangeStart(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssSelector {
    CssBogusSelector(CssBogusSelector),
    CssComplexSelector(CssComplexSelector),
    CssCompoundSelector(CssCompoundSelector),
    CssMetavariable(CssMetavariable),
}
impl AnyCssSelector {
    pub fn as_css_bogus_selector(&self) -> Option<&CssBogusSelector> {
        match &self {
            Self::CssBogusSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_complex_selector(&self) -> Option<&CssComplexSelector> {
        match &self {
            Self::CssComplexSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_compound_selector(&self) -> Option<&CssCompoundSelector> {
        match &self {
            Self::CssCompoundSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_metavariable(&self) -> Option<&CssMetavariable> {
        match &self {
            Self::CssMetavariable(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssSimpleSelector {
    CssTypeSelector(CssTypeSelector),
    CssUniversalSelector(CssUniversalSelector),
}
impl AnyCssSimpleSelector {
    pub fn as_css_type_selector(&self) -> Option<&CssTypeSelector> {
        match &self {
            Self::CssTypeSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_universal_selector(&self) -> Option<&CssUniversalSelector> {
        match &self {
            Self::CssUniversalSelector(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
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
            Self::CssAttributeSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_bogus_sub_selector(&self) -> Option<&CssBogusSubSelector> {
        match &self {
            Self::CssBogusSubSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_class_selector(&self) -> Option<&CssClassSelector> {
        match &self {
            Self::CssClassSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_id_selector(&self) -> Option<&CssIdSelector> {
        match &self {
            Self::CssIdSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_selector(&self) -> Option<&CssPseudoClassSelector> {
        match &self {
            Self::CssPseudoClassSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_element_selector(&self) -> Option<&CssPseudoElementSelector> {
        match &self {
            Self::CssPseudoElementSelector(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssSupportsAndCombinableCondition {
    AnyCssSupportsInParens(AnyCssSupportsInParens),
    CssSupportsAndCondition(CssSupportsAndCondition),
}
impl AnyCssSupportsAndCombinableCondition {
    pub fn as_any_css_supports_in_parens(&self) -> Option<&AnyCssSupportsInParens> {
        match &self {
            Self::AnyCssSupportsInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_supports_and_condition(&self) -> Option<&CssSupportsAndCondition> {
        match &self {
            Self::CssSupportsAndCondition(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssSupportsCondition {
    AnyCssSupportsInParens(AnyCssSupportsInParens),
    CssBogusSupportsCondition(CssBogusSupportsCondition),
    CssSupportsAndCondition(CssSupportsAndCondition),
    CssSupportsNotCondition(CssSupportsNotCondition),
    CssSupportsOrCondition(CssSupportsOrCondition),
}
impl AnyCssSupportsCondition {
    pub fn as_any_css_supports_in_parens(&self) -> Option<&AnyCssSupportsInParens> {
        match &self {
            Self::AnyCssSupportsInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_bogus_supports_condition(&self) -> Option<&CssBogusSupportsCondition> {
        match &self {
            Self::CssBogusSupportsCondition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_supports_and_condition(&self) -> Option<&CssSupportsAndCondition> {
        match &self {
            Self::CssSupportsAndCondition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_supports_not_condition(&self) -> Option<&CssSupportsNotCondition> {
        match &self {
            Self::CssSupportsNotCondition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_supports_or_condition(&self) -> Option<&CssSupportsOrCondition> {
        match &self {
            Self::CssSupportsOrCondition(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssSupportsInParens {
    AnyCssValue(AnyCssValue),
    CssSupportsConditionInParens(CssSupportsConditionInParens),
    CssSupportsFeatureDeclaration(CssSupportsFeatureDeclaration),
    CssSupportsFeatureSelector(CssSupportsFeatureSelector),
}
impl AnyCssSupportsInParens {
    pub fn as_any_css_value(&self) -> Option<&AnyCssValue> {
        match &self {
            Self::AnyCssValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_supports_condition_in_parens(&self) -> Option<&CssSupportsConditionInParens> {
        match &self {
            Self::CssSupportsConditionInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_supports_feature_declaration(&self) -> Option<&CssSupportsFeatureDeclaration> {
        match &self {
            Self::CssSupportsFeatureDeclaration(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_supports_feature_selector(&self) -> Option<&CssSupportsFeatureSelector> {
        match &self {
            Self::CssSupportsFeatureSelector(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssSupportsOrCombinableCondition {
    AnyCssSupportsInParens(AnyCssSupportsInParens),
    CssSupportsOrCondition(CssSupportsOrCondition),
}
impl AnyCssSupportsOrCombinableCondition {
    pub fn as_any_css_supports_in_parens(&self) -> Option<&AnyCssSupportsInParens> {
        match &self {
            Self::AnyCssSupportsInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_supports_or_condition(&self) -> Option<&CssSupportsOrCondition> {
        match &self {
            Self::CssSupportsOrCondition(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssUnicodeValue {
    CssBogusUnicodeRangeValue(CssBogusUnicodeRangeValue),
    CssUnicodeCodepoint(CssUnicodeCodepoint),
    CssUnicodeRangeInterval(CssUnicodeRangeInterval),
    CssUnicodeRangeWildcard(CssUnicodeRangeWildcard),
}
impl AnyCssUnicodeValue {
    pub fn as_css_bogus_unicode_range_value(&self) -> Option<&CssBogusUnicodeRangeValue> {
        match &self {
            Self::CssBogusUnicodeRangeValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_unicode_codepoint(&self) -> Option<&CssUnicodeCodepoint> {
        match &self {
            Self::CssUnicodeCodepoint(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_unicode_range_interval(&self) -> Option<&CssUnicodeRangeInterval> {
        match &self {
            Self::CssUnicodeRangeInterval(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_unicode_range_wildcard(&self) -> Option<&CssUnicodeRangeWildcard> {
        match &self {
            Self::CssUnicodeRangeWildcard(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssUrlModifier {
    CssBogusUrlModifier(CssBogusUrlModifier),
    CssFunction(CssFunction),
    CssIdentifier(CssIdentifier),
}
impl AnyCssUrlModifier {
    pub fn as_css_bogus_url_modifier(&self) -> Option<&CssBogusUrlModifier> {
        match &self {
            Self::CssBogusUrlModifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_function(&self) -> Option<&CssFunction> {
        match &self {
            Self::CssFunction(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_identifier(&self) -> Option<&CssIdentifier> {
        match &self {
            Self::CssIdentifier(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssUrlValue {
    CssString(CssString),
    CssUrlValueRaw(CssUrlValueRaw),
}
impl AnyCssUrlValue {
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
    CssBracketedValue(CssBracketedValue),
    CssColor(CssColor),
    CssCustomIdentifier(CssCustomIdentifier),
    CssDashedIdentifier(CssDashedIdentifier),
    CssIdentifier(CssIdentifier),
    CssMetavariable(CssMetavariable),
    CssNumber(CssNumber),
    CssRatio(CssRatio),
    CssString(CssString),
    CssUnicodeRange(CssUnicodeRange),
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
    pub fn as_css_bracketed_value(&self) -> Option<&CssBracketedValue> {
        match &self {
            Self::CssBracketedValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_color(&self) -> Option<&CssColor> {
        match &self {
            Self::CssColor(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_custom_identifier(&self) -> Option<&CssCustomIdentifier> {
        match &self {
            Self::CssCustomIdentifier(item) => Some(item),
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
    pub fn as_css_metavariable(&self) -> Option<&CssMetavariable> {
        match &self {
            Self::CssMetavariable(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_number(&self) -> Option<&CssNumber> {
        match &self {
            Self::CssNumber(item) => Some(item),
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
    pub fn as_css_unicode_range(&self) -> Option<&CssUnicodeRange> {
        match &self {
            Self::CssUnicodeRange(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssValueAtRuleClause {
    CssValueAtRuleDeclarationClause(CssValueAtRuleDeclarationClause),
    CssValueAtRuleImportClause(CssValueAtRuleImportClause),
}
impl AnyCssValueAtRuleClause {
    pub fn as_css_value_at_rule_declaration_clause(
        &self,
    ) -> Option<&CssValueAtRuleDeclarationClause> {
        match &self {
            Self::CssValueAtRuleDeclarationClause(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_value_at_rule_import_clause(&self) -> Option<&CssValueAtRuleImportClause> {
        match &self {
            Self::CssValueAtRuleImportClause(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssValueAtRuleImportSource {
    CssIdentifier(CssIdentifier),
    CssString(CssString),
}
impl AnyCssValueAtRuleImportSource {
    pub fn as_css_identifier(&self) -> Option<&CssIdentifier> {
        match &self {
            Self::CssIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_string(&self) -> Option<&CssString> {
        match &self {
            Self::CssString(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssValueAtRuleImportSpecifier {
    CssValueAtRuleImportSpecifier(CssValueAtRuleImportSpecifier),
    CssValueAtRuleNamedImportSpecifier(CssValueAtRuleNamedImportSpecifier),
}
impl AnyCssValueAtRuleImportSpecifier {
    pub fn as_css_value_at_rule_import_specifier(&self) -> Option<&CssValueAtRuleImportSpecifier> {
        match &self {
            Self::CssValueAtRuleImportSpecifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_value_at_rule_named_import_specifier(
        &self,
    ) -> Option<&CssValueAtRuleNamedImportSpecifier> {
        match &self {
            Self::CssValueAtRuleNamedImportSpecifier(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyCssValueAtRuleProperty {
    CssBogusProperty(CssBogusProperty),
    CssValueAtRuleGenericProperty(CssValueAtRuleGenericProperty),
}
impl AnyCssValueAtRuleProperty {
    pub fn as_css_bogus_property(&self) -> Option<&CssBogusProperty> {
        match &self {
            Self::CssBogusProperty(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_value_at_rule_generic_property(&self) -> Option<&CssValueAtRuleGenericProperty> {
        match &self {
            Self::CssValueAtRuleGenericProperty(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for CssAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssAtRule")
                .field("at_token", &support::DebugSyntaxResult(self.at_token()))
                .field("rule", &support::DebugSyntaxResult(self.rule()))
                .finish()
        } else {
            f.debug_struct("CssAtRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssAtRule> for SyntaxNode {
    fn from(n: CssAtRule) -> Self {
        n.syntax
    }
}
impl From<CssAtRule> for SyntaxElement {
    fn from(n: CssAtRule) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssAttributeMatcher")
                .field("operator", &support::DebugSyntaxResult(self.operator()))
                .field("value", &support::DebugSyntaxResult(self.value()))
                .field("modifier", &support::DebugOptionalElement(self.modifier()))
                .finish()
        } else {
            f.debug_struct("CssAttributeMatcher").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssAttributeMatcher> for SyntaxNode {
    fn from(n: CssAttributeMatcher) -> Self {
        n.syntax
    }
}
impl From<CssAttributeMatcher> for SyntaxElement {
    fn from(n: CssAttributeMatcher) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssAttributeMatcherValue")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("CssAttributeMatcherValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssAttributeMatcherValue> for SyntaxNode {
    fn from(n: CssAttributeMatcherValue) -> Self {
        n.syntax
    }
}
impl From<CssAttributeMatcherValue> for SyntaxElement {
    fn from(n: CssAttributeMatcherValue) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssAttributeName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_ATTRIBUTE_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_ATTRIBUTE_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssAttributeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssAttributeName")
                .field(
                    "namespace",
                    &support::DebugOptionalElement(self.namespace()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("CssAttributeName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssAttributeName> for SyntaxNode {
    fn from(n: CssAttributeName) -> Self {
        n.syntax
    }
}
impl From<CssAttributeName> for SyntaxElement {
    fn from(n: CssAttributeName) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
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
        } else {
            f.debug_struct("CssAttributeSelector").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssAttributeSelector> for SyntaxNode {
    fn from(n: CssAttributeSelector) -> Self {
        n.syntax
    }
}
impl From<CssAttributeSelector> for SyntaxElement {
    fn from(n: CssAttributeSelector) -> Self {
        n.syntax.into()
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssBinaryExpression")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field(
                    "operator_token",
                    &support::DebugSyntaxResult(self.operator_token()),
                )
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("CssBinaryExpression").finish()
        };
        DEPTH.set(current_depth);
        result
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
impl AstNode for CssBracketedValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BRACKETED_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BRACKETED_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBracketedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssBracketedValue")
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("items", &self.items())
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssBracketedValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssBracketedValue> for SyntaxNode {
    fn from(n: CssBracketedValue) -> Self {
        n.syntax
    }
}
impl From<CssBracketedValue> for SyntaxElement {
    fn from(n: CssBracketedValue) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssCharsetAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CHARSET_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CHARSET_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssCharsetAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssCharsetAtRule")
                .field(
                    "charset_token",
                    &support::DebugSyntaxResult(self.charset_token()),
                )
                .field("encoding", &support::DebugSyntaxResult(self.encoding()))
                .field(
                    "semicolon_token",
                    &support::DebugSyntaxResult(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssCharsetAtRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssCharsetAtRule> for SyntaxNode {
    fn from(n: CssCharsetAtRule) -> Self {
        n.syntax
    }
}
impl From<CssCharsetAtRule> for SyntaxElement {
    fn from(n: CssCharsetAtRule) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssClassSelector")
                .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("CssClassSelector").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssClassSelector> for SyntaxNode {
    fn from(n: CssClassSelector) -> Self {
        n.syntax
    }
}
impl From<CssClassSelector> for SyntaxElement {
    fn from(n: CssClassSelector) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssColor")
                .field("hash_token", &support::DebugSyntaxResult(self.hash_token()))
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssColor").finish()
        };
        DEPTH.set(current_depth);
        result
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
impl AstNode for CssColorProfileAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_COLOR_PROFILE_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_COLOR_PROFILE_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssColorProfileAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssColorProfileAtRule")
                .field(
                    "color_profile_token",
                    &support::DebugSyntaxResult(self.color_profile_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("block", &support::DebugSyntaxResult(self.block()))
                .finish()
        } else {
            f.debug_struct("CssColorProfileAtRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssColorProfileAtRule> for SyntaxNode {
    fn from(n: CssColorProfileAtRule) -> Self {
        n.syntax
    }
}
impl From<CssColorProfileAtRule> for SyntaxElement {
    fn from(n: CssColorProfileAtRule) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssComplexSelector")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field("combinator", &support::DebugSyntaxResult(self.combinator()))
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("CssComplexSelector").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssComplexSelector> for SyntaxNode {
    fn from(n: CssComplexSelector) -> Self {
        n.syntax
    }
}
impl From<CssComplexSelector> for SyntaxElement {
    fn from(n: CssComplexSelector) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssComposesImportSpecifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_COMPOSES_IMPORT_SPECIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_COMPOSES_IMPORT_SPECIFIER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssComposesImportSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssComposesImportSpecifier")
                .field("from_token", &support::DebugSyntaxResult(self.from_token()))
                .field("source", &support::DebugSyntaxResult(self.source()))
                .finish()
        } else {
            f.debug_struct("CssComposesImportSpecifier").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssComposesImportSpecifier> for SyntaxNode {
    fn from(n: CssComposesImportSpecifier) -> Self {
        n.syntax
    }
}
impl From<CssComposesImportSpecifier> for SyntaxElement {
    fn from(n: CssComposesImportSpecifier) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssComposesProperty {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_COMPOSES_PROPERTY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_COMPOSES_PROPERTY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssComposesProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssComposesProperty")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("CssComposesProperty").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssComposesProperty> for SyntaxNode {
    fn from(n: CssComposesProperty) -> Self {
        n.syntax
    }
}
impl From<CssComposesProperty> for SyntaxElement {
    fn from(n: CssComposesProperty) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssComposesPropertyValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_COMPOSES_PROPERTY_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_COMPOSES_PROPERTY_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssComposesPropertyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssComposesPropertyValue")
                .field("classes", &self.classes())
                .field(
                    "specifier",
                    &support::DebugOptionalElement(self.specifier()),
                )
                .finish()
        } else {
            f.debug_struct("CssComposesPropertyValue").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssComposesPropertyValue> for SyntaxNode {
    fn from(n: CssComposesPropertyValue) -> Self {
        n.syntax
    }
}
impl From<CssComposesPropertyValue> for SyntaxElement {
    fn from(n: CssComposesPropertyValue) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssCompoundSelector")
                .field("nesting_selectors", &self.nesting_selectors())
                .field(
                    "simple_selector",
                    &support::DebugOptionalElement(self.simple_selector()),
                )
                .field("sub_selectors", &self.sub_selectors())
                .finish()
        } else {
            f.debug_struct("CssCompoundSelector").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssCompoundSelector> for SyntaxNode {
    fn from(n: CssCompoundSelector) -> Self {
        n.syntax
    }
}
impl From<CssCompoundSelector> for SyntaxElement {
    fn from(n: CssCompoundSelector) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssContainerAndQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CONTAINER_AND_QUERY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CONTAINER_AND_QUERY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssContainerAndQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssContainerAndQuery")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field("and_token", &support::DebugSyntaxResult(self.and_token()))
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("CssContainerAndQuery").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssContainerAndQuery> for SyntaxNode {
    fn from(n: CssContainerAndQuery) -> Self {
        n.syntax
    }
}
impl From<CssContainerAndQuery> for SyntaxElement {
    fn from(n: CssContainerAndQuery) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssContainerAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CONTAINER_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CONTAINER_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssContainerAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssContainerAtRule")
                .field(
                    "container_token",
                    &support::DebugSyntaxResult(self.container_token()),
                )
                .field("name", &support::DebugOptionalElement(self.name()))
                .field("query", &support::DebugSyntaxResult(self.query()))
                .field("block", &support::DebugSyntaxResult(self.block()))
                .finish()
        } else {
            f.debug_struct("CssContainerAtRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssContainerAtRule> for SyntaxNode {
    fn from(n: CssContainerAtRule) -> Self {
        n.syntax
    }
}
impl From<CssContainerAtRule> for SyntaxElement {
    fn from(n: CssContainerAtRule) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssContainerNotQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CONTAINER_NOT_QUERY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CONTAINER_NOT_QUERY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssContainerNotQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssContainerNotQuery")
                .field("not_token", &support::DebugSyntaxResult(self.not_token()))
                .field("query", &support::DebugSyntaxResult(self.query()))
                .finish()
        } else {
            f.debug_struct("CssContainerNotQuery").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssContainerNotQuery> for SyntaxNode {
    fn from(n: CssContainerNotQuery) -> Self {
        n.syntax
    }
}
impl From<CssContainerNotQuery> for SyntaxElement {
    fn from(n: CssContainerNotQuery) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssContainerOrQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CONTAINER_OR_QUERY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CONTAINER_OR_QUERY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssContainerOrQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssContainerOrQuery")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field("or_token", &support::DebugSyntaxResult(self.or_token()))
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("CssContainerOrQuery").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssContainerOrQuery> for SyntaxNode {
    fn from(n: CssContainerOrQuery) -> Self {
        n.syntax
    }
}
impl From<CssContainerOrQuery> for SyntaxElement {
    fn from(n: CssContainerOrQuery) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssContainerQueryInParens {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CONTAINER_QUERY_IN_PARENS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CONTAINER_QUERY_IN_PARENS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssContainerQueryInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssContainerQueryInParens")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("query", &support::DebugSyntaxResult(self.query()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssContainerQueryInParens").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssContainerQueryInParens> for SyntaxNode {
    fn from(n: CssContainerQueryInParens) -> Self {
        n.syntax
    }
}
impl From<CssContainerQueryInParens> for SyntaxElement {
    fn from(n: CssContainerQueryInParens) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssContainerSizeFeatureInParens {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CONTAINER_SIZE_FEATURE_IN_PARENS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CONTAINER_SIZE_FEATURE_IN_PARENS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssContainerSizeFeatureInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssContainerSizeFeatureInParens")
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
        } else {
            f.debug_struct("CssContainerSizeFeatureInParens").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssContainerSizeFeatureInParens> for SyntaxNode {
    fn from(n: CssContainerSizeFeatureInParens) -> Self {
        n.syntax
    }
}
impl From<CssContainerSizeFeatureInParens> for SyntaxElement {
    fn from(n: CssContainerSizeFeatureInParens) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssContainerStyleAndQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CONTAINER_STYLE_AND_QUERY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CONTAINER_STYLE_AND_QUERY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssContainerStyleAndQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssContainerStyleAndQuery")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field("and_token", &support::DebugSyntaxResult(self.and_token()))
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("CssContainerStyleAndQuery").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssContainerStyleAndQuery> for SyntaxNode {
    fn from(n: CssContainerStyleAndQuery) -> Self {
        n.syntax
    }
}
impl From<CssContainerStyleAndQuery> for SyntaxElement {
    fn from(n: CssContainerStyleAndQuery) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssContainerStyleInParens {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CONTAINER_STYLE_IN_PARENS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CONTAINER_STYLE_IN_PARENS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssContainerStyleInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssContainerStyleInParens")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("query", &support::DebugSyntaxResult(self.query()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssContainerStyleInParens").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssContainerStyleInParens> for SyntaxNode {
    fn from(n: CssContainerStyleInParens) -> Self {
        n.syntax
    }
}
impl From<CssContainerStyleInParens> for SyntaxElement {
    fn from(n: CssContainerStyleInParens) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssContainerStyleNotQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CONTAINER_STYLE_NOT_QUERY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CONTAINER_STYLE_NOT_QUERY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssContainerStyleNotQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssContainerStyleNotQuery")
                .field("not_token", &support::DebugSyntaxResult(self.not_token()))
                .field("query", &support::DebugSyntaxResult(self.query()))
                .finish()
        } else {
            f.debug_struct("CssContainerStyleNotQuery").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssContainerStyleNotQuery> for SyntaxNode {
    fn from(n: CssContainerStyleNotQuery) -> Self {
        n.syntax
    }
}
impl From<CssContainerStyleNotQuery> for SyntaxElement {
    fn from(n: CssContainerStyleNotQuery) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssContainerStyleOrQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CONTAINER_STYLE_OR_QUERY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CONTAINER_STYLE_OR_QUERY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssContainerStyleOrQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssContainerStyleOrQuery")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field("or_token", &support::DebugSyntaxResult(self.or_token()))
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("CssContainerStyleOrQuery").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssContainerStyleOrQuery> for SyntaxNode {
    fn from(n: CssContainerStyleOrQuery) -> Self {
        n.syntax
    }
}
impl From<CssContainerStyleOrQuery> for SyntaxElement {
    fn from(n: CssContainerStyleOrQuery) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssContainerStyleQueryInParens {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CONTAINER_STYLE_QUERY_IN_PARENS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CONTAINER_STYLE_QUERY_IN_PARENS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssContainerStyleQueryInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssContainerStyleQueryInParens")
                .field(
                    "style_token",
                    &support::DebugSyntaxResult(self.style_token()),
                )
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("query", &support::DebugSyntaxResult(self.query()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssContainerStyleQueryInParens").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssContainerStyleQueryInParens> for SyntaxNode {
    fn from(n: CssContainerStyleQueryInParens) -> Self {
        n.syntax
    }
}
impl From<CssContainerStyleQueryInParens> for SyntaxElement {
    fn from(n: CssContainerStyleQueryInParens) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssCounterStyleAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_COUNTER_STYLE_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_COUNTER_STYLE_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssCounterStyleAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssCounterStyleAtRule")
                .field(
                    "counter_style_token",
                    &support::DebugSyntaxResult(self.counter_style_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("block", &support::DebugSyntaxResult(self.block()))
                .finish()
        } else {
            f.debug_struct("CssCounterStyleAtRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssCounterStyleAtRule> for SyntaxNode {
    fn from(n: CssCounterStyleAtRule) -> Self {
        n.syntax
    }
}
impl From<CssCounterStyleAtRule> for SyntaxElement {
    fn from(n: CssCounterStyleAtRule) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssCustomIdentifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CUSTOM_IDENTIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CUSTOM_IDENTIFIER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssCustomIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssCustomIdentifier")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssCustomIdentifier").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssCustomIdentifier> for SyntaxNode {
    fn from(n: CssCustomIdentifier) -> Self {
        n.syntax
    }
}
impl From<CssCustomIdentifier> for SyntaxElement {
    fn from(n: CssCustomIdentifier) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssDashedIdentifier")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssDashedIdentifier").finish()
        };
        DEPTH.set(current_depth);
        result
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssDeclaration")
                .field("property", &support::DebugSyntaxResult(self.property()))
                .field(
                    "important",
                    &support::DebugOptionalElement(self.important()),
                )
                .finish()
        } else {
            f.debug_struct("CssDeclaration").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssDeclaration> for SyntaxNode {
    fn from(n: CssDeclaration) -> Self {
        n.syntax
    }
}
impl From<CssDeclaration> for SyntaxElement {
    fn from(n: CssDeclaration) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssDeclarationBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_DECLARATION_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_DECLARATION_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssDeclarationBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssDeclarationBlock")
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
        } else {
            f.debug_struct("CssDeclarationBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssDeclarationBlock> for SyntaxNode {
    fn from(n: CssDeclarationBlock) -> Self {
        n.syntax
    }
}
impl From<CssDeclarationBlock> for SyntaxElement {
    fn from(n: CssDeclarationBlock) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssDeclarationImportant")
                .field("excl_token", &support::DebugSyntaxResult(self.excl_token()))
                .field(
                    "important_token",
                    &support::DebugSyntaxResult(self.important_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssDeclarationImportant").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssDeclarationImportant> for SyntaxNode {
    fn from(n: CssDeclarationImportant) -> Self {
        n.syntax
    }
}
impl From<CssDeclarationImportant> for SyntaxElement {
    fn from(n: CssDeclarationImportant) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssDeclarationOrAtRuleBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_DECLARATION_OR_AT_RULE_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_DECLARATION_OR_AT_RULE_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssDeclarationOrAtRuleBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssDeclarationOrAtRuleBlock")
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
        } else {
            f.debug_struct("CssDeclarationOrAtRuleBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssDeclarationOrAtRuleBlock> for SyntaxNode {
    fn from(n: CssDeclarationOrAtRuleBlock) -> Self {
        n.syntax
    }
}
impl From<CssDeclarationOrAtRuleBlock> for SyntaxElement {
    fn from(n: CssDeclarationOrAtRuleBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssDeclarationOrRuleBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_DECLARATION_OR_RULE_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_DECLARATION_OR_RULE_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssDeclarationOrRuleBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssDeclarationOrRuleBlock")
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
        } else {
            f.debug_struct("CssDeclarationOrRuleBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssDeclarationOrRuleBlock> for SyntaxNode {
    fn from(n: CssDeclarationOrRuleBlock) -> Self {
        n.syntax
    }
}
impl From<CssDeclarationOrRuleBlock> for SyntaxElement {
    fn from(n: CssDeclarationOrRuleBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssDeclarationWithSemicolon {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_DECLARATION_WITH_SEMICOLON as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_DECLARATION_WITH_SEMICOLON
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssDeclarationWithSemicolon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssDeclarationWithSemicolon")
                .field(
                    "declaration",
                    &support::DebugSyntaxResult(self.declaration()),
                )
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssDeclarationWithSemicolon").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssDeclarationWithSemicolon> for SyntaxNode {
    fn from(n: CssDeclarationWithSemicolon) -> Self {
        n.syntax
    }
}
impl From<CssDeclarationWithSemicolon> for SyntaxElement {
    fn from(n: CssDeclarationWithSemicolon) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssDocumentAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_DOCUMENT_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_DOCUMENT_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssDocumentAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssDocumentAtRule")
                .field(
                    "document_token",
                    &support::DebugSyntaxResult(self.document_token()),
                )
                .field("matchers", &self.matchers())
                .field("block", &support::DebugSyntaxResult(self.block()))
                .finish()
        } else {
            f.debug_struct("CssDocumentAtRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssDocumentAtRule> for SyntaxNode {
    fn from(n: CssDocumentAtRule) -> Self {
        n.syntax
    }
}
impl From<CssDocumentAtRule> for SyntaxElement {
    fn from(n: CssDocumentAtRule) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssDocumentCustomMatcher {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_DOCUMENT_CUSTOM_MATCHER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_DOCUMENT_CUSTOM_MATCHER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssDocumentCustomMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssDocumentCustomMatcher")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("value", &support::DebugOptionalElement(self.value()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssDocumentCustomMatcher").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssDocumentCustomMatcher> for SyntaxNode {
    fn from(n: CssDocumentCustomMatcher) -> Self {
        n.syntax
    }
}
impl From<CssDocumentCustomMatcher> for SyntaxElement {
    fn from(n: CssDocumentCustomMatcher) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssEmptyDeclaration {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_EMPTY_DECLARATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_EMPTY_DECLARATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssEmptyDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssEmptyDeclaration")
                .field(
                    "semicolon_token",
                    &support::DebugSyntaxResult(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssEmptyDeclaration").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssEmptyDeclaration> for SyntaxNode {
    fn from(n: CssEmptyDeclaration) -> Self {
        n.syntax
    }
}
impl From<CssEmptyDeclaration> for SyntaxElement {
    fn from(n: CssEmptyDeclaration) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssFontFaceAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_FONT_FACE_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_FONT_FACE_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssFontFaceAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssFontFaceAtRule")
                .field(
                    "font_face_token",
                    &support::DebugSyntaxResult(self.font_face_token()),
                )
                .field("block", &support::DebugSyntaxResult(self.block()))
                .finish()
        } else {
            f.debug_struct("CssFontFaceAtRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssFontFaceAtRule> for SyntaxNode {
    fn from(n: CssFontFaceAtRule) -> Self {
        n.syntax
    }
}
impl From<CssFontFaceAtRule> for SyntaxElement {
    fn from(n: CssFontFaceAtRule) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssFontFamilyName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_FONT_FAMILY_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_FONT_FAMILY_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssFontFamilyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssFontFamilyName")
                .field("names", &self.names())
                .finish()
        } else {
            f.debug_struct("CssFontFamilyName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssFontFamilyName> for SyntaxNode {
    fn from(n: CssFontFamilyName) -> Self {
        n.syntax
    }
}
impl From<CssFontFamilyName> for SyntaxElement {
    fn from(n: CssFontFamilyName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssFontFeatureValuesAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_FONT_FEATURE_VALUES_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_FONT_FEATURE_VALUES_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssFontFeatureValuesAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssFontFeatureValuesAtRule")
                .field(
                    "font_feature_values_token",
                    &support::DebugSyntaxResult(self.font_feature_values_token()),
                )
                .field("names", &self.names())
                .field("block", &support::DebugSyntaxResult(self.block()))
                .finish()
        } else {
            f.debug_struct("CssFontFeatureValuesAtRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssFontFeatureValuesAtRule> for SyntaxNode {
    fn from(n: CssFontFeatureValuesAtRule) -> Self {
        n.syntax
    }
}
impl From<CssFontFeatureValuesAtRule> for SyntaxElement {
    fn from(n: CssFontFeatureValuesAtRule) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssFontFeatureValuesBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_FONT_FEATURE_VALUES_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_FONT_FEATURE_VALUES_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssFontFeatureValuesBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssFontFeatureValuesBlock")
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
        } else {
            f.debug_struct("CssFontFeatureValuesBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssFontFeatureValuesBlock> for SyntaxNode {
    fn from(n: CssFontFeatureValuesBlock) -> Self {
        n.syntax
    }
}
impl From<CssFontFeatureValuesBlock> for SyntaxElement {
    fn from(n: CssFontFeatureValuesBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssFontFeatureValuesItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_FONT_FEATURE_VALUES_ITEM as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_FONT_FEATURE_VALUES_ITEM
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssFontFeatureValuesItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssFontFeatureValuesItem")
                .field("at_token", &support::DebugSyntaxResult(self.at_token()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("block", &support::DebugSyntaxResult(self.block()))
                .finish()
        } else {
            f.debug_struct("CssFontFeatureValuesItem").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssFontFeatureValuesItem> for SyntaxNode {
    fn from(n: CssFontFeatureValuesItem) -> Self {
        n.syntax
    }
}
impl From<CssFontFeatureValuesItem> for SyntaxElement {
    fn from(n: CssFontFeatureValuesItem) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssFontPaletteValuesAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_FONT_PALETTE_VALUES_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_FONT_PALETTE_VALUES_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssFontPaletteValuesAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssFontPaletteValuesAtRule")
                .field(
                    "font_palette_values_token",
                    &support::DebugSyntaxResult(self.font_palette_values_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("block", &support::DebugSyntaxResult(self.block()))
                .finish()
        } else {
            f.debug_struct("CssFontPaletteValuesAtRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssFontPaletteValuesAtRule> for SyntaxNode {
    fn from(n: CssFontPaletteValuesAtRule) -> Self {
        n.syntax
    }
}
impl From<CssFontPaletteValuesAtRule> for SyntaxElement {
    fn from(n: CssFontPaletteValuesAtRule) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssFunction")
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
        } else {
            f.debug_struct("CssFunction").finish()
        };
        DEPTH.set(current_depth);
        result
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssGenericDelimiter")
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("CssGenericDelimiter").finish()
        };
        DEPTH.set(current_depth);
        result
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
impl AstNode for CssGenericProperty {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_GENERIC_PROPERTY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_GENERIC_PROPERTY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssGenericProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssGenericProperty")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field("value", &self.value())
                .finish()
        } else {
            f.debug_struct("CssGenericProperty").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssGenericProperty> for SyntaxNode {
    fn from(n: CssGenericProperty) -> Self {
        n.syntax
    }
}
impl From<CssGenericProperty> for SyntaxElement {
    fn from(n: CssGenericProperty) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssIdSelector")
                .field("hash_token", &support::DebugSyntaxResult(self.hash_token()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("CssIdSelector").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssIdSelector> for SyntaxNode {
    fn from(n: CssIdSelector) -> Self {
        n.syntax
    }
}
impl From<CssIdSelector> for SyntaxElement {
    fn from(n: CssIdSelector) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssIdentifier")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssIdentifier").finish()
        };
        DEPTH.set(current_depth);
        result
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
impl AstNode for CssImportAnonymousLayer {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_IMPORT_ANONYMOUS_LAYER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_IMPORT_ANONYMOUS_LAYER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssImportAnonymousLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssImportAnonymousLayer")
                .field(
                    "layer_token",
                    &support::DebugSyntaxResult(self.layer_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssImportAnonymousLayer").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssImportAnonymousLayer> for SyntaxNode {
    fn from(n: CssImportAnonymousLayer) -> Self {
        n.syntax
    }
}
impl From<CssImportAnonymousLayer> for SyntaxElement {
    fn from(n: CssImportAnonymousLayer) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssImportAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_IMPORT_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_IMPORT_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssImportAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssImportAtRule")
                .field(
                    "import_token",
                    &support::DebugSyntaxResult(self.import_token()),
                )
                .field("url", &support::DebugSyntaxResult(self.url()))
                .field("layer", &support::DebugOptionalElement(self.layer()))
                .field("supports", &support::DebugOptionalElement(self.supports()))
                .field("media", &self.media())
                .field(
                    "semicolon_token",
                    &support::DebugSyntaxResult(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssImportAtRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssImportAtRule> for SyntaxNode {
    fn from(n: CssImportAtRule) -> Self {
        n.syntax
    }
}
impl From<CssImportAtRule> for SyntaxElement {
    fn from(n: CssImportAtRule) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssImportNamedLayer {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_IMPORT_NAMED_LAYER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_IMPORT_NAMED_LAYER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssImportNamedLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssImportNamedLayer")
                .field(
                    "layer_token",
                    &support::DebugSyntaxResult(self.layer_token()),
                )
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("name", &self.name())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssImportNamedLayer").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssImportNamedLayer> for SyntaxNode {
    fn from(n: CssImportNamedLayer) -> Self {
        n.syntax
    }
}
impl From<CssImportNamedLayer> for SyntaxElement {
    fn from(n: CssImportNamedLayer) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssImportSupports {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_IMPORT_SUPPORTS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_IMPORT_SUPPORTS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssImportSupports {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssImportSupports")
                .field(
                    "supports_token",
                    &support::DebugSyntaxResult(self.supports_token()),
                )
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("condition", &support::DebugSyntaxResult(self.condition()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssImportSupports").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssImportSupports> for SyntaxNode {
    fn from(n: CssImportSupports) -> Self {
        n.syntax
    }
}
impl From<CssImportSupports> for SyntaxElement {
    fn from(n: CssImportSupports) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssKeyframesAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_KEYFRAMES_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_KEYFRAMES_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssKeyframesAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssKeyframesAtRule")
                .field(
                    "keyframes_token",
                    &support::DebugSyntaxResult(self.keyframes_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("block", &support::DebugSyntaxResult(self.block()))
                .finish()
        } else {
            f.debug_struct("CssKeyframesAtRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssKeyframesAtRule> for SyntaxNode {
    fn from(n: CssKeyframesAtRule) -> Self {
        n.syntax
    }
}
impl From<CssKeyframesAtRule> for SyntaxElement {
    fn from(n: CssKeyframesAtRule) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssKeyframesBlock")
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
        } else {
            f.debug_struct("CssKeyframesBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssKeyframesBlock> for SyntaxNode {
    fn from(n: CssKeyframesBlock) -> Self {
        n.syntax
    }
}
impl From<CssKeyframesBlock> for SyntaxElement {
    fn from(n: CssKeyframesBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssKeyframesIdentSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_KEYFRAMES_IDENT_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_KEYFRAMES_IDENT_SELECTOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssKeyframesIdentSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssKeyframesIdentSelector")
                .field("selector", &support::DebugSyntaxResult(self.selector()))
                .finish()
        } else {
            f.debug_struct("CssKeyframesIdentSelector").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssKeyframesIdentSelector> for SyntaxNode {
    fn from(n: CssKeyframesIdentSelector) -> Self {
        n.syntax
    }
}
impl From<CssKeyframesIdentSelector> for SyntaxElement {
    fn from(n: CssKeyframesIdentSelector) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssKeyframesItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_KEYFRAMES_ITEM as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_KEYFRAMES_ITEM
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssKeyframesItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssKeyframesItem")
                .field("selectors", &self.selectors())
                .field("block", &support::DebugSyntaxResult(self.block()))
                .finish()
        } else {
            f.debug_struct("CssKeyframesItem").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssKeyframesItem> for SyntaxNode {
    fn from(n: CssKeyframesItem) -> Self {
        n.syntax
    }
}
impl From<CssKeyframesItem> for SyntaxElement {
    fn from(n: CssKeyframesItem) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssKeyframesPercentageSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_KEYFRAMES_PERCENTAGE_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_KEYFRAMES_PERCENTAGE_SELECTOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssKeyframesPercentageSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssKeyframesPercentageSelector")
                .field("selector", &support::DebugSyntaxResult(self.selector()))
                .finish()
        } else {
            f.debug_struct("CssKeyframesPercentageSelector").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssKeyframesPercentageSelector> for SyntaxNode {
    fn from(n: CssKeyframesPercentageSelector) -> Self {
        n.syntax
    }
}
impl From<CssKeyframesPercentageSelector> for SyntaxElement {
    fn from(n: CssKeyframesPercentageSelector) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssKeyframesScopeFunction {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_KEYFRAMES_SCOPE_FUNCTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_KEYFRAMES_SCOPE_FUNCTION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssKeyframesScopeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssKeyframesScopeFunction")
                .field("scope", &support::DebugSyntaxResult(self.scope()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssKeyframesScopeFunction").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssKeyframesScopeFunction> for SyntaxNode {
    fn from(n: CssKeyframesScopeFunction) -> Self {
        n.syntax
    }
}
impl From<CssKeyframesScopeFunction> for SyntaxElement {
    fn from(n: CssKeyframesScopeFunction) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssKeyframesScopePrefix {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_KEYFRAMES_SCOPE_PREFIX as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_KEYFRAMES_SCOPE_PREFIX
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssKeyframesScopePrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssKeyframesScopePrefix")
                .field("scope", &support::DebugSyntaxResult(self.scope()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("CssKeyframesScopePrefix").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssKeyframesScopePrefix> for SyntaxNode {
    fn from(n: CssKeyframesScopePrefix) -> Self {
        n.syntax
    }
}
impl From<CssKeyframesScopePrefix> for SyntaxElement {
    fn from(n: CssKeyframesScopePrefix) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssKeyframesScopedName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_KEYFRAMES_SCOPED_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_KEYFRAMES_SCOPED_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssKeyframesScopedName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssKeyframesScopedName")
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field("scope", &support::DebugSyntaxResult(self.scope()))
                .finish()
        } else {
            f.debug_struct("CssKeyframesScopedName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssKeyframesScopedName> for SyntaxNode {
    fn from(n: CssKeyframesScopedName) -> Self {
        n.syntax
    }
}
impl From<CssKeyframesScopedName> for SyntaxElement {
    fn from(n: CssKeyframesScopedName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssLayerAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_LAYER_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_LAYER_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssLayerAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssLayerAtRule")
                .field(
                    "layer_token",
                    &support::DebugSyntaxResult(self.layer_token()),
                )
                .field("layer", &support::DebugSyntaxResult(self.layer()))
                .finish()
        } else {
            f.debug_struct("CssLayerAtRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssLayerAtRule> for SyntaxNode {
    fn from(n: CssLayerAtRule) -> Self {
        n.syntax
    }
}
impl From<CssLayerAtRule> for SyntaxElement {
    fn from(n: CssLayerAtRule) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssLayerDeclaration {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_LAYER_DECLARATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_LAYER_DECLARATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssLayerDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssLayerDeclaration")
                .field("references", &self.references())
                .field("block", &support::DebugSyntaxResult(self.block()))
                .finish()
        } else {
            f.debug_struct("CssLayerDeclaration").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssLayerDeclaration> for SyntaxNode {
    fn from(n: CssLayerDeclaration) -> Self {
        n.syntax
    }
}
impl From<CssLayerDeclaration> for SyntaxElement {
    fn from(n: CssLayerDeclaration) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssLayerReference {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_LAYER_REFERENCE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_LAYER_REFERENCE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssLayerReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssLayerReference")
                .field("references", &self.references())
                .field(
                    "semicolon_token",
                    &support::DebugSyntaxResult(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssLayerReference").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssLayerReference> for SyntaxNode {
    fn from(n: CssLayerReference) -> Self {
        n.syntax
    }
}
impl From<CssLayerReference> for SyntaxElement {
    fn from(n: CssLayerReference) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssListOfComponentValuesExpression")
                .field("css_component_value_list", &self.css_component_value_list())
                .finish()
        } else {
            f.debug_struct("CssListOfComponentValuesExpression")
                .finish()
        };
        DEPTH.set(current_depth);
        result
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
impl AstNode for CssMarginAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MARGIN_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MARGIN_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMarginAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssMarginAtRule")
                .field("at_token", &support::DebugSyntaxResult(self.at_token()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("block", &support::DebugSyntaxResult(self.block()))
                .finish()
        } else {
            f.debug_struct("CssMarginAtRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssMarginAtRule> for SyntaxNode {
    fn from(n: CssMarginAtRule) -> Self {
        n.syntax
    }
}
impl From<CssMarginAtRule> for SyntaxElement {
    fn from(n: CssMarginAtRule) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssMediaAndCondition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_AND_CONDITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_AND_CONDITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaAndCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssMediaAndCondition")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field("and_token", &support::DebugSyntaxResult(self.and_token()))
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("CssMediaAndCondition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssMediaAndCondition> for SyntaxNode {
    fn from(n: CssMediaAndCondition) -> Self {
        n.syntax
    }
}
impl From<CssMediaAndCondition> for SyntaxElement {
    fn from(n: CssMediaAndCondition) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssMediaAndTypeQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_AND_TYPE_QUERY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_AND_TYPE_QUERY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaAndTypeQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssMediaAndTypeQuery")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field("and_token", &support::DebugSyntaxResult(self.and_token()))
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("CssMediaAndTypeQuery").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssMediaAndTypeQuery> for SyntaxNode {
    fn from(n: CssMediaAndTypeQuery) -> Self {
        n.syntax
    }
}
impl From<CssMediaAndTypeQuery> for SyntaxElement {
    fn from(n: CssMediaAndTypeQuery) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssMediaAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssMediaAtRule")
                .field(
                    "media_token",
                    &support::DebugSyntaxResult(self.media_token()),
                )
                .field("queries", &self.queries())
                .field("block", &support::DebugSyntaxResult(self.block()))
                .finish()
        } else {
            f.debug_struct("CssMediaAtRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssMediaAtRule> for SyntaxNode {
    fn from(n: CssMediaAtRule) -> Self {
        n.syntax
    }
}
impl From<CssMediaAtRule> for SyntaxElement {
    fn from(n: CssMediaAtRule) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssMediaConditionInParens {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_CONDITION_IN_PARENS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_CONDITION_IN_PARENS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaConditionInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssMediaConditionInParens")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("condition", &support::DebugSyntaxResult(self.condition()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssMediaConditionInParens").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssMediaConditionInParens> for SyntaxNode {
    fn from(n: CssMediaConditionInParens) -> Self {
        n.syntax
    }
}
impl From<CssMediaConditionInParens> for SyntaxElement {
    fn from(n: CssMediaConditionInParens) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssMediaConditionQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_CONDITION_QUERY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_CONDITION_QUERY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaConditionQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssMediaConditionQuery")
                .field("condition", &support::DebugSyntaxResult(self.condition()))
                .finish()
        } else {
            f.debug_struct("CssMediaConditionQuery").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssMediaConditionQuery> for SyntaxNode {
    fn from(n: CssMediaConditionQuery) -> Self {
        n.syntax
    }
}
impl From<CssMediaConditionQuery> for SyntaxElement {
    fn from(n: CssMediaConditionQuery) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssMediaFeatureInParens {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_FEATURE_IN_PARENS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_FEATURE_IN_PARENS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaFeatureInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssMediaFeatureInParens")
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
        } else {
            f.debug_struct("CssMediaFeatureInParens").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssMediaFeatureInParens> for SyntaxNode {
    fn from(n: CssMediaFeatureInParens) -> Self {
        n.syntax
    }
}
impl From<CssMediaFeatureInParens> for SyntaxElement {
    fn from(n: CssMediaFeatureInParens) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssMediaNotCondition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_NOT_CONDITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_NOT_CONDITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaNotCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssMediaNotCondition")
                .field("not_token", &support::DebugSyntaxResult(self.not_token()))
                .field("condition", &support::DebugSyntaxResult(self.condition()))
                .finish()
        } else {
            f.debug_struct("CssMediaNotCondition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssMediaNotCondition> for SyntaxNode {
    fn from(n: CssMediaNotCondition) -> Self {
        n.syntax
    }
}
impl From<CssMediaNotCondition> for SyntaxElement {
    fn from(n: CssMediaNotCondition) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssMediaOrCondition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_OR_CONDITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_OR_CONDITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaOrCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssMediaOrCondition")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field("or_token", &support::DebugSyntaxResult(self.or_token()))
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("CssMediaOrCondition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssMediaOrCondition> for SyntaxNode {
    fn from(n: CssMediaOrCondition) -> Self {
        n.syntax
    }
}
impl From<CssMediaOrCondition> for SyntaxElement {
    fn from(n: CssMediaOrCondition) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssMediaType {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_TYPE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_TYPE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssMediaType")
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("CssMediaType").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssMediaType> for SyntaxNode {
    fn from(n: CssMediaType) -> Self {
        n.syntax
    }
}
impl From<CssMediaType> for SyntaxElement {
    fn from(n: CssMediaType) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssMediaTypeQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_TYPE_QUERY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_TYPE_QUERY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaTypeQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssMediaTypeQuery")
                .field("modifier", &support::DebugOptionalElement(self.modifier()))
                .field("ty", &support::DebugSyntaxResult(self.ty()))
                .finish()
        } else {
            f.debug_struct("CssMediaTypeQuery").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssMediaTypeQuery> for SyntaxNode {
    fn from(n: CssMediaTypeQuery) -> Self {
        n.syntax
    }
}
impl From<CssMediaTypeQuery> for SyntaxElement {
    fn from(n: CssMediaTypeQuery) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssMetavariable {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_METAVARIABLE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_METAVARIABLE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMetavariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssMetavariable")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssMetavariable").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssMetavariable> for SyntaxNode {
    fn from(n: CssMetavariable) -> Self {
        n.syntax
    }
}
impl From<CssMetavariable> for SyntaxElement {
    fn from(n: CssMetavariable) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssNamedNamespacePrefix {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_NAMED_NAMESPACE_PREFIX as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_NAMED_NAMESPACE_PREFIX
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssNamedNamespacePrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssNamedNamespacePrefix")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("CssNamedNamespacePrefix").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssNamedNamespacePrefix> for SyntaxNode {
    fn from(n: CssNamedNamespacePrefix) -> Self {
        n.syntax
    }
}
impl From<CssNamedNamespacePrefix> for SyntaxElement {
    fn from(n: CssNamedNamespacePrefix) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssNamespace {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_NAMESPACE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_NAMESPACE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssNamespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssNamespace")
                .field("prefix", &support::DebugOptionalElement(self.prefix()))
                .field(
                    "bitwise_or_token",
                    &support::DebugSyntaxResult(self.bitwise_or_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssNamespace").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssNamespace> for SyntaxNode {
    fn from(n: CssNamespace) -> Self {
        n.syntax
    }
}
impl From<CssNamespace> for SyntaxElement {
    fn from(n: CssNamespace) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssNamespaceAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_NAMESPACE_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_NAMESPACE_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssNamespaceAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssNamespaceAtRule")
                .field(
                    "namespace_token",
                    &support::DebugSyntaxResult(self.namespace_token()),
                )
                .field("prefix", &support::DebugOptionalElement(self.prefix()))
                .field("url", &support::DebugSyntaxResult(self.url()))
                .field(
                    "semicolon_token",
                    &support::DebugSyntaxResult(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssNamespaceAtRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssNamespaceAtRule> for SyntaxNode {
    fn from(n: CssNamespaceAtRule) -> Self {
        n.syntax
    }
}
impl From<CssNamespaceAtRule> for SyntaxElement {
    fn from(n: CssNamespaceAtRule) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssNestedQualifiedRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_NESTED_QUALIFIED_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_NESTED_QUALIFIED_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssNestedQualifiedRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssNestedQualifiedRule")
                .field("prelude", &self.prelude())
                .field("block", &support::DebugSyntaxResult(self.block()))
                .finish()
        } else {
            f.debug_struct("CssNestedQualifiedRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssNestedQualifiedRule> for SyntaxNode {
    fn from(n: CssNestedQualifiedRule) -> Self {
        n.syntax
    }
}
impl From<CssNestedQualifiedRule> for SyntaxElement {
    fn from(n: CssNestedQualifiedRule) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssNestedSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_NESTED_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_NESTED_SELECTOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssNestedSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssNestedSelector")
                .field("amp_token", &support::DebugSyntaxResult(self.amp_token()))
                .finish()
        } else {
            f.debug_struct("CssNestedSelector").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssNestedSelector> for SyntaxNode {
    fn from(n: CssNestedSelector) -> Self {
        n.syntax
    }
}
impl From<CssNestedSelector> for SyntaxElement {
    fn from(n: CssNestedSelector) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssNthOffset")
                .field("sign", &support::DebugSyntaxResult(self.sign()))
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("CssNthOffset").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssNthOffset> for SyntaxNode {
    fn from(n: CssNthOffset) -> Self {
        n.syntax
    }
}
impl From<CssNthOffset> for SyntaxElement {
    fn from(n: CssNthOffset) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssNumber")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssNumber").finish()
        };
        DEPTH.set(current_depth);
        result
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
impl AstNode for CssPageAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PAGE_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PAGE_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPageAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssPageAtRule")
                .field("page_token", &support::DebugSyntaxResult(self.page_token()))
                .field("selectors", &self.selectors())
                .field("block", &support::DebugSyntaxResult(self.block()))
                .finish()
        } else {
            f.debug_struct("CssPageAtRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPageAtRule> for SyntaxNode {
    fn from(n: CssPageAtRule) -> Self {
        n.syntax
    }
}
impl From<CssPageAtRule> for SyntaxElement {
    fn from(n: CssPageAtRule) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssPageAtRuleBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PAGE_AT_RULE_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PAGE_AT_RULE_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPageAtRuleBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssPageAtRuleBlock")
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
        } else {
            f.debug_struct("CssPageAtRuleBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPageAtRuleBlock> for SyntaxNode {
    fn from(n: CssPageAtRuleBlock) -> Self {
        n.syntax
    }
}
impl From<CssPageAtRuleBlock> for SyntaxElement {
    fn from(n: CssPageAtRuleBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssPageSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PAGE_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PAGE_SELECTOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPageSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssPageSelector")
                .field("ty", &support::DebugOptionalElement(self.ty()))
                .field("pseudos", &self.pseudos())
                .finish()
        } else {
            f.debug_struct("CssPageSelector").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPageSelector> for SyntaxNode {
    fn from(n: CssPageSelector) -> Self {
        n.syntax
    }
}
impl From<CssPageSelector> for SyntaxElement {
    fn from(n: CssPageSelector) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssPageSelectorPseudo {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PAGE_SELECTOR_PSEUDO as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PAGE_SELECTOR_PSEUDO
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPageSelectorPseudo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssPageSelectorPseudo")
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field("selector", &support::DebugSyntaxResult(self.selector()))
                .finish()
        } else {
            f.debug_struct("CssPageSelectorPseudo").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPageSelectorPseudo> for SyntaxNode {
    fn from(n: CssPageSelectorPseudo) -> Self {
        n.syntax
    }
}
impl From<CssPageSelectorPseudo> for SyntaxElement {
    fn from(n: CssPageSelectorPseudo) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssParameter")
                .field(
                    "any_css_expression",
                    &support::DebugSyntaxResult(self.any_css_expression()),
                )
                .finish()
        } else {
            f.debug_struct("CssParameter").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssParameter> for SyntaxNode {
    fn from(n: CssParameter) -> Self {
        n.syntax
    }
}
impl From<CssParameter> for SyntaxElement {
    fn from(n: CssParameter) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssParenthesizedExpression")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field(
                    "expression",
                    &support::DebugOptionalElement(self.expression()),
                )
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssParenthesizedExpression").finish()
        };
        DEPTH.set(current_depth);
        result
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssPercentage")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .field(
                    "percent_token",
                    &support::DebugSyntaxResult(self.percent_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssPercentage").finish()
        };
        DEPTH.set(current_depth);
        result
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
impl AstNode for CssPositionTryAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_POSITION_TRY_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_POSITION_TRY_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPositionTryAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssPositionTryAtRule")
                .field(
                    "position_try_token",
                    &support::DebugSyntaxResult(self.position_try_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("block", &support::DebugSyntaxResult(self.block()))
                .finish()
        } else {
            f.debug_struct("CssPositionTryAtRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPositionTryAtRule> for SyntaxNode {
    fn from(n: CssPositionTryAtRule) -> Self {
        n.syntax
    }
}
impl From<CssPositionTryAtRule> for SyntaxElement {
    fn from(n: CssPositionTryAtRule) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssPropertyAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PROPERTY_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PROPERTY_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPropertyAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssPropertyAtRule")
                .field(
                    "property_token",
                    &support::DebugSyntaxResult(self.property_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("block", &support::DebugSyntaxResult(self.block()))
                .finish()
        } else {
            f.debug_struct("CssPropertyAtRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPropertyAtRule> for SyntaxNode {
    fn from(n: CssPropertyAtRule) -> Self {
        n.syntax
    }
}
impl From<CssPropertyAtRule> for SyntaxElement {
    fn from(n: CssPropertyAtRule) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
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
        } else {
            f.debug_struct("CssPseudoClassFunctionCompoundSelector")
                .finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPseudoClassFunctionCompoundSelector> for SyntaxNode {
    fn from(n: CssPseudoClassFunctionCompoundSelector) -> Self {
        n.syntax
    }
}
impl From<CssPseudoClassFunctionCompoundSelector> for SyntaxElement {
    fn from(n: CssPseudoClassFunctionCompoundSelector) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssPseudoClassFunctionCompoundSelectorList")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("compound_selectors", &self.compound_selectors())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssPseudoClassFunctionCompoundSelectorList")
                .finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPseudoClassFunctionCompoundSelectorList> for SyntaxNode {
    fn from(n: CssPseudoClassFunctionCompoundSelectorList) -> Self {
        n.syntax
    }
}
impl From<CssPseudoClassFunctionCompoundSelectorList> for SyntaxElement {
    fn from(n: CssPseudoClassFunctionCompoundSelectorList) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassFunctionCustomIdentifierList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        CSS_PSEUDO_CLASS_FUNCTION_CUSTOM_IDENTIFIER_LIST as u16,
    ));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_FUNCTION_CUSTOM_IDENTIFIER_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPseudoClassFunctionCustomIdentifierList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssPseudoClassFunctionCustomIdentifierList")
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
        } else {
            f.debug_struct("CssPseudoClassFunctionCustomIdentifierList")
                .finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPseudoClassFunctionCustomIdentifierList> for SyntaxNode {
    fn from(n: CssPseudoClassFunctionCustomIdentifierList) -> Self {
        n.syntax
    }
}
impl From<CssPseudoClassFunctionCustomIdentifierList> for SyntaxElement {
    fn from(n: CssPseudoClassFunctionCustomIdentifierList) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
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
        } else {
            f.debug_struct("CssPseudoClassFunctionIdentifier").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPseudoClassFunctionIdentifier> for SyntaxNode {
    fn from(n: CssPseudoClassFunctionIdentifier) -> Self {
        n.syntax
    }
}
impl From<CssPseudoClassFunctionIdentifier> for SyntaxElement {
    fn from(n: CssPseudoClassFunctionIdentifier) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
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
        } else {
            f.debug_struct("CssPseudoClassFunctionNth").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPseudoClassFunctionNth> for SyntaxNode {
    fn from(n: CssPseudoClassFunctionNth) -> Self {
        n.syntax
    }
}
impl From<CssPseudoClassFunctionNth> for SyntaxElement {
    fn from(n: CssPseudoClassFunctionNth) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssPseudoClassFunctionRelativeSelectorList")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("relative_selectors", &self.relative_selectors())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssPseudoClassFunctionRelativeSelectorList")
                .finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPseudoClassFunctionRelativeSelectorList> for SyntaxNode {
    fn from(n: CssPseudoClassFunctionRelativeSelectorList) -> Self {
        n.syntax
    }
}
impl From<CssPseudoClassFunctionRelativeSelectorList> for SyntaxElement {
    fn from(n: CssPseudoClassFunctionRelativeSelectorList) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
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
        } else {
            f.debug_struct("CssPseudoClassFunctionSelector").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPseudoClassFunctionSelector> for SyntaxNode {
    fn from(n: CssPseudoClassFunctionSelector) -> Self {
        n.syntax
    }
}
impl From<CssPseudoClassFunctionSelector> for SyntaxElement {
    fn from(n: CssPseudoClassFunctionSelector) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssPseudoClassFunctionSelectorList")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("selectors", &self.selectors())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssPseudoClassFunctionSelectorList")
                .finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPseudoClassFunctionSelectorList> for SyntaxNode {
    fn from(n: CssPseudoClassFunctionSelectorList) -> Self {
        n.syntax
    }
}
impl From<CssPseudoClassFunctionSelectorList> for SyntaxElement {
    fn from(n: CssPseudoClassFunctionSelectorList) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssPseudoClassFunctionValueList")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("values", &self.values())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssPseudoClassFunctionValueList").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPseudoClassFunctionValueList> for SyntaxNode {
    fn from(n: CssPseudoClassFunctionValueList) -> Self {
        n.syntax
    }
}
impl From<CssPseudoClassFunctionValueList> for SyntaxElement {
    fn from(n: CssPseudoClassFunctionValueList) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssPseudoClassIdentifier")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("CssPseudoClassIdentifier").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPseudoClassIdentifier> for SyntaxNode {
    fn from(n: CssPseudoClassIdentifier) -> Self {
        n.syntax
    }
}
impl From<CssPseudoClassIdentifier> for SyntaxElement {
    fn from(n: CssPseudoClassIdentifier) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssPseudoClassNth")
                .field("sign", &support::DebugOptionalElement(self.sign()))
                .field("value", &support::DebugOptionalElement(self.value()))
                .field(
                    "symbol_token",
                    &support::DebugSyntaxResult(self.symbol_token()),
                )
                .field("offset", &support::DebugOptionalElement(self.offset()))
                .finish()
        } else {
            f.debug_struct("CssPseudoClassNth").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPseudoClassNth> for SyntaxNode {
    fn from(n: CssPseudoClassNth) -> Self {
        n.syntax
    }
}
impl From<CssPseudoClassNth> for SyntaxElement {
    fn from(n: CssPseudoClassNth) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssPseudoClassNthIdentifier")
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("CssPseudoClassNthIdentifier").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPseudoClassNthIdentifier> for SyntaxNode {
    fn from(n: CssPseudoClassNthIdentifier) -> Self {
        n.syntax
    }
}
impl From<CssPseudoClassNthIdentifier> for SyntaxElement {
    fn from(n: CssPseudoClassNthIdentifier) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssPseudoClassNthNumber")
                .field("sign", &support::DebugOptionalElement(self.sign()))
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("CssPseudoClassNthNumber").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPseudoClassNthNumber> for SyntaxNode {
    fn from(n: CssPseudoClassNthNumber) -> Self {
        n.syntax
    }
}
impl From<CssPseudoClassNthNumber> for SyntaxElement {
    fn from(n: CssPseudoClassNthNumber) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssPseudoClassNthSelector")
                .field("nth", &support::DebugSyntaxResult(self.nth()))
                .field(
                    "of_selector",
                    &support::DebugOptionalElement(self.of_selector()),
                )
                .finish()
        } else {
            f.debug_struct("CssPseudoClassNthSelector").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPseudoClassNthSelector> for SyntaxNode {
    fn from(n: CssPseudoClassNthSelector) -> Self {
        n.syntax
    }
}
impl From<CssPseudoClassNthSelector> for SyntaxElement {
    fn from(n: CssPseudoClassNthSelector) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssPseudoClassOfNthSelector")
                .field("of_token", &support::DebugSyntaxResult(self.of_token()))
                .field("selectors", &self.selectors())
                .finish()
        } else {
            f.debug_struct("CssPseudoClassOfNthSelector").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPseudoClassOfNthSelector> for SyntaxNode {
    fn from(n: CssPseudoClassOfNthSelector) -> Self {
        n.syntax
    }
}
impl From<CssPseudoClassOfNthSelector> for SyntaxElement {
    fn from(n: CssPseudoClassOfNthSelector) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssPseudoClassSelector")
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field("class", &support::DebugSyntaxResult(self.class()))
                .finish()
        } else {
            f.debug_struct("CssPseudoClassSelector").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPseudoClassSelector> for SyntaxNode {
    fn from(n: CssPseudoClassSelector) -> Self {
        n.syntax
    }
}
impl From<CssPseudoClassSelector> for SyntaxElement {
    fn from(n: CssPseudoClassSelector) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoElementFunction {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_ELEMENT_FUNCTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_ELEMENT_FUNCTION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPseudoElementFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssPseudoElementFunction")
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
        } else {
            f.debug_struct("CssPseudoElementFunction").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPseudoElementFunction> for SyntaxNode {
    fn from(n: CssPseudoElementFunction) -> Self {
        n.syntax
    }
}
impl From<CssPseudoElementFunction> for SyntaxElement {
    fn from(n: CssPseudoElementFunction) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoElementFunctionCustomIdentifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        CSS_PSEUDO_ELEMENT_FUNCTION_CUSTOM_IDENTIFIER as u16,
    ));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_ELEMENT_FUNCTION_CUSTOM_IDENTIFIER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPseudoElementFunctionCustomIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssPseudoElementFunctionCustomIdentifier")
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
        } else {
            f.debug_struct("CssPseudoElementFunctionCustomIdentifier")
                .finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPseudoElementFunctionCustomIdentifier> for SyntaxNode {
    fn from(n: CssPseudoElementFunctionCustomIdentifier) -> Self {
        n.syntax
    }
}
impl From<CssPseudoElementFunctionCustomIdentifier> for SyntaxElement {
    fn from(n: CssPseudoElementFunctionCustomIdentifier) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
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
        } else {
            f.debug_struct("CssPseudoElementFunctionSelector").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPseudoElementFunctionSelector> for SyntaxNode {
    fn from(n: CssPseudoElementFunctionSelector) -> Self {
        n.syntax
    }
}
impl From<CssPseudoElementFunctionSelector> for SyntaxElement {
    fn from(n: CssPseudoElementFunctionSelector) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssPseudoElementIdentifier")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("CssPseudoElementIdentifier").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPseudoElementIdentifier> for SyntaxNode {
    fn from(n: CssPseudoElementIdentifier) -> Self {
        n.syntax
    }
}
impl From<CssPseudoElementIdentifier> for SyntaxElement {
    fn from(n: CssPseudoElementIdentifier) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssPseudoElementSelector")
                .field(
                    "double_colon_token",
                    &support::DebugSyntaxResult(self.double_colon_token()),
                )
                .field("element", &support::DebugSyntaxResult(self.element()))
                .finish()
        } else {
            f.debug_struct("CssPseudoElementSelector").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssPseudoElementSelector> for SyntaxNode {
    fn from(n: CssPseudoElementSelector) -> Self {
        n.syntax
    }
}
impl From<CssPseudoElementSelector> for SyntaxElement {
    fn from(n: CssPseudoElementSelector) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssQualifiedRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_QUALIFIED_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_QUALIFIED_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssQualifiedRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssQualifiedRule")
                .field("prelude", &self.prelude())
                .field("block", &support::DebugSyntaxResult(self.block()))
                .finish()
        } else {
            f.debug_struct("CssQualifiedRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssQualifiedRule> for SyntaxNode {
    fn from(n: CssQualifiedRule) -> Self {
        n.syntax
    }
}
impl From<CssQualifiedRule> for SyntaxElement {
    fn from(n: CssQualifiedRule) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssQueryFeatureBoolean {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_QUERY_FEATURE_BOOLEAN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_QUERY_FEATURE_BOOLEAN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssQueryFeatureBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssQueryFeatureBoolean")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("CssQueryFeatureBoolean").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssQueryFeatureBoolean> for SyntaxNode {
    fn from(n: CssQueryFeatureBoolean) -> Self {
        n.syntax
    }
}
impl From<CssQueryFeatureBoolean> for SyntaxElement {
    fn from(n: CssQueryFeatureBoolean) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssQueryFeaturePlain {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_QUERY_FEATURE_PLAIN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_QUERY_FEATURE_PLAIN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssQueryFeaturePlain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssQueryFeaturePlain")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("CssQueryFeaturePlain").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssQueryFeaturePlain> for SyntaxNode {
    fn from(n: CssQueryFeaturePlain) -> Self {
        n.syntax
    }
}
impl From<CssQueryFeaturePlain> for SyntaxElement {
    fn from(n: CssQueryFeaturePlain) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssQueryFeatureRange {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_QUERY_FEATURE_RANGE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_QUERY_FEATURE_RANGE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssQueryFeatureRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssQueryFeatureRange")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field("comparison", &support::DebugSyntaxResult(self.comparison()))
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("CssQueryFeatureRange").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssQueryFeatureRange> for SyntaxNode {
    fn from(n: CssQueryFeatureRange) -> Self {
        n.syntax
    }
}
impl From<CssQueryFeatureRange> for SyntaxElement {
    fn from(n: CssQueryFeatureRange) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssQueryFeatureRangeComparison {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_QUERY_FEATURE_RANGE_COMPARISON as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_QUERY_FEATURE_RANGE_COMPARISON
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssQueryFeatureRangeComparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssQueryFeatureRangeComparison")
                .field("operator", &support::DebugSyntaxResult(self.operator()))
                .finish()
        } else {
            f.debug_struct("CssQueryFeatureRangeComparison").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssQueryFeatureRangeComparison> for SyntaxNode {
    fn from(n: CssQueryFeatureRangeComparison) -> Self {
        n.syntax
    }
}
impl From<CssQueryFeatureRangeComparison> for SyntaxElement {
    fn from(n: CssQueryFeatureRangeComparison) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssQueryFeatureRangeInterval {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_QUERY_FEATURE_RANGE_INTERVAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_QUERY_FEATURE_RANGE_INTERVAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssQueryFeatureRangeInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssQueryFeatureRangeInterval")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field(
                    "left_comparison",
                    &support::DebugSyntaxResult(self.left_comparison()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "right_comparison",
                    &support::DebugSyntaxResult(self.right_comparison()),
                )
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("CssQueryFeatureRangeInterval").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssQueryFeatureRangeInterval> for SyntaxNode {
    fn from(n: CssQueryFeatureRangeInterval) -> Self {
        n.syntax
    }
}
impl From<CssQueryFeatureRangeInterval> for SyntaxElement {
    fn from(n: CssQueryFeatureRangeInterval) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssQueryFeatureReverseRange {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_QUERY_FEATURE_REVERSE_RANGE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_QUERY_FEATURE_REVERSE_RANGE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssQueryFeatureReverseRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssQueryFeatureReverseRange")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field("comparison", &support::DebugSyntaxResult(self.comparison()))
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("CssQueryFeatureReverseRange").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssQueryFeatureReverseRange> for SyntaxNode {
    fn from(n: CssQueryFeatureReverseRange) -> Self {
        n.syntax
    }
}
impl From<CssQueryFeatureReverseRange> for SyntaxElement {
    fn from(n: CssQueryFeatureReverseRange) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssRatio")
                .field("numerator", &support::DebugSyntaxResult(self.numerator()))
                .field(
                    "slash_token",
                    &support::DebugSyntaxResult(self.slash_token()),
                )
                .field(
                    "denominator",
                    &support::DebugSyntaxResult(self.denominator()),
                )
                .finish()
        } else {
            f.debug_struct("CssRatio").finish()
        };
        DEPTH.set(current_depth);
        result
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssRegularDimension")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .field("unit_token", &support::DebugSyntaxResult(self.unit_token()))
                .finish()
        } else {
            f.debug_struct("CssRegularDimension").finish()
        };
        DEPTH.set(current_depth);
        result
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssRelativeSelector")
                .field(
                    "combinator",
                    &support::DebugOptionalElement(self.combinator()),
                )
                .field("selector", &support::DebugSyntaxResult(self.selector()))
                .finish()
        } else {
            f.debug_struct("CssRelativeSelector").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssRelativeSelector> for SyntaxNode {
    fn from(n: CssRelativeSelector) -> Self {
        n.syntax
    }
}
impl From<CssRelativeSelector> for SyntaxElement {
    fn from(n: CssRelativeSelector) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssRoot")
                .field(
                    "bom_token",
                    &support::DebugOptionalElement(self.bom_token()),
                )
                .field("rules", &self.rules())
                .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
                .finish()
        } else {
            f.debug_struct("CssRoot").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssRoot> for SyntaxNode {
    fn from(n: CssRoot) -> Self {
        n.syntax
    }
}
impl From<CssRoot> for SyntaxElement {
    fn from(n: CssRoot) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssRuleBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_RULE_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_RULE_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssRuleBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssRuleBlock")
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("rules", &self.rules())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssRuleBlock").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssRuleBlock> for SyntaxNode {
    fn from(n: CssRuleBlock) -> Self {
        n.syntax
    }
}
impl From<CssRuleBlock> for SyntaxElement {
    fn from(n: CssRuleBlock) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssScopeAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SCOPE_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SCOPE_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssScopeAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssScopeAtRule")
                .field(
                    "scope_token",
                    &support::DebugSyntaxResult(self.scope_token()),
                )
                .field("range", &support::DebugOptionalElement(self.range()))
                .field("block", &support::DebugSyntaxResult(self.block()))
                .finish()
        } else {
            f.debug_struct("CssScopeAtRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssScopeAtRule> for SyntaxNode {
    fn from(n: CssScopeAtRule) -> Self {
        n.syntax
    }
}
impl From<CssScopeAtRule> for SyntaxElement {
    fn from(n: CssScopeAtRule) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssScopeEdge {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SCOPE_EDGE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SCOPE_EDGE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssScopeEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssScopeEdge")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("selectors", &self.selectors())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssScopeEdge").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssScopeEdge> for SyntaxNode {
    fn from(n: CssScopeEdge) -> Self {
        n.syntax
    }
}
impl From<CssScopeEdge> for SyntaxElement {
    fn from(n: CssScopeEdge) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssScopeRangeEnd {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SCOPE_RANGE_END as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SCOPE_RANGE_END
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssScopeRangeEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssScopeRangeEnd")
                .field("to_token", &support::DebugSyntaxResult(self.to_token()))
                .field("end", &support::DebugSyntaxResult(self.end()))
                .finish()
        } else {
            f.debug_struct("CssScopeRangeEnd").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssScopeRangeEnd> for SyntaxNode {
    fn from(n: CssScopeRangeEnd) -> Self {
        n.syntax
    }
}
impl From<CssScopeRangeEnd> for SyntaxElement {
    fn from(n: CssScopeRangeEnd) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssScopeRangeInterval {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SCOPE_RANGE_INTERVAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SCOPE_RANGE_INTERVAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssScopeRangeInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssScopeRangeInterval")
                .field("start", &support::DebugSyntaxResult(self.start()))
                .field("to_token", &support::DebugSyntaxResult(self.to_token()))
                .field("end", &support::DebugSyntaxResult(self.end()))
                .finish()
        } else {
            f.debug_struct("CssScopeRangeInterval").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssScopeRangeInterval> for SyntaxNode {
    fn from(n: CssScopeRangeInterval) -> Self {
        n.syntax
    }
}
impl From<CssScopeRangeInterval> for SyntaxElement {
    fn from(n: CssScopeRangeInterval) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssScopeRangeStart {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SCOPE_RANGE_START as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SCOPE_RANGE_START
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssScopeRangeStart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssScopeRangeStart")
                .field("start", &support::DebugSyntaxResult(self.start()))
                .finish()
        } else {
            f.debug_struct("CssScopeRangeStart").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssScopeRangeStart> for SyntaxNode {
    fn from(n: CssScopeRangeStart) -> Self {
        n.syntax
    }
}
impl From<CssScopeRangeStart> for SyntaxElement {
    fn from(n: CssScopeRangeStart) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssStartingStyleAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_STARTING_STYLE_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_STARTING_STYLE_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssStartingStyleAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssStartingStyleAtRule")
                .field(
                    "starting_style_token",
                    &support::DebugSyntaxResult(self.starting_style_token()),
                )
                .field("block", &support::DebugSyntaxResult(self.block()))
                .finish()
        } else {
            f.debug_struct("CssStartingStyleAtRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssStartingStyleAtRule> for SyntaxNode {
    fn from(n: CssStartingStyleAtRule) -> Self {
        n.syntax
    }
}
impl From<CssStartingStyleAtRule> for SyntaxElement {
    fn from(n: CssStartingStyleAtRule) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssString")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssString").finish()
        };
        DEPTH.set(current_depth);
        result
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
impl AstNode for CssSupportsAndCondition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SUPPORTS_AND_CONDITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SUPPORTS_AND_CONDITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssSupportsAndCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssSupportsAndCondition")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field("and_token", &support::DebugSyntaxResult(self.and_token()))
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("CssSupportsAndCondition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssSupportsAndCondition> for SyntaxNode {
    fn from(n: CssSupportsAndCondition) -> Self {
        n.syntax
    }
}
impl From<CssSupportsAndCondition> for SyntaxElement {
    fn from(n: CssSupportsAndCondition) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssSupportsAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SUPPORTS_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SUPPORTS_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssSupportsAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssSupportsAtRule")
                .field(
                    "supports_token",
                    &support::DebugSyntaxResult(self.supports_token()),
                )
                .field("condition", &support::DebugSyntaxResult(self.condition()))
                .field("block", &support::DebugSyntaxResult(self.block()))
                .finish()
        } else {
            f.debug_struct("CssSupportsAtRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssSupportsAtRule> for SyntaxNode {
    fn from(n: CssSupportsAtRule) -> Self {
        n.syntax
    }
}
impl From<CssSupportsAtRule> for SyntaxElement {
    fn from(n: CssSupportsAtRule) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssSupportsConditionInParens {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SUPPORTS_CONDITION_IN_PARENS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SUPPORTS_CONDITION_IN_PARENS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssSupportsConditionInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssSupportsConditionInParens")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("condition", &support::DebugSyntaxResult(self.condition()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssSupportsConditionInParens").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssSupportsConditionInParens> for SyntaxNode {
    fn from(n: CssSupportsConditionInParens) -> Self {
        n.syntax
    }
}
impl From<CssSupportsConditionInParens> for SyntaxElement {
    fn from(n: CssSupportsConditionInParens) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssSupportsFeatureDeclaration {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SUPPORTS_FEATURE_DECLARATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SUPPORTS_FEATURE_DECLARATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssSupportsFeatureDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssSupportsFeatureDeclaration")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field(
                    "declaration",
                    &support::DebugSyntaxResult(self.declaration()),
                )
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssSupportsFeatureDeclaration").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssSupportsFeatureDeclaration> for SyntaxNode {
    fn from(n: CssSupportsFeatureDeclaration) -> Self {
        n.syntax
    }
}
impl From<CssSupportsFeatureDeclaration> for SyntaxElement {
    fn from(n: CssSupportsFeatureDeclaration) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssSupportsFeatureSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SUPPORTS_FEATURE_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SUPPORTS_FEATURE_SELECTOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssSupportsFeatureSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssSupportsFeatureSelector")
                .field(
                    "selector_token",
                    &support::DebugSyntaxResult(self.selector_token()),
                )
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
        } else {
            f.debug_struct("CssSupportsFeatureSelector").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssSupportsFeatureSelector> for SyntaxNode {
    fn from(n: CssSupportsFeatureSelector) -> Self {
        n.syntax
    }
}
impl From<CssSupportsFeatureSelector> for SyntaxElement {
    fn from(n: CssSupportsFeatureSelector) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssSupportsNotCondition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SUPPORTS_NOT_CONDITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SUPPORTS_NOT_CONDITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssSupportsNotCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssSupportsNotCondition")
                .field("not_token", &support::DebugSyntaxResult(self.not_token()))
                .field("query", &support::DebugSyntaxResult(self.query()))
                .finish()
        } else {
            f.debug_struct("CssSupportsNotCondition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssSupportsNotCondition> for SyntaxNode {
    fn from(n: CssSupportsNotCondition) -> Self {
        n.syntax
    }
}
impl From<CssSupportsNotCondition> for SyntaxElement {
    fn from(n: CssSupportsNotCondition) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssSupportsOrCondition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SUPPORTS_OR_CONDITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SUPPORTS_OR_CONDITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssSupportsOrCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssSupportsOrCondition")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field("or_token", &support::DebugSyntaxResult(self.or_token()))
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("CssSupportsOrCondition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssSupportsOrCondition> for SyntaxNode {
    fn from(n: CssSupportsOrCondition) -> Self {
        n.syntax
    }
}
impl From<CssSupportsOrCondition> for SyntaxElement {
    fn from(n: CssSupportsOrCondition) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssTypeSelector")
                .field(
                    "namespace",
                    &support::DebugOptionalElement(self.namespace()),
                )
                .field("ident", &support::DebugSyntaxResult(self.ident()))
                .finish()
        } else {
            f.debug_struct("CssTypeSelector").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssTypeSelector> for SyntaxNode {
    fn from(n: CssTypeSelector) -> Self {
        n.syntax
    }
}
impl From<CssTypeSelector> for SyntaxElement {
    fn from(n: CssTypeSelector) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssUnicodeCodepoint {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_UNICODE_CODEPOINT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_UNICODE_CODEPOINT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssUnicodeCodepoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssUnicodeCodepoint")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssUnicodeCodepoint").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssUnicodeCodepoint> for SyntaxNode {
    fn from(n: CssUnicodeCodepoint) -> Self {
        n.syntax
    }
}
impl From<CssUnicodeCodepoint> for SyntaxElement {
    fn from(n: CssUnicodeCodepoint) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssUnicodeRange {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_UNICODE_RANGE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_UNICODE_RANGE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssUnicodeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssUnicodeRange")
                .field(
                    "prefix_token",
                    &support::DebugSyntaxResult(self.prefix_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("CssUnicodeRange").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssUnicodeRange> for SyntaxNode {
    fn from(n: CssUnicodeRange) -> Self {
        n.syntax
    }
}
impl From<CssUnicodeRange> for SyntaxElement {
    fn from(n: CssUnicodeRange) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssUnicodeRangeInterval {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_UNICODE_RANGE_INTERVAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_UNICODE_RANGE_INTERVAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssUnicodeRangeInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssUnicodeRangeInterval")
                .field("start", &support::DebugSyntaxResult(self.start()))
                .field(
                    "minus_token",
                    &support::DebugSyntaxResult(self.minus_token()),
                )
                .field("end", &support::DebugSyntaxResult(self.end()))
                .finish()
        } else {
            f.debug_struct("CssUnicodeRangeInterval").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssUnicodeRangeInterval> for SyntaxNode {
    fn from(n: CssUnicodeRangeInterval) -> Self {
        n.syntax
    }
}
impl From<CssUnicodeRangeInterval> for SyntaxElement {
    fn from(n: CssUnicodeRangeInterval) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssUnicodeRangeWildcard {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_UNICODE_RANGE_WILDCARD as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_UNICODE_RANGE_WILDCARD
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssUnicodeRangeWildcard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssUnicodeRangeWildcard")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssUnicodeRangeWildcard").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssUnicodeRangeWildcard> for SyntaxNode {
    fn from(n: CssUnicodeRangeWildcard) -> Self {
        n.syntax
    }
}
impl From<CssUnicodeRangeWildcard> for SyntaxElement {
    fn from(n: CssUnicodeRangeWildcard) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssUniversalNamespacePrefix {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_UNIVERSAL_NAMESPACE_PREFIX as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_UNIVERSAL_NAMESPACE_PREFIX
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssUniversalNamespacePrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssUniversalNamespacePrefix")
                .field("star_token", &support::DebugSyntaxResult(self.star_token()))
                .finish()
        } else {
            f.debug_struct("CssUniversalNamespacePrefix").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssUniversalNamespacePrefix> for SyntaxNode {
    fn from(n: CssUniversalNamespacePrefix) -> Self {
        n.syntax
    }
}
impl From<CssUniversalNamespacePrefix> for SyntaxElement {
    fn from(n: CssUniversalNamespacePrefix) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssUniversalSelector")
                .field(
                    "namespace",
                    &support::DebugOptionalElement(self.namespace()),
                )
                .field("star_token", &support::DebugSyntaxResult(self.star_token()))
                .finish()
        } else {
            f.debug_struct("CssUniversalSelector").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssUniversalSelector> for SyntaxNode {
    fn from(n: CssUniversalSelector) -> Self {
        n.syntax
    }
}
impl From<CssUniversalSelector> for SyntaxElement {
    fn from(n: CssUniversalSelector) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssUnknownBlockAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_UNKNOWN_BLOCK_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_UNKNOWN_BLOCK_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssUnknownBlockAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssUnknownBlockAtRule")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("components", &support::DebugSyntaxResult(self.components()))
                .field("block", &support::DebugSyntaxResult(self.block()))
                .finish()
        } else {
            f.debug_struct("CssUnknownBlockAtRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssUnknownBlockAtRule> for SyntaxNode {
    fn from(n: CssUnknownBlockAtRule) -> Self {
        n.syntax
    }
}
impl From<CssUnknownBlockAtRule> for SyntaxElement {
    fn from(n: CssUnknownBlockAtRule) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssUnknownDimension")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .field("unit_token", &support::DebugSyntaxResult(self.unit_token()))
                .finish()
        } else {
            f.debug_struct("CssUnknownDimension").finish()
        };
        DEPTH.set(current_depth);
        result
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
impl AstNode for CssUnknownValueAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_UNKNOWN_VALUE_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_UNKNOWN_VALUE_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssUnknownValueAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssUnknownValueAtRule")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("components", &support::DebugSyntaxResult(self.components()))
                .field(
                    "semicolon_token",
                    &support::DebugSyntaxResult(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssUnknownValueAtRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssUnknownValueAtRule> for SyntaxNode {
    fn from(n: CssUnknownValueAtRule) -> Self {
        n.syntax
    }
}
impl From<CssUnknownValueAtRule> for SyntaxElement {
    fn from(n: CssUnknownValueAtRule) -> Self {
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssUrlFunction")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("value", &support::DebugOptionalElement(self.value()))
                .field("modifiers", &self.modifiers())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssUrlFunction").finish()
        };
        DEPTH.set(current_depth);
        result
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
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssUrlValueRaw")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssUrlValueRaw").finish()
        };
        DEPTH.set(current_depth);
        result
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
impl AstNode for CssValueAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_VALUE_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_VALUE_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssValueAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssValueAtRule")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .field("clause", &support::DebugSyntaxResult(self.clause()))
                .field(
                    "semicolon_token",
                    &support::DebugSyntaxResult(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("CssValueAtRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssValueAtRule> for SyntaxNode {
    fn from(n: CssValueAtRule) -> Self {
        n.syntax
    }
}
impl From<CssValueAtRule> for SyntaxElement {
    fn from(n: CssValueAtRule) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssValueAtRuleDeclarationClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_VALUE_AT_RULE_DECLARATION_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_VALUE_AT_RULE_DECLARATION_CLAUSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssValueAtRuleDeclarationClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssValueAtRuleDeclarationClause")
                .field("properties", &self.properties())
                .finish()
        } else {
            f.debug_struct("CssValueAtRuleDeclarationClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssValueAtRuleDeclarationClause> for SyntaxNode {
    fn from(n: CssValueAtRuleDeclarationClause) -> Self {
        n.syntax
    }
}
impl From<CssValueAtRuleDeclarationClause> for SyntaxElement {
    fn from(n: CssValueAtRuleDeclarationClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssValueAtRuleGenericProperty {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_VALUE_AT_RULE_GENERIC_PROPERTY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_VALUE_AT_RULE_GENERIC_PROPERTY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssValueAtRuleGenericProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssValueAtRuleGenericProperty")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("CssValueAtRuleGenericProperty").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssValueAtRuleGenericProperty> for SyntaxNode {
    fn from(n: CssValueAtRuleGenericProperty) -> Self {
        n.syntax
    }
}
impl From<CssValueAtRuleGenericProperty> for SyntaxElement {
    fn from(n: CssValueAtRuleGenericProperty) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssValueAtRuleImportClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_VALUE_AT_RULE_IMPORT_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_VALUE_AT_RULE_IMPORT_CLAUSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssValueAtRuleImportClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssValueAtRuleImportClause")
                .field("specifiers", &self.specifiers())
                .field("from_token", &support::DebugSyntaxResult(self.from_token()))
                .field("source", &support::DebugSyntaxResult(self.source()))
                .finish()
        } else {
            f.debug_struct("CssValueAtRuleImportClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssValueAtRuleImportClause> for SyntaxNode {
    fn from(n: CssValueAtRuleImportClause) -> Self {
        n.syntax
    }
}
impl From<CssValueAtRuleImportClause> for SyntaxElement {
    fn from(n: CssValueAtRuleImportClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssValueAtRuleImportSpecifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_VALUE_AT_RULE_IMPORT_SPECIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_VALUE_AT_RULE_IMPORT_SPECIFIER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssValueAtRuleImportSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssValueAtRuleImportSpecifier")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("CssValueAtRuleImportSpecifier").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssValueAtRuleImportSpecifier> for SyntaxNode {
    fn from(n: CssValueAtRuleImportSpecifier) -> Self {
        n.syntax
    }
}
impl From<CssValueAtRuleImportSpecifier> for SyntaxElement {
    fn from(n: CssValueAtRuleImportSpecifier) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssValueAtRuleNamedImportSpecifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        CSS_VALUE_AT_RULE_NAMED_IMPORT_SPECIFIER as u16,
    ));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_VALUE_AT_RULE_NAMED_IMPORT_SPECIFIER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssValueAtRuleNamedImportSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssValueAtRuleNamedImportSpecifier")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("as_token", &support::DebugSyntaxResult(self.as_token()))
                .field("local_name", &support::DebugSyntaxResult(self.local_name()))
                .finish()
        } else {
            f.debug_struct("CssValueAtRuleNamedImportSpecifier")
                .finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssValueAtRuleNamedImportSpecifier> for SyntaxNode {
    fn from(n: CssValueAtRuleNamedImportSpecifier) -> Self {
        n.syntax
    }
}
impl From<CssValueAtRuleNamedImportSpecifier> for SyntaxElement {
    fn from(n: CssValueAtRuleNamedImportSpecifier) -> Self {
        n.syntax.into()
    }
}
impl AstNode for CssViewTransitionAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_VIEW_TRANSITION_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_VIEW_TRANSITION_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssViewTransitionAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("CssViewTransitionAtRule")
                .field(
                    "view_transition_token",
                    &support::DebugSyntaxResult(self.view_transition_token()),
                )
                .field("block", &support::DebugSyntaxResult(self.block()))
                .finish()
        } else {
            f.debug_struct("CssViewTransitionAtRule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<CssViewTransitionAtRule> for SyntaxNode {
    fn from(n: CssViewTransitionAtRule) -> Self {
        n.syntax
    }
}
impl From<CssViewTransitionAtRule> for SyntaxElement {
    fn from(n: CssViewTransitionAtRule) -> Self {
        n.syntax.into()
    }
}
impl From<CssBogusAtRule> for AnyCssAtRule {
    fn from(node: CssBogusAtRule) -> Self {
        Self::CssBogusAtRule(node)
    }
}
impl From<CssCharsetAtRule> for AnyCssAtRule {
    fn from(node: CssCharsetAtRule) -> Self {
        Self::CssCharsetAtRule(node)
    }
}
impl From<CssColorProfileAtRule> for AnyCssAtRule {
    fn from(node: CssColorProfileAtRule) -> Self {
        Self::CssColorProfileAtRule(node)
    }
}
impl From<CssContainerAtRule> for AnyCssAtRule {
    fn from(node: CssContainerAtRule) -> Self {
        Self::CssContainerAtRule(node)
    }
}
impl From<CssCounterStyleAtRule> for AnyCssAtRule {
    fn from(node: CssCounterStyleAtRule) -> Self {
        Self::CssCounterStyleAtRule(node)
    }
}
impl From<CssDocumentAtRule> for AnyCssAtRule {
    fn from(node: CssDocumentAtRule) -> Self {
        Self::CssDocumentAtRule(node)
    }
}
impl From<CssFontFaceAtRule> for AnyCssAtRule {
    fn from(node: CssFontFaceAtRule) -> Self {
        Self::CssFontFaceAtRule(node)
    }
}
impl From<CssFontFeatureValuesAtRule> for AnyCssAtRule {
    fn from(node: CssFontFeatureValuesAtRule) -> Self {
        Self::CssFontFeatureValuesAtRule(node)
    }
}
impl From<CssFontPaletteValuesAtRule> for AnyCssAtRule {
    fn from(node: CssFontPaletteValuesAtRule) -> Self {
        Self::CssFontPaletteValuesAtRule(node)
    }
}
impl From<CssImportAtRule> for AnyCssAtRule {
    fn from(node: CssImportAtRule) -> Self {
        Self::CssImportAtRule(node)
    }
}
impl From<CssKeyframesAtRule> for AnyCssAtRule {
    fn from(node: CssKeyframesAtRule) -> Self {
        Self::CssKeyframesAtRule(node)
    }
}
impl From<CssLayerAtRule> for AnyCssAtRule {
    fn from(node: CssLayerAtRule) -> Self {
        Self::CssLayerAtRule(node)
    }
}
impl From<CssMediaAtRule> for AnyCssAtRule {
    fn from(node: CssMediaAtRule) -> Self {
        Self::CssMediaAtRule(node)
    }
}
impl From<CssNamespaceAtRule> for AnyCssAtRule {
    fn from(node: CssNamespaceAtRule) -> Self {
        Self::CssNamespaceAtRule(node)
    }
}
impl From<CssPageAtRule> for AnyCssAtRule {
    fn from(node: CssPageAtRule) -> Self {
        Self::CssPageAtRule(node)
    }
}
impl From<CssPositionTryAtRule> for AnyCssAtRule {
    fn from(node: CssPositionTryAtRule) -> Self {
        Self::CssPositionTryAtRule(node)
    }
}
impl From<CssPropertyAtRule> for AnyCssAtRule {
    fn from(node: CssPropertyAtRule) -> Self {
        Self::CssPropertyAtRule(node)
    }
}
impl From<CssScopeAtRule> for AnyCssAtRule {
    fn from(node: CssScopeAtRule) -> Self {
        Self::CssScopeAtRule(node)
    }
}
impl From<CssStartingStyleAtRule> for AnyCssAtRule {
    fn from(node: CssStartingStyleAtRule) -> Self {
        Self::CssStartingStyleAtRule(node)
    }
}
impl From<CssSupportsAtRule> for AnyCssAtRule {
    fn from(node: CssSupportsAtRule) -> Self {
        Self::CssSupportsAtRule(node)
    }
}
impl From<CssUnknownBlockAtRule> for AnyCssAtRule {
    fn from(node: CssUnknownBlockAtRule) -> Self {
        Self::CssUnknownBlockAtRule(node)
    }
}
impl From<CssUnknownValueAtRule> for AnyCssAtRule {
    fn from(node: CssUnknownValueAtRule) -> Self {
        Self::CssUnknownValueAtRule(node)
    }
}
impl From<CssValueAtRule> for AnyCssAtRule {
    fn from(node: CssValueAtRule) -> Self {
        Self::CssValueAtRule(node)
    }
}
impl From<CssViewTransitionAtRule> for AnyCssAtRule {
    fn from(node: CssViewTransitionAtRule) -> Self {
        Self::CssViewTransitionAtRule(node)
    }
}
impl AstNode for AnyCssAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssBogusAtRule::KIND_SET
        .union(CssCharsetAtRule::KIND_SET)
        .union(CssColorProfileAtRule::KIND_SET)
        .union(CssContainerAtRule::KIND_SET)
        .union(CssCounterStyleAtRule::KIND_SET)
        .union(CssDocumentAtRule::KIND_SET)
        .union(CssFontFaceAtRule::KIND_SET)
        .union(CssFontFeatureValuesAtRule::KIND_SET)
        .union(CssFontPaletteValuesAtRule::KIND_SET)
        .union(CssImportAtRule::KIND_SET)
        .union(CssKeyframesAtRule::KIND_SET)
        .union(CssLayerAtRule::KIND_SET)
        .union(CssMediaAtRule::KIND_SET)
        .union(CssNamespaceAtRule::KIND_SET)
        .union(CssPageAtRule::KIND_SET)
        .union(CssPositionTryAtRule::KIND_SET)
        .union(CssPropertyAtRule::KIND_SET)
        .union(CssScopeAtRule::KIND_SET)
        .union(CssStartingStyleAtRule::KIND_SET)
        .union(CssSupportsAtRule::KIND_SET)
        .union(CssUnknownBlockAtRule::KIND_SET)
        .union(CssUnknownValueAtRule::KIND_SET)
        .union(CssValueAtRule::KIND_SET)
        .union(CssViewTransitionAtRule::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_BOGUS_AT_RULE
                | CSS_CHARSET_AT_RULE
                | CSS_COLOR_PROFILE_AT_RULE
                | CSS_CONTAINER_AT_RULE
                | CSS_COUNTER_STYLE_AT_RULE
                | CSS_DOCUMENT_AT_RULE
                | CSS_FONT_FACE_AT_RULE
                | CSS_FONT_FEATURE_VALUES_AT_RULE
                | CSS_FONT_PALETTE_VALUES_AT_RULE
                | CSS_IMPORT_AT_RULE
                | CSS_KEYFRAMES_AT_RULE
                | CSS_LAYER_AT_RULE
                | CSS_MEDIA_AT_RULE
                | CSS_NAMESPACE_AT_RULE
                | CSS_PAGE_AT_RULE
                | CSS_POSITION_TRY_AT_RULE
                | CSS_PROPERTY_AT_RULE
                | CSS_SCOPE_AT_RULE
                | CSS_STARTING_STYLE_AT_RULE
                | CSS_SUPPORTS_AT_RULE
                | CSS_UNKNOWN_BLOCK_AT_RULE
                | CSS_UNKNOWN_VALUE_AT_RULE
                | CSS_VALUE_AT_RULE
                | CSS_VIEW_TRANSITION_AT_RULE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_AT_RULE => Self::CssBogusAtRule(CssBogusAtRule { syntax }),
            CSS_CHARSET_AT_RULE => Self::CssCharsetAtRule(CssCharsetAtRule { syntax }),
            CSS_COLOR_PROFILE_AT_RULE => {
                Self::CssColorProfileAtRule(CssColorProfileAtRule { syntax })
            }
            CSS_CONTAINER_AT_RULE => Self::CssContainerAtRule(CssContainerAtRule { syntax }),
            CSS_COUNTER_STYLE_AT_RULE => {
                Self::CssCounterStyleAtRule(CssCounterStyleAtRule { syntax })
            }
            CSS_DOCUMENT_AT_RULE => Self::CssDocumentAtRule(CssDocumentAtRule { syntax }),
            CSS_FONT_FACE_AT_RULE => Self::CssFontFaceAtRule(CssFontFaceAtRule { syntax }),
            CSS_FONT_FEATURE_VALUES_AT_RULE => {
                Self::CssFontFeatureValuesAtRule(CssFontFeatureValuesAtRule { syntax })
            }
            CSS_FONT_PALETTE_VALUES_AT_RULE => {
                Self::CssFontPaletteValuesAtRule(CssFontPaletteValuesAtRule { syntax })
            }
            CSS_IMPORT_AT_RULE => Self::CssImportAtRule(CssImportAtRule { syntax }),
            CSS_KEYFRAMES_AT_RULE => Self::CssKeyframesAtRule(CssKeyframesAtRule { syntax }),
            CSS_LAYER_AT_RULE => Self::CssLayerAtRule(CssLayerAtRule { syntax }),
            CSS_MEDIA_AT_RULE => Self::CssMediaAtRule(CssMediaAtRule { syntax }),
            CSS_NAMESPACE_AT_RULE => Self::CssNamespaceAtRule(CssNamespaceAtRule { syntax }),
            CSS_PAGE_AT_RULE => Self::CssPageAtRule(CssPageAtRule { syntax }),
            CSS_POSITION_TRY_AT_RULE => Self::CssPositionTryAtRule(CssPositionTryAtRule { syntax }),
            CSS_PROPERTY_AT_RULE => Self::CssPropertyAtRule(CssPropertyAtRule { syntax }),
            CSS_SCOPE_AT_RULE => Self::CssScopeAtRule(CssScopeAtRule { syntax }),
            CSS_STARTING_STYLE_AT_RULE => {
                Self::CssStartingStyleAtRule(CssStartingStyleAtRule { syntax })
            }
            CSS_SUPPORTS_AT_RULE => Self::CssSupportsAtRule(CssSupportsAtRule { syntax }),
            CSS_UNKNOWN_BLOCK_AT_RULE => {
                Self::CssUnknownBlockAtRule(CssUnknownBlockAtRule { syntax })
            }
            CSS_UNKNOWN_VALUE_AT_RULE => {
                Self::CssUnknownValueAtRule(CssUnknownValueAtRule { syntax })
            }
            CSS_VALUE_AT_RULE => Self::CssValueAtRule(CssValueAtRule { syntax }),
            CSS_VIEW_TRANSITION_AT_RULE => {
                Self::CssViewTransitionAtRule(CssViewTransitionAtRule { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusAtRule(it) => &it.syntax,
            Self::CssCharsetAtRule(it) => &it.syntax,
            Self::CssColorProfileAtRule(it) => &it.syntax,
            Self::CssContainerAtRule(it) => &it.syntax,
            Self::CssCounterStyleAtRule(it) => &it.syntax,
            Self::CssDocumentAtRule(it) => &it.syntax,
            Self::CssFontFaceAtRule(it) => &it.syntax,
            Self::CssFontFeatureValuesAtRule(it) => &it.syntax,
            Self::CssFontPaletteValuesAtRule(it) => &it.syntax,
            Self::CssImportAtRule(it) => &it.syntax,
            Self::CssKeyframesAtRule(it) => &it.syntax,
            Self::CssLayerAtRule(it) => &it.syntax,
            Self::CssMediaAtRule(it) => &it.syntax,
            Self::CssNamespaceAtRule(it) => &it.syntax,
            Self::CssPageAtRule(it) => &it.syntax,
            Self::CssPositionTryAtRule(it) => &it.syntax,
            Self::CssPropertyAtRule(it) => &it.syntax,
            Self::CssScopeAtRule(it) => &it.syntax,
            Self::CssStartingStyleAtRule(it) => &it.syntax,
            Self::CssSupportsAtRule(it) => &it.syntax,
            Self::CssUnknownBlockAtRule(it) => &it.syntax,
            Self::CssUnknownValueAtRule(it) => &it.syntax,
            Self::CssValueAtRule(it) => &it.syntax,
            Self::CssViewTransitionAtRule(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusAtRule(it) => it.syntax,
            Self::CssCharsetAtRule(it) => it.syntax,
            Self::CssColorProfileAtRule(it) => it.syntax,
            Self::CssContainerAtRule(it) => it.syntax,
            Self::CssCounterStyleAtRule(it) => it.syntax,
            Self::CssDocumentAtRule(it) => it.syntax,
            Self::CssFontFaceAtRule(it) => it.syntax,
            Self::CssFontFeatureValuesAtRule(it) => it.syntax,
            Self::CssFontPaletteValuesAtRule(it) => it.syntax,
            Self::CssImportAtRule(it) => it.syntax,
            Self::CssKeyframesAtRule(it) => it.syntax,
            Self::CssLayerAtRule(it) => it.syntax,
            Self::CssMediaAtRule(it) => it.syntax,
            Self::CssNamespaceAtRule(it) => it.syntax,
            Self::CssPageAtRule(it) => it.syntax,
            Self::CssPositionTryAtRule(it) => it.syntax,
            Self::CssPropertyAtRule(it) => it.syntax,
            Self::CssScopeAtRule(it) => it.syntax,
            Self::CssStartingStyleAtRule(it) => it.syntax,
            Self::CssSupportsAtRule(it) => it.syntax,
            Self::CssUnknownBlockAtRule(it) => it.syntax,
            Self::CssUnknownValueAtRule(it) => it.syntax,
            Self::CssValueAtRule(it) => it.syntax,
            Self::CssViewTransitionAtRule(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssCharsetAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssColorProfileAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssContainerAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssCounterStyleAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssDocumentAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssFontFaceAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssFontFeatureValuesAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssFontPaletteValuesAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssImportAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssKeyframesAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssLayerAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssMediaAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssNamespaceAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssPageAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssPositionTryAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssPropertyAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssScopeAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssStartingStyleAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssSupportsAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssUnknownBlockAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssUnknownValueAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssValueAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssViewTransitionAtRule(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssAtRule> for SyntaxNode {
    fn from(n: AnyCssAtRule) -> Self {
        match n {
            AnyCssAtRule::CssBogusAtRule(it) => it.into(),
            AnyCssAtRule::CssCharsetAtRule(it) => it.into(),
            AnyCssAtRule::CssColorProfileAtRule(it) => it.into(),
            AnyCssAtRule::CssContainerAtRule(it) => it.into(),
            AnyCssAtRule::CssCounterStyleAtRule(it) => it.into(),
            AnyCssAtRule::CssDocumentAtRule(it) => it.into(),
            AnyCssAtRule::CssFontFaceAtRule(it) => it.into(),
            AnyCssAtRule::CssFontFeatureValuesAtRule(it) => it.into(),
            AnyCssAtRule::CssFontPaletteValuesAtRule(it) => it.into(),
            AnyCssAtRule::CssImportAtRule(it) => it.into(),
            AnyCssAtRule::CssKeyframesAtRule(it) => it.into(),
            AnyCssAtRule::CssLayerAtRule(it) => it.into(),
            AnyCssAtRule::CssMediaAtRule(it) => it.into(),
            AnyCssAtRule::CssNamespaceAtRule(it) => it.into(),
            AnyCssAtRule::CssPageAtRule(it) => it.into(),
            AnyCssAtRule::CssPositionTryAtRule(it) => it.into(),
            AnyCssAtRule::CssPropertyAtRule(it) => it.into(),
            AnyCssAtRule::CssScopeAtRule(it) => it.into(),
            AnyCssAtRule::CssStartingStyleAtRule(it) => it.into(),
            AnyCssAtRule::CssSupportsAtRule(it) => it.into(),
            AnyCssAtRule::CssUnknownBlockAtRule(it) => it.into(),
            AnyCssAtRule::CssUnknownValueAtRule(it) => it.into(),
            AnyCssAtRule::CssValueAtRule(it) => it.into(),
            AnyCssAtRule::CssViewTransitionAtRule(it) => it.into(),
        }
    }
}
impl From<AnyCssAtRule> for SyntaxElement {
    fn from(n: AnyCssAtRule) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssIdentifier> for AnyCssAttributeMatcherValue {
    fn from(node: CssIdentifier) -> Self {
        Self::CssIdentifier(node)
    }
}
impl From<CssString> for AnyCssAttributeMatcherValue {
    fn from(node: CssString) -> Self {
        Self::CssString(node)
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
            CSS_IDENTIFIER => Self::CssIdentifier(CssIdentifier { syntax }),
            CSS_STRING => Self::CssString(CssString { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssIdentifier(it) => &it.syntax,
            Self::CssString(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssIdentifier(it) => it.syntax,
            Self::CssString(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssAttributeMatcherValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssIdentifier(it) => std::fmt::Debug::fmt(it, f),
            Self::CssString(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssAttributeMatcherValue> for SyntaxNode {
    fn from(n: AnyCssAttributeMatcherValue) -> Self {
        match n {
            AnyCssAttributeMatcherValue::CssIdentifier(it) => it.into(),
            AnyCssAttributeMatcherValue::CssString(it) => it.into(),
        }
    }
}
impl From<AnyCssAttributeMatcherValue> for SyntaxElement {
    fn from(n: AnyCssAttributeMatcherValue) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssIdentifier> for AnyCssComposesImportSource {
    fn from(node: CssIdentifier) -> Self {
        Self::CssIdentifier(node)
    }
}
impl From<CssString> for AnyCssComposesImportSource {
    fn from(node: CssString) -> Self {
        Self::CssString(node)
    }
}
impl AstNode for AnyCssComposesImportSource {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssIdentifier::KIND_SET.union(CssString::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_IDENTIFIER | CSS_STRING)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_IDENTIFIER => Self::CssIdentifier(CssIdentifier { syntax }),
            CSS_STRING => Self::CssString(CssString { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssIdentifier(it) => &it.syntax,
            Self::CssString(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssIdentifier(it) => it.syntax,
            Self::CssString(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssComposesImportSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssIdentifier(it) => std::fmt::Debug::fmt(it, f),
            Self::CssString(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssComposesImportSource> for SyntaxNode {
    fn from(n: AnyCssComposesImportSource) -> Self {
        match n {
            AnyCssComposesImportSource::CssIdentifier(it) => it.into(),
            AnyCssComposesImportSource::CssString(it) => it.into(),
        }
    }
}
impl From<AnyCssComposesImportSource> for SyntaxElement {
    fn from(n: AnyCssComposesImportSource) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusSelector> for AnyCssCompoundSelector {
    fn from(node: CssBogusSelector) -> Self {
        Self::CssBogusSelector(node)
    }
}
impl From<CssCompoundSelector> for AnyCssCompoundSelector {
    fn from(node: CssCompoundSelector) -> Self {
        Self::CssCompoundSelector(node)
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
            CSS_BOGUS_SELECTOR => Self::CssBogusSelector(CssBogusSelector { syntax }),
            CSS_COMPOUND_SELECTOR => Self::CssCompoundSelector(CssCompoundSelector { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusSelector(it) => &it.syntax,
            Self::CssCompoundSelector(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusSelector(it) => it.syntax,
            Self::CssCompoundSelector(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssCompoundSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusSelector(it) => std::fmt::Debug::fmt(it, f),
            Self::CssCompoundSelector(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssCompoundSelector> for SyntaxNode {
    fn from(n: AnyCssCompoundSelector) -> Self {
        match n {
            AnyCssCompoundSelector::CssBogusSelector(it) => it.into(),
            AnyCssCompoundSelector::CssCompoundSelector(it) => it.into(),
        }
    }
}
impl From<AnyCssCompoundSelector> for SyntaxElement {
    fn from(n: AnyCssCompoundSelector) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusBlock> for AnyCssConditionalBlock {
    fn from(node: CssBogusBlock) -> Self {
        Self::CssBogusBlock(node)
    }
}
impl From<CssDeclarationOrRuleBlock> for AnyCssConditionalBlock {
    fn from(node: CssDeclarationOrRuleBlock) -> Self {
        Self::CssDeclarationOrRuleBlock(node)
    }
}
impl From<CssRuleBlock> for AnyCssConditionalBlock {
    fn from(node: CssRuleBlock) -> Self {
        Self::CssRuleBlock(node)
    }
}
impl AstNode for AnyCssConditionalBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssBogusBlock::KIND_SET
        .union(CssDeclarationOrRuleBlock::KIND_SET)
        .union(CssRuleBlock::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_BOGUS_BLOCK | CSS_DECLARATION_OR_RULE_BLOCK | CSS_RULE_BLOCK
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_BLOCK => Self::CssBogusBlock(CssBogusBlock { syntax }),
            CSS_DECLARATION_OR_RULE_BLOCK => {
                Self::CssDeclarationOrRuleBlock(CssDeclarationOrRuleBlock { syntax })
            }
            CSS_RULE_BLOCK => Self::CssRuleBlock(CssRuleBlock { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusBlock(it) => &it.syntax,
            Self::CssDeclarationOrRuleBlock(it) => &it.syntax,
            Self::CssRuleBlock(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusBlock(it) => it.syntax,
            Self::CssDeclarationOrRuleBlock(it) => it.syntax,
            Self::CssRuleBlock(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssConditionalBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::CssDeclarationOrRuleBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::CssRuleBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssConditionalBlock> for SyntaxNode {
    fn from(n: AnyCssConditionalBlock) -> Self {
        match n {
            AnyCssConditionalBlock::CssBogusBlock(it) => it.into(),
            AnyCssConditionalBlock::CssDeclarationOrRuleBlock(it) => it.into(),
            AnyCssConditionalBlock::CssRuleBlock(it) => it.into(),
        }
    }
}
impl From<AnyCssConditionalBlock> for SyntaxElement {
    fn from(n: AnyCssConditionalBlock) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssContainerAndQuery> for AnyCssContainerAndCombinableQuery {
    fn from(node: CssContainerAndQuery) -> Self {
        Self::CssContainerAndQuery(node)
    }
}
impl AstNode for AnyCssContainerAndCombinableQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyCssContainerQueryInParens::KIND_SET.union(CssContainerAndQuery::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_CONTAINER_AND_QUERY => true,
            k if AnyCssContainerQueryInParens::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_CONTAINER_AND_QUERY => Self::CssContainerAndQuery(CssContainerAndQuery { syntax }),
            _ => {
                if let Some(any_css_container_query_in_parens) =
                    AnyCssContainerQueryInParens::cast(syntax)
                {
                    return Some(Self::AnyCssContainerQueryInParens(
                        any_css_container_query_in_parens,
                    ));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssContainerAndQuery(it) => &it.syntax,
            Self::AnyCssContainerQueryInParens(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssContainerAndQuery(it) => it.syntax,
            Self::AnyCssContainerQueryInParens(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssContainerAndCombinableQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyCssContainerQueryInParens(it) => std::fmt::Debug::fmt(it, f),
            Self::CssContainerAndQuery(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssContainerAndCombinableQuery> for SyntaxNode {
    fn from(n: AnyCssContainerAndCombinableQuery) -> Self {
        match n {
            AnyCssContainerAndCombinableQuery::AnyCssContainerQueryInParens(it) => it.into(),
            AnyCssContainerAndCombinableQuery::CssContainerAndQuery(it) => it.into(),
        }
    }
}
impl From<AnyCssContainerAndCombinableQuery> for SyntaxElement {
    fn from(n: AnyCssContainerAndCombinableQuery) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssContainerOrQuery> for AnyCssContainerOrCombinableQuery {
    fn from(node: CssContainerOrQuery) -> Self {
        Self::CssContainerOrQuery(node)
    }
}
impl AstNode for AnyCssContainerOrCombinableQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyCssContainerQueryInParens::KIND_SET.union(CssContainerOrQuery::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_CONTAINER_OR_QUERY => true,
            k if AnyCssContainerQueryInParens::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_CONTAINER_OR_QUERY => Self::CssContainerOrQuery(CssContainerOrQuery { syntax }),
            _ => {
                if let Some(any_css_container_query_in_parens) =
                    AnyCssContainerQueryInParens::cast(syntax)
                {
                    return Some(Self::AnyCssContainerQueryInParens(
                        any_css_container_query_in_parens,
                    ));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssContainerOrQuery(it) => &it.syntax,
            Self::AnyCssContainerQueryInParens(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssContainerOrQuery(it) => it.syntax,
            Self::AnyCssContainerQueryInParens(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssContainerOrCombinableQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyCssContainerQueryInParens(it) => std::fmt::Debug::fmt(it, f),
            Self::CssContainerOrQuery(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssContainerOrCombinableQuery> for SyntaxNode {
    fn from(n: AnyCssContainerOrCombinableQuery) -> Self {
        match n {
            AnyCssContainerOrCombinableQuery::AnyCssContainerQueryInParens(it) => it.into(),
            AnyCssContainerOrCombinableQuery::CssContainerOrQuery(it) => it.into(),
        }
    }
}
impl From<AnyCssContainerOrCombinableQuery> for SyntaxElement {
    fn from(n: AnyCssContainerOrCombinableQuery) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssContainerAndQuery> for AnyCssContainerQuery {
    fn from(node: CssContainerAndQuery) -> Self {
        Self::CssContainerAndQuery(node)
    }
}
impl From<CssContainerNotQuery> for AnyCssContainerQuery {
    fn from(node: CssContainerNotQuery) -> Self {
        Self::CssContainerNotQuery(node)
    }
}
impl From<CssContainerOrQuery> for AnyCssContainerQuery {
    fn from(node: CssContainerOrQuery) -> Self {
        Self::CssContainerOrQuery(node)
    }
}
impl AstNode for AnyCssContainerQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyCssContainerQueryInParens::KIND_SET
        .union(CssContainerAndQuery::KIND_SET)
        .union(CssContainerNotQuery::KIND_SET)
        .union(CssContainerOrQuery::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_CONTAINER_AND_QUERY | CSS_CONTAINER_NOT_QUERY | CSS_CONTAINER_OR_QUERY => true,
            k if AnyCssContainerQueryInParens::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_CONTAINER_AND_QUERY => Self::CssContainerAndQuery(CssContainerAndQuery { syntax }),
            CSS_CONTAINER_NOT_QUERY => Self::CssContainerNotQuery(CssContainerNotQuery { syntax }),
            CSS_CONTAINER_OR_QUERY => Self::CssContainerOrQuery(CssContainerOrQuery { syntax }),
            _ => {
                if let Some(any_css_container_query_in_parens) =
                    AnyCssContainerQueryInParens::cast(syntax)
                {
                    return Some(Self::AnyCssContainerQueryInParens(
                        any_css_container_query_in_parens,
                    ));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssContainerAndQuery(it) => &it.syntax,
            Self::CssContainerNotQuery(it) => &it.syntax,
            Self::CssContainerOrQuery(it) => &it.syntax,
            Self::AnyCssContainerQueryInParens(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssContainerAndQuery(it) => it.syntax,
            Self::CssContainerNotQuery(it) => it.syntax,
            Self::CssContainerOrQuery(it) => it.syntax,
            Self::AnyCssContainerQueryInParens(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssContainerQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyCssContainerQueryInParens(it) => std::fmt::Debug::fmt(it, f),
            Self::CssContainerAndQuery(it) => std::fmt::Debug::fmt(it, f),
            Self::CssContainerNotQuery(it) => std::fmt::Debug::fmt(it, f),
            Self::CssContainerOrQuery(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssContainerQuery> for SyntaxNode {
    fn from(n: AnyCssContainerQuery) -> Self {
        match n {
            AnyCssContainerQuery::AnyCssContainerQueryInParens(it) => it.into(),
            AnyCssContainerQuery::CssContainerAndQuery(it) => it.into(),
            AnyCssContainerQuery::CssContainerNotQuery(it) => it.into(),
            AnyCssContainerQuery::CssContainerOrQuery(it) => it.into(),
        }
    }
}
impl From<AnyCssContainerQuery> for SyntaxElement {
    fn from(n: AnyCssContainerQuery) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssContainerQueryInParens> for AnyCssContainerQueryInParens {
    fn from(node: CssContainerQueryInParens) -> Self {
        Self::CssContainerQueryInParens(node)
    }
}
impl From<CssContainerSizeFeatureInParens> for AnyCssContainerQueryInParens {
    fn from(node: CssContainerSizeFeatureInParens) -> Self {
        Self::CssContainerSizeFeatureInParens(node)
    }
}
impl From<CssContainerStyleQueryInParens> for AnyCssContainerQueryInParens {
    fn from(node: CssContainerStyleQueryInParens) -> Self {
        Self::CssContainerStyleQueryInParens(node)
    }
}
impl AstNode for AnyCssContainerQueryInParens {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssContainerQueryInParens::KIND_SET
        .union(CssContainerSizeFeatureInParens::KIND_SET)
        .union(CssContainerStyleQueryInParens::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_CONTAINER_QUERY_IN_PARENS
                | CSS_CONTAINER_SIZE_FEATURE_IN_PARENS
                | CSS_CONTAINER_STYLE_QUERY_IN_PARENS
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_CONTAINER_QUERY_IN_PARENS => {
                Self::CssContainerQueryInParens(CssContainerQueryInParens { syntax })
            }
            CSS_CONTAINER_SIZE_FEATURE_IN_PARENS => {
                Self::CssContainerSizeFeatureInParens(CssContainerSizeFeatureInParens { syntax })
            }
            CSS_CONTAINER_STYLE_QUERY_IN_PARENS => {
                Self::CssContainerStyleQueryInParens(CssContainerStyleQueryInParens { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssContainerQueryInParens(it) => &it.syntax,
            Self::CssContainerSizeFeatureInParens(it) => &it.syntax,
            Self::CssContainerStyleQueryInParens(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssContainerQueryInParens(it) => it.syntax,
            Self::CssContainerSizeFeatureInParens(it) => it.syntax,
            Self::CssContainerStyleQueryInParens(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssContainerQueryInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssContainerQueryInParens(it) => std::fmt::Debug::fmt(it, f),
            Self::CssContainerSizeFeatureInParens(it) => std::fmt::Debug::fmt(it, f),
            Self::CssContainerStyleQueryInParens(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssContainerQueryInParens> for SyntaxNode {
    fn from(n: AnyCssContainerQueryInParens) -> Self {
        match n {
            AnyCssContainerQueryInParens::CssContainerQueryInParens(it) => it.into(),
            AnyCssContainerQueryInParens::CssContainerSizeFeatureInParens(it) => it.into(),
            AnyCssContainerQueryInParens::CssContainerStyleQueryInParens(it) => it.into(),
        }
    }
}
impl From<AnyCssContainerQueryInParens> for SyntaxElement {
    fn from(n: AnyCssContainerQueryInParens) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssContainerStyleAndQuery> for AnyCssContainerStyleAndCombinableQuery {
    fn from(node: CssContainerStyleAndQuery) -> Self {
        Self::CssContainerStyleAndQuery(node)
    }
}
impl From<CssContainerStyleInParens> for AnyCssContainerStyleAndCombinableQuery {
    fn from(node: CssContainerStyleInParens) -> Self {
        Self::CssContainerStyleInParens(node)
    }
}
impl AstNode for AnyCssContainerStyleAndCombinableQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssContainerStyleAndQuery::KIND_SET.union(CssContainerStyleInParens::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_CONTAINER_STYLE_AND_QUERY | CSS_CONTAINER_STYLE_IN_PARENS
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_CONTAINER_STYLE_AND_QUERY => {
                Self::CssContainerStyleAndQuery(CssContainerStyleAndQuery { syntax })
            }
            CSS_CONTAINER_STYLE_IN_PARENS => {
                Self::CssContainerStyleInParens(CssContainerStyleInParens { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssContainerStyleAndQuery(it) => &it.syntax,
            Self::CssContainerStyleInParens(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssContainerStyleAndQuery(it) => it.syntax,
            Self::CssContainerStyleInParens(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssContainerStyleAndCombinableQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssContainerStyleAndQuery(it) => std::fmt::Debug::fmt(it, f),
            Self::CssContainerStyleInParens(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssContainerStyleAndCombinableQuery> for SyntaxNode {
    fn from(n: AnyCssContainerStyleAndCombinableQuery) -> Self {
        match n {
            AnyCssContainerStyleAndCombinableQuery::CssContainerStyleAndQuery(it) => it.into(),
            AnyCssContainerStyleAndCombinableQuery::CssContainerStyleInParens(it) => it.into(),
        }
    }
}
impl From<AnyCssContainerStyleAndCombinableQuery> for SyntaxElement {
    fn from(n: AnyCssContainerStyleAndCombinableQuery) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssDeclaration> for AnyCssContainerStyleInParens {
    fn from(node: CssDeclaration) -> Self {
        Self::CssDeclaration(node)
    }
}
impl AstNode for AnyCssContainerStyleInParens {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyCssContainerStyleQuery::KIND_SET.union(CssDeclaration::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_DECLARATION => true,
            k if AnyCssContainerStyleQuery::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_DECLARATION => Self::CssDeclaration(CssDeclaration { syntax }),
            _ => {
                if let Some(any_css_container_style_query) = AnyCssContainerStyleQuery::cast(syntax)
                {
                    return Some(Self::AnyCssContainerStyleQuery(
                        any_css_container_style_query,
                    ));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssDeclaration(it) => &it.syntax,
            Self::AnyCssContainerStyleQuery(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssDeclaration(it) => it.syntax,
            Self::AnyCssContainerStyleQuery(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssContainerStyleInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyCssContainerStyleQuery(it) => std::fmt::Debug::fmt(it, f),
            Self::CssDeclaration(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssContainerStyleInParens> for SyntaxNode {
    fn from(n: AnyCssContainerStyleInParens) -> Self {
        match n {
            AnyCssContainerStyleInParens::AnyCssContainerStyleQuery(it) => it.into(),
            AnyCssContainerStyleInParens::CssDeclaration(it) => it.into(),
        }
    }
}
impl From<AnyCssContainerStyleInParens> for SyntaxElement {
    fn from(n: AnyCssContainerStyleInParens) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssContainerStyleInParens> for AnyCssContainerStyleOrCombinableQuery {
    fn from(node: CssContainerStyleInParens) -> Self {
        Self::CssContainerStyleInParens(node)
    }
}
impl From<CssContainerStyleOrQuery> for AnyCssContainerStyleOrCombinableQuery {
    fn from(node: CssContainerStyleOrQuery) -> Self {
        Self::CssContainerStyleOrQuery(node)
    }
}
impl AstNode for AnyCssContainerStyleOrCombinableQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssContainerStyleInParens::KIND_SET.union(CssContainerStyleOrQuery::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_CONTAINER_STYLE_IN_PARENS | CSS_CONTAINER_STYLE_OR_QUERY
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_CONTAINER_STYLE_IN_PARENS => {
                Self::CssContainerStyleInParens(CssContainerStyleInParens { syntax })
            }
            CSS_CONTAINER_STYLE_OR_QUERY => {
                Self::CssContainerStyleOrQuery(CssContainerStyleOrQuery { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssContainerStyleInParens(it) => &it.syntax,
            Self::CssContainerStyleOrQuery(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssContainerStyleInParens(it) => it.syntax,
            Self::CssContainerStyleOrQuery(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssContainerStyleOrCombinableQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssContainerStyleInParens(it) => std::fmt::Debug::fmt(it, f),
            Self::CssContainerStyleOrQuery(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssContainerStyleOrCombinableQuery> for SyntaxNode {
    fn from(n: AnyCssContainerStyleOrCombinableQuery) -> Self {
        match n {
            AnyCssContainerStyleOrCombinableQuery::CssContainerStyleInParens(it) => it.into(),
            AnyCssContainerStyleOrCombinableQuery::CssContainerStyleOrQuery(it) => it.into(),
        }
    }
}
impl From<AnyCssContainerStyleOrCombinableQuery> for SyntaxElement {
    fn from(n: AnyCssContainerStyleOrCombinableQuery) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssContainerStyleAndQuery> for AnyCssContainerStyleQuery {
    fn from(node: CssContainerStyleAndQuery) -> Self {
        Self::CssContainerStyleAndQuery(node)
    }
}
impl From<CssContainerStyleInParens> for AnyCssContainerStyleQuery {
    fn from(node: CssContainerStyleInParens) -> Self {
        Self::CssContainerStyleInParens(node)
    }
}
impl From<CssContainerStyleNotQuery> for AnyCssContainerStyleQuery {
    fn from(node: CssContainerStyleNotQuery) -> Self {
        Self::CssContainerStyleNotQuery(node)
    }
}
impl From<CssContainerStyleOrQuery> for AnyCssContainerStyleQuery {
    fn from(node: CssContainerStyleOrQuery) -> Self {
        Self::CssContainerStyleOrQuery(node)
    }
}
impl From<CssDeclaration> for AnyCssContainerStyleQuery {
    fn from(node: CssDeclaration) -> Self {
        Self::CssDeclaration(node)
    }
}
impl AstNode for AnyCssContainerStyleQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssContainerStyleAndQuery::KIND_SET
        .union(CssContainerStyleInParens::KIND_SET)
        .union(CssContainerStyleNotQuery::KIND_SET)
        .union(CssContainerStyleOrQuery::KIND_SET)
        .union(CssDeclaration::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_CONTAINER_STYLE_AND_QUERY
                | CSS_CONTAINER_STYLE_IN_PARENS
                | CSS_CONTAINER_STYLE_NOT_QUERY
                | CSS_CONTAINER_STYLE_OR_QUERY
                | CSS_DECLARATION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_CONTAINER_STYLE_AND_QUERY => {
                Self::CssContainerStyleAndQuery(CssContainerStyleAndQuery { syntax })
            }
            CSS_CONTAINER_STYLE_IN_PARENS => {
                Self::CssContainerStyleInParens(CssContainerStyleInParens { syntax })
            }
            CSS_CONTAINER_STYLE_NOT_QUERY => {
                Self::CssContainerStyleNotQuery(CssContainerStyleNotQuery { syntax })
            }
            CSS_CONTAINER_STYLE_OR_QUERY => {
                Self::CssContainerStyleOrQuery(CssContainerStyleOrQuery { syntax })
            }
            CSS_DECLARATION => Self::CssDeclaration(CssDeclaration { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssContainerStyleAndQuery(it) => &it.syntax,
            Self::CssContainerStyleInParens(it) => &it.syntax,
            Self::CssContainerStyleNotQuery(it) => &it.syntax,
            Self::CssContainerStyleOrQuery(it) => &it.syntax,
            Self::CssDeclaration(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssContainerStyleAndQuery(it) => it.syntax,
            Self::CssContainerStyleInParens(it) => it.syntax,
            Self::CssContainerStyleNotQuery(it) => it.syntax,
            Self::CssContainerStyleOrQuery(it) => it.syntax,
            Self::CssDeclaration(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssContainerStyleQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssContainerStyleAndQuery(it) => std::fmt::Debug::fmt(it, f),
            Self::CssContainerStyleInParens(it) => std::fmt::Debug::fmt(it, f),
            Self::CssContainerStyleNotQuery(it) => std::fmt::Debug::fmt(it, f),
            Self::CssContainerStyleOrQuery(it) => std::fmt::Debug::fmt(it, f),
            Self::CssDeclaration(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssContainerStyleQuery> for SyntaxNode {
    fn from(n: AnyCssContainerStyleQuery) -> Self {
        match n {
            AnyCssContainerStyleQuery::CssContainerStyleAndQuery(it) => it.into(),
            AnyCssContainerStyleQuery::CssContainerStyleInParens(it) => it.into(),
            AnyCssContainerStyleQuery::CssContainerStyleNotQuery(it) => it.into(),
            AnyCssContainerStyleQuery::CssContainerStyleOrQuery(it) => it.into(),
            AnyCssContainerStyleQuery::CssDeclaration(it) => it.into(),
        }
    }
}
impl From<AnyCssContainerStyleQuery> for SyntaxElement {
    fn from(n: AnyCssContainerStyleQuery) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusCustomIdentifier> for AnyCssCustomIdentifier {
    fn from(node: CssBogusCustomIdentifier) -> Self {
        Self::CssBogusCustomIdentifier(node)
    }
}
impl From<CssCustomIdentifier> for AnyCssCustomIdentifier {
    fn from(node: CssCustomIdentifier) -> Self {
        Self::CssCustomIdentifier(node)
    }
}
impl AstNode for AnyCssCustomIdentifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssBogusCustomIdentifier::KIND_SET.union(CssCustomIdentifier::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_BOGUS_CUSTOM_IDENTIFIER | CSS_CUSTOM_IDENTIFIER)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_CUSTOM_IDENTIFIER => {
                Self::CssBogusCustomIdentifier(CssBogusCustomIdentifier { syntax })
            }
            CSS_CUSTOM_IDENTIFIER => Self::CssCustomIdentifier(CssCustomIdentifier { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusCustomIdentifier(it) => &it.syntax,
            Self::CssCustomIdentifier(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusCustomIdentifier(it) => it.syntax,
            Self::CssCustomIdentifier(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssCustomIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusCustomIdentifier(it) => std::fmt::Debug::fmt(it, f),
            Self::CssCustomIdentifier(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssCustomIdentifier> for SyntaxNode {
    fn from(n: AnyCssCustomIdentifier) -> Self {
        match n {
            AnyCssCustomIdentifier::CssBogusCustomIdentifier(it) => it.into(),
            AnyCssCustomIdentifier::CssCustomIdentifier(it) => it.into(),
        }
    }
}
impl From<AnyCssCustomIdentifier> for SyntaxElement {
    fn from(n: AnyCssCustomIdentifier) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssDeclarationWithSemicolon> for AnyCssDeclaration {
    fn from(node: CssDeclarationWithSemicolon) -> Self {
        Self::CssDeclarationWithSemicolon(node)
    }
}
impl From<CssEmptyDeclaration> for AnyCssDeclaration {
    fn from(node: CssEmptyDeclaration) -> Self {
        Self::CssEmptyDeclaration(node)
    }
}
impl AstNode for AnyCssDeclaration {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssDeclarationWithSemicolon::KIND_SET.union(CssEmptyDeclaration::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_DECLARATION_WITH_SEMICOLON | CSS_EMPTY_DECLARATION)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_DECLARATION_WITH_SEMICOLON => {
                Self::CssDeclarationWithSemicolon(CssDeclarationWithSemicolon { syntax })
            }
            CSS_EMPTY_DECLARATION => Self::CssEmptyDeclaration(CssEmptyDeclaration { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssDeclarationWithSemicolon(it) => &it.syntax,
            Self::CssEmptyDeclaration(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssDeclarationWithSemicolon(it) => it.syntax,
            Self::CssEmptyDeclaration(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssDeclarationWithSemicolon(it) => std::fmt::Debug::fmt(it, f),
            Self::CssEmptyDeclaration(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssDeclaration> for SyntaxNode {
    fn from(n: AnyCssDeclaration) -> Self {
        match n {
            AnyCssDeclaration::CssDeclarationWithSemicolon(it) => it.into(),
            AnyCssDeclaration::CssEmptyDeclaration(it) => it.into(),
        }
    }
}
impl From<AnyCssDeclaration> for SyntaxElement {
    fn from(n: AnyCssDeclaration) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusBlock> for AnyCssDeclarationBlock {
    fn from(node: CssBogusBlock) -> Self {
        Self::CssBogusBlock(node)
    }
}
impl From<CssDeclarationBlock> for AnyCssDeclarationBlock {
    fn from(node: CssDeclarationBlock) -> Self {
        Self::CssDeclarationBlock(node)
    }
}
impl AstNode for AnyCssDeclarationBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssBogusBlock::KIND_SET.union(CssDeclarationBlock::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_BOGUS_BLOCK | CSS_DECLARATION_BLOCK)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_BLOCK => Self::CssBogusBlock(CssBogusBlock { syntax }),
            CSS_DECLARATION_BLOCK => Self::CssDeclarationBlock(CssDeclarationBlock { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusBlock(it) => &it.syntax,
            Self::CssDeclarationBlock(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusBlock(it) => it.syntax,
            Self::CssDeclarationBlock(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssDeclarationBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::CssDeclarationBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssDeclarationBlock> for SyntaxNode {
    fn from(n: AnyCssDeclarationBlock) -> Self {
        match n {
            AnyCssDeclarationBlock::CssBogusBlock(it) => it.into(),
            AnyCssDeclarationBlock::CssDeclarationBlock(it) => it.into(),
        }
    }
}
impl From<AnyCssDeclarationBlock> for SyntaxElement {
    fn from(n: AnyCssDeclarationBlock) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssDashedIdentifier> for AnyCssDeclarationName {
    fn from(node: CssDashedIdentifier) -> Self {
        Self::CssDashedIdentifier(node)
    }
}
impl From<CssIdentifier> for AnyCssDeclarationName {
    fn from(node: CssIdentifier) -> Self {
        Self::CssIdentifier(node)
    }
}
impl AstNode for AnyCssDeclarationName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssDashedIdentifier::KIND_SET.union(CssIdentifier::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_DASHED_IDENTIFIER | CSS_IDENTIFIER)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_DASHED_IDENTIFIER => Self::CssDashedIdentifier(CssDashedIdentifier { syntax }),
            CSS_IDENTIFIER => Self::CssIdentifier(CssIdentifier { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssDashedIdentifier(it) => &it.syntax,
            Self::CssIdentifier(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssDashedIdentifier(it) => it.syntax,
            Self::CssIdentifier(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssDeclarationName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssDashedIdentifier(it) => std::fmt::Debug::fmt(it, f),
            Self::CssIdentifier(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssDeclarationName> for SyntaxNode {
    fn from(n: AnyCssDeclarationName) -> Self {
        match n {
            AnyCssDeclarationName::CssDashedIdentifier(it) => it.into(),
            AnyCssDeclarationName::CssIdentifier(it) => it.into(),
        }
    }
}
impl From<AnyCssDeclarationName> for SyntaxElement {
    fn from(n: AnyCssDeclarationName) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssAtRule> for AnyCssDeclarationOrAtRule {
    fn from(node: CssAtRule) -> Self {
        Self::CssAtRule(node)
    }
}
impl From<CssDeclarationWithSemicolon> for AnyCssDeclarationOrAtRule {
    fn from(node: CssDeclarationWithSemicolon) -> Self {
        Self::CssDeclarationWithSemicolon(node)
    }
}
impl From<CssEmptyDeclaration> for AnyCssDeclarationOrAtRule {
    fn from(node: CssEmptyDeclaration) -> Self {
        Self::CssEmptyDeclaration(node)
    }
}
impl AstNode for AnyCssDeclarationOrAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssAtRule::KIND_SET
        .union(CssDeclarationWithSemicolon::KIND_SET)
        .union(CssEmptyDeclaration::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_AT_RULE | CSS_DECLARATION_WITH_SEMICOLON | CSS_EMPTY_DECLARATION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_AT_RULE => Self::CssAtRule(CssAtRule { syntax }),
            CSS_DECLARATION_WITH_SEMICOLON => {
                Self::CssDeclarationWithSemicolon(CssDeclarationWithSemicolon { syntax })
            }
            CSS_EMPTY_DECLARATION => Self::CssEmptyDeclaration(CssEmptyDeclaration { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssAtRule(it) => &it.syntax,
            Self::CssDeclarationWithSemicolon(it) => &it.syntax,
            Self::CssEmptyDeclaration(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssAtRule(it) => it.syntax,
            Self::CssDeclarationWithSemicolon(it) => it.syntax,
            Self::CssEmptyDeclaration(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssDeclarationOrAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssDeclarationWithSemicolon(it) => std::fmt::Debug::fmt(it, f),
            Self::CssEmptyDeclaration(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssDeclarationOrAtRule> for SyntaxNode {
    fn from(n: AnyCssDeclarationOrAtRule) -> Self {
        match n {
            AnyCssDeclarationOrAtRule::CssAtRule(it) => it.into(),
            AnyCssDeclarationOrAtRule::CssDeclarationWithSemicolon(it) => it.into(),
            AnyCssDeclarationOrAtRule::CssEmptyDeclaration(it) => it.into(),
        }
    }
}
impl From<AnyCssDeclarationOrAtRule> for SyntaxElement {
    fn from(n: AnyCssDeclarationOrAtRule) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusBlock> for AnyCssDeclarationOrAtRuleBlock {
    fn from(node: CssBogusBlock) -> Self {
        Self::CssBogusBlock(node)
    }
}
impl From<CssDeclarationOrAtRuleBlock> for AnyCssDeclarationOrAtRuleBlock {
    fn from(node: CssDeclarationOrAtRuleBlock) -> Self {
        Self::CssDeclarationOrAtRuleBlock(node)
    }
}
impl AstNode for AnyCssDeclarationOrAtRuleBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssBogusBlock::KIND_SET.union(CssDeclarationOrAtRuleBlock::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_BOGUS_BLOCK | CSS_DECLARATION_OR_AT_RULE_BLOCK)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_BLOCK => Self::CssBogusBlock(CssBogusBlock { syntax }),
            CSS_DECLARATION_OR_AT_RULE_BLOCK => {
                Self::CssDeclarationOrAtRuleBlock(CssDeclarationOrAtRuleBlock { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusBlock(it) => &it.syntax,
            Self::CssDeclarationOrAtRuleBlock(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusBlock(it) => it.syntax,
            Self::CssDeclarationOrAtRuleBlock(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssDeclarationOrAtRuleBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::CssDeclarationOrAtRuleBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssDeclarationOrAtRuleBlock> for SyntaxNode {
    fn from(n: AnyCssDeclarationOrAtRuleBlock) -> Self {
        match n {
            AnyCssDeclarationOrAtRuleBlock::CssBogusBlock(it) => it.into(),
            AnyCssDeclarationOrAtRuleBlock::CssDeclarationOrAtRuleBlock(it) => it.into(),
        }
    }
}
impl From<AnyCssDeclarationOrAtRuleBlock> for SyntaxElement {
    fn from(n: AnyCssDeclarationOrAtRuleBlock) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogus> for AnyCssDeclarationOrRule {
    fn from(node: CssBogus) -> Self {
        Self::CssBogus(node)
    }
}
impl From<CssDeclarationWithSemicolon> for AnyCssDeclarationOrRule {
    fn from(node: CssDeclarationWithSemicolon) -> Self {
        Self::CssDeclarationWithSemicolon(node)
    }
}
impl From<CssEmptyDeclaration> for AnyCssDeclarationOrRule {
    fn from(node: CssEmptyDeclaration) -> Self {
        Self::CssEmptyDeclaration(node)
    }
}
impl From<CssMetavariable> for AnyCssDeclarationOrRule {
    fn from(node: CssMetavariable) -> Self {
        Self::CssMetavariable(node)
    }
}
impl AstNode for AnyCssDeclarationOrRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyCssRule::KIND_SET
        .union(CssBogus::KIND_SET)
        .union(CssDeclarationWithSemicolon::KIND_SET)
        .union(CssEmptyDeclaration::KIND_SET)
        .union(CssMetavariable::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_BOGUS
            | CSS_DECLARATION_WITH_SEMICOLON
            | CSS_EMPTY_DECLARATION
            | CSS_METAVARIABLE => true,
            k if AnyCssRule::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS => Self::CssBogus(CssBogus { syntax }),
            CSS_DECLARATION_WITH_SEMICOLON => {
                Self::CssDeclarationWithSemicolon(CssDeclarationWithSemicolon { syntax })
            }
            CSS_EMPTY_DECLARATION => Self::CssEmptyDeclaration(CssEmptyDeclaration { syntax }),
            CSS_METAVARIABLE => Self::CssMetavariable(CssMetavariable { syntax }),
            _ => {
                if let Some(any_css_rule) = AnyCssRule::cast(syntax) {
                    return Some(Self::AnyCssRule(any_css_rule));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogus(it) => &it.syntax,
            Self::CssDeclarationWithSemicolon(it) => &it.syntax,
            Self::CssEmptyDeclaration(it) => &it.syntax,
            Self::CssMetavariable(it) => &it.syntax,
            Self::AnyCssRule(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogus(it) => it.syntax,
            Self::CssDeclarationWithSemicolon(it) => it.syntax,
            Self::CssEmptyDeclaration(it) => it.syntax,
            Self::CssMetavariable(it) => it.syntax,
            Self::AnyCssRule(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssDeclarationOrRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyCssRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssBogus(it) => std::fmt::Debug::fmt(it, f),
            Self::CssDeclarationWithSemicolon(it) => std::fmt::Debug::fmt(it, f),
            Self::CssEmptyDeclaration(it) => std::fmt::Debug::fmt(it, f),
            Self::CssMetavariable(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssDeclarationOrRule> for SyntaxNode {
    fn from(n: AnyCssDeclarationOrRule) -> Self {
        match n {
            AnyCssDeclarationOrRule::AnyCssRule(it) => it.into(),
            AnyCssDeclarationOrRule::CssBogus(it) => it.into(),
            AnyCssDeclarationOrRule::CssDeclarationWithSemicolon(it) => it.into(),
            AnyCssDeclarationOrRule::CssEmptyDeclaration(it) => it.into(),
            AnyCssDeclarationOrRule::CssMetavariable(it) => it.into(),
        }
    }
}
impl From<AnyCssDeclarationOrRule> for SyntaxElement {
    fn from(n: AnyCssDeclarationOrRule) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusBlock> for AnyCssDeclarationOrRuleBlock {
    fn from(node: CssBogusBlock) -> Self {
        Self::CssBogusBlock(node)
    }
}
impl From<CssDeclarationOrRuleBlock> for AnyCssDeclarationOrRuleBlock {
    fn from(node: CssDeclarationOrRuleBlock) -> Self {
        Self::CssDeclarationOrRuleBlock(node)
    }
}
impl AstNode for AnyCssDeclarationOrRuleBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssBogusBlock::KIND_SET.union(CssDeclarationOrRuleBlock::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_BOGUS_BLOCK | CSS_DECLARATION_OR_RULE_BLOCK)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_BLOCK => Self::CssBogusBlock(CssBogusBlock { syntax }),
            CSS_DECLARATION_OR_RULE_BLOCK => {
                Self::CssDeclarationOrRuleBlock(CssDeclarationOrRuleBlock { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusBlock(it) => &it.syntax,
            Self::CssDeclarationOrRuleBlock(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusBlock(it) => it.syntax,
            Self::CssDeclarationOrRuleBlock(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssDeclarationOrRuleBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::CssDeclarationOrRuleBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssDeclarationOrRuleBlock> for SyntaxNode {
    fn from(n: AnyCssDeclarationOrRuleBlock) -> Self {
        match n {
            AnyCssDeclarationOrRuleBlock::CssBogusBlock(it) => it.into(),
            AnyCssDeclarationOrRuleBlock::CssDeclarationOrRuleBlock(it) => it.into(),
        }
    }
}
impl From<AnyCssDeclarationOrRuleBlock> for SyntaxElement {
    fn from(n: AnyCssDeclarationOrRuleBlock) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssPercentage> for AnyCssDimension {
    fn from(node: CssPercentage) -> Self {
        Self::CssPercentage(node)
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
    const KIND_SET: SyntaxKindSet<Language> = CssPercentage::KIND_SET
        .union(CssRegularDimension::KIND_SET)
        .union(CssUnknownDimension::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_PERCENTAGE | CSS_REGULAR_DIMENSION | CSS_UNKNOWN_DIMENSION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_PERCENTAGE => Self::CssPercentage(CssPercentage { syntax }),
            CSS_REGULAR_DIMENSION => Self::CssRegularDimension(CssRegularDimension { syntax }),
            CSS_UNKNOWN_DIMENSION => Self::CssUnknownDimension(CssUnknownDimension { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssPercentage(it) => &it.syntax,
            Self::CssRegularDimension(it) => &it.syntax,
            Self::CssUnknownDimension(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssPercentage(it) => it.syntax,
            Self::CssRegularDimension(it) => it.syntax,
            Self::CssUnknownDimension(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssPercentage(it) => std::fmt::Debug::fmt(it, f),
            Self::CssRegularDimension(it) => std::fmt::Debug::fmt(it, f),
            Self::CssUnknownDimension(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssDimension> for SyntaxNode {
    fn from(n: AnyCssDimension) -> Self {
        match n {
            AnyCssDimension::CssPercentage(it) => it.into(),
            AnyCssDimension::CssRegularDimension(it) => it.into(),
            AnyCssDimension::CssUnknownDimension(it) => it.into(),
        }
    }
}
impl From<AnyCssDimension> for SyntaxElement {
    fn from(n: AnyCssDimension) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusDocumentMatcher> for AnyCssDocumentMatcher {
    fn from(node: CssBogusDocumentMatcher) -> Self {
        Self::CssBogusDocumentMatcher(node)
    }
}
impl From<CssDocumentCustomMatcher> for AnyCssDocumentMatcher {
    fn from(node: CssDocumentCustomMatcher) -> Self {
        Self::CssDocumentCustomMatcher(node)
    }
}
impl From<CssUrlFunction> for AnyCssDocumentMatcher {
    fn from(node: CssUrlFunction) -> Self {
        Self::CssUrlFunction(node)
    }
}
impl AstNode for AnyCssDocumentMatcher {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssBogusDocumentMatcher::KIND_SET
        .union(CssDocumentCustomMatcher::KIND_SET)
        .union(CssUrlFunction::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_BOGUS_DOCUMENT_MATCHER | CSS_DOCUMENT_CUSTOM_MATCHER | CSS_URL_FUNCTION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_DOCUMENT_MATCHER => {
                Self::CssBogusDocumentMatcher(CssBogusDocumentMatcher { syntax })
            }
            CSS_DOCUMENT_CUSTOM_MATCHER => {
                Self::CssDocumentCustomMatcher(CssDocumentCustomMatcher { syntax })
            }
            CSS_URL_FUNCTION => Self::CssUrlFunction(CssUrlFunction { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusDocumentMatcher(it) => &it.syntax,
            Self::CssDocumentCustomMatcher(it) => &it.syntax,
            Self::CssUrlFunction(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusDocumentMatcher(it) => it.syntax,
            Self::CssDocumentCustomMatcher(it) => it.syntax,
            Self::CssUrlFunction(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssDocumentMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusDocumentMatcher(it) => std::fmt::Debug::fmt(it, f),
            Self::CssDocumentCustomMatcher(it) => std::fmt::Debug::fmt(it, f),
            Self::CssUrlFunction(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssDocumentMatcher> for SyntaxNode {
    fn from(n: AnyCssDocumentMatcher) -> Self {
        match n {
            AnyCssDocumentMatcher::CssBogusDocumentMatcher(it) => it.into(),
            AnyCssDocumentMatcher::CssDocumentCustomMatcher(it) => it.into(),
            AnyCssDocumentMatcher::CssUrlFunction(it) => it.into(),
        }
    }
}
impl From<AnyCssDocumentMatcher> for SyntaxElement {
    fn from(n: AnyCssDocumentMatcher) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBinaryExpression> for AnyCssExpression {
    fn from(node: CssBinaryExpression) -> Self {
        Self::CssBinaryExpression(node)
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
impl AstNode for AnyCssExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssBinaryExpression::KIND_SET
        .union(CssListOfComponentValuesExpression::KIND_SET)
        .union(CssParenthesizedExpression::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_BINARY_EXPRESSION
                | CSS_LIST_OF_COMPONENT_VALUES_EXPRESSION
                | CSS_PARENTHESIZED_EXPRESSION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BINARY_EXPRESSION => Self::CssBinaryExpression(CssBinaryExpression { syntax }),
            CSS_LIST_OF_COMPONENT_VALUES_EXPRESSION => {
                Self::CssListOfComponentValuesExpression(CssListOfComponentValuesExpression {
                    syntax,
                })
            }
            CSS_PARENTHESIZED_EXPRESSION => {
                Self::CssParenthesizedExpression(CssParenthesizedExpression { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBinaryExpression(it) => &it.syntax,
            Self::CssListOfComponentValuesExpression(it) => &it.syntax,
            Self::CssParenthesizedExpression(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBinaryExpression(it) => it.syntax,
            Self::CssListOfComponentValuesExpression(it) => it.syntax,
            Self::CssParenthesizedExpression(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBinaryExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::CssListOfComponentValuesExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::CssParenthesizedExpression(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssExpression> for SyntaxNode {
    fn from(n: AnyCssExpression) -> Self {
        match n {
            AnyCssExpression::CssBinaryExpression(it) => it.into(),
            AnyCssExpression::CssListOfComponentValuesExpression(it) => it.into(),
            AnyCssExpression::CssParenthesizedExpression(it) => it.into(),
        }
    }
}
impl From<AnyCssExpression> for SyntaxElement {
    fn from(n: AnyCssExpression) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusFontFamilyName> for AnyCssFontFamilyName {
    fn from(node: CssBogusFontFamilyName) -> Self {
        Self::CssBogusFontFamilyName(node)
    }
}
impl From<CssFontFamilyName> for AnyCssFontFamilyName {
    fn from(node: CssFontFamilyName) -> Self {
        Self::CssFontFamilyName(node)
    }
}
impl From<CssString> for AnyCssFontFamilyName {
    fn from(node: CssString) -> Self {
        Self::CssString(node)
    }
}
impl AstNode for AnyCssFontFamilyName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssBogusFontFamilyName::KIND_SET
        .union(CssFontFamilyName::KIND_SET)
        .union(CssString::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_BOGUS_FONT_FAMILY_NAME | CSS_FONT_FAMILY_NAME | CSS_STRING
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_FONT_FAMILY_NAME => {
                Self::CssBogusFontFamilyName(CssBogusFontFamilyName { syntax })
            }
            CSS_FONT_FAMILY_NAME => Self::CssFontFamilyName(CssFontFamilyName { syntax }),
            CSS_STRING => Self::CssString(CssString { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusFontFamilyName(it) => &it.syntax,
            Self::CssFontFamilyName(it) => &it.syntax,
            Self::CssString(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusFontFamilyName(it) => it.syntax,
            Self::CssFontFamilyName(it) => it.syntax,
            Self::CssString(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssFontFamilyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusFontFamilyName(it) => std::fmt::Debug::fmt(it, f),
            Self::CssFontFamilyName(it) => std::fmt::Debug::fmt(it, f),
            Self::CssString(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssFontFamilyName> for SyntaxNode {
    fn from(n: AnyCssFontFamilyName) -> Self {
        match n {
            AnyCssFontFamilyName::CssBogusFontFamilyName(it) => it.into(),
            AnyCssFontFamilyName::CssFontFamilyName(it) => it.into(),
            AnyCssFontFamilyName::CssString(it) => it.into(),
        }
    }
}
impl From<AnyCssFontFamilyName> for SyntaxElement {
    fn from(n: AnyCssFontFamilyName) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusBlock> for AnyCssFontFeatureValuesBlock {
    fn from(node: CssBogusBlock) -> Self {
        Self::CssBogusBlock(node)
    }
}
impl From<CssFontFeatureValuesBlock> for AnyCssFontFeatureValuesBlock {
    fn from(node: CssFontFeatureValuesBlock) -> Self {
        Self::CssFontFeatureValuesBlock(node)
    }
}
impl AstNode for AnyCssFontFeatureValuesBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssBogusBlock::KIND_SET.union(CssFontFeatureValuesBlock::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_BOGUS_BLOCK | CSS_FONT_FEATURE_VALUES_BLOCK)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_BLOCK => Self::CssBogusBlock(CssBogusBlock { syntax }),
            CSS_FONT_FEATURE_VALUES_BLOCK => {
                Self::CssFontFeatureValuesBlock(CssFontFeatureValuesBlock { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusBlock(it) => &it.syntax,
            Self::CssFontFeatureValuesBlock(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusBlock(it) => it.syntax,
            Self::CssFontFeatureValuesBlock(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssFontFeatureValuesBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::CssFontFeatureValuesBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssFontFeatureValuesBlock> for SyntaxNode {
    fn from(n: AnyCssFontFeatureValuesBlock) -> Self {
        match n {
            AnyCssFontFeatureValuesBlock::CssBogusBlock(it) => it.into(),
            AnyCssFontFeatureValuesBlock::CssFontFeatureValuesBlock(it) => it.into(),
        }
    }
}
impl From<AnyCssFontFeatureValuesBlock> for SyntaxElement {
    fn from(n: AnyCssFontFeatureValuesBlock) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusFontFeatureValuesItem> for AnyCssFontFeatureValuesItem {
    fn from(node: CssBogusFontFeatureValuesItem) -> Self {
        Self::CssBogusFontFeatureValuesItem(node)
    }
}
impl From<CssFontFeatureValuesItem> for AnyCssFontFeatureValuesItem {
    fn from(node: CssFontFeatureValuesItem) -> Self {
        Self::CssFontFeatureValuesItem(node)
    }
}
impl AstNode for AnyCssFontFeatureValuesItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssBogusFontFeatureValuesItem::KIND_SET.union(CssFontFeatureValuesItem::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_BOGUS_FONT_FEATURE_VALUES_ITEM | CSS_FONT_FEATURE_VALUES_ITEM
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_FONT_FEATURE_VALUES_ITEM => {
                Self::CssBogusFontFeatureValuesItem(CssBogusFontFeatureValuesItem { syntax })
            }
            CSS_FONT_FEATURE_VALUES_ITEM => {
                Self::CssFontFeatureValuesItem(CssFontFeatureValuesItem { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusFontFeatureValuesItem(it) => &it.syntax,
            Self::CssFontFeatureValuesItem(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusFontFeatureValuesItem(it) => it.syntax,
            Self::CssFontFeatureValuesItem(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssFontFeatureValuesItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusFontFeatureValuesItem(it) => std::fmt::Debug::fmt(it, f),
            Self::CssFontFeatureValuesItem(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssFontFeatureValuesItem> for SyntaxNode {
    fn from(n: AnyCssFontFeatureValuesItem) -> Self {
        match n {
            AnyCssFontFeatureValuesItem::CssBogusFontFeatureValuesItem(it) => it.into(),
            AnyCssFontFeatureValuesItem::CssFontFeatureValuesItem(it) => it.into(),
        }
    }
}
impl From<AnyCssFontFeatureValuesItem> for SyntaxElement {
    fn from(n: AnyCssFontFeatureValuesItem) -> Self {
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
            Self::CssFunction(it) => &it.syntax,
            Self::CssUrlFunction(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssFunction(it) => it.syntax,
            Self::CssUrlFunction(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssFunction(it) => std::fmt::Debug::fmt(it, f),
            Self::CssUrlFunction(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssFunction> for SyntaxNode {
    fn from(n: AnyCssFunction) -> Self {
        match n {
            AnyCssFunction::CssFunction(it) => it.into(),
            AnyCssFunction::CssUrlFunction(it) => it.into(),
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
            Self::CssGenericDelimiter(it) => &it.syntax,
            Self::AnyCssValue(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssGenericDelimiter(it) => it.syntax,
            Self::AnyCssValue(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssGenericComponentValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyCssValue(it) => std::fmt::Debug::fmt(it, f),
            Self::CssGenericDelimiter(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssGenericComponentValue> for SyntaxNode {
    fn from(n: AnyCssGenericComponentValue) -> Self {
        match n {
            AnyCssGenericComponentValue::AnyCssValue(it) => it.into(),
            AnyCssGenericComponentValue::CssGenericDelimiter(it) => it.into(),
        }
    }
}
impl From<AnyCssGenericComponentValue> for SyntaxElement {
    fn from(n: AnyCssGenericComponentValue) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssImportAnonymousLayer> for AnyCssImportLayer {
    fn from(node: CssImportAnonymousLayer) -> Self {
        Self::CssImportAnonymousLayer(node)
    }
}
impl From<CssImportNamedLayer> for AnyCssImportLayer {
    fn from(node: CssImportNamedLayer) -> Self {
        Self::CssImportNamedLayer(node)
    }
}
impl AstNode for AnyCssImportLayer {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssImportAnonymousLayer::KIND_SET.union(CssImportNamedLayer::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_IMPORT_ANONYMOUS_LAYER | CSS_IMPORT_NAMED_LAYER)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_IMPORT_ANONYMOUS_LAYER => {
                Self::CssImportAnonymousLayer(CssImportAnonymousLayer { syntax })
            }
            CSS_IMPORT_NAMED_LAYER => Self::CssImportNamedLayer(CssImportNamedLayer { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssImportAnonymousLayer(it) => &it.syntax,
            Self::CssImportNamedLayer(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssImportAnonymousLayer(it) => it.syntax,
            Self::CssImportNamedLayer(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssImportLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssImportAnonymousLayer(it) => std::fmt::Debug::fmt(it, f),
            Self::CssImportNamedLayer(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssImportLayer> for SyntaxNode {
    fn from(n: AnyCssImportLayer) -> Self {
        match n {
            AnyCssImportLayer::CssImportAnonymousLayer(it) => it.into(),
            AnyCssImportLayer::CssImportNamedLayer(it) => it.into(),
        }
    }
}
impl From<AnyCssImportLayer> for SyntaxElement {
    fn from(n: AnyCssImportLayer) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssDeclaration> for AnyCssImportSupportsCondition {
    fn from(node: CssDeclaration) -> Self {
        Self::CssDeclaration(node)
    }
}
impl AstNode for AnyCssImportSupportsCondition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyCssSupportsCondition::KIND_SET.union(CssDeclaration::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_DECLARATION => true,
            k if AnyCssSupportsCondition::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_DECLARATION => Self::CssDeclaration(CssDeclaration { syntax }),
            _ => {
                if let Some(any_css_supports_condition) = AnyCssSupportsCondition::cast(syntax) {
                    return Some(Self::AnyCssSupportsCondition(any_css_supports_condition));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssDeclaration(it) => &it.syntax,
            Self::AnyCssSupportsCondition(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssDeclaration(it) => it.syntax,
            Self::AnyCssSupportsCondition(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssImportSupportsCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyCssSupportsCondition(it) => std::fmt::Debug::fmt(it, f),
            Self::CssDeclaration(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssImportSupportsCondition> for SyntaxNode {
    fn from(n: AnyCssImportSupportsCondition) -> Self {
        match n {
            AnyCssImportSupportsCondition::AnyCssSupportsCondition(it) => it.into(),
            AnyCssImportSupportsCondition::CssDeclaration(it) => it.into(),
        }
    }
}
impl From<AnyCssImportSupportsCondition> for SyntaxElement {
    fn from(n: AnyCssImportSupportsCondition) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssString> for AnyCssImportUrl {
    fn from(node: CssString) -> Self {
        Self::CssString(node)
    }
}
impl From<CssUrlFunction> for AnyCssImportUrl {
    fn from(node: CssUrlFunction) -> Self {
        Self::CssUrlFunction(node)
    }
}
impl AstNode for AnyCssImportUrl {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssString::KIND_SET.union(CssUrlFunction::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_STRING | CSS_URL_FUNCTION)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_STRING => Self::CssString(CssString { syntax }),
            CSS_URL_FUNCTION => Self::CssUrlFunction(CssUrlFunction { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssString(it) => &it.syntax,
            Self::CssUrlFunction(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssString(it) => it.syntax,
            Self::CssUrlFunction(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssImportUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssString(it) => std::fmt::Debug::fmt(it, f),
            Self::CssUrlFunction(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssImportUrl> for SyntaxNode {
    fn from(n: AnyCssImportUrl) -> Self {
        match n {
            AnyCssImportUrl::CssString(it) => it.into(),
            AnyCssImportUrl::CssUrlFunction(it) => it.into(),
        }
    }
}
impl From<AnyCssImportUrl> for SyntaxElement {
    fn from(n: AnyCssImportUrl) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusBlock> for AnyCssKeyframesBlock {
    fn from(node: CssBogusBlock) -> Self {
        Self::CssBogusBlock(node)
    }
}
impl From<CssKeyframesBlock> for AnyCssKeyframesBlock {
    fn from(node: CssKeyframesBlock) -> Self {
        Self::CssKeyframesBlock(node)
    }
}
impl AstNode for AnyCssKeyframesBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssBogusBlock::KIND_SET.union(CssKeyframesBlock::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_BOGUS_BLOCK | CSS_KEYFRAMES_BLOCK)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_BLOCK => Self::CssBogusBlock(CssBogusBlock { syntax }),
            CSS_KEYFRAMES_BLOCK => Self::CssKeyframesBlock(CssKeyframesBlock { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusBlock(it) => &it.syntax,
            Self::CssKeyframesBlock(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusBlock(it) => it.syntax,
            Self::CssKeyframesBlock(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssKeyframesBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::CssKeyframesBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssKeyframesBlock> for SyntaxNode {
    fn from(n: AnyCssKeyframesBlock) -> Self {
        match n {
            AnyCssKeyframesBlock::CssBogusBlock(it) => it.into(),
            AnyCssKeyframesBlock::CssKeyframesBlock(it) => it.into(),
        }
    }
}
impl From<AnyCssKeyframesBlock> for SyntaxElement {
    fn from(n: AnyCssKeyframesBlock) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssCustomIdentifier> for AnyCssKeyframesIdentifier {
    fn from(node: CssCustomIdentifier) -> Self {
        Self::CssCustomIdentifier(node)
    }
}
impl From<CssString> for AnyCssKeyframesIdentifier {
    fn from(node: CssString) -> Self {
        Self::CssString(node)
    }
}
impl AstNode for AnyCssKeyframesIdentifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssCustomIdentifier::KIND_SET.union(CssString::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_CUSTOM_IDENTIFIER | CSS_STRING)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_CUSTOM_IDENTIFIER => Self::CssCustomIdentifier(CssCustomIdentifier { syntax }),
            CSS_STRING => Self::CssString(CssString { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssCustomIdentifier(it) => &it.syntax,
            Self::CssString(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssCustomIdentifier(it) => it.syntax,
            Self::CssString(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssKeyframesIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssCustomIdentifier(it) => std::fmt::Debug::fmt(it, f),
            Self::CssString(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssKeyframesIdentifier> for SyntaxNode {
    fn from(n: AnyCssKeyframesIdentifier) -> Self {
        match n {
            AnyCssKeyframesIdentifier::CssCustomIdentifier(it) => it.into(),
            AnyCssKeyframesIdentifier::CssString(it) => it.into(),
        }
    }
}
impl From<AnyCssKeyframesIdentifier> for SyntaxElement {
    fn from(n: AnyCssKeyframesIdentifier) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusKeyframesItem> for AnyCssKeyframesItem {
    fn from(node: CssBogusKeyframesItem) -> Self {
        Self::CssBogusKeyframesItem(node)
    }
}
impl From<CssKeyframesItem> for AnyCssKeyframesItem {
    fn from(node: CssKeyframesItem) -> Self {
        Self::CssKeyframesItem(node)
    }
}
impl AstNode for AnyCssKeyframesItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssBogusKeyframesItem::KIND_SET.union(CssKeyframesItem::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_BOGUS_KEYFRAMES_ITEM | CSS_KEYFRAMES_ITEM)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_KEYFRAMES_ITEM => {
                Self::CssBogusKeyframesItem(CssBogusKeyframesItem { syntax })
            }
            CSS_KEYFRAMES_ITEM => Self::CssKeyframesItem(CssKeyframesItem { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusKeyframesItem(it) => &it.syntax,
            Self::CssKeyframesItem(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusKeyframesItem(it) => it.syntax,
            Self::CssKeyframesItem(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssKeyframesItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusKeyframesItem(it) => std::fmt::Debug::fmt(it, f),
            Self::CssKeyframesItem(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssKeyframesItem> for SyntaxNode {
    fn from(n: AnyCssKeyframesItem) -> Self {
        match n {
            AnyCssKeyframesItem::CssBogusKeyframesItem(it) => it.into(),
            AnyCssKeyframesItem::CssKeyframesItem(it) => it.into(),
        }
    }
}
impl From<AnyCssKeyframesItem> for SyntaxElement {
    fn from(n: AnyCssKeyframesItem) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusKeyframesName> for AnyCssKeyframesName {
    fn from(node: CssBogusKeyframesName) -> Self {
        Self::CssBogusKeyframesName(node)
    }
}
impl From<CssKeyframesScopedName> for AnyCssKeyframesName {
    fn from(node: CssKeyframesScopedName) -> Self {
        Self::CssKeyframesScopedName(node)
    }
}
impl AstNode for AnyCssKeyframesName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyCssKeyframesIdentifier::KIND_SET
        .union(CssBogusKeyframesName::KIND_SET)
        .union(CssKeyframesScopedName::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_BOGUS_KEYFRAMES_NAME | CSS_KEYFRAMES_SCOPED_NAME => true,
            k if AnyCssKeyframesIdentifier::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_KEYFRAMES_NAME => {
                Self::CssBogusKeyframesName(CssBogusKeyframesName { syntax })
            }
            CSS_KEYFRAMES_SCOPED_NAME => {
                Self::CssKeyframesScopedName(CssKeyframesScopedName { syntax })
            }
            _ => {
                if let Some(any_css_keyframes_identifier) = AnyCssKeyframesIdentifier::cast(syntax)
                {
                    return Some(Self::AnyCssKeyframesIdentifier(
                        any_css_keyframes_identifier,
                    ));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusKeyframesName(it) => &it.syntax,
            Self::CssKeyframesScopedName(it) => &it.syntax,
            Self::AnyCssKeyframesIdentifier(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusKeyframesName(it) => it.syntax,
            Self::CssKeyframesScopedName(it) => it.syntax,
            Self::AnyCssKeyframesIdentifier(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssKeyframesName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyCssKeyframesIdentifier(it) => std::fmt::Debug::fmt(it, f),
            Self::CssBogusKeyframesName(it) => std::fmt::Debug::fmt(it, f),
            Self::CssKeyframesScopedName(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssKeyframesName> for SyntaxNode {
    fn from(n: AnyCssKeyframesName) -> Self {
        match n {
            AnyCssKeyframesName::AnyCssKeyframesIdentifier(it) => it.into(),
            AnyCssKeyframesName::CssBogusKeyframesName(it) => it.into(),
            AnyCssKeyframesName::CssKeyframesScopedName(it) => it.into(),
        }
    }
}
impl From<AnyCssKeyframesName> for SyntaxElement {
    fn from(n: AnyCssKeyframesName) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssKeyframesScopeFunction> for AnyCssKeyframesScope {
    fn from(node: CssKeyframesScopeFunction) -> Self {
        Self::CssKeyframesScopeFunction(node)
    }
}
impl From<CssKeyframesScopePrefix> for AnyCssKeyframesScope {
    fn from(node: CssKeyframesScopePrefix) -> Self {
        Self::CssKeyframesScopePrefix(node)
    }
}
impl AstNode for AnyCssKeyframesScope {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssKeyframesScopeFunction::KIND_SET.union(CssKeyframesScopePrefix::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_KEYFRAMES_SCOPE_FUNCTION | CSS_KEYFRAMES_SCOPE_PREFIX
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_KEYFRAMES_SCOPE_FUNCTION => {
                Self::CssKeyframesScopeFunction(CssKeyframesScopeFunction { syntax })
            }
            CSS_KEYFRAMES_SCOPE_PREFIX => {
                Self::CssKeyframesScopePrefix(CssKeyframesScopePrefix { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssKeyframesScopeFunction(it) => &it.syntax,
            Self::CssKeyframesScopePrefix(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssKeyframesScopeFunction(it) => it.syntax,
            Self::CssKeyframesScopePrefix(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssKeyframesScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssKeyframesScopeFunction(it) => std::fmt::Debug::fmt(it, f),
            Self::CssKeyframesScopePrefix(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssKeyframesScope> for SyntaxNode {
    fn from(n: AnyCssKeyframesScope) -> Self {
        match n {
            AnyCssKeyframesScope::CssKeyframesScopeFunction(it) => it.into(),
            AnyCssKeyframesScope::CssKeyframesScopePrefix(it) => it.into(),
        }
    }
}
impl From<AnyCssKeyframesScope> for SyntaxElement {
    fn from(n: AnyCssKeyframesScope) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusSelector> for AnyCssKeyframesSelector {
    fn from(node: CssBogusSelector) -> Self {
        Self::CssBogusSelector(node)
    }
}
impl From<CssKeyframesIdentSelector> for AnyCssKeyframesSelector {
    fn from(node: CssKeyframesIdentSelector) -> Self {
        Self::CssKeyframesIdentSelector(node)
    }
}
impl From<CssKeyframesPercentageSelector> for AnyCssKeyframesSelector {
    fn from(node: CssKeyframesPercentageSelector) -> Self {
        Self::CssKeyframesPercentageSelector(node)
    }
}
impl AstNode for AnyCssKeyframesSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssBogusSelector::KIND_SET
        .union(CssKeyframesIdentSelector::KIND_SET)
        .union(CssKeyframesPercentageSelector::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_BOGUS_SELECTOR | CSS_KEYFRAMES_IDENT_SELECTOR | CSS_KEYFRAMES_PERCENTAGE_SELECTOR
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_SELECTOR => Self::CssBogusSelector(CssBogusSelector { syntax }),
            CSS_KEYFRAMES_IDENT_SELECTOR => {
                Self::CssKeyframesIdentSelector(CssKeyframesIdentSelector { syntax })
            }
            CSS_KEYFRAMES_PERCENTAGE_SELECTOR => {
                Self::CssKeyframesPercentageSelector(CssKeyframesPercentageSelector { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusSelector(it) => &it.syntax,
            Self::CssKeyframesIdentSelector(it) => &it.syntax,
            Self::CssKeyframesPercentageSelector(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusSelector(it) => it.syntax,
            Self::CssKeyframesIdentSelector(it) => it.syntax,
            Self::CssKeyframesPercentageSelector(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssKeyframesSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusSelector(it) => std::fmt::Debug::fmt(it, f),
            Self::CssKeyframesIdentSelector(it) => std::fmt::Debug::fmt(it, f),
            Self::CssKeyframesPercentageSelector(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssKeyframesSelector> for SyntaxNode {
    fn from(n: AnyCssKeyframesSelector) -> Self {
        match n {
            AnyCssKeyframesSelector::CssBogusSelector(it) => it.into(),
            AnyCssKeyframesSelector::CssKeyframesIdentSelector(it) => it.into(),
            AnyCssKeyframesSelector::CssKeyframesPercentageSelector(it) => it.into(),
        }
    }
}
impl From<AnyCssKeyframesSelector> for SyntaxElement {
    fn from(n: AnyCssKeyframesSelector) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusLayer> for AnyCssLayer {
    fn from(node: CssBogusLayer) -> Self {
        Self::CssBogusLayer(node)
    }
}
impl From<CssLayerDeclaration> for AnyCssLayer {
    fn from(node: CssLayerDeclaration) -> Self {
        Self::CssLayerDeclaration(node)
    }
}
impl From<CssLayerReference> for AnyCssLayer {
    fn from(node: CssLayerReference) -> Self {
        Self::CssLayerReference(node)
    }
}
impl AstNode for AnyCssLayer {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssBogusLayer::KIND_SET
        .union(CssLayerDeclaration::KIND_SET)
        .union(CssLayerReference::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_BOGUS_LAYER | CSS_LAYER_DECLARATION | CSS_LAYER_REFERENCE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_LAYER => Self::CssBogusLayer(CssBogusLayer { syntax }),
            CSS_LAYER_DECLARATION => Self::CssLayerDeclaration(CssLayerDeclaration { syntax }),
            CSS_LAYER_REFERENCE => Self::CssLayerReference(CssLayerReference { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusLayer(it) => &it.syntax,
            Self::CssLayerDeclaration(it) => &it.syntax,
            Self::CssLayerReference(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusLayer(it) => it.syntax,
            Self::CssLayerDeclaration(it) => it.syntax,
            Self::CssLayerReference(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusLayer(it) => std::fmt::Debug::fmt(it, f),
            Self::CssLayerDeclaration(it) => std::fmt::Debug::fmt(it, f),
            Self::CssLayerReference(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssLayer> for SyntaxNode {
    fn from(n: AnyCssLayer) -> Self {
        match n {
            AnyCssLayer::CssBogusLayer(it) => it.into(),
            AnyCssLayer::CssLayerDeclaration(it) => it.into(),
            AnyCssLayer::CssLayerReference(it) => it.into(),
        }
    }
}
impl From<AnyCssLayer> for SyntaxElement {
    fn from(n: AnyCssLayer) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssMediaAndCondition> for AnyCssMediaAndCombinableCondition {
    fn from(node: CssMediaAndCondition) -> Self {
        Self::CssMediaAndCondition(node)
    }
}
impl AstNode for AnyCssMediaAndCombinableCondition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyCssMediaInParens::KIND_SET.union(CssMediaAndCondition::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_MEDIA_AND_CONDITION => true,
            k if AnyCssMediaInParens::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_MEDIA_AND_CONDITION => Self::CssMediaAndCondition(CssMediaAndCondition { syntax }),
            _ => {
                if let Some(any_css_media_in_parens) = AnyCssMediaInParens::cast(syntax) {
                    return Some(Self::AnyCssMediaInParens(any_css_media_in_parens));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssMediaAndCondition(it) => &it.syntax,
            Self::AnyCssMediaInParens(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssMediaAndCondition(it) => it.syntax,
            Self::AnyCssMediaInParens(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssMediaAndCombinableCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyCssMediaInParens(it) => std::fmt::Debug::fmt(it, f),
            Self::CssMediaAndCondition(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssMediaAndCombinableCondition> for SyntaxNode {
    fn from(n: AnyCssMediaAndCombinableCondition) -> Self {
        match n {
            AnyCssMediaAndCombinableCondition::AnyCssMediaInParens(it) => it.into(),
            AnyCssMediaAndCombinableCondition::CssMediaAndCondition(it) => it.into(),
        }
    }
}
impl From<AnyCssMediaAndCombinableCondition> for SyntaxElement {
    fn from(n: AnyCssMediaAndCombinableCondition) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssMediaAndCondition> for AnyCssMediaCondition {
    fn from(node: CssMediaAndCondition) -> Self {
        Self::CssMediaAndCondition(node)
    }
}
impl From<CssMediaNotCondition> for AnyCssMediaCondition {
    fn from(node: CssMediaNotCondition) -> Self {
        Self::CssMediaNotCondition(node)
    }
}
impl From<CssMediaOrCondition> for AnyCssMediaCondition {
    fn from(node: CssMediaOrCondition) -> Self {
        Self::CssMediaOrCondition(node)
    }
}
impl AstNode for AnyCssMediaCondition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyCssMediaInParens::KIND_SET
        .union(CssMediaAndCondition::KIND_SET)
        .union(CssMediaNotCondition::KIND_SET)
        .union(CssMediaOrCondition::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_MEDIA_AND_CONDITION | CSS_MEDIA_NOT_CONDITION | CSS_MEDIA_OR_CONDITION => true,
            k if AnyCssMediaInParens::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_MEDIA_AND_CONDITION => Self::CssMediaAndCondition(CssMediaAndCondition { syntax }),
            CSS_MEDIA_NOT_CONDITION => Self::CssMediaNotCondition(CssMediaNotCondition { syntax }),
            CSS_MEDIA_OR_CONDITION => Self::CssMediaOrCondition(CssMediaOrCondition { syntax }),
            _ => {
                if let Some(any_css_media_in_parens) = AnyCssMediaInParens::cast(syntax) {
                    return Some(Self::AnyCssMediaInParens(any_css_media_in_parens));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssMediaAndCondition(it) => &it.syntax,
            Self::CssMediaNotCondition(it) => &it.syntax,
            Self::CssMediaOrCondition(it) => &it.syntax,
            Self::AnyCssMediaInParens(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssMediaAndCondition(it) => it.syntax,
            Self::CssMediaNotCondition(it) => it.syntax,
            Self::CssMediaOrCondition(it) => it.syntax,
            Self::AnyCssMediaInParens(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssMediaCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyCssMediaInParens(it) => std::fmt::Debug::fmt(it, f),
            Self::CssMediaAndCondition(it) => std::fmt::Debug::fmt(it, f),
            Self::CssMediaNotCondition(it) => std::fmt::Debug::fmt(it, f),
            Self::CssMediaOrCondition(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssMediaCondition> for SyntaxNode {
    fn from(n: AnyCssMediaCondition) -> Self {
        match n {
            AnyCssMediaCondition::AnyCssMediaInParens(it) => it.into(),
            AnyCssMediaCondition::CssMediaAndCondition(it) => it.into(),
            AnyCssMediaCondition::CssMediaNotCondition(it) => it.into(),
            AnyCssMediaCondition::CssMediaOrCondition(it) => it.into(),
        }
    }
}
impl From<AnyCssMediaCondition> for SyntaxElement {
    fn from(n: AnyCssMediaCondition) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssMediaConditionInParens> for AnyCssMediaInParens {
    fn from(node: CssMediaConditionInParens) -> Self {
        Self::CssMediaConditionInParens(node)
    }
}
impl From<CssMediaFeatureInParens> for AnyCssMediaInParens {
    fn from(node: CssMediaFeatureInParens) -> Self {
        Self::CssMediaFeatureInParens(node)
    }
}
impl AstNode for AnyCssMediaInParens {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssMediaConditionInParens::KIND_SET.union(CssMediaFeatureInParens::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_MEDIA_CONDITION_IN_PARENS | CSS_MEDIA_FEATURE_IN_PARENS
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_MEDIA_CONDITION_IN_PARENS => {
                Self::CssMediaConditionInParens(CssMediaConditionInParens { syntax })
            }
            CSS_MEDIA_FEATURE_IN_PARENS => {
                Self::CssMediaFeatureInParens(CssMediaFeatureInParens { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssMediaConditionInParens(it) => &it.syntax,
            Self::CssMediaFeatureInParens(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssMediaConditionInParens(it) => it.syntax,
            Self::CssMediaFeatureInParens(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssMediaInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssMediaConditionInParens(it) => std::fmt::Debug::fmt(it, f),
            Self::CssMediaFeatureInParens(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssMediaInParens> for SyntaxNode {
    fn from(n: AnyCssMediaInParens) -> Self {
        match n {
            AnyCssMediaInParens::CssMediaConditionInParens(it) => it.into(),
            AnyCssMediaInParens::CssMediaFeatureInParens(it) => it.into(),
        }
    }
}
impl From<AnyCssMediaInParens> for SyntaxElement {
    fn from(n: AnyCssMediaInParens) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssMediaOrCondition> for AnyCssMediaOrCombinableCondition {
    fn from(node: CssMediaOrCondition) -> Self {
        Self::CssMediaOrCondition(node)
    }
}
impl AstNode for AnyCssMediaOrCombinableCondition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyCssMediaInParens::KIND_SET.union(CssMediaOrCondition::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_MEDIA_OR_CONDITION => true,
            k if AnyCssMediaInParens::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_MEDIA_OR_CONDITION => Self::CssMediaOrCondition(CssMediaOrCondition { syntax }),
            _ => {
                if let Some(any_css_media_in_parens) = AnyCssMediaInParens::cast(syntax) {
                    return Some(Self::AnyCssMediaInParens(any_css_media_in_parens));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssMediaOrCondition(it) => &it.syntax,
            Self::AnyCssMediaInParens(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssMediaOrCondition(it) => it.syntax,
            Self::AnyCssMediaInParens(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssMediaOrCombinableCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyCssMediaInParens(it) => std::fmt::Debug::fmt(it, f),
            Self::CssMediaOrCondition(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssMediaOrCombinableCondition> for SyntaxNode {
    fn from(n: AnyCssMediaOrCombinableCondition) -> Self {
        match n {
            AnyCssMediaOrCombinableCondition::AnyCssMediaInParens(it) => it.into(),
            AnyCssMediaOrCombinableCondition::CssMediaOrCondition(it) => it.into(),
        }
    }
}
impl From<AnyCssMediaOrCombinableCondition> for SyntaxElement {
    fn from(n: AnyCssMediaOrCombinableCondition) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusMediaQuery> for AnyCssMediaQuery {
    fn from(node: CssBogusMediaQuery) -> Self {
        Self::CssBogusMediaQuery(node)
    }
}
impl From<CssMediaConditionQuery> for AnyCssMediaQuery {
    fn from(node: CssMediaConditionQuery) -> Self {
        Self::CssMediaConditionQuery(node)
    }
}
impl From<CssMetavariable> for AnyCssMediaQuery {
    fn from(node: CssMetavariable) -> Self {
        Self::CssMetavariable(node)
    }
}
impl AstNode for AnyCssMediaQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyCssMediaTypeQuery::KIND_SET
        .union(CssBogusMediaQuery::KIND_SET)
        .union(CssMediaConditionQuery::KIND_SET)
        .union(CssMetavariable::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_BOGUS_MEDIA_QUERY | CSS_MEDIA_CONDITION_QUERY | CSS_METAVARIABLE => true,
            k if AnyCssMediaTypeQuery::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_MEDIA_QUERY => Self::CssBogusMediaQuery(CssBogusMediaQuery { syntax }),
            CSS_MEDIA_CONDITION_QUERY => {
                Self::CssMediaConditionQuery(CssMediaConditionQuery { syntax })
            }
            CSS_METAVARIABLE => Self::CssMetavariable(CssMetavariable { syntax }),
            _ => {
                if let Some(any_css_media_type_query) = AnyCssMediaTypeQuery::cast(syntax) {
                    return Some(Self::AnyCssMediaTypeQuery(any_css_media_type_query));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusMediaQuery(it) => &it.syntax,
            Self::CssMediaConditionQuery(it) => &it.syntax,
            Self::CssMetavariable(it) => &it.syntax,
            Self::AnyCssMediaTypeQuery(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusMediaQuery(it) => it.syntax,
            Self::CssMediaConditionQuery(it) => it.syntax,
            Self::CssMetavariable(it) => it.syntax,
            Self::AnyCssMediaTypeQuery(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssMediaQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyCssMediaTypeQuery(it) => std::fmt::Debug::fmt(it, f),
            Self::CssBogusMediaQuery(it) => std::fmt::Debug::fmt(it, f),
            Self::CssMediaConditionQuery(it) => std::fmt::Debug::fmt(it, f),
            Self::CssMetavariable(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssMediaQuery> for SyntaxNode {
    fn from(n: AnyCssMediaQuery) -> Self {
        match n {
            AnyCssMediaQuery::AnyCssMediaTypeQuery(it) => it.into(),
            AnyCssMediaQuery::CssBogusMediaQuery(it) => it.into(),
            AnyCssMediaQuery::CssMediaConditionQuery(it) => it.into(),
            AnyCssMediaQuery::CssMetavariable(it) => it.into(),
        }
    }
}
impl From<AnyCssMediaQuery> for SyntaxElement {
    fn from(n: AnyCssMediaQuery) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssMediaAndCondition> for AnyCssMediaTypeCondition {
    fn from(node: CssMediaAndCondition) -> Self {
        Self::CssMediaAndCondition(node)
    }
}
impl From<CssMediaNotCondition> for AnyCssMediaTypeCondition {
    fn from(node: CssMediaNotCondition) -> Self {
        Self::CssMediaNotCondition(node)
    }
}
impl AstNode for AnyCssMediaTypeCondition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyCssMediaInParens::KIND_SET
        .union(CssMediaAndCondition::KIND_SET)
        .union(CssMediaNotCondition::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_MEDIA_AND_CONDITION | CSS_MEDIA_NOT_CONDITION => true,
            k if AnyCssMediaInParens::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_MEDIA_AND_CONDITION => Self::CssMediaAndCondition(CssMediaAndCondition { syntax }),
            CSS_MEDIA_NOT_CONDITION => Self::CssMediaNotCondition(CssMediaNotCondition { syntax }),
            _ => {
                if let Some(any_css_media_in_parens) = AnyCssMediaInParens::cast(syntax) {
                    return Some(Self::AnyCssMediaInParens(any_css_media_in_parens));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssMediaAndCondition(it) => &it.syntax,
            Self::CssMediaNotCondition(it) => &it.syntax,
            Self::AnyCssMediaInParens(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssMediaAndCondition(it) => it.syntax,
            Self::CssMediaNotCondition(it) => it.syntax,
            Self::AnyCssMediaInParens(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssMediaTypeCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyCssMediaInParens(it) => std::fmt::Debug::fmt(it, f),
            Self::CssMediaAndCondition(it) => std::fmt::Debug::fmt(it, f),
            Self::CssMediaNotCondition(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssMediaTypeCondition> for SyntaxNode {
    fn from(n: AnyCssMediaTypeCondition) -> Self {
        match n {
            AnyCssMediaTypeCondition::AnyCssMediaInParens(it) => it.into(),
            AnyCssMediaTypeCondition::CssMediaAndCondition(it) => it.into(),
            AnyCssMediaTypeCondition::CssMediaNotCondition(it) => it.into(),
        }
    }
}
impl From<AnyCssMediaTypeCondition> for SyntaxElement {
    fn from(n: AnyCssMediaTypeCondition) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssMediaAndTypeQuery> for AnyCssMediaTypeQuery {
    fn from(node: CssMediaAndTypeQuery) -> Self {
        Self::CssMediaAndTypeQuery(node)
    }
}
impl From<CssMediaTypeQuery> for AnyCssMediaTypeQuery {
    fn from(node: CssMediaTypeQuery) -> Self {
        Self::CssMediaTypeQuery(node)
    }
}
impl AstNode for AnyCssMediaTypeQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssMediaAndTypeQuery::KIND_SET.union(CssMediaTypeQuery::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_MEDIA_AND_TYPE_QUERY | CSS_MEDIA_TYPE_QUERY)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_MEDIA_AND_TYPE_QUERY => Self::CssMediaAndTypeQuery(CssMediaAndTypeQuery { syntax }),
            CSS_MEDIA_TYPE_QUERY => Self::CssMediaTypeQuery(CssMediaTypeQuery { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssMediaAndTypeQuery(it) => &it.syntax,
            Self::CssMediaTypeQuery(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssMediaAndTypeQuery(it) => it.syntax,
            Self::CssMediaTypeQuery(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssMediaTypeQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssMediaAndTypeQuery(it) => std::fmt::Debug::fmt(it, f),
            Self::CssMediaTypeQuery(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssMediaTypeQuery> for SyntaxNode {
    fn from(n: AnyCssMediaTypeQuery) -> Self {
        match n {
            AnyCssMediaTypeQuery::CssMediaAndTypeQuery(it) => it.into(),
            AnyCssMediaTypeQuery::CssMediaTypeQuery(it) => it.into(),
        }
    }
}
impl From<AnyCssMediaTypeQuery> for SyntaxElement {
    fn from(n: AnyCssMediaTypeQuery) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssNamedNamespacePrefix> for AnyCssNamespacePrefix {
    fn from(node: CssNamedNamespacePrefix) -> Self {
        Self::CssNamedNamespacePrefix(node)
    }
}
impl From<CssUniversalNamespacePrefix> for AnyCssNamespacePrefix {
    fn from(node: CssUniversalNamespacePrefix) -> Self {
        Self::CssUniversalNamespacePrefix(node)
    }
}
impl AstNode for AnyCssNamespacePrefix {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssNamedNamespacePrefix::KIND_SET.union(CssUniversalNamespacePrefix::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_NAMED_NAMESPACE_PREFIX | CSS_UNIVERSAL_NAMESPACE_PREFIX
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_NAMED_NAMESPACE_PREFIX => {
                Self::CssNamedNamespacePrefix(CssNamedNamespacePrefix { syntax })
            }
            CSS_UNIVERSAL_NAMESPACE_PREFIX => {
                Self::CssUniversalNamespacePrefix(CssUniversalNamespacePrefix { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssNamedNamespacePrefix(it) => &it.syntax,
            Self::CssUniversalNamespacePrefix(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssNamedNamespacePrefix(it) => it.syntax,
            Self::CssUniversalNamespacePrefix(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssNamespacePrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssNamedNamespacePrefix(it) => std::fmt::Debug::fmt(it, f),
            Self::CssUniversalNamespacePrefix(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssNamespacePrefix> for SyntaxNode {
    fn from(n: AnyCssNamespacePrefix) -> Self {
        match n {
            AnyCssNamespacePrefix::CssNamedNamespacePrefix(it) => it.into(),
            AnyCssNamespacePrefix::CssUniversalNamespacePrefix(it) => it.into(),
        }
    }
}
impl From<AnyCssNamespacePrefix> for SyntaxElement {
    fn from(n: AnyCssNamespacePrefix) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssString> for AnyCssNamespaceUrl {
    fn from(node: CssString) -> Self {
        Self::CssString(node)
    }
}
impl From<CssUrlFunction> for AnyCssNamespaceUrl {
    fn from(node: CssUrlFunction) -> Self {
        Self::CssUrlFunction(node)
    }
}
impl AstNode for AnyCssNamespaceUrl {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssString::KIND_SET.union(CssUrlFunction::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_STRING | CSS_URL_FUNCTION)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_STRING => Self::CssString(CssString { syntax }),
            CSS_URL_FUNCTION => Self::CssUrlFunction(CssUrlFunction { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssString(it) => &it.syntax,
            Self::CssUrlFunction(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssString(it) => it.syntax,
            Self::CssUrlFunction(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssNamespaceUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssString(it) => std::fmt::Debug::fmt(it, f),
            Self::CssUrlFunction(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssNamespaceUrl> for SyntaxNode {
    fn from(n: AnyCssNamespaceUrl) -> Self {
        match n {
            AnyCssNamespaceUrl::CssString(it) => it.into(),
            AnyCssNamespaceUrl::CssUrlFunction(it) => it.into(),
        }
    }
}
impl From<AnyCssNamespaceUrl> for SyntaxElement {
    fn from(n: AnyCssNamespaceUrl) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusBlock> for AnyCssPageAtRuleBlock {
    fn from(node: CssBogusBlock) -> Self {
        Self::CssBogusBlock(node)
    }
}
impl From<CssPageAtRuleBlock> for AnyCssPageAtRuleBlock {
    fn from(node: CssPageAtRuleBlock) -> Self {
        Self::CssPageAtRuleBlock(node)
    }
}
impl AstNode for AnyCssPageAtRuleBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssBogusBlock::KIND_SET.union(CssPageAtRuleBlock::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_BOGUS_BLOCK | CSS_PAGE_AT_RULE_BLOCK)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_BLOCK => Self::CssBogusBlock(CssBogusBlock { syntax }),
            CSS_PAGE_AT_RULE_BLOCK => Self::CssPageAtRuleBlock(CssPageAtRuleBlock { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusBlock(it) => &it.syntax,
            Self::CssPageAtRuleBlock(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusBlock(it) => it.syntax,
            Self::CssPageAtRuleBlock(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssPageAtRuleBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::CssPageAtRuleBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssPageAtRuleBlock> for SyntaxNode {
    fn from(n: AnyCssPageAtRuleBlock) -> Self {
        match n {
            AnyCssPageAtRuleBlock::CssBogusBlock(it) => it.into(),
            AnyCssPageAtRuleBlock::CssPageAtRuleBlock(it) => it.into(),
        }
    }
}
impl From<AnyCssPageAtRuleBlock> for SyntaxElement {
    fn from(n: AnyCssPageAtRuleBlock) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssAtRule> for AnyCssPageAtRuleItem {
    fn from(node: CssAtRule) -> Self {
        Self::CssAtRule(node)
    }
}
impl From<CssDeclarationWithSemicolon> for AnyCssPageAtRuleItem {
    fn from(node: CssDeclarationWithSemicolon) -> Self {
        Self::CssDeclarationWithSemicolon(node)
    }
}
impl From<CssEmptyDeclaration> for AnyCssPageAtRuleItem {
    fn from(node: CssEmptyDeclaration) -> Self {
        Self::CssEmptyDeclaration(node)
    }
}
impl From<CssMarginAtRule> for AnyCssPageAtRuleItem {
    fn from(node: CssMarginAtRule) -> Self {
        Self::CssMarginAtRule(node)
    }
}
impl AstNode for AnyCssPageAtRuleItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssAtRule::KIND_SET
        .union(CssDeclarationWithSemicolon::KIND_SET)
        .union(CssEmptyDeclaration::KIND_SET)
        .union(CssMarginAtRule::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_AT_RULE
                | CSS_DECLARATION_WITH_SEMICOLON
                | CSS_EMPTY_DECLARATION
                | CSS_MARGIN_AT_RULE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_AT_RULE => Self::CssAtRule(CssAtRule { syntax }),
            CSS_DECLARATION_WITH_SEMICOLON => {
                Self::CssDeclarationWithSemicolon(CssDeclarationWithSemicolon { syntax })
            }
            CSS_EMPTY_DECLARATION => Self::CssEmptyDeclaration(CssEmptyDeclaration { syntax }),
            CSS_MARGIN_AT_RULE => Self::CssMarginAtRule(CssMarginAtRule { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssAtRule(it) => &it.syntax,
            Self::CssDeclarationWithSemicolon(it) => &it.syntax,
            Self::CssEmptyDeclaration(it) => &it.syntax,
            Self::CssMarginAtRule(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssAtRule(it) => it.syntax,
            Self::CssDeclarationWithSemicolon(it) => it.syntax,
            Self::CssEmptyDeclaration(it) => it.syntax,
            Self::CssMarginAtRule(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssPageAtRuleItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssDeclarationWithSemicolon(it) => std::fmt::Debug::fmt(it, f),
            Self::CssEmptyDeclaration(it) => std::fmt::Debug::fmt(it, f),
            Self::CssMarginAtRule(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssPageAtRuleItem> for SyntaxNode {
    fn from(n: AnyCssPageAtRuleItem) -> Self {
        match n {
            AnyCssPageAtRuleItem::CssAtRule(it) => it.into(),
            AnyCssPageAtRuleItem::CssDeclarationWithSemicolon(it) => it.into(),
            AnyCssPageAtRuleItem::CssEmptyDeclaration(it) => it.into(),
            AnyCssPageAtRuleItem::CssMarginAtRule(it) => it.into(),
        }
    }
}
impl From<AnyCssPageAtRuleItem> for SyntaxElement {
    fn from(n: AnyCssPageAtRuleItem) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusSelector> for AnyCssPageSelector {
    fn from(node: CssBogusSelector) -> Self {
        Self::CssBogusSelector(node)
    }
}
impl From<CssPageSelector> for AnyCssPageSelector {
    fn from(node: CssPageSelector) -> Self {
        Self::CssPageSelector(node)
    }
}
impl AstNode for AnyCssPageSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssBogusSelector::KIND_SET.union(CssPageSelector::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_BOGUS_SELECTOR | CSS_PAGE_SELECTOR)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_SELECTOR => Self::CssBogusSelector(CssBogusSelector { syntax }),
            CSS_PAGE_SELECTOR => Self::CssPageSelector(CssPageSelector { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusSelector(it) => &it.syntax,
            Self::CssPageSelector(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusSelector(it) => it.syntax,
            Self::CssPageSelector(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssPageSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusSelector(it) => std::fmt::Debug::fmt(it, f),
            Self::CssPageSelector(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssPageSelector> for SyntaxNode {
    fn from(n: AnyCssPageSelector) -> Self {
        match n {
            AnyCssPageSelector::CssBogusSelector(it) => it.into(),
            AnyCssPageSelector::CssPageSelector(it) => it.into(),
        }
    }
}
impl From<AnyCssPageSelector> for SyntaxElement {
    fn from(n: AnyCssPageSelector) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusPageSelectorPseudo> for AnyCssPageSelectorPseudo {
    fn from(node: CssBogusPageSelectorPseudo) -> Self {
        Self::CssBogusPageSelectorPseudo(node)
    }
}
impl From<CssPageSelectorPseudo> for AnyCssPageSelectorPseudo {
    fn from(node: CssPageSelectorPseudo) -> Self {
        Self::CssPageSelectorPseudo(node)
    }
}
impl AstNode for AnyCssPageSelectorPseudo {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssBogusPageSelectorPseudo::KIND_SET.union(CssPageSelectorPseudo::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_BOGUS_PAGE_SELECTOR_PSEUDO | CSS_PAGE_SELECTOR_PSEUDO
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_PAGE_SELECTOR_PSEUDO => {
                Self::CssBogusPageSelectorPseudo(CssBogusPageSelectorPseudo { syntax })
            }
            CSS_PAGE_SELECTOR_PSEUDO => {
                Self::CssPageSelectorPseudo(CssPageSelectorPseudo { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusPageSelectorPseudo(it) => &it.syntax,
            Self::CssPageSelectorPseudo(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusPageSelectorPseudo(it) => it.syntax,
            Self::CssPageSelectorPseudo(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssPageSelectorPseudo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusPageSelectorPseudo(it) => std::fmt::Debug::fmt(it, f),
            Self::CssPageSelectorPseudo(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssPageSelectorPseudo> for SyntaxNode {
    fn from(n: AnyCssPageSelectorPseudo) -> Self {
        match n {
            AnyCssPageSelectorPseudo::CssBogusPageSelectorPseudo(it) => it.into(),
            AnyCssPageSelectorPseudo::CssPageSelectorPseudo(it) => it.into(),
        }
    }
}
impl From<AnyCssPageSelectorPseudo> for SyntaxElement {
    fn from(n: AnyCssPageSelectorPseudo) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusProperty> for AnyCssProperty {
    fn from(node: CssBogusProperty) -> Self {
        Self::CssBogusProperty(node)
    }
}
impl From<CssComposesProperty> for AnyCssProperty {
    fn from(node: CssComposesProperty) -> Self {
        Self::CssComposesProperty(node)
    }
}
impl From<CssGenericProperty> for AnyCssProperty {
    fn from(node: CssGenericProperty) -> Self {
        Self::CssGenericProperty(node)
    }
}
impl AstNode for AnyCssProperty {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssBogusProperty::KIND_SET
        .union(CssComposesProperty::KIND_SET)
        .union(CssGenericProperty::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_BOGUS_PROPERTY | CSS_COMPOSES_PROPERTY | CSS_GENERIC_PROPERTY
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_PROPERTY => Self::CssBogusProperty(CssBogusProperty { syntax }),
            CSS_COMPOSES_PROPERTY => Self::CssComposesProperty(CssComposesProperty { syntax }),
            CSS_GENERIC_PROPERTY => Self::CssGenericProperty(CssGenericProperty { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusProperty(it) => &it.syntax,
            Self::CssComposesProperty(it) => &it.syntax,
            Self::CssGenericProperty(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusProperty(it) => it.syntax,
            Self::CssComposesProperty(it) => it.syntax,
            Self::CssGenericProperty(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusProperty(it) => std::fmt::Debug::fmt(it, f),
            Self::CssComposesProperty(it) => std::fmt::Debug::fmt(it, f),
            Self::CssGenericProperty(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssProperty> for SyntaxNode {
    fn from(n: AnyCssProperty) -> Self {
        match n {
            AnyCssProperty::CssBogusProperty(it) => it.into(),
            AnyCssProperty::CssComposesProperty(it) => it.into(),
            AnyCssProperty::CssGenericProperty(it) => it.into(),
        }
    }
}
impl From<AnyCssProperty> for SyntaxElement {
    fn from(n: AnyCssProperty) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusPseudoClass> for AnyCssPseudoClass {
    fn from(node: CssBogusPseudoClass) -> Self {
        Self::CssBogusPseudoClass(node)
    }
}
impl From<CssPseudoClassFunctionCompoundSelector> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassFunctionCompoundSelector) -> Self {
        Self::CssPseudoClassFunctionCompoundSelector(node)
    }
}
impl From<CssPseudoClassFunctionCompoundSelectorList> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassFunctionCompoundSelectorList) -> Self {
        Self::CssPseudoClassFunctionCompoundSelectorList(node)
    }
}
impl From<CssPseudoClassFunctionCustomIdentifierList> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassFunctionCustomIdentifierList) -> Self {
        Self::CssPseudoClassFunctionCustomIdentifierList(node)
    }
}
impl From<CssPseudoClassFunctionIdentifier> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassFunctionIdentifier) -> Self {
        Self::CssPseudoClassFunctionIdentifier(node)
    }
}
impl From<CssPseudoClassFunctionNth> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassFunctionNth) -> Self {
        Self::CssPseudoClassFunctionNth(node)
    }
}
impl From<CssPseudoClassFunctionRelativeSelectorList> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassFunctionRelativeSelectorList) -> Self {
        Self::CssPseudoClassFunctionRelativeSelectorList(node)
    }
}
impl From<CssPseudoClassFunctionSelector> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassFunctionSelector) -> Self {
        Self::CssPseudoClassFunctionSelector(node)
    }
}
impl From<CssPseudoClassFunctionSelectorList> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassFunctionSelectorList) -> Self {
        Self::CssPseudoClassFunctionSelectorList(node)
    }
}
impl From<CssPseudoClassFunctionValueList> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassFunctionValueList) -> Self {
        Self::CssPseudoClassFunctionValueList(node)
    }
}
impl From<CssPseudoClassIdentifier> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassIdentifier) -> Self {
        Self::CssPseudoClassIdentifier(node)
    }
}
impl AstNode for AnyCssPseudoClass {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssBogusPseudoClass::KIND_SET
        .union(CssPseudoClassFunctionCompoundSelector::KIND_SET)
        .union(CssPseudoClassFunctionCompoundSelectorList::KIND_SET)
        .union(CssPseudoClassFunctionCustomIdentifierList::KIND_SET)
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
            CSS_BOGUS_PSEUDO_CLASS
                | CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR
                | CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST
                | CSS_PSEUDO_CLASS_FUNCTION_CUSTOM_IDENTIFIER_LIST
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
            CSS_BOGUS_PSEUDO_CLASS => Self::CssBogusPseudoClass(CssBogusPseudoClass { syntax }),
            CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR => {
                Self::CssPseudoClassFunctionCompoundSelector(
                    CssPseudoClassFunctionCompoundSelector { syntax },
                )
            }
            CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST => {
                Self::CssPseudoClassFunctionCompoundSelectorList(
                    CssPseudoClassFunctionCompoundSelectorList { syntax },
                )
            }
            CSS_PSEUDO_CLASS_FUNCTION_CUSTOM_IDENTIFIER_LIST => {
                Self::CssPseudoClassFunctionCustomIdentifierList(
                    CssPseudoClassFunctionCustomIdentifierList { syntax },
                )
            }
            CSS_PSEUDO_CLASS_FUNCTION_IDENTIFIER => {
                Self::CssPseudoClassFunctionIdentifier(CssPseudoClassFunctionIdentifier { syntax })
            }
            CSS_PSEUDO_CLASS_FUNCTION_NTH => {
                Self::CssPseudoClassFunctionNth(CssPseudoClassFunctionNth { syntax })
            }
            CSS_PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST => {
                Self::CssPseudoClassFunctionRelativeSelectorList(
                    CssPseudoClassFunctionRelativeSelectorList { syntax },
                )
            }
            CSS_PSEUDO_CLASS_FUNCTION_SELECTOR => {
                Self::CssPseudoClassFunctionSelector(CssPseudoClassFunctionSelector { syntax })
            }
            CSS_PSEUDO_CLASS_FUNCTION_SELECTOR_LIST => {
                Self::CssPseudoClassFunctionSelectorList(CssPseudoClassFunctionSelectorList {
                    syntax,
                })
            }
            CSS_PSEUDO_CLASS_FUNCTION_VALUE_LIST => {
                Self::CssPseudoClassFunctionValueList(CssPseudoClassFunctionValueList { syntax })
            }
            CSS_PSEUDO_CLASS_IDENTIFIER => {
                Self::CssPseudoClassIdentifier(CssPseudoClassIdentifier { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusPseudoClass(it) => &it.syntax,
            Self::CssPseudoClassFunctionCompoundSelector(it) => &it.syntax,
            Self::CssPseudoClassFunctionCompoundSelectorList(it) => &it.syntax,
            Self::CssPseudoClassFunctionCustomIdentifierList(it) => &it.syntax,
            Self::CssPseudoClassFunctionIdentifier(it) => &it.syntax,
            Self::CssPseudoClassFunctionNth(it) => &it.syntax,
            Self::CssPseudoClassFunctionRelativeSelectorList(it) => &it.syntax,
            Self::CssPseudoClassFunctionSelector(it) => &it.syntax,
            Self::CssPseudoClassFunctionSelectorList(it) => &it.syntax,
            Self::CssPseudoClassFunctionValueList(it) => &it.syntax,
            Self::CssPseudoClassIdentifier(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusPseudoClass(it) => it.syntax,
            Self::CssPseudoClassFunctionCompoundSelector(it) => it.syntax,
            Self::CssPseudoClassFunctionCompoundSelectorList(it) => it.syntax,
            Self::CssPseudoClassFunctionCustomIdentifierList(it) => it.syntax,
            Self::CssPseudoClassFunctionIdentifier(it) => it.syntax,
            Self::CssPseudoClassFunctionNth(it) => it.syntax,
            Self::CssPseudoClassFunctionRelativeSelectorList(it) => it.syntax,
            Self::CssPseudoClassFunctionSelector(it) => it.syntax,
            Self::CssPseudoClassFunctionSelectorList(it) => it.syntax,
            Self::CssPseudoClassFunctionValueList(it) => it.syntax,
            Self::CssPseudoClassIdentifier(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssPseudoClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusPseudoClass(it) => std::fmt::Debug::fmt(it, f),
            Self::CssPseudoClassFunctionCompoundSelector(it) => std::fmt::Debug::fmt(it, f),
            Self::CssPseudoClassFunctionCompoundSelectorList(it) => std::fmt::Debug::fmt(it, f),
            Self::CssPseudoClassFunctionCustomIdentifierList(it) => std::fmt::Debug::fmt(it, f),
            Self::CssPseudoClassFunctionIdentifier(it) => std::fmt::Debug::fmt(it, f),
            Self::CssPseudoClassFunctionNth(it) => std::fmt::Debug::fmt(it, f),
            Self::CssPseudoClassFunctionRelativeSelectorList(it) => std::fmt::Debug::fmt(it, f),
            Self::CssPseudoClassFunctionSelector(it) => std::fmt::Debug::fmt(it, f),
            Self::CssPseudoClassFunctionSelectorList(it) => std::fmt::Debug::fmt(it, f),
            Self::CssPseudoClassFunctionValueList(it) => std::fmt::Debug::fmt(it, f),
            Self::CssPseudoClassIdentifier(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssPseudoClass> for SyntaxNode {
    fn from(n: AnyCssPseudoClass) -> Self {
        match n {
            AnyCssPseudoClass::CssBogusPseudoClass(it) => it.into(),
            AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelector(it) => it.into(),
            AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelectorList(it) => it.into(),
            AnyCssPseudoClass::CssPseudoClassFunctionCustomIdentifierList(it) => it.into(),
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
    fn from(n: AnyCssPseudoClass) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssPseudoClassNth> for AnyCssPseudoClassNth {
    fn from(node: CssPseudoClassNth) -> Self {
        Self::CssPseudoClassNth(node)
    }
}
impl From<CssPseudoClassNthIdentifier> for AnyCssPseudoClassNth {
    fn from(node: CssPseudoClassNthIdentifier) -> Self {
        Self::CssPseudoClassNthIdentifier(node)
    }
}
impl From<CssPseudoClassNthNumber> for AnyCssPseudoClassNth {
    fn from(node: CssPseudoClassNthNumber) -> Self {
        Self::CssPseudoClassNthNumber(node)
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
            CSS_PSEUDO_CLASS_NTH => Self::CssPseudoClassNth(CssPseudoClassNth { syntax }),
            CSS_PSEUDO_CLASS_NTH_IDENTIFIER => {
                Self::CssPseudoClassNthIdentifier(CssPseudoClassNthIdentifier { syntax })
            }
            CSS_PSEUDO_CLASS_NTH_NUMBER => {
                Self::CssPseudoClassNthNumber(CssPseudoClassNthNumber { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssPseudoClassNth(it) => &it.syntax,
            Self::CssPseudoClassNthIdentifier(it) => &it.syntax,
            Self::CssPseudoClassNthNumber(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssPseudoClassNth(it) => it.syntax,
            Self::CssPseudoClassNthIdentifier(it) => it.syntax,
            Self::CssPseudoClassNthNumber(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssPseudoClassNth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssPseudoClassNth(it) => std::fmt::Debug::fmt(it, f),
            Self::CssPseudoClassNthIdentifier(it) => std::fmt::Debug::fmt(it, f),
            Self::CssPseudoClassNthNumber(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssPseudoClassNth> for SyntaxNode {
    fn from(n: AnyCssPseudoClassNth) -> Self {
        match n {
            AnyCssPseudoClassNth::CssPseudoClassNth(it) => it.into(),
            AnyCssPseudoClassNth::CssPseudoClassNthIdentifier(it) => it.into(),
            AnyCssPseudoClassNth::CssPseudoClassNthNumber(it) => it.into(),
        }
    }
}
impl From<AnyCssPseudoClassNth> for SyntaxElement {
    fn from(n: AnyCssPseudoClassNth) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusSelector> for AnyCssPseudoClassNthSelector {
    fn from(node: CssBogusSelector) -> Self {
        Self::CssBogusSelector(node)
    }
}
impl From<CssPseudoClassNthSelector> for AnyCssPseudoClassNthSelector {
    fn from(node: CssPseudoClassNthSelector) -> Self {
        Self::CssPseudoClassNthSelector(node)
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
            CSS_BOGUS_SELECTOR => Self::CssBogusSelector(CssBogusSelector { syntax }),
            CSS_PSEUDO_CLASS_NTH_SELECTOR => {
                Self::CssPseudoClassNthSelector(CssPseudoClassNthSelector { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusSelector(it) => &it.syntax,
            Self::CssPseudoClassNthSelector(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusSelector(it) => it.syntax,
            Self::CssPseudoClassNthSelector(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssPseudoClassNthSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusSelector(it) => std::fmt::Debug::fmt(it, f),
            Self::CssPseudoClassNthSelector(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssPseudoClassNthSelector> for SyntaxNode {
    fn from(n: AnyCssPseudoClassNthSelector) -> Self {
        match n {
            AnyCssPseudoClassNthSelector::CssBogusSelector(it) => it.into(),
            AnyCssPseudoClassNthSelector::CssPseudoClassNthSelector(it) => it.into(),
        }
    }
}
impl From<AnyCssPseudoClassNthSelector> for SyntaxElement {
    fn from(n: AnyCssPseudoClassNthSelector) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusPseudoElement> for AnyCssPseudoElement {
    fn from(node: CssBogusPseudoElement) -> Self {
        Self::CssBogusPseudoElement(node)
    }
}
impl From<CssPseudoElementFunction> for AnyCssPseudoElement {
    fn from(node: CssPseudoElementFunction) -> Self {
        Self::CssPseudoElementFunction(node)
    }
}
impl From<CssPseudoElementFunctionCustomIdentifier> for AnyCssPseudoElement {
    fn from(node: CssPseudoElementFunctionCustomIdentifier) -> Self {
        Self::CssPseudoElementFunctionCustomIdentifier(node)
    }
}
impl From<CssPseudoElementFunctionSelector> for AnyCssPseudoElement {
    fn from(node: CssPseudoElementFunctionSelector) -> Self {
        Self::CssPseudoElementFunctionSelector(node)
    }
}
impl From<CssPseudoElementIdentifier> for AnyCssPseudoElement {
    fn from(node: CssPseudoElementIdentifier) -> Self {
        Self::CssPseudoElementIdentifier(node)
    }
}
impl AstNode for AnyCssPseudoElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssBogusPseudoElement::KIND_SET
        .union(CssPseudoElementFunction::KIND_SET)
        .union(CssPseudoElementFunctionCustomIdentifier::KIND_SET)
        .union(CssPseudoElementFunctionSelector::KIND_SET)
        .union(CssPseudoElementIdentifier::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_BOGUS_PSEUDO_ELEMENT
                | CSS_PSEUDO_ELEMENT_FUNCTION
                | CSS_PSEUDO_ELEMENT_FUNCTION_CUSTOM_IDENTIFIER
                | CSS_PSEUDO_ELEMENT_FUNCTION_SELECTOR
                | CSS_PSEUDO_ELEMENT_IDENTIFIER
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_PSEUDO_ELEMENT => {
                Self::CssBogusPseudoElement(CssBogusPseudoElement { syntax })
            }
            CSS_PSEUDO_ELEMENT_FUNCTION => {
                Self::CssPseudoElementFunction(CssPseudoElementFunction { syntax })
            }
            CSS_PSEUDO_ELEMENT_FUNCTION_CUSTOM_IDENTIFIER => {
                Self::CssPseudoElementFunctionCustomIdentifier(
                    CssPseudoElementFunctionCustomIdentifier { syntax },
                )
            }
            CSS_PSEUDO_ELEMENT_FUNCTION_SELECTOR => {
                Self::CssPseudoElementFunctionSelector(CssPseudoElementFunctionSelector { syntax })
            }
            CSS_PSEUDO_ELEMENT_IDENTIFIER => {
                Self::CssPseudoElementIdentifier(CssPseudoElementIdentifier { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusPseudoElement(it) => &it.syntax,
            Self::CssPseudoElementFunction(it) => &it.syntax,
            Self::CssPseudoElementFunctionCustomIdentifier(it) => &it.syntax,
            Self::CssPseudoElementFunctionSelector(it) => &it.syntax,
            Self::CssPseudoElementIdentifier(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusPseudoElement(it) => it.syntax,
            Self::CssPseudoElementFunction(it) => it.syntax,
            Self::CssPseudoElementFunctionCustomIdentifier(it) => it.syntax,
            Self::CssPseudoElementFunctionSelector(it) => it.syntax,
            Self::CssPseudoElementIdentifier(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssPseudoElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusPseudoElement(it) => std::fmt::Debug::fmt(it, f),
            Self::CssPseudoElementFunction(it) => std::fmt::Debug::fmt(it, f),
            Self::CssPseudoElementFunctionCustomIdentifier(it) => std::fmt::Debug::fmt(it, f),
            Self::CssPseudoElementFunctionSelector(it) => std::fmt::Debug::fmt(it, f),
            Self::CssPseudoElementIdentifier(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssPseudoElement> for SyntaxNode {
    fn from(n: AnyCssPseudoElement) -> Self {
        match n {
            AnyCssPseudoElement::CssBogusPseudoElement(it) => it.into(),
            AnyCssPseudoElement::CssPseudoElementFunction(it) => it.into(),
            AnyCssPseudoElement::CssPseudoElementFunctionCustomIdentifier(it) => it.into(),
            AnyCssPseudoElement::CssPseudoElementFunctionSelector(it) => it.into(),
            AnyCssPseudoElement::CssPseudoElementIdentifier(it) => it.into(),
        }
    }
}
impl From<AnyCssPseudoElement> for SyntaxElement {
    fn from(n: AnyCssPseudoElement) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssIdentifier> for AnyCssPseudoValue {
    fn from(node: CssIdentifier) -> Self {
        Self::CssIdentifier(node)
    }
}
impl From<CssString> for AnyCssPseudoValue {
    fn from(node: CssString) -> Self {
        Self::CssString(node)
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
            CSS_IDENTIFIER => Self::CssIdentifier(CssIdentifier { syntax }),
            CSS_STRING => Self::CssString(CssString { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssIdentifier(it) => &it.syntax,
            Self::CssString(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssIdentifier(it) => it.syntax,
            Self::CssString(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssPseudoValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssIdentifier(it) => std::fmt::Debug::fmt(it, f),
            Self::CssString(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssPseudoValue> for SyntaxNode {
    fn from(n: AnyCssPseudoValue) -> Self {
        match n {
            AnyCssPseudoValue::CssIdentifier(it) => it.into(),
            AnyCssPseudoValue::CssString(it) => it.into(),
        }
    }
}
impl From<AnyCssPseudoValue> for SyntaxElement {
    fn from(n: AnyCssPseudoValue) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssQueryFeatureBoolean> for AnyCssQueryFeature {
    fn from(node: CssQueryFeatureBoolean) -> Self {
        Self::CssQueryFeatureBoolean(node)
    }
}
impl From<CssQueryFeaturePlain> for AnyCssQueryFeature {
    fn from(node: CssQueryFeaturePlain) -> Self {
        Self::CssQueryFeaturePlain(node)
    }
}
impl From<CssQueryFeatureRange> for AnyCssQueryFeature {
    fn from(node: CssQueryFeatureRange) -> Self {
        Self::CssQueryFeatureRange(node)
    }
}
impl From<CssQueryFeatureRangeInterval> for AnyCssQueryFeature {
    fn from(node: CssQueryFeatureRangeInterval) -> Self {
        Self::CssQueryFeatureRangeInterval(node)
    }
}
impl From<CssQueryFeatureReverseRange> for AnyCssQueryFeature {
    fn from(node: CssQueryFeatureReverseRange) -> Self {
        Self::CssQueryFeatureReverseRange(node)
    }
}
impl AstNode for AnyCssQueryFeature {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssQueryFeatureBoolean::KIND_SET
        .union(CssQueryFeaturePlain::KIND_SET)
        .union(CssQueryFeatureRange::KIND_SET)
        .union(CssQueryFeatureRangeInterval::KIND_SET)
        .union(CssQueryFeatureReverseRange::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_QUERY_FEATURE_BOOLEAN
                | CSS_QUERY_FEATURE_PLAIN
                | CSS_QUERY_FEATURE_RANGE
                | CSS_QUERY_FEATURE_RANGE_INTERVAL
                | CSS_QUERY_FEATURE_REVERSE_RANGE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_QUERY_FEATURE_BOOLEAN => {
                Self::CssQueryFeatureBoolean(CssQueryFeatureBoolean { syntax })
            }
            CSS_QUERY_FEATURE_PLAIN => Self::CssQueryFeaturePlain(CssQueryFeaturePlain { syntax }),
            CSS_QUERY_FEATURE_RANGE => Self::CssQueryFeatureRange(CssQueryFeatureRange { syntax }),
            CSS_QUERY_FEATURE_RANGE_INTERVAL => {
                Self::CssQueryFeatureRangeInterval(CssQueryFeatureRangeInterval { syntax })
            }
            CSS_QUERY_FEATURE_REVERSE_RANGE => {
                Self::CssQueryFeatureReverseRange(CssQueryFeatureReverseRange { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssQueryFeatureBoolean(it) => &it.syntax,
            Self::CssQueryFeaturePlain(it) => &it.syntax,
            Self::CssQueryFeatureRange(it) => &it.syntax,
            Self::CssQueryFeatureRangeInterval(it) => &it.syntax,
            Self::CssQueryFeatureReverseRange(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssQueryFeatureBoolean(it) => it.syntax,
            Self::CssQueryFeaturePlain(it) => it.syntax,
            Self::CssQueryFeatureRange(it) => it.syntax,
            Self::CssQueryFeatureRangeInterval(it) => it.syntax,
            Self::CssQueryFeatureReverseRange(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssQueryFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssQueryFeatureBoolean(it) => std::fmt::Debug::fmt(it, f),
            Self::CssQueryFeaturePlain(it) => std::fmt::Debug::fmt(it, f),
            Self::CssQueryFeatureRange(it) => std::fmt::Debug::fmt(it, f),
            Self::CssQueryFeatureRangeInterval(it) => std::fmt::Debug::fmt(it, f),
            Self::CssQueryFeatureReverseRange(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssQueryFeature> for SyntaxNode {
    fn from(n: AnyCssQueryFeature) -> Self {
        match n {
            AnyCssQueryFeature::CssQueryFeatureBoolean(it) => it.into(),
            AnyCssQueryFeature::CssQueryFeaturePlain(it) => it.into(),
            AnyCssQueryFeature::CssQueryFeatureRange(it) => it.into(),
            AnyCssQueryFeature::CssQueryFeatureRangeInterval(it) => it.into(),
            AnyCssQueryFeature::CssQueryFeatureReverseRange(it) => it.into(),
        }
    }
}
impl From<AnyCssQueryFeature> for SyntaxElement {
    fn from(n: AnyCssQueryFeature) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssIdentifier> for AnyCssQueryFeatureValue {
    fn from(node: CssIdentifier) -> Self {
        Self::CssIdentifier(node)
    }
}
impl From<CssNumber> for AnyCssQueryFeatureValue {
    fn from(node: CssNumber) -> Self {
        Self::CssNumber(node)
    }
}
impl From<CssRatio> for AnyCssQueryFeatureValue {
    fn from(node: CssRatio) -> Self {
        Self::CssRatio(node)
    }
}
impl AstNode for AnyCssQueryFeatureValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyCssDimension::KIND_SET
        .union(AnyCssFunction::KIND_SET)
        .union(CssIdentifier::KIND_SET)
        .union(CssNumber::KIND_SET)
        .union(CssRatio::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_IDENTIFIER | CSS_NUMBER | CSS_RATIO => true,
            k if AnyCssDimension::can_cast(k) => true,
            k if AnyCssFunction::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_IDENTIFIER => Self::CssIdentifier(CssIdentifier { syntax }),
            CSS_NUMBER => Self::CssNumber(CssNumber { syntax }),
            CSS_RATIO => Self::CssRatio(CssRatio { syntax }),
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
            Self::CssIdentifier(it) => &it.syntax,
            Self::CssNumber(it) => &it.syntax,
            Self::CssRatio(it) => &it.syntax,
            Self::AnyCssDimension(it) => it.syntax(),
            Self::AnyCssFunction(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssIdentifier(it) => it.syntax,
            Self::CssNumber(it) => it.syntax,
            Self::CssRatio(it) => it.syntax,
            Self::AnyCssDimension(it) => it.into_syntax(),
            Self::AnyCssFunction(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssQueryFeatureValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyCssDimension(it) => std::fmt::Debug::fmt(it, f),
            Self::AnyCssFunction(it) => std::fmt::Debug::fmt(it, f),
            Self::CssIdentifier(it) => std::fmt::Debug::fmt(it, f),
            Self::CssNumber(it) => std::fmt::Debug::fmt(it, f),
            Self::CssRatio(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssQueryFeatureValue> for SyntaxNode {
    fn from(n: AnyCssQueryFeatureValue) -> Self {
        match n {
            AnyCssQueryFeatureValue::AnyCssDimension(it) => it.into(),
            AnyCssQueryFeatureValue::AnyCssFunction(it) => it.into(),
            AnyCssQueryFeatureValue::CssIdentifier(it) => it.into(),
            AnyCssQueryFeatureValue::CssNumber(it) => it.into(),
            AnyCssQueryFeatureValue::CssRatio(it) => it.into(),
        }
    }
}
impl From<AnyCssQueryFeatureValue> for SyntaxElement {
    fn from(n: AnyCssQueryFeatureValue) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusSelector> for AnyCssRelativeSelector {
    fn from(node: CssBogusSelector) -> Self {
        Self::CssBogusSelector(node)
    }
}
impl From<CssRelativeSelector> for AnyCssRelativeSelector {
    fn from(node: CssRelativeSelector) -> Self {
        Self::CssRelativeSelector(node)
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
            CSS_BOGUS_SELECTOR => Self::CssBogusSelector(CssBogusSelector { syntax }),
            CSS_RELATIVE_SELECTOR => Self::CssRelativeSelector(CssRelativeSelector { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusSelector(it) => &it.syntax,
            Self::CssRelativeSelector(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusSelector(it) => it.syntax,
            Self::CssRelativeSelector(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssRelativeSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusSelector(it) => std::fmt::Debug::fmt(it, f),
            Self::CssRelativeSelector(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssRelativeSelector> for SyntaxNode {
    fn from(n: AnyCssRelativeSelector) -> Self {
        match n {
            AnyCssRelativeSelector::CssBogusSelector(it) => it.into(),
            AnyCssRelativeSelector::CssRelativeSelector(it) => it.into(),
        }
    }
}
impl From<AnyCssRelativeSelector> for SyntaxElement {
    fn from(n: AnyCssRelativeSelector) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssAtRule> for AnyCssRule {
    fn from(node: CssAtRule) -> Self {
        Self::CssAtRule(node)
    }
}
impl From<CssBogusRule> for AnyCssRule {
    fn from(node: CssBogusRule) -> Self {
        Self::CssBogusRule(node)
    }
}
impl From<CssNestedQualifiedRule> for AnyCssRule {
    fn from(node: CssNestedQualifiedRule) -> Self {
        Self::CssNestedQualifiedRule(node)
    }
}
impl From<CssQualifiedRule> for AnyCssRule {
    fn from(node: CssQualifiedRule) -> Self {
        Self::CssQualifiedRule(node)
    }
}
impl AstNode for AnyCssRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssAtRule::KIND_SET
        .union(CssBogusRule::KIND_SET)
        .union(CssNestedQualifiedRule::KIND_SET)
        .union(CssQualifiedRule::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_AT_RULE | CSS_BOGUS_RULE | CSS_NESTED_QUALIFIED_RULE | CSS_QUALIFIED_RULE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_AT_RULE => Self::CssAtRule(CssAtRule { syntax }),
            CSS_BOGUS_RULE => Self::CssBogusRule(CssBogusRule { syntax }),
            CSS_NESTED_QUALIFIED_RULE => {
                Self::CssNestedQualifiedRule(CssNestedQualifiedRule { syntax })
            }
            CSS_QUALIFIED_RULE => Self::CssQualifiedRule(CssQualifiedRule { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssAtRule(it) => &it.syntax,
            Self::CssBogusRule(it) => &it.syntax,
            Self::CssNestedQualifiedRule(it) => &it.syntax,
            Self::CssQualifiedRule(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssAtRule(it) => it.syntax,
            Self::CssBogusRule(it) => it.syntax,
            Self::CssNestedQualifiedRule(it) => it.syntax,
            Self::CssQualifiedRule(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssAtRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssBogusRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssNestedQualifiedRule(it) => std::fmt::Debug::fmt(it, f),
            Self::CssQualifiedRule(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssRule> for SyntaxNode {
    fn from(n: AnyCssRule) -> Self {
        match n {
            AnyCssRule::CssAtRule(it) => it.into(),
            AnyCssRule::CssBogusRule(it) => it.into(),
            AnyCssRule::CssNestedQualifiedRule(it) => it.into(),
            AnyCssRule::CssQualifiedRule(it) => it.into(),
        }
    }
}
impl From<AnyCssRule> for SyntaxElement {
    fn from(n: AnyCssRule) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusBlock> for AnyCssRuleBlock {
    fn from(node: CssBogusBlock) -> Self {
        Self::CssBogusBlock(node)
    }
}
impl From<CssRuleBlock> for AnyCssRuleBlock {
    fn from(node: CssRuleBlock) -> Self {
        Self::CssRuleBlock(node)
    }
}
impl AstNode for AnyCssRuleBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssBogusBlock::KIND_SET.union(CssRuleBlock::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_BOGUS_BLOCK | CSS_RULE_BLOCK)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_BLOCK => Self::CssBogusBlock(CssBogusBlock { syntax }),
            CSS_RULE_BLOCK => Self::CssRuleBlock(CssRuleBlock { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusBlock(it) => &it.syntax,
            Self::CssRuleBlock(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusBlock(it) => it.syntax,
            Self::CssRuleBlock(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssRuleBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusBlock(it) => std::fmt::Debug::fmt(it, f),
            Self::CssRuleBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssRuleBlock> for SyntaxNode {
    fn from(n: AnyCssRuleBlock) -> Self {
        match n {
            AnyCssRuleBlock::CssBogusBlock(it) => it.into(),
            AnyCssRuleBlock::CssRuleBlock(it) => it.into(),
        }
    }
}
impl From<AnyCssRuleBlock> for SyntaxElement {
    fn from(n: AnyCssRuleBlock) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusScopeRange> for AnyCssScopeRange {
    fn from(node: CssBogusScopeRange) -> Self {
        Self::CssBogusScopeRange(node)
    }
}
impl From<CssScopeRangeEnd> for AnyCssScopeRange {
    fn from(node: CssScopeRangeEnd) -> Self {
        Self::CssScopeRangeEnd(node)
    }
}
impl From<CssScopeRangeInterval> for AnyCssScopeRange {
    fn from(node: CssScopeRangeInterval) -> Self {
        Self::CssScopeRangeInterval(node)
    }
}
impl From<CssScopeRangeStart> for AnyCssScopeRange {
    fn from(node: CssScopeRangeStart) -> Self {
        Self::CssScopeRangeStart(node)
    }
}
impl AstNode for AnyCssScopeRange {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssBogusScopeRange::KIND_SET
        .union(CssScopeRangeEnd::KIND_SET)
        .union(CssScopeRangeInterval::KIND_SET)
        .union(CssScopeRangeStart::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_BOGUS_SCOPE_RANGE
                | CSS_SCOPE_RANGE_END
                | CSS_SCOPE_RANGE_INTERVAL
                | CSS_SCOPE_RANGE_START
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_SCOPE_RANGE => Self::CssBogusScopeRange(CssBogusScopeRange { syntax }),
            CSS_SCOPE_RANGE_END => Self::CssScopeRangeEnd(CssScopeRangeEnd { syntax }),
            CSS_SCOPE_RANGE_INTERVAL => {
                Self::CssScopeRangeInterval(CssScopeRangeInterval { syntax })
            }
            CSS_SCOPE_RANGE_START => Self::CssScopeRangeStart(CssScopeRangeStart { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusScopeRange(it) => &it.syntax,
            Self::CssScopeRangeEnd(it) => &it.syntax,
            Self::CssScopeRangeInterval(it) => &it.syntax,
            Self::CssScopeRangeStart(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusScopeRange(it) => it.syntax,
            Self::CssScopeRangeEnd(it) => it.syntax,
            Self::CssScopeRangeInterval(it) => it.syntax,
            Self::CssScopeRangeStart(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssScopeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusScopeRange(it) => std::fmt::Debug::fmt(it, f),
            Self::CssScopeRangeEnd(it) => std::fmt::Debug::fmt(it, f),
            Self::CssScopeRangeInterval(it) => std::fmt::Debug::fmt(it, f),
            Self::CssScopeRangeStart(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssScopeRange> for SyntaxNode {
    fn from(n: AnyCssScopeRange) -> Self {
        match n {
            AnyCssScopeRange::CssBogusScopeRange(it) => it.into(),
            AnyCssScopeRange::CssScopeRangeEnd(it) => it.into(),
            AnyCssScopeRange::CssScopeRangeInterval(it) => it.into(),
            AnyCssScopeRange::CssScopeRangeStart(it) => it.into(),
        }
    }
}
impl From<AnyCssScopeRange> for SyntaxElement {
    fn from(n: AnyCssScopeRange) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusSelector> for AnyCssSelector {
    fn from(node: CssBogusSelector) -> Self {
        Self::CssBogusSelector(node)
    }
}
impl From<CssComplexSelector> for AnyCssSelector {
    fn from(node: CssComplexSelector) -> Self {
        Self::CssComplexSelector(node)
    }
}
impl From<CssCompoundSelector> for AnyCssSelector {
    fn from(node: CssCompoundSelector) -> Self {
        Self::CssCompoundSelector(node)
    }
}
impl From<CssMetavariable> for AnyCssSelector {
    fn from(node: CssMetavariable) -> Self {
        Self::CssMetavariable(node)
    }
}
impl AstNode for AnyCssSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssBogusSelector::KIND_SET
        .union(CssComplexSelector::KIND_SET)
        .union(CssCompoundSelector::KIND_SET)
        .union(CssMetavariable::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_BOGUS_SELECTOR | CSS_COMPLEX_SELECTOR | CSS_COMPOUND_SELECTOR | CSS_METAVARIABLE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_SELECTOR => Self::CssBogusSelector(CssBogusSelector { syntax }),
            CSS_COMPLEX_SELECTOR => Self::CssComplexSelector(CssComplexSelector { syntax }),
            CSS_COMPOUND_SELECTOR => Self::CssCompoundSelector(CssCompoundSelector { syntax }),
            CSS_METAVARIABLE => Self::CssMetavariable(CssMetavariable { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusSelector(it) => &it.syntax,
            Self::CssComplexSelector(it) => &it.syntax,
            Self::CssCompoundSelector(it) => &it.syntax,
            Self::CssMetavariable(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusSelector(it) => it.syntax,
            Self::CssComplexSelector(it) => it.syntax,
            Self::CssCompoundSelector(it) => it.syntax,
            Self::CssMetavariable(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusSelector(it) => std::fmt::Debug::fmt(it, f),
            Self::CssComplexSelector(it) => std::fmt::Debug::fmt(it, f),
            Self::CssCompoundSelector(it) => std::fmt::Debug::fmt(it, f),
            Self::CssMetavariable(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssSelector> for SyntaxNode {
    fn from(n: AnyCssSelector) -> Self {
        match n {
            AnyCssSelector::CssBogusSelector(it) => it.into(),
            AnyCssSelector::CssComplexSelector(it) => it.into(),
            AnyCssSelector::CssCompoundSelector(it) => it.into(),
            AnyCssSelector::CssMetavariable(it) => it.into(),
        }
    }
}
impl From<AnyCssSelector> for SyntaxElement {
    fn from(n: AnyCssSelector) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssTypeSelector> for AnyCssSimpleSelector {
    fn from(node: CssTypeSelector) -> Self {
        Self::CssTypeSelector(node)
    }
}
impl From<CssUniversalSelector> for AnyCssSimpleSelector {
    fn from(node: CssUniversalSelector) -> Self {
        Self::CssUniversalSelector(node)
    }
}
impl AstNode for AnyCssSimpleSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssTypeSelector::KIND_SET.union(CssUniversalSelector::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_TYPE_SELECTOR | CSS_UNIVERSAL_SELECTOR)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_TYPE_SELECTOR => Self::CssTypeSelector(CssTypeSelector { syntax }),
            CSS_UNIVERSAL_SELECTOR => Self::CssUniversalSelector(CssUniversalSelector { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssTypeSelector(it) => &it.syntax,
            Self::CssUniversalSelector(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssTypeSelector(it) => it.syntax,
            Self::CssUniversalSelector(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssSimpleSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssTypeSelector(it) => std::fmt::Debug::fmt(it, f),
            Self::CssUniversalSelector(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssSimpleSelector> for SyntaxNode {
    fn from(n: AnyCssSimpleSelector) -> Self {
        match n {
            AnyCssSimpleSelector::CssTypeSelector(it) => it.into(),
            AnyCssSimpleSelector::CssUniversalSelector(it) => it.into(),
        }
    }
}
impl From<AnyCssSimpleSelector> for SyntaxElement {
    fn from(n: AnyCssSimpleSelector) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssAttributeSelector> for AnyCssSubSelector {
    fn from(node: CssAttributeSelector) -> Self {
        Self::CssAttributeSelector(node)
    }
}
impl From<CssBogusSubSelector> for AnyCssSubSelector {
    fn from(node: CssBogusSubSelector) -> Self {
        Self::CssBogusSubSelector(node)
    }
}
impl From<CssClassSelector> for AnyCssSubSelector {
    fn from(node: CssClassSelector) -> Self {
        Self::CssClassSelector(node)
    }
}
impl From<CssIdSelector> for AnyCssSubSelector {
    fn from(node: CssIdSelector) -> Self {
        Self::CssIdSelector(node)
    }
}
impl From<CssPseudoClassSelector> for AnyCssSubSelector {
    fn from(node: CssPseudoClassSelector) -> Self {
        Self::CssPseudoClassSelector(node)
    }
}
impl From<CssPseudoElementSelector> for AnyCssSubSelector {
    fn from(node: CssPseudoElementSelector) -> Self {
        Self::CssPseudoElementSelector(node)
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
            CSS_ATTRIBUTE_SELECTOR => Self::CssAttributeSelector(CssAttributeSelector { syntax }),
            CSS_BOGUS_SUB_SELECTOR => Self::CssBogusSubSelector(CssBogusSubSelector { syntax }),
            CSS_CLASS_SELECTOR => Self::CssClassSelector(CssClassSelector { syntax }),
            CSS_ID_SELECTOR => Self::CssIdSelector(CssIdSelector { syntax }),
            CSS_PSEUDO_CLASS_SELECTOR => {
                Self::CssPseudoClassSelector(CssPseudoClassSelector { syntax })
            }
            CSS_PSEUDO_ELEMENT_SELECTOR => {
                Self::CssPseudoElementSelector(CssPseudoElementSelector { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssAttributeSelector(it) => &it.syntax,
            Self::CssBogusSubSelector(it) => &it.syntax,
            Self::CssClassSelector(it) => &it.syntax,
            Self::CssIdSelector(it) => &it.syntax,
            Self::CssPseudoClassSelector(it) => &it.syntax,
            Self::CssPseudoElementSelector(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssAttributeSelector(it) => it.syntax,
            Self::CssBogusSubSelector(it) => it.syntax,
            Self::CssClassSelector(it) => it.syntax,
            Self::CssIdSelector(it) => it.syntax,
            Self::CssPseudoClassSelector(it) => it.syntax,
            Self::CssPseudoElementSelector(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssSubSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssAttributeSelector(it) => std::fmt::Debug::fmt(it, f),
            Self::CssBogusSubSelector(it) => std::fmt::Debug::fmt(it, f),
            Self::CssClassSelector(it) => std::fmt::Debug::fmt(it, f),
            Self::CssIdSelector(it) => std::fmt::Debug::fmt(it, f),
            Self::CssPseudoClassSelector(it) => std::fmt::Debug::fmt(it, f),
            Self::CssPseudoElementSelector(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssSubSelector> for SyntaxNode {
    fn from(n: AnyCssSubSelector) -> Self {
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
    fn from(n: AnyCssSubSelector) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssSupportsAndCondition> for AnyCssSupportsAndCombinableCondition {
    fn from(node: CssSupportsAndCondition) -> Self {
        Self::CssSupportsAndCondition(node)
    }
}
impl AstNode for AnyCssSupportsAndCombinableCondition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyCssSupportsInParens::KIND_SET.union(CssSupportsAndCondition::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_SUPPORTS_AND_CONDITION => true,
            k if AnyCssSupportsInParens::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_SUPPORTS_AND_CONDITION => {
                Self::CssSupportsAndCondition(CssSupportsAndCondition { syntax })
            }
            _ => {
                if let Some(any_css_supports_in_parens) = AnyCssSupportsInParens::cast(syntax) {
                    return Some(Self::AnyCssSupportsInParens(any_css_supports_in_parens));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssSupportsAndCondition(it) => &it.syntax,
            Self::AnyCssSupportsInParens(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssSupportsAndCondition(it) => it.syntax,
            Self::AnyCssSupportsInParens(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssSupportsAndCombinableCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyCssSupportsInParens(it) => std::fmt::Debug::fmt(it, f),
            Self::CssSupportsAndCondition(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssSupportsAndCombinableCondition> for SyntaxNode {
    fn from(n: AnyCssSupportsAndCombinableCondition) -> Self {
        match n {
            AnyCssSupportsAndCombinableCondition::AnyCssSupportsInParens(it) => it.into(),
            AnyCssSupportsAndCombinableCondition::CssSupportsAndCondition(it) => it.into(),
        }
    }
}
impl From<AnyCssSupportsAndCombinableCondition> for SyntaxElement {
    fn from(n: AnyCssSupportsAndCombinableCondition) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusSupportsCondition> for AnyCssSupportsCondition {
    fn from(node: CssBogusSupportsCondition) -> Self {
        Self::CssBogusSupportsCondition(node)
    }
}
impl From<CssSupportsAndCondition> for AnyCssSupportsCondition {
    fn from(node: CssSupportsAndCondition) -> Self {
        Self::CssSupportsAndCondition(node)
    }
}
impl From<CssSupportsNotCondition> for AnyCssSupportsCondition {
    fn from(node: CssSupportsNotCondition) -> Self {
        Self::CssSupportsNotCondition(node)
    }
}
impl From<CssSupportsOrCondition> for AnyCssSupportsCondition {
    fn from(node: CssSupportsOrCondition) -> Self {
        Self::CssSupportsOrCondition(node)
    }
}
impl AstNode for AnyCssSupportsCondition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyCssSupportsInParens::KIND_SET
        .union(CssBogusSupportsCondition::KIND_SET)
        .union(CssSupportsAndCondition::KIND_SET)
        .union(CssSupportsNotCondition::KIND_SET)
        .union(CssSupportsOrCondition::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_BOGUS_SUPPORTS_CONDITION
            | CSS_SUPPORTS_AND_CONDITION
            | CSS_SUPPORTS_NOT_CONDITION
            | CSS_SUPPORTS_OR_CONDITION => true,
            k if AnyCssSupportsInParens::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_SUPPORTS_CONDITION => {
                Self::CssBogusSupportsCondition(CssBogusSupportsCondition { syntax })
            }
            CSS_SUPPORTS_AND_CONDITION => {
                Self::CssSupportsAndCondition(CssSupportsAndCondition { syntax })
            }
            CSS_SUPPORTS_NOT_CONDITION => {
                Self::CssSupportsNotCondition(CssSupportsNotCondition { syntax })
            }
            CSS_SUPPORTS_OR_CONDITION => {
                Self::CssSupportsOrCondition(CssSupportsOrCondition { syntax })
            }
            _ => {
                if let Some(any_css_supports_in_parens) = AnyCssSupportsInParens::cast(syntax) {
                    return Some(Self::AnyCssSupportsInParens(any_css_supports_in_parens));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusSupportsCondition(it) => &it.syntax,
            Self::CssSupportsAndCondition(it) => &it.syntax,
            Self::CssSupportsNotCondition(it) => &it.syntax,
            Self::CssSupportsOrCondition(it) => &it.syntax,
            Self::AnyCssSupportsInParens(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusSupportsCondition(it) => it.syntax,
            Self::CssSupportsAndCondition(it) => it.syntax,
            Self::CssSupportsNotCondition(it) => it.syntax,
            Self::CssSupportsOrCondition(it) => it.syntax,
            Self::AnyCssSupportsInParens(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssSupportsCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyCssSupportsInParens(it) => std::fmt::Debug::fmt(it, f),
            Self::CssBogusSupportsCondition(it) => std::fmt::Debug::fmt(it, f),
            Self::CssSupportsAndCondition(it) => std::fmt::Debug::fmt(it, f),
            Self::CssSupportsNotCondition(it) => std::fmt::Debug::fmt(it, f),
            Self::CssSupportsOrCondition(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssSupportsCondition> for SyntaxNode {
    fn from(n: AnyCssSupportsCondition) -> Self {
        match n {
            AnyCssSupportsCondition::AnyCssSupportsInParens(it) => it.into(),
            AnyCssSupportsCondition::CssBogusSupportsCondition(it) => it.into(),
            AnyCssSupportsCondition::CssSupportsAndCondition(it) => it.into(),
            AnyCssSupportsCondition::CssSupportsNotCondition(it) => it.into(),
            AnyCssSupportsCondition::CssSupportsOrCondition(it) => it.into(),
        }
    }
}
impl From<AnyCssSupportsCondition> for SyntaxElement {
    fn from(n: AnyCssSupportsCondition) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssSupportsConditionInParens> for AnyCssSupportsInParens {
    fn from(node: CssSupportsConditionInParens) -> Self {
        Self::CssSupportsConditionInParens(node)
    }
}
impl From<CssSupportsFeatureDeclaration> for AnyCssSupportsInParens {
    fn from(node: CssSupportsFeatureDeclaration) -> Self {
        Self::CssSupportsFeatureDeclaration(node)
    }
}
impl From<CssSupportsFeatureSelector> for AnyCssSupportsInParens {
    fn from(node: CssSupportsFeatureSelector) -> Self {
        Self::CssSupportsFeatureSelector(node)
    }
}
impl AstNode for AnyCssSupportsInParens {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyCssValue::KIND_SET
        .union(CssSupportsConditionInParens::KIND_SET)
        .union(CssSupportsFeatureDeclaration::KIND_SET)
        .union(CssSupportsFeatureSelector::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_SUPPORTS_CONDITION_IN_PARENS
            | CSS_SUPPORTS_FEATURE_DECLARATION
            | CSS_SUPPORTS_FEATURE_SELECTOR => true,
            k if AnyCssValue::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_SUPPORTS_CONDITION_IN_PARENS => {
                Self::CssSupportsConditionInParens(CssSupportsConditionInParens { syntax })
            }
            CSS_SUPPORTS_FEATURE_DECLARATION => {
                Self::CssSupportsFeatureDeclaration(CssSupportsFeatureDeclaration { syntax })
            }
            CSS_SUPPORTS_FEATURE_SELECTOR => {
                Self::CssSupportsFeatureSelector(CssSupportsFeatureSelector { syntax })
            }
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
            Self::CssSupportsConditionInParens(it) => &it.syntax,
            Self::CssSupportsFeatureDeclaration(it) => &it.syntax,
            Self::CssSupportsFeatureSelector(it) => &it.syntax,
            Self::AnyCssValue(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssSupportsConditionInParens(it) => it.syntax,
            Self::CssSupportsFeatureDeclaration(it) => it.syntax,
            Self::CssSupportsFeatureSelector(it) => it.syntax,
            Self::AnyCssValue(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssSupportsInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyCssValue(it) => std::fmt::Debug::fmt(it, f),
            Self::CssSupportsConditionInParens(it) => std::fmt::Debug::fmt(it, f),
            Self::CssSupportsFeatureDeclaration(it) => std::fmt::Debug::fmt(it, f),
            Self::CssSupportsFeatureSelector(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssSupportsInParens> for SyntaxNode {
    fn from(n: AnyCssSupportsInParens) -> Self {
        match n {
            AnyCssSupportsInParens::AnyCssValue(it) => it.into(),
            AnyCssSupportsInParens::CssSupportsConditionInParens(it) => it.into(),
            AnyCssSupportsInParens::CssSupportsFeatureDeclaration(it) => it.into(),
            AnyCssSupportsInParens::CssSupportsFeatureSelector(it) => it.into(),
        }
    }
}
impl From<AnyCssSupportsInParens> for SyntaxElement {
    fn from(n: AnyCssSupportsInParens) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssSupportsOrCondition> for AnyCssSupportsOrCombinableCondition {
    fn from(node: CssSupportsOrCondition) -> Self {
        Self::CssSupportsOrCondition(node)
    }
}
impl AstNode for AnyCssSupportsOrCombinableCondition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyCssSupportsInParens::KIND_SET.union(CssSupportsOrCondition::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_SUPPORTS_OR_CONDITION => true,
            k if AnyCssSupportsInParens::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_SUPPORTS_OR_CONDITION => {
                Self::CssSupportsOrCondition(CssSupportsOrCondition { syntax })
            }
            _ => {
                if let Some(any_css_supports_in_parens) = AnyCssSupportsInParens::cast(syntax) {
                    return Some(Self::AnyCssSupportsInParens(any_css_supports_in_parens));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssSupportsOrCondition(it) => &it.syntax,
            Self::AnyCssSupportsInParens(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssSupportsOrCondition(it) => it.syntax,
            Self::AnyCssSupportsInParens(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssSupportsOrCombinableCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyCssSupportsInParens(it) => std::fmt::Debug::fmt(it, f),
            Self::CssSupportsOrCondition(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssSupportsOrCombinableCondition> for SyntaxNode {
    fn from(n: AnyCssSupportsOrCombinableCondition) -> Self {
        match n {
            AnyCssSupportsOrCombinableCondition::AnyCssSupportsInParens(it) => it.into(),
            AnyCssSupportsOrCombinableCondition::CssSupportsOrCondition(it) => it.into(),
        }
    }
}
impl From<AnyCssSupportsOrCombinableCondition> for SyntaxElement {
    fn from(n: AnyCssSupportsOrCombinableCondition) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusUnicodeRangeValue> for AnyCssUnicodeValue {
    fn from(node: CssBogusUnicodeRangeValue) -> Self {
        Self::CssBogusUnicodeRangeValue(node)
    }
}
impl From<CssUnicodeCodepoint> for AnyCssUnicodeValue {
    fn from(node: CssUnicodeCodepoint) -> Self {
        Self::CssUnicodeCodepoint(node)
    }
}
impl From<CssUnicodeRangeInterval> for AnyCssUnicodeValue {
    fn from(node: CssUnicodeRangeInterval) -> Self {
        Self::CssUnicodeRangeInterval(node)
    }
}
impl From<CssUnicodeRangeWildcard> for AnyCssUnicodeValue {
    fn from(node: CssUnicodeRangeWildcard) -> Self {
        Self::CssUnicodeRangeWildcard(node)
    }
}
impl AstNode for AnyCssUnicodeValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssBogusUnicodeRangeValue::KIND_SET
        .union(CssUnicodeCodepoint::KIND_SET)
        .union(CssUnicodeRangeInterval::KIND_SET)
        .union(CssUnicodeRangeWildcard::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_BOGUS_UNICODE_RANGE_VALUE
                | CSS_UNICODE_CODEPOINT
                | CSS_UNICODE_RANGE_INTERVAL
                | CSS_UNICODE_RANGE_WILDCARD
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_UNICODE_RANGE_VALUE => {
                Self::CssBogusUnicodeRangeValue(CssBogusUnicodeRangeValue { syntax })
            }
            CSS_UNICODE_CODEPOINT => Self::CssUnicodeCodepoint(CssUnicodeCodepoint { syntax }),
            CSS_UNICODE_RANGE_INTERVAL => {
                Self::CssUnicodeRangeInterval(CssUnicodeRangeInterval { syntax })
            }
            CSS_UNICODE_RANGE_WILDCARD => {
                Self::CssUnicodeRangeWildcard(CssUnicodeRangeWildcard { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusUnicodeRangeValue(it) => &it.syntax,
            Self::CssUnicodeCodepoint(it) => &it.syntax,
            Self::CssUnicodeRangeInterval(it) => &it.syntax,
            Self::CssUnicodeRangeWildcard(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusUnicodeRangeValue(it) => it.syntax,
            Self::CssUnicodeCodepoint(it) => it.syntax,
            Self::CssUnicodeRangeInterval(it) => it.syntax,
            Self::CssUnicodeRangeWildcard(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssUnicodeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusUnicodeRangeValue(it) => std::fmt::Debug::fmt(it, f),
            Self::CssUnicodeCodepoint(it) => std::fmt::Debug::fmt(it, f),
            Self::CssUnicodeRangeInterval(it) => std::fmt::Debug::fmt(it, f),
            Self::CssUnicodeRangeWildcard(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssUnicodeValue> for SyntaxNode {
    fn from(n: AnyCssUnicodeValue) -> Self {
        match n {
            AnyCssUnicodeValue::CssBogusUnicodeRangeValue(it) => it.into(),
            AnyCssUnicodeValue::CssUnicodeCodepoint(it) => it.into(),
            AnyCssUnicodeValue::CssUnicodeRangeInterval(it) => it.into(),
            AnyCssUnicodeValue::CssUnicodeRangeWildcard(it) => it.into(),
        }
    }
}
impl From<AnyCssUnicodeValue> for SyntaxElement {
    fn from(n: AnyCssUnicodeValue) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusUrlModifier> for AnyCssUrlModifier {
    fn from(node: CssBogusUrlModifier) -> Self {
        Self::CssBogusUrlModifier(node)
    }
}
impl From<CssFunction> for AnyCssUrlModifier {
    fn from(node: CssFunction) -> Self {
        Self::CssFunction(node)
    }
}
impl From<CssIdentifier> for AnyCssUrlModifier {
    fn from(node: CssIdentifier) -> Self {
        Self::CssIdentifier(node)
    }
}
impl AstNode for AnyCssUrlModifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssBogusUrlModifier::KIND_SET
        .union(CssFunction::KIND_SET)
        .union(CssIdentifier::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_BOGUS_URL_MODIFIER | CSS_FUNCTION | CSS_IDENTIFIER)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_URL_MODIFIER => Self::CssBogusUrlModifier(CssBogusUrlModifier { syntax }),
            CSS_FUNCTION => Self::CssFunction(CssFunction { syntax }),
            CSS_IDENTIFIER => Self::CssIdentifier(CssIdentifier { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusUrlModifier(it) => &it.syntax,
            Self::CssFunction(it) => &it.syntax,
            Self::CssIdentifier(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusUrlModifier(it) => it.syntax,
            Self::CssFunction(it) => it.syntax,
            Self::CssIdentifier(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssUrlModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusUrlModifier(it) => std::fmt::Debug::fmt(it, f),
            Self::CssFunction(it) => std::fmt::Debug::fmt(it, f),
            Self::CssIdentifier(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssUrlModifier> for SyntaxNode {
    fn from(n: AnyCssUrlModifier) -> Self {
        match n {
            AnyCssUrlModifier::CssBogusUrlModifier(it) => it.into(),
            AnyCssUrlModifier::CssFunction(it) => it.into(),
            AnyCssUrlModifier::CssIdentifier(it) => it.into(),
        }
    }
}
impl From<AnyCssUrlModifier> for SyntaxElement {
    fn from(n: AnyCssUrlModifier) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
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
    const KIND_SET: SyntaxKindSet<Language> = CssString::KIND_SET.union(CssUrlValueRaw::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_STRING | CSS_URL_VALUE_RAW)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_STRING => Self::CssString(CssString { syntax }),
            CSS_URL_VALUE_RAW => Self::CssUrlValueRaw(CssUrlValueRaw { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssString(it) => &it.syntax,
            Self::CssUrlValueRaw(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssString(it) => it.syntax,
            Self::CssUrlValueRaw(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssUrlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssString(it) => std::fmt::Debug::fmt(it, f),
            Self::CssUrlValueRaw(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssUrlValue> for SyntaxNode {
    fn from(n: AnyCssUrlValue) -> Self {
        match n {
            AnyCssUrlValue::CssString(it) => it.into(),
            AnyCssUrlValue::CssUrlValueRaw(it) => it.into(),
        }
    }
}
impl From<AnyCssUrlValue> for SyntaxElement {
    fn from(n: AnyCssUrlValue) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBracketedValue> for AnyCssValue {
    fn from(node: CssBracketedValue) -> Self {
        Self::CssBracketedValue(node)
    }
}
impl From<CssColor> for AnyCssValue {
    fn from(node: CssColor) -> Self {
        Self::CssColor(node)
    }
}
impl From<CssCustomIdentifier> for AnyCssValue {
    fn from(node: CssCustomIdentifier) -> Self {
        Self::CssCustomIdentifier(node)
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
impl From<CssMetavariable> for AnyCssValue {
    fn from(node: CssMetavariable) -> Self {
        Self::CssMetavariable(node)
    }
}
impl From<CssNumber> for AnyCssValue {
    fn from(node: CssNumber) -> Self {
        Self::CssNumber(node)
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
impl From<CssUnicodeRange> for AnyCssValue {
    fn from(node: CssUnicodeRange) -> Self {
        Self::CssUnicodeRange(node)
    }
}
impl AstNode for AnyCssValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyCssDimension::KIND_SET
        .union(AnyCssFunction::KIND_SET)
        .union(CssBracketedValue::KIND_SET)
        .union(CssColor::KIND_SET)
        .union(CssCustomIdentifier::KIND_SET)
        .union(CssDashedIdentifier::KIND_SET)
        .union(CssIdentifier::KIND_SET)
        .union(CssMetavariable::KIND_SET)
        .union(CssNumber::KIND_SET)
        .union(CssRatio::KIND_SET)
        .union(CssString::KIND_SET)
        .union(CssUnicodeRange::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_BRACKETED_VALUE
            | CSS_COLOR
            | CSS_CUSTOM_IDENTIFIER
            | CSS_DASHED_IDENTIFIER
            | CSS_IDENTIFIER
            | CSS_METAVARIABLE
            | CSS_NUMBER
            | CSS_RATIO
            | CSS_STRING
            | CSS_UNICODE_RANGE => true,
            k if AnyCssDimension::can_cast(k) => true,
            k if AnyCssFunction::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BRACKETED_VALUE => Self::CssBracketedValue(CssBracketedValue { syntax }),
            CSS_COLOR => Self::CssColor(CssColor { syntax }),
            CSS_CUSTOM_IDENTIFIER => Self::CssCustomIdentifier(CssCustomIdentifier { syntax }),
            CSS_DASHED_IDENTIFIER => Self::CssDashedIdentifier(CssDashedIdentifier { syntax }),
            CSS_IDENTIFIER => Self::CssIdentifier(CssIdentifier { syntax }),
            CSS_METAVARIABLE => Self::CssMetavariable(CssMetavariable { syntax }),
            CSS_NUMBER => Self::CssNumber(CssNumber { syntax }),
            CSS_RATIO => Self::CssRatio(CssRatio { syntax }),
            CSS_STRING => Self::CssString(CssString { syntax }),
            CSS_UNICODE_RANGE => Self::CssUnicodeRange(CssUnicodeRange { syntax }),
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
            Self::CssBracketedValue(it) => &it.syntax,
            Self::CssColor(it) => &it.syntax,
            Self::CssCustomIdentifier(it) => &it.syntax,
            Self::CssDashedIdentifier(it) => &it.syntax,
            Self::CssIdentifier(it) => &it.syntax,
            Self::CssMetavariable(it) => &it.syntax,
            Self::CssNumber(it) => &it.syntax,
            Self::CssRatio(it) => &it.syntax,
            Self::CssString(it) => &it.syntax,
            Self::CssUnicodeRange(it) => &it.syntax,
            Self::AnyCssDimension(it) => it.syntax(),
            Self::AnyCssFunction(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBracketedValue(it) => it.syntax,
            Self::CssColor(it) => it.syntax,
            Self::CssCustomIdentifier(it) => it.syntax,
            Self::CssDashedIdentifier(it) => it.syntax,
            Self::CssIdentifier(it) => it.syntax,
            Self::CssMetavariable(it) => it.syntax,
            Self::CssNumber(it) => it.syntax,
            Self::CssRatio(it) => it.syntax,
            Self::CssString(it) => it.syntax,
            Self::CssUnicodeRange(it) => it.syntax,
            Self::AnyCssDimension(it) => it.into_syntax(),
            Self::AnyCssFunction(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyCssDimension(it) => std::fmt::Debug::fmt(it, f),
            Self::AnyCssFunction(it) => std::fmt::Debug::fmt(it, f),
            Self::CssBracketedValue(it) => std::fmt::Debug::fmt(it, f),
            Self::CssColor(it) => std::fmt::Debug::fmt(it, f),
            Self::CssCustomIdentifier(it) => std::fmt::Debug::fmt(it, f),
            Self::CssDashedIdentifier(it) => std::fmt::Debug::fmt(it, f),
            Self::CssIdentifier(it) => std::fmt::Debug::fmt(it, f),
            Self::CssMetavariable(it) => std::fmt::Debug::fmt(it, f),
            Self::CssNumber(it) => std::fmt::Debug::fmt(it, f),
            Self::CssRatio(it) => std::fmt::Debug::fmt(it, f),
            Self::CssString(it) => std::fmt::Debug::fmt(it, f),
            Self::CssUnicodeRange(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssValue> for SyntaxNode {
    fn from(n: AnyCssValue) -> Self {
        match n {
            AnyCssValue::AnyCssDimension(it) => it.into(),
            AnyCssValue::AnyCssFunction(it) => it.into(),
            AnyCssValue::CssBracketedValue(it) => it.into(),
            AnyCssValue::CssColor(it) => it.into(),
            AnyCssValue::CssCustomIdentifier(it) => it.into(),
            AnyCssValue::CssDashedIdentifier(it) => it.into(),
            AnyCssValue::CssIdentifier(it) => it.into(),
            AnyCssValue::CssMetavariable(it) => it.into(),
            AnyCssValue::CssNumber(it) => it.into(),
            AnyCssValue::CssRatio(it) => it.into(),
            AnyCssValue::CssString(it) => it.into(),
            AnyCssValue::CssUnicodeRange(it) => it.into(),
        }
    }
}
impl From<AnyCssValue> for SyntaxElement {
    fn from(n: AnyCssValue) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssValueAtRuleDeclarationClause> for AnyCssValueAtRuleClause {
    fn from(node: CssValueAtRuleDeclarationClause) -> Self {
        Self::CssValueAtRuleDeclarationClause(node)
    }
}
impl From<CssValueAtRuleImportClause> for AnyCssValueAtRuleClause {
    fn from(node: CssValueAtRuleImportClause) -> Self {
        Self::CssValueAtRuleImportClause(node)
    }
}
impl AstNode for AnyCssValueAtRuleClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssValueAtRuleDeclarationClause::KIND_SET.union(CssValueAtRuleImportClause::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_VALUE_AT_RULE_DECLARATION_CLAUSE | CSS_VALUE_AT_RULE_IMPORT_CLAUSE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_VALUE_AT_RULE_DECLARATION_CLAUSE => {
                Self::CssValueAtRuleDeclarationClause(CssValueAtRuleDeclarationClause { syntax })
            }
            CSS_VALUE_AT_RULE_IMPORT_CLAUSE => {
                Self::CssValueAtRuleImportClause(CssValueAtRuleImportClause { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssValueAtRuleDeclarationClause(it) => &it.syntax,
            Self::CssValueAtRuleImportClause(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssValueAtRuleDeclarationClause(it) => it.syntax,
            Self::CssValueAtRuleImportClause(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssValueAtRuleClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssValueAtRuleDeclarationClause(it) => std::fmt::Debug::fmt(it, f),
            Self::CssValueAtRuleImportClause(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssValueAtRuleClause> for SyntaxNode {
    fn from(n: AnyCssValueAtRuleClause) -> Self {
        match n {
            AnyCssValueAtRuleClause::CssValueAtRuleDeclarationClause(it) => it.into(),
            AnyCssValueAtRuleClause::CssValueAtRuleImportClause(it) => it.into(),
        }
    }
}
impl From<AnyCssValueAtRuleClause> for SyntaxElement {
    fn from(n: AnyCssValueAtRuleClause) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssIdentifier> for AnyCssValueAtRuleImportSource {
    fn from(node: CssIdentifier) -> Self {
        Self::CssIdentifier(node)
    }
}
impl From<CssString> for AnyCssValueAtRuleImportSource {
    fn from(node: CssString) -> Self {
        Self::CssString(node)
    }
}
impl AstNode for AnyCssValueAtRuleImportSource {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssIdentifier::KIND_SET.union(CssString::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_IDENTIFIER | CSS_STRING)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_IDENTIFIER => Self::CssIdentifier(CssIdentifier { syntax }),
            CSS_STRING => Self::CssString(CssString { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssIdentifier(it) => &it.syntax,
            Self::CssString(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssIdentifier(it) => it.syntax,
            Self::CssString(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssValueAtRuleImportSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssIdentifier(it) => std::fmt::Debug::fmt(it, f),
            Self::CssString(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssValueAtRuleImportSource> for SyntaxNode {
    fn from(n: AnyCssValueAtRuleImportSource) -> Self {
        match n {
            AnyCssValueAtRuleImportSource::CssIdentifier(it) => it.into(),
            AnyCssValueAtRuleImportSource::CssString(it) => it.into(),
        }
    }
}
impl From<AnyCssValueAtRuleImportSource> for SyntaxElement {
    fn from(n: AnyCssValueAtRuleImportSource) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssValueAtRuleImportSpecifier> for AnyCssValueAtRuleImportSpecifier {
    fn from(node: CssValueAtRuleImportSpecifier) -> Self {
        Self::CssValueAtRuleImportSpecifier(node)
    }
}
impl From<CssValueAtRuleNamedImportSpecifier> for AnyCssValueAtRuleImportSpecifier {
    fn from(node: CssValueAtRuleNamedImportSpecifier) -> Self {
        Self::CssValueAtRuleNamedImportSpecifier(node)
    }
}
impl AstNode for AnyCssValueAtRuleImportSpecifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssValueAtRuleImportSpecifier::KIND_SET.union(CssValueAtRuleNamedImportSpecifier::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_VALUE_AT_RULE_IMPORT_SPECIFIER | CSS_VALUE_AT_RULE_NAMED_IMPORT_SPECIFIER
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_VALUE_AT_RULE_IMPORT_SPECIFIER => {
                Self::CssValueAtRuleImportSpecifier(CssValueAtRuleImportSpecifier { syntax })
            }
            CSS_VALUE_AT_RULE_NAMED_IMPORT_SPECIFIER => {
                Self::CssValueAtRuleNamedImportSpecifier(CssValueAtRuleNamedImportSpecifier {
                    syntax,
                })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssValueAtRuleImportSpecifier(it) => &it.syntax,
            Self::CssValueAtRuleNamedImportSpecifier(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssValueAtRuleImportSpecifier(it) => it.syntax,
            Self::CssValueAtRuleNamedImportSpecifier(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssValueAtRuleImportSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssValueAtRuleImportSpecifier(it) => std::fmt::Debug::fmt(it, f),
            Self::CssValueAtRuleNamedImportSpecifier(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssValueAtRuleImportSpecifier> for SyntaxNode {
    fn from(n: AnyCssValueAtRuleImportSpecifier) -> Self {
        match n {
            AnyCssValueAtRuleImportSpecifier::CssValueAtRuleImportSpecifier(it) => it.into(),
            AnyCssValueAtRuleImportSpecifier::CssValueAtRuleNamedImportSpecifier(it) => it.into(),
        }
    }
}
impl From<AnyCssValueAtRuleImportSpecifier> for SyntaxElement {
    fn from(n: AnyCssValueAtRuleImportSpecifier) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusProperty> for AnyCssValueAtRuleProperty {
    fn from(node: CssBogusProperty) -> Self {
        Self::CssBogusProperty(node)
    }
}
impl From<CssValueAtRuleGenericProperty> for AnyCssValueAtRuleProperty {
    fn from(node: CssValueAtRuleGenericProperty) -> Self {
        Self::CssValueAtRuleGenericProperty(node)
    }
}
impl AstNode for AnyCssValueAtRuleProperty {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssBogusProperty::KIND_SET.union(CssValueAtRuleGenericProperty::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_BOGUS_PROPERTY | CSS_VALUE_AT_RULE_GENERIC_PROPERTY
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_PROPERTY => Self::CssBogusProperty(CssBogusProperty { syntax }),
            CSS_VALUE_AT_RULE_GENERIC_PROPERTY => {
                Self::CssValueAtRuleGenericProperty(CssValueAtRuleGenericProperty { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::CssBogusProperty(it) => &it.syntax,
            Self::CssValueAtRuleGenericProperty(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::CssBogusProperty(it) => it.syntax,
            Self::CssValueAtRuleGenericProperty(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssValueAtRuleProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CssBogusProperty(it) => std::fmt::Debug::fmt(it, f),
            Self::CssValueAtRuleGenericProperty(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssValueAtRuleProperty> for SyntaxNode {
    fn from(n: AnyCssValueAtRuleProperty) -> Self {
        match n {
            AnyCssValueAtRuleProperty::CssBogusProperty(it) => it.into(),
            AnyCssValueAtRuleProperty::CssValueAtRuleGenericProperty(it) => it.into(),
        }
    }
}
impl From<AnyCssValueAtRuleProperty> for SyntaxElement {
    fn from(n: AnyCssValueAtRuleProperty) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
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
impl std::fmt::Display for AnyCssComposesImportSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssCompoundSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssConditionalBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssContainerAndCombinableQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssContainerOrCombinableQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssContainerQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssContainerQueryInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssContainerStyleAndCombinableQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssContainerStyleInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssContainerStyleOrCombinableQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssContainerStyleQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssCustomIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssDeclarationBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssDeclarationName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssDeclarationOrAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssDeclarationOrAtRuleBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssDeclarationOrRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssDeclarationOrRuleBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssDocumentMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssFontFamilyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssFontFeatureValuesBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssFontFeatureValuesItem {
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
impl std::fmt::Display for AnyCssImportLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssImportSupportsCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssImportUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssKeyframesBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssKeyframesIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssKeyframesItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssKeyframesName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssKeyframesScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssKeyframesSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssMediaAndCombinableCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssMediaCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssMediaInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssMediaOrCombinableCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssMediaQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssMediaTypeCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssMediaTypeQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssNamespacePrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssNamespaceUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssPageAtRuleBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssPageAtRuleItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssPageSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssPageSelectorPseudo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssProperty {
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
impl std::fmt::Display for AnyCssQueryFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssQueryFeatureValue {
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
impl std::fmt::Display for AnyCssRuleBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssScopeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssSimpleSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssSubSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssSupportsAndCombinableCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssSupportsCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssSupportsInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssSupportsOrCombinableCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssUnicodeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssUrlModifier {
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
impl std::fmt::Display for AnyCssValueAtRuleClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssValueAtRuleImportSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssValueAtRuleImportSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssValueAtRuleProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtRule {
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
impl std::fmt::Display for CssAttributeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAttributeSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssBinaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssBracketedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssCharsetAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssClassSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssColorProfileAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssComplexSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssComposesImportSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssComposesProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssComposesPropertyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssCompoundSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssContainerAndQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssContainerAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssContainerNotQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssContainerOrQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssContainerQueryInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssContainerSizeFeatureInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssContainerStyleAndQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssContainerStyleInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssContainerStyleNotQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssContainerStyleOrQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssContainerStyleQueryInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssCounterStyleAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssCustomIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssDashedIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssDeclarationBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssDeclarationImportant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssDeclarationOrAtRuleBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssDeclarationOrRuleBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssDeclarationWithSemicolon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssDocumentAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssDocumentCustomMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssEmptyDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssFontFaceAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssFontFamilyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssFontFeatureValuesAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssFontFeatureValuesBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssFontFeatureValuesItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssFontPaletteValuesAtRule {
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
impl std::fmt::Display for CssGenericProperty {
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
impl std::fmt::Display for CssImportAnonymousLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssImportAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssImportNamedLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssImportSupports {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssKeyframesAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssKeyframesBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssKeyframesIdentSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssKeyframesItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssKeyframesPercentageSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssKeyframesScopeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssKeyframesScopePrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssKeyframesScopedName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssLayerAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssLayerDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssLayerReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssListOfComponentValuesExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMarginAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaAndCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaAndTypeQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaConditionInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaConditionQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaFeatureInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaNotCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaOrCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaTypeQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMetavariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssNamedNamespacePrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssNamespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssNamespaceAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssNestedQualifiedRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssNestedSelector {
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
impl std::fmt::Display for CssPageAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPageAtRuleBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPageSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPageSelectorPseudo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssParameter {
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
impl std::fmt::Display for CssPositionTryAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPropertyAtRule {
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
impl std::fmt::Display for CssPseudoClassFunctionCustomIdentifierList {
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
impl std::fmt::Display for CssPseudoElementFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoElementFunctionCustomIdentifier {
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
impl std::fmt::Display for CssQualifiedRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssQueryFeatureBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssQueryFeaturePlain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssQueryFeatureRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssQueryFeatureRangeComparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssQueryFeatureRangeInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssQueryFeatureReverseRange {
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
impl std::fmt::Display for CssRuleBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssScopeAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssScopeEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssScopeRangeEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssScopeRangeInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssScopeRangeStart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssStartingStyleAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssSupportsAndCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssSupportsAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssSupportsConditionInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssSupportsFeatureDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssSupportsFeatureSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssSupportsNotCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssSupportsOrCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssTypeSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssUnicodeCodepoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssUnicodeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssUnicodeRangeInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssUnicodeRangeWildcard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssUniversalNamespacePrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssUniversalSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssUnknownBlockAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssUnknownDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssUnknownValueAtRule {
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
impl std::fmt::Display for CssValueAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssValueAtRuleDeclarationClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssValueAtRuleGenericProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssValueAtRuleImportClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssValueAtRuleImportSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssValueAtRuleNamedImportSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssViewTransitionAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
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
    fn from(n: CssBogus) -> Self {
        n.syntax
    }
}
impl From<CssBogus> for SyntaxElement {
    fn from(n: CssBogus) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct CssBogusAtRule {
    syntax: SyntaxNode,
}
impl CssBogusAtRule {
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
impl AstNode for CssBogusAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusAtRule")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusAtRule> for SyntaxNode {
    fn from(n: CssBogusAtRule) -> Self {
        n.syntax
    }
}
impl From<CssBogusAtRule> for SyntaxElement {
    fn from(n: CssBogusAtRule) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct CssBogusBlock {
    syntax: SyntaxNode,
}
impl CssBogusBlock {
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
impl AstNode for CssBogusBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusBlock")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusBlock> for SyntaxNode {
    fn from(n: CssBogusBlock) -> Self {
        n.syntax
    }
}
impl From<CssBogusBlock> for SyntaxElement {
    fn from(n: CssBogusBlock) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct CssBogusCustomIdentifier {
    syntax: SyntaxNode,
}
impl CssBogusCustomIdentifier {
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
impl AstNode for CssBogusCustomIdentifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_CUSTOM_IDENTIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_CUSTOM_IDENTIFIER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusCustomIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusCustomIdentifier")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusCustomIdentifier> for SyntaxNode {
    fn from(n: CssBogusCustomIdentifier) -> Self {
        n.syntax
    }
}
impl From<CssBogusCustomIdentifier> for SyntaxElement {
    fn from(n: CssBogusCustomIdentifier) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct CssBogusDeclarationItem {
    syntax: SyntaxNode,
}
impl CssBogusDeclarationItem {
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
impl AstNode for CssBogusDeclarationItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_DECLARATION_ITEM as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_DECLARATION_ITEM
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusDeclarationItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusDeclarationItem")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusDeclarationItem> for SyntaxNode {
    fn from(n: CssBogusDeclarationItem) -> Self {
        n.syntax
    }
}
impl From<CssBogusDeclarationItem> for SyntaxElement {
    fn from(n: CssBogusDeclarationItem) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct CssBogusDocumentMatcher {
    syntax: SyntaxNode,
}
impl CssBogusDocumentMatcher {
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
impl AstNode for CssBogusDocumentMatcher {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_DOCUMENT_MATCHER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_DOCUMENT_MATCHER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusDocumentMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusDocumentMatcher")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusDocumentMatcher> for SyntaxNode {
    fn from(n: CssBogusDocumentMatcher) -> Self {
        n.syntax
    }
}
impl From<CssBogusDocumentMatcher> for SyntaxElement {
    fn from(n: CssBogusDocumentMatcher) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct CssBogusFontFamilyName {
    syntax: SyntaxNode,
}
impl CssBogusFontFamilyName {
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
impl AstNode for CssBogusFontFamilyName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_FONT_FAMILY_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_FONT_FAMILY_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusFontFamilyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusFontFamilyName")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusFontFamilyName> for SyntaxNode {
    fn from(n: CssBogusFontFamilyName) -> Self {
        n.syntax
    }
}
impl From<CssBogusFontFamilyName> for SyntaxElement {
    fn from(n: CssBogusFontFamilyName) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct CssBogusFontFeatureValuesItem {
    syntax: SyntaxNode,
}
impl CssBogusFontFeatureValuesItem {
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
impl AstNode for CssBogusFontFeatureValuesItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_FONT_FEATURE_VALUES_ITEM as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_FONT_FEATURE_VALUES_ITEM
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusFontFeatureValuesItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusFontFeatureValuesItem")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusFontFeatureValuesItem> for SyntaxNode {
    fn from(n: CssBogusFontFeatureValuesItem) -> Self {
        n.syntax
    }
}
impl From<CssBogusFontFeatureValuesItem> for SyntaxElement {
    fn from(n: CssBogusFontFeatureValuesItem) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct CssBogusKeyframesItem {
    syntax: SyntaxNode,
}
impl CssBogusKeyframesItem {
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
impl AstNode for CssBogusKeyframesItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_KEYFRAMES_ITEM as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_KEYFRAMES_ITEM
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusKeyframesItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusKeyframesItem")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusKeyframesItem> for SyntaxNode {
    fn from(n: CssBogusKeyframesItem) -> Self {
        n.syntax
    }
}
impl From<CssBogusKeyframesItem> for SyntaxElement {
    fn from(n: CssBogusKeyframesItem) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct CssBogusKeyframesName {
    syntax: SyntaxNode,
}
impl CssBogusKeyframesName {
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
impl AstNode for CssBogusKeyframesName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_KEYFRAMES_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_KEYFRAMES_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusKeyframesName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusKeyframesName")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusKeyframesName> for SyntaxNode {
    fn from(n: CssBogusKeyframesName) -> Self {
        n.syntax
    }
}
impl From<CssBogusKeyframesName> for SyntaxElement {
    fn from(n: CssBogusKeyframesName) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct CssBogusLayer {
    syntax: SyntaxNode,
}
impl CssBogusLayer {
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
impl AstNode for CssBogusLayer {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_LAYER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_LAYER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusLayer")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusLayer> for SyntaxNode {
    fn from(n: CssBogusLayer) -> Self {
        n.syntax
    }
}
impl From<CssBogusLayer> for SyntaxElement {
    fn from(n: CssBogusLayer) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct CssBogusMediaQuery {
    syntax: SyntaxNode,
}
impl CssBogusMediaQuery {
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
impl AstNode for CssBogusMediaQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_MEDIA_QUERY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_MEDIA_QUERY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusMediaQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusMediaQuery")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusMediaQuery> for SyntaxNode {
    fn from(n: CssBogusMediaQuery) -> Self {
        n.syntax
    }
}
impl From<CssBogusMediaQuery> for SyntaxElement {
    fn from(n: CssBogusMediaQuery) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct CssBogusPageSelectorPseudo {
    syntax: SyntaxNode,
}
impl CssBogusPageSelectorPseudo {
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
impl AstNode for CssBogusPageSelectorPseudo {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_PAGE_SELECTOR_PSEUDO as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_PAGE_SELECTOR_PSEUDO
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusPageSelectorPseudo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusPageSelectorPseudo")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusPageSelectorPseudo> for SyntaxNode {
    fn from(n: CssBogusPageSelectorPseudo) -> Self {
        n.syntax
    }
}
impl From<CssBogusPageSelectorPseudo> for SyntaxElement {
    fn from(n: CssBogusPageSelectorPseudo) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct CssBogusParameter {
    syntax: SyntaxNode,
}
impl CssBogusParameter {
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
impl AstNode for CssBogusParameter {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_PARAMETER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_PARAMETER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusParameter")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusParameter> for SyntaxNode {
    fn from(n: CssBogusParameter) -> Self {
        n.syntax
    }
}
impl From<CssBogusParameter> for SyntaxElement {
    fn from(n: CssBogusParameter) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct CssBogusProperty {
    syntax: SyntaxNode,
}
impl CssBogusProperty {
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
impl AstNode for CssBogusProperty {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_PROPERTY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_PROPERTY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusProperty")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusProperty> for SyntaxNode {
    fn from(n: CssBogusProperty) -> Self {
        n.syntax
    }
}
impl From<CssBogusProperty> for SyntaxElement {
    fn from(n: CssBogusProperty) -> Self {
        n.syntax.into()
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
        f.debug_struct("CssBogusPropertyValue")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
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
pub struct CssBogusPseudoClass {
    syntax: SyntaxNode,
}
impl CssBogusPseudoClass {
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
impl AstNode for CssBogusPseudoClass {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_PSEUDO_CLASS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_PSEUDO_CLASS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusPseudoClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusPseudoClass")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusPseudoClass> for SyntaxNode {
    fn from(n: CssBogusPseudoClass) -> Self {
        n.syntax
    }
}
impl From<CssBogusPseudoClass> for SyntaxElement {
    fn from(n: CssBogusPseudoClass) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct CssBogusPseudoElement {
    syntax: SyntaxNode,
}
impl CssBogusPseudoElement {
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
impl AstNode for CssBogusPseudoElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_PSEUDO_ELEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_PSEUDO_ELEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusPseudoElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusPseudoElement")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusPseudoElement> for SyntaxNode {
    fn from(n: CssBogusPseudoElement) -> Self {
        n.syntax
    }
}
impl From<CssBogusPseudoElement> for SyntaxElement {
    fn from(n: CssBogusPseudoElement) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
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
    fn from(n: CssBogusRule) -> Self {
        n.syntax
    }
}
impl From<CssBogusRule> for SyntaxElement {
    fn from(n: CssBogusRule) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct CssBogusScopeRange {
    syntax: SyntaxNode,
}
impl CssBogusScopeRange {
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
impl AstNode for CssBogusScopeRange {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_SCOPE_RANGE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_SCOPE_RANGE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusScopeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusScopeRange")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusScopeRange> for SyntaxNode {
    fn from(n: CssBogusScopeRange) -> Self {
        n.syntax
    }
}
impl From<CssBogusScopeRange> for SyntaxElement {
    fn from(n: CssBogusScopeRange) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
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
    fn from(n: CssBogusSelector) -> Self {
        n.syntax
    }
}
impl From<CssBogusSelector> for SyntaxElement {
    fn from(n: CssBogusSelector) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
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
    fn from(n: CssBogusSubSelector) -> Self {
        n.syntax
    }
}
impl From<CssBogusSubSelector> for SyntaxElement {
    fn from(n: CssBogusSubSelector) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct CssBogusSupportsCondition {
    syntax: SyntaxNode,
}
impl CssBogusSupportsCondition {
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
impl AstNode for CssBogusSupportsCondition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_SUPPORTS_CONDITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_SUPPORTS_CONDITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusSupportsCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusSupportsCondition")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusSupportsCondition> for SyntaxNode {
    fn from(n: CssBogusSupportsCondition) -> Self {
        n.syntax
    }
}
impl From<CssBogusSupportsCondition> for SyntaxElement {
    fn from(n: CssBogusSupportsCondition) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct CssBogusUnicodeRangeValue {
    syntax: SyntaxNode,
}
impl CssBogusUnicodeRangeValue {
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
impl AstNode for CssBogusUnicodeRangeValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_UNICODE_RANGE_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_UNICODE_RANGE_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusUnicodeRangeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusUnicodeRangeValue")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusUnicodeRangeValue> for SyntaxNode {
    fn from(n: CssBogusUnicodeRangeValue) -> Self {
        n.syntax
    }
}
impl From<CssBogusUnicodeRangeValue> for SyntaxElement {
    fn from(n: CssBogusUnicodeRangeValue) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct CssBogusUrlModifier {
    syntax: SyntaxNode,
}
impl CssBogusUrlModifier {
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
impl AstNode for CssBogusUrlModifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_URL_MODIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_URL_MODIFIER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusUrlModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusUrlModifier")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusUrlModifier> for SyntaxNode {
    fn from(n: CssBogusUrlModifier) -> Self {
        n.syntax
    }
}
impl From<CssBogusUrlModifier> for SyntaxElement {
    fn from(n: CssBogusUrlModifier) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct CssUnknownAtRuleComponentList {
    syntax: SyntaxNode,
}
impl CssUnknownAtRuleComponentList {
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
impl AstNode for CssUnknownAtRuleComponentList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_UNKNOWN_AT_RULE_COMPONENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_UNKNOWN_AT_RULE_COMPONENT_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssUnknownAtRuleComponentList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssUnknownAtRuleComponentList")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssUnknownAtRuleComponentList> for SyntaxNode {
    fn from(n: CssUnknownAtRuleComponentList) -> Self {
        n.syntax
    }
}
impl From<CssUnknownAtRuleComponentList> for SyntaxElement {
    fn from(n: CssUnknownAtRuleComponentList) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct CssValueAtRuleGenericValue {
    syntax: SyntaxNode,
}
impl CssValueAtRuleGenericValue {
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
impl AstNode for CssValueAtRuleGenericValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_VALUE_AT_RULE_GENERIC_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_VALUE_AT_RULE_GENERIC_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssValueAtRuleGenericValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssValueAtRuleGenericValue")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssValueAtRuleGenericValue> for SyntaxNode {
    fn from(n: CssValueAtRuleGenericValue) -> Self {
        n.syntax
    }
}
impl From<CssValueAtRuleGenericValue> for SyntaxElement {
    fn from(n: CssValueAtRuleGenericValue) -> Self {
        n.syntax.into()
    }
}
biome_rowan::declare_node_union! { pub AnyCssBogusNode = CssBogus | CssBogusAtRule | CssBogusBlock | CssBogusCustomIdentifier | CssBogusDeclarationItem | CssBogusDocumentMatcher | CssBogusFontFamilyName | CssBogusFontFeatureValuesItem | CssBogusKeyframesItem | CssBogusKeyframesName | CssBogusLayer | CssBogusMediaQuery | CssBogusPageSelectorPseudo | CssBogusParameter | CssBogusProperty | CssBogusPropertyValue | CssBogusPseudoClass | CssBogusPseudoElement | CssBogusRule | CssBogusScopeRange | CssBogusSelector | CssBogusSubSelector | CssBogusSupportsCondition | CssBogusUnicodeRangeValue | CssBogusUrlModifier | CssUnknownAtRuleComponentList | CssValueAtRuleGenericValue }
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssBracketedValueList {
    syntax_list: SyntaxList,
}
impl CssBracketedValueList {
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
impl AstNode for CssBracketedValueList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BRACKETED_VALUE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BRACKETED_VALUE_LIST
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
impl Serialize for CssBracketedValueList {
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
impl AstNodeList for CssBracketedValueList {
    type Language = Language;
    type Node = AnyCssCustomIdentifier;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssBracketedValueList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssBracketedValueList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssBracketedValueList {
    type Item = AnyCssCustomIdentifier;
    type IntoIter = AstNodeListIterator<Language, AnyCssCustomIdentifier>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssBracketedValueList {
    type Item = AnyCssCustomIdentifier;
    type IntoIter = AstNodeListIterator<Language, AnyCssCustomIdentifier>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
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
        f.write_str("CssComponentValueList ")?;
        f.debug_list().entries(self.iter()).finish()
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
pub struct CssComposesClassList {
    syntax_list: SyntaxList,
}
impl CssComposesClassList {
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
impl AstNode for CssComposesClassList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_COMPOSES_CLASS_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_COMPOSES_CLASS_LIST
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
impl Serialize for CssComposesClassList {
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
impl AstNodeList for CssComposesClassList {
    type Language = Language;
    type Node = CssCustomIdentifier;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssComposesClassList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssComposesClassList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssComposesClassList {
    type Item = CssCustomIdentifier;
    type IntoIter = AstNodeListIterator<Language, CssCustomIdentifier>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssComposesClassList {
    type Item = CssCustomIdentifier;
    type IntoIter = AstNodeListIterator<Language, CssCustomIdentifier>;
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
pub struct CssCustomIdentifierList {
    syntax_list: SyntaxList,
}
impl CssCustomIdentifierList {
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
impl AstNode for CssCustomIdentifierList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CUSTOM_IDENTIFIER_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CUSTOM_IDENTIFIER_LIST
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
impl Serialize for CssCustomIdentifierList {
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
impl AstNodeList for CssCustomIdentifierList {
    type Language = Language;
    type Node = AnyCssCustomIdentifier;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssCustomIdentifierList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssCustomIdentifierList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssCustomIdentifierList {
    type Item = AnyCssCustomIdentifier;
    type IntoIter = AstNodeListIterator<Language, AnyCssCustomIdentifier>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssCustomIdentifierList {
    type Item = AnyCssCustomIdentifier;
    type IntoIter = AstNodeListIterator<Language, AnyCssCustomIdentifier>;
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
    type Node = AnyCssDeclaration;
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
    type Item = AnyCssDeclaration;
    type IntoIter = AstNodeListIterator<Language, AnyCssDeclaration>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssDeclarationList {
    type Item = AnyCssDeclaration;
    type IntoIter = AstNodeListIterator<Language, AnyCssDeclaration>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssDeclarationOrAtRuleList {
    syntax_list: SyntaxList,
}
impl CssDeclarationOrAtRuleList {
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
impl AstNode for CssDeclarationOrAtRuleList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_DECLARATION_OR_AT_RULE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_DECLARATION_OR_AT_RULE_LIST
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
impl Serialize for CssDeclarationOrAtRuleList {
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
impl AstNodeList for CssDeclarationOrAtRuleList {
    type Language = Language;
    type Node = AnyCssDeclarationOrAtRule;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssDeclarationOrAtRuleList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssDeclarationOrAtRuleList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssDeclarationOrAtRuleList {
    type Item = AnyCssDeclarationOrAtRule;
    type IntoIter = AstNodeListIterator<Language, AnyCssDeclarationOrAtRule>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssDeclarationOrAtRuleList {
    type Item = AnyCssDeclarationOrAtRule;
    type IntoIter = AstNodeListIterator<Language, AnyCssDeclarationOrAtRule>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssDeclarationOrRuleList {
    syntax_list: SyntaxList,
}
impl CssDeclarationOrRuleList {
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
impl AstNode for CssDeclarationOrRuleList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_DECLARATION_OR_RULE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_DECLARATION_OR_RULE_LIST
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
impl Serialize for CssDeclarationOrRuleList {
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
impl AstNodeList for CssDeclarationOrRuleList {
    type Language = Language;
    type Node = AnyCssDeclarationOrRule;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssDeclarationOrRuleList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssDeclarationOrRuleList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssDeclarationOrRuleList {
    type Item = AnyCssDeclarationOrRule;
    type IntoIter = AstNodeListIterator<Language, AnyCssDeclarationOrRule>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssDeclarationOrRuleList {
    type Item = AnyCssDeclarationOrRule;
    type IntoIter = AstNodeListIterator<Language, AnyCssDeclarationOrRule>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssDocumentMatcherList {
    syntax_list: SyntaxList,
}
impl CssDocumentMatcherList {
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
impl AstNode for CssDocumentMatcherList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_DOCUMENT_MATCHER_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_DOCUMENT_MATCHER_LIST
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
impl Serialize for CssDocumentMatcherList {
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
impl AstSeparatedList for CssDocumentMatcherList {
    type Language = Language;
    type Node = AnyCssDocumentMatcher;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssDocumentMatcherList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssDocumentMatcherList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssDocumentMatcherList {
    type Item = SyntaxResult<AnyCssDocumentMatcher>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssDocumentMatcher>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssDocumentMatcherList {
    type Item = SyntaxResult<AnyCssDocumentMatcher>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssDocumentMatcher>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssFontFamilyNameList {
    syntax_list: SyntaxList,
}
impl CssFontFamilyNameList {
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
impl AstNode for CssFontFamilyNameList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_FONT_FAMILY_NAME_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_FONT_FAMILY_NAME_LIST
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
impl Serialize for CssFontFamilyNameList {
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
impl AstSeparatedList for CssFontFamilyNameList {
    type Language = Language;
    type Node = AnyCssFontFamilyName;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssFontFamilyNameList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssFontFamilyNameList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssFontFamilyNameList {
    type Item = SyntaxResult<AnyCssFontFamilyName>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssFontFamilyName>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssFontFamilyNameList {
    type Item = SyntaxResult<AnyCssFontFamilyName>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssFontFamilyName>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssFontFeatureValuesItemList {
    syntax_list: SyntaxList,
}
impl CssFontFeatureValuesItemList {
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
impl AstNode for CssFontFeatureValuesItemList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_FONT_FEATURE_VALUES_ITEM_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_FONT_FEATURE_VALUES_ITEM_LIST
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
impl Serialize for CssFontFeatureValuesItemList {
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
impl AstNodeList for CssFontFeatureValuesItemList {
    type Language = Language;
    type Node = AnyCssFontFeatureValuesItem;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssFontFeatureValuesItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssFontFeatureValuesItemList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssFontFeatureValuesItemList {
    type Item = AnyCssFontFeatureValuesItem;
    type IntoIter = AstNodeListIterator<Language, AnyCssFontFeatureValuesItem>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssFontFeatureValuesItemList {
    type Item = AnyCssFontFeatureValuesItem;
    type IntoIter = AstNodeListIterator<Language, AnyCssFontFeatureValuesItem>;
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
        f.write_str("CssGenericComponentValueList ")?;
        f.debug_list().entries(self.iter()).finish()
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
pub struct CssKeyframesItemList {
    syntax_list: SyntaxList,
}
impl CssKeyframesItemList {
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
impl AstNode for CssKeyframesItemList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_KEYFRAMES_ITEM_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_KEYFRAMES_ITEM_LIST
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
impl Serialize for CssKeyframesItemList {
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
impl AstNodeList for CssKeyframesItemList {
    type Language = Language;
    type Node = AnyCssKeyframesItem;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssKeyframesItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssKeyframesItemList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssKeyframesItemList {
    type Item = AnyCssKeyframesItem;
    type IntoIter = AstNodeListIterator<Language, AnyCssKeyframesItem>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssKeyframesItemList {
    type Item = AnyCssKeyframesItem;
    type IntoIter = AstNodeListIterator<Language, AnyCssKeyframesItem>;
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
    type Node = AnyCssKeyframesSelector;
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
    type Item = SyntaxResult<AnyCssKeyframesSelector>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssKeyframesSelector>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssKeyframesSelectorList {
    type Item = SyntaxResult<AnyCssKeyframesSelector>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssKeyframesSelector>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssLayerNameList {
    syntax_list: SyntaxList,
}
impl CssLayerNameList {
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
impl AstNode for CssLayerNameList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_LAYER_NAME_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_LAYER_NAME_LIST
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
impl Serialize for CssLayerNameList {
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
impl AstSeparatedList for CssLayerNameList {
    type Language = Language;
    type Node = CssIdentifier;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssLayerNameList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssLayerNameList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssLayerNameList {
    type Item = SyntaxResult<CssIdentifier>;
    type IntoIter = AstSeparatedListNodesIterator<Language, CssIdentifier>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssLayerNameList {
    type Item = SyntaxResult<CssIdentifier>;
    type IntoIter = AstSeparatedListNodesIterator<Language, CssIdentifier>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssLayerReferenceList {
    syntax_list: SyntaxList,
}
impl CssLayerReferenceList {
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
impl AstNode for CssLayerReferenceList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_LAYER_REFERENCE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_LAYER_REFERENCE_LIST
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
impl Serialize for CssLayerReferenceList {
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
impl AstSeparatedList for CssLayerReferenceList {
    type Language = Language;
    type Node = CssLayerNameList;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssLayerReferenceList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssLayerReferenceList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssLayerReferenceList {
    type Item = SyntaxResult<CssLayerNameList>;
    type IntoIter = AstSeparatedListNodesIterator<Language, CssLayerNameList>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssLayerReferenceList {
    type Item = SyntaxResult<CssLayerNameList>;
    type IntoIter = AstSeparatedListNodesIterator<Language, CssLayerNameList>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssMediaQueryList {
    syntax_list: SyntaxList,
}
impl CssMediaQueryList {
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
impl AstNode for CssMediaQueryList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_QUERY_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_QUERY_LIST
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
impl Serialize for CssMediaQueryList {
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
impl AstSeparatedList for CssMediaQueryList {
    type Language = Language;
    type Node = AnyCssMediaQuery;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssMediaQueryList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssMediaQueryList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssMediaQueryList {
    type Item = SyntaxResult<AnyCssMediaQuery>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssMediaQuery>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssMediaQueryList {
    type Item = SyntaxResult<AnyCssMediaQuery>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssMediaQuery>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssNestedSelectorList {
    syntax_list: SyntaxList,
}
impl CssNestedSelectorList {
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
impl AstNode for CssNestedSelectorList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_NESTED_SELECTOR_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_NESTED_SELECTOR_LIST
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
impl Serialize for CssNestedSelectorList {
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
impl AstNodeList for CssNestedSelectorList {
    type Language = Language;
    type Node = CssNestedSelector;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssNestedSelectorList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssNestedSelectorList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssNestedSelectorList {
    type Item = CssNestedSelector;
    type IntoIter = AstNodeListIterator<Language, CssNestedSelector>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssNestedSelectorList {
    type Item = CssNestedSelector;
    type IntoIter = AstNodeListIterator<Language, CssNestedSelector>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssPageAtRuleItemList {
    syntax_list: SyntaxList,
}
impl CssPageAtRuleItemList {
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
impl AstNode for CssPageAtRuleItemList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PAGE_AT_RULE_ITEM_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PAGE_AT_RULE_ITEM_LIST
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
impl Serialize for CssPageAtRuleItemList {
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
impl AstNodeList for CssPageAtRuleItemList {
    type Language = Language;
    type Node = AnyCssPageAtRuleItem;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssPageAtRuleItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssPageAtRuleItemList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssPageAtRuleItemList {
    type Item = AnyCssPageAtRuleItem;
    type IntoIter = AstNodeListIterator<Language, AnyCssPageAtRuleItem>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssPageAtRuleItemList {
    type Item = AnyCssPageAtRuleItem;
    type IntoIter = AstNodeListIterator<Language, AnyCssPageAtRuleItem>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssPageSelectorList {
    syntax_list: SyntaxList,
}
impl CssPageSelectorList {
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
impl AstNode for CssPageSelectorList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PAGE_SELECTOR_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PAGE_SELECTOR_LIST
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
impl Serialize for CssPageSelectorList {
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
impl AstSeparatedList for CssPageSelectorList {
    type Language = Language;
    type Node = AnyCssPageSelector;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssPageSelectorList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssPageSelectorList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssPageSelectorList {
    type Item = SyntaxResult<AnyCssPageSelector>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssPageSelector>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssPageSelectorList {
    type Item = SyntaxResult<AnyCssPageSelector>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssPageSelector>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssPageSelectorPseudoList {
    syntax_list: SyntaxList,
}
impl CssPageSelectorPseudoList {
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
impl AstNode for CssPageSelectorPseudoList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PAGE_SELECTOR_PSEUDO_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PAGE_SELECTOR_PSEUDO_LIST
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
impl Serialize for CssPageSelectorPseudoList {
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
impl AstNodeList for CssPageSelectorPseudoList {
    type Language = Language;
    type Node = AnyCssPageSelectorPseudo;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssPageSelectorPseudoList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssPageSelectorPseudoList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssPageSelectorPseudoList {
    type Item = AnyCssPageSelectorPseudo;
    type IntoIter = AstNodeListIterator<Language, AnyCssPageSelectorPseudo>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssPageSelectorPseudoList {
    type Item = AnyCssPageSelectorPseudo;
    type IntoIter = AstNodeListIterator<Language, AnyCssPageSelectorPseudo>;
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
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssParameterList {
    type Item = SyntaxResult<CssParameter>;
    type IntoIter = AstSeparatedListNodesIterator<Language, CssParameter>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssParameterList {
    type Item = SyntaxResult<CssParameter>;
    type IntoIter = AstSeparatedListNodesIterator<Language, CssParameter>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssPseudoElementFunctionParameterList {
    syntax_list: SyntaxList,
}
impl CssPseudoElementFunctionParameterList {
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
impl AstNode for CssPseudoElementFunctionParameterList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        CSS_PSEUDO_ELEMENT_FUNCTION_PARAMETER_LIST as u16,
    ));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_ELEMENT_FUNCTION_PARAMETER_LIST
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
impl Serialize for CssPseudoElementFunctionParameterList {
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
impl AstNodeList for CssPseudoElementFunctionParameterList {
    type Language = Language;
    type Node = CssIdentifier;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssPseudoElementFunctionParameterList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssPseudoElementFunctionParameterList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssPseudoElementFunctionParameterList {
    type Item = CssIdentifier;
    type IntoIter = AstNodeListIterator<Language, CssIdentifier>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssPseudoElementFunctionParameterList {
    type Item = CssIdentifier;
    type IntoIter = AstNodeListIterator<Language, CssIdentifier>;
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
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssUrlModifierList {
    syntax_list: SyntaxList,
}
impl CssUrlModifierList {
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
impl AstNode for CssUrlModifierList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_URL_MODIFIER_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_URL_MODIFIER_LIST
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
impl Serialize for CssUrlModifierList {
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
impl AstNodeList for CssUrlModifierList {
    type Language = Language;
    type Node = AnyCssUrlModifier;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssUrlModifierList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssUrlModifierList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssUrlModifierList {
    type Item = AnyCssUrlModifier;
    type IntoIter = AstNodeListIterator<Language, AnyCssUrlModifier>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssUrlModifierList {
    type Item = AnyCssUrlModifier;
    type IntoIter = AstNodeListIterator<Language, AnyCssUrlModifier>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssValueAtRuleImportSpecifierList {
    syntax_list: SyntaxList,
}
impl CssValueAtRuleImportSpecifierList {
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
impl AstNode for CssValueAtRuleImportSpecifierList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        CSS_VALUE_AT_RULE_IMPORT_SPECIFIER_LIST as u16,
    ));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_VALUE_AT_RULE_IMPORT_SPECIFIER_LIST
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
impl Serialize for CssValueAtRuleImportSpecifierList {
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
impl AstSeparatedList for CssValueAtRuleImportSpecifierList {
    type Language = Language;
    type Node = AnyCssValueAtRuleImportSpecifier;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssValueAtRuleImportSpecifierList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssValueAtRuleImportSpecifierList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssValueAtRuleImportSpecifierList {
    type Item = SyntaxResult<AnyCssValueAtRuleImportSpecifier>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssValueAtRuleImportSpecifier>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssValueAtRuleImportSpecifierList {
    type Item = SyntaxResult<AnyCssValueAtRuleImportSpecifier>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssValueAtRuleImportSpecifier>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssValueAtRulePropertyList {
    syntax_list: SyntaxList,
}
impl CssValueAtRulePropertyList {
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
impl AstNode for CssValueAtRulePropertyList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_VALUE_AT_RULE_PROPERTY_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_VALUE_AT_RULE_PROPERTY_LIST
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
impl Serialize for CssValueAtRulePropertyList {
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
impl AstSeparatedList for CssValueAtRulePropertyList {
    type Language = Language;
    type Node = AnyCssValueAtRuleProperty;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssValueAtRulePropertyList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssValueAtRulePropertyList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssValueAtRulePropertyList {
    type Item = SyntaxResult<AnyCssValueAtRuleProperty>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssValueAtRuleProperty>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssValueAtRulePropertyList {
    type Item = SyntaxResult<AnyCssValueAtRuleProperty>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssValueAtRuleProperty>;
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
