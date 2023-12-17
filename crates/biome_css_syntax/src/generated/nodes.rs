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
pub struct CssAtKeyword {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtKeyword {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssAtKeywordFields {
        CssAtKeywordFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAtKeyword {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAtKeywordFields {
    pub value_token: SyntaxResult<SyntaxToken>,
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
            name: self.name(),
            prelude: self.prelude(),
            css_at_rule_content: self.css_at_rule_content(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssAtKeyword> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn prelude(&self) -> CssAtRuleComponentValue {
        support::list(&self.syntax, 1usize)
    }
    pub fn css_at_rule_content(&self) -> SyntaxResult<CssAtRuleContent> {
        support::required_node(&self.syntax, 2usize)
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
    pub name: SyntaxResult<CssAtKeyword>,
    pub prelude: CssAtRuleComponentValue,
    pub css_at_rule_content: SyntaxResult<CssAtRuleContent>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAtRuleSemicolon {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtRuleSemicolon {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssAtRuleSemicolonFields {
        CssAtRuleSemicolonFields {
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn semicolon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAtRuleSemicolon {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAtRuleSemicolonFields {
    pub semicolon_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssBlockDeclarationList {
    pub(crate) syntax: SyntaxNode,
}
impl CssBlockDeclarationList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssBlockDeclarationListFields {
        CssBlockDeclarationListFields {
            css_declaration_list: self.css_declaration_list(),
        }
    }
    pub fn css_declaration_list(&self) -> CssDeclarationList {
        support::list(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssBlockDeclarationList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssBlockDeclarationListFields {
    pub css_declaration_list: CssDeclarationList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssCurlyBracketsBlock {
    pub(crate) syntax: SyntaxNode,
}
impl CssCurlyBracketsBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssCurlyBracketsBlockFields {
        CssCurlyBracketsBlockFields {
            l_curly_token: self.l_curly_token(),
            content: self.content(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn content(&self) -> CssCurlyBracketsBlockContent {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssCurlyBracketsBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssCurlyBracketsBlockFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub content: CssCurlyBracketsBlockContent,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
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
            valie: self.valie(),
            important: self.important(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn valie(&self) -> CssComponentValueList {
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
    pub name: SyntaxResult<CssIdentifier>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub valie: CssComponentValueList,
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
pub struct CssDelim {
    pub(crate) syntax: SyntaxNode,
}
impl CssDelim {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssDelimFields {
        CssDelimFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssDelim {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssDelimFields {
    pub value_token: SyntaxResult<SyntaxToken>,
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
pub struct CssFunctionBlock {
    pub(crate) syntax: SyntaxNode,
}
impl CssFunctionBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssFunctionBlockFields {
        CssFunctionBlockFields {
            css_function_token: self.css_function_token(),
            css_component_value_list: self.css_component_value_list(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn css_function_token(&self) -> SyntaxResult<CssFunctionToken> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn css_component_value_list(&self) -> CssComponentValueList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssFunctionBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssFunctionBlockFields {
    pub css_function_token: SyntaxResult<CssFunctionToken>,
    pub css_component_value_list: CssComponentValueList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssFunctionToken {
    pub(crate) syntax: SyntaxNode,
}
impl CssFunctionToken {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssFunctionTokenFields {
        CssFunctionTokenFields {
            valye_token: self.valye_token(),
        }
    }
    pub fn valye_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssFunctionToken {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssFunctionTokenFields {
    pub valye_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssHash {
    pub(crate) syntax: SyntaxNode,
}
impl CssHash {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssHashFields {
        CssHashFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssHashFields {
    pub value_token: SyntaxResult<SyntaxToken>,
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
pub struct CssPreservedTokenKey {
    pub(crate) syntax: SyntaxNode,
}
impl CssPreservedTokenKey {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPreservedTokenKeyFields {
        CssPreservedTokenKeyFields {
            value: self.value(),
        }
    }
    pub fn value(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPreservedTokenKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPreservedTokenKeyFields {
    pub value: SyntaxResult<SyntaxToken>,
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
    pub fn prelude(&self) -> CssQualifiedRulePrelude {
        support::list(&self.syntax, 0usize)
    }
    pub fn block(&self) -> SyntaxResult<CssCurlyBracketsBlock> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssQualifiedRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssQualifiedRuleFields {
    pub prelude: CssQualifiedRulePrelude,
    pub block: SyntaxResult<CssCurlyBracketsBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssSimpleCurlyBracketsBlock {
    pub(crate) syntax: SyntaxNode,
}
impl CssSimpleCurlyBracketsBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssSimpleCurlyBracketsBlockFields {
        CssSimpleCurlyBracketsBlockFields {
            l_curly_token: self.l_curly_token(),
            content: self.content(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn content(&self) -> CssSimpleComponentValueList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssSimpleCurlyBracketsBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssSimpleCurlyBracketsBlockFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub content: CssSimpleComponentValueList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssSimpleParenthesesBlock {
    pub(crate) syntax: SyntaxNode,
}
impl CssSimpleParenthesesBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssSimpleParenthesesBlockFields {
        CssSimpleParenthesesBlockFields {
            l_paren_token: self.l_paren_token(),
            content: self.content(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn content(&self) -> CssSimpleComponentValueList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssSimpleParenthesesBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssSimpleParenthesesBlockFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub content: CssSimpleComponentValueList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssSimpleSquareBracketsBlock {
    pub(crate) syntax: SyntaxNode,
}
impl CssSimpleSquareBracketsBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssSimpleSquareBracketsBlockFields {
        CssSimpleSquareBracketsBlockFields {
            l_brack_token: self.l_brack_token(),
            content: self.content(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn content(&self) -> CssSimpleComponentValueList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssSimpleSquareBracketsBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssSimpleSquareBracketsBlockFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub content: CssSimpleComponentValueList,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
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
pub struct CssStyleSheet {
    pub(crate) syntax: SyntaxNode,
}
impl CssStyleSheet {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssStyleSheetFields {
        CssStyleSheetFields {
            bom_token: self.bom_token(),
            content: self.content(),
            eof_token: self.eof_token(),
        }
    }
    pub fn bom_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn content(&self) -> CssStyleSheetContent {
        support::list(&self.syntax, 1usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssStyleSheet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssStyleSheetFields {
    pub bom_token: Option<SyntaxToken>,
    pub content: CssStyleSheetContent,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssCurlyBracketsBlockContent {
    CssAtRule(CssAtRule),
    CssBlockDeclarationList(CssBlockDeclarationList),
    CssQualifiedRule(CssQualifiedRule),
}
impl AnyCssCurlyBracketsBlockContent {
    pub fn as_css_at_rule(&self) -> Option<&CssAtRule> {
        match &self {
            AnyCssCurlyBracketsBlockContent::CssAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_block_declaration_list(&self) -> Option<&CssBlockDeclarationList> {
        match &self {
            AnyCssCurlyBracketsBlockContent::CssBlockDeclarationList(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_qualified_rule(&self) -> Option<&CssQualifiedRule> {
        match &self {
            AnyCssCurlyBracketsBlockContent::CssQualifiedRule(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssStylesheetContent {
    CssAtRule(CssAtRule),
    CssQualifiedRule(CssQualifiedRule),
}
impl AnyCssStylesheetContent {
    pub fn as_css_at_rule(&self) -> Option<&CssAtRule> {
        match &self {
            AnyCssStylesheetContent::CssAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_qualified_rule(&self) -> Option<&CssQualifiedRule> {
        match &self {
            AnyCssStylesheetContent::CssQualifiedRule(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum CssAtRuleContent {
    CssAtRuleSemicolon(CssAtRuleSemicolon),
    CssCurlyBracketsBlock(CssCurlyBracketsBlock),
}
impl CssAtRuleContent {
    pub fn as_css_at_rule_semicolon(&self) -> Option<&CssAtRuleSemicolon> {
        match &self {
            CssAtRuleContent::CssAtRuleSemicolon(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_curly_brackets_block(&self) -> Option<&CssCurlyBracketsBlock> {
        match &self {
            CssAtRuleContent::CssCurlyBracketsBlock(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum CssComponentValue {
    CssFunctionBlock(CssFunctionBlock),
    CssPreservedToken(CssPreservedToken),
    CssSimpleBlock(CssSimpleBlock),
}
impl CssComponentValue {
    pub fn as_css_function_block(&self) -> Option<&CssFunctionBlock> {
        match &self {
            CssComponentValue::CssFunctionBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_preserved_token(&self) -> Option<&CssPreservedToken> {
        match &self {
            CssComponentValue::CssPreservedToken(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_simple_block(&self) -> Option<&CssSimpleBlock> {
        match &self {
            CssComponentValue::CssSimpleBlock(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum CssPreservedToken {
    CssAtKeyword(CssAtKeyword),
    CssDelim(CssDelim),
    CssHash(CssHash),
    CssIdentifier(CssIdentifier),
    CssNumber(CssNumber),
    CssPercentage(CssPercentage),
    CssPreservedTokenKey(CssPreservedTokenKey),
    CssString(CssString),
}
impl CssPreservedToken {
    pub fn as_css_at_keyword(&self) -> Option<&CssAtKeyword> {
        match &self {
            CssPreservedToken::CssAtKeyword(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_delim(&self) -> Option<&CssDelim> {
        match &self {
            CssPreservedToken::CssDelim(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_hash(&self) -> Option<&CssHash> {
        match &self {
            CssPreservedToken::CssHash(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_identifier(&self) -> Option<&CssIdentifier> {
        match &self {
            CssPreservedToken::CssIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_number(&self) -> Option<&CssNumber> {
        match &self {
            CssPreservedToken::CssNumber(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_percentage(&self) -> Option<&CssPercentage> {
        match &self {
            CssPreservedToken::CssPercentage(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_preserved_token_key(&self) -> Option<&CssPreservedTokenKey> {
        match &self {
            CssPreservedToken::CssPreservedTokenKey(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_string(&self) -> Option<&CssString> {
        match &self {
            CssPreservedToken::CssString(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum CssSimpleBlock {
    CssSimpleCurlyBracketsBlock(CssSimpleCurlyBracketsBlock),
    CssSimpleParenthesesBlock(CssSimpleParenthesesBlock),
    CssSimpleSquareBracketsBlock(CssSimpleSquareBracketsBlock),
}
impl CssSimpleBlock {
    pub fn as_css_simple_curly_brackets_block(&self) -> Option<&CssSimpleCurlyBracketsBlock> {
        match &self {
            CssSimpleBlock::CssSimpleCurlyBracketsBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_simple_parentheses_block(&self) -> Option<&CssSimpleParenthesesBlock> {
        match &self {
            CssSimpleBlock::CssSimpleParenthesesBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_simple_square_brackets_block(&self) -> Option<&CssSimpleSquareBracketsBlock> {
        match &self {
            CssSimpleBlock::CssSimpleSquareBracketsBlock(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for CssAtKeyword {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_KEYWORD as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_AT_KEYWORD
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssAtKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAtKeyword")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<CssAtKeyword> for SyntaxNode {
    fn from(n: CssAtKeyword) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssAtKeyword> for SyntaxElement {
    fn from(n: CssAtKeyword) -> SyntaxElement {
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
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field("prelude", &self.prelude())
            .field(
                "css_at_rule_content",
                &support::DebugSyntaxResult(self.css_at_rule_content()),
            )
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
impl AstNode for CssAtRuleSemicolon {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_RULE_SEMICOLON as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_AT_RULE_SEMICOLON
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssAtRuleSemicolon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAtRuleSemicolon")
            .field(
                "semicolon_token",
                &support::DebugSyntaxResult(self.semicolon_token()),
            )
            .finish()
    }
}
impl From<CssAtRuleSemicolon> for SyntaxNode {
    fn from(n: CssAtRuleSemicolon) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssAtRuleSemicolon> for SyntaxElement {
    fn from(n: CssAtRuleSemicolon) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssBlockDeclarationList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BLOCK_DECLARATION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BLOCK_DECLARATION_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBlockDeclarationList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBlockDeclarationList")
            .field("css_declaration_list", &self.css_declaration_list())
            .finish()
    }
}
impl From<CssBlockDeclarationList> for SyntaxNode {
    fn from(n: CssBlockDeclarationList) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssBlockDeclarationList> for SyntaxElement {
    fn from(n: CssBlockDeclarationList) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssCurlyBracketsBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CURLY_BRACKETS_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CURLY_BRACKETS_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssCurlyBracketsBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssCurlyBracketsBlock")
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("content", &self.content())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<CssCurlyBracketsBlock> for SyntaxNode {
    fn from(n: CssCurlyBracketsBlock) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssCurlyBracketsBlock> for SyntaxElement {
    fn from(n: CssCurlyBracketsBlock) -> SyntaxElement {
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
            .field("valie", &self.valie())
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
impl AstNode for CssDelim {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_DELIM as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_DELIM
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssDelim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssDelim")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<CssDelim> for SyntaxNode {
    fn from(n: CssDelim) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssDelim> for SyntaxElement {
    fn from(n: CssDelim) -> SyntaxElement {
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
impl AstNode for CssFunctionBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_FUNCTION_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_FUNCTION_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssFunctionBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssFunctionBlock")
            .field(
                "css_function_token",
                &support::DebugSyntaxResult(self.css_function_token()),
            )
            .field("css_component_value_list", &self.css_component_value_list())
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssFunctionBlock> for SyntaxNode {
    fn from(n: CssFunctionBlock) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssFunctionBlock> for SyntaxElement {
    fn from(n: CssFunctionBlock) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssFunctionToken {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_FUNCTION_TOKEN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_FUNCTION_TOKEN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssFunctionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssFunctionToken")
            .field(
                "valye_token",
                &support::DebugSyntaxResult(self.valye_token()),
            )
            .finish()
    }
}
impl From<CssFunctionToken> for SyntaxNode {
    fn from(n: CssFunctionToken) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssFunctionToken> for SyntaxElement {
    fn from(n: CssFunctionToken) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssHash {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_HASH as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_HASH
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssHash")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<CssHash> for SyntaxNode {
    fn from(n: CssHash) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssHash> for SyntaxElement {
    fn from(n: CssHash) -> SyntaxElement {
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
impl AstNode for CssPreservedTokenKey {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PRESERVED_TOKEN_KEY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PRESERVED_TOKEN_KEY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPreservedTokenKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPreservedTokenKey")
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<CssPreservedTokenKey> for SyntaxNode {
    fn from(n: CssPreservedTokenKey) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPreservedTokenKey> for SyntaxElement {
    fn from(n: CssPreservedTokenKey) -> SyntaxElement {
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
        f.debug_struct("CssQualifiedRule")
            .field("prelude", &self.prelude())
            .field("block", &support::DebugSyntaxResult(self.block()))
            .finish()
    }
}
impl From<CssQualifiedRule> for SyntaxNode {
    fn from(n: CssQualifiedRule) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssQualifiedRule> for SyntaxElement {
    fn from(n: CssQualifiedRule) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssSimpleCurlyBracketsBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SIMPLE_CURLY_BRACKETS_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SIMPLE_CURLY_BRACKETS_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssSimpleCurlyBracketsBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssSimpleCurlyBracketsBlock")
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("content", &self.content())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<CssSimpleCurlyBracketsBlock> for SyntaxNode {
    fn from(n: CssSimpleCurlyBracketsBlock) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssSimpleCurlyBracketsBlock> for SyntaxElement {
    fn from(n: CssSimpleCurlyBracketsBlock) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssSimpleParenthesesBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SIMPLE_PARENTHESES_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SIMPLE_PARENTHESES_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssSimpleParenthesesBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssSimpleParenthesesBlock")
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("content", &self.content())
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssSimpleParenthesesBlock> for SyntaxNode {
    fn from(n: CssSimpleParenthesesBlock) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssSimpleParenthesesBlock> for SyntaxElement {
    fn from(n: CssSimpleParenthesesBlock) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssSimpleSquareBracketsBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SIMPLE_SQUARE_BRACKETS_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SIMPLE_SQUARE_BRACKETS_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssSimpleSquareBracketsBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssSimpleSquareBracketsBlock")
            .field(
                "l_brack_token",
                &support::DebugSyntaxResult(self.l_brack_token()),
            )
            .field("content", &self.content())
            .field(
                "r_brack_token",
                &support::DebugSyntaxResult(self.r_brack_token()),
            )
            .finish()
    }
}
impl From<CssSimpleSquareBracketsBlock> for SyntaxNode {
    fn from(n: CssSimpleSquareBracketsBlock) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssSimpleSquareBracketsBlock> for SyntaxElement {
    fn from(n: CssSimpleSquareBracketsBlock) -> SyntaxElement {
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
impl AstNode for CssStyleSheet {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_STYLE_SHEET as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_STYLE_SHEET
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssStyleSheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssStyleSheet")
            .field(
                "bom_token",
                &support::DebugOptionalElement(self.bom_token()),
            )
            .field("content", &self.content())
            .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
            .finish()
    }
}
impl From<CssStyleSheet> for SyntaxNode {
    fn from(n: CssStyleSheet) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssStyleSheet> for SyntaxElement {
    fn from(n: CssStyleSheet) -> SyntaxElement {
        n.syntax.into()
    }
}
impl From<CssAtRule> for AnyCssCurlyBracketsBlockContent {
    fn from(node: CssAtRule) -> AnyCssCurlyBracketsBlockContent {
        AnyCssCurlyBracketsBlockContent::CssAtRule(node)
    }
}
impl From<CssBlockDeclarationList> for AnyCssCurlyBracketsBlockContent {
    fn from(node: CssBlockDeclarationList) -> AnyCssCurlyBracketsBlockContent {
        AnyCssCurlyBracketsBlockContent::CssBlockDeclarationList(node)
    }
}
impl From<CssQualifiedRule> for AnyCssCurlyBracketsBlockContent {
    fn from(node: CssQualifiedRule) -> AnyCssCurlyBracketsBlockContent {
        AnyCssCurlyBracketsBlockContent::CssQualifiedRule(node)
    }
}
impl AstNode for AnyCssCurlyBracketsBlockContent {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssAtRule::KIND_SET
        .union(CssBlockDeclarationList::KIND_SET)
        .union(CssQualifiedRule::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_AT_RULE | CSS_BLOCK_DECLARATION_LIST | CSS_QUALIFIED_RULE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_AT_RULE => AnyCssCurlyBracketsBlockContent::CssAtRule(CssAtRule { syntax }),
            CSS_BLOCK_DECLARATION_LIST => {
                AnyCssCurlyBracketsBlockContent::CssBlockDeclarationList(CssBlockDeclarationList {
                    syntax,
                })
            }
            CSS_QUALIFIED_RULE => {
                AnyCssCurlyBracketsBlockContent::CssQualifiedRule(CssQualifiedRule { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssCurlyBracketsBlockContent::CssAtRule(it) => &it.syntax,
            AnyCssCurlyBracketsBlockContent::CssBlockDeclarationList(it) => &it.syntax,
            AnyCssCurlyBracketsBlockContent::CssQualifiedRule(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssCurlyBracketsBlockContent::CssAtRule(it) => it.syntax,
            AnyCssCurlyBracketsBlockContent::CssBlockDeclarationList(it) => it.syntax,
            AnyCssCurlyBracketsBlockContent::CssQualifiedRule(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssCurlyBracketsBlockContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssCurlyBracketsBlockContent::CssAtRule(it) => std::fmt::Debug::fmt(it, f),
            AnyCssCurlyBracketsBlockContent::CssBlockDeclarationList(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssCurlyBracketsBlockContent::CssQualifiedRule(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssCurlyBracketsBlockContent> for SyntaxNode {
    fn from(n: AnyCssCurlyBracketsBlockContent) -> SyntaxNode {
        match n {
            AnyCssCurlyBracketsBlockContent::CssAtRule(it) => it.into(),
            AnyCssCurlyBracketsBlockContent::CssBlockDeclarationList(it) => it.into(),
            AnyCssCurlyBracketsBlockContent::CssQualifiedRule(it) => it.into(),
        }
    }
}
impl From<AnyCssCurlyBracketsBlockContent> for SyntaxElement {
    fn from(n: AnyCssCurlyBracketsBlockContent) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssAtRule> for AnyCssStylesheetContent {
    fn from(node: CssAtRule) -> AnyCssStylesheetContent {
        AnyCssStylesheetContent::CssAtRule(node)
    }
}
impl From<CssQualifiedRule> for AnyCssStylesheetContent {
    fn from(node: CssQualifiedRule) -> AnyCssStylesheetContent {
        AnyCssStylesheetContent::CssQualifiedRule(node)
    }
}
impl AstNode for AnyCssStylesheetContent {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssAtRule::KIND_SET.union(CssQualifiedRule::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_AT_RULE | CSS_QUALIFIED_RULE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_AT_RULE => AnyCssStylesheetContent::CssAtRule(CssAtRule { syntax }),
            CSS_QUALIFIED_RULE => {
                AnyCssStylesheetContent::CssQualifiedRule(CssQualifiedRule { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssStylesheetContent::CssAtRule(it) => &it.syntax,
            AnyCssStylesheetContent::CssQualifiedRule(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssStylesheetContent::CssAtRule(it) => it.syntax,
            AnyCssStylesheetContent::CssQualifiedRule(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssStylesheetContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssStylesheetContent::CssAtRule(it) => std::fmt::Debug::fmt(it, f),
            AnyCssStylesheetContent::CssQualifiedRule(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssStylesheetContent> for SyntaxNode {
    fn from(n: AnyCssStylesheetContent) -> SyntaxNode {
        match n {
            AnyCssStylesheetContent::CssAtRule(it) => it.into(),
            AnyCssStylesheetContent::CssQualifiedRule(it) => it.into(),
        }
    }
}
impl From<AnyCssStylesheetContent> for SyntaxElement {
    fn from(n: AnyCssStylesheetContent) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssAtRuleSemicolon> for CssAtRuleContent {
    fn from(node: CssAtRuleSemicolon) -> CssAtRuleContent {
        CssAtRuleContent::CssAtRuleSemicolon(node)
    }
}
impl From<CssCurlyBracketsBlock> for CssAtRuleContent {
    fn from(node: CssCurlyBracketsBlock) -> CssAtRuleContent {
        CssAtRuleContent::CssCurlyBracketsBlock(node)
    }
}
impl AstNode for CssAtRuleContent {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssAtRuleSemicolon::KIND_SET.union(CssCurlyBracketsBlock::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_AT_RULE_SEMICOLON | CSS_CURLY_BRACKETS_BLOCK)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_AT_RULE_SEMICOLON => {
                CssAtRuleContent::CssAtRuleSemicolon(CssAtRuleSemicolon { syntax })
            }
            CSS_CURLY_BRACKETS_BLOCK => {
                CssAtRuleContent::CssCurlyBracketsBlock(CssCurlyBracketsBlock { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            CssAtRuleContent::CssAtRuleSemicolon(it) => &it.syntax,
            CssAtRuleContent::CssCurlyBracketsBlock(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            CssAtRuleContent::CssAtRuleSemicolon(it) => it.syntax,
            CssAtRuleContent::CssCurlyBracketsBlock(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for CssAtRuleContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CssAtRuleContent::CssAtRuleSemicolon(it) => std::fmt::Debug::fmt(it, f),
            CssAtRuleContent::CssCurlyBracketsBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<CssAtRuleContent> for SyntaxNode {
    fn from(n: CssAtRuleContent) -> SyntaxNode {
        match n {
            CssAtRuleContent::CssAtRuleSemicolon(it) => it.into(),
            CssAtRuleContent::CssCurlyBracketsBlock(it) => it.into(),
        }
    }
}
impl From<CssAtRuleContent> for SyntaxElement {
    fn from(n: CssAtRuleContent) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssFunctionBlock> for CssComponentValue {
    fn from(node: CssFunctionBlock) -> CssComponentValue {
        CssComponentValue::CssFunctionBlock(node)
    }
}
impl AstNode for CssComponentValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssFunctionBlock::KIND_SET
        .union(CssPreservedToken::KIND_SET)
        .union(CssSimpleBlock::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_FUNCTION_BLOCK => true,
            k if CssPreservedToken::can_cast(k) => true,
            k if CssSimpleBlock::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_FUNCTION_BLOCK => CssComponentValue::CssFunctionBlock(CssFunctionBlock { syntax }),
            _ => {
                if let Some(css_preserved_token) = CssPreservedToken::cast(syntax.clone()) {
                    return Some(CssComponentValue::CssPreservedToken(css_preserved_token));
                }
                if let Some(css_simple_block) = CssSimpleBlock::cast(syntax) {
                    return Some(CssComponentValue::CssSimpleBlock(css_simple_block));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            CssComponentValue::CssFunctionBlock(it) => &it.syntax,
            CssComponentValue::CssPreservedToken(it) => it.syntax(),
            CssComponentValue::CssSimpleBlock(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            CssComponentValue::CssFunctionBlock(it) => it.syntax,
            CssComponentValue::CssPreservedToken(it) => it.into_syntax(),
            CssComponentValue::CssSimpleBlock(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for CssComponentValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CssComponentValue::CssFunctionBlock(it) => std::fmt::Debug::fmt(it, f),
            CssComponentValue::CssPreservedToken(it) => std::fmt::Debug::fmt(it, f),
            CssComponentValue::CssSimpleBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<CssComponentValue> for SyntaxNode {
    fn from(n: CssComponentValue) -> SyntaxNode {
        match n {
            CssComponentValue::CssFunctionBlock(it) => it.into(),
            CssComponentValue::CssPreservedToken(it) => it.into(),
            CssComponentValue::CssSimpleBlock(it) => it.into(),
        }
    }
}
impl From<CssComponentValue> for SyntaxElement {
    fn from(n: CssComponentValue) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssAtKeyword> for CssPreservedToken {
    fn from(node: CssAtKeyword) -> CssPreservedToken {
        CssPreservedToken::CssAtKeyword(node)
    }
}
impl From<CssDelim> for CssPreservedToken {
    fn from(node: CssDelim) -> CssPreservedToken {
        CssPreservedToken::CssDelim(node)
    }
}
impl From<CssHash> for CssPreservedToken {
    fn from(node: CssHash) -> CssPreservedToken {
        CssPreservedToken::CssHash(node)
    }
}
impl From<CssIdentifier> for CssPreservedToken {
    fn from(node: CssIdentifier) -> CssPreservedToken {
        CssPreservedToken::CssIdentifier(node)
    }
}
impl From<CssNumber> for CssPreservedToken {
    fn from(node: CssNumber) -> CssPreservedToken {
        CssPreservedToken::CssNumber(node)
    }
}
impl From<CssPercentage> for CssPreservedToken {
    fn from(node: CssPercentage) -> CssPreservedToken {
        CssPreservedToken::CssPercentage(node)
    }
}
impl From<CssPreservedTokenKey> for CssPreservedToken {
    fn from(node: CssPreservedTokenKey) -> CssPreservedToken {
        CssPreservedToken::CssPreservedTokenKey(node)
    }
}
impl From<CssString> for CssPreservedToken {
    fn from(node: CssString) -> CssPreservedToken {
        CssPreservedToken::CssString(node)
    }
}
impl AstNode for CssPreservedToken {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssAtKeyword::KIND_SET
        .union(CssDelim::KIND_SET)
        .union(CssHash::KIND_SET)
        .union(CssIdentifier::KIND_SET)
        .union(CssNumber::KIND_SET)
        .union(CssPercentage::KIND_SET)
        .union(CssPreservedTokenKey::KIND_SET)
        .union(CssString::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_AT_KEYWORD
                | CSS_DELIM
                | CSS_HASH
                | CSS_IDENTIFIER
                | CSS_NUMBER
                | CSS_PERCENTAGE
                | CSS_PRESERVED_TOKEN_KEY
                | CSS_STRING
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_AT_KEYWORD => CssPreservedToken::CssAtKeyword(CssAtKeyword { syntax }),
            CSS_DELIM => CssPreservedToken::CssDelim(CssDelim { syntax }),
            CSS_HASH => CssPreservedToken::CssHash(CssHash { syntax }),
            CSS_IDENTIFIER => CssPreservedToken::CssIdentifier(CssIdentifier { syntax }),
            CSS_NUMBER => CssPreservedToken::CssNumber(CssNumber { syntax }),
            CSS_PERCENTAGE => CssPreservedToken::CssPercentage(CssPercentage { syntax }),
            CSS_PRESERVED_TOKEN_KEY => {
                CssPreservedToken::CssPreservedTokenKey(CssPreservedTokenKey { syntax })
            }
            CSS_STRING => CssPreservedToken::CssString(CssString { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            CssPreservedToken::CssAtKeyword(it) => &it.syntax,
            CssPreservedToken::CssDelim(it) => &it.syntax,
            CssPreservedToken::CssHash(it) => &it.syntax,
            CssPreservedToken::CssIdentifier(it) => &it.syntax,
            CssPreservedToken::CssNumber(it) => &it.syntax,
            CssPreservedToken::CssPercentage(it) => &it.syntax,
            CssPreservedToken::CssPreservedTokenKey(it) => &it.syntax,
            CssPreservedToken::CssString(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            CssPreservedToken::CssAtKeyword(it) => it.syntax,
            CssPreservedToken::CssDelim(it) => it.syntax,
            CssPreservedToken::CssHash(it) => it.syntax,
            CssPreservedToken::CssIdentifier(it) => it.syntax,
            CssPreservedToken::CssNumber(it) => it.syntax,
            CssPreservedToken::CssPercentage(it) => it.syntax,
            CssPreservedToken::CssPreservedTokenKey(it) => it.syntax,
            CssPreservedToken::CssString(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for CssPreservedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CssPreservedToken::CssAtKeyword(it) => std::fmt::Debug::fmt(it, f),
            CssPreservedToken::CssDelim(it) => std::fmt::Debug::fmt(it, f),
            CssPreservedToken::CssHash(it) => std::fmt::Debug::fmt(it, f),
            CssPreservedToken::CssIdentifier(it) => std::fmt::Debug::fmt(it, f),
            CssPreservedToken::CssNumber(it) => std::fmt::Debug::fmt(it, f),
            CssPreservedToken::CssPercentage(it) => std::fmt::Debug::fmt(it, f),
            CssPreservedToken::CssPreservedTokenKey(it) => std::fmt::Debug::fmt(it, f),
            CssPreservedToken::CssString(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<CssPreservedToken> for SyntaxNode {
    fn from(n: CssPreservedToken) -> SyntaxNode {
        match n {
            CssPreservedToken::CssAtKeyword(it) => it.into(),
            CssPreservedToken::CssDelim(it) => it.into(),
            CssPreservedToken::CssHash(it) => it.into(),
            CssPreservedToken::CssIdentifier(it) => it.into(),
            CssPreservedToken::CssNumber(it) => it.into(),
            CssPreservedToken::CssPercentage(it) => it.into(),
            CssPreservedToken::CssPreservedTokenKey(it) => it.into(),
            CssPreservedToken::CssString(it) => it.into(),
        }
    }
}
impl From<CssPreservedToken> for SyntaxElement {
    fn from(n: CssPreservedToken) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssSimpleCurlyBracketsBlock> for CssSimpleBlock {
    fn from(node: CssSimpleCurlyBracketsBlock) -> CssSimpleBlock {
        CssSimpleBlock::CssSimpleCurlyBracketsBlock(node)
    }
}
impl From<CssSimpleParenthesesBlock> for CssSimpleBlock {
    fn from(node: CssSimpleParenthesesBlock) -> CssSimpleBlock {
        CssSimpleBlock::CssSimpleParenthesesBlock(node)
    }
}
impl From<CssSimpleSquareBracketsBlock> for CssSimpleBlock {
    fn from(node: CssSimpleSquareBracketsBlock) -> CssSimpleBlock {
        CssSimpleBlock::CssSimpleSquareBracketsBlock(node)
    }
}
impl AstNode for CssSimpleBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssSimpleCurlyBracketsBlock::KIND_SET
        .union(CssSimpleParenthesesBlock::KIND_SET)
        .union(CssSimpleSquareBracketsBlock::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_SIMPLE_CURLY_BRACKETS_BLOCK
                | CSS_SIMPLE_PARENTHESES_BLOCK
                | CSS_SIMPLE_SQUARE_BRACKETS_BLOCK
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_SIMPLE_CURLY_BRACKETS_BLOCK => {
                CssSimpleBlock::CssSimpleCurlyBracketsBlock(CssSimpleCurlyBracketsBlock { syntax })
            }
            CSS_SIMPLE_PARENTHESES_BLOCK => {
                CssSimpleBlock::CssSimpleParenthesesBlock(CssSimpleParenthesesBlock { syntax })
            }
            CSS_SIMPLE_SQUARE_BRACKETS_BLOCK => {
                CssSimpleBlock::CssSimpleSquareBracketsBlock(CssSimpleSquareBracketsBlock {
                    syntax,
                })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            CssSimpleBlock::CssSimpleCurlyBracketsBlock(it) => &it.syntax,
            CssSimpleBlock::CssSimpleParenthesesBlock(it) => &it.syntax,
            CssSimpleBlock::CssSimpleSquareBracketsBlock(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            CssSimpleBlock::CssSimpleCurlyBracketsBlock(it) => it.syntax,
            CssSimpleBlock::CssSimpleParenthesesBlock(it) => it.syntax,
            CssSimpleBlock::CssSimpleSquareBracketsBlock(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for CssSimpleBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CssSimpleBlock::CssSimpleCurlyBracketsBlock(it) => std::fmt::Debug::fmt(it, f),
            CssSimpleBlock::CssSimpleParenthesesBlock(it) => std::fmt::Debug::fmt(it, f),
            CssSimpleBlock::CssSimpleSquareBracketsBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<CssSimpleBlock> for SyntaxNode {
    fn from(n: CssSimpleBlock) -> SyntaxNode {
        match n {
            CssSimpleBlock::CssSimpleCurlyBracketsBlock(it) => it.into(),
            CssSimpleBlock::CssSimpleParenthesesBlock(it) => it.into(),
            CssSimpleBlock::CssSimpleSquareBracketsBlock(it) => it.into(),
        }
    }
}
impl From<CssSimpleBlock> for SyntaxElement {
    fn from(n: CssSimpleBlock) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyCssCurlyBracketsBlockContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssStylesheetContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtRuleContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssComponentValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPreservedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssSimpleBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtRuleSemicolon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssBlockDeclarationList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssCurlyBracketsBlock {
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
impl std::fmt::Display for CssDelim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssFunctionBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssFunctionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPercentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPreservedTokenKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssQualifiedRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssSimpleCurlyBracketsBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssSimpleParenthesesBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssSimpleSquareBracketsBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssStyleSheet {
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
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssAtRuleComponentValue {
    syntax_list: SyntaxList,
}
impl CssAtRuleComponentValue {
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
impl AstNode for CssAtRuleComponentValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_RULE_COMPONENT_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_AT_RULE_COMPONENT_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<CssAtRuleComponentValue> {
        if Self::can_cast(syntax.kind()) {
            Some(CssAtRuleComponentValue {
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
impl Serialize for CssAtRuleComponentValue {
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
impl AstNodeList for CssAtRuleComponentValue {
    type Language = Language;
    type Node = CssComponentValue;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssAtRuleComponentValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssAtRuleComponentValue ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssAtRuleComponentValue {
    type Item = CssComponentValue;
    type IntoIter = AstNodeListIterator<Language, CssComponentValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssAtRuleComponentValue {
    type Item = CssComponentValue;
    type IntoIter = AstNodeListIterator<Language, CssComponentValue>;
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
    type Node = CssComponentValue;
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
    type Item = CssComponentValue;
    type IntoIter = AstNodeListIterator<Language, CssComponentValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssComponentValueList {
    type Item = CssComponentValue;
    type IntoIter = AstNodeListIterator<Language, CssComponentValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssCurlyBracketsBlockContent {
    syntax_list: SyntaxList,
}
impl CssCurlyBracketsBlockContent {
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
impl AstNode for CssCurlyBracketsBlockContent {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CURLY_BRACKETS_BLOCK_CONTENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CURLY_BRACKETS_BLOCK_CONTENT
    }
    fn cast(syntax: SyntaxNode) -> Option<CssCurlyBracketsBlockContent> {
        if Self::can_cast(syntax.kind()) {
            Some(CssCurlyBracketsBlockContent {
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
impl Serialize for CssCurlyBracketsBlockContent {
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
impl AstNodeList for CssCurlyBracketsBlockContent {
    type Language = Language;
    type Node = AnyCssCurlyBracketsBlockContent;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssCurlyBracketsBlockContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssCurlyBracketsBlockContent ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssCurlyBracketsBlockContent {
    type Item = AnyCssCurlyBracketsBlockContent;
    type IntoIter = AstNodeListIterator<Language, AnyCssCurlyBracketsBlockContent>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssCurlyBracketsBlockContent {
    type Item = AnyCssCurlyBracketsBlockContent;
    type IntoIter = AstNodeListIterator<Language, AnyCssCurlyBracketsBlockContent>;
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
pub struct CssQualifiedRulePrelude {
    syntax_list: SyntaxList,
}
impl CssQualifiedRulePrelude {
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
impl AstNode for CssQualifiedRulePrelude {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_QUALIFIED_RULE_PRELUDE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_QUALIFIED_RULE_PRELUDE
    }
    fn cast(syntax: SyntaxNode) -> Option<CssQualifiedRulePrelude> {
        if Self::can_cast(syntax.kind()) {
            Some(CssQualifiedRulePrelude {
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
impl Serialize for CssQualifiedRulePrelude {
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
impl AstNodeList for CssQualifiedRulePrelude {
    type Language = Language;
    type Node = CssComponentValue;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssQualifiedRulePrelude {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssQualifiedRulePrelude ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssQualifiedRulePrelude {
    type Item = CssComponentValue;
    type IntoIter = AstNodeListIterator<Language, CssComponentValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssQualifiedRulePrelude {
    type Item = CssComponentValue;
    type IntoIter = AstNodeListIterator<Language, CssComponentValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssSimpleComponentValueList {
    syntax_list: SyntaxList,
}
impl CssSimpleComponentValueList {
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
impl AstNode for CssSimpleComponentValueList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SIMPLE_COMPONENT_VALUE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SIMPLE_COMPONENT_VALUE_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<CssSimpleComponentValueList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssSimpleComponentValueList {
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
impl Serialize for CssSimpleComponentValueList {
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
impl AstNodeList for CssSimpleComponentValueList {
    type Language = Language;
    type Node = CssComponentValue;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssSimpleComponentValueList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssSimpleComponentValueList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssSimpleComponentValueList {
    type Item = CssComponentValue;
    type IntoIter = AstNodeListIterator<Language, CssComponentValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssSimpleComponentValueList {
    type Item = CssComponentValue;
    type IntoIter = AstNodeListIterator<Language, CssComponentValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssStyleSheetContent {
    syntax_list: SyntaxList,
}
impl CssStyleSheetContent {
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
impl AstNode for CssStyleSheetContent {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_STYLE_SHEET_CONTENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_STYLE_SHEET_CONTENT
    }
    fn cast(syntax: SyntaxNode) -> Option<CssStyleSheetContent> {
        if Self::can_cast(syntax.kind()) {
            Some(CssStyleSheetContent {
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
impl Serialize for CssStyleSheetContent {
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
impl AstNodeList for CssStyleSheetContent {
    type Language = Language;
    type Node = AnyCssStylesheetContent;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssStyleSheetContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssStyleSheetContent ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssStyleSheetContent {
    type Item = AnyCssStylesheetContent;
    type IntoIter = AstNodeListIterator<Language, AnyCssStylesheetContent>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssStyleSheetContent {
    type Item = AnyCssStylesheetContent;
    type IntoIter = AstNodeListIterator<Language, AnyCssStylesheetContent>;
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
