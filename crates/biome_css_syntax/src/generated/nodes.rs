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
#[cfg(feature = "serde")]
impl Serialize for CssAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
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
#[cfg(feature = "serde")]
impl Serialize for CssAttributeName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
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
    pub name: SyntaxResult<CssAttributeName>,
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
#[cfg(feature = "serde")]
impl Serialize for CssCharsetAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
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
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn block(&self) -> SyntaxResult<CssBlock> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssColorProfileAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssColorProfileAtRuleFields {
    pub color_profile_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<CssIdentifier>,
    pub block: SyntaxResult<CssBlock>,
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
    pub fn simple_selector(&self) -> Option<AnyCssSimpleSelector> {
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
    pub fn right(&self) -> SyntaxResult<AnyCssContainerCombinableQuery> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssContainerAndQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssContainerAndQueryFields {
    pub left: SyntaxResult<AnyCssContainerQueryInParens>,
    pub and_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyCssContainerCombinableQuery>,
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
    pub fn name(&self) -> Option<CssIdentifier> {
        support::node(&self.syntax, 1usize)
    }
    pub fn query(&self) -> SyntaxResult<AnyCssContainerQuery> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn block(&self) -> SyntaxResult<CssBlock> {
        support::required_node(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssContainerAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssContainerAtRuleFields {
    pub container_token: SyntaxResult<SyntaxToken>,
    pub name: Option<CssIdentifier>,
    pub query: SyntaxResult<AnyCssContainerQuery>,
    pub block: SyntaxResult<CssBlock>,
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
#[cfg(feature = "serde")]
impl Serialize for CssContainerNotQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
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
    pub fn right(&self) -> SyntaxResult<AnyCssContainerCombinableQuery> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssContainerOrQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssContainerOrQueryFields {
    pub left: SyntaxResult<AnyCssContainerQueryInParens>,
    pub or_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyCssContainerCombinableQuery>,
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
#[cfg(feature = "serde")]
impl Serialize for CssContainerQueryInParens {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
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
            query: self.query(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn query(&self) -> SyntaxResult<AnyCssContainerSizeFeature> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssContainerSizeFeatureInParens {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssContainerSizeFeatureInParensFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub query: SyntaxResult<AnyCssContainerSizeFeature>,
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
    pub fn right(&self) -> SyntaxResult<AnyCssContainerStyleCombinableQuery> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssContainerStyleAndQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssContainerStyleAndQueryFields {
    pub left: SyntaxResult<CssContainerStyleInParens>,
    pub and_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyCssContainerStyleCombinableQuery>,
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
#[cfg(feature = "serde")]
impl Serialize for CssContainerStyleInParens {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
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
#[cfg(feature = "serde")]
impl Serialize for CssContainerStyleNotQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
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
    pub fn right(&self) -> SyntaxResult<AnyCssContainerStyleCombinableQuery> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssContainerStyleOrQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssContainerStyleOrQueryFields {
    pub left: SyntaxResult<CssContainerStyleInParens>,
    pub or_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyCssContainerStyleCombinableQuery>,
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
#[cfg(feature = "serde")]
impl Serialize for CssContainerStyleQueryInParens {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
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
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn block(&self) -> SyntaxResult<CssBlock> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssCounterStyleAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssCounterStyleAtRuleFields {
    pub counter_style_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<CssIdentifier>,
    pub block: SyntaxResult<CssBlock>,
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
            value: self.value(),
        }
    }
    pub fn value(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
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
    pub value: SyntaxResult<CssIdentifier>,
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
            colon_token: self.colon_token(),
            value: self.value(),
            important: self.important(),
        }
    }
    pub fn name(&self) -> SyntaxResult<AnyCssDeclarationName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> CssComponentValueList {
        support::list(&self.syntax, 2usize)
    }
    pub fn important(&self) -> Option<CssDeclarationImportant> {
        support::node(&self.syntax, 3usize)
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
    pub name: SyntaxResult<AnyCssDeclarationName>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: CssComponentValueList,
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
    pub fn block(&self) -> SyntaxResult<CssBlock> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssFontFaceAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssFontFaceAtRuleFields {
    pub font_face_token: SyntaxResult<SyntaxToken>,
    pub block: SyntaxResult<CssBlock>,
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
            css_string: self.css_string(),
            body: self.body(),
        }
    }
    pub fn keyframes_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn css_string(&self) -> SyntaxResult<CssString> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn body(&self) -> SyntaxResult<CssKeyframesBody> {
        support::required_node(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssKeyframesAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssKeyframesAtRuleFields {
    pub keyframes_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<CssIdentifier>,
    pub css_string: SyntaxResult<CssString>,
    pub body: SyntaxResult<CssKeyframesBody>,
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
pub struct CssKeyframesBody {
    pub(crate) syntax: SyntaxNode,
}
impl CssKeyframesBody {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssKeyframesBodyFields {
        CssKeyframesBodyFields {
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
#[cfg(feature = "serde")]
impl Serialize for CssKeyframesBody {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssKeyframesBodyFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub items: CssKeyframesItemList,
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
            query_list: self.query_list(),
            l_curly_token: self.l_curly_token(),
            body: self.body(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn media_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn query_list(&self) -> CssMediaQueryList {
        support::list(&self.syntax, 1usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn body(&self) -> SyntaxResult<AnyCssRule> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssMediaAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssMediaAtRuleFields {
    pub media_token: SyntaxResult<SyntaxToken>,
    pub query_list: CssMediaQueryList,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub body: SyntaxResult<AnyCssRule>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaQuery {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaQuery {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaQueryFields {
        CssMediaQueryFields {
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
    pub fn ty(&self) -> SyntaxResult<AnyCssMediaQueryType> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn consequent(&self) -> Option<CssMediaQueryConsequent> {
        support::node(&self.syntax, 4usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssMediaQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssMediaQueryFields {
    pub condition_token: SyntaxResult<SyntaxToken>,
    pub or_token: SyntaxResult<SyntaxToken>,
    pub only_token: Option<SyntaxToken>,
    pub ty: SyntaxResult<AnyCssMediaQueryType>,
    pub consequent: Option<CssMediaQueryConsequent>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaQueryConsequent {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaQueryConsequent {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaQueryConsequentFields {
        CssMediaQueryConsequentFields {
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
    pub fn ty(&self) -> SyntaxResult<AnyCssMediaQueryType> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssMediaQueryConsequent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssMediaQueryConsequentFields {
    pub and_token: SyntaxResult<SyntaxToken>,
    pub condition_token: Option<SyntaxToken>,
    pub ty: SyntaxResult<AnyCssMediaQueryType>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaQueryFeature {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaQueryFeature {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaQueryFeatureFields {
        CssMediaQueryFeatureFields {
            l_paren_token: self.l_paren_token(),
            feature: self.feature(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn feature(&self) -> SyntaxResult<AnyCssMediaQueryFeatureType> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssMediaQueryFeature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssMediaQueryFeatureFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub feature: SyntaxResult<AnyCssMediaQueryFeatureType>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaQueryFeatureBoolean {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaQueryFeatureBoolean {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaQueryFeatureBooleanFields {
        CssMediaQueryFeatureBooleanFields {
            css_identifier: self.css_identifier(),
        }
    }
    pub fn css_identifier(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssMediaQueryFeatureBoolean {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssMediaQueryFeatureBooleanFields {
    pub css_identifier: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaQueryFeatureCompare {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaQueryFeatureCompare {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaQueryFeatureCompareFields {
        CssMediaQueryFeatureCompareFields {
            name: self.name(),
            range: self.range(),
            value: self.value(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn range(&self) -> SyntaxResult<CssMediaQueryRange> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<AnyCssValue> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssMediaQueryFeatureCompare {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssMediaQueryFeatureCompareFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub range: SyntaxResult<CssMediaQueryRange>,
    pub value: SyntaxResult<AnyCssValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaQueryFeaturePlain {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaQueryFeaturePlain {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaQueryFeaturePlainFields {
        CssMediaQueryFeaturePlainFields {
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
impl Serialize for CssMediaQueryFeaturePlain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssMediaQueryFeaturePlainFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnyCssValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaQueryFeatureRange {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaQueryFeatureRange {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaQueryFeatureRangeFields {
        CssMediaQueryFeatureRangeFields {
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
    pub fn first_range(&self) -> SyntaxResult<CssMediaQueryRange> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn second_value(&self) -> SyntaxResult<AnyCssValue> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn second_range(&self) -> SyntaxResult<CssMediaQueryRange> {
        support::required_node(&self.syntax, 4usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssMediaQueryFeatureRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssMediaQueryFeatureRangeFields {
    pub first_value: SyntaxResult<AnyCssValue>,
    pub first_range: SyntaxResult<CssMediaQueryRange>,
    pub name: SyntaxResult<CssIdentifier>,
    pub second_value: SyntaxResult<AnyCssValue>,
    pub second_range: SyntaxResult<CssMediaQueryRange>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaQueryRange {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaQueryRange {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaQueryRangeFields {
        CssMediaQueryRangeFields {
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
impl Serialize for CssMediaQueryRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssMediaQueryRangeFields {
    pub r_angle_token: SyntaxResult<SyntaxToken>,
    pub l_angle_token: SyntaxResult<SyntaxToken>,
    pub greater_than_equal_token: SyntaxResult<SyntaxToken>,
    pub less_than_equal_token: SyntaxResult<SyntaxToken>,
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
#[cfg(feature = "serde")]
impl Serialize for CssNamedNamespacePrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
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
#[cfg(feature = "serde")]
impl Serialize for CssNamespace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssNamespaceFields {
    pub prefix: Option<AnyCssNamespacePrefix>,
    pub bitwise_or_token: SyntaxResult<SyntaxToken>,
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
            css_component_value_list: self.css_component_value_list(),
        }
    }
    pub fn css_component_value_list(&self) -> CssComponentValueList {
        support::list(&self.syntax, 0usize)
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
    pub css_component_value_list: CssComponentValueList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPercentDimension {
    pub(crate) syntax: SyntaxNode,
}
impl CssPercentDimension {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPercentDimensionFields {
        CssPercentDimensionFields {
            value: self.value(),
            unit_token: self.unit_token(),
        }
    }
    pub fn value(&self) -> SyntaxResult<CssNumber> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn unit_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPercentDimension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPercentDimensionFields {
    pub value: SyntaxResult<CssNumber>,
    pub unit_token: SyntaxResult<SyntaxToken>,
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
    pub fn name(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
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
    pub name: SyntaxResult<SyntaxToken>,
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
    pub fn name(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
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
    pub name: SyntaxResult<SyntaxToken>,
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
            name_token: self.name_token(),
            l_paren_token: self.l_paren_token(),
            ident: self.ident(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
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
    pub name_token: SyntaxResult<SyntaxToken>,
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
    pub fn name(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
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
    pub name: SyntaxResult<SyntaxToken>,
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
            name_token: self.name_token(),
            l_paren_token: self.l_paren_token(),
            relative_selector_list: self.relative_selector_list(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
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
    pub name_token: SyntaxResult<SyntaxToken>,
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
    pub fn name(&self) -> SyntaxResult<SyntaxToken> {
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
    pub name: SyntaxResult<SyntaxToken>,
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
    pub fn name(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
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
    pub name: SyntaxResult<SyntaxToken>,
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
            name_token: self.name_token(),
            l_paren_token: self.l_paren_token(),
            value_list: self.value_list(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
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
    pub name_token: SyntaxResult<SyntaxToken>,
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
impl Serialize for CssRegularDimension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssRegularDimensionFields {
    pub value: SyntaxResult<CssNumber>,
    pub unit: SyntaxResult<CssIdentifier>,
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
pub struct CssSizeFeatureBoolean {
    pub(crate) syntax: SyntaxNode,
}
impl CssSizeFeatureBoolean {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssSizeFeatureBooleanFields {
        CssSizeFeatureBooleanFields { name: self.name() }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssSizeFeatureBoolean {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssSizeFeatureBooleanFields {
    pub name: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssSizeFeaturePlain {
    pub(crate) syntax: SyntaxNode,
}
impl CssSizeFeaturePlain {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssSizeFeaturePlainFields {
        CssSizeFeaturePlainFields {
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
    pub fn value(&self) -> SyntaxResult<AnyCssSizeFeatureValue> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssSizeFeaturePlain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssSizeFeaturePlainFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnyCssSizeFeatureValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssSizeFeatureRange {
    pub(crate) syntax: SyntaxNode,
}
impl CssSizeFeatureRange {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssSizeFeatureRangeFields {
        CssSizeFeatureRangeFields {
            left: self.left(),
            comparison: self.comparison(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn comparison(&self) -> SyntaxResult<CssSizeFeatureRangeComparison> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyCssSizeFeatureValue> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssSizeFeatureRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssSizeFeatureRangeFields {
    pub left: SyntaxResult<CssIdentifier>,
    pub comparison: SyntaxResult<CssSizeFeatureRangeComparison>,
    pub right: SyntaxResult<AnyCssSizeFeatureValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssSizeFeatureRangeComparison {
    pub(crate) syntax: SyntaxNode,
}
impl CssSizeFeatureRangeComparison {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssSizeFeatureRangeComparisonFields {
        CssSizeFeatureRangeComparisonFields {
            operator: self.operator(),
        }
    }
    pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssSizeFeatureRangeComparison {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssSizeFeatureRangeComparisonFields {
    pub operator: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssSizeFeatureRangeInterval {
    pub(crate) syntax: SyntaxNode,
}
impl CssSizeFeatureRangeInterval {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssSizeFeatureRangeIntervalFields {
        CssSizeFeatureRangeIntervalFields {
            left: self.left(),
            left_comparison: self.left_comparison(),
            name: self.name(),
            right_comparison: self.right_comparison(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyCssSizeFeatureValue> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn left_comparison(&self) -> SyntaxResult<CssSizeFeatureRangeComparison> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn right_comparison(&self) -> SyntaxResult<CssSizeFeatureRangeComparison> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyCssSizeFeatureValue> {
        support::required_node(&self.syntax, 4usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssSizeFeatureRangeInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssSizeFeatureRangeIntervalFields {
    pub left: SyntaxResult<AnyCssSizeFeatureValue>,
    pub left_comparison: SyntaxResult<CssSizeFeatureRangeComparison>,
    pub name: SyntaxResult<CssIdentifier>,
    pub right_comparison: SyntaxResult<CssSizeFeatureRangeComparison>,
    pub right: SyntaxResult<AnyCssSizeFeatureValue>,
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
    pub namespace: Option<CssNamespace>,
    pub ident: SyntaxResult<CssIdentifier>,
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
#[cfg(feature = "serde")]
impl Serialize for CssUniversalNamespacePrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
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
    pub namespace: Option<CssNamespace>,
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
pub enum AnyCssAtRule {
    CssBogusAtRule(CssBogusAtRule),
    CssCharsetAtRule(CssCharsetAtRule),
    CssColorProfileAtRule(CssColorProfileAtRule),
    CssContainerAtRule(CssContainerAtRule),
    CssCounterStyleAtRule(CssCounterStyleAtRule),
    CssFontFaceAtRule(CssFontFaceAtRule),
    CssKeyframesAtRule(CssKeyframesAtRule),
    CssMediaAtRule(CssMediaAtRule),
}
impl AnyCssAtRule {
    pub fn as_css_bogus_at_rule(&self) -> Option<&CssBogusAtRule> {
        match &self {
            AnyCssAtRule::CssBogusAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_charset_at_rule(&self) -> Option<&CssCharsetAtRule> {
        match &self {
            AnyCssAtRule::CssCharsetAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_color_profile_at_rule(&self) -> Option<&CssColorProfileAtRule> {
        match &self {
            AnyCssAtRule::CssColorProfileAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_at_rule(&self) -> Option<&CssContainerAtRule> {
        match &self {
            AnyCssAtRule::CssContainerAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_counter_style_at_rule(&self) -> Option<&CssCounterStyleAtRule> {
        match &self {
            AnyCssAtRule::CssCounterStyleAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_font_face_at_rule(&self) -> Option<&CssFontFaceAtRule> {
        match &self {
            AnyCssAtRule::CssFontFaceAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_keyframes_at_rule(&self) -> Option<&CssKeyframesAtRule> {
        match &self {
            AnyCssAtRule::CssKeyframesAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_at_rule(&self) -> Option<&CssMediaAtRule> {
        match &self {
            AnyCssAtRule::CssMediaAtRule(item) => Some(item),
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
pub enum AnyCssContainerCombinableQuery {
    AnyCssContainerQueryInParens(AnyCssContainerQueryInParens),
    CssContainerAndQuery(CssContainerAndQuery),
    CssContainerOrQuery(CssContainerOrQuery),
}
impl AnyCssContainerCombinableQuery {
    pub fn as_any_css_container_query_in_parens(&self) -> Option<&AnyCssContainerQueryInParens> {
        match &self {
            AnyCssContainerCombinableQuery::AnyCssContainerQueryInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_and_query(&self) -> Option<&CssContainerAndQuery> {
        match &self {
            AnyCssContainerCombinableQuery::CssContainerAndQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_or_query(&self) -> Option<&CssContainerOrQuery> {
        match &self {
            AnyCssContainerCombinableQuery::CssContainerOrQuery(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssContainerQuery {
    AnyCssContainerCombinableQuery(AnyCssContainerCombinableQuery),
    CssContainerNotQuery(CssContainerNotQuery),
}
impl AnyCssContainerQuery {
    pub fn as_any_css_container_combinable_query(&self) -> Option<&AnyCssContainerCombinableQuery> {
        match &self {
            AnyCssContainerQuery::AnyCssContainerCombinableQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_not_query(&self) -> Option<&CssContainerNotQuery> {
        match &self {
            AnyCssContainerQuery::CssContainerNotQuery(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssContainerQueryInParens {
    CssContainerQueryInParens(CssContainerQueryInParens),
    CssContainerSizeFeatureInParens(CssContainerSizeFeatureInParens),
    CssContainerStyleQueryInParens(CssContainerStyleQueryInParens),
}
impl AnyCssContainerQueryInParens {
    pub fn as_css_container_query_in_parens(&self) -> Option<&CssContainerQueryInParens> {
        match &self {
            AnyCssContainerQueryInParens::CssContainerQueryInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_size_feature_in_parens(
        &self,
    ) -> Option<&CssContainerSizeFeatureInParens> {
        match &self {
            AnyCssContainerQueryInParens::CssContainerSizeFeatureInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_style_query_in_parens(
        &self,
    ) -> Option<&CssContainerStyleQueryInParens> {
        match &self {
            AnyCssContainerQueryInParens::CssContainerStyleQueryInParens(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssContainerSizeFeature {
    CssSizeFeatureBoolean(CssSizeFeatureBoolean),
    CssSizeFeaturePlain(CssSizeFeaturePlain),
    CssSizeFeatureRange(CssSizeFeatureRange),
    CssSizeFeatureRangeInterval(CssSizeFeatureRangeInterval),
}
impl AnyCssContainerSizeFeature {
    pub fn as_css_size_feature_boolean(&self) -> Option<&CssSizeFeatureBoolean> {
        match &self {
            AnyCssContainerSizeFeature::CssSizeFeatureBoolean(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_size_feature_plain(&self) -> Option<&CssSizeFeaturePlain> {
        match &self {
            AnyCssContainerSizeFeature::CssSizeFeaturePlain(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_size_feature_range(&self) -> Option<&CssSizeFeatureRange> {
        match &self {
            AnyCssContainerSizeFeature::CssSizeFeatureRange(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_size_feature_range_interval(&self) -> Option<&CssSizeFeatureRangeInterval> {
        match &self {
            AnyCssContainerSizeFeature::CssSizeFeatureRangeInterval(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssContainerStyleCombinableQuery {
    CssContainerStyleAndQuery(CssContainerStyleAndQuery),
    CssContainerStyleInParens(CssContainerStyleInParens),
    CssContainerStyleOrQuery(CssContainerStyleOrQuery),
}
impl AnyCssContainerStyleCombinableQuery {
    pub fn as_css_container_style_and_query(&self) -> Option<&CssContainerStyleAndQuery> {
        match &self {
            AnyCssContainerStyleCombinableQuery::CssContainerStyleAndQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_style_in_parens(&self) -> Option<&CssContainerStyleInParens> {
        match &self {
            AnyCssContainerStyleCombinableQuery::CssContainerStyleInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_style_or_query(&self) -> Option<&CssContainerStyleOrQuery> {
        match &self {
            AnyCssContainerStyleCombinableQuery::CssContainerStyleOrQuery(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssContainerStyleInParens {
    AnyCssContainerStyleQuery(AnyCssContainerStyleQuery),
    CssDeclaration(CssDeclaration),
}
impl AnyCssContainerStyleInParens {
    pub fn as_any_css_container_style_query(&self) -> Option<&AnyCssContainerStyleQuery> {
        match &self {
            AnyCssContainerStyleInParens::AnyCssContainerStyleQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_declaration(&self) -> Option<&CssDeclaration> {
        match &self {
            AnyCssContainerStyleInParens::CssDeclaration(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssContainerStyleQuery {
    AnyCssContainerStyleCombinableQuery(AnyCssContainerStyleCombinableQuery),
    CssContainerStyleNotQuery(CssContainerStyleNotQuery),
    CssDeclaration(CssDeclaration),
}
impl AnyCssContainerStyleQuery {
    pub fn as_any_css_container_style_combinable_query(
        &self,
    ) -> Option<&AnyCssContainerStyleCombinableQuery> {
        match &self {
            AnyCssContainerStyleQuery::AnyCssContainerStyleCombinableQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_style_not_query(&self) -> Option<&CssContainerStyleNotQuery> {
        match &self {
            AnyCssContainerStyleQuery::CssContainerStyleNotQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_declaration(&self) -> Option<&CssDeclaration> {
        match &self {
            AnyCssContainerStyleQuery::CssDeclaration(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssDeclarationName {
    CssCustomProperty(CssCustomProperty),
    CssIdentifier(CssIdentifier),
}
impl AnyCssDeclarationName {
    pub fn as_css_custom_property(&self) -> Option<&CssCustomProperty> {
        match &self {
            AnyCssDeclarationName::CssCustomProperty(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_identifier(&self) -> Option<&CssIdentifier> {
        match &self {
            AnyCssDeclarationName::CssIdentifier(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssDimension {
    CssPercentage(CssPercentage),
    CssRegularDimension(CssRegularDimension),
}
impl AnyCssDimension {
    pub fn as_css_percentage(&self) -> Option<&CssPercentage> {
        match &self {
            AnyCssDimension::CssPercentage(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_regular_dimension(&self) -> Option<&CssRegularDimension> {
        match &self {
            AnyCssDimension::CssRegularDimension(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssMediaQueryFeatureType {
    CssMediaQueryFeatureBoolean(CssMediaQueryFeatureBoolean),
    CssMediaQueryFeatureCompare(CssMediaQueryFeatureCompare),
    CssMediaQueryFeaturePlain(CssMediaQueryFeaturePlain),
    CssMediaQueryFeatureRange(CssMediaQueryFeatureRange),
}
impl AnyCssMediaQueryFeatureType {
    pub fn as_css_media_query_feature_boolean(&self) -> Option<&CssMediaQueryFeatureBoolean> {
        match &self {
            AnyCssMediaQueryFeatureType::CssMediaQueryFeatureBoolean(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_query_feature_compare(&self) -> Option<&CssMediaQueryFeatureCompare> {
        match &self {
            AnyCssMediaQueryFeatureType::CssMediaQueryFeatureCompare(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_query_feature_plain(&self) -> Option<&CssMediaQueryFeaturePlain> {
        match &self {
            AnyCssMediaQueryFeatureType::CssMediaQueryFeaturePlain(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_query_feature_range(&self) -> Option<&CssMediaQueryFeatureRange> {
        match &self {
            AnyCssMediaQueryFeatureType::CssMediaQueryFeatureRange(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssMediaQueryType {
    CssIdentifier(CssIdentifier),
    CssMediaQueryFeature(CssMediaQueryFeature),
}
impl AnyCssMediaQueryType {
    pub fn as_css_identifier(&self) -> Option<&CssIdentifier> {
        match &self {
            AnyCssMediaQueryType::CssIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_query_feature(&self) -> Option<&CssMediaQueryFeature> {
        match &self {
            AnyCssMediaQueryType::CssMediaQueryFeature(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssNamespacePrefix {
    CssNamedNamespacePrefix(CssNamedNamespacePrefix),
    CssUniversalNamespacePrefix(CssUniversalNamespacePrefix),
}
impl AnyCssNamespacePrefix {
    pub fn as_css_named_namespace_prefix(&self) -> Option<&CssNamedNamespacePrefix> {
        match &self {
            AnyCssNamespacePrefix::CssNamedNamespacePrefix(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_universal_namespace_prefix(&self) -> Option<&CssUniversalNamespacePrefix> {
        match &self {
            AnyCssNamespacePrefix::CssUniversalNamespacePrefix(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssPseudoClass {
    CssBogusPseudoClass(CssBogusPseudoClass),
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
    pub fn as_css_bogus_pseudo_class(&self) -> Option<&CssBogusPseudoClass> {
        match &self {
            AnyCssPseudoClass::CssBogusPseudoClass(item) => Some(item),
            _ => None,
        }
    }
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
    CssBogusPseudoElement(CssBogusPseudoElement),
    CssPseudoElementFunctionIdentifier(CssPseudoElementFunctionIdentifier),
    CssPseudoElementFunctionSelector(CssPseudoElementFunctionSelector),
    CssPseudoElementIdentifier(CssPseudoElementIdentifier),
}
impl AnyCssPseudoElement {
    pub fn as_css_bogus_pseudo_element(&self) -> Option<&CssBogusPseudoElement> {
        match &self {
            AnyCssPseudoElement::CssBogusPseudoElement(item) => Some(item),
            _ => None,
        }
    }
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
    CssAtRule(CssAtRule),
    CssBogusRule(CssBogusRule),
    CssRule(CssRule),
}
impl AnyCssRule {
    pub fn as_css_at_rule(&self) -> Option<&CssAtRule> {
        match &self {
            AnyCssRule::CssAtRule(item) => Some(item),
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
pub enum AnyCssSimpleSelector {
    CssTypeSelector(CssTypeSelector),
    CssUniversalSelector(CssUniversalSelector),
}
impl AnyCssSimpleSelector {
    pub fn as_css_type_selector(&self) -> Option<&CssTypeSelector> {
        match &self {
            AnyCssSimpleSelector::CssTypeSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_universal_selector(&self) -> Option<&CssUniversalSelector> {
        match &self {
            AnyCssSimpleSelector::CssUniversalSelector(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssSizeFeatureValue {
    AnyCssDimension(AnyCssDimension),
    CssAnyFunction(CssAnyFunction),
    CssIdentifier(CssIdentifier),
    CssNumber(CssNumber),
    CssRatio(CssRatio),
}
impl AnyCssSizeFeatureValue {
    pub fn as_any_css_dimension(&self) -> Option<&AnyCssDimension> {
        match &self {
            AnyCssSizeFeatureValue::AnyCssDimension(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_any_function(&self) -> Option<&CssAnyFunction> {
        match &self {
            AnyCssSizeFeatureValue::CssAnyFunction(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_identifier(&self) -> Option<&CssIdentifier> {
        match &self {
            AnyCssSizeFeatureValue::CssIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_number(&self) -> Option<&CssNumber> {
        match &self {
            AnyCssSizeFeatureValue::CssNumber(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_ratio(&self) -> Option<&CssRatio> {
        match &self {
            AnyCssSizeFeatureValue::CssRatio(item) => Some(item),
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
    AnyCssDimension(AnyCssDimension),
    CssAnyFunction(CssAnyFunction),
    CssCustomProperty(CssCustomProperty),
    CssIdentifier(CssIdentifier),
    CssNumber(CssNumber),
    CssRatio(CssRatio),
    CssString(CssString),
}
impl AnyCssValue {
    pub fn as_any_css_dimension(&self) -> Option<&AnyCssDimension> {
        match &self {
            AnyCssValue::AnyCssDimension(item) => Some(item),
            _ => None,
        }
    }
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
        f.debug_struct("CssAtRule")
            .field("at_token", &support::DebugSyntaxResult(self.at_token()))
            .field("rule", &support::DebugSyntaxResult(self.rule()))
            .finish()
    }
}
impl From<CssAtRule> for SyntaxNode {
    fn from(n: CssAtRule) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssAtRule> for SyntaxElement {
    fn from(n: CssAtRule) -> SyntaxElement {
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
        f.debug_struct("CssAttributeName")
            .field(
                "namespace",
                &support::DebugOptionalElement(self.namespace()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .finish()
    }
}
impl From<CssAttributeName> for SyntaxNode {
    fn from(n: CssAttributeName) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssAttributeName> for SyntaxElement {
    fn from(n: CssAttributeName) -> SyntaxElement {
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
    }
}
impl From<CssCharsetAtRule> for SyntaxNode {
    fn from(n: CssCharsetAtRule) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssCharsetAtRule> for SyntaxElement {
    fn from(n: CssCharsetAtRule) -> SyntaxElement {
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
        f.debug_struct("CssColorProfileAtRule")
            .field(
                "color_profile_token",
                &support::DebugSyntaxResult(self.color_profile_token()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field("block", &support::DebugSyntaxResult(self.block()))
            .finish()
    }
}
impl From<CssColorProfileAtRule> for SyntaxNode {
    fn from(n: CssColorProfileAtRule) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssColorProfileAtRule> for SyntaxElement {
    fn from(n: CssColorProfileAtRule) -> SyntaxElement {
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
        f.debug_struct("CssContainerAndQuery")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field("and_token", &support::DebugSyntaxResult(self.and_token()))
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<CssContainerAndQuery> for SyntaxNode {
    fn from(n: CssContainerAndQuery) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssContainerAndQuery> for SyntaxElement {
    fn from(n: CssContainerAndQuery) -> SyntaxElement {
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
        f.debug_struct("CssContainerAtRule")
            .field(
                "container_token",
                &support::DebugSyntaxResult(self.container_token()),
            )
            .field("name", &support::DebugOptionalElement(self.name()))
            .field("query", &support::DebugSyntaxResult(self.query()))
            .field("block", &support::DebugSyntaxResult(self.block()))
            .finish()
    }
}
impl From<CssContainerAtRule> for SyntaxNode {
    fn from(n: CssContainerAtRule) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssContainerAtRule> for SyntaxElement {
    fn from(n: CssContainerAtRule) -> SyntaxElement {
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
        f.debug_struct("CssContainerNotQuery")
            .field("not_token", &support::DebugSyntaxResult(self.not_token()))
            .field("query", &support::DebugSyntaxResult(self.query()))
            .finish()
    }
}
impl From<CssContainerNotQuery> for SyntaxNode {
    fn from(n: CssContainerNotQuery) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssContainerNotQuery> for SyntaxElement {
    fn from(n: CssContainerNotQuery) -> SyntaxElement {
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
        f.debug_struct("CssContainerOrQuery")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field("or_token", &support::DebugSyntaxResult(self.or_token()))
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<CssContainerOrQuery> for SyntaxNode {
    fn from(n: CssContainerOrQuery) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssContainerOrQuery> for SyntaxElement {
    fn from(n: CssContainerOrQuery) -> SyntaxElement {
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
    }
}
impl From<CssContainerQueryInParens> for SyntaxNode {
    fn from(n: CssContainerQueryInParens) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssContainerQueryInParens> for SyntaxElement {
    fn from(n: CssContainerQueryInParens) -> SyntaxElement {
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
        f.debug_struct("CssContainerSizeFeatureInParens")
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
    }
}
impl From<CssContainerSizeFeatureInParens> for SyntaxNode {
    fn from(n: CssContainerSizeFeatureInParens) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssContainerSizeFeatureInParens> for SyntaxElement {
    fn from(n: CssContainerSizeFeatureInParens) -> SyntaxElement {
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
        f.debug_struct("CssContainerStyleAndQuery")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field("and_token", &support::DebugSyntaxResult(self.and_token()))
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<CssContainerStyleAndQuery> for SyntaxNode {
    fn from(n: CssContainerStyleAndQuery) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssContainerStyleAndQuery> for SyntaxElement {
    fn from(n: CssContainerStyleAndQuery) -> SyntaxElement {
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
    }
}
impl From<CssContainerStyleInParens> for SyntaxNode {
    fn from(n: CssContainerStyleInParens) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssContainerStyleInParens> for SyntaxElement {
    fn from(n: CssContainerStyleInParens) -> SyntaxElement {
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
        f.debug_struct("CssContainerStyleNotQuery")
            .field("not_token", &support::DebugSyntaxResult(self.not_token()))
            .field("query", &support::DebugSyntaxResult(self.query()))
            .finish()
    }
}
impl From<CssContainerStyleNotQuery> for SyntaxNode {
    fn from(n: CssContainerStyleNotQuery) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssContainerStyleNotQuery> for SyntaxElement {
    fn from(n: CssContainerStyleNotQuery) -> SyntaxElement {
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
        f.debug_struct("CssContainerStyleOrQuery")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field("or_token", &support::DebugSyntaxResult(self.or_token()))
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<CssContainerStyleOrQuery> for SyntaxNode {
    fn from(n: CssContainerStyleOrQuery) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssContainerStyleOrQuery> for SyntaxElement {
    fn from(n: CssContainerStyleOrQuery) -> SyntaxElement {
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
    }
}
impl From<CssContainerStyleQueryInParens> for SyntaxNode {
    fn from(n: CssContainerStyleQueryInParens) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssContainerStyleQueryInParens> for SyntaxElement {
    fn from(n: CssContainerStyleQueryInParens) -> SyntaxElement {
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
        f.debug_struct("CssCounterStyleAtRule")
            .field(
                "counter_style_token",
                &support::DebugSyntaxResult(self.counter_style_token()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field("block", &support::DebugSyntaxResult(self.block()))
            .finish()
    }
}
impl From<CssCounterStyleAtRule> for SyntaxNode {
    fn from(n: CssCounterStyleAtRule) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssCounterStyleAtRule> for SyntaxElement {
    fn from(n: CssCounterStyleAtRule) -> SyntaxElement {
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
            .field("value", &support::DebugSyntaxResult(self.value()))
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
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("value", &self.value())
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
        f.debug_struct("CssFontFaceAtRule")
            .field(
                "font_face_token",
                &support::DebugSyntaxResult(self.font_face_token()),
            )
            .field("block", &support::DebugSyntaxResult(self.block()))
            .finish()
    }
}
impl From<CssFontFaceAtRule> for SyntaxNode {
    fn from(n: CssFontFaceAtRule) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssFontFaceAtRule> for SyntaxElement {
    fn from(n: CssFontFaceAtRule) -> SyntaxElement {
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
        f.debug_struct("CssKeyframesAtRule")
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
impl From<CssKeyframesAtRule> for SyntaxNode {
    fn from(n: CssKeyframesAtRule) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssKeyframesAtRule> for SyntaxElement {
    fn from(n: CssKeyframesAtRule) -> SyntaxElement {
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
impl AstNode for CssKeyframesBody {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_KEYFRAMES_BODY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_KEYFRAMES_BODY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssKeyframesBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssKeyframesBody")
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
impl From<CssKeyframesBody> for SyntaxNode {
    fn from(n: CssKeyframesBody) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssKeyframesBody> for SyntaxElement {
    fn from(n: CssKeyframesBody) -> SyntaxElement {
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
        f.debug_struct("CssMediaAtRule")
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
impl From<CssMediaAtRule> for SyntaxNode {
    fn from(n: CssMediaAtRule) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssMediaAtRule> for SyntaxElement {
    fn from(n: CssMediaAtRule) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssMediaQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_QUERY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_QUERY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssMediaQuery")
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
impl From<CssMediaQuery> for SyntaxNode {
    fn from(n: CssMediaQuery) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssMediaQuery> for SyntaxElement {
    fn from(n: CssMediaQuery) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssMediaQueryConsequent {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_QUERY_CONSEQUENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_QUERY_CONSEQUENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaQueryConsequent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssMediaQueryConsequent")
            .field("and_token", &support::DebugSyntaxResult(self.and_token()))
            .field(
                "condition_token",
                &support::DebugOptionalElement(self.condition_token()),
            )
            .field("ty", &support::DebugSyntaxResult(self.ty()))
            .finish()
    }
}
impl From<CssMediaQueryConsequent> for SyntaxNode {
    fn from(n: CssMediaQueryConsequent) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssMediaQueryConsequent> for SyntaxElement {
    fn from(n: CssMediaQueryConsequent) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssMediaQueryFeature {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_QUERY_FEATURE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_QUERY_FEATURE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaQueryFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssMediaQueryFeature")
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
impl From<CssMediaQueryFeature> for SyntaxNode {
    fn from(n: CssMediaQueryFeature) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssMediaQueryFeature> for SyntaxElement {
    fn from(n: CssMediaQueryFeature) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssMediaQueryFeatureBoolean {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_QUERY_FEATURE_BOOLEAN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_QUERY_FEATURE_BOOLEAN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaQueryFeatureBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssMediaQueryFeatureBoolean")
            .field(
                "css_identifier",
                &support::DebugSyntaxResult(self.css_identifier()),
            )
            .finish()
    }
}
impl From<CssMediaQueryFeatureBoolean> for SyntaxNode {
    fn from(n: CssMediaQueryFeatureBoolean) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssMediaQueryFeatureBoolean> for SyntaxElement {
    fn from(n: CssMediaQueryFeatureBoolean) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssMediaQueryFeatureCompare {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_QUERY_FEATURE_COMPARE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_QUERY_FEATURE_COMPARE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaQueryFeatureCompare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssMediaQueryFeatureCompare")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field("range", &support::DebugSyntaxResult(self.range()))
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<CssMediaQueryFeatureCompare> for SyntaxNode {
    fn from(n: CssMediaQueryFeatureCompare) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssMediaQueryFeatureCompare> for SyntaxElement {
    fn from(n: CssMediaQueryFeatureCompare) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssMediaQueryFeaturePlain {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_QUERY_FEATURE_PLAIN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_QUERY_FEATURE_PLAIN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaQueryFeaturePlain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssMediaQueryFeaturePlain")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<CssMediaQueryFeaturePlain> for SyntaxNode {
    fn from(n: CssMediaQueryFeaturePlain) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssMediaQueryFeaturePlain> for SyntaxElement {
    fn from(n: CssMediaQueryFeaturePlain) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssMediaQueryFeatureRange {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_QUERY_FEATURE_RANGE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_QUERY_FEATURE_RANGE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaQueryFeatureRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssMediaQueryFeatureRange")
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
impl From<CssMediaQueryFeatureRange> for SyntaxNode {
    fn from(n: CssMediaQueryFeatureRange) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssMediaQueryFeatureRange> for SyntaxElement {
    fn from(n: CssMediaQueryFeatureRange) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssMediaQueryRange {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_QUERY_RANGE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_QUERY_RANGE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaQueryRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssMediaQueryRange")
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
impl From<CssMediaQueryRange> for SyntaxNode {
    fn from(n: CssMediaQueryRange) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssMediaQueryRange> for SyntaxElement {
    fn from(n: CssMediaQueryRange) -> SyntaxElement {
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
        f.debug_struct("CssNamedNamespacePrefix")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .finish()
    }
}
impl From<CssNamedNamespacePrefix> for SyntaxNode {
    fn from(n: CssNamedNamespacePrefix) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssNamedNamespacePrefix> for SyntaxElement {
    fn from(n: CssNamedNamespacePrefix) -> SyntaxElement {
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
        f.debug_struct("CssNamespace")
            .field("prefix", &support::DebugOptionalElement(self.prefix()))
            .field(
                "bitwise_or_token",
                &support::DebugSyntaxResult(self.bitwise_or_token()),
            )
            .finish()
    }
}
impl From<CssNamespace> for SyntaxNode {
    fn from(n: CssNamespace) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssNamespace> for SyntaxElement {
    fn from(n: CssNamespace) -> SyntaxElement {
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
            .field("css_component_value_list", &self.css_component_value_list())
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
impl AstNode for CssPercentDimension {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PERCENT_DIMENSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PERCENT_DIMENSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPercentDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPercentDimension")
            .field("value", &support::DebugSyntaxResult(self.value()))
            .field("unit_token", &support::DebugSyntaxResult(self.unit_token()))
            .finish()
    }
}
impl From<CssPercentDimension> for SyntaxNode {
    fn from(n: CssPercentDimension) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPercentDimension> for SyntaxElement {
    fn from(n: CssPercentDimension) -> SyntaxElement {
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
            .field("name_token", &support::DebugSyntaxResult(self.name_token()))
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
            .field("name_token", &support::DebugSyntaxResult(self.name_token()))
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
            .field("name_token", &support::DebugSyntaxResult(self.name_token()))
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
                "slash_token",
                &support::DebugSyntaxResult(self.slash_token()),
            )
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
        f.debug_struct("CssRegularDimension")
            .field("value", &support::DebugSyntaxResult(self.value()))
            .field("unit", &support::DebugSyntaxResult(self.unit()))
            .finish()
    }
}
impl From<CssRegularDimension> for SyntaxNode {
    fn from(n: CssRegularDimension) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssRegularDimension> for SyntaxElement {
    fn from(n: CssRegularDimension) -> SyntaxElement {
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
impl AstNode for CssSizeFeatureBoolean {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SIZE_FEATURE_BOOLEAN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SIZE_FEATURE_BOOLEAN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssSizeFeatureBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssSizeFeatureBoolean")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .finish()
    }
}
impl From<CssSizeFeatureBoolean> for SyntaxNode {
    fn from(n: CssSizeFeatureBoolean) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssSizeFeatureBoolean> for SyntaxElement {
    fn from(n: CssSizeFeatureBoolean) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssSizeFeaturePlain {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SIZE_FEATURE_PLAIN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SIZE_FEATURE_PLAIN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssSizeFeaturePlain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssSizeFeaturePlain")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<CssSizeFeaturePlain> for SyntaxNode {
    fn from(n: CssSizeFeaturePlain) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssSizeFeaturePlain> for SyntaxElement {
    fn from(n: CssSizeFeaturePlain) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssSizeFeatureRange {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SIZE_FEATURE_RANGE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SIZE_FEATURE_RANGE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssSizeFeatureRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssSizeFeatureRange")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field("comparison", &support::DebugSyntaxResult(self.comparison()))
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<CssSizeFeatureRange> for SyntaxNode {
    fn from(n: CssSizeFeatureRange) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssSizeFeatureRange> for SyntaxElement {
    fn from(n: CssSizeFeatureRange) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssSizeFeatureRangeComparison {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SIZE_FEATURE_RANGE_COMPARISON as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SIZE_FEATURE_RANGE_COMPARISON
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssSizeFeatureRangeComparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssSizeFeatureRangeComparison")
            .field("operator", &support::DebugSyntaxResult(self.operator()))
            .finish()
    }
}
impl From<CssSizeFeatureRangeComparison> for SyntaxNode {
    fn from(n: CssSizeFeatureRangeComparison) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssSizeFeatureRangeComparison> for SyntaxElement {
    fn from(n: CssSizeFeatureRangeComparison) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssSizeFeatureRangeInterval {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SIZE_FEATURE_RANGE_INTERVAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SIZE_FEATURE_RANGE_INTERVAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssSizeFeatureRangeInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssSizeFeatureRangeInterval")
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
    }
}
impl From<CssSizeFeatureRangeInterval> for SyntaxNode {
    fn from(n: CssSizeFeatureRangeInterval) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssSizeFeatureRangeInterval> for SyntaxElement {
    fn from(n: CssSizeFeatureRangeInterval) -> SyntaxElement {
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
            .field(
                "namespace",
                &support::DebugOptionalElement(self.namespace()),
            )
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
        f.debug_struct("CssUniversalNamespacePrefix")
            .field("star_token", &support::DebugSyntaxResult(self.star_token()))
            .finish()
    }
}
impl From<CssUniversalNamespacePrefix> for SyntaxNode {
    fn from(n: CssUniversalNamespacePrefix) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssUniversalNamespacePrefix> for SyntaxElement {
    fn from(n: CssUniversalNamespacePrefix) -> SyntaxElement {
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
            .field(
                "namespace",
                &support::DebugOptionalElement(self.namespace()),
            )
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
impl From<CssBogusAtRule> for AnyCssAtRule {
    fn from(node: CssBogusAtRule) -> AnyCssAtRule {
        AnyCssAtRule::CssBogusAtRule(node)
    }
}
impl From<CssCharsetAtRule> for AnyCssAtRule {
    fn from(node: CssCharsetAtRule) -> AnyCssAtRule {
        AnyCssAtRule::CssCharsetAtRule(node)
    }
}
impl From<CssColorProfileAtRule> for AnyCssAtRule {
    fn from(node: CssColorProfileAtRule) -> AnyCssAtRule {
        AnyCssAtRule::CssColorProfileAtRule(node)
    }
}
impl From<CssContainerAtRule> for AnyCssAtRule {
    fn from(node: CssContainerAtRule) -> AnyCssAtRule {
        AnyCssAtRule::CssContainerAtRule(node)
    }
}
impl From<CssCounterStyleAtRule> for AnyCssAtRule {
    fn from(node: CssCounterStyleAtRule) -> AnyCssAtRule {
        AnyCssAtRule::CssCounterStyleAtRule(node)
    }
}
impl From<CssFontFaceAtRule> for AnyCssAtRule {
    fn from(node: CssFontFaceAtRule) -> AnyCssAtRule {
        AnyCssAtRule::CssFontFaceAtRule(node)
    }
}
impl From<CssKeyframesAtRule> for AnyCssAtRule {
    fn from(node: CssKeyframesAtRule) -> AnyCssAtRule {
        AnyCssAtRule::CssKeyframesAtRule(node)
    }
}
impl From<CssMediaAtRule> for AnyCssAtRule {
    fn from(node: CssMediaAtRule) -> AnyCssAtRule {
        AnyCssAtRule::CssMediaAtRule(node)
    }
}
impl AstNode for AnyCssAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssBogusAtRule::KIND_SET
        .union(CssCharsetAtRule::KIND_SET)
        .union(CssColorProfileAtRule::KIND_SET)
        .union(CssContainerAtRule::KIND_SET)
        .union(CssCounterStyleAtRule::KIND_SET)
        .union(CssFontFaceAtRule::KIND_SET)
        .union(CssKeyframesAtRule::KIND_SET)
        .union(CssMediaAtRule::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_BOGUS_AT_RULE
                | CSS_CHARSET_AT_RULE
                | CSS_COLOR_PROFILE_AT_RULE
                | CSS_CONTAINER_AT_RULE
                | CSS_COUNTER_STYLE_AT_RULE
                | CSS_FONT_FACE_AT_RULE
                | CSS_KEYFRAMES_AT_RULE
                | CSS_MEDIA_AT_RULE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_AT_RULE => AnyCssAtRule::CssBogusAtRule(CssBogusAtRule { syntax }),
            CSS_CHARSET_AT_RULE => AnyCssAtRule::CssCharsetAtRule(CssCharsetAtRule { syntax }),
            CSS_COLOR_PROFILE_AT_RULE => {
                AnyCssAtRule::CssColorProfileAtRule(CssColorProfileAtRule { syntax })
            }
            CSS_CONTAINER_AT_RULE => {
                AnyCssAtRule::CssContainerAtRule(CssContainerAtRule { syntax })
            }
            CSS_COUNTER_STYLE_AT_RULE => {
                AnyCssAtRule::CssCounterStyleAtRule(CssCounterStyleAtRule { syntax })
            }
            CSS_FONT_FACE_AT_RULE => AnyCssAtRule::CssFontFaceAtRule(CssFontFaceAtRule { syntax }),
            CSS_KEYFRAMES_AT_RULE => {
                AnyCssAtRule::CssKeyframesAtRule(CssKeyframesAtRule { syntax })
            }
            CSS_MEDIA_AT_RULE => AnyCssAtRule::CssMediaAtRule(CssMediaAtRule { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssAtRule::CssBogusAtRule(it) => &it.syntax,
            AnyCssAtRule::CssCharsetAtRule(it) => &it.syntax,
            AnyCssAtRule::CssColorProfileAtRule(it) => &it.syntax,
            AnyCssAtRule::CssContainerAtRule(it) => &it.syntax,
            AnyCssAtRule::CssCounterStyleAtRule(it) => &it.syntax,
            AnyCssAtRule::CssFontFaceAtRule(it) => &it.syntax,
            AnyCssAtRule::CssKeyframesAtRule(it) => &it.syntax,
            AnyCssAtRule::CssMediaAtRule(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssAtRule::CssBogusAtRule(it) => it.syntax,
            AnyCssAtRule::CssCharsetAtRule(it) => it.syntax,
            AnyCssAtRule::CssColorProfileAtRule(it) => it.syntax,
            AnyCssAtRule::CssContainerAtRule(it) => it.syntax,
            AnyCssAtRule::CssCounterStyleAtRule(it) => it.syntax,
            AnyCssAtRule::CssFontFaceAtRule(it) => it.syntax,
            AnyCssAtRule::CssKeyframesAtRule(it) => it.syntax,
            AnyCssAtRule::CssMediaAtRule(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssAtRule::CssBogusAtRule(it) => std::fmt::Debug::fmt(it, f),
            AnyCssAtRule::CssCharsetAtRule(it) => std::fmt::Debug::fmt(it, f),
            AnyCssAtRule::CssColorProfileAtRule(it) => std::fmt::Debug::fmt(it, f),
            AnyCssAtRule::CssContainerAtRule(it) => std::fmt::Debug::fmt(it, f),
            AnyCssAtRule::CssCounterStyleAtRule(it) => std::fmt::Debug::fmt(it, f),
            AnyCssAtRule::CssFontFaceAtRule(it) => std::fmt::Debug::fmt(it, f),
            AnyCssAtRule::CssKeyframesAtRule(it) => std::fmt::Debug::fmt(it, f),
            AnyCssAtRule::CssMediaAtRule(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssAtRule> for SyntaxNode {
    fn from(n: AnyCssAtRule) -> SyntaxNode {
        match n {
            AnyCssAtRule::CssBogusAtRule(it) => it.into(),
            AnyCssAtRule::CssCharsetAtRule(it) => it.into(),
            AnyCssAtRule::CssColorProfileAtRule(it) => it.into(),
            AnyCssAtRule::CssContainerAtRule(it) => it.into(),
            AnyCssAtRule::CssCounterStyleAtRule(it) => it.into(),
            AnyCssAtRule::CssFontFaceAtRule(it) => it.into(),
            AnyCssAtRule::CssKeyframesAtRule(it) => it.into(),
            AnyCssAtRule::CssMediaAtRule(it) => it.into(),
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
impl From<CssContainerAndQuery> for AnyCssContainerCombinableQuery {
    fn from(node: CssContainerAndQuery) -> AnyCssContainerCombinableQuery {
        AnyCssContainerCombinableQuery::CssContainerAndQuery(node)
    }
}
impl From<CssContainerOrQuery> for AnyCssContainerCombinableQuery {
    fn from(node: CssContainerOrQuery) -> AnyCssContainerCombinableQuery {
        AnyCssContainerCombinableQuery::CssContainerOrQuery(node)
    }
}
impl AstNode for AnyCssContainerCombinableQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyCssContainerQueryInParens::KIND_SET
        .union(CssContainerAndQuery::KIND_SET)
        .union(CssContainerOrQuery::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_CONTAINER_AND_QUERY | CSS_CONTAINER_OR_QUERY => true,
            k if AnyCssContainerQueryInParens::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_CONTAINER_AND_QUERY => {
                AnyCssContainerCombinableQuery::CssContainerAndQuery(CssContainerAndQuery {
                    syntax,
                })
            }
            CSS_CONTAINER_OR_QUERY => {
                AnyCssContainerCombinableQuery::CssContainerOrQuery(CssContainerOrQuery { syntax })
            }
            _ => {
                if let Some(any_css_container_query_in_parens) =
                    AnyCssContainerQueryInParens::cast(syntax)
                {
                    return Some(
                        AnyCssContainerCombinableQuery::AnyCssContainerQueryInParens(
                            any_css_container_query_in_parens,
                        ),
                    );
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssContainerCombinableQuery::CssContainerAndQuery(it) => &it.syntax,
            AnyCssContainerCombinableQuery::CssContainerOrQuery(it) => &it.syntax,
            AnyCssContainerCombinableQuery::AnyCssContainerQueryInParens(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssContainerCombinableQuery::CssContainerAndQuery(it) => it.syntax,
            AnyCssContainerCombinableQuery::CssContainerOrQuery(it) => it.syntax,
            AnyCssContainerCombinableQuery::AnyCssContainerQueryInParens(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssContainerCombinableQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssContainerCombinableQuery::AnyCssContainerQueryInParens(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssContainerCombinableQuery::CssContainerAndQuery(it) => std::fmt::Debug::fmt(it, f),
            AnyCssContainerCombinableQuery::CssContainerOrQuery(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssContainerCombinableQuery> for SyntaxNode {
    fn from(n: AnyCssContainerCombinableQuery) -> SyntaxNode {
        match n {
            AnyCssContainerCombinableQuery::AnyCssContainerQueryInParens(it) => it.into(),
            AnyCssContainerCombinableQuery::CssContainerAndQuery(it) => it.into(),
            AnyCssContainerCombinableQuery::CssContainerOrQuery(it) => it.into(),
        }
    }
}
impl From<AnyCssContainerCombinableQuery> for SyntaxElement {
    fn from(n: AnyCssContainerCombinableQuery) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssContainerNotQuery> for AnyCssContainerQuery {
    fn from(node: CssContainerNotQuery) -> AnyCssContainerQuery {
        AnyCssContainerQuery::CssContainerNotQuery(node)
    }
}
impl AstNode for AnyCssContainerQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyCssContainerCombinableQuery::KIND_SET.union(CssContainerNotQuery::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_CONTAINER_NOT_QUERY => true,
            k if AnyCssContainerCombinableQuery::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_CONTAINER_NOT_QUERY => {
                AnyCssContainerQuery::CssContainerNotQuery(CssContainerNotQuery { syntax })
            }
            _ => {
                if let Some(any_css_container_combinable_query) =
                    AnyCssContainerCombinableQuery::cast(syntax)
                {
                    return Some(AnyCssContainerQuery::AnyCssContainerCombinableQuery(
                        any_css_container_combinable_query,
                    ));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssContainerQuery::CssContainerNotQuery(it) => &it.syntax,
            AnyCssContainerQuery::AnyCssContainerCombinableQuery(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssContainerQuery::CssContainerNotQuery(it) => it.syntax,
            AnyCssContainerQuery::AnyCssContainerCombinableQuery(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssContainerQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssContainerQuery::AnyCssContainerCombinableQuery(it) => std::fmt::Debug::fmt(it, f),
            AnyCssContainerQuery::CssContainerNotQuery(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssContainerQuery> for SyntaxNode {
    fn from(n: AnyCssContainerQuery) -> SyntaxNode {
        match n {
            AnyCssContainerQuery::AnyCssContainerCombinableQuery(it) => it.into(),
            AnyCssContainerQuery::CssContainerNotQuery(it) => it.into(),
        }
    }
}
impl From<AnyCssContainerQuery> for SyntaxElement {
    fn from(n: AnyCssContainerQuery) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssContainerQueryInParens> for AnyCssContainerQueryInParens {
    fn from(node: CssContainerQueryInParens) -> AnyCssContainerQueryInParens {
        AnyCssContainerQueryInParens::CssContainerQueryInParens(node)
    }
}
impl From<CssContainerSizeFeatureInParens> for AnyCssContainerQueryInParens {
    fn from(node: CssContainerSizeFeatureInParens) -> AnyCssContainerQueryInParens {
        AnyCssContainerQueryInParens::CssContainerSizeFeatureInParens(node)
    }
}
impl From<CssContainerStyleQueryInParens> for AnyCssContainerQueryInParens {
    fn from(node: CssContainerStyleQueryInParens) -> AnyCssContainerQueryInParens {
        AnyCssContainerQueryInParens::CssContainerStyleQueryInParens(node)
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
                AnyCssContainerQueryInParens::CssContainerQueryInParens(CssContainerQueryInParens {
                    syntax,
                })
            }
            CSS_CONTAINER_SIZE_FEATURE_IN_PARENS => {
                AnyCssContainerQueryInParens::CssContainerSizeFeatureInParens(
                    CssContainerSizeFeatureInParens { syntax },
                )
            }
            CSS_CONTAINER_STYLE_QUERY_IN_PARENS => {
                AnyCssContainerQueryInParens::CssContainerStyleQueryInParens(
                    CssContainerStyleQueryInParens { syntax },
                )
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssContainerQueryInParens::CssContainerQueryInParens(it) => &it.syntax,
            AnyCssContainerQueryInParens::CssContainerSizeFeatureInParens(it) => &it.syntax,
            AnyCssContainerQueryInParens::CssContainerStyleQueryInParens(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssContainerQueryInParens::CssContainerQueryInParens(it) => it.syntax,
            AnyCssContainerQueryInParens::CssContainerSizeFeatureInParens(it) => it.syntax,
            AnyCssContainerQueryInParens::CssContainerStyleQueryInParens(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssContainerQueryInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssContainerQueryInParens::CssContainerQueryInParens(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssContainerQueryInParens::CssContainerSizeFeatureInParens(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssContainerQueryInParens::CssContainerStyleQueryInParens(it) => {
                std::fmt::Debug::fmt(it, f)
            }
        }
    }
}
impl From<AnyCssContainerQueryInParens> for SyntaxNode {
    fn from(n: AnyCssContainerQueryInParens) -> SyntaxNode {
        match n {
            AnyCssContainerQueryInParens::CssContainerQueryInParens(it) => it.into(),
            AnyCssContainerQueryInParens::CssContainerSizeFeatureInParens(it) => it.into(),
            AnyCssContainerQueryInParens::CssContainerStyleQueryInParens(it) => it.into(),
        }
    }
}
impl From<AnyCssContainerQueryInParens> for SyntaxElement {
    fn from(n: AnyCssContainerQueryInParens) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssSizeFeatureBoolean> for AnyCssContainerSizeFeature {
    fn from(node: CssSizeFeatureBoolean) -> AnyCssContainerSizeFeature {
        AnyCssContainerSizeFeature::CssSizeFeatureBoolean(node)
    }
}
impl From<CssSizeFeaturePlain> for AnyCssContainerSizeFeature {
    fn from(node: CssSizeFeaturePlain) -> AnyCssContainerSizeFeature {
        AnyCssContainerSizeFeature::CssSizeFeaturePlain(node)
    }
}
impl From<CssSizeFeatureRange> for AnyCssContainerSizeFeature {
    fn from(node: CssSizeFeatureRange) -> AnyCssContainerSizeFeature {
        AnyCssContainerSizeFeature::CssSizeFeatureRange(node)
    }
}
impl From<CssSizeFeatureRangeInterval> for AnyCssContainerSizeFeature {
    fn from(node: CssSizeFeatureRangeInterval) -> AnyCssContainerSizeFeature {
        AnyCssContainerSizeFeature::CssSizeFeatureRangeInterval(node)
    }
}
impl AstNode for AnyCssContainerSizeFeature {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssSizeFeatureBoolean::KIND_SET
        .union(CssSizeFeaturePlain::KIND_SET)
        .union(CssSizeFeatureRange::KIND_SET)
        .union(CssSizeFeatureRangeInterval::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_SIZE_FEATURE_BOOLEAN
                | CSS_SIZE_FEATURE_PLAIN
                | CSS_SIZE_FEATURE_RANGE
                | CSS_SIZE_FEATURE_RANGE_INTERVAL
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_SIZE_FEATURE_BOOLEAN => {
                AnyCssContainerSizeFeature::CssSizeFeatureBoolean(CssSizeFeatureBoolean { syntax })
            }
            CSS_SIZE_FEATURE_PLAIN => {
                AnyCssContainerSizeFeature::CssSizeFeaturePlain(CssSizeFeaturePlain { syntax })
            }
            CSS_SIZE_FEATURE_RANGE => {
                AnyCssContainerSizeFeature::CssSizeFeatureRange(CssSizeFeatureRange { syntax })
            }
            CSS_SIZE_FEATURE_RANGE_INTERVAL => {
                AnyCssContainerSizeFeature::CssSizeFeatureRangeInterval(
                    CssSizeFeatureRangeInterval { syntax },
                )
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssContainerSizeFeature::CssSizeFeatureBoolean(it) => &it.syntax,
            AnyCssContainerSizeFeature::CssSizeFeaturePlain(it) => &it.syntax,
            AnyCssContainerSizeFeature::CssSizeFeatureRange(it) => &it.syntax,
            AnyCssContainerSizeFeature::CssSizeFeatureRangeInterval(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssContainerSizeFeature::CssSizeFeatureBoolean(it) => it.syntax,
            AnyCssContainerSizeFeature::CssSizeFeaturePlain(it) => it.syntax,
            AnyCssContainerSizeFeature::CssSizeFeatureRange(it) => it.syntax,
            AnyCssContainerSizeFeature::CssSizeFeatureRangeInterval(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssContainerSizeFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssContainerSizeFeature::CssSizeFeatureBoolean(it) => std::fmt::Debug::fmt(it, f),
            AnyCssContainerSizeFeature::CssSizeFeaturePlain(it) => std::fmt::Debug::fmt(it, f),
            AnyCssContainerSizeFeature::CssSizeFeatureRange(it) => std::fmt::Debug::fmt(it, f),
            AnyCssContainerSizeFeature::CssSizeFeatureRangeInterval(it) => {
                std::fmt::Debug::fmt(it, f)
            }
        }
    }
}
impl From<AnyCssContainerSizeFeature> for SyntaxNode {
    fn from(n: AnyCssContainerSizeFeature) -> SyntaxNode {
        match n {
            AnyCssContainerSizeFeature::CssSizeFeatureBoolean(it) => it.into(),
            AnyCssContainerSizeFeature::CssSizeFeaturePlain(it) => it.into(),
            AnyCssContainerSizeFeature::CssSizeFeatureRange(it) => it.into(),
            AnyCssContainerSizeFeature::CssSizeFeatureRangeInterval(it) => it.into(),
        }
    }
}
impl From<AnyCssContainerSizeFeature> for SyntaxElement {
    fn from(n: AnyCssContainerSizeFeature) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssContainerStyleAndQuery> for AnyCssContainerStyleCombinableQuery {
    fn from(node: CssContainerStyleAndQuery) -> AnyCssContainerStyleCombinableQuery {
        AnyCssContainerStyleCombinableQuery::CssContainerStyleAndQuery(node)
    }
}
impl From<CssContainerStyleInParens> for AnyCssContainerStyleCombinableQuery {
    fn from(node: CssContainerStyleInParens) -> AnyCssContainerStyleCombinableQuery {
        AnyCssContainerStyleCombinableQuery::CssContainerStyleInParens(node)
    }
}
impl From<CssContainerStyleOrQuery> for AnyCssContainerStyleCombinableQuery {
    fn from(node: CssContainerStyleOrQuery) -> AnyCssContainerStyleCombinableQuery {
        AnyCssContainerStyleCombinableQuery::CssContainerStyleOrQuery(node)
    }
}
impl AstNode for AnyCssContainerStyleCombinableQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssContainerStyleAndQuery::KIND_SET
        .union(CssContainerStyleInParens::KIND_SET)
        .union(CssContainerStyleOrQuery::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_CONTAINER_STYLE_AND_QUERY
                | CSS_CONTAINER_STYLE_IN_PARENS
                | CSS_CONTAINER_STYLE_OR_QUERY
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_CONTAINER_STYLE_AND_QUERY => {
                AnyCssContainerStyleCombinableQuery::CssContainerStyleAndQuery(
                    CssContainerStyleAndQuery { syntax },
                )
            }
            CSS_CONTAINER_STYLE_IN_PARENS => {
                AnyCssContainerStyleCombinableQuery::CssContainerStyleInParens(
                    CssContainerStyleInParens { syntax },
                )
            }
            CSS_CONTAINER_STYLE_OR_QUERY => {
                AnyCssContainerStyleCombinableQuery::CssContainerStyleOrQuery(
                    CssContainerStyleOrQuery { syntax },
                )
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssContainerStyleCombinableQuery::CssContainerStyleAndQuery(it) => &it.syntax,
            AnyCssContainerStyleCombinableQuery::CssContainerStyleInParens(it) => &it.syntax,
            AnyCssContainerStyleCombinableQuery::CssContainerStyleOrQuery(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssContainerStyleCombinableQuery::CssContainerStyleAndQuery(it) => it.syntax,
            AnyCssContainerStyleCombinableQuery::CssContainerStyleInParens(it) => it.syntax,
            AnyCssContainerStyleCombinableQuery::CssContainerStyleOrQuery(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssContainerStyleCombinableQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssContainerStyleCombinableQuery::CssContainerStyleAndQuery(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssContainerStyleCombinableQuery::CssContainerStyleInParens(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssContainerStyleCombinableQuery::CssContainerStyleOrQuery(it) => {
                std::fmt::Debug::fmt(it, f)
            }
        }
    }
}
impl From<AnyCssContainerStyleCombinableQuery> for SyntaxNode {
    fn from(n: AnyCssContainerStyleCombinableQuery) -> SyntaxNode {
        match n {
            AnyCssContainerStyleCombinableQuery::CssContainerStyleAndQuery(it) => it.into(),
            AnyCssContainerStyleCombinableQuery::CssContainerStyleInParens(it) => it.into(),
            AnyCssContainerStyleCombinableQuery::CssContainerStyleOrQuery(it) => it.into(),
        }
    }
}
impl From<AnyCssContainerStyleCombinableQuery> for SyntaxElement {
    fn from(n: AnyCssContainerStyleCombinableQuery) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssDeclaration> for AnyCssContainerStyleInParens {
    fn from(node: CssDeclaration) -> AnyCssContainerStyleInParens {
        AnyCssContainerStyleInParens::CssDeclaration(node)
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
            CSS_DECLARATION => {
                AnyCssContainerStyleInParens::CssDeclaration(CssDeclaration { syntax })
            }
            _ => {
                if let Some(any_css_container_style_query) = AnyCssContainerStyleQuery::cast(syntax)
                {
                    return Some(AnyCssContainerStyleInParens::AnyCssContainerStyleQuery(
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
            AnyCssContainerStyleInParens::CssDeclaration(it) => &it.syntax,
            AnyCssContainerStyleInParens::AnyCssContainerStyleQuery(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssContainerStyleInParens::CssDeclaration(it) => it.syntax,
            AnyCssContainerStyleInParens::AnyCssContainerStyleQuery(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssContainerStyleInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssContainerStyleInParens::AnyCssContainerStyleQuery(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssContainerStyleInParens::CssDeclaration(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssContainerStyleInParens> for SyntaxNode {
    fn from(n: AnyCssContainerStyleInParens) -> SyntaxNode {
        match n {
            AnyCssContainerStyleInParens::AnyCssContainerStyleQuery(it) => it.into(),
            AnyCssContainerStyleInParens::CssDeclaration(it) => it.into(),
        }
    }
}
impl From<AnyCssContainerStyleInParens> for SyntaxElement {
    fn from(n: AnyCssContainerStyleInParens) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssContainerStyleNotQuery> for AnyCssContainerStyleQuery {
    fn from(node: CssContainerStyleNotQuery) -> AnyCssContainerStyleQuery {
        AnyCssContainerStyleQuery::CssContainerStyleNotQuery(node)
    }
}
impl From<CssDeclaration> for AnyCssContainerStyleQuery {
    fn from(node: CssDeclaration) -> AnyCssContainerStyleQuery {
        AnyCssContainerStyleQuery::CssDeclaration(node)
    }
}
impl AstNode for AnyCssContainerStyleQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyCssContainerStyleCombinableQuery::KIND_SET
        .union(CssContainerStyleNotQuery::KIND_SET)
        .union(CssDeclaration::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_CONTAINER_STYLE_NOT_QUERY | CSS_DECLARATION => true,
            k if AnyCssContainerStyleCombinableQuery::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_CONTAINER_STYLE_NOT_QUERY => {
                AnyCssContainerStyleQuery::CssContainerStyleNotQuery(CssContainerStyleNotQuery {
                    syntax,
                })
            }
            CSS_DECLARATION => AnyCssContainerStyleQuery::CssDeclaration(CssDeclaration { syntax }),
            _ => {
                if let Some(any_css_container_style_combinable_query) =
                    AnyCssContainerStyleCombinableQuery::cast(syntax)
                {
                    return Some(
                        AnyCssContainerStyleQuery::AnyCssContainerStyleCombinableQuery(
                            any_css_container_style_combinable_query,
                        ),
                    );
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssContainerStyleQuery::CssContainerStyleNotQuery(it) => &it.syntax,
            AnyCssContainerStyleQuery::CssDeclaration(it) => &it.syntax,
            AnyCssContainerStyleQuery::AnyCssContainerStyleCombinableQuery(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssContainerStyleQuery::CssContainerStyleNotQuery(it) => it.syntax,
            AnyCssContainerStyleQuery::CssDeclaration(it) => it.syntax,
            AnyCssContainerStyleQuery::AnyCssContainerStyleCombinableQuery(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssContainerStyleQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssContainerStyleQuery::AnyCssContainerStyleCombinableQuery(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssContainerStyleQuery::CssContainerStyleNotQuery(it) => std::fmt::Debug::fmt(it, f),
            AnyCssContainerStyleQuery::CssDeclaration(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssContainerStyleQuery> for SyntaxNode {
    fn from(n: AnyCssContainerStyleQuery) -> SyntaxNode {
        match n {
            AnyCssContainerStyleQuery::AnyCssContainerStyleCombinableQuery(it) => it.into(),
            AnyCssContainerStyleQuery::CssContainerStyleNotQuery(it) => it.into(),
            AnyCssContainerStyleQuery::CssDeclaration(it) => it.into(),
        }
    }
}
impl From<AnyCssContainerStyleQuery> for SyntaxElement {
    fn from(n: AnyCssContainerStyleQuery) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssCustomProperty> for AnyCssDeclarationName {
    fn from(node: CssCustomProperty) -> AnyCssDeclarationName {
        AnyCssDeclarationName::CssCustomProperty(node)
    }
}
impl From<CssIdentifier> for AnyCssDeclarationName {
    fn from(node: CssIdentifier) -> AnyCssDeclarationName {
        AnyCssDeclarationName::CssIdentifier(node)
    }
}
impl AstNode for AnyCssDeclarationName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssCustomProperty::KIND_SET.union(CssIdentifier::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_CUSTOM_PROPERTY | CSS_IDENTIFIER)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_CUSTOM_PROPERTY => {
                AnyCssDeclarationName::CssCustomProperty(CssCustomProperty { syntax })
            }
            CSS_IDENTIFIER => AnyCssDeclarationName::CssIdentifier(CssIdentifier { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssDeclarationName::CssCustomProperty(it) => &it.syntax,
            AnyCssDeclarationName::CssIdentifier(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssDeclarationName::CssCustomProperty(it) => it.syntax,
            AnyCssDeclarationName::CssIdentifier(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssDeclarationName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssDeclarationName::CssCustomProperty(it) => std::fmt::Debug::fmt(it, f),
            AnyCssDeclarationName::CssIdentifier(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssDeclarationName> for SyntaxNode {
    fn from(n: AnyCssDeclarationName) -> SyntaxNode {
        match n {
            AnyCssDeclarationName::CssCustomProperty(it) => it.into(),
            AnyCssDeclarationName::CssIdentifier(it) => it.into(),
        }
    }
}
impl From<AnyCssDeclarationName> for SyntaxElement {
    fn from(n: AnyCssDeclarationName) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssPercentage> for AnyCssDimension {
    fn from(node: CssPercentage) -> AnyCssDimension {
        AnyCssDimension::CssPercentage(node)
    }
}
impl From<CssRegularDimension> for AnyCssDimension {
    fn from(node: CssRegularDimension) -> AnyCssDimension {
        AnyCssDimension::CssRegularDimension(node)
    }
}
impl AstNode for AnyCssDimension {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssPercentage::KIND_SET.union(CssRegularDimension::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_PERCENTAGE | CSS_REGULAR_DIMENSION)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_PERCENTAGE => AnyCssDimension::CssPercentage(CssPercentage { syntax }),
            CSS_REGULAR_DIMENSION => {
                AnyCssDimension::CssRegularDimension(CssRegularDimension { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssDimension::CssPercentage(it) => &it.syntax,
            AnyCssDimension::CssRegularDimension(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssDimension::CssPercentage(it) => it.syntax,
            AnyCssDimension::CssRegularDimension(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssDimension::CssPercentage(it) => std::fmt::Debug::fmt(it, f),
            AnyCssDimension::CssRegularDimension(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssDimension> for SyntaxNode {
    fn from(n: AnyCssDimension) -> SyntaxNode {
        match n {
            AnyCssDimension::CssPercentage(it) => it.into(),
            AnyCssDimension::CssRegularDimension(it) => it.into(),
        }
    }
}
impl From<AnyCssDimension> for SyntaxElement {
    fn from(n: AnyCssDimension) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssMediaQueryFeatureBoolean> for AnyCssMediaQueryFeatureType {
    fn from(node: CssMediaQueryFeatureBoolean) -> AnyCssMediaQueryFeatureType {
        AnyCssMediaQueryFeatureType::CssMediaQueryFeatureBoolean(node)
    }
}
impl From<CssMediaQueryFeatureCompare> for AnyCssMediaQueryFeatureType {
    fn from(node: CssMediaQueryFeatureCompare) -> AnyCssMediaQueryFeatureType {
        AnyCssMediaQueryFeatureType::CssMediaQueryFeatureCompare(node)
    }
}
impl From<CssMediaQueryFeaturePlain> for AnyCssMediaQueryFeatureType {
    fn from(node: CssMediaQueryFeaturePlain) -> AnyCssMediaQueryFeatureType {
        AnyCssMediaQueryFeatureType::CssMediaQueryFeaturePlain(node)
    }
}
impl From<CssMediaQueryFeatureRange> for AnyCssMediaQueryFeatureType {
    fn from(node: CssMediaQueryFeatureRange) -> AnyCssMediaQueryFeatureType {
        AnyCssMediaQueryFeatureType::CssMediaQueryFeatureRange(node)
    }
}
impl AstNode for AnyCssMediaQueryFeatureType {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssMediaQueryFeatureBoolean::KIND_SET
        .union(CssMediaQueryFeatureCompare::KIND_SET)
        .union(CssMediaQueryFeaturePlain::KIND_SET)
        .union(CssMediaQueryFeatureRange::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_MEDIA_QUERY_FEATURE_BOOLEAN
                | CSS_MEDIA_QUERY_FEATURE_COMPARE
                | CSS_MEDIA_QUERY_FEATURE_PLAIN
                | CSS_MEDIA_QUERY_FEATURE_RANGE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_MEDIA_QUERY_FEATURE_BOOLEAN => {
                AnyCssMediaQueryFeatureType::CssMediaQueryFeatureBoolean(
                    CssMediaQueryFeatureBoolean { syntax },
                )
            }
            CSS_MEDIA_QUERY_FEATURE_COMPARE => {
                AnyCssMediaQueryFeatureType::CssMediaQueryFeatureCompare(
                    CssMediaQueryFeatureCompare { syntax },
                )
            }
            CSS_MEDIA_QUERY_FEATURE_PLAIN => {
                AnyCssMediaQueryFeatureType::CssMediaQueryFeaturePlain(CssMediaQueryFeaturePlain {
                    syntax,
                })
            }
            CSS_MEDIA_QUERY_FEATURE_RANGE => {
                AnyCssMediaQueryFeatureType::CssMediaQueryFeatureRange(CssMediaQueryFeatureRange {
                    syntax,
                })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssMediaQueryFeatureType::CssMediaQueryFeatureBoolean(it) => &it.syntax,
            AnyCssMediaQueryFeatureType::CssMediaQueryFeatureCompare(it) => &it.syntax,
            AnyCssMediaQueryFeatureType::CssMediaQueryFeaturePlain(it) => &it.syntax,
            AnyCssMediaQueryFeatureType::CssMediaQueryFeatureRange(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssMediaQueryFeatureType::CssMediaQueryFeatureBoolean(it) => it.syntax,
            AnyCssMediaQueryFeatureType::CssMediaQueryFeatureCompare(it) => it.syntax,
            AnyCssMediaQueryFeatureType::CssMediaQueryFeaturePlain(it) => it.syntax,
            AnyCssMediaQueryFeatureType::CssMediaQueryFeatureRange(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssMediaQueryFeatureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssMediaQueryFeatureType::CssMediaQueryFeatureBoolean(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssMediaQueryFeatureType::CssMediaQueryFeatureCompare(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssMediaQueryFeatureType::CssMediaQueryFeaturePlain(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssMediaQueryFeatureType::CssMediaQueryFeatureRange(it) => {
                std::fmt::Debug::fmt(it, f)
            }
        }
    }
}
impl From<AnyCssMediaQueryFeatureType> for SyntaxNode {
    fn from(n: AnyCssMediaQueryFeatureType) -> SyntaxNode {
        match n {
            AnyCssMediaQueryFeatureType::CssMediaQueryFeatureBoolean(it) => it.into(),
            AnyCssMediaQueryFeatureType::CssMediaQueryFeatureCompare(it) => it.into(),
            AnyCssMediaQueryFeatureType::CssMediaQueryFeaturePlain(it) => it.into(),
            AnyCssMediaQueryFeatureType::CssMediaQueryFeatureRange(it) => it.into(),
        }
    }
}
impl From<AnyCssMediaQueryFeatureType> for SyntaxElement {
    fn from(n: AnyCssMediaQueryFeatureType) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssIdentifier> for AnyCssMediaQueryType {
    fn from(node: CssIdentifier) -> AnyCssMediaQueryType {
        AnyCssMediaQueryType::CssIdentifier(node)
    }
}
impl From<CssMediaQueryFeature> for AnyCssMediaQueryType {
    fn from(node: CssMediaQueryFeature) -> AnyCssMediaQueryType {
        AnyCssMediaQueryType::CssMediaQueryFeature(node)
    }
}
impl AstNode for AnyCssMediaQueryType {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssIdentifier::KIND_SET.union(CssMediaQueryFeature::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_IDENTIFIER | CSS_MEDIA_QUERY_FEATURE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_IDENTIFIER => AnyCssMediaQueryType::CssIdentifier(CssIdentifier { syntax }),
            CSS_MEDIA_QUERY_FEATURE => {
                AnyCssMediaQueryType::CssMediaQueryFeature(CssMediaQueryFeature { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssMediaQueryType::CssIdentifier(it) => &it.syntax,
            AnyCssMediaQueryType::CssMediaQueryFeature(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssMediaQueryType::CssIdentifier(it) => it.syntax,
            AnyCssMediaQueryType::CssMediaQueryFeature(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssMediaQueryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssMediaQueryType::CssIdentifier(it) => std::fmt::Debug::fmt(it, f),
            AnyCssMediaQueryType::CssMediaQueryFeature(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssMediaQueryType> for SyntaxNode {
    fn from(n: AnyCssMediaQueryType) -> SyntaxNode {
        match n {
            AnyCssMediaQueryType::CssIdentifier(it) => it.into(),
            AnyCssMediaQueryType::CssMediaQueryFeature(it) => it.into(),
        }
    }
}
impl From<AnyCssMediaQueryType> for SyntaxElement {
    fn from(n: AnyCssMediaQueryType) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssNamedNamespacePrefix> for AnyCssNamespacePrefix {
    fn from(node: CssNamedNamespacePrefix) -> AnyCssNamespacePrefix {
        AnyCssNamespacePrefix::CssNamedNamespacePrefix(node)
    }
}
impl From<CssUniversalNamespacePrefix> for AnyCssNamespacePrefix {
    fn from(node: CssUniversalNamespacePrefix) -> AnyCssNamespacePrefix {
        AnyCssNamespacePrefix::CssUniversalNamespacePrefix(node)
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
                AnyCssNamespacePrefix::CssNamedNamespacePrefix(CssNamedNamespacePrefix { syntax })
            }
            CSS_UNIVERSAL_NAMESPACE_PREFIX => {
                AnyCssNamespacePrefix::CssUniversalNamespacePrefix(CssUniversalNamespacePrefix {
                    syntax,
                })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssNamespacePrefix::CssNamedNamespacePrefix(it) => &it.syntax,
            AnyCssNamespacePrefix::CssUniversalNamespacePrefix(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssNamespacePrefix::CssNamedNamespacePrefix(it) => it.syntax,
            AnyCssNamespacePrefix::CssUniversalNamespacePrefix(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssNamespacePrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssNamespacePrefix::CssNamedNamespacePrefix(it) => std::fmt::Debug::fmt(it, f),
            AnyCssNamespacePrefix::CssUniversalNamespacePrefix(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssNamespacePrefix> for SyntaxNode {
    fn from(n: AnyCssNamespacePrefix) -> SyntaxNode {
        match n {
            AnyCssNamespacePrefix::CssNamedNamespacePrefix(it) => it.into(),
            AnyCssNamespacePrefix::CssUniversalNamespacePrefix(it) => it.into(),
        }
    }
}
impl From<AnyCssNamespacePrefix> for SyntaxElement {
    fn from(n: AnyCssNamespacePrefix) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusPseudoClass> for AnyCssPseudoClass {
    fn from(node: CssBogusPseudoClass) -> AnyCssPseudoClass {
        AnyCssPseudoClass::CssBogusPseudoClass(node)
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
    const KIND_SET: SyntaxKindSet<Language> = CssBogusPseudoClass::KIND_SET
        .union(CssPseudoClassFunctionCompoundSelector::KIND_SET)
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
            CSS_BOGUS_PSEUDO_CLASS
                | CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR
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
            CSS_BOGUS_PSEUDO_CLASS => {
                AnyCssPseudoClass::CssBogusPseudoClass(CssBogusPseudoClass { syntax })
            }
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
            AnyCssPseudoClass::CssBogusPseudoClass(it) => &it.syntax,
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
            AnyCssPseudoClass::CssBogusPseudoClass(it) => it.syntax,
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
            AnyCssPseudoClass::CssBogusPseudoClass(it) => std::fmt::Debug::fmt(it, f),
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
            AnyCssPseudoClass::CssBogusPseudoClass(it) => it.into(),
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
impl From<CssBogusPseudoElement> for AnyCssPseudoElement {
    fn from(node: CssBogusPseudoElement) -> AnyCssPseudoElement {
        AnyCssPseudoElement::CssBogusPseudoElement(node)
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
    const KIND_SET: SyntaxKindSet<Language> = CssBogusPseudoElement::KIND_SET
        .union(CssPseudoElementFunctionIdentifier::KIND_SET)
        .union(CssPseudoElementFunctionSelector::KIND_SET)
        .union(CssPseudoElementIdentifier::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_BOGUS_PSEUDO_ELEMENT
                | CSS_PSEUDO_ELEMENT_FUNCTION_IDENTIFIER
                | CSS_PSEUDO_ELEMENT_FUNCTION_SELECTOR
                | CSS_PSEUDO_ELEMENT_IDENTIFIER
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_PSEUDO_ELEMENT => {
                AnyCssPseudoElement::CssBogusPseudoElement(CssBogusPseudoElement { syntax })
            }
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
            AnyCssPseudoElement::CssBogusPseudoElement(it) => &it.syntax,
            AnyCssPseudoElement::CssPseudoElementFunctionIdentifier(it) => &it.syntax,
            AnyCssPseudoElement::CssPseudoElementFunctionSelector(it) => &it.syntax,
            AnyCssPseudoElement::CssPseudoElementIdentifier(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssPseudoElement::CssBogusPseudoElement(it) => it.syntax,
            AnyCssPseudoElement::CssPseudoElementFunctionIdentifier(it) => it.syntax,
            AnyCssPseudoElement::CssPseudoElementFunctionSelector(it) => it.syntax,
            AnyCssPseudoElement::CssPseudoElementIdentifier(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssPseudoElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssPseudoElement::CssBogusPseudoElement(it) => std::fmt::Debug::fmt(it, f),
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
            AnyCssPseudoElement::CssBogusPseudoElement(it) => it.into(),
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
impl From<CssAtRule> for AnyCssRule {
    fn from(node: CssAtRule) -> AnyCssRule {
        AnyCssRule::CssAtRule(node)
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
    const KIND_SET: SyntaxKindSet<Language> = CssAtRule::KIND_SET
        .union(CssBogusRule::KIND_SET)
        .union(CssRule::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_AT_RULE | CSS_BOGUS_RULE | CSS_RULE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_AT_RULE => AnyCssRule::CssAtRule(CssAtRule { syntax }),
            CSS_BOGUS_RULE => AnyCssRule::CssBogusRule(CssBogusRule { syntax }),
            CSS_RULE => AnyCssRule::CssRule(CssRule { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssRule::CssAtRule(it) => &it.syntax,
            AnyCssRule::CssBogusRule(it) => &it.syntax,
            AnyCssRule::CssRule(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssRule::CssAtRule(it) => it.syntax,
            AnyCssRule::CssBogusRule(it) => it.syntax,
            AnyCssRule::CssRule(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssRule::CssAtRule(it) => std::fmt::Debug::fmt(it, f),
            AnyCssRule::CssBogusRule(it) => std::fmt::Debug::fmt(it, f),
            AnyCssRule::CssRule(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssRule> for SyntaxNode {
    fn from(n: AnyCssRule) -> SyntaxNode {
        match n {
            AnyCssRule::CssAtRule(it) => it.into(),
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
impl From<CssTypeSelector> for AnyCssSimpleSelector {
    fn from(node: CssTypeSelector) -> AnyCssSimpleSelector {
        AnyCssSimpleSelector::CssTypeSelector(node)
    }
}
impl From<CssUniversalSelector> for AnyCssSimpleSelector {
    fn from(node: CssUniversalSelector) -> AnyCssSimpleSelector {
        AnyCssSimpleSelector::CssUniversalSelector(node)
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
            CSS_TYPE_SELECTOR => AnyCssSimpleSelector::CssTypeSelector(CssTypeSelector { syntax }),
            CSS_UNIVERSAL_SELECTOR => {
                AnyCssSimpleSelector::CssUniversalSelector(CssUniversalSelector { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssSimpleSelector::CssTypeSelector(it) => &it.syntax,
            AnyCssSimpleSelector::CssUniversalSelector(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssSimpleSelector::CssTypeSelector(it) => it.syntax,
            AnyCssSimpleSelector::CssUniversalSelector(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssSimpleSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssSimpleSelector::CssTypeSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssSimpleSelector::CssUniversalSelector(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssSimpleSelector> for SyntaxNode {
    fn from(n: AnyCssSimpleSelector) -> SyntaxNode {
        match n {
            AnyCssSimpleSelector::CssTypeSelector(it) => it.into(),
            AnyCssSimpleSelector::CssUniversalSelector(it) => it.into(),
        }
    }
}
impl From<AnyCssSimpleSelector> for SyntaxElement {
    fn from(n: AnyCssSimpleSelector) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssAnyFunction> for AnyCssSizeFeatureValue {
    fn from(node: CssAnyFunction) -> AnyCssSizeFeatureValue {
        AnyCssSizeFeatureValue::CssAnyFunction(node)
    }
}
impl From<CssIdentifier> for AnyCssSizeFeatureValue {
    fn from(node: CssIdentifier) -> AnyCssSizeFeatureValue {
        AnyCssSizeFeatureValue::CssIdentifier(node)
    }
}
impl From<CssNumber> for AnyCssSizeFeatureValue {
    fn from(node: CssNumber) -> AnyCssSizeFeatureValue {
        AnyCssSizeFeatureValue::CssNumber(node)
    }
}
impl From<CssRatio> for AnyCssSizeFeatureValue {
    fn from(node: CssRatio) -> AnyCssSizeFeatureValue {
        AnyCssSizeFeatureValue::CssRatio(node)
    }
}
impl AstNode for AnyCssSizeFeatureValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyCssDimension::KIND_SET
        .union(CssAnyFunction::KIND_SET)
        .union(CssIdentifier::KIND_SET)
        .union(CssNumber::KIND_SET)
        .union(CssRatio::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_ANY_FUNCTION | CSS_IDENTIFIER | CSS_NUMBER | CSS_RATIO => true,
            k if AnyCssDimension::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_ANY_FUNCTION => AnyCssSizeFeatureValue::CssAnyFunction(CssAnyFunction { syntax }),
            CSS_IDENTIFIER => AnyCssSizeFeatureValue::CssIdentifier(CssIdentifier { syntax }),
            CSS_NUMBER => AnyCssSizeFeatureValue::CssNumber(CssNumber { syntax }),
            CSS_RATIO => AnyCssSizeFeatureValue::CssRatio(CssRatio { syntax }),
            _ => {
                if let Some(any_css_dimension) = AnyCssDimension::cast(syntax) {
                    return Some(AnyCssSizeFeatureValue::AnyCssDimension(any_css_dimension));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssSizeFeatureValue::CssAnyFunction(it) => &it.syntax,
            AnyCssSizeFeatureValue::CssIdentifier(it) => &it.syntax,
            AnyCssSizeFeatureValue::CssNumber(it) => &it.syntax,
            AnyCssSizeFeatureValue::CssRatio(it) => &it.syntax,
            AnyCssSizeFeatureValue::AnyCssDimension(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssSizeFeatureValue::CssAnyFunction(it) => it.syntax,
            AnyCssSizeFeatureValue::CssIdentifier(it) => it.syntax,
            AnyCssSizeFeatureValue::CssNumber(it) => it.syntax,
            AnyCssSizeFeatureValue::CssRatio(it) => it.syntax,
            AnyCssSizeFeatureValue::AnyCssDimension(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssSizeFeatureValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssSizeFeatureValue::AnyCssDimension(it) => std::fmt::Debug::fmt(it, f),
            AnyCssSizeFeatureValue::CssAnyFunction(it) => std::fmt::Debug::fmt(it, f),
            AnyCssSizeFeatureValue::CssIdentifier(it) => std::fmt::Debug::fmt(it, f),
            AnyCssSizeFeatureValue::CssNumber(it) => std::fmt::Debug::fmt(it, f),
            AnyCssSizeFeatureValue::CssRatio(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssSizeFeatureValue> for SyntaxNode {
    fn from(n: AnyCssSizeFeatureValue) -> SyntaxNode {
        match n {
            AnyCssSizeFeatureValue::AnyCssDimension(it) => it.into(),
            AnyCssSizeFeatureValue::CssAnyFunction(it) => it.into(),
            AnyCssSizeFeatureValue::CssIdentifier(it) => it.into(),
            AnyCssSizeFeatureValue::CssNumber(it) => it.into(),
            AnyCssSizeFeatureValue::CssRatio(it) => it.into(),
        }
    }
}
impl From<AnyCssSizeFeatureValue> for SyntaxElement {
    fn from(n: AnyCssSizeFeatureValue) -> SyntaxElement {
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
    const KIND_SET: SyntaxKindSet<Language> = AnyCssDimension::KIND_SET
        .union(CssAnyFunction::KIND_SET)
        .union(CssCustomProperty::KIND_SET)
        .union(CssIdentifier::KIND_SET)
        .union(CssNumber::KIND_SET)
        .union(CssRatio::KIND_SET)
        .union(CssString::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_ANY_FUNCTION | CSS_CUSTOM_PROPERTY | CSS_IDENTIFIER | CSS_NUMBER | CSS_RATIO
            | CSS_STRING => true,
            k if AnyCssDimension::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_ANY_FUNCTION => AnyCssValue::CssAnyFunction(CssAnyFunction { syntax }),
            CSS_CUSTOM_PROPERTY => AnyCssValue::CssCustomProperty(CssCustomProperty { syntax }),
            CSS_IDENTIFIER => AnyCssValue::CssIdentifier(CssIdentifier { syntax }),
            CSS_NUMBER => AnyCssValue::CssNumber(CssNumber { syntax }),
            CSS_RATIO => AnyCssValue::CssRatio(CssRatio { syntax }),
            CSS_STRING => AnyCssValue::CssString(CssString { syntax }),
            _ => {
                if let Some(any_css_dimension) = AnyCssDimension::cast(syntax) {
                    return Some(AnyCssValue::AnyCssDimension(any_css_dimension));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssValue::CssAnyFunction(it) => &it.syntax,
            AnyCssValue::CssCustomProperty(it) => &it.syntax,
            AnyCssValue::CssIdentifier(it) => &it.syntax,
            AnyCssValue::CssNumber(it) => &it.syntax,
            AnyCssValue::CssRatio(it) => &it.syntax,
            AnyCssValue::CssString(it) => &it.syntax,
            AnyCssValue::AnyCssDimension(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssValue::CssAnyFunction(it) => it.syntax,
            AnyCssValue::CssCustomProperty(it) => it.syntax,
            AnyCssValue::CssIdentifier(it) => it.syntax,
            AnyCssValue::CssNumber(it) => it.syntax,
            AnyCssValue::CssRatio(it) => it.syntax,
            AnyCssValue::CssString(it) => it.syntax,
            AnyCssValue::AnyCssDimension(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssValue::AnyCssDimension(it) => std::fmt::Debug::fmt(it, f),
            AnyCssValue::CssAnyFunction(it) => std::fmt::Debug::fmt(it, f),
            AnyCssValue::CssCustomProperty(it) => std::fmt::Debug::fmt(it, f),
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
            AnyCssValue::AnyCssDimension(it) => it.into(),
            AnyCssValue::CssAnyFunction(it) => it.into(),
            AnyCssValue::CssCustomProperty(it) => it.into(),
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
impl std::fmt::Display for AnyCssContainerCombinableQuery {
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
impl std::fmt::Display for AnyCssContainerSizeFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssContainerStyleCombinableQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssContainerStyleInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssContainerStyleQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssDeclarationName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssMediaQueryFeatureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssMediaQueryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssNamespacePrefix {
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
impl std::fmt::Display for AnyCssSimpleSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssSizeFeatureValue {
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
impl std::fmt::Display for CssAnyFunction {
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
impl std::fmt::Display for CssBlock {
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
impl std::fmt::Display for CssFontFaceAtRule {
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
impl std::fmt::Display for CssKeyframesBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssKeyframesSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaQueryConsequent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaQueryFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaQueryFeatureBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaQueryFeatureCompare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaQueryFeaturePlain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaQueryFeatureRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaQueryRange {
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
impl std::fmt::Display for CssPercentDimension {
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
impl std::fmt::Display for CssSizeFeatureBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssSizeFeaturePlain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssSizeFeatureRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssSizeFeatureRangeComparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssSizeFeatureRangeInterval {
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
    fn from(n: CssBogusAtRule) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssBogusAtRule> for SyntaxElement {
    fn from(n: CssBogusAtRule) -> SyntaxElement {
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
    fn from(n: CssBogusPseudoClass) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssBogusPseudoClass> for SyntaxElement {
    fn from(n: CssBogusPseudoClass) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
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
    fn from(n: CssBogusPseudoElement) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssBogusPseudoElement> for SyntaxElement {
    fn from(n: CssBogusPseudoElement) -> SyntaxElement {
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
    fn cast(syntax: SyntaxNode) -> Option<CssComponentValueList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssComponentValueList {
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
impl AstSeparatedList for CssDeclarationList {
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
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssDeclarationList {
    type Item = SyntaxResult<CssDeclaration>;
    type IntoIter = AstSeparatedListNodesIterator<Language, CssDeclaration>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssDeclarationList {
    type Item = SyntaxResult<CssDeclaration>;
    type IntoIter = AstSeparatedListNodesIterator<Language, CssDeclaration>;
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
    fn cast(syntax: SyntaxNode) -> Option<CssKeyframesItemList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssKeyframesItemList {
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
    type Node = CssKeyframesBlock;
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
    type Item = CssKeyframesBlock;
    type IntoIter = AstNodeListIterator<Language, CssKeyframesBlock>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssKeyframesItemList {
    type Item = CssKeyframesBlock;
    type IntoIter = AstNodeListIterator<Language, CssKeyframesBlock>;
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
    fn cast(syntax: SyntaxNode) -> Option<CssMediaQueryList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssMediaQueryList {
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
    type Node = CssMediaQuery;
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
    type Item = SyntaxResult<CssMediaQuery>;
    type IntoIter = AstSeparatedListNodesIterator<Language, CssMediaQuery>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssMediaQueryList {
    type Item = SyntaxResult<CssMediaQuery>;
    type IntoIter = AstSeparatedListNodesIterator<Language, CssMediaQuery>;
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
